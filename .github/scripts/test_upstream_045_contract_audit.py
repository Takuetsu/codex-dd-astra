"""CodexDD 0.4.5 packet 2B.3 — v1/v2 immutability and workflow gates.

No tests invoke an upstream action, GitHub API, compiler, candidate publish
or validation receipt producer. Synthetic receipt fixtures are schema checks,
NOT evidence that Daniel-CL native release acceptance has happened.
"""

from __future__ import annotations

import hashlib
import json
import unittest
from pathlib import Path

import test_upstream_reconciliation_receipt as receipt_fixtures
import upstream_auto_eligibility as policy
import upstream_auto_prepare as preparer
import upstream_auto_pr as draft
from upstream_reconciliation import canonical_json_bytes
from upstream_reconciliation_receipt import validate_v2_release_receipt
from upstream_sync import (
    candidate_branch,
    candidate_key,
    parse_validation_receipt_json,
    validate_validation_receipt,
)

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = ROOT / ".github/workflows"
SCRIPTS = ROOT / "scripts"
HISTORIC_SHA1 = {
    # Exact accepted 0.4.3 blocked/v2 source evidence objects, not a new 0.4.5 candidate.
    "docs/upstream-candidates/original-blocked-v1-manifest.json": "ff4376a08c0023dee754a8e77f3dc950f498be2c",
    "docs/upstream-candidates/original-blocked-conflicts.txt": "434ec9a6abf1417a2adea38d537f3151497c024e",
    "docs/upstream-candidates/codexdd-upstream-rust-v0.162.0.json": "a49bc9b33ada2ac2f82a12f19a4ab6e369f6736a",
    "docs/upstream-candidates/codexdd-upstream-rust-v0.162.0-reconciliation.json": "8d2c0645147660bf7514899057ac47177cba7608",
}


def v1_receipt() -> dict[str, object]:
    target = "rust-v0.163.0"
    sha = "5" * 40
    return {
        "contract_version": 1,
        "receipt_type": "codexdd_upstream_local_validation",
        "candidate_key": candidate_key(target, sha),
        "candidate_branch": candidate_branch(target, sha),
        "candidate_head_sha": "8" * 40,
        "production_sha": "1" * 40,
        "target_upstream_tag": target,
        "target_upstream_sha": sha,
        "manifest_path": "docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-555555555555.json",
        "manifest_sha256": "a" * 64,
        "profile": "work-packet",
        "validation_contract_version": 1,
        "validation_status": "pass",
        "completed_stages": 6,
        "completed_at": "2026-10-18T12:00:00Z",
    }


def v2_input():
    manifest, sidecar, manifest_bytes, sidecar_bytes = receipt_fixtures.fixture()
    receipt = receipt_fixtures.receipt_for(manifest, manifest_bytes)
    return {
        "receipt_json": canonical_json_bytes(receipt),
        "manifest_bytes": manifest_bytes,
        "sidecar_bytes": sidecar_bytes,
        "expected_manifest_path": receipt_fixtures.MANIFEST_PATH,
        "actual_candidate_sha": receipt_fixtures.CANDIDATE,
    }


