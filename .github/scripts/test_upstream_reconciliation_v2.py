"""Adversarial schema/binding tests for the separately gated v2 recovery lane."""

from __future__ import annotations

import copy
import hashlib
import unittest

from upstream_reconciliation import canonical_json_bytes
from upstream_reconciliation_v2 import validate_reconciled_v2_documents
from upstream_sync import candidate_branch, candidate_key

REPO = "https://github.com/Takuetsu/codex-dd-astra/"
SOURCE_SHA = "a" * 40
SOURCE_TREE = "b" * 40
PROD_SHA = "1" * 40
TARGET_SHA = "2" * 40
TRACKED_SHA = "3" * 40
TRACKED_TREE = "4" * 40
TARGET_TREE = "5" * 40
PROD_TREE = "6" * 40
TRACKED_TAG_OBJECT = "7" * 40
TARGET_TAG_OBJECT = "8" * 40
GENERATED = "codex-rs/app-server-protocol/schema/typescript/ClientRequest.ts"
OVERLAPS = [
    GENERATED,
    "codex-rs/core/src/config/mod.rs",
    "codex-rs/tui/src/app_server_session.rs",
]
CONFLICTS = [GENERATED, "codex-rs/tui/src/app_server_session.rs"]
OLD_TAG = "rust-v0.159.2"
NEW_TAG = "rust-v0.162.0"
EVIDENCE = REPO + "commit/" + SOURCE_SHA


