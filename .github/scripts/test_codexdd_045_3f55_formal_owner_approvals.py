"""3F.55: formal 109-path owner decisions require accountable evidence and signoff.

Source provenance is checked against actual Git objects. Merely preparing a
review is never a release approval or an owner acceptance.
"""
from __future__ import annotations
import json
import re
import subprocess
import unittest
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / "docs/upstream-candidates"
LEDGER = BASE / "codexdd-045-3f55-formal-owner-approvals.json"
MATRIX = BASE / "codexdd-045-109-semantic-owner-matrix.json"
UPSTREAM = BASE / "codexdd-upstream-rust-v0.163.0-overlap-source-receipt.json"
VALID = {"PENDING_SIGNOFF", "OWNER_ACCEPTED"}

class Formal109OwnerApprovals(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.ledger = json.loads(LEDGER.read_text(encoding="utf-8"))
        cls.matrix = json.loads(MATRIX.read_text(encoding="utf-8"))
        cls.overlap = json.loads(UPSTREAM.read_text(encoding="utf-8"))

    def test_all_109_paths_exactly_once_and_owner_roster_preserved(self):
        d = self.ledger
        self.assertEqual(d["contract_version"], 1)
        self.assertEqual(d["scope"], "FORMAL_REVIEW_OPEN_NOT_AUTOMATIC_OWNER_APPROVAL")
        self.assertEqual(d["source_sha"], "72cd09d3f52c076d445078758af20489ccdb4e74")
        self.assertEqual(d["upstream_ref"], "openai/codex@rust-v0.163.0")
        self.assertFalse(d["release_approved"])
        self.assertEqual(d["required_owner_count"], 109)
        self.assertEqual(len(d["paths"]), 109)
        p = {r["path"]: r for r in d["paths"]}
        m = {r["path"]: r for r in self.matrix["paths"]}
        u = {r["path"]: r for r in self.overlap["overlaps"]}
        self.assertEqual(len(p), len(m))
        self.assertEqual(set(p), set(m))
        self.assertEqual(set(p), set(u))
        self.assertEqual(Counter(x["owner"] for x in p.values()), self.matrix["owner_counts"])
        self.assertEqual(sum(x["original_direct_conflict"] for x in p.values()), 15)
        self.assertEqual(self.matrix["release_status"], "NOT_ACCEPTED")

    def test_each_path_is_source_pinned_with_explicit_blocker(self):
        m = {r["path"]: r for r in self.matrix["paths"]}
        u = {r["path"]: r for r in self.overlap["overlaps"]}
        for r in self.ledger["paths"]:
            with self.subTest(path=r["path"]):
                p = r["path"]
                self.assertEqual(r["candidate_blob"], m[p]["current_blob"])
                self.assertEqual(r["official_upstream_0163_blob"], u[p]["official_163_blob"])
                self.assertEqual(r["original_direct_conflict"], m[p]["original_direct_conflict"])
                self.assertEqual(r["owner"], m[p]["owner"])
                self.assertRegex(r["candidate_blob"], r"^[0-9a-f]{40}$")
                self.assertRegex(r["official_upstream_0163_blob"], r"^[0-9a-f]{40}$")
                self.assertTrue(r["required_evidence"])
                self.assertTrue(r["proposed_disposition"])
                self.assertEqual(r["release_status"], "NOT_ACCEPTED")
                self.assertIn(r["formal_owner_decision"], VALID)
                actual = subprocess.check_output(
                    ["git", "rev-parse", "HEAD:" + p], cwd=ROOT, text=True
                ).strip()
                self.assertEqual(actual, r["candidate_blob"], "Owner review source blob drift: " + p)
                if r["original_direct_conflict"]:
                    self.assertTrue(m[p]["original_conflict_gate"])

    def test_signoff_requires_distinct_approver_and_per_path_receipt(self):
        accepted = []
        for row in self.ledger["paths"]:
            if row["formal_owner_decision"] == "OWNER_ACCEPTED":
                accepted.append(row["path"])
                self.assertRegex(row["approved_by"], r"^[A-Za-z][A-Za-z0-9_.@/-]{2,100}$")
                self.assertRegex(row["approval_utc"], r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$")
                self.assertTrue(row["evidence"], "No evidence: " + row["path"])
                self.assertEqual(row["source_review_status"], "SOURCE_COMPARISON_AND_EVIDENCE_ATTACHED")
                self.assertEqual(row["release_status"], "NOT_ACCEPTED")
            else:
                self.assertIsNone(row["approved_by"])
                self.assertIsNone(row["approval_utc"])
        self.assertEqual(self.ledger["approved_owner_count"], len(accepted))
        self.assertEqual(
            self.ledger["ready_for_formal_review_count"],
            sum(r["source_review_status"] == "SOURCE_COMPARISON_AND_EVIDENCE_ATTACHED" for r in self.ledger["paths"]),
        )
        self.assertEqual(
            self.ledger["source_review_queued_count"],
            sum(r["source_review_status"] == "SOURCE_DIFF_REVIEW_QUEUED" for r in self.ledger["paths"]),
        )
        self.assertEqual(
            self.ledger["required_owner_count"],
            self.ledger["ready_for_formal_review_count"] + self.ledger["source_review_queued_count"],
        )
        self.assertEqual(self.ledger["approved_owner_count"], 0)
        self.assertTrue(all(r["formal_owner_decision"] == "PENDING_SIGNOFF" for r in self.ledger["paths"]))

    def test_generated_and_app_server_evidence_is_bounded(self):
        rows = self.ledger["paths"]
        generated = [p for p in rows if p["owner"] == "generated_protocol"]
        app_server = [p for p in rows if p["owner"] == "app_server_protocol"]
        self.assertEqual((len(generated), len(app_server)), (12, 11))
        self.assertTrue(all(p["readiness"] == "GENERATED_BYTES_AND_SDK_REPRODUCIBLE" for p in generated))
        self.assertTrue(all(p["readiness"] == "ALL_ELEVEN_SOURCE_DIFFS_RECONSTRUCTED_NATIVE_PARTIAL" for p in app_server))
        self.assertTrue(all(any(x.get("run_id") == 41062921661 for x in p["evidence"]) for p in generated))
        self.assertTrue(all(any(x.get("kind") == "DANIEL_CL_TARGETED_NATIVE" for x in p["evidence"]) for p in app_server))

if __name__ == "__main__":
    unittest.main()
