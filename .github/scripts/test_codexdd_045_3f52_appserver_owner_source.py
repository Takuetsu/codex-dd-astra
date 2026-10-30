"""3F.52: all eleven app-server/protocol owner sources reconstruct official 0.163.

This only establishes exact provenance/deltas. No runtime/security signoff.
"""
from __future__ import annotations

import hashlib
import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / "docs/upstream-candidates"
BATCH_A = BASE / "codexdd-045-3f51-appserver-upstream-delta.json"
BATCH_B = BASE / "codexdd-045-3f52-appserver-owner-remaining-deltas.json"
MATRIX = BASE / "codexdd-045-109-semantic-owner-matrix.json"
OFFICIAL = "openai/codex@rust-v0.163.0"
LATEST = {
    "codex-rs/app-server/src/bespoke_event_handling.rs": ("8f77eee9595c8780d6c76ded43d3043b30af8ce8", "5e5430826edf33b22c5cdbdec3f5e5a6078f20aa"),
    "codex-rs/app-server/src/notification_media.rs": ("c93967717664145a72390384749747212fd5c63a", "a1572cbaf7b9026f789c96ce9d027e0a7cc4b1be"),
    "codex-rs/app-server/src/request_processors.rs": ("5c4633ab48ea77692e28193ea6e128ec58067a37", "a40486e0d512af66aeef1c1307696b2740053500"),
    "codex-rs/app-server/src/thread_state.rs": ("42b434e017c527a8785fbaedf43292a83e1af09d", "6dfe969ab3adfc5d703184ee2ee61a1b6335f282"),
    "codex-rs/protocol/src/protocol.rs": ("9ff46a4e1331e15f35f95b8e17ba41dfc931a5cf", "146209407b78b71c461c854c5d9d1e4d7816b873"),
}

def git_oid(data: bytes) -> str:
    return hashlib.sha1(b"blob " + str(len(data)).encode("ascii") + b"\x00" + data).hexdigest()

class AppServerOwnerElevenSource(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.all_docs = [json.loads(p.read_text(encoding="utf-8")) for p in (BATCH_A, BATCH_B)]
        cls.rows = [r for d in cls.all_docs for r in d["paths"]]
        cls.matrix = json.loads(MATRIX.read_text(encoding="utf-8"))

    def test_all_eleven_protocol_owner_paths_have_exact_reverse_upstream_proof(self):
        self.assertEqual([d["reviewed_count"] for d in self.all_docs], [6, 5])
        self.assertTrue(all(d["official_ref"] == OFFICIAL and not d["release_approved"] for d in self.all_docs))
        owner = {p["path"] for p in self.matrix["paths"] if p["owner"] == "app_server_protocol"}
        self.assertEqual(len(owner), 11)
        self.assertEqual({p["path"] for p in self.rows}, owner)
        self.assertEqual(len(self.rows), 11)
        self.assertEqual({p["path"] for p in self.all_docs[1]["paths"]}, set(LATEST))
        self.assertTrue(all(p["semantic_signoff"] == "NOT_GRANTED" for p in self.rows))

    def test_remaining_five_git_blobs_reconstruct_exact_official_sources(self):
        matrix = {p["path"]: p for p in self.matrix["paths"]}
        for row in self.all_docs[1]["paths"]:
            with self.subTest(path=row["path"]):
                path = row["path"]
                self.assertEqual((row["candidate_blob"], row["upstream_blob"]), LATEST[path])
                self.assertEqual(row["candidate_blob"], matrix[path]["current_blob"])
                self.assertEqual(matrix[path]["semantic_status"], "PENDING_OWNER_REVIEW")
                self.assertEqual(matrix[path]["release_status"], "NOT_ACCEPTED")
                source = subprocess.check_output(["git", "show", "HEAD:" + path], cwd=ROOT)
                self.assertEqual(git_oid(source), row["candidate_blob"])
                lines = source.decode("utf-8").split("\n")
                position = len(lines) + 1
                for hunk in reversed(row["hunks"]):
                    at = hunk["candidate_start_line"] - 1
                    self.assertLess(at, position)
                    self.assertGreaterEqual(at, 0)
                    self.assertEqual(lines[at:at + len(hunk["candidate_lines"])], hunk["candidate_lines"])
                    lines[at:at + len(hunk["candidate_lines"])] = hunk["upstream_lines"]
                    position = at
                self.assertEqual(git_oid("\n".join(lines).encode("utf-8")), row["upstream_blob"])

    def test_core_generated_signal_identity_is_attached_by_app_server_not_client(self):
        rust = (ROOT / "codex-rs/protocol/src/protocol.rs").read_text(encoding="utf-8")
        begin = rust.index("pub struct AdaptiveRuntimeSignalEvent {")
        end = rust.index("}", begin)
        envelope = rust[begin:end]
        self.assertIn("signal_kind: AdaptiveRuntimeSignalKind", envelope)
        self.assertIn("evidence_refs: Vec<String>", envelope)
        self.assertIn("diagnostic_note: Option<String>", envelope)
        self.assertNotIn("thread_id", envelope)
        self.assertNotIn("source_turn_id", envelope)
        self.assertIn("AdaptiveRuntimeSignal(AdaptiveRuntimeSignalEvent)", rust)
        event = (ROOT / "codex-rs/app-server/src/bespoke_event_handling.rs").read_text(encoding="utf-8")
        self.assertIn("source_turn_id: String", event)
        self.assertIn("thread_id: conversation_id.to_string()", event)
        self.assertIn("EventMsg::AdaptiveRuntimeSignal(signal) => {", event)
        self.assertIn("async fn adaptive_runtime_signal_does_not_replay_on_failed_or_interrupted_turn()", event)

    def test_signal_is_transported_without_media_modification_and_delegates_typed_params(self):
        media = (ROOT / "codex-rs/app-server/src/notification_media.rs").read_text(encoding="utf-8")
        self.assertIn("ServerNotification::AdaptiveRuntimeSignal(_) => notification", media)
        imports = (ROOT / "codex-rs/app-server/src/request_processors.rs").read_text(encoding="utf-8")
        for item in ("ThreadWorkflowStateOperation", "ThreadWorkflowStateUpdateParams", "ThreadWorkflowStateUpdateResponse"):
            self.assertIn("use codex_app_server_protocol::" + item + ";", imports)
        state = (ROOT / "codex-rs/app-server/src/thread_state.rs").read_text(encoding="utf-8")
        self.assertIn("adaptive_runtime_signals: Vec<AdaptiveRuntimeSignalEnvelope>", state)
        self.assertIn("pub(crate) struct TurnSummary", state)
        self.assertIn("pub(crate) struct ThreadState", state)
        self.assertIn("TurnSummary", state)


if __name__ == "__main__":
    unittest.main()