def fixtures():
    original = {
        "contract_version": 1,
        "state": "blocked_transplant_conflict",
        "transplant_dry_run": "conflict",
        "production_sha": PROD_SHA,
        "production_tree": PROD_TREE,
        "codexdd_product_version": "0.4.2",
        "tracked_upstream_tag": OLD_TAG,
        "tracked_upstream_sha": TRACKED_SHA,
        "tracked_upstream_tree": TRACKED_TREE,
        "target_upstream_tag": NEW_TAG,
        "target_upstream_sha": TARGET_SHA,
        "target_upstream_tree": TARGET_TREE,
        "overlap_path_count": len(OVERLAPS),
        "overlap_paths": OVERLAPS,
        "sensitive_overlap_categories": {
            "app_server_runtime": [GENERATED],
            "core_runtime": ["codex-rs/core/src/config/mod.rs"],
            "tui_runtime": ["codex-rs/tui/src/app_server_session.rs"],
        },
    }
    original_data = canonical_json_bytes(original)
    conflicts_data = ("\n".join(CONFLICTS) + "\n").encode("utf-8")
    discovery_sha = hashlib.sha256(original_data).hexdigest()
    conflicts_sha = hashlib.sha256(conflicts_data).hexdigest()
    key = candidate_key(NEW_TAG, TARGET_SHA)
    manifest = {
        "contract_version": 2,
        "state": "manual_conflict_reconciled_review_required",
        "transplant_dry_run": "conflict",
        "candidate_key": key,
        "candidate_branch": candidate_branch(NEW_TAG, TARGET_SHA),
        "production_branch": "dd/astra-policy-v2",
        "production_sha": PROD_SHA,
        "production_tree": PROD_TREE,
        "codexdd_product_version": "0.4.3",
        "tracked_upstream_tag": OLD_TAG,
        "tracked_upstream_tag_object_sha": TRACKED_TAG_OBJECT,
        "tracked_upstream_sha": TRACKED_SHA,
        "tracked_upstream_tree": TRACKED_TREE,
        "target_upstream_tag": NEW_TAG,
        "target_upstream_tag_object_sha": TARGET_TAG_OBJECT,
        "target_upstream_sha": TARGET_SHA,
        "target_upstream_tree": TARGET_TREE,
        "discovery_run_id": 37960772252,
        "discovery_manifest_sha256": discovery_sha,
        "discovery_conflicts_sha256": conflicts_sha,
        "overlap_paths": OVERLAPS,
        "sensitive_overlap_categories": original["sensitive_overlap_categories"],
        "source_snapshot_commit_sha": SOURCE_SHA,
        "source_snapshot_tree_sha": SOURCE_TREE,
        "sidecar_path": "docs/upstream-candidates/codexdd-reconciliation.json",
        "sidecar_sha256": "f" * 64,
    }
    def resolution(path):
        return {
            "path": path,
            "owner_packet": "3B.4",
            "strategy": "generator_rebuilt" if path == GENERATED else "manual_merge",
            "review_sha": SOURCE_SHA,
            "test_evidence_url": EVIDENCE,
            "semantic_evidence_url": EVIDENCE,
        }
    def overlap(path):
        categories = sorted(k for k, value in original["sensitive_overlap_categories"].items()
                            if path in value)
        return {
            "path": path,
            "owner_packet": "3B.4",
            "disposition": "generator_rebuilt" if path == GENERATED else "merged",
            "sensitivity_categories": categories,
            "test_evidence_url": EVIDENCE,
            "semantic_evidence_url": EVIDENCE,
        }
    sidecar = {
        "attestation_type": "codexdd_upstream_conflict_reconciliation",
        "contract_version": 1,
        "candidate_key": key,
        "discovery": {
            "run_id": 37960772252,
            "artifact_name": "upstream-candidate-rust-v0.162.0-37960772252",
            "manifest_sha256": discovery_sha,
            "conflicts_sha256": conflicts_sha,
        },
        "production": {"branch": "dd/astra-policy-v2", "sha": PROD_SHA, "tree": PROD_TREE},
        "tracked_upstream": {
            "tag": OLD_TAG, "tag_object_sha": TRACKED_TAG_OBJECT,
            "sha": TRACKED_SHA, "tree": TRACKED_TREE,
        },
        "target_upstream": {
            "tag": NEW_TAG, "tag_object_sha": TARGET_TAG_OBJECT,
            "sha": TARGET_SHA, "tree": TARGET_TREE,
        },
        "source_snapshot": {
            "branch": "dd/codexdd-v0.4.3-upstream-0.162.0-integration",
            "commit_sha": SOURCE_SHA,
            "tree_sha": SOURCE_TREE,
        },
        "conflict_resolutions": [resolution(path) for path in CONFLICTS],
        "overlap_reviews": [overlap(path) for path in OVERLAPS],
        "generated_artifacts": [
            {
                "path": GENERATED,
                "generator_command": "cargo run -p codex-app-server-protocol -- generate",
                "generator_source_sha": SOURCE_SHA,
                "drift_evidence_url": EVIDENCE,
            }
        ],
        "security_reviews": {
            name: {"source_snapshot_commit_sha": SOURCE_SHA, "evidence_url": EVIDENCE}
            for name in (
                "windows_elevated_f2", "adaptive_model_routing", "worker_scope_and_lifecycle",
                "lvo_receipt", "updater_fail_closed", "resume_fork",
            )
        },
        "semantic_review_url": EVIDENCE,
    }
    return original_data, conflicts_data, manifest, sidecar