class Upstream045ContractAuditTests(unittest.TestCase):
    def test_four_historical_evidence_blobs_are_bitwise_unchanged(self):
        for path, expected_sha in HISTORIC_SHA1.items():
            with self.subTest(path=path):
                data = (ROOT / path).read_bytes()
                raw = b"blob " + str(len(data)).encode("ascii") + b"\0" + data
                self.assertEqual(hashlib.sha1(raw).hexdigest(), expected_sha)

    def test_v1_schema_still_accepts_only_its_own_work_packet_claim(self):
        receipt = v1_receipt()
        validate_validation_receipt(receipt)
        self.assertEqual(
            parse_validation_receipt_json(json.dumps(receipt))["profile"],
            "work-packet",
        )
        # V1 schema acceptance is NOT native release-profile evidence.
        self.assertNotEqual(receipt["profile"], "release")
        self.assertLess(receipt["completed_stages"], 21)

    def test_v1_forged_pass_wrong_candidate_or_bad_hash_is_rejected(self):
        for key, value in (
            ("validation_status", "fail"),
            ("candidate_branch", "automation/unrelated"),
            ("manifest_sha256", "bad-hash"),
            ("candidate_head_sha", "bad-head"),
        ):
            with self.subTest(field=key):
                receipt = v1_receipt()
                receipt[key] = value
                with self.assertRaises(ValueError):
                    validate_validation_receipt(receipt)

    def test_v2_schema_fixture_accepts_exact_native_release_structure_only(self):
        data = v2_input()
        checked = validate_v2_release_receipt(**data)
        self.assertEqual(checked["contract_version"], 2)
        self.assertEqual(checked["completed_stages"], 21)
        self.assertEqual(checked["profile"], "release")
        self.assertEqual(checked["native_platform"], "windows")
        self.assertEqual(checked["native_validation_host"], "Daniel-CL")

    def test_v1_cannot_cross_promote_as_v2_release_receipt(self):
        data = v2_input()
        data["receipt_json"] = canonical_json_bytes(v1_receipt())
        with self.assertRaises(ValueError):
            validate_v2_release_receipt(**data)

    def test_v2_work_packet_short_stage_or_wrong_host_cannot_pass(self):
        for field, value in (
            ("profile", "work-packet"),
            ("completed_stages", 6),
            ("native_platform", "linux"),
            ("native_validation_host", "CI runner"),
            ("validation_status", "fail"),
        ):
            with self.subTest(field=field):
                data = v2_input()
                receipt = json.loads(data["receipt_json"])
                receipt[field] = value
                data["receipt_json"] = canonical_json_bytes(receipt)
                with self.assertRaises(ValueError):
                    validate_v2_release_receipt(**data)

    def test_v2_receipt_binds_manifest_sidecar_and_candidate_head(self):
        data = v2_input()
        bad = dict(data)
        bad["actual_candidate_sha"] = "f" * 40
        with self.assertRaises(ValueError):
            validate_v2_release_receipt(**bad)
        bad = dict(data)
        bad["sidecar_bytes"] = data["sidecar_bytes"] + b" "
        with self.assertRaises(ValueError):
            validate_v2_release_receipt(**bad)
        bad = dict(data)
        bad["manifest_bytes"] = data["manifest_bytes"] + b" "
        with self.assertRaises(ValueError):
            validate_v2_release_receipt(**bad)

    def test_original_blocked_v1_discovery_is_not_a_v2_accepted_receipt(self):
        original = (ROOT / "docs/upstream-candidates/original-blocked-v1-manifest.json").read_bytes()
        parsed = json.loads(original)
        self.assertEqual(parsed["contract_version"], 1)
        self.assertEqual(parsed["state"], "blocked_transplant_conflict")
        self.assertEqual(parsed["transplant_dry_run"], "conflict")
        self.assertNotIn("native_validation_host", parsed)
        self.assertNotIn("source_snapshot_tree_sha", parsed)

    def test_product_and_upstream_markers_remain_044_and_0162_before_integration(self):
        product = (ROOT / "codex-rs/codexdd-version.txt").read_bytes()
        upstream = (ROOT / "codex-rs/upstream-codex-release.txt").read_bytes()
        self.assertEqual(product, b"0.4.4\n")
        self.assertEqual(upstream, b"rust-v0.162.0\n")

    def test_shipped_policy_contains_no_reviewed_safe_paths_or_write_gate(self):
        self.assertFalse(policy.REVIEWED_SAFE_UPSTREAM_PATHS)
        self.assertIs(preparer.AUTO_PUBLISH_ENABLED, False)
        self.assertIs(draft.DRAFT_PR_WRITES_ENABLED, False)
        self.assertEqual(draft.BASE_BRANCH, "dd/astra-policy-v2")
        self.assertEqual(draft.REPOSITORY, "Takuetsu/codex-dd-astra")

    def test_schedule_is_read_only_and_no_automated_writer_or_draft_step(self):
        sync = (WORKFLOWS / "upstream-sync.yml").read_text(encoding="utf-8")
        self.assertIn("schedule:\n    - cron:", sync)
        self.assertIn("permissions:\n  contents: read", sync)
        self.assertNotIn("contents: write", sync)
        self.assertNotIn("pull-requests: write", sync)
        self.assertIn("upstream_auto_report.py", sync)
        self.assertNotIn("upstream_auto_prepare.py", sync)
        self.assertNotIn("upstream_auto_pr.py", sync)
        self.assertNotIn("gh pr create", sync)
        self.assertNotIn("git push", sync)

    def test_explicit_prepare_and_promotion_keep_separate_privileges(self):
        prep = (WORKFLOWS / "upstream-prepare.yml").read_text(encoding="utf-8")
        promote = (WORKFLOWS / "upstream-promote.yml").read_text(encoding="utf-8")
        self.assertIn("workflow_dispatch:", prep)
        self.assertNotIn("schedule:\n", prep)
        self.assertIn("permissions:\n  contents: write", prep)
        self.assertNotIn("pull-requests: write", prep)
        self.assertIn('git push --force-with-lease="refs/heads/', prep)
        self.assertNotIn("gh pr create", prep)
        self.assertIn("workflow_dispatch:", promote)
        self.assertNotIn("schedule:\n", promote)
        self.assertIn("permissions:\n  contents: read", promote)
        self.assertIn("pull-requests: write", promote)
        self.assertNotIn("contents: write", promote)
        self.assertIn("LOCAL_VALIDATION_PROFILE", promote)
        self.assertIn("semantic_review_accepted", promote)
        self.assertNotIn("gh pr merge", promote)
        self.assertNotIn("git push", promote)

    def test_original_v2_workflows_still_require_owner_and_lease(self):
        publish = (WORKFLOWS / "upstream-reconcile-publish.yml").read_text(encoding="utf-8")
        promote = (WORKFLOWS / "upstream-reconcile-promote.yml").read_text(encoding="utf-8")
        self.assertNotIn("schedule:\n", publish)
        self.assertNotIn("schedule:\n", promote)
        self.assertIn("contents: write", publish)
        self.assertNotIn("pull-requests: write", publish)
        self.assertIn('git push --force-with-lease="refs/heads/', publish)
        self.assertIn("owner", publish.lower())
        self.assertIn("contents: read", promote)
        self.assertIn("pull-requests: write", promote)
        self.assertNotIn("contents: write", promote)
        self.assertIn("native release receipt", promote.lower())
        self.assertIn("owner", promote.lower())
        self.assertNotIn("gh pr merge", promote)
        self.assertNotIn("git push", promote)

    def test_windows_validation_scripts_still_emit_native_candidate_receipts(self):
        v1 = (SCRIPTS / "codexdd-validate-upstream-candidate.ps1").read_text(encoding="utf-8")
        v2 = (SCRIPTS / "codexdd-validate-upstream-candidate-v2.ps1").read_text(encoding="utf-8")
        for version, script in (("v1", v1), ("v2", v2)):
            with self.subTest(version=version):
                self.assertIn("CODEXDD_UPSTREAM_VALIDATION_RECEIPT", script)
                self.assertIn("CODEXDD_VALIDATION_JSON", script)
                self.assertIn("ApprovedDiscoveryManifestSha256", script)
                self.assertIn("ManifestPath", script)
                self.assertNotIn("gh pr create", script)
                self.assertNotIn("git push", script)
        self.assertIn("Daniel-CL", v2)
        self.assertIn("Windows_NT", v2)

    def test_new_draft_adapter_cannot_invoke_historical_promotion_workflows(self):
        adapter_source = (ROOT / ".github/scripts/upstream_auto_pr.py").read_text(
            encoding="utf-8"
        )
        self.assertIn("DRAFT_PR_WRITES_ENABLED = False", adapter_source)
        self.assertIn("--draft", adapter_source)
        self.assertNotIn("upstream-promote.yml", adapter_source)
        self.assertNotIn("upstream-reconcile-promote.yml", adapter_source)
        self.assertNotIn("gh pr merge", adapter_source)
        self.assertNotIn("auto-merge --enable", adapter_source)


if __name__ == "__main__":
    unittest.main()
