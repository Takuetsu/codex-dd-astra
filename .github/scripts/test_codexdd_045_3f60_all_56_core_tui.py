"""3F.60: all final 56 Core/TUI source differences and formal owner risk holds.

Every file is pinned to its current Git blob and can be reconstructed to exactly
official upstream 0.163.0 by reversing the archived hunks. This validates what
changed, not whether the change is acceptable at runtime.
"""
from __future__ import annotations
import hashlib
import json
import subprocess
import unittest
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
DIR=ROOT/"docs/upstream-candidates"
RECEIPTS=[
"codexdd-045-3f60-core-authority-deltas-a.json",
"codexdd-045-3f60-core-authority-deltas-b.json",
"codexdd-045-3f60-tui-deltas-a.json",
"codexdd-045-3f60-tui-deltas-b.json",
"codexdd-045-3f60-tui-deltas-c.json",
"codexdd-045-3f60-tui-deltas-d.json",
"codexdd-045-3f60-tui-deltas-e.json",
]
HIGH_RISK={
"codex-rs/tui/src/app.rs",
"codex-rs/tui/src/app/event_dispatch.rs",
"codex-rs/tui/src/app/config_persistence.rs",
"codex-rs/tui/src/app/tests.rs",
"codex-rs/tui/src/app/thread_routing.rs",
"codex-rs/tui/src/chatwidget/tests.rs",
"codex-rs/tui/src/chatwidget/tests/slash_commands.rs",
"codex-rs/tui/src/app_server_session.rs",
"codex-rs/tui/src/chatwidget/settings.rs",
"codex-rs/core/src/tools/mod.rs",
"codex-rs/core/src/tools/spec_plan.rs",
"codex-rs/core/src/client.rs",
"codex-rs/core/src/thread_manager.rs",
}
def git_oid(data:bytes)->str:
 return hashlib.sha1(b"blob "+str(len(data)).encode()+b"\x00"+data).hexdigest()

