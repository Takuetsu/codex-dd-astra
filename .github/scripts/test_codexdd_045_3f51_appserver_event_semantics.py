"""3F.51: source-only app-server dispatch, replay, and authority invariants.

This is a fail-closed source proof; native FIFO delivery and reconnect remain
explicit release blockers until exercised on the exact frozen Windows source.
"""
from __future__ import annotations

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PROTO = ROOT / "codex-rs/app-server-protocol/src/protocol/common.rs"
SERVER = ROOT / "codex-rs/app-server/src"
TUI = ROOT / "codex-rs/tui/src"


class AppServerSemanticContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.proto = PROTO.read_text(encoding="utf-8")
        cls.event = (SERVER / "bespoke_event_handling.rs").read_text(encoding="utf-8")
        cls.processor = (SERVER / "request_processors/turn_processor.rs").read_text(encoding="utf-8")
        cls.dispatcher = (SERVER / "message_processor.rs").read_text(encoding="utf-8")
        cls.thread_state = (SERVER / "thread_state.rs").read_text(encoding="utf-8")

    def test_workflow_mutations_are_queued_by_thread_and_persist_private_rollout_state(self):
        start = self.proto.index('ThreadWorkflowStateUpdate => "thread/workflowState/update"')
        definition = self.proto[start:start + 300]
        self.assertIn("serialization: thread_id(params.thread_id)", definition)
        self.assertNotIn("serialization: None", definition)
        self.assertIn("ClientRequest::ThreadWorkflowStateUpdate { params, .. }", self.dispatcher)
        self.assertIn(".thread_workflow_state_update(params)", self.dispatcher)
        start = self.processor.index("pub(crate) async fn thread_workflow_state_update(")
        end = self.processor.index("pub(crate) async fn thread_settings_update(", start)
        body = self.processor[start:end]
        self.assertIn("self.load_thread(&params.thread_id).await?", body)
        self.assertIn("ThreadWorkflowStateOperation::SetReadyForOwnerQa", body)
        self.assertIn("ThreadWorkflowStateOperation::SetAdaptiveState", body)
        self.assertIn("ThreadWorkflowStateOperation::Clear", body)
        self.assertIn("setAdaptiveState requires adaptiveState", body)
        self.assertIn(".persist_workflow_state(item)", body)
        self.assertNotIn("add_user_message", body)

    def test_adaptive_signal_is_source_turn_scoped_and_replayed_before_successful_terminal(self):
        start = self.event.index("async fn handle_adaptive_runtime_signal(")
        end = self.event.index("pub(crate) async fn apply_bespoke_event_handling(", start)
        body = self.event[start:end]
        for part in ("source_turn_id,", "signal_kind: signal.signal_kind", "evidence_refs: signal.evidence_refs", ".adaptive_runtime_signals", ".push(signal.clone())", "send_server_notification(ServerNotification::AdaptiveRuntimeSignal("):
            self.assertIn(part, body)
        self.assertIn("EventMsg::AdaptiveRuntimeSignal(signal) => {", self.event)
        self.assertIn("event_turn_id,", self.event)
        start = self.event.index("async fn handle_turn_complete(")
        end = self.event.index("async fn handle_turn_interrupted(", start)
        completed = self.event[start:end]
        guard = completed.index("if matches!(status, TurnStatus::Completed) {")
        replay = completed.index("for signal in adaptive_runtime_signals {")
        notify = completed.index("emit_turn_completed_with_status(")
        self.assertLess(guard, replay)
        self.assertLess(replay, notify)
        self.assertIn("TurnStatus::Failed", completed)
        interrupted = self.event[end:self.event.index("async fn respond_to_pending_interrupts(", end)]
        self.assertIn("TurnStatus::Interrupted", interrupted)
        self.assertNotIn("for signal in adaptive_runtime_signals", interrupted)
        self.assertIn("std::mem::take(&mut state.turn_summary)", self.event)

    def test_per_thread_event_state_has_explicit_terminal_turn_tracking(self):
        self.assertIn("pub(crate) adaptive_runtime_signals: Vec<AdaptiveRuntimeSignalEnvelope>", self.thread_state)
        self.assertIn("pub(crate) last_terminal_turn_id: Option<String>", self.thread_state)
        self.assertIn("self.last_terminal_turn_id = Some(event_turn_id.to_string());", self.thread_state)
        self.assertIn("EventMsg::TurnAborted(_) | EventMsg::TurnComplete(_)", self.thread_state)
        self.assertIn("self.current_turn_history.reset();", self.thread_state)

    def test_tui_typed_rpc_provides_thread_and_turn_provenance(self):
        tui_session = (TUI / "app_server_session.rs").read_text(encoding="utf-8")
        begin = tui_session.index("pub(crate) async fn thread_workflow_state_update(")
        end = tui_session.index("pub(crate) async fn turn_start(", begin)
        body = tui_session[begin:end]
        self.assertIn("ThreadWorkflowStateUpdateParams {", body)
        self.assertIn("thread_id: thread_id.to_string()", body)
        self.assertIn("operation,", body)
        self.assertIn("source_turn_id,", body)
        self.assertIn("adaptive_state,", body)
        self.assertIn("ClientRequest::ThreadWorkflowStateUpdate", body)
        self.assertNotIn("RequestId::null", body)

    def test_core_readonly_rejects_hook_rewrite_and_completes_denied_tool(self):
        registry = (ROOT / "codex-rs/core/src/tools/registry.rs").read_text(encoding="utf-8")
        early = registry.index("if let Some(message) = adaptive_reconnaissance_rejection(&invocation)")
        hook = registry.index("match run_pre_tool_use_hooks(", early)
        late = registry.index("if let Some(message) = adaptive_reconnaissance_rejection(&invocation)", hook)
        start_handler = registry.index("notify_tool_start(&invocation", late)
        self.assertLess(early, hook)
        self.assertLess(hook, late)
        self.assertLess(late, start_handler)
        self.assertIn("ToolCallOutcome::Blocked", registry[late:start_handler])
        self.assertIn("PreToolUse hook may have rewritten the invocation", registry[hook:late])
        interaction = (TUI / "chatwidget/interaction.rs").read_text(encoding="utf-8")
        self.assertIn("handle_key_event_with_activation_key(key_event", interaction)
        self.assertIn("let is_manual_esc_interrupt = should_pause_active_goal", interaction)
        self.assertIn("key_event.kind == KeyEventKind::Press", interaction)


if __name__ == "__main__":
    unittest.main()
