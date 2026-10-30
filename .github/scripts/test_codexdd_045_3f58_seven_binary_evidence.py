"""3F.58: validate GREEN seven-program unsigned Windows evidence and owner boundaries."""
from __future__ import annotations
import json
import re
import unittest
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/"docs/upstream-candidates"
SEVEN=BASE/"codexdd-045-3f58-seven-program-windows-evidence.json"
FIVE=BASE/"codexdd-045-windows-distribution-contract.json"
OWNERS=BASE/"codexdd-045-3f55-formal-owner-approvals.json"
EXTRA={
 "codex-windows-sandbox-service.exe":(55867392,"7738934e39ce451f15f342b4ce65710aaab192e5c29fca7707c05d20c523e2ac"),
 "codex-windows-sandbox-service.pdb":(165584896,"13f2abafd1a312bfb69e1310124796a99af7cff83850bba457dce8ba7e58fc6b"),
 "codex-responses-api-proxy.exe":(5852672,"690d0e855464135a197e2e6cca08d7da3f63c3a76aab405f1b934eb05866b6a6"),
 "codex-responses-api-proxy.pdb":(21147648,"e001827c2562b87241f70ad1b9722d1d829b5d8468a6fb15f0f63e54eecb8d6d"),
}
class SevenProgramEvidence(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.seven=json.loads(SEVEN.read_text())
  cls.five=json.loads(FIVE.read_text())
  cls.owners=json.loads(OWNERS.read_text())

 def test_true_14_member_provenance_not_derived_release(self):
  s=self.seven; f=self.five
  self.assertEqual(s["verification_run_id"],41073817590)
  self.assertEqual(s["verification_run_conclusion"],"success")
  self.assertEqual(s["original_five_program_run_id"],40922368730)
  self.assertEqual(s["binary_source_sha"],f["source_sha"])
  self.assertEqual(s["binary_source_sha"],"57152222a1ef523fe6e2f5079871672423c6cb56")
  self.assertNotEqual(s["candidate_source_sha"],s["binary_source_sha"])
  self.assertEqual(s["tooling_sha"],"e20ea711f3cd7eda56eb5fe8b477434c735527cf")
  self.assertEqual(s["original_five_archive_sha256"],f["build_evidence"]["combined_archive_sha256"])
  self.assertEqual(s["archive"]["archive_sha256"],"dccb86a05caa7bf695f4ddd2b471025a50683853fd5dffacc35f0cb981d6ed77")
  self.assertEqual(s["archive"]["artifact_id"],12990307741)
  self.assertEqual((s["archive"]["program_count"],s["archive"]["symbol_count"],s["archive"]["file_count"]),(7,7,14))
  self.assertEqual(set(s["files"]),set(f["build_evidence"]["combined_members"])|set(EXTRA))
  self.assertEqual(len(s["files"]),14)
  for name,data in s["files"].items():
   with self.subTest(name=name):
    self.assertRegex(name,r"^codex.*\.(exe|pdb)$")
    self.assertRegex(data["sha256"],r"^[0-9a-f]{64}$")
    self.assertEqual(data["source_sha"],s["binary_source_sha"])
    self.assertGreater(data["bytes"],4096)
    if name in EXTRA:
     self.assertEqual((data["bytes"],data["sha256"]),EXTRA[name])
    else:
     original=f["build_evidence"]["combined_members"][name]
     self.assertEqual((data["bytes"],data["sha256"]),(original["bytes"],original["sha256"]))
  for name in f["full_installer_binaries"]:
   self.assertIn(name,s["files"])
   self.assertIn(name[:-4]+".pdb",s["files"])

 def test_unsigned_artifact_is_available_but_not_release_approved(self):
  s=self.seven
  self.assertGreater(s["archive"]["artifact_bytes"],100000000)
  self.assertEqual(s["archive"]["expires_at"],"2026-11-06T15:42:15Z")
  self.assertEqual({x["id"] for x in s["standalone_artifacts"].values()},
                   {12990072080,12990352271,12989538654})
  v=s["verification"]
  for key in ("source_sha_matched_all_seven","archive_sha256_independent_verified",
              "all_14_members_verified","post_build_exe_pdb_headers_verified"):
   self.assertIs(v[key],True)
  for key in ("code_signed","voice_runtime_included","official_installer_built",
              "release_approved","twenty_one_stage_release_validation"):
   self.assertIs(v[key],False)
  self.assertEqual(v["owner_formal_signoffs"],0)

 def test_three_bazel_owner_paths_evidenced_but_not_accepted(self):
  d=self.owners
  self.assertEqual(d["approved_owner_count"],0)
  self.assertGreaterEqual(d["ready_for_formal_review_count"],44)
  self.assertLessEqual(d["source_review_queued_count"],65)
  self.assertEqual(d["ready_for_formal_review_count"]+d["source_review_queued_count"],109)
  matches=[p for p in d["paths"] if p["owner"]=="bazel_packaging"]
  self.assertEqual(len(matches),3)
  for p in matches:
   self.assertEqual(p["readiness"],"SEVEN_OPTIMIZED_WINDOWS_EXES_AND_PDBS_VERIFIED_UNSIGNED_BASELINE_SOURCE")
   self.assertEqual(p["formal_owner_decision"],"PENDING_SIGNOFF")
   self.assertEqual(p["release_status"],"NOT_ACCEPTED")
   evidence=[x for x in p["evidence"] if x.get("run_id")==41073817590]
   self.assertEqual(len(evidence),1)
   self.assertIs(evidence[0]["source_bound_to_reviewed_candidate"],False)
   self.assertEqual(evidence[0]["archive_sha256"],self.seven["archive"]["archive_sha256"])

if __name__=="__main__":
 unittest.main()
