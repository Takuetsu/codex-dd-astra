use super::*;
use crate::adaptive_complexity::AdaptiveComplexityClass;
use crate::adaptive_evidence::AUTO_FAILURE_PRESSURE_DIAGNOSTIC;
use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveFamily;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use crate::chatwidget::adaptive_effort::AdaptivePendingDecision;
use crate::chatwidget::adaptive_effort::AdaptivePendingSignal;
use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;
use codex_app_server_protocol::AdaptiveRuntimeSignalNotification;
use codex_app_server_protocol::CommandExecutionSource;
use codex_app_server_protocol::CommandExecutionStatus;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::ThreadItem;
use codex_protocol::protocol::AdaptiveRuntimeSignalKind;
use codex_utils_absolute_path::AbsolutePathBuf;

fn failed_command(thread_id: ThreadId, turn_id: &str, item_id: &str) -> ItemCompletedNotification {
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
    assert_eq!(
        chat.adaptive_effort.current_family,
        Some(AdaptiveFamily::Luna)
    );
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Low)
    );
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-1"));
    assert!(chat.adaptive_effort.pending_signal.is_none());
    assert!(
        chat.adaptive_effort_status_text()
            .contains("Failure pressure: 1/2")
    );

    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-2"));
    assert!(
        chat.adaptive_effort_status_text()
            .contains("Failure pressure: 2/2")
    );
    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Pending(ref signal))
            if signal.signal_kind == AdaptiveRuntimeSignalKind::Capability
                && signal.diagnostic_note.as_deref()
                    == Some(AUTO_FAILURE_PRESSURE_DIAGNOSTIC)
    ));

    chat.turn_lifecycle.agent_turn_running = false;
    assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
    assert_eq!(
        chat.adaptive_effort.current_family,
        Some(AdaptiveFamily::Luna)
    );
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
    assert_eq!(
        chat.adaptive_effort.worker_context.role,
        AdaptiveWorkerRole::Implementation
    );
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_eq!(chat.adaptive_effort.pending_attempt, None);
    assert_eq!(chat.adaptive_effort.successor_admission, None);
    assert_eq!(
        chat.adaptive_effort.current_family,
        Some(AdaptiveFamily::Luna)
    );
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Low)
    );
}

#[tokio::test]
async fn validation_failure_then_success_latches_owner_qa_terminal() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let validation_turn = "validation-turn";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Validation;
    chat.adaptive_effort.worker_context.authorized_scope = Some("validation-scope".to_string());
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(validation_turn.to_string());
    chat.register_adaptive_evidence(&failed_command(
        thread_id,
        validation_turn,
        "validation-failure",
    ));
    let mut success = failed_command(thread_id, validation_turn, "validation-success");
    if let ThreadItem::CommandExecution {
        status, exit_code, ..
    } = &mut success.item
    {
        *status = CommandExecutionStatus::Completed;
        *exit_code = Some(0);
    }
    chat.register_adaptive_evidence(&success);
    chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
        thread_id: thread_id.to_string(),
        signal: AdaptiveRuntimeSignalEnvelope {
            source_turn_id: validation_turn.to_string(),
            signal_kind: AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
            evidence_refs: vec!["validation-success".to_string()],
            diagnostic_note: None,
        },
    });
    chat.turn_lifecycle.agent_turn_running = false;

    assert!(chat.consume_adaptive_signal_at_terminal(validation_turn));
    assert_eq!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa)
    );
    assert_eq!(chat.adaptive_effort.pending_attempt, None);
    assert_eq!(chat.adaptive_effort.successor_admission, None);
    assert!(
        chat.adaptive_effort_status_text()
            .contains("Failure pressure: 1/2")
    );
    assert!(!chat.maybe_submit_adaptive_successor());
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
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Medium)
    );
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
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Medium)
    );
    assert_ne!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa)
    );
}

#[tokio::test]
async fn conflicting_workflow_signals_before_pressure_do_not_suppress_escalation() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "conflict-before-pressure";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    for signal_kind in [
        AdaptiveRuntimeSignalKind::ReadyForValidation,
        AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
    ] {
        chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
            thread_id: thread_id.to_string(),
            signal: AdaptiveRuntimeSignalEnvelope {
                source_turn_id: turn_id.to_string(),
                signal_kind,
                evidence_refs: Vec::new(),
                diagnostic_note: None,
            },
        });
    }
    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Conflicted { ref source_turn_id })
            if source_turn_id == turn_id
    ));

    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-1"));
    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-2"));
    chat.turn_lifecycle.agent_turn_running = false;

    assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Medium)
    );
    assert!(chat.adaptive_effort.workflow_terminal.is_none());
}

