"""CodexDD 0.4.5 packet 2B.2 — draft-PR policy and denial regression."""

from __future__ import annotations

import json
import os
import subprocess
import unittest
from contextlib import ExitStack
from dataclasses import replace
from pathlib import Path
from unittest.mock import patch

import test_upstream_auto_prepare as prep_fixture
import upstream_auto_eligibility as policy
import upstream_auto_prepare as preparer
import upstream_auto_pr as adapter
import upstream_auto_report as report
from upstream_sync import candidate_manifest_json


def output(stdout="", stderr="", code=0):
    return subprocess.CompletedProcess(
        args=["gh"], returncode=code, stdout=stdout, stderr=stderr
    )


class DraftPRTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        # Real temporary Git object fixture, no network/credential interaction.
        prep_fixture.GuardedPreparationTests.setUpClass()
        cls.case = prep_fixture.GuardedPreparationTests()
        cls.fx = prep_fixture.GuardedPreparationTests.fx
        cls.manifest = prep_fixture.GuardedPreparationTests.manifest
        cls.raw = candidate_manifest_json(cls.manifest).encode("utf-8")
        patches = cls.case._mocks()
        with patches[0], patches[1], patches[2], patches[3]:
            cls.prepared = cls.case._build()
        cls.plan = adapter.draft_plan(cls.prepared, cls.raw)

    @classmethod
    def tearDownClass(cls):
        prep_fixture.GuardedPreparationTests.tearDownClass()

    def verified(self, *, candidate_sha=None, production_sha=None, target_oid=None):
        stack = ExitStack()
        live_candidate = (
            self.prepared.prepared_commit_sha
            if candidate_sha is None
            else candidate_sha
        )
        live_production = (
            self.fx.production if production_sha is None else production_sha
        )
        stack.enter_context(
            patch.object(
                report,
                "_remote_branch",
                side_effect=lambda _repo, ref: (
                    live_production if ref == report.PRODUCTION_REF else live_candidate
                ),
            )
        )
        stack.enter_context(
            patch.object(
                report,
                "_official_oid",
                side_effect=lambda _repo, tag: (
                    self.fx.target_oid
                    if tag == "rust-v0.163.0" and target_oid is None
                    else target_oid
                    if tag == "rust-v0.163.0"
                    else self.fx.tracked_oid
                ),
            )
        )
        stack.enter_context(
            patch.object(
                policy,
                "REVIEWED_SAFE_UPSTREAM_PATHS",
                frozenset({"docs/overview.md"}),
            )
        )
        return stack

    def attempt(self):
        return adapter.attempt_draft_pr(
            self.fx.repo,
            self.prepared,
            self.raw,
            pinned_target_oid=self.fx.target_oid,
            pinned_tracked_oid=self.fx.tracked_oid,
        )

    def test_default_draft_write_gate_is_hard_disabled(self):
        self.assertFalse(adapter.DRAFT_PR_WRITES_ENABLED)
        self.assertFalse(preparer.AUTO_PUBLISH_ENABLED)
        self.assertFalse(policy.REVIEWED_SAFE_UPSTREAM_PATHS)
        with patch.object(
            adapter,
            "_gh",
            side_effect=AssertionError("disabled adapter must not call GitHub CLI"),
        ):
            result = self.attempt()
        self.assertEqual(result.status, "owner_action_required")
        self.assertEqual(result.reason, "automatic_draft_pr_gate_closed")
        self.assertIsNone(result.pr_url)

    def test_plan_carries_unvalidated_banner_and_complete_provenance(self):
        plan = self.plan
        self.assertIn("UNVALIDATED", plan.title)
        self.assertIn("NEEDS_WINDOWS_AND_OWNER_REVIEW", plan.body)
        self.assertIn(self.prepared.prepared_commit_sha, plan.body)
        self.assertIn(self.manifest.target_upstream_sha, plan.body)
        self.assertIn(self.manifest.production_sha, plan.body)
        self.assertIn(self.prepared.manifest_sha256, plan.body)
        self.assertIn("DO NOT MERGE", plan.body)
        self.assertNotIn("[skip ci]", plan.body.lower())
        self.assertNotIn("validation passed", plan.body.lower())
        self.assertNotIn("auto-merge enabled", plan.body.lower())

    def test_tampered_candidate_key_hash_branch_and_commit_are_rejected(self):
        changes = (
            dict(candidate_key="wrong"),
            dict(candidate_branch="automation/sneaky"),
            dict(manifest_sha256="0" * 64),
            dict(prepared_commit_sha="not-oid"),
            dict(target_sha="0" * 40),
            dict(production_sha="0" * 40),
        )
        for value in changes:
            with self.subTest(value=value):
                with self.assertRaises(adapter.DraftPRBlocked):
                    adapter.draft_plan(replace(self.prepared, **value), self.raw)

    def test_conflicted_or_sensitive_manifest_cannot_generate_draft(self):
        for changed in (
            dict(transplant_dry_run="conflict", state="blocked_transplant_conflict"),
            dict(overlap_path_count=2, state="manual_semantic_review_required"),
            dict(sensitive_overlap_count=1, state="manual_semantic_review_required"),
        ):
            with self.subTest(changed=changed):
                manifest = replace(self.manifest, **changed)
                with self.assertRaises(adapter.DraftPRBlocked):
                    adapter.draft_plan(
                        self.prepared, candidate_manifest_json(manifest).encode()
                    )

    def test_live_candidate_identity_can_be_proven_from_actual_git_objects(self):
        with self.verified():
            adapter.verify_published_source(
                self.fx.repo,
                self.plan,
                self.prepared,
                self.raw,
                pinned_target_oid=self.fx.target_oid,
                pinned_tracked_oid=self.fx.tracked_oid,
            )

    def test_remote_candidate_ref_missing_or_moved_blocks(self):
        for bad, message in (
            (None, "candidate_not_published"),
            ("f" * 40, "candidate_ref_moved"),
        ):
            # None in verified() means default; override the candidate ref directly.
            with (
                self.verified(),
                patch.object(
                    report,
                    "_remote_branch",
                    side_effect=lambda _r, ref: (
                        self.fx.production if ref == report.PRODUCTION_REF else bad
                    ),
                ),
            ):
                with self.assertRaisesRegex(adapter.DraftPRBlocked, message):
                    adapter.verify_published_source(
                        self.fx.repo,
                        self.plan,
                        self.prepared,
                        self.raw,
                        pinned_target_oid=self.fx.target_oid,
                        pinned_tracked_oid=self.fx.tracked_oid,
                    )

    def test_production_sha_or_official_tag_moves_fail_closed(self):
        with self.verified(production_sha="f" * 40):
            with self.assertRaisesRegex(adapter.DraftPRBlocked, "production_ref_moved"):
                adapter.verify_published_source(
                    self.fx.repo,
                    self.plan,
                    self.prepared,
                    self.raw,
                    pinned_target_oid=self.fx.target_oid,
                    pinned_tracked_oid=self.fx.tracked_oid,
                )
        with self.verified(target_oid="f" * 40):
            with self.assertRaisesRegex(
                adapter.DraftPRBlocked, "official_tag_ref_moved"
            ):
                adapter.verify_published_source(
                    self.fx.repo,
                    self.plan,
                    self.prepared,
                    self.raw,
                    pinned_target_oid=self.fx.target_oid,
                    pinned_tracked_oid=self.fx.tracked_oid,
                )

    def test_exact_candidate_tree_manifest_and_base_verified(self):
        tampered_plan = replace(self.plan, manifest_sha256="a" * 64)
        with (
            self.verified(),
            self.assertRaisesRegex(adapter.DraftPRBlocked, "draft_plan_tampered"),
        ):
            adapter.verify_published_source(
                self.fx.repo,
                tampered_plan,
                self.prepared,
                self.raw,
                pinned_target_oid=self.fx.target_oid,
                pinned_tracked_oid=self.fx.tracked_oid,
            )

    def test_no_gh_token_does_not_attempt_pr_api(self):
        with (
            patch.object(adapter, "DRAFT_PR_WRITES_ENABLED", True),
            patch.dict(os.environ, {"GH_TOKEN": ""}),
            patch.object(adapter, "_gh", side_effect=AssertionError("No token")),
        ):
            result = self.attempt()
        self.assertEqual(result.status, "blocked_incomplete")
        self.assertEqual(result.reason, "no_authorized_gh_token")

    def test_existing_open_or_closed_candidate_pr_blocks_any_attempt(self):
        with (
            self.verified(),
            patch.object(adapter, "DRAFT_PR_WRITES_ENABLED", True),
            patch.dict(os.environ, {"GH_TOKEN": "fake-test-token"}),
            patch.object(
                adapter,
                "_gh",
                return_value=output(
                    json.dumps(
                        [{"number": 53, "headRefName": self.plan.candidate_branch}]
                    )
                ),
            ) as gh,
        ):
            result = self.attempt()
        self.assertEqual(result.status, "blocked_duplicate")
        self.assertEqual(result.reason, "candidate_pr_already_exists")
        self.assertEqual(gh.call_count, 1)

    def test_pr_query_error_or_truncation_never_means_no_duplicate(self):
        with (
            self.assertRaisesRegex(adapter.DraftPRBlocked, "pr_query_unavailable"),
            patch.object(adapter, "_gh", return_value=output(stderr="denied", code=1)),
        ):
            adapter._any_existing_pr(self.fx.repo, self.plan.candidate_branch)
        many = [{"number": i + 1, "headRefName": "other"} for i in range(1000)]
        with (
            self.assertRaisesRegex(adapter.DraftPRBlocked, "pr_query_incomplete"),
            patch.object(adapter, "_gh", return_value=output(json.dumps(many))),
        ):
            adapter._any_existing_pr(self.fx.repo, self.plan.candidate_branch)

    def test_actions_policy_denial_results_in_owner_handoff_not_retry(self):
        attempts = []

        def fake_gh(_repo, *args):
            attempts.append(args)
            if args[:2] == ("pr", "list"):
                return output("[]")
            if args[:2] == ("pr", "create"):
                return output(
                    stderr="GraphQL: GitHub Actions is not permitted to create or approve pull requests.",
                    code=1,
                )
            raise AssertionError("No retries / no alternative credential route")

        with (
            self.verified(),
            patch.object(adapter, "DRAFT_PR_WRITES_ENABLED", True),
            patch.dict(os.environ, {"GH_TOKEN": "fake-test-token"}),
            patch.object(adapter, "_gh", side_effect=fake_gh),
        ):
            result = self.attempt()
        self.assertEqual(result.status, "owner_action_required")
        self.assertEqual(result.reason, "github_actions_pr_creation_forbidden")
        self.assertIsNone(result.pr_url)
        self.assertEqual(len([a for a in attempts if a[:2] == ("pr", "create")]), 1)
        self.assertEqual(len([a for a in attempts if a[:2] == ("pr", "list")]), 2)
        self.assertIn(
            self.prepared.prepared_commit_sha, adapter.owner_handoff(self.plan, result)
        )

    def test_unknown_permission_failure_does_not_claim_creation_or_retry(self):
        def fake_gh(_repo, *args):
            if args[:2] == ("pr", "list"):
                return output("[]")
            return output(stderr="unexpected remote error", code=1)

        with (
            self.verified(),
            patch.object(adapter, "DRAFT_PR_WRITES_ENABLED", True),
            patch.dict(os.environ, {"GH_TOKEN": "fake-test-token"}),
            patch.object(adapter, "_gh", side_effect=fake_gh),
        ):
            result = self.attempt()
        self.assertEqual(result.status, "blocked_incomplete")
        self.assertEqual(result.reason, "pr_create_failed_no_retry")

    def test_mocked_success_requires_exact_draft_ref_sha_and_base(self):
        url = "https://github.com/Takuetsu/codex-dd-astra/pull/80"
        calls = []

        def fake_gh(_repo, *args):
            calls.append(args)
            if args[:2] == ("pr", "list"):
                return output("[]")
            if args[:2] == ("pr", "create"):
                self.assertIn("--draft", args)
                self.assertNotIn("--auto", args)
                self.assertNotIn("--admin", args)
                return output(url + "\n")
            if args[:2] == ("pr", "view"):
                return output(
                    json.dumps(
                        {
                            "url": url,
                            "isDraft": True,
                            "headRefName": self.plan.candidate_branch,
                            "headRefOid": self.plan.candidate_sha,
                            "baseRefName": "dd/astra-policy-v2",
                        }
                    )
                )
            raise AssertionError("Unexpected GH action")

        with (
            self.verified(),
            patch.object(adapter, "DRAFT_PR_WRITES_ENABLED", True),
            patch.dict(os.environ, {"GH_TOKEN": "fake-test-token"}),
            patch.object(adapter, "_gh", side_effect=fake_gh),
        ):
            result = self.attempt()
        self.assertEqual(result.status, "draft_created_unvalidated")
        self.assertEqual(result.pr_url, url)
        self.assertEqual(len([a for a in calls if a[:2] == ("pr", "create")]), 1)

    def test_returned_pr_must_still_be_draft_at_exact_head(self):
        url = "https://github.com/Takuetsu/codex-dd-astra/pull/80"

        def fake_gh(_repo, *args):
            if args[:2] == ("pr", "list"):
                return output("[]")
            if args[:2] == ("pr", "create"):
                return output(url)
            return output(
                json.dumps(
                    {
                        "url": url,
                        "isDraft": False,
                        "headRefName": self.plan.candidate_branch,
                        "headRefOid": self.plan.candidate_sha,
                        "baseRefName": "dd/astra-policy-v2",
                    }
                )
            )

        with (
            self.verified(),
            patch.object(adapter, "DRAFT_PR_WRITES_ENABLED", True),
            patch.dict(os.environ, {"GH_TOKEN": "fake-test-token"}),
            patch.object(adapter, "_gh", side_effect=fake_gh),
        ):
            result = self.attempt()
        self.assertEqual(result.status, "blocked_incomplete")
        self.assertEqual(result.reason, "created_pr_identity_unverified")
        self.assertIsNone(result.pr_url)

    def test_second_preflight_detects_race_before_create(self):
        count = {"candidate": 0}

        def remote(_repo, ref):
            if ref == report.PRODUCTION_REF:
                return self.fx.production
            count["candidate"] += 1
            return (
                self.prepared.prepared_commit_sha
                if count["candidate"] == 1
                else "f" * 40
            )

        with (
            self.verified(),
            patch.object(report, "_remote_branch", side_effect=remote),
            patch.object(adapter, "DRAFT_PR_WRITES_ENABLED", True),
            patch.dict(os.environ, {"GH_TOKEN": "fake-test-token"}),
            patch.object(adapter, "_gh", return_value=output("[]")) as gh,
        ):
            result = self.attempt()
        self.assertEqual(result.status, "blocked_incomplete")
        self.assertEqual(result.reason, "candidate_ref_moved")
        self.assertEqual(
            len([x for x in gh.call_args_list if x.args[1:3] == ("pr", "create")]), 0
        )

    def test_owner_fallback_is_source_bound_and_not_fake_validation(self):
        result = adapter.DraftPRResult(
            status="owner_action_required",
            reason="github_actions_pr_creation_forbidden",
            candidate_branch=self.plan.candidate_branch,
            candidate_sha=self.plan.candidate_sha,
        )
        text = adapter.owner_handoff(self.plan, result)
        self.assertIn("UNVALIDATED", text)
        self.assertIn(self.plan.manifest_sha256, text)
        self.assertIn("GitHub", text)
        with self.assertRaises(adapter.DraftPRBlocked):
            adapter.owner_handoff(self.plan, replace(result, candidate_sha="f" * 40))
        self.assertEqual(result.canonical_json(), result.canonical_json())

    def test_workflows_keep_permissions_and_no_pr_adapter_execution(self):
        root = Path(__file__).resolve().parents[1] / "workflows"
        discovery = (root / "upstream-sync.yml").read_text(encoding="utf-8")
        prepare = (root / "upstream-prepare.yml").read_text(encoding="utf-8")
        promotion = (root / "upstream-promote.yml").read_text(encoding="utf-8")
        for name, workflow in (
            ("discover", discovery),
            ("prepare", prepare),
            ("promote", promotion),
        ):
            with self.subTest(workflow=name):
                self.assertNotIn("upstream_auto_pr.py", workflow)
                self.assertNotIn("gh pr merge", workflow)
                self.assertNotIn("auto-merge", workflow)
        self.assertIn("permissions:\n  contents: read", discovery)
        self.assertIn("permissions:\n  contents: write", prepare)
        self.assertIn("pull-requests: write", promotion)
        self.assertNotIn("pull-requests: write", prepare)
        self.assertNotIn("gh pr create", discovery)


if __name__ == "__main__":
    unittest.main()