class AllRemainingOwners(unittest.TestCase):
 @classmethod
 def setUpClass(cls):
  cls.receipts={name:json.loads((DIR/name).read_text(encoding="utf-8")) for name in RECEIPTS}
  cls.paths={p["path"]:p for doc in cls.receipts.values() for p in doc["paths"]}
  cls.formal=json.loads((DIR/"codexdd-045-3f55-formal-owner-approvals.json").read_text(encoding="utf-8"))
  cls.owner={r["path"]:r for r in cls.formal["paths"]}
  cls.original=json.loads((DIR/"codexdd-045-109-semantic-owner-matrix.json").read_text(encoding="utf-8"))

 def test_18_core_and_38_tui_completely_accounted_for(self):
  self.assertEqual([r["reviewed_count"] for r in self.receipts.values()],[9,9,8,8,8,8,6])
  self.assertEqual(len(self.paths),56)
  self.assertEqual(len([p for p in self.paths if p.startswith("codex-rs/core/")]),18)
  self.assertEqual(len([p for p in self.paths if p.startswith("codex-rs/tui/")]),38)
  self.assertEqual(len(self.formal["paths"]),109)
  self.assertEqual((self.formal["ready_for_formal_review_count"],self.formal["source_review_queued_count"],self.formal["approved_owner_count"]),(109,0,0))
  self.assertEqual(set(self.formal["pending_native_or_semantic_restoration"]),HIGH_RISK)
  self.assertFalse(self.formal["release_approved"])
  self.assertEqual({p["path"] for p in self.original["paths"]},set(self.owner))
  self.assertTrue(all(row["semantic_status"]=="PENDING_OWNER_REVIEW" for row in self.original["paths"]))

 def test_all_fifty_six_reverse_hunks_exact_current_and_upstream_git(self):
  for path,r in self.paths.items():
   with self.subTest(path=path):
    p=self.owner[path]
    self.assertEqual(p["candidate_blob"],r["candidate_blob"])
    self.assertEqual(p["official_upstream_0163_blob"],r["upstream_blob"])
    self.assertEqual(p["source_review_status"],"SOURCE_COMPARISON_AND_EVIDENCE_ATTACHED")
    self.assertEqual(p["formal_owner_decision"],"PENDING_SIGNOFF")
    self.assertEqual(p["release_status"],"NOT_ACCEPTED")
    self.assertTrue(p["review_summary"])
    self.assertTrue(p["required_evidence"])
    self.assertTrue(any(x.get("hunk_count")==len(r["hunks"]) and "3f60" in x.get("receipt","") for x in p["evidence"]))
    data=subprocess.check_output(["git","show","HEAD:"+path],cwd=ROOT)
    self.assertEqual(git_oid(data),r["candidate_blob"])
    text=data.decode("utf-8")
    lines=text.split("\n")
    prior=len(lines)+1
    for h in reversed(r["hunks"]):
     start=h["candidate_start_line"]-1
     self.assertGreaterEqual(start,0)
     self.assertLess(start,prior)
     self.assertEqual(lines[start:start+len(h["candidate_lines"])],h["candidate_lines"])
     lines[start:start+len(h["candidate_lines"])]=h["upstream_lines"]
     prior=start
    self.assertEqual(git_oid("\n".join(lines).encode("utf-8")),r["upstream_blob"])

 def test_risk_holds_not_converted_into_automatic_signoffs(self):
  for path in HIGH_RISK:
   with self.subTest(path=path):
    r=self.owner[path]
    self.assertEqual(r["readiness"],"BLOCKED_NATIVE_OR_UPSTREAM_BEHAVIOR_RESTORATION")
    self.assertEqual(r["native_acceptance_priority"],"P0_BLOCKING")
    self.assertTrue(any("Explicit upstream semantic preservation" in t for t in r["required_evidence"]))
    self.assertFalse(r["approved_by"])
  for p in self.owner.values():
   self.assertEqual(p["formal_owner_decision"],"PENDING_SIGNOFF")
   self.assertIsNone(p["approved_by"])
   self.assertIsNone(p["approval_utc"])
  self.assertEqual((ROOT/"codex-rs/codexdd-version.txt").read_text().strip(),"0.4.4")
  self.assertEqual((ROOT/"codex-rs/upstream-codex-release.txt").read_text().strip(),"rust-v0.162.0")

 def test_upstream_gaps_are_explicit_not_silently_waived(self):
  app=self.paths["codex-rs/tui/src/app.rs"]
  app_removed="\n".join("\n".join(h["upstream_lines"]) for h in app["hunks"])
  self.assertIn("discard_startup_modal_input",app_removed)
  self.assertIn("handle_rendered_selection_event",app_removed)
  self.assertIn("active_view_accepts_input_when_disconnected",app_removed)
  tests=self.paths["codex-rs/tui/src/app/tests.rs"]
  self.assertIn("background_resolution_keeps_matching_foreground_user_input_visible", "\n".join("\n".join(h["upstream_lines"]) for h in tests["hunks"]))
  slash=self.paths["codex-rs/tui/src/chatwidget/tests/slash_commands.rs"]
  deleted="\n".join("\n".join(h["upstream_lines"]) for h in slash["hunks"])
  self.assertIn("ApiKeyCyberAccessPrograms",deleted)
  self.assertIn("KeyCode::Insert",deleted)
  self.assertIn("text_fallback",deleted)
  routing=self.paths["codex-rs/tui/src/app/thread_routing.rs"]
  self.assertIn("try_send", "\n".join("\n".join(h["upstream_lines"]) for h in routing["hunks"]))
  self.assertIn("UnboundedReceiver", "\n".join("\n".join(h["candidate_lines"]) for h in routing["hunks"]))
  core=self.paths["codex-rs/core/src/client.rs"]
  self.assertIn('tool_choice: "auto"',"\n".join("\n".join(h["upstream_lines"]) for h in core["hunks"]))

if __name__=="__main__":
 unittest.main()
