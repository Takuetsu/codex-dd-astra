"""Adversarial v2 Windows release receipt and candidate-identity tests."""

import copy
import hashlib
import unittest

from test_upstream_reconciliation_v2 import (
    NEW_TAG, PROD_SHA, REPO, SOURCE_SHA, SOURCE_TREE, TARGET_SHA, fixtures,
)
from upstream_reconciliation import canonical_json_bytes
from upstream_reconciliation_receipt import validate_v2_release_receipt
from upstream_sync import candidate_branch, candidate_key

CANDIDATE = "9" * 40
MANIFEST_PATH = "docs/upstream-candidates/codexdd-upstream.json"


def fixture():
    _, _, manifest, sidecar = fixtures()
    sidecar_data = canonical_json_bytes(sidecar)
    manifest["sidecar_sha256"] = hashlib.sha256(sidecar_data).hexdigest()
    manifest["sidecar_path"] = "docs/upstream-candidates/codexdd-reconciliation.json"
    manifest_data = canonical_json_bytes(manifest)
    return manifest, sidecar, manifest_data, sidecar_data


def receipt_for(manifest, manifest_bytes):
    return {
        "contract_version": 2,
        "receipt_type": "codexdd_upstream_local_validation",
        "candidate_key": candidate_key(NEW_TAG, TARGET_SHA),
        "candidate_branch": candidate_branch(NEW_TAG, TARGET_SHA),
        "candidate_head_sha": CANDIDATE,
        "production_sha": PROD_SHA,
        "target_upstream_tag": NEW_TAG,
        "target_upstream_sha": TARGET_SHA,
        "manifest_path": MANIFEST_PATH,
        "manifest_sha256": hashlib.sha256(manifest_bytes).hexdigest(),
        "reconciliation_sidecar_path": manifest["sidecar_path"],
        "reconciliation_sidecar_sha256": manifest["sidecar_sha256"],
        "source_snapshot_commit_sha": SOURCE_SHA,
        "source_snapshot_tree_sha": SOURCE_TREE,
        "profile": "release",
        "validation_contract_version": 1,
        "validation_status": "pass",
        "completed_stages": 21,
        "completed_at": "2026-10-11T15:10:00.123Z",
        "native_platform": "windows",
        "native_validation_host": "Daniel-CL",
    }


