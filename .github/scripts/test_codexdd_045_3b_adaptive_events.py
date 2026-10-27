"""CodexDD 0.4.5 / 3B.3: offline adaptive event and thread-state provenance.

Static source checks only; Rust compilation and behavior validation must follow
once the entire upstream 0.163.0 source graph is reconciled.
"""

from __future__ import annotations

import hashlib
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ROOT_SOURCE = "codex-rs/app-server/src/"
FILES = {
    "bespoke_event_handling.rs": "1c4a913d2373a614a2dfe73c10e6f65c64a727a1",
    "thread_state.rs": "42b434e017c527a8785fbaedf43292a83e1af09d",
    "notification_media.rs": "c93967717664145a72390384749747212fd5c63a",
}


def committed(path: str) -> bytes:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT,
        capture_output=True,
        check=True,
    ).stdout


def sha(raw: bytes) -> str:
    return hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()


def text(path: str) -> str:
    return committed(ROOT_SOURCE + path).decode("utf-8")


class AdaptiveEventIntegrationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.handler = text("bespoke_event_handling.rs")
        cls.state = text("thread_state.rs")
        cls.media = text("notification_media.rs")

    def test_exact_three_source_git_blobs(self):
        for path, expected in FILES.items():
            with self.subTest(path=path):
                self.assertEqual(sha(committed(ROOT_SOURCE + path)), expected)

    def test_core_adaptive_signal_event_has_a_single_handler(self):
        self.assertEqual(
            self.handler.count("async fn handle_adaptive_runtime_signal("), 1
        )
        self.assertEqual(
            self.handler.count("EventMsg::AdaptiveRuntimeSignal(signal) => {"), 1
        )

    def test_session_owned_notification_contains_core_source_turn_id(self):
        start = self.handler.index("async fn handle_adaptive_runtime_signal(")
        end = self.handler.index(
            "pub(crate) async fn apply_bespoke_event_handling(", start
        )
        body = self.handler[start:end]
        for marker in (
            "source_turn_id,",
            "signal_kind: signal.signal_kind",
            "evidence_refs: signal.evidence_refs",
            "diagnostic_note: signal.diagnostic_note",
            "thread_id: conversation_id.to_string()",
        ):
            self.assertIn(marker, body)

    def test_live_signal_is_saved_to_thread_turn_summary(self):
        start = self.handler.index("async fn handle_adaptive_runtime_signal(")
        end = self.handler.index(
            "pub(crate) async fn apply_bespoke_event_handling(", start
        )
        body = self.handler[start:end]
        self.assertIn(".adaptive_runtime_signals", body)
        self.assertIn(".push(signal.clone());", body)
        self.assertIn(
            "send_server_notification(ServerNotification::AdaptiveRuntimeSignal(", body
        )

    def test_thread_state_has_turn_scoped_adaptive_signal_buffer(self):
        self.assertIn(
            "pub(crate) adaptive_runtime_signals: Vec<AdaptiveRuntimeSignalEnvelope>",
            self.state,
        )
        self.assertIn(
            "use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;", self.state
        )
        self.assertIn("#[derive(Default)]", self.state)

    def test_turn_completion_replays_trusted_signal_only_if_successful(self):
        start = self.handler.index("async fn handle_turn_complete(")
        end = self.handler.index("async fn handle_turn_interrupted(", start)
        body = self.handler[start:end]
        self.assertIn("let TurnSummary {", body)
        self.assertIn("adaptive_runtime_signals,", body)
        self.assertIn("if matches!(status, TurnStatus::Completed) {", body)
        self.assertIn("for signal in adaptive_runtime_signals {", body)

    def test_terminal_replay_is_before_turn_completed_notification(self):
        start = self.handler.index("async fn handle_turn_complete(")
        end = self.handler.index("async fn handle_turn_interrupted(", start)
        body = self.handler[start:end]
        self.assertLess(
            body.index("for signal in adaptive_runtime_signals {"),
            body.index("emit_turn_completed_with_status("),
        )

    def test_failed_or_interrupted_turn_does_not_terminally_replay(self):
        start = self.handler.index("async fn handle_turn_interrupted(")
        end = self.handler.index("async fn respond_to_pending_interrupts(", start)
        self.assertNotIn(
            "for signal in adaptive_runtime_signals", self.handler[start:end]
        )
        self.assertIn("TurnStatus::Interrupted", self.handler[start:end])

    def test_fork_native_rust_test_preserves_order_and_identity(self):
        self.assertIn(
            "async fn adaptive_runtime_signal_binds_event_turn_and_conversation_identity()",
            self.handler,
        )
        self.assertIn(
            "trusted adaptive signal must be replayed immediately before turn completion",
            self.handler,
        )
        self.assertIn("replayed_signal, expected_signal,", self.handler)

    def test_official_realtime_session_tracking_was_not_reverted(self):
        self.assertIn(
            "outgoing.track_realtime_session_updated(realtime_session_id)",
            self.handler,
        )
        self.assertNotIn("RealtimeEvent::SessionUpdated { .. } => {}", self.handler)

    def test_official_listener_replacement_cleanup_retained(self):
        self.assertIn("pub(crate) type PendingThreadUnloads =", self.state)
        self.assertIn("pub(crate) async fn remove_unsubscribed_listener(", self.state)
        self.assertIn("listener_generation", self.state)

    def test_official_read_state_change_media_passthrough_retained(self):
        self.assertIn("ServerNotification::ThreadReadStateChanged(_)", self.media)
        self.assertIn(
            "ServerNotification::AdaptiveRuntimeSignal(_) => notification",
            self.media,
        )

    def test_notification_filter_does_not_extract_signal_evidence_as_media(self):
        self.assertEqual(
            self.media.count("ServerNotification::AdaptiveRuntimeSignal(_)"), 1
        )
        self.assertNotIn("without_thread_item_media(notification.signal", self.media)

    def test_product_markers_and_validation_boundary_unmodified(self):
        self.assertEqual(committed("codex-rs/codexdd-version.txt").strip(), b"0.4.4")
        self.assertEqual(
            committed("codex-rs/upstream-codex-release.txt").strip(),
            b"rust-v0.162.0",
        )
        doc = (
            ROOT / "docs/codexdd-0.4.5-packet-3b3-adaptive-event-replay.md"
        ).read_text(encoding="utf-8")
        self.assertIn("INCOMPLETE / NOT A RELEASE CANDIDATE", doc)
        self.assertIn("native Rust", doc)
        self.assertIn("109", doc)


if __name__ == "__main__":
    unittest.main()
