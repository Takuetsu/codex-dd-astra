"""Synthetic release-specific v2 contract checks, never actual native evidence."""
from __future__ import annotations

import copy
import hashlib
import json
import unittest
from pathlib import Path

from test_upstream_reconciliation_v2 import fixtures
from test_upstream_reconciliation_receipt import CANDIDATE, MANIFEST_PATH, fixture as receipt_fixture, receipt_for
from upstream_reconciliation import canonical_json_bytes
from upstream_reconciliation_git import verify_source_snapshot_git
from upstream_reconciliation_promotion import validate_v2_owner_review_and_receipt
from upstream_reconciliation_release_targets import reviewed_reconciliation_target
from upstream_reconciliation_v2 import validate_reconciled_v2_documents
from upstream_sync import candidate_branch, candidate_key

NEW_TAG = "rust-v0.163.0"
NEW_SHA = "83086188ae0614bd882818ff1aa23f3194962012"
OLD_TAG = "rust-v0.162.0"
OLD_SHA = "c1382380de69521303b416720a52f42d51af6248"
NEW_BRANCH = "dd/codexdd-v0.4.5-upstream-0.163.0-integration"
OLD_BRANCH = "dd/codexdd-v0.4.3-upstream-0.162.0-integration"


def new_v2_fixture():
    original_data, conflicts_data, manifest, sidecar = fixtures()
    original = json.loads(original_data)
    original.update({
        "codexdd_product_version": "0.4.4",
        "tracked_upstream_tag": OLD_TAG,
        "target_upstream_tag": NEW_TAG,
        "target_upstream_sha": NEW_SHA,
    })
    original_data = canonical_json_bytes(original)
    manifest.update({
        "codexdd_product_version": "0.4.5",
        "tracked_upstream_tag": OLD_TAG,
        "target_upstream_tag": NEW_TAG,
        "target_upstream_sha": NEW_SHA,
        "candidate_key": candidate_key(NEW_TAG, NEW_SHA),
        "candidate_branch": candidate_branch(NEW_TAG, NEW_SHA),
        "discovery_manifest_sha256": hashlib.sha256(original_data).hexdigest(),
    })
    sidecar["candidate_key"] = manifest["candidate_key"]
    sidecar["tracked_upstream"]["tag"] = OLD_TAG
    sidecar["target_upstream"]["tag"] = NEW_TAG
    sidecar["target_upstream"]["sha"] = NEW_SHA
    sidecar["source_snapshot"]["branch"] = NEW_BRANCH
    sidecar["discovery"]["artifact_name"] = "upstream-candidate-rust-v0.163.0-37960772252"
    sidecar["discovery"]["manifest_sha256"] = manifest["discovery_manifest_sha256"]
    raw_sidecar = canonical_json_bytes(sidecar)
    manifest["sidecar_sha256"] = hashlib.sha256(raw_sidecar).hexdigest()
    return original_data, conflicts_data, manifest, sidecar


def verify_045_v2(*, change_sidecar=None):
    original, conflicts, manifest, sidecar = new_v2_fixture()
    if change_sidecar:
        change_sidecar(sidecar)
    sidecar_raw = canonical_json_bytes(sidecar)
    manifest["sidecar_sha256"] = hashlib.sha256(sidecar_raw).hexdigest()
    return validate_reconciled_v2_documents(
        original_manifest_bytes=original,
        original_conflicts_bytes=conflicts,
        approved_manifest_sha256=hashlib.sha256(original).hexdigest(),
        approved_conflicts_sha256=hashlib.sha256(conflicts).hexdigest(),
        reconciled_manifest_bytes=canonical_json_bytes(manifest),
        reconciliation_sidecar_bytes=sidecar_raw,
        expected_production_sha=manifest["production_sha"],
        expected_target_tag=NEW_TAG,
        expected_target_sha=NEW_SHA,
        expected_source_snapshot_sha=manifest["source_snapshot_commit_sha"],
        expected_source_snapshot_tree=manifest["source_snapshot_tree_sha"],
    )


def promotion_fixture_045():
    manifest, sidecar, _, _ = receipt_fixture()
    manifest = copy.deepcopy(manifest)
    sidecar = copy.deepcopy(sidecar)
    manifest.update({
        "target_upstream_tag": NEW_TAG,
        "target_upstream_sha": NEW_SHA,
        "codexdd_product_version": "0.4.5",
        "candidate_key": candidate_key(NEW_TAG, NEW_SHA),
        "candidate_branch": candidate_branch(NEW_TAG, NEW_SHA),
    })
    sidecar["target_upstream"]["tag"] = NEW_TAG
    sidecar["target_upstream"]["sha"] = NEW_SHA
    sidecar["candidate_key"] = manifest["candidate_key"]
    sidecar["source_snapshot"]["branch"] = NEW_BRANCH
    return manifest, sidecar


