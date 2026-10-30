"""3F.57: byte-exact 0.163 CLI, config, rollout + history source review.

This is seven formal review submissions, not native test passage or owner signoff.
"""
from __future__ import annotations
import hashlib
import json
import subprocess
import unittest
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/"docs/upstream-candidates"
RECEIPT=BASE/"codexdd-045-3f57-cli-state-reversible-source-review.json"
FORMAL=BASE/"codexdd-045-3f55-formal-owner-approvals.json"
EXPECTED={
 "codex-rs/cli/src/main.rs":("9379d52b41473f1a57cb49d07048af7c139ebcf9","9d540c3124ee692250d345719d122aa1884ba12f"),
 "codex-rs/cli/tests/daemon_startup.rs":("e1c6d330f602f3c4cebf9d7163ecdcd2adb9100f","f50758363578497af6e1139e8a1410858706e38c"),
 "codex-rs/config/src/config_toml.rs":("a07f08c8831dd011d4950a52088c7233722620d8","8c8c66ea1dc45669a3f52e3e0d3f10e633267167"),
 "codex-rs/rollout/src/policy.rs":("294ce2a11eacf8aca837a828a0012cb1301f3e4d","eb13ec553ef546f8c982dd46deef9d0880a24555"),
 "codex-rs/rollout/src/recorder.rs":("6e70783e7c2e93e9a705a2ce32f7317545ea9228","765d83ccc7df4b136c04774930f4ac65af15bbd5"),
 "codex-rs/history/src/rollout_payload.rs":("180b889e87cf3b0c13d1e43cfef0f21bb88c7c1c","9ab89363df8a895ee4cfb28c4f38acb8a9d35250"),
 "codex-rs/thread-store/src/types.rs":("315498be3dcdd650fa147ca6d3c69abd9c089e2c","5b3dea35516301a35d47c70b83e548dc060b8c9c"),
}
def git_oid(data:bytes)->str:
 return hashlib.sha1(b"blob "+str(len(data)).encode()+b"\x00"+data).hexdigest()

class SevenOwnerSourceReview(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.receipt=json.loads(RECEIPT.read_text(encoding="utf-8"))
  cls.formal=json.loads(FORMAL.read_text(encoding="utf-8"))
  cls.owner={r["path"]:r for r in cls.formal["paths"]}

 def test_seven_exact_reverse_upstream_hunks(self):
  self.assertEqual(self.receipt["reviewed_count"],7)
  self.assertEqual(self.receipt["owner_approvals"],0)
  self.assertFalse(self.receipt["release_approved"])
  self.assertEqual(self.receipt["official_upstream"],"openai/codex@rust-v0.163.0")
  self.assertEqual({r["path"] for r in self.receipt["paths"]},set(EXPECTED))
  for row in self.receipt["paths"]:
   with self.subTest(path=row["path"]):
    path=row["path"]
    self.assertEqual((row["candidate_blob"],row["upstream_blob"]),EXPECTED[path])
    self.assertEqual(row["formal_owner_decision"],"PENDING_SIGNOFF")
    self.assertTrue(row["semantic_note"])
    data=subprocess.check_output(["git","show","HEAD:"+path],cwd=ROOT)
    self.assertEqual(git_oid(data),row["candidate_blob"])
    lines=data.decode("utf-8").split("\n")
    last=len(lines)+1
    for h in reversed(row["hunks"]):
     pos=h["candidate_start_line"]-1
     self.assertGreaterEqual(pos,0)
     self.assertLess(pos,last)
     self.assertEqual(lines[pos:pos+len(h["candidate_lines"])],h["candidate_lines"])
     lines[pos:pos+len(h["candidate_lines"])]=h["upstream_lines"]
     last=pos
    self.assertEqual(git_oid("\n".join(lines).encode("utf-8")),row["upstream_blob"])

 def test_formal_owner_submission_is_evidenced_not_accepted(self):
  self.assertGreaterEqual(self.formal["ready_for_formal_review_count"],44)
  self.assertLessEqual(self.formal["source_review_queued_count"],65)
  self.assertEqual(self.formal["ready_for_formal_review_count"]+self.formal["source_review_queued_count"],109)
  self.assertEqual(self.formal["approved_owner_count"],0)
  self.assertEqual(len(self.formal["paths"]),109)
  for path in EXPECTED:
   row=self.owner[path]
   with self.subTest(path=path):
    self.assertEqual(row["source_review_status"],"SOURCE_COMPARISON_AND_EVIDENCE_ATTACHED")
    self.assertEqual(row["formal_owner_decision"],"PENDING_SIGNOFF")
    self.assertEqual(row["release_status"],"NOT_ACCEPTED")
    self.assertTrue(any(e.get("kind")=="EXACT_REVERSIBLE_UPSTREAM_SOURCE_DELTA" for e in row["evidence"]))
    self.assertTrue(any("Native exact-source regressions" in e for e in row["required_evidence"]))

 def test_core_cli_state_wire_semantic_guards(self):
  main=(ROOT/"codex-rs/cli/src/main.rs").read_text()
  self.assertIn("interactive.adaptive = adaptive;",main)
  daemon=(ROOT/"codex-rs/cli/tests/daemon_startup.rs").read_text()
  self.assertIn("unexpected shared daemon PID",daemon)
  cfg=(ROOT/"codex-rs/config/src/config_toml.rs").read_text()
  self.assertIn("pub adaptive_worker: Option<AdaptiveWorkerConfigToml>",cfg)
  self.assertIn("adaptive_worker_and_daybreak_settings_deserialize_together",cfg)
  policy=(ROOT/"codex-rs/rollout/src/policy.rs").read_text()
  self.assertIn("RolloutItem::WorkflowState(_)",policy)
  self.assertIn("EventMsg::AdaptiveRuntimeSignal(_)",policy)
  recorder=(ROOT/"codex-rs/rollout/src/recorder.rs").read_text()
  self.assertIn("State-db-only queries must reach the database",recorder)
  wire=(ROOT/"codex-rs/history/src/rollout_payload.rs").read_text()
  self.assertIn("CodexDdWorkflowState",wire)
  store=(ROOT/"codex-rs/thread-store/src/types.rs").read_text()
  self.assertIn("pub workflow_state: codex_rollout::RestoredWorkflowState",store)

if __name__=="__main__":
 unittest.main()