#[tokio::test]
async fn conflicting_workflow_signals_after_pressure_do_not_suppress_escalation() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "conflict-after-pressure";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-1"));
    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-2"));
    for signal_kind in [
        AdaptiveRuntimeSignalKind::ReadyForValidation,
        AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
    ] {
        chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
            thread_id: thread_id.to_string(),
            signal: AdaptiveRuntimeSignalEnvelope {
                source_turn_id: turn_id.to_string(),
                signal_kind,
                evidence_refs: Vec::new(),
                diagnostic_note: None,
            },
        });
    }
    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Conflicted { .. })
    ));

    chat.turn_lifecycle.agent_turn_running = false;
    assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Medium)
    );
    assert!(chat.adaptive_effort.workflow_terminal.is_none());
}

#[tokio::test]
async fn explicit_cancellation_suppresses_available_failure_pressure() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "cancelled-pressure";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-1"));
    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-2"));
    chat.cancel_pending_adaptive_signal_for_active_turn();
    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Cancelled { ref source_turn_id })
            if source_turn_id == turn_id
    ));

    chat.turn_lifecycle.agent_turn_running = false;
    assert!(!chat.consume_adaptive_signal_at_terminal(turn_id));
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Low)
    );
}

#[tokio::test]
async fn successful_work_without_handoff_continues_once_then_escalates() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.adaptive_effort.worker_context.authorized_scope = Some("cycle-vengeance/A-B1".to_string());
    let worker_context = chat.adaptive_effort.worker_context.clone();

    let mut success = failed_command(thread_id, "unfinished-1", "successful-check");
    if let ThreadItem::CommandExecution {
        status, exit_code, ..
    } = &mut success.item
    {
        *status = CommandExecutionStatus::Completed;
        *exit_code = Some(0);
    }
    chat.register_adaptive_evidence(&success);
    assert!(
        chat.adaptive_effort_status_text()
            .contains("Failure pressure: 0/2")
    );

    assert!(chat.apply_adaptive_unfinished_authorized_turn("unfinished-1"));
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 1);
    assert_eq!(chat.adaptive_effort.attempt_number, 2);
    assert_eq!(chat.adaptive_effort.worker_context, worker_context);
    assert_eq!(
        chat.adaptive_effort.current_family,
        Some(AdaptiveFamily::Luna)
    );
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Low)
    );
    assert_matches!(
        chat.adaptive_effort.pending_attempt,
        Some(ref pending)
            if pending.decision == AdaptivePendingDecision::ContinueSameRoute
                && pending.attempt_number == 2
                && pending.worker_context == worker_context
    );
    assert!(!chat.apply_adaptive_unfinished_authorized_turn("unfinished-1"));
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 1);

    assert!(chat.apply_adaptive_unfinished_authorized_turn("unfinished-2"));
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 0);
    assert_eq!(chat.adaptive_effort.attempt_number, 3);
    assert_eq!(chat.adaptive_effort.worker_context, worker_context);
    assert_eq!(
        chat.adaptive_effort.current_family,
        Some(AdaptiveFamily::Luna)
    );
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Medium)
    );
    assert_matches!(
        chat.adaptive_effort.pending_attempt,
        Some(ref pending)
            if pending.decision == AdaptivePendingDecision::EscalateEffort
                && pending.attempt_number == 3
                && pending.worker_context == worker_context
    );
    assert_eq!(chat.adaptive_effort.last_failure_kind, None);
}

#[tokio::test]
async fn unfinished_auto_continuation_requires_bound_authorized_scope() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.adaptive_effort.worker_context.authorized_scope = None;

    assert!(!chat.apply_adaptive_unfinished_authorized_turn("unbound-turn"));
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 0);
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_eq!(chat.adaptive_effort.pending_attempt, None);

    chat.adaptive_effort.worker_context.authorized_scope = Some("   ".to_string());
    assert!(!chat.apply_adaptive_unfinished_authorized_turn("blank-scope-turn"));
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 0);
}