class ReceiptBindingTests(unittest.TestCase):
    def verify(self, change=None, *, manifest=None, sidecar=None, override=None):
        default_manifest, default_sidecar, _, _ = fixture()
        manifest = copy.deepcopy(default_manifest if manifest is None else manifest)
        sidecar = copy.deepcopy(default_sidecar if sidecar is None else sidecar)
        sidecar_data = canonical_json_bytes(sidecar)
        manifest["sidecar_sha256"] = hashlib.sha256(sidecar_data).hexdigest()
        manifest_data = canonical_json_bytes(manifest)
        receipt = receipt_for(manifest, manifest_data)
        if change is not None:
            change(receipt)
        kwargs = {
            "receipt_json": canonical_json_bytes(receipt),
            "manifest_bytes": manifest_data,
            "sidecar_bytes": sidecar_data,
            "expected_manifest_path": MANIFEST_PATH,
            "actual_candidate_sha": CANDIDATE,
        }
        kwargs.update(override or {})
        return validate_v2_release_receipt(**kwargs)

    def test_valid_native_release_receipt_is_schema_accepted_only(self):
        self.assertEqual(self.verify()["completed_stages"], 21)

    def test_work_packet_and_targeted_cannot_substitute_for_release(self):
        for profile in ("work-packet", "targeted", ""):
            with self.subTest(profile=profile), self.assertRaisesRegex(
                ValueError, "mismatch: profile"
            ):
                self.verify(lambda item: item.__setitem__("profile", profile))

    def test_failed_or_forged_validation_status_rejected(self):
        for status in ("fail", "error", None, True):
            with self.subTest(status=status), self.assertRaisesRegex(
                ValueError, "validation_status"
            ):
                self.verify(lambda item: item.__setitem__("validation_status", status))

    def test_short_or_noninteger_native_stage_count_rejected(self):
        for count in (0, 1, 20, True, "21", None):
            with self.subTest(count=count), self.assertRaisesRegex(
                ValueError, "complete native release stages"
            ):
                self.verify(lambda item: item.__setitem__("completed_stages", count))

    def test_wrong_candidate_and_stale_source_rejected(self):
        for field, value in (
            ("candidate_head_sha", "f" * 40),
            ("production_sha", "f" * 40),
            ("source_snapshot_commit_sha", "f" * 40),
            ("source_snapshot_tree_sha", "f" * 40),
        ):
            with self.subTest(field=field), self.assertRaisesRegex(
                ValueError, f"mismatch: {field}"
            ):
                self.verify(lambda item: item.__setitem__(field, value))

    def test_manifest_sidecar_and_branch_sha_bound(self):
        for field, value in (
            ("manifest_sha256", "f" * 64),
            ("reconciliation_sidecar_sha256", "f" * 64),
            ("candidate_branch", "dd/recovery-tooling"),
            ("candidate_key", "forged"),
            ("reconciliation_sidecar_path", "../secret"),
        ):
            with self.subTest(field=field), self.assertRaisesRegex(
                ValueError, "identity/validation mismatch"
            ):
                self.verify(lambda item: item.__setitem__(field, value))

    def test_unknown_or_missing_receipt_fields_rejected(self):
        with self.assertRaisesRegex(ValueError, "unknown fields"):
            self.verify(lambda item: item.__setitem__("approval", "yes"))
        with self.assertRaisesRegex(ValueError, "missing"):
            self.verify(lambda item: item.pop("manifest_sha256"))

    def test_v1_and_relabelled_clean_receipts_cannot_pass(self):
        for change in (
            lambda x: x.__setitem__("contract_version", 1),
            lambda x: x.__setitem__("receipt_type", "other"),
        ):
            with self.assertRaisesRegex(ValueError, "mismatch"):
                self.verify(change)

    def test_unauthenticated_machine_claims_do_not_bypass_host_gate(self):
        for field, value in (
            ("native_platform", "linux"),
            ("native_validation_host", "runner-windows-2022"),
            ("native_validation_host", "Doug"),
        ):
            with self.subTest(field=field), self.assertRaisesRegex(
                ValueError, "identity/validation mismatch"
            ):
                self.verify(lambda item: item.__setitem__(field, value))

    def test_bad_timestamp_rejected(self):
        for value in (
            "2026-10-11T15:10:00-05:00",
            "2026-13-99T00:00:00Z",
            "yesterday",
            "",
        ):
            with self.subTest(value=value), self.assertRaisesRegex(
                ValueError, "timestamp"
            ):
                self.verify(lambda item: item.__setitem__("completed_at", value))

    def test_original_v1_manifest_cannot_masquerade_as_v2_candidate(self):
        manifest, sidecar, _, _ = fixture()
        manifest["contract_version"] = 1
        with self.assertRaisesRegex(ValueError, "cannot promote"):
            self.verify(manifest=manifest, sidecar=sidecar)
        manifest["contract_version"] = 2
        manifest["state"] = "preparation_ready"
        with self.assertRaisesRegex(ValueError, "cannot promote"):
            self.verify(manifest=manifest, sidecar=sidecar)

    def test_noncanonical_receipt_and_duplicate_json_field_rejected(self):
        valid = self.verify()
        data = canonical_json_bytes(valid)
        with self.assertRaisesRegex(ValueError, "not canonical"):
            self.verify(override={"receipt_json": data.replace(b'":', b'" :', 1)})
        dup = data.replace(b'"contract_version":2', b'"contract_version":2,"contract_version":2')
        with self.assertRaisesRegex(ValueError, "duplicate"):
            self.verify(override={"receipt_json": dup})

    def test_corrupted_sidecar_bytes_are_rejected(self):
        _, _, _, raw = fixture()
        corrupted = raw.replace(b'conflict_reconciliation', b'other_attestation')
        with self.assertRaisesRegex(ValueError, "checksum"):
            self.verify(override={"sidecar_bytes": corrupted})

    def test_different_observed_candidate_head_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "candidate_head_sha"):
            self.verify(override={"actual_candidate_sha": "a" * 40})


if __name__ == "__main__":
    unittest.main()
