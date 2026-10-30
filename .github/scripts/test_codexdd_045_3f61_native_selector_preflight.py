"""3F.61 cheap negative gate: all native selectors must exist before Windows compiling.

A test that disappeared from the fork is not an optional skipped test. Exact names
must have a unique function definition in the frozen Rust source.
"""
from __future__ import annotations
import json
import re
import unittest
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
SPEC=ROOT/"docs/upstream-candidates/codexdd-045-3f61-native-test-cases.json"
FORMAL=ROOT/"docs/upstream-candidates/codexdd-045-3f55-formal-owner-approvals.json"
class FrozenNativeTestSelectors(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.spec=json.loads(SPEC.read_text(encoding="utf-8"))
  cls.owners=json.loads(FORMAL.read_text(encoding="utf-8"))

 def test_exact_source_and_unwaived_signoff(self):
  s=self.spec
  self.assertEqual(s["contract_version"],1)
  self.assertEqual(s["candidate_source_sha"],"72cd09d3f52c076d445078758af20489ccdb4e74")
  self.assertEqual(s["cargo_lock_blob"],"13bc15ff844e93dfaf7777c73bb7313b35bb3b81")
  self.assertEqual((s["expected_core"],s["expected_tui"]),(14,18))
  self.assertEqual(self.owners["approved_owner_count"],0)
  self.assertEqual(self.owners["ready_for_formal_review_count"],109)
  self.assertEqual(self.owners["source_review_queued_count"],0)
  self.assertFalse(s["release_approved"])
  self.assertGreaterEqual(len(s["source_only_gaps"]),6)
  self.assertEqual(self.owners["pending_native_or_semantic_restoration"].__len__(),13)

 def test_unique_exact_named_test_function_declarations(self):
  for owner in ("core","tui"):
   files=list((ROOT/"codex-rs"/owner/"src").rglob("*.rs"))
   self.assertGreater(len(files),5)
   strings=[(p,p.read_text(encoding="utf-8")) for p in files]
   seen=set()
   for item in self.spec["cases"][owner]:
    selector=item["selector"]
    with self.subTest(owner=owner,test=selector):
     self.assertRegex(selector,r"^[a-z][a-z0-9_]+$")
     self.assertTrue(item["rationale"])
     self.assertNotIn(selector,seen)
     seen.add(selector)
     declaration=re.compile(r"\b(?:async\s+)?fn\s+"+re.escape(selector)+r"\s*\(")
     hits=[str(p.relative_to(ROOT)) for p,data in strings if declaration.search(data)]
     self.assertEqual(len(hits),1,"Missing/duplicate frozen native test: "+selector+" "+str(hits))
   self.assertEqual(len(seen),self.spec["expected_"+owner])

 def test_separate_unresolved_upstream_semantic_holds_are_not_waived(self):
  gaps="\n".join(self.spec["source_only_gaps"])
  for token in ("modal","foreground","backpressure","Daybreak","realtime","terminal program"):
   self.assertIn(token,gaps)
  self.assertTrue(all(r["formal_owner_decision"]=="PENDING_SIGNOFF" for r in self.owners["paths"]))

if __name__=="__main__":
 unittest.main()
