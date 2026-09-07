use super::*;
use crate::adaptive_evidence::AUTO_FAILURE_PRESSURE_DIAGNOSTIC;
use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveFamily;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use crate::chatwidget::adaptive_effort::AdaptivePendingSignal;
use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;
use codex_app_server_protocol::AdaptiveRuntimeSignalNotification;
use codex_app_server_protocol::CommandExecutionSource;
use codex_app_server_protocol::CommandExecutionStatus;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::ThreadItem;
use codex_protocol::protocol::AdaptiveRuntimeSignalKind;
use codex_utils_absolute_path::AbsolutePathBuf;

fn failed_command(
    thread_id: ThreadId,
    turn_id: &str,
    item_id: &str,
) -> ItemCompletedNotification {
    ItemCompletedNotification {
        item: ThreadItem::CommandExecution {
            id: item_id.to_string(),
            plugin_id: None,
            script_path: None,
            command: "test command".to_string(),
            cwd: AbsolutePathBuf::from_absolute_path(std::env::current_dir().expect("cwd"))
                .expect("absolute cwd")
                .into(),
            process_id: None,
            source: CommandExecutionSource::Agent,
            status: CommandExecutionStatus::Failed,
            command_actions: Vec::new(),
            aggregated_output: None,
            exit_code: Some(1),
            duration_ms: Some(1),
        },
        thread_id: thread_id.to_string(),
        turn_id: turn_id.to_string(),
        completed_at_ms: 1,
    }
}

#[tokio::test]
async fn two_native_failures_arm_capability_and_advance_luna_low_to_medium() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "pressure-turn";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    assert_eq!(chat.adaptive_effort.current_family, Some(AdaptiveFamily::Luna));
    assert_eq!(chat.adaptive_effort.current_effort, Some(AdaptiveEffort::Low));
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-1"));
    assert!(chat.adaptive_effort.pending_signal.is_none());
    assert!(chat.adaptive_effort_status_text().contains("Failure pressure: 1/2"));

    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-2"));
    assert!(chat.adaptive_effort_status_text().contains("Failure pressure: 2/2"));
    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Pending(ref signal))
            if signal.signal_kind == AdaptiveRuntimeSignalKind::Capability
                && signal.diagnostic_note.as_deref()
                    == Some(AUTO_FAILURE_PRESSURE_DIAGNOSTIC)
    ));

    chat.turn_lifecycle.agent_turn_running = false;
    assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
    assert_eq!(chat.adaptive_effort.current_family, Some(AdaptiveFamily::Luna));
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Medium)
    );
}

#[tokio::test]
async fn workflow_success_overrides_automatic_failure_pressure() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "recovered-turn";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-1"));
    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-2"));
    chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
        thread_id: thread_id.to_string(),
        signal: AdaptiveRuntimeSignalEnvelope {
            source_turn_id: turn_id.to_string(),
            signal_kind: AdaptiveRuntimeSignalKind::ReadyForValidation,
            evidence_refs: Vec::new(),
            diagnostic_note: None,
        },
    });

    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Pending(ref signal))
            if signal.signal_kind == AdaptiveRuntimeSignalKind::ReadyForValidation
    ));
    chat.turn_lifecycle.agent_turn_running = false;
    assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
    assert_eq!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::ReadyForValidation)
    );
    assert_eq!(chat.adaptive_effort.current_family, Some(AdaptiveFamily::Luna));
    assert_eq!(chat.adaptive_effort.current_effort, Some(AdaptiveEffort::Low));
}

#[tokio::test]
async fn invalid_workflow_signal_before_pressure_does_not_suppress_escalation() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "invalid-before-pressure";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
        thread_id: thread_id.to_string(),
        signal: AdaptiveRuntimeSignalEnvelope {
            source_turn_id: turn_id.to_string(),
            signal_kind: AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
            evidence_refs: Vec::new(),
            diagnostic_note: None,
        },
    });
    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-1"));
    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-2"));

    chat.turn_lifecycle.agent_turn_running = false;
    assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
    assert_eq!(chat.adaptive_effort.current_effort, Some(AdaptiveEffort::Medium));
    assert_ne!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa)
    );
}

#[tokio::test]
async fn invalid_workflow_signal_after_pressure_does_not_suppress_escalation() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "invalid-after-pressure";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-1"));
    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-2"));
    chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
        thread_id: thread_id.to_string(),
        signal: AdaptiveRuntimeSignalEnvelope {
            source_turn_id: turn_id.to_string(),
            signal_kind: AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
            evidence_refs: Vec::new(),
            diagnostic_note: None,
        },
    });

    chat.turn_lifecycle.agent_turn_running = false;
    assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
    assert_eq!(chat.adaptive_effort.current_effort, Some(AdaptiveEffort::Medium));
    assert_ne!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa)
    );
}
