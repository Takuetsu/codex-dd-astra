"""Promotion receipt gate tests: no credentials, GitHub writes or internet."""

import copy
import hashlib
import unittest

from test_upstream_reconciliation_receipt import (
    CANDIDATE,
    MANIFEST_PATH,
    fixture,
    receipt_for,
)
from test_upstream_reconciliation_v2 import REPO, SOURCE_SHA
from upstream_reconciliation import canonical_json_bytes
from upstream_reconciliation_promotion import (
    validate_v2_owner_review_and_receipt,
    normalize_console_or_file_receipt_json,
)

from upstream_sync import candidate_branch, candidate_key

PINNED = "c1382380de69521303b416720a52f42d51af6248"
ACCEPT = "ACCEPT_CODEXDD_0.4.3_RECONCILED_" + SOURCE_SHA


class OwnerPromotionGateTests(unittest.TestCase):
    def verify(self, *, change_receipt=None, change_manifest=None,
               change_sidecar=None, semantic_url=None, accept=ACCEPT, observed=None):
        manifest, sidecar, _, _ = fixture()
        manifest = copy.deepcopy(manifest)
        sidecar = copy.deepcopy(sidecar)
        # Base fixture uses synthetic 40-hex target identities. The promotion
        # gate intentionally allows ONLY the real owner-pinned 0.162.0 target.
        manifest["target_upstream_sha"] = PINNED
        manifest["candidate_key"] = candidate_key("rust-v0.162.0", PINNED)
        manifest["candidate_branch"] = candidate_branch("rust-v0.162.0", PINNED)
        sidecar["candidate_key"] = manifest["candidate_key"]
        sidecar["target_upstream"]["sha"] = PINNED
        if change_manifest is not None:
            change_manifest(manifest)
        if change_sidecar is not None:
            change_sidecar(sidecar)
        raw_sidecar = canonical_json_bytes(sidecar)
        manifest["sidecar_sha256"] = hashlib.sha256(raw_sidecar).hexdigest()
        raw_manifest = canonical_json_bytes(manifest)
        receipt = receipt_for(manifest, raw_manifest)
        receipt["target_upstream_sha"] = manifest["target_upstream_sha"]
        receipt["candidate_key"] = manifest["candidate_key"]
        receipt["candidate_branch"] = manifest["candidate_branch"]
        if change_receipt is not None:
            change_receipt(receipt)
        return validate_v2_owner_review_and_receipt(
            manifest_bytes=raw_manifest,
            sidecar_bytes=raw_sidecar,
            receipt_bytes=canonical_json_bytes(receipt),
            observed_candidate_branch=manifest["candidate_branch"],
            observed_candidate_sha=CANDIDATE if observed is None else observed,
            manifest_path=MANIFEST_PATH,
            semantic_review_url=sidecar["semantic_review_url"] if semantic_url is None else semantic_url,
            owner_source_acceptance=accept,
        )

    def test_positive_v2_owner_review_and_native_release_receipt(self):
        self.assertEqual(self.verify()["profile"], "release")

    def test_console_receipt_copy_is_canonicalized_to_single_final_lf(self):
        raw = b'{"contract_version":2}\n'
        self.assertEqual(
            normalize_console_or_file_receipt_json(raw.decode().rstrip("\n")), raw
        )
        self.assertEqual(normalize_console_or_file_receipt_json(raw.decode()), raw)
        for invalid in ('', '{}\n\n', '{}\r\n', '{\n}', '  '):
            with self.subTest(invalid=invalid):
                if invalid == '  ':
                    self.assertEqual(normalize_console_or_file_receipt_json(invalid), b'  \n')
                else:
                    with self.assertRaises(ValueError):
                        normalize_console_or_file_receipt_json(invalid)



    def test_owner_acceptance_cannot_be_missing_or_for_different_source(self):
        for value in ("", "yes", ACCEPT[:-1], "ACCEPT_CODEXDD_0.4.3_RECONCILED_" + ("f"*40)):
            with self.subTest(value=value), self.assertRaisesRegex(ValueError, "owner acceptance"):
                self.verify(accept=value)

    def test_semantic_link_must_match_sidecar_and_same_repository(self):
        with self.assertRaisesRegex(ValueError, "review link"):
            self.verify(semantic_url=REPO+"commit/"+("f"*40))
        with self.assertRaisesRegex(ValueError, "same-repository"):
            self.verify(semantic_url="https://example.com/attest")

    def test_receipt_workpacket_cannot_enable_review_promotion(self):
        with self.assertRaisesRegex(ValueError, "mismatch: profile"):
            self.verify(change_receipt=lambda r:r.__setitem__("profile","work-packet"))

    def test_receipt_incomplete_or_forged_platform_rejected(self):
        for field,value in (("completed_stages",10),("validation_status","fail"),
                            ("native_platform","linux"),("native_validation_host","CI")):
            with self.subTest(field=field), self.assertRaises(ValueError):
                self.verify(change_receipt=lambda r:r.__setitem__(field,value))

    def test_stale_candidate_head_rejected(self):
        with self.assertRaisesRegex(ValueError, "candidate_head_sha"):
            self.verify(observed="f"*40)

    def test_source_or_target_mutation_fails_closed(self):
        with self.assertRaises(ValueError):
            self.verify(change_manifest=lambda m:m.__setitem__("target_upstream_sha","f"*40))
        with self.assertRaises(ValueError):
            self.verify(change_manifest=lambda m:m.__setitem__("source_snapshot_commit_sha","f"*40))

    def test_unknown_receipt_fields_rejected(self):
        with self.assertRaisesRegex(ValueError, "unknown"):
            self.verify(change_receipt=lambda r:r.__setitem__("extra","yes"))

    def test_v1_manifest_cannot_promote_through_v2(self):
        with self.assertRaisesRegex(ValueError, "cannot promote v1"):
            self.verify(change_manifest=lambda m:m.__setitem__("contract_version",1))

    def test_claimed_review_and_attested_review_must_be_identical(self):
        with self.assertRaisesRegex(ValueError, "review link"):
            self.verify(change_sidecar=lambda s:s.__setitem__("semantic_review_url",REPO+"commit/"+("f"*40)),
                        semantic_url=REPO+"commit/"+SOURCE_SHA)


if __name__=="__main__":
    unittest.main()