class CrossBoundReconciliationTests(unittest.TestCase):
    def verify(self, *, manifest=None, sidecar=None, original=None, conflicts=None, extra=None):
        original_data, conflicts_data, default_manifest, default_sidecar = fixtures()
        if original is not None:
            original_data = original
        if conflicts is not None:
            conflicts_data = conflicts
        manifest = copy.deepcopy(default_manifest if manifest is None else manifest)
        sidecar = copy.deepcopy(default_sidecar if sidecar is None else sidecar)
        manifest["sidecar_sha256"] = hashlib.sha256(
            canonical_json_bytes(sidecar)
        ).hexdigest()
        args = {
            "original_manifest_bytes": original_data,
            "original_conflicts_bytes": conflicts_data,
            "approved_manifest_sha256": hashlib.sha256(fixtures()[0]).hexdigest(),
            "approved_conflicts_sha256": hashlib.sha256(fixtures()[1]).hexdigest(),
            "reconciled_manifest_bytes": canonical_json_bytes(manifest),
            "reconciliation_sidecar_bytes": canonical_json_bytes(sidecar),
            "expected_production_sha": PROD_SHA,
            "expected_target_tag": NEW_TAG,
            "expected_target_sha": TARGET_SHA,
            "expected_source_snapshot_sha": SOURCE_SHA,
            "expected_source_snapshot_tree": SOURCE_TREE,
        }
        args.update(extra or {})
        return validate_reconciled_v2_documents(**args)

    def test_valid_bound_documents_pass_schemas_without_granting_publication(self):
        manifest, sidecar = self.verify()
        self.assertEqual(manifest["contract_version"], 2)
        self.assertEqual(len(sidecar["conflict_resolutions"]), 2)
        self.assertEqual(len(sidecar["overlap_reviews"]), 3)

    def test_original_blocked_discovery_cannot_be_relabeled(self):
        original_data, _, _, _ = fixtures()
        original = __import__("json").loads(original_data)
        original["state"] = "preparation_ready"
        relabeled = canonical_json_bytes(original)
        with self.assertRaisesRegex(ValueError, "immutably blocked"):
            self.verify(
                original=relabeled,
                extra={"approved_manifest_sha256": hashlib.sha256(relabeled).hexdigest()},
            )

    def test_wrong_original_digest_rejected_even_when_reconciled_pair_matches(self):
        with self.assertRaisesRegex(ValueError, "digest"):
            self.verify(extra={"approved_manifest_sha256": "f" * 64})
        with self.assertRaisesRegex(ValueError, "digest"):
            self.verify(extra={"approved_conflicts_sha256": "f" * 64})

    def test_v1_clean_only_contract_does_not_masquerade_as_v2(self):
        _, _, m, s = fixtures()
        for field, changed in (
            ("contract_version", 1),
            ("state", "manual_semantic_review_required"),
            ("transplant_dry_run", "clean"),
        ):
            broken = copy.deepcopy(m)
            broken[field] = changed
            with self.subTest(field=field), self.assertRaisesRegex(
                ValueError, "binding mismatch"
            ):
                self.verify(manifest=broken)
        s["contract_version"] = 2
        with self.assertRaisesRegex(ValueError, "binding mismatch"):
            self.verify(sidecar=s)

    def test_original_conflict_reordering_or_removal_rejected(self):
        original_data, _, _, _ = fixtures()
        for report in (
            ("\n".join(reversed(CONFLICTS)) + "\n").encode(),
            (CONFLICTS[0] + "\n").encode(),
        ):
            with self.subTest(report=report), self.assertRaisesRegex(ValueError, "digest"):
                self.verify(original=original_data, conflicts=report)

    def test_unlisted_or_duplicate_conflict_resolution_is_rejected(self):
        _, _, _, sidecar = fixtures()
        for changes in (
            [sidecar["conflict_resolutions"][0]],
            list(reversed(sidecar["conflict_resolutions"])),
            [sidecar["conflict_resolutions"][0]] * 2,
        ):
            revised = copy.deepcopy(sidecar)
            revised["conflict_resolutions"] = changes
            with self.subTest(changes=changes), self.assertRaises(ValueError):
                self.verify(sidecar=revised)

    def test_overlap_missing_reordered_or_duplicate_rejected(self):
        _, _, _, s = fixtures()
        for overlap_records in (
            s["overlap_reviews"][:2],
            list(reversed(s["overlap_reviews"])),
            [s["overlap_reviews"][0]] * 3,
        ):
            altered = copy.deepcopy(s)
            altered["overlap_reviews"] = overlap_records
            with self.subTest(overlap_records=overlap_records), self.assertRaises(ValueError):
                self.verify(sidecar=altered)

    def test_sensitive_classification_cannot_be_omitted_or_relabeled(self):
        _, _, _, s = fixtures()
        s["overlap_reviews"][1]["sensitivity_categories"] = ["manual_high_risk"]
        with self.assertRaisesRegex(ValueError, "drops an original"):
            self.verify(sidecar=s)

    def test_generated_artifact_requires_complete_provenance(self):
        _, _, _, s = fixtures()
        s["generated_artifacts"] = []
        with self.assertRaisesRegex(ValueError, "generated artifact provenance"):
            self.verify(sidecar=s)
        _, _, _, s = fixtures()
        s["generated_artifacts"][0]["generator_source_sha"] = "f" * 40
        with self.assertRaisesRegex(ValueError, "generated artifact snapshot"):
            self.verify(sidecar=s)

    def test_source_and_production_identity_binding(self):
        _, _, m, s = fixtures()
        for field, value in (
            ("production_sha", "f" * 40),
            ("source_snapshot_commit_sha", "f" * 40),
            ("source_snapshot_tree_sha", "f" * 40),
            ("target_upstream_sha", "f" * 40),
            ("tracked_upstream_tree", "f" * 40),
        ):
            altered = copy.deepcopy(m)
            altered[field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                self.verify(manifest=altered)
        s["source_snapshot"]["commit_sha"] = "f" * 40
        with self.assertRaisesRegex(ValueError, "source commit"):
            self.verify(sidecar=s)

    def test_canonical_sidecar_hash_must_match(self):
        _, _, m, s = fixtures()
        altered = copy.deepcopy(s)
        altered["semantic_review_url"] = REPO + "commit/" + ("b" * 40)
        blob = canonical_json_bytes(altered)
        with self.assertRaisesRegex(ValueError, "sidecar digest"):
            self.verify(
                manifest=m,
                sidecar=s,
                extra={"reconciliation_sidecar_bytes": blob},
            )

    def test_unknown_manifest_or_sidecar_fields_fail_closed(self):
        _, _, m, s = fixtures()
        m["unexpected"] = "allow"
        with self.assertRaisesRegex(ValueError, "unknown fields"):
            self.verify(manifest=m)
        _, _, m, s = fixtures()
        s["unexpected"] = "allow"
        with self.assertRaisesRegex(ValueError, "unknown fields"):
            self.verify(sidecar=s)
        _, _, m, s = fixtures()
        s["production"]["unexpected"] = "allow"
        with self.assertRaisesRegex(ValueError, "unknown fields"):
            self.verify(sidecar=s)

    def test_product_release_and_canonical_candidate_are_bound(self):
        _, _, m, _ = fixtures()
        m["codexdd_product_version"] = "0.4.2"
        with self.assertRaisesRegex(ValueError, "explicitly advanced"):
            self.verify(manifest=m)
        _, _, m, _ = fixtures()
        m["candidate_branch"] = "dd/codexdd-v0.4.3-upstream-0.162.0-integration"
        with self.assertRaisesRegex(ValueError, "candidate branch"):
            self.verify(manifest=m)

    def test_missing_owner_and_bad_review_links_fail_closed(self):
        _, _, _, s = fixtures()
        s["conflict_resolutions"][0]["owner_packet"] = ""
        with self.assertRaisesRegex(ValueError, "packet owner"):
            self.verify(sidecar=s)
        _, _, _, s = fixtures()
        s["semantic_review_url"] = "https://example.org/review"
        with self.assertRaisesRegex(ValueError, "same-repository"):
            self.verify(sidecar=s)
        _, _, _, s = fixtures()
        s["security_reviews"]["windows_elevated_f2"]["source_snapshot_commit_sha"] = "f" * 40
        with self.assertRaisesRegex(ValueError, "security review"):
            self.verify(sidecar=s)

    def test_discovery_run_mismatch_fails_closed(self):
        _, _, m, s = fixtures()
        s["discovery"]["run_id"] += 1
        with self.assertRaisesRegex(ValueError, "discovery run ID"):
            self.verify(sidecar=s)
        m["discovery_run_id"] = True
        with self.assertRaises(ValueError):
            self.verify(manifest=m)

    def test_noncanonical_json_is_rejected_by_parser(self):
        _, _, m, s = fixtures()
        raw = canonical_json_bytes(m).replace(b'":', b'" :', 1)
        with self.assertRaisesRegex(ValueError, "not canonical"):
            self.verify(extra={"reconciled_manifest_bytes": raw})


if __name__ == "__main__":
    unittest.main()