#[tokio::test]
async fn trusted_handoff_resets_unfinished_pressure() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.adaptive_effort.worker_context.authorized_scope = Some("slice-reset".to_string());

    assert!(chat.apply_adaptive_unfinished_authorized_turn("unfinished-before-handoff"));
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 1);

    chat.adaptive_effort.last_processed_terminal_turn_id = None;
    chat.adaptive_effort.pending_signal = Some(AdaptivePendingSignal::Pending(
        AdaptiveRuntimeSignalEnvelope {
            source_turn_id: "handoff-turn".to_string(),
            signal_kind: AdaptiveRuntimeSignalKind::ReadyForValidation,
            evidence_refs: Vec::new(),
            diagnostic_note: None,
        },
    ));
    assert!(chat.consume_adaptive_signal_at_terminal("handoff-turn"));
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 0);
    assert_eq!(
        chat.adaptive_effort.worker_context.role,
        AdaptiveWorkerRole::Implementation
    );
    assert_eq!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::ReadyForValidation)
    );
    assert_eq!(chat.adaptive_effort.pending_attempt, None);
    assert_eq!(chat.adaptive_effort.successor_admission, None);
}

fn completed_turn_with_adaptive_report(turn_id: &str) -> TurnCompletedNotification {
    TurnCompletedNotification {
        thread_id: String::new(),
        turn: AppServerTurn {
            id: turn_id.to_string(),
            items: vec![ThreadItem::FunctionCallOutput {
                id: "adaptive-signal-output".to_string(),
                name: "report_adaptive_signal".to_string(),
                namespace: None,
                output: codex_protocol::models::FunctionCallOutputBody::Text(
                    "{\"status\":\"request_emitted\"}".to_string(),
                ),
            }],
            items_view: codex_app_server_protocol::TurnItemsView::Full,
            status: AppServerTurnStatus::Completed,
            error: None,
            started_at: None,
            completed_at: None,
            duration_ms: Some(1),
        },
    }
}

