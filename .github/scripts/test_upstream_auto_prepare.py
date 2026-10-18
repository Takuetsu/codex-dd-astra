"""CodexDD 0.4.5 — packet 2B.1 fail-closed local preparer tests."""

from __future__ import annotations

import hashlib
import json
import subprocess
import unittest
from dataclasses import replace
from pathlib import Path
from unittest.mock import patch

import test_upstream_auto_report as fixture
import upstream_auto_eligibility as planner
import upstream_auto_prepare as preparer
import upstream_auto_report as report
from upstream_sync import build_candidate_manifest, candidate_manifest_json, git_tree_entries


class GuardedPreparationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        # Reuse the proven actual Git SHA/tree/tag/lockfile fixture; no network.
        fixture.AutoReportTests.setUpClass()
        cls.fx = fixture.AutoReportTests
        cls.manifest = build_candidate_manifest(
            **cls.fx.fixture_args, transplant_result="clean"
        )
        cls.raw_manifest = candidate_manifest_json(cls.manifest).encode("utf-8")
        cls.allowlisted_path = "docs/overview.md"

    @classmethod
    def tearDownClass(cls):
        fixture.AutoReportTests.tearDownClass()

    def _mocks(self, *, safe: bool = True, change_remote: str = ""):
        fx = self.fx
        attempts = {"production": 0, "candidate": 0}

        def remote(_repo, ref):
            if ref == report.PRODUCTION_REF:
                attempts["production"] += 1
                if change_remote == "production" and attempts["production"] > 1:
                    return "f" * 40
                return fx.production
            attempts["candidate"] += 1
            if change_remote == "candidate" and attempts["candidate"] > 1:
                return "f" * 40
            return None

        def official(_repo, tag):
            return fx.target_oid if tag == "rust-v0.163.0" else fx.tracked_oid

        allowlist = frozenset({self.allowlisted_path}) if safe else frozenset()
        return (
            patch.object(report, "_remote_branch", side_effect=remote),
            patch.object(report, "_official_oid", side_effect=official),
            patch.object(report, "_open_pr_exists", return_value=False),
            patch.object(planner, "REVIEWED_SAFE_UPSTREAM_PATHS", allowlist),
        )

    def _build(self, raw: bytes | None = None):
        return preparer.build_isolated_candidate(
            self.fx.repo,
            self.raw_manifest if raw is None else raw,
            pinned_target_oid=self.fx.target_oid,
            pinned_tracked_oid=self.fx.tracked_oid,
        )

    def _candidate_ref_exists(self) -> bool:
        branch = preparer.candidate_branch(
            self.manifest.target_upstream_tag, self.manifest.target_upstream_sha
        )
        result = subprocess.run(
            ["git", "show-ref", "--verify", "--quiet", "refs/heads/" + branch],
            cwd=self.fx.repo, check=False, capture_output=True,
        )
        return result.returncode == 0

    def test_publishing_is_disabled_by_source_policy(self):
        self.assertIs(preparer.AUTO_PUBLISH_ENABLED, False)
        self.assertFalse(planner.REVIEWED_SAFE_UPSTREAM_PATHS)
        self.assertFalse(self._candidate_ref_exists())

    def test_default_policy_refuses_even_disjoint_clean_upstream_delta(self):
        mocks = self._mocks(safe=False)
        with mocks[0], mocks[1], mocks[2], mocks[3]:
            with self.assertRaisesRegex(
                preparer.PreparationBlocked,
                "eligibility_manual_semantic_review_required",
            ):
                self._build()
        self.assertFalse(self._candidate_ref_exists())

    def test_exact_tree_overlay_retains_both_sources_and_manifest_bytes(self):
        mocks = self._mocks()
        status_before = preparer._git(
            self.fx.repo, "status", "--porcelain"
        )
        with mocks[0], mocks[1], mocks[2], mocks[3]:
            built = self._build()
        self.assertEqual(built.production_sha, self.fx.production)
        self.assertEqual(built.target_sha, self.fx.target)
        self.assertEqual(
            built.candidate_branch,
            "automation/upstream-candidate-rust-v0.163.0-" + self.fx.target[:12],
        )
        self.assertEqual(
            built.manifest_sha256,
            hashlib.sha256(self.raw_manifest).hexdigest(),
        )
        tree = git_tree_entries(self.fx.repo, built.prepared_tree_sha)
        production = git_tree_entries(self.fx.repo, self.fx.production)
        target = git_tree_entries(self.fx.repo, self.fx.target)
        self.assertEqual(tree["fork-only.txt"], production["fork-only.txt"])
        self.assertEqual(tree["docs/overview.md"], target["docs/overview.md"])
        self.assertEqual(
            preparer._git_blob(
                self.fx.repo, built.prepared_tree_sha, built.manifest_path
            ),
            self.raw_manifest,
        )
        self.assertEqual(
            preparer._git_blob(
                self.fx.repo, built.prepared_tree_sha,
                "codex-rs/upstream-codex-release.txt",
            ),
            b"rust-v0.163.0\n",
        )
        self.assertEqual(
            preparer._git(self.fx.repo, "status", "--porcelain"), status_before
        )
        self.assertFalse(self._candidate_ref_exists())

    def test_repeated_provenance_produces_same_tree_and_commit_shas(self):
        mocks = self._mocks()
        with mocks[0], mocks[1], mocks[2], mocks[3]:
            first = self._build()
            second = self._build()
        self.assertEqual(first, second)
        self.assertFalse(self._candidate_ref_exists())

    def test_manifest_reproduces_unmodified_v1_json_schema(self):
        mocks = self._mocks()
        with mocks[0], mocks[1], mocks[2], mocks[3]:
            built = self._build()
        encoded = preparer._git_blob(
            self.fx.repo, built.prepared_tree_sha, built.manifest_path
        )
        self.assertEqual(encoded, self.raw_manifest)
        parsed = json.loads(encoded)
        self.assertEqual(parsed["contract_version"], 1)
        self.assertEqual(parsed["target_upstream_tag"], "rust-v0.163.0")
        self.assertNotIn("auto_preparation_ready", parsed["state"])
        self.assertNotIn("product_version_target", parsed)

    def test_owner_gate_prevents_remote_write_before_any_git_call(self):
        dummy = preparer.PreparedCandidate(
            candidate_branch="automation/upstream-candidate-rust-v0.163.0-" + "5" * 12,
            candidate_key="rust-v0.163.0@" + "5" * 40,
            production_sha="1" * 40,
            target_sha="5" * 40,
            prepared_commit_sha="3" * 40,
            prepared_tree_sha="4" * 40,
            manifest_path="docs/upstream-candidates/placeholder.json",
            manifest_sha256="a" * 64,
        )
        with patch.object(preparer, "_git", side_effect=AssertionError(
            "Git must not run under a closed publication gate"
        )):
            with self.assertRaisesRegex(
                preparer.PreparationBlocked, "owner_publication_gate_closed"
            ):
                preparer.publish_isolated_candidate(
                    self.fx.repo, dummy, b"irrelevant",
                    pinned_target_oid="8" * 40,
                    pinned_tracked_oid="9" * 40,
                )

    def test_stale_production_during_second_recheck_aborts(self):
        mocks = self._mocks(change_remote="production")
        with mocks[0], mocks[1], mocks[2], mocks[3]:
            with self.assertRaisesRegex(
                report.VerificationError, "production_ref_moved"
            ):
                self._build()
        self.assertFalse(self._candidate_ref_exists())

    def test_candidate_branch_race_between_rechecks_aborts(self):
        mocks = self._mocks(change_remote="candidate")
        with mocks[0], mocks[1], mocks[2], mocks[3]:
            with self.assertRaisesRegex(
                preparer.PreparationBlocked, "eligibility_blocked_duplicate"
            ):
                self._build()
        self.assertFalse(self._candidate_ref_exists())

    def test_unauthorized_github_pr_lookup_failure_blocks_build(self):
        mocks = self._mocks()
        with (
            mocks[0],
            mocks[1],
            patch.object(report, "_open_pr_exists", side_effect=report.VerificationError(
                "blocked_incomplete", "pr_read_unavailable"
            )),
            mocks[3],
        ):
            with self.assertRaisesRegex(
                report.VerificationError, "pr_read_unavailable"
            ):
                self._build()
        self.assertFalse(self._candidate_ref_exists())

    def test_invalid_or_tampered_manifest_fails_before_stage(self):
        mocks = self._mocks()
        with mocks[0], mocks[1], mocks[2], mocks[3]:
            with self.assertRaisesRegex(
                report.VerificationError, "invalid_manifest_json"
            ):
                self._build(b"not json")
            changed = replace(self.manifest, overlap_path_count=987)
            with self.assertRaisesRegex(
                preparer.PreparationBlocked, "eligibility_blocked_identity"
            ):
                self._build(candidate_manifest_json(changed).encode())
        self.assertFalse(self._candidate_ref_exists())

    def test_conflicted_manifest_does_not_create_a_tree(self):
        m = build_candidate_manifest(
            **self.fx.fixture_args, transplant_result="conflict"
        )
        mocks = self._mocks()
        with mocks[0], mocks[1], mocks[2], mocks[3]:
            with self.assertRaisesRegex(
                report.VerificationError, "transplant_state_mismatch"
            ):
                self._build(candidate_manifest_json(m).encode())
        self.assertFalse(self._candidate_ref_exists())

    def test_path_injection_is_rejected(self):
        for path in (
            "../outside", "docs/../outside", "/absolute",
            "docs//double", "docs/back\\slash", "docs/with,comma",
            "docs/./relative", ".", "docs/CON.txt", "docs/nul",
            "docs/.git/config", "docs/trailing.",
        ):
            with self.subTest(path=path):
                with self.assertRaises(preparer.PreparationBlocked):
                    preparer._checked_path(path)

    def test_mode_changes_are_denied_even_if_new_target_is_regular(self):
        path = "docs/overview.md"
        tracked = {path: "100755 blob " + "a" * 40}
        target = {path: "100644 blob " + "b" * 40}
        with self.assertRaisesRegex(
            preparer.PreparationBlocked, "upstream_mode_transition"
        ):
            preparer._verified_blob_change(path, tracked, target)
        for mode in ("120000 blob", "160000 commit", "100755 blob"):
            with self.subTest(mode=mode):
                with self.assertRaisesRegex(
                    preparer.PreparationBlocked, "upstream_nonregular_change"
                ):
                    preparer._verified_blob_change(
                        path, {path: "100644 blob " + "a" * 40},
                        {path: mode + " " + "b" * 40},
                    )

    def test_case_insensitive_windows_collisions_fail_closed(self):
        with self.assertRaisesRegex(
            preparer.PreparationBlocked, "windows_path_case_collision"
        ):
            preparer._check_windows_collisions(
                {"docs/Guide.md", "docs/guide.md"}
            )
        preparer._check_windows_collisions(
            {"docs/Guide.md", "docs/other.md"}
        )

    def test_interrupted_local_index_build_cannot_publish_branch(self):
        mocks = self._mocks()
        with mocks[0], mocks[1], mocks[2], mocks[3]:
            with patch.object(
                preparer, "_index_blob", side_effect=preparer.PreparationBlocked(
                    "incomplete_local_stage"
                )
            ):
                with self.assertRaisesRegex(
                    preparer.PreparationBlocked, "incomplete_local_stage"
                ):
                    self._build()
        self.assertFalse(self._candidate_ref_exists())

    def test_workflow_remains_read_only_and_never_invokes_preparer(self):
        workflow = (
            Path(__file__).resolve().parents[1]
            / "workflows/upstream-sync.yml"
        ).read_text(encoding="utf-8")
        self.assertIn("permissions:\n  contents: read", workflow)
        self.assertIn("upstream_auto_report.py", workflow)
        self.assertNotIn("upstream_auto_prepare.py", workflow)
        self.assertNotIn("contents: write", workflow)
        self.assertNotIn("gh pr create", workflow)
        self.assertNotIn("git push", workflow)


if __name__ == "__main__":
    unittest.main()
