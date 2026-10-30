"""3F.59: exact nine upstream-0.163 private persistence/memory owner deltas.

Machine-readable source review only. No native runtime or individual owner acceptance.
"""
from __future__ import annotations
import hashlib
import json
import subprocess
import unittest
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/"docs/upstream-candidates"
RECEIPT=BASE/"codexdd-045-3f59-persistence-memory-upstream-deltas.json"
FORMAL=BASE/"codexdd-045-3f55-formal-owner-approvals.json"
EXPECTED={
 "codex-rs/external-agent-migration/src/sessions/append_tests.rs":("72d8fc5d5cf9c388f179247330a354bd88d4c8f8","cbe3ae6a7c1509d582a959af38839eb082f1ab55"),
 "codex-rs/history/src/lib.rs":("a54a5733e029fe3728a46adb3f50c68dae4d0eb2","826669cc3335dc23102e6a5bdbf2d62c1cb22fa4"),
 "codex-rs/history/src/tests.rs":("a0a99cddd94064b6245e174cc74818ea3b5a3df2","32a6110d3b53ee1dac0a7ac5ed12ede35a012812"),
 "codex-rs/memories/write/src/phase1.rs":("a036229bb256483f3a6e96ef6ceed86e4d12f3af","aa11ce276a51b197b9f9ba3e58d66eb16401b066"),
 "codex-rs/memories/write/src/rollout_input.rs":("3c54f1884b409a7060f54dd90739fa897ef7e604","2d45c7b63bdcda64ff2d37eccb112556e10a2f4e"),
 "codex-rs/rollout-trace/src/protocol_event.rs":("4d16022f93841cb5d92eca00c76ec8cd84dd5bfd","f4d66320352ca0266ca7f8c1cc823d4c054adcd3"),
 "codex-rs/rollout/src/search.rs":("d222871cab7b8e55dcd1dec8faef7e9cf4918796","77206404e1acd9f120240d41c155e4aa9b503cb5"),
 "codex-rs/state/src/extract.rs":("f9792c339f184ea66f58db9463ba8b85a1cbe4d9","f079ebc4ee8c09213df53b234e20d8c94800adf8"),
 "codex-rs/state/src/runtime/threads.rs":("5a0edb6a4f5c4968df7586b5d7adae7c1653c301","1179126f6241be8a8d39cefcbd5ded38660d8cb9"),
}
def oid(data:bytes)->str:
 return hashlib.sha1(b"blob "+str(len(data)).encode()+b"\x00"+data).hexdigest()

class PrivateStatePersistenceReview(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.receipt=json.loads(RECEIPT.read_text())
  cls.formal=json.loads(FORMAL.read_text())
  cls.owners={r["path"]:r for r in cls.formal["paths"]}

 def test_all_nine_reconstruct_exact_official_0163_git_blobs(self):
  self.assertEqual(self.receipt["reviewed_count"],9)
  self.assertEqual(self.receipt["official_upstream"],"openai/codex@rust-v0.163.0")
  self.assertFalse(self.receipt["release_approved"])
  self.assertEqual(self.receipt["owner_approvals"],0)
  self.assertEqual({r["path"] for r in self.receipt["paths"]},set(EXPECTED))
  for row in self.receipt["paths"]:
   with self.subTest(path=row["path"]):
    path=row["path"]
    self.assertEqual((row["candidate_blob"],row["upstream_blob"]),EXPECTED[path])
    self.assertEqual(row["formal_owner_decision"],"PENDING_SIGNOFF")
    self.assertTrue(row["semantic_note"])
    data=subprocess.check_output(["git","show","HEAD:"+path],cwd=ROOT)
    self.assertEqual(oid(data),row["candidate_blob"])
    lines=data.decode("utf-8").split("\n")
    last=len(lines)+1
    for h in reversed(row["hunks"]):
     at=h["candidate_start_line"]-1
     self.assertGreaterEqual(at,0)
     self.assertLess(at,last)
     self.assertEqual(lines[at:at+len(h["candidate_lines"])],h["candidate_lines"])
     lines[at:at+len(h["candidate_lines"])]=h["upstream_lines"]
     last=at
    self.assertEqual(oid("\n".join(lines).encode()),row["upstream_blob"])

 def test_nine_owner_reviews_submitted_but_not_accepted(self):
  self.assertEqual(len(self.formal["paths"]),109)
  self.assertEqual((self.formal["ready_for_formal_review_count"],self.formal["source_review_queued_count"],self.formal["approved_owner_count"]),(53,56,0))
  for path in EXPECTED:
   p=self.owners[path]
   with self.subTest(path=path):
    self.assertEqual(p["owner"],"persistence_rollout_memory")
    self.assertEqual(p["source_review_status"],"SOURCE_COMPARISON_AND_EVIDENCE_ATTACHED")
    self.assertEqual(p["formal_owner_decision"],"PENDING_SIGNOFF")
    self.assertEqual(p["release_status"],"NOT_ACCEPTED")
    self.assertTrue(any(e.get("kind")=="EXACT_REVERSIBLE_UPSTREAM_SOURCE_DELTA" and "3f59" in e["receipt"] for e in p["evidence"]))
    self.assertTrue(any("Native exact-source" in e for e in p["required_evidence"]))

 def test_private_workflow_state_never_enters_model_memory_search_or_traces(self):
  checks={
   "codex-rs/memories/write/src/phase1.rs":("RolloutItem::WorkflowState(_)",),
   "codex-rs/memories/write/src/rollout_input.rs":("RolloutItem::WorkflowState(_)",),
   "codex-rs/rollout/src/search.rs":("RolloutItem::WorkflowState(_)",),
   "codex-rs/state/src/extract.rs":("RolloutItem::WorkflowState(_)",),
   "codex-rs/state/src/runtime/threads.rs":("RolloutItem::WorkflowState(_)",),
   "codex-rs/rollout-trace/src/protocol_event.rs":("EventMsg::AdaptiveRuntimeSignal(_)",),
   "codex-rs/history/src/lib.rs":("WORKFLOW_STATE_SCHEMA_VERSION","WorkflowState(WorkflowStateItem)"),
   "codex-rs/history/src/tests.rs":("workflow_state_rollout_records_round_trip_without_thread_identity",),
   "codex-rs/external-agent-migration/src/sessions/append_tests.rs":("external_import_source_rejects_workflow_authority_but_allows_native_history",),
  }
  for path,fragments in checks.items():
   with self.subTest(path=path):
    source=(ROOT/path).read_text(encoding="utf-8")
    for fragment in fragments:
     self.assertIn(fragment,source)

if __name__=="__main__":
 unittest.main()