#[tokio::test]
async fn completed_repair_waits_for_late_ready_for_validation_and_admits_no_successor() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "late-ready-for-validation";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Repair;
    chat.adaptive_effort.worker_context.authorized_scope =
        Some("codexdd/post-terminal-race-repair".to_string());
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    let mut completed = completed_turn_with_adaptive_report(turn_id);
    completed.thread_id = thread_id.to_string();
    chat.handle_turn_completed_notification(completed.clone(), None);

    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Awaiting { ref source_turn_id })
            if source_turn_id == turn_id
    ));
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_eq!(chat.adaptive_effort.pending_attempt, None);
    assert_eq!(chat.adaptive_effort.successor_admission, None);
    assert_no_submit_op(&mut op_rx);

    chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
        thread_id: thread_id.to_string(),
        signal: AdaptiveRuntimeSignalEnvelope {
            source_turn_id: turn_id.to_string(),
            signal_kind: AdaptiveRuntimeSignalKind::ReadyForValidation,
            evidence_refs: Vec::new(),
            diagnostic_note: Some("READY_FOR_VALIDATION".to_string()),
        },
    });

    assert_eq!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::ReadyForValidation)
    );
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 0);
    assert_eq!(chat.adaptive_effort.pending_attempt, None);
    assert_eq!(chat.adaptive_effort.successor_admission, None);
    assert!(!chat.maybe_submit_adaptive_successor());
    assert_no_submit_op(&mut op_rx);

    chat.handle_turn_completed_notification(completed, None);
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn completed_validation_waits_for_late_owner_qa_and_admits_no_successor() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "late-ready-for-owner-qa";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Validation;
    chat.adaptive_effort.worker_context.authorized_scope =
        Some("codexdd/post-terminal-race-validation".to_string());
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    let mut success = failed_command(thread_id, turn_id, "late-validation-success");
    if let ThreadItem::CommandExecution {
        status, exit_code, ..
    } = &mut success.item
    {
        *status = CommandExecutionStatus::Completed;
        *exit_code = Some(0);
    }
    chat.register_adaptive_evidence(&success);

    let mut completed = completed_turn_with_adaptive_report(turn_id);
    completed.thread_id = thread_id.to_string();
    chat.handle_turn_completed_notification(completed, None);

    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Awaiting { ref source_turn_id })
            if source_turn_id == turn_id
    ));
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_no_submit_op(&mut op_rx);

    chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
        thread_id: thread_id.to_string(),
        signal: AdaptiveRuntimeSignalEnvelope {
            source_turn_id: turn_id.to_string(),
            signal_kind: AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
            evidence_refs: vec!["late-validation-success".to_string()],
            diagnostic_note: Some("PASS - READY FOR REPOSITORY HANDOFF".to_string()),
        },
    });

    assert_eq!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa)
    );
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 0);
    assert_eq!(chat.adaptive_effort.pending_attempt, None);
    assert_eq!(chat.adaptive_effort.successor_admission, None);
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn completed_complexity_recon_waits_for_late_signal_before_selecting_floor() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "late-complexity";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.adaptive_effort.worker_context.authorized_scope =
        Some("codexdd/0.3.1-late-complexity".to_string());
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    let mut completed = completed_turn_with_adaptive_report(turn_id);
    completed.thread_id = thread_id.to_string();
    chat.handle_turn_completed_notification(completed, None);

    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Awaiting { ref source_turn_id })
            if source_turn_id == turn_id
    ));
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_eq!(
        chat.adaptive_effort.current_family,
        Some(AdaptiveFamily::Luna)
    );
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Low)
    );
    assert_no_submit_op(&mut op_rx);

    chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
        thread_id: thread_id.to_string(),
        signal: AdaptiveRuntimeSignalEnvelope {
            source_turn_id: turn_id.to_string(),
            signal_kind: AdaptiveRuntimeSignalKind::Complexity,
            evidence_refs: Vec::new(),
            diagnostic_note: Some("architectural".to_string()),
        },
    });

    assert_eq!(
        chat.adaptive_effort.complexity_class,
        Some(AdaptiveComplexityClass::Architectural)
    );
    assert_eq!(
        chat.adaptive_effort.current_family,
        Some(AdaptiveFamily::Terra)
    );
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Medium)
    );
    assert_eq!(chat.adaptive_effort.attempt_number, 2);
    assert_matches!(
        chat.adaptive_effort.pending_attempt,
        Some(ref pending)
            if pending.decision == AdaptivePendingDecision::EscalateModel
                && pending.route.family == AdaptiveFamily::Terra
                && pending.route.effort == AdaptiveEffort::Medium
                && pending.attempt_number == 2
    );
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn late_workflow_terminal_still_overrides_synthetic_failure_pressure() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "late-terminal-overrides-pressure";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.adaptive_effort.worker_context.authorized_scope =
        Some("codexdd/late-terminal-overrides-pressure".to_string());
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-a"));
    chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-b"));
    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Pending(ref signal))
            if signal.signal_kind == AdaptiveRuntimeSignalKind::Capability
                && signal.diagnostic_note.as_deref()
                    == Some(AUTO_FAILURE_PRESSURE_DIAGNOSTIC)
    ));

    let mut completed = completed_turn_with_adaptive_report(turn_id);
    completed.thread_id = thread_id.to_string();
    chat.handle_turn_completed_notification(completed, None);

    assert!(matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Awaiting { ref source_turn_id })
            if source_turn_id == turn_id
    ));
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Low)
    );
    assert_no_submit_op(&mut op_rx);

    chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
        thread_id: thread_id.to_string(),
        signal: AdaptiveRuntimeSignalEnvelope {
            source_turn_id: turn_id.to_string(),
            signal_kind: AdaptiveRuntimeSignalKind::ReadyForValidation,
            evidence_refs: Vec::new(),
            diagnostic_note: None,
        },
    });

    assert_eq!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::ReadyForValidation)
    );
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Low)
    );
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 0);
    assert_eq!(chat.adaptive_effort.pending_attempt, None);
    assert_eq!(chat.adaptive_effort.successor_admission, None);
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn completed_bound_worker_without_adaptive_report_keeps_normal_unfinished_continuation() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    let turn_id = "ordinary-unfinished-completion";
    chat.thread_id = Some(thread_id);
    chat.dispatch_adaptive_command("astra");
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
    chat.adaptive_effort.worker_context.authorized_scope =
        Some("codexdd/ordinary-continuation".to_string());
    {
        let mask = chat
            .active_collaboration_mask
            .as_mut()
            .expect("test chat has a collaboration mask");
        mask.model = Some(AdaptiveFamily::Luna.model().to_string());
        mask.reasoning_effort = Some(Some(codex_protocol::openai_models::ReasoningEffort::Low));
    }
    chat.turn_lifecycle.agent_turn_running = true;
    chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

    chat.handle_turn_completed_notification(
        TurnCompletedNotification {
            thread_id: thread_id.to_string(),
            turn: AppServerTurn {
                id: turn_id.to_string(),
                items: Vec::new(),
                items_view: codex_app_server_protocol::TurnItemsView::Full,
                status: AppServerTurnStatus::Completed,
                error: None,
                started_at: None,
                completed_at: None,
                duration_ms: Some(1),
            },
        },
        None,
    );

    assert_eq!(chat.adaptive_effort.workflow_terminal, None);
    assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 1);
    assert_eq!(chat.adaptive_effort.attempt_number, 2);
    assert!(chat.adaptive_effort.pending_signal.is_none());
    assert_matches!(chat.adaptive_effort.pending_attempt, None);
    let submitted = next_submit_op(&mut op_rx);
    assert!(matches!(submitted, Op::UserTurn { .. }));
}
