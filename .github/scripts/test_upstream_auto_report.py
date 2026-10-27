"""2A.3: local Git and mocked-network tests for read-only policy reporting."""

from __future__ import annotations

import hashlib
import json
import subprocess
import tempfile
import unittest
from dataclasses import replace
from pathlib import Path
from unittest.mock import patch

import upstream_auto_report as report
from upstream_sync import (
    build_candidate_manifest,
    candidate_manifest_json,
    git_tree_entries,
)


def git(repo: Path, *args: str) -> str:
    return subprocess.run(
        ["git", *args],
        cwd=repo,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


LOCKFILE = """version = 3

[[package]]
name = "rustls"
version = "0.23.46"

[[package]]
name = "quinn-proto"
version = "0.11.18"

[[package]]
name = "noyalib-serde-yaml"
version = "0.0.57"
"""


class AutoReportTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory()
        repo = cls.repo = Path(cls.temp.name) / "repo"
        repo.mkdir()
        git(repo, "init", "-q")
        git(repo, "config", "user.name", "auto-report-test")
        git(repo, "config", "user.email", "auto-report-test@example.invalid")
        git(repo, "config", "core.autocrlf", "false")
        git(
            repo,
            "remote",
            "add",
            "origin",
            "https://github.com/Takuetsu/codex-dd-astra.git",
        )
        (repo / "codex-rs").mkdir()
        (repo / "docs").mkdir()
        (repo / "codex-rs/Cargo.lock").write_text(LOCKFILE, encoding="utf-8")
        (repo / "codex-rs/codexdd-version.txt").write_text("0.4.4\n")
        (repo / "codex-rs/upstream-codex-release.txt").write_text("rust-v0.162.0\n")
        (repo / "docs/overview.md").write_text("tracked\n")
        git(repo, "add", "-A")
        git(repo, "commit", "-q", "-m", "tracked baseline")
        cls.tracked = git(repo, "rev-parse", "HEAD")
        git(repo, "tag", "-a", "rust-v0.162.0", "-m", "tracked")

        git(repo, "switch", "-q", "-c", "dd/astra-policy-v2")
        (repo / "fork-only.txt").write_text("fork\n")
        git(repo, "add", "-A")
        git(repo, "commit", "-q", "-m", "production")
        cls.production = git(repo, "rev-parse", "HEAD")

        git(repo, "switch", "-q", "-c", "upstream-next", cls.tracked)
        (repo / "docs/overview.md").write_text("upstream update\n")
        git(repo, "add", "-A")
        git(repo, "commit", "-q", "-m", "upstream target")
        cls.target = git(repo, "rev-parse", "HEAD")
        git(repo, "tag", "-a", "rust-v0.163.0", "-m", "target")
        git(repo, "switch", "-q", "dd/astra-policy-v2")

        cls.tracked_oid = git(repo, "rev-parse", "refs/tags/rust-v0.162.0")
        cls.target_oid = git(repo, "rev-parse", "refs/tags/rust-v0.163.0")
        cls.pinned_lock = (repo / "codex-rs/Cargo.lock").read_bytes()
        cls.fixture_args = dict(
            discovery_timestamp="2026-10-17T20:00:00Z",
            production_branch="dd/astra-policy-v2",
            production_sha=cls.production,
            production_tree=git(repo, "rev-parse", cls.production + "^{tree}"),
            codexdd_product_version="0.4.4",
            tracked_upstream_tag="rust-v0.162.0",
            tracked_upstream_sha=cls.tracked,
            tracked_upstream_tree=git(repo, "rev-parse", cls.tracked + "^{tree}"),
            target_upstream_tag="rust-v0.163.0",
            target_upstream_sha=cls.target,
            target_upstream_tree=git(repo, "rev-parse", cls.target + "^{tree}"),
            ancestry_status="target_descends_from_tracked",
            merge_base_sha=cls.tracked,
            tracked_entries=git_tree_entries(repo, cls.tracked),
            production_entries=git_tree_entries(repo, cls.production),
            target_entries=git_tree_entries(repo, cls.target),
        )

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    def run_policy(
        self,
        *,
        manifest=None,
        observed="clean",
        conflicts=b"",
        lockfile=None,
        prod_remote=None,
        candidate_remote=None,
        target_oid=None,
        tracked_oid=None,
        pr_exists=False,
    ):
        if manifest is None:
            manifest = build_candidate_manifest(
                **self.fixture_args, transplant_result=observed
            )
        raw = candidate_manifest_json(manifest).encode("utf-8")

        def remote(_repo, ref):
            if ref == report.PRODUCTION_REF:
                return self.production if prod_remote is None else prod_remote
            return candidate_remote

        def official(_repo, tag):
            return (
                (self.target_oid if target_oid is None else target_oid)
                if tag == "rust-v0.163.0"
                else (self.tracked_oid if tracked_oid is None else tracked_oid)
            )

        with (
            patch.object(report, "_remote_branch", side_effect=remote),
            patch.object(report, "_official_oid", side_effect=official),
            patch.object(report, "_open_pr_exists", return_value=pr_exists),
        ):
            return report.evaluate(
                self.repo,
                raw,
                pinned_target_oid=self.target_oid,
                pinned_tracked_oid=self.tracked_oid,
                observed_transplant=observed,
                conflicts=conflicts,
                candidate_lockfile=self.pinned_lock if lockfile is None else lockfile,
            )

    def test_real_tree_recheck_keeps_report_only_and_blocks_unallowlisted(self):
        value = self.run_policy()
        self.assertTrue(value["report_only"])
        self.assertFalse(value["may_publish"])
        self.assertFalse(value["windows_validated"])
        self.assertEqual(value["status"], "manual_semantic_review_required")
        self.assertEqual(value["production_sha"], self.production)
        self.assertEqual(value["pinned_tag_oid"], value["observed_tag_oid"])
        self.assertIn("upstream_path_not_allowlisted", value["reasons"])
        self.assertNotIn("dependency_graph_changed", value["reasons"])

    def test_both_tag_and_production_oid_are_bound_to_report(self):
        value = self.run_policy()
        self.assertEqual(value["candidate_key"], "rust-v0.163.0@" + self.target)
        self.assertRegex(value["manifest_sha256"], r"^[0-9a-f]{64}$")

    def test_original_conflict_not_relabelled_clean(self):
        value = self.run_policy(
            observed="conflict",
            conflicts=b"docs/overview.md\n",
            lockfile=b"",
        )
        self.assertEqual(value["status"], "blocked_transplant_conflict")

    def test_conflicted_discovery_does_not_depend_on_pr_metadata(self):
        manifest = build_candidate_manifest(
            **self.fixture_args, transplant_result="conflict"
        )

        def remote(_repo, ref):
            if ref == report.PRODUCTION_REF:
                return self.production
            raise AssertionError("candidate ref should not be queried for conflicts")

        with (
            patch.object(report, "_remote_branch", side_effect=remote),
            patch.object(
                report,
                "_official_oid",
                side_effect=lambda _r, tag: (
                    self.target_oid if tag == "rust-v0.163.0" else self.tracked_oid
                ),
            ),
            patch.object(
                report,
                "_open_pr_exists",
                side_effect=AssertionError(
                    "PR metadata is not needed to report a transplant conflict"
                ),
            ),
        ):
            value = report.evaluate(
                self.repo,
                candidate_manifest_json(manifest).encode("utf-8"),
                pinned_target_oid=self.target_oid,
                pinned_tracked_oid=self.tracked_oid,
                observed_transplant="conflict",
                conflicts=b"docs/overview.md\n",
                candidate_lockfile=None,
            )
        self.assertEqual(value["status"], "blocked_transplant_conflict")

    def test_conflict_evidence_mismatch_fails_closed(self):
        m = build_candidate_manifest(**self.fixture_args, transplant_result="clean")
        with self.assertRaisesRegex(
            report.VerificationError, "transplant_state_mismatch"
        ):
            self.run_policy(manifest=m, observed="conflict")
        with self.assertRaisesRegex(
            report.VerificationError, "conflicts_in_clean_transplant"
        ):
            self.run_policy(manifest=m, observed="clean", conflicts=b"unmerged\n")

    def test_stale_remote_production_is_blocked(self):
        with self.assertRaisesRegex(report.VerificationError, "production_ref_moved"):
            self.run_policy(prod_remote="f" * 40)

    def test_canonical_upstream_tag_retarget_is_blocked(self):
        with self.assertRaisesRegex(report.VerificationError, "official_tag_moved"):
            self.run_policy(target_oid="f" * 40)
        with self.assertRaisesRegex(report.VerificationError, "official_tag_moved"):
            self.run_policy(tracked_oid="f" * 40)

    def test_existing_branch_and_pull_request_are_duplicates(self):
        self.assertEqual(
            self.run_policy(candidate_remote="a" * 40)["status"], "blocked_duplicate"
        )
        self.assertEqual(self.run_policy(pr_exists=True)["status"], "blocked_duplicate")

    def test_unsafe_dependency_rollbacks_are_reported(self):
        lock = self.pinned_lock.replace(b"0.23.46", b"0.23.45")
        value = self.run_policy(lockfile=lock)
        self.assertEqual(value["status"], "manual_semantic_review_required")
        self.assertIn("security_dependency_downgrade", value["reasons"])
        self.assertIn("dependency_graph_changed", value["reasons"])

    def test_missing_candidate_lockfile_for_clean_source_blocks(self):
        # Directly invoke with missing evidence instead of run_policy's default.
        manifest = build_candidate_manifest(
            **self.fixture_args, transplant_result="clean"
        )
        raw = candidate_manifest_json(manifest).encode()
        with (
            patch.object(
                report,
                "_remote_branch",
                side_effect=lambda _r, ref: (
                    self.production if ref == report.PRODUCTION_REF else None
                ),
            ),
            patch.object(
                report,
                "_official_oid",
                side_effect=lambda _r, tag: (
                    self.target_oid if tag == "rust-v0.163.0" else self.tracked_oid
                ),
            ),
            patch.object(report, "_open_pr_exists", return_value=False),
        ):
            value = report.evaluate(
                self.repo,
                raw,
                pinned_target_oid=self.target_oid,
                pinned_tracked_oid=self.tracked_oid,
                observed_transplant="clean",
                conflicts=b"",
                candidate_lockfile=None,
            )
        self.assertIn("missing_dependency_snapshot", value["reasons"])

    def test_manifest_duplicate_key_and_noncanonical_bytes_are_rejected(self):
        m = build_candidate_manifest(**self.fixture_args, transplant_result="clean")
        raw = candidate_manifest_json(m).encode()
        with self.assertRaisesRegex(report.VerificationError, "noncanonical_manifest"):
            report._manifest(b" " + raw)
        with self.assertRaisesRegex(report.VerificationError, "duplicate_json_key"):
            report._manifest(
                raw.replace(
                    b'"candidate_key":', b'"candidate_key":"x","candidate_key":', 1
                )
            )

    def test_manifest_extra_field_or_bad_types_are_rejected(self):
        m = build_candidate_manifest(**self.fixture_args, transplant_result="clean")
        payload = json.loads(candidate_manifest_json(m))
        payload["untrusted_field"] = "ignored?"
        with self.assertRaisesRegex(
            report.VerificationError, "invalid_manifest_fields"
        ):
            report._manifest(json.dumps(payload).encode())
        del payload["untrusted_field"]
        payload["overlap_paths"] = "not a list"
        with self.assertRaisesRegex(report.VerificationError, "invalid_manifest_types"):
            report._manifest(json.dumps(payload).encode())

    def test_stale_or_forged_manifest_tree_inventory_is_rejected(self):
        m = build_candidate_manifest(**self.fixture_args, transplant_result="clean")
        forged = replace(m, upstream_change_path_count=1234)
        value = self.run_policy(manifest=forged)
        self.assertEqual(value["status"], "blocked_identity")
        self.assertIn("manifest_inventory_mismatch", value["reasons"])

    def test_lockfile_hash_captures_graph_only_drift(self):
        original = report._packages(self.pinned_lock)
        changed = report._packages(
            self.pinned_lock + b"\n# changed dependency graph metadata\n"
        )
        self.assertEqual(original["rustls"], changed["rustls"])
        self.assertNotEqual(original["$lockfile_sha256"], changed["$lockfile_sha256"])

    def test_untrusted_origin_remote_refused_without_network(self):
        git(
            self.repo,
            "remote",
            "set-url",
            "origin",
            "https://github.com/other/codex.git",
        )
        try:
            with self.assertRaisesRegex(report.VerificationError, "untrusted_origin"):
                report._remote_branch(self.repo, report.PRODUCTION_REF)
        finally:
            git(
                self.repo,
                "remote",
                "set-url",
                "origin",
                "https://github.com/Takuetsu/codex-dd-astra.git",
            )

    def test_pr_lookup_unavailable_blocks_not_assumes_absent(self):
        with patch.dict("os.environ", {"GH_TOKEN": ""}):
            with self.assertRaisesRegex(
                report.VerificationError, "pr_read_unavailable"
            ):
                report._open_pr_exists(self.repo, "automation/test")

    def test_summary_contains_only_policy_codes_not_upstream_text(self):
        value = self.run_policy()
        text = report.summary_text(value)
        self.assertIn("report", text.lower())
        self.assertIn("Windows validation", text)
        value["reasons"] = ["evil" + chr(10) + "injection"]
        with self.assertRaisesRegex(report.VerificationError, "unsafe_report_reason"):
            report.summary_text(value)

    def test_workflow_permissions_and_inert_report_adapter(self):
        workflow = (
            Path(__file__).resolve().parents[1] / "workflows/upstream-sync.yml"
        ).read_text(encoding="utf-8")
        self.assertIn("permissions:\n  contents: read", workflow)
        self.assertIn("python3 .github/scripts/upstream_auto_report.py", workflow)
        self.assertIn("--pinned-target-oid", workflow)
        self.assertIn("--pinned-tracked-oid", workflow)
        self.assertIn("eligibility-report.json", workflow)
        self.assertIn("Upload candidate evidence", workflow)
        self.assertNotIn("contents: write", workflow)
        self.assertNotIn("pull-requests: write", workflow)
        self.assertNotIn("git push", workflow)
        self.assertNotIn("gh pr create", workflow)
        self.assertNotIn("gh pr merge", workflow)


if __name__ == "__main__":
    unittest.main()