def validate_promotion_045(*, product=None, upstream_sha=None, accept=None, stages=21):
    manifest, sidecar = promotion_fixture_045()
    if product is not None:
        manifest["codexdd_product_version"] = product
    if upstream_sha is not None:
        manifest["target_upstream_sha"] = upstream_sha
        sidecar["target_upstream"]["sha"] = upstream_sha
    raw_sidecar = canonical_json_bytes(sidecar)
    manifest["sidecar_sha256"] = hashlib.sha256(raw_sidecar).hexdigest()
    raw_manifest = canonical_json_bytes(manifest)
    receipt = receipt_for(manifest, raw_manifest)
    receipt.update({
        "target_upstream_tag": manifest["target_upstream_tag"],
        "target_upstream_sha": manifest["target_upstream_sha"],
        "candidate_key": manifest["candidate_key"],
        "candidate_branch": manifest["candidate_branch"],
        "completed_stages": stages,
    })
    return validate_v2_owner_review_and_receipt(
        manifest_bytes=raw_manifest,
        sidecar_bytes=raw_sidecar,
        receipt_bytes=canonical_json_bytes(receipt),
        observed_candidate_branch=manifest["candidate_branch"],
        observed_candidate_sha=CANDIDATE,
        manifest_path=MANIFEST_PATH,
        semantic_review_url=sidecar["semantic_review_url"],
        owner_source_acceptance=(
            accept if accept is not None
            else "ACCEPT_CODEXDD_0.4.5_RECONCILED_" + manifest["source_snapshot_commit_sha"]
        ),
    )


class ReviewedTarget045Tests(unittest.TestCase):
    def test_exact_legacy_and_new_reviewed_targets(self):
        old = reviewed_reconciliation_target(OLD_TAG)
        new = reviewed_reconciliation_target(NEW_TAG)
        self.assertEqual((old.target_sha, old.product_version, old.source_branch), (OLD_SHA, "0.4.3", OLD_BRANCH))
        self.assertEqual((new.target_sha, new.product_version, new.source_branch), (NEW_SHA, "0.4.5", NEW_BRANCH))
        for invalid in ("rust-v0.164.0", "", True, None):
            with self.subTest(invalid=invalid), self.assertRaises(ValueError):
                reviewed_reconciliation_target(invalid)

    def test_new_v2_manual_conflict_source_is_bound(self):
        manifest, sidecar = verify_045_v2()
        self.assertEqual(manifest["state"], "manual_conflict_reconciled_review_required")
        self.assertEqual(sidecar["source_snapshot"]["branch"], NEW_BRANCH)

    def test_old_source_branch_cannot_be_reused_for_045(self):
        with self.assertRaisesRegex(ValueError, "unexpected source-development branch"):
            verify_045_v2(change_sidecar=lambda s: s["source_snapshot"].__setitem__("branch", OLD_BRANCH))

    def test_genuine_git_source_ref_must_match_release_policy(self):
        with self.assertRaisesRegex(ValueError, "source ref is for another release"):
            verify_source_snapshot_git(Path("."), {"target_upstream_tag": NEW_TAG}, source_ref="refs/heads/" + OLD_BRANCH)
        with self.assertRaisesRegex(ValueError, "untrusted release verification Git ref"):
            verify_source_snapshot_git(Path("."), {"target_upstream_tag": NEW_TAG}, source_ref="refs/heads/dd/unreviewed")

    def test_new_owner_receipt_contract_synthetic_pass(self):
        receipt = validate_promotion_045()
        self.assertEqual(receipt["profile"], "release")
        self.assertEqual(receipt["completed_stages"], 21)

    def test_historical_owner_token_never_accepts_045(self):
        with self.assertRaisesRegex(ValueError, "owner acceptance"):
            validate_promotion_045(accept="ACCEPT_CODEXDD_0.4.3_RECONCILED_" + ("a" * 40))

    def test_product_target_and_native_stage_denials(self):
        for product in ("0.4.3", "0.4.6"):
            with self.subTest(product=product), self.assertRaisesRegex(ValueError, "product version"):
                validate_promotion_045(product=product)
        with self.assertRaisesRegex(ValueError, "pinned upstream target"):
            validate_promotion_045(upstream_sha="f" * 40)
        with self.assertRaisesRegex(ValueError, "complete native release stages"):
            validate_promotion_045(stages=20)


if __name__ == "__main__":
    unittest.main()
