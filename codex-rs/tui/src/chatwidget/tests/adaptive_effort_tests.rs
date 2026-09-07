use super::*;
use crate::adaptive_evidence::AdaptiveEvidenceKind;
use crate::adaptive_evidence::AdaptiveEvidenceOutcome;
use crate::adaptive_evidence::AdaptiveEvidenceRecord;
use crate::adaptive_policy::AdaptiveClassification;
use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveFailureKind;
use crate::adaptive_policy::AdaptiveFamily;
use crate::adaptive_policy::AdaptiveOutcomeSignal;
use crate::adaptive_worker::AdaptiveWorkerContext;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use crate::chatwidget::adaptive_admission::AdaptiveAdmissionRejectedReason;
use crate::chatwidget::adaptive_admission::AdaptiveAdmissionResult;
use crate::chatwidget::adaptive_admission::AdaptiveAdmissionSuppressedReason;
use crate::chatwidget::adaptive_admission::AdaptiveAdmissionWaitingReason;
use crate::chatwidget::adaptive_effort::AdaptiveEffortState;
use crate::chatwidget::adaptive_effort::AdaptiveOutcome;
use crate::chatwidget::adaptive_effort::AdaptivePendingAttempt;
use crate::chatwidget::adaptive_effort::AdaptivePendingDecision;
use crate::chatwidget::adaptive_effort::AdaptivePendingSignal;
use crate::chatwidget::adaptive_effort::AdaptiveSuccessorAdmission;
use crate::slash_command::built_in_slash_commands;
use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;
use codex_app_server_protocol::AdaptiveRuntimeSignalNotification;
use codex_app_server_protocol::CommandExecutionSource;
use codex_app_server_protocol::CommandExecutionStatus;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::ThreadItem;
use codex_protocol::openai_models::ReasoningEffort;
use codex_protocol::protocol::AdaptiveRuntimeSignalKind;
use std::str::FromStr;

fn configure_admission(
    chat: &mut ChatWidget,
    decision: AdaptivePendingDecision,
    route: crate::adaptive_policy::AdaptiveRoute,
) {
    let thread_id = chat.thread_id.expect("thread configured");
    chat.adaptive_effort.enabled = true;
    chat.adaptive_effort.starting_family = Some(route.family);
    chat.adaptive_effort.current_family = Some(route.family);
    chat.adaptive_effort.current_effort = Some(route.effort);
    chat.adaptive_effort.attempt_number = 2;
    chat.adaptive_effort.worker_context = AdaptiveWorkerContext {
        role: AdaptiveWorkerRole::Implementation,
        authorized_scope: Some("slice-7d-a".to_string()),
    };
    chat.adaptive_effort.last_processed_terminal_turn_id = Some("source-turn".to_string());
    chat.adaptive_effort.pending_attempt = Some(AdaptivePendingAttempt {
        thread_id,
        source_turn_id: "source-turn".to_string(),
        decision,
        route,
        attempt_number: 2,
        worker_context: chat.adaptive_effort.worker_context.clone(),
    });
    chat.turn_lifecycle.last_turn_id = Some("source-turn".to_string());
}

fn synchronize_admission_route(
    chat: &mut ChatWidget,
    route: crate::adaptive_policy::AdaptiveRoute,
) {
    let mask = chat
        .active_collaboration_mask
        .as_mut()
        .expect("test chat has a collaboration mask");
    mask.model = Some(route.family.model().to_string());
    mask.reasoning_effort = Some(Some(match route.effort {
        AdaptiveEffort::Low => ReasoningEffort::Low,
        AdaptiveEffort::Medium => ReasoningEffort::Medium,
        AdaptiveEffort::High => ReasoningEffort::High,
        AdaptiveEffort::XHigh => ReasoningEffort::XHigh,
        AdaptiveEffort::Max => ReasoningEffort::Max,
    }));
}

fn admission_route(
    family: AdaptiveFamily,
    effort: AdaptiveEffort,
) -> crate::adaptive_policy::AdaptiveRoute {
    crate::adaptive_policy::AdaptiveRoute { family, effort }
}

#[tokio::test]
async fn adaptive_successor_submits_each_authorized_decision_once() {
    for (decision, route) in [
        (
            AdaptivePendingDecision::RetrySameLevel,
            admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Low),
        ),
        (
            AdaptivePendingDecision::EscalateEffort,
            admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Medium),
        ),
        (
            AdaptivePendingDecision::EscalateModel,
            admission_route(AdaptiveFamily::Sol, AdaptiveEffort::Low),
        ),
    ] {
        let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
        chat.thread_id = Some(ThreadId::new());
        configure_admission(&mut chat, decision, route);
        synchronize_admission_route(&mut chat, route);

        assert!(chat.maybe_submit_adaptive_successor());
        let Op::UserTurn {
            items,
            model,
            effort,
            ..
        } = next_submit_op(&mut op_rx)
        else {
            panic!("expected native user turn");
        };
        let expected_effort = match route.effort {
            AdaptiveEffort::Low => Some(ReasoningEffort::Low),
            AdaptiveEffort::Medium => Some(ReasoningEffort::Medium),
            AdaptiveEffort::High => Some(ReasoningEffort::High),
            AdaptiveEffort::XHigh => Some(ReasoningEffort::XHigh),
            AdaptiveEffort::Max => Some(ReasoningEffort::Max),
        };
        assert_eq!(model, route.family.model());
        assert_eq!(effort, expected_effort);
        assert_matches!(
            items.as_slice(),
            [UserInput::Text { text, text_elements }]
                if text.starts_with("[Adaptive continuation]")
                    && text.contains("Attempt 2")
                    && text.contains(match decision {
                        AdaptivePendingDecision::RetrySameLevel => "RetrySameLevel",
                        AdaptivePendingDecision::EscalateEffort => "EscalateEffort",
                        AdaptivePendingDecision::EscalateModel => "EscalateModel",
                    })
                    && !text.contains("original root prompt")
                    && text_elements.is_empty()
        );
        assert_matches!(
            chat.adaptive_effort.successor_admission,
            Some(AdaptiveSuccessorAdmission::Consumed(_))
        );
        assert_eq!(chat.adaptive_effort.pending_attempt, None);
        assert!(!chat.maybe_submit_adaptive_successor());
        assert_no_submit_op(&mut op_rx);
    }
}

#[tokio::test]
async fn adaptive_successor_yields_to_manual_input_and_stale_route() {
    let route = admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Medium);

    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    configure_admission(&mut chat, AdaptivePendingDecision::EscalateEffort, route);
    synchronize_admission_route(&mut chat, route);
    chat.queue_user_message(UserMessage::from("human input"));
    assert_eq!(chat.adaptive_effort.pending_attempt, None);
    assert!(!chat.maybe_submit_adaptive_successor());
    assert_matches!(next_submit_op(&mut op_rx), Op::UserTurn { .. });
    assert_no_submit_op(&mut op_rx);

    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    configure_admission(&mut chat, AdaptivePendingDecision::EscalateEffort, route);
    synchronize_admission_route(&mut chat, route);
    assert_matches!(
        chat.reserve_adaptive_successor_admission("source-turn"),
        AdaptiveAdmissionResult::Authorized(_)
    );
    synchronize_admission_route(
        &mut chat,
        admission_route(AdaptiveFamily::Terra, AdaptiveEffort::High),
    );
    assert!(!chat.maybe_submit_adaptive_successor());
    assert_eq!(chat.adaptive_effort.successor_admission, None);
    assert_eq!(chat.adaptive_effort.pending_attempt, None);
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn adaptive_successor_submission_failure_is_consumed_and_new_permit_can_follow() {
    let route = admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Low);
    let (mut chat, _rx, op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    configure_admission(&mut chat, AdaptivePendingDecision::RetrySameLevel, route);
    synchronize_admission_route(&mut chat, route);
    drop(op_rx);

    assert!(!chat.maybe_submit_adaptive_successor());
    assert_matches!(
        chat.adaptive_effort.successor_admission,
        Some(AdaptiveSuccessorAdmission::Consumed(_))
    );
    assert!(!chat.maybe_submit_adaptive_successor());

    chat.input_queue.user_turn_pending_start = false;
    chat.adaptive_effort.successor_admission = None;
    chat.adaptive_effort.last_processed_terminal_turn_id = Some("later-turn".to_string());
    chat.turn_lifecycle.last_turn_id = Some("later-turn".to_string());
    chat.adaptive_effort.attempt_number = 3;
    chat.adaptive_effort.pending_attempt = Some(AdaptivePendingAttempt {
        thread_id: chat.thread_id.expect("thread configured"),
        source_turn_id: "later-turn".to_string(),
        decision: AdaptivePendingDecision::RetrySameLevel,
        route,
        attempt_number: 3,
        worker_context: chat.adaptive_effort.worker_context.clone(),
    });
    assert!(!chat.maybe_submit_adaptive_successor());
    assert_matches!(
        chat.adaptive_effort.successor_admission,
        Some(AdaptiveSuccessorAdmission::Consumed(ref permit))
            if permit.source_turn_id == "later-turn" && permit.attempt_number == 3
    );
}

#[tokio::test]
async fn admission_reserves_each_controller_authorized_decision_once() {
    for (decision, route) in [
        (
            AdaptivePendingDecision::RetrySameLevel,
            admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Low),
        ),
        (
            AdaptivePendingDecision::EscalateEffort,
            admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Medium),
        ),
        (
            AdaptivePendingDecision::EscalateModel,
            admission_route(AdaptiveFamily::Sol, AdaptiveEffort::Low),
        ),
    ] {
        let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
        chat.thread_id = Some(ThreadId::new());
        configure_admission(&mut chat, decision, route);
        synchronize_admission_route(&mut chat, route);

        let permit = match chat.reserve_adaptive_successor_admission("source-turn") {
            AdaptiveAdmissionResult::Authorized(permit) => permit,
            result => panic!("expected authorized admission, got {result:?}"),
        };
        assert_eq!(permit.decision, decision);
        assert_eq!(permit.route, route);
        assert_eq!(permit.attempt_number, 2);
        assert_matches!(
            chat.adaptive_effort.successor_admission,
            Some(AdaptiveSuccessorAdmission::Reserved(ref reserved)) if reserved == &permit
        );
        assert_eq!(
            chat.reserve_adaptive_successor_admission("source-turn"),
            AdaptiveAdmissionResult::Suppressed(AdaptiveAdmissionSuppressedReason::AlreadyReserved)
        );
        assert_eq!(chat.adaptive_effort.attempt_number, 2);
        assert_no_submit_op(&mut op_rx);
    }
}

#[tokio::test]
async fn admission_waits_for_effective_route_and_consumed_permit_cannot_replay() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    let route = admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Medium);
    configure_admission(&mut chat, AdaptivePendingDecision::EscalateEffort, route);

    assert_eq!(
        chat.reserve_adaptive_successor_admission("source-turn"),
        AdaptiveAdmissionResult::Waiting(
            AdaptiveAdmissionWaitingReason::EffectiveRouteSynchronization
        )
    );
    assert_eq!(chat.adaptive_effort.successor_admission, None);

    synchronize_admission_route(&mut chat, route);
    let permit = match chat.reserve_adaptive_successor_admission("source-turn") {
        AdaptiveAdmissionResult::Authorized(permit) => permit,
        result => panic!("expected authorized admission, got {result:?}"),
    };
    assert!(chat.consume_adaptive_successor_admission(&permit));
    assert!(!chat.consume_adaptive_successor_admission(&permit));
    assert_eq!(
        chat.reserve_adaptive_successor_admission("source-turn"),
        AdaptiveAdmissionResult::Suppressed(AdaptiveAdmissionSuppressedReason::AlreadyConsumed)
    );
    assert_eq!(chat.adaptive_effort.attempt_number, 2);
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn admission_suppresses_hard_terminals_and_user_controls() {
    for terminal in [
        AdaptiveWorkflowTerminal::ReadyForValidation,
        AdaptiveWorkflowTerminal::RepairRequired,
        AdaptiveWorkflowTerminal::ReadyForOwnerQa,
        AdaptiveWorkflowTerminal::Blocked,
    ] {
        let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
        chat.thread_id = Some(ThreadId::new());
        let route = admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Low);
        configure_admission(&mut chat, AdaptivePendingDecision::RetrySameLevel, route);
        synchronize_admission_route(&mut chat, route);
        chat.adaptive_effort.workflow_terminal = Some(terminal);
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Validation;
        assert_eq!(
            chat.reserve_adaptive_successor_admission("source-turn"),
            AdaptiveAdmissionResult::Suppressed(
                AdaptiveAdmissionSuppressedReason::WorkflowTerminal
            )
        );
        assert_eq!(chat.adaptive_effort.successor_admission, None);
        assert_eq!(chat.adaptive_effort.attempt_number, 2);
        assert_no_submit_op(&mut op_rx);
    }

    for control in ["off", "pause"] {
        let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
        chat.thread_id = Some(ThreadId::new());
        let route = admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Low);
        configure_admission(&mut chat, AdaptivePendingDecision::RetrySameLevel, route);
        synchronize_admission_route(&mut chat, route);
        chat.dispatch_adaptive_command(control);
        drain_events(&mut rx);
        assert!(matches!(
            chat.reserve_adaptive_successor_admission("source-turn"),
            AdaptiveAdmissionResult::Suppressed(
                AdaptiveAdmissionSuppressedReason::AdaptiveDisabled
                    | AdaptiveAdmissionSuppressedReason::PausedByUser
            )
        ));
        assert_eq!(chat.adaptive_effort.successor_admission, None);
        assert_no_submit_op(&mut op_rx);
    }
}

#[tokio::test]
async fn admission_rejects_stale_identity_and_worker_bindings() {
    let route = admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Low);
    for case in ["source", "thread", "attempt", "route", "role", "scope"] {
        let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
        chat.thread_id = Some(ThreadId::new());
        configure_admission(&mut chat, AdaptivePendingDecision::RetrySameLevel, route);
        synchronize_admission_route(&mut chat, route);
        let expected = match case {
            "source" => {
                chat.turn_lifecycle.last_turn_id = Some("other-turn".to_string());
                AdaptiveAdmissionRejectedReason::SourceTurnMismatch
            }
            "thread" => {
                chat.thread_id = Some(ThreadId::new());
                AdaptiveAdmissionRejectedReason::ThreadMismatch
            }
            "attempt" => {
                chat.adaptive_effort.attempt_number = 3;
                AdaptiveAdmissionRejectedReason::AttemptMismatch
            }
            "route" => {
                chat.adaptive_effort.current_effort = Some(AdaptiveEffort::Medium);
                AdaptiveAdmissionRejectedReason::RouteMismatch
            }
            "role" => {
                chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Repair;
                AdaptiveAdmissionRejectedReason::WorkerContextMismatch
            }
            "scope" => {
                chat.adaptive_effort.worker_context.authorized_scope = Some("other".to_string());
                AdaptiveAdmissionRejectedReason::WorkerContextMismatch
            }
            _ => unreachable!(),
        };
        assert_eq!(
            chat.reserve_adaptive_successor_admission("source-turn"),
            AdaptiveAdmissionResult::Rejected(expected),
            "{case}"
        );
        assert_eq!(chat.adaptive_effort.successor_admission, None);
        assert_no_submit_op(&mut op_rx);
    }
}

#[tokio::test]
async fn admission_reservations_are_invalidated_by_user_interrupt_and_new_controller_result() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    let route = admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Low);
    configure_admission(&mut chat, AdaptivePendingDecision::RetrySameLevel, route);
    synchronize_admission_route(&mut chat, route);
    assert_matches!(
        chat.reserve_adaptive_successor_admission("source-turn"),
        AdaptiveAdmissionResult::Authorized(_)
    );

    chat.on_adaptive_user_interrupt();
    assert_eq!(chat.adaptive_effort.successor_admission, None);

    chat.adaptive_effort.paused_by_user = false;
    configure_admission(&mut chat, AdaptivePendingDecision::RetrySameLevel, route);
    assert_matches!(
        chat.reserve_adaptive_successor_admission("source-turn"),
        AdaptiveAdmissionResult::Authorized(_)
    );
    chat.apply_adaptive_terminal_signal("new-controller-turn", AdaptiveOutcomeSignal::Failed);
    assert_eq!(chat.adaptive_effort.successor_admission, None);
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn admission_consumed_state_survives_resume_and_new_threads_start_isolated() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.thread_id = Some(thread_id);
    let route = admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Low);
    configure_admission(&mut chat, AdaptivePendingDecision::RetrySameLevel, route);
    synchronize_admission_route(&mut chat, route);
    let permit = match chat.reserve_adaptive_successor_admission("source-turn") {
        AdaptiveAdmissionResult::Authorized(permit) => permit,
        result => panic!("expected authorized admission, got {result:?}"),
    };
    assert!(chat.consume_adaptive_successor_admission(&permit));

    let mut resumed = adaptive_test_session(
        thread_id,
        /*forked_from_id*/ None,
        route.family.model(),
        Some(ReasoningEffort::Low),
    );
    resumed.adaptive_effort = chat.adaptive_effort.clone();
    chat.handle_thread_session_quiet(resumed);
    assert_matches!(
        chat.adaptive_effort.successor_admission,
        Some(AdaptiveSuccessorAdmission::Consumed(ref consumed)) if consumed == &permit
    );
    assert_eq!(
        chat.reserve_adaptive_successor_admission("source-turn"),
        AdaptiveAdmissionResult::Suppressed(AdaptiveAdmissionSuppressedReason::AlreadyConsumed)
    );

    let new_thread = adaptive_test_session(
        ThreadId::new(),
        /*forked_from_id*/ None,
        "gpt-5.6-luna",
        Some(ReasoningEffort::Low),
    );
    assert_eq!(new_thread.adaptive_effort, AdaptiveEffortState::default());
    assert_no_submit_op(&mut op_rx);
}

fn next_non_adaptive_state_event(
    rx: &mut tokio::sync::mpsc::UnboundedReceiver<AppEvent>,
) -> Result<AppEvent, tokio::sync::mpsc::error::TryRecvError> {
    loop {
        match rx.try_recv() {
            Ok(AppEvent::UpdateAdaptiveEffortState(_)) => {}
            event => return event,
        }
    }
}

fn drain_events(rx: &mut tokio::sync::mpsc::UnboundedReceiver<AppEvent>) -> Vec<AppEvent> {
    let mut events = Vec::new();
    while let Ok(event) = rx.try_recv() {
        events.push(event);
    }
    events
}

fn adaptive_runtime_signal(
    thread_id: ThreadId,
    source_turn_id: &str,
    signal_kind: AdaptiveRuntimeSignalKind,
) -> ServerNotification {
    ServerNotification::AdaptiveRuntimeSignal(AdaptiveRuntimeSignalNotification {
        thread_id: thread_id.to_string(),
        signal: AdaptiveRuntimeSignalEnvelope {
            source_turn_id: source_turn_id.to_string(),
            signal_kind,
            evidence_refs: Vec::new(),
            diagnostic_note: None,
        },
    })
}

fn pending_signal(
    source_turn_id: &str,
    signal_kind: AdaptiveRuntimeSignalKind,
    evidence_refs: Vec<String>,
) -> AdaptivePendingSignal {
    AdaptivePendingSignal::Pending(AdaptiveRuntimeSignalEnvelope {
        source_turn_id: source_turn_id.to_string(),
        signal_kind,
        evidence_refs,
        diagnostic_note: None,
    })
}

fn register_evidence(
    chat: &mut ChatWidget,
    evidence_id: &str,
    thread_id: ThreadId,
    outcome: AdaptiveEvidenceOutcome,
) {
    chat.adaptive_effort
        .evidence_registry
        .register(AdaptiveEvidenceRecord {
            evidence_id: evidence_id.to_string(),
            thread_id,
            source_turn_id: "evidence-turn".to_string(),
            outcome,
            kind: AdaptiveEvidenceKind::CommandExecution,
        });
}

#[tokio::test]
async fn ready_for_validation_enforces_complete_role_matrix_without_handoff() {
    for (role, accepted) in [
        (AdaptiveWorkerRole::Implementation, true),
        (AdaptiveWorkerRole::Repair, true),
        (AdaptiveWorkerRole::Validation, false),
        (AdaptiveWorkerRole::Unspecified, false),
    ] {
        let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
        chat.thread_id = Some(ThreadId::new());
        chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
        drain_events(&mut rx);
        chat.adaptive_effort.worker_context = AdaptiveWorkerContext {
            role,
            authorized_scope: Some("fixed-scope".to_string()),
        };
        let before = chat.adaptive_effort.clone();
        chat.adaptive_effort.pending_signal = Some(pending_signal(
            "role-turn",
            AdaptiveRuntimeSignalKind::ReadyForValidation,
            Vec::new(),
        ));

        assert_eq!(
            chat.consume_adaptive_signal_at_terminal("role-turn"),
            accepted
        );
        assert_eq!(
            chat.adaptive_effort.workflow_terminal,
            accepted.then_some(AdaptiveWorkflowTerminal::ReadyForValidation)
        );
        assert_eq!(chat.adaptive_effort.worker_context, before.worker_context);
        assert_eq!(chat.adaptive_effort.current_family, before.current_family);
        assert_eq!(chat.adaptive_effort.current_effort, before.current_effort);
        assert_eq!(chat.adaptive_effort.attempt_number, before.attempt_number);
        assert_eq!(chat.adaptive_effort.pending_attempt, None);
        assert_no_submit_op(&mut op_rx);
    }
}

#[tokio::test]
async fn evidence_workflow_terminals_enforce_complete_evidence_matrix() {
    for (kind, expected, terminal) in [
        (
            AdaptiveRuntimeSignalKind::RepairRequired,
            AdaptiveEvidenceOutcome::Failure,
            AdaptiveWorkflowTerminal::RepairRequired,
        ),
        (
            AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
            AdaptiveEvidenceOutcome::Success,
            AdaptiveWorkflowTerminal::ReadyForOwnerQa,
        ),
    ] {
        for (case, role, refs, record_thread, outcome, conflict, accepted) in [
            (
                "valid",
                AdaptiveWorkerRole::Validation,
                vec!["e"],
                0,
                expected,
                false,
                true,
            ),
            (
                "none",
                AdaptiveWorkerRole::Validation,
                vec![],
                0,
                expected,
                false,
                false,
            ),
            (
                "missing",
                AdaptiveWorkerRole::Validation,
                vec!["missing"],
                0,
                expected,
                false,
                false,
            ),
            (
                "cross",
                AdaptiveWorkerRole::Validation,
                vec!["e"],
                1,
                expected,
                false,
                false,
            ),
            (
                "conflict",
                AdaptiveWorkerRole::Validation,
                vec!["e"],
                0,
                expected,
                true,
                false,
            ),
            (
                "success",
                AdaptiveWorkerRole::Validation,
                vec!["e"],
                0,
                AdaptiveEvidenceOutcome::Success,
                false,
                expected == AdaptiveEvidenceOutcome::Success,
            ),
            (
                "failure",
                AdaptiveWorkerRole::Validation,
                vec!["e"],
                0,
                AdaptiveEvidenceOutcome::Failure,
                false,
                expected == AdaptiveEvidenceOutcome::Failure,
            ),
            (
                "cancelled",
                AdaptiveWorkerRole::Validation,
                vec!["e"],
                0,
                AdaptiveEvidenceOutcome::Cancelled,
                false,
                false,
            ),
            (
                "incomplete",
                AdaptiveWorkerRole::Validation,
                vec!["e"],
                0,
                AdaptiveEvidenceOutcome::Incomplete,
                false,
                false,
            ),
            (
                "wrong-role",
                AdaptiveWorkerRole::Implementation,
                vec!["e"],
                0,
                expected,
                false,
                false,
            ),
        ] {
            let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
            let thread_id = ThreadId::new();
            chat.thread_id = Some(thread_id);
            chat.dispatch_command_with_args(
                SlashCommand::Adaptive,
                "terra".to_string(),
                Vec::new(),
            );
            drain_events(&mut rx);
            chat.adaptive_effort.worker_context = AdaptiveWorkerContext {
                role,
                authorized_scope: Some("fixed-scope".to_string()),
            };
            if refs == ["e"] {
                let evidence_thread = if record_thread == 0 {
                    thread_id
                } else {
                    ThreadId::new()
                };
                register_evidence(&mut chat, "e", evidence_thread, outcome);
                if conflict {
                    register_evidence(
                        &mut chat,
                        "e",
                        evidence_thread,
                        AdaptiveEvidenceOutcome::Cancelled,
                    );
                }
            }
            let before = chat.adaptive_effort.clone();
            chat.adaptive_effort.pending_signal = Some(pending_signal(
                "terminal-turn",
                kind,
                refs.into_iter().map(str::to_string).collect(),
            ));
            assert_eq!(
                chat.consume_adaptive_signal_at_terminal("terminal-turn"),
                accepted,
                "{kind:?} case {case}"
            );
            assert_eq!(
                chat.adaptive_effort.workflow_terminal,
                accepted.then_some(terminal)
            );
            assert_eq!(chat.adaptive_effort.worker_context, before.worker_context);
            assert_eq!(chat.adaptive_effort.current_family, before.current_family);
            assert_eq!(chat.adaptive_effort.current_effort, before.current_effort);
            assert_eq!(chat.adaptive_effort.attempt_number, before.attempt_number);
            assert_eq!(chat.adaptive_effort.pending_attempt, None);
            assert!(!chat.consume_adaptive_signal_at_terminal("terminal-turn"));
            assert_no_submit_op(&mut op_rx);
        }
    }
}

#[tokio::test]
async fn producer_self_evidence_and_ordinary_prose_have_no_workflow_authority() {
    for kind in [
        AdaptiveRuntimeSignalKind::RepairRequired,
        AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
    ] {
        let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
        let thread_id = ThreadId::new();
        chat.thread_id = Some(thread_id);
        chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
        drain_events(&mut rx);
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Validation;
        chat.adaptive_effort.pending_signal = Some(pending_signal(
            "producer-turn",
            kind,
            vec!["report-adaptive-signal-result".to_string()],
        ));
        assert!(!chat.consume_adaptive_signal_at_terminal("producer-turn"));
        assert_eq!(chat.adaptive_effort.workflow_terminal, None);
        assert_no_submit_op(&mut op_rx);
    }

    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.handle_thread_session(adaptive_test_session(
        ThreadId::new(),
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    ));
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    drain_events(&mut rx);
    for _text in [
        "ready for QA",
        "tests passed",
        "repair required",
        "I need more reasoning",
        "compiler output: tests passed; ready for QA",
    ] {
        chat.apply_adaptive_terminal_signal("ordinary-turn", AdaptiveOutcomeSignal::Completed);
    }
    assert_eq!(chat.adaptive_effort.workflow_terminal, None);
    assert_eq!(chat.adaptive_effort.attempt_number, 1);
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn typed_failures_outrank_pending_capability_without_extra_escalation() {
    for (failure, expected_attempt, expected_retry) in [
        (AdaptiveFailureKind::TransientInfrastructure, 2, true),
        (AdaptiveFailureKind::Authentication, 1, false),
        (AdaptiveFailureKind::Quota, 1, false),
        (AdaptiveFailureKind::Environment, 1, false),
        (AdaptiveFailureKind::Permission, 1, false),
        (AdaptiveFailureKind::Dependency, 1, false),
        (AdaptiveFailureKind::ModelUnavailable, 1, false),
        (AdaptiveFailureKind::OwnerDecision, 1, false),
        (AdaptiveFailureKind::AuthorizationOrScope, 1, false),
        (AdaptiveFailureKind::Unknown, 1, false),
    ] {
        let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
        chat.handle_thread_session(adaptive_test_session(
            ThreadId::new(),
            /*forked_from_id*/ None,
            "gpt-5.6-terra",
            Some(ReasoningEffort::Low),
        ));
        chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
        drain_events(&mut rx);
        chat.adaptive_effort.pending_signal = Some(pending_signal(
            "failure-turn",
            AdaptiveRuntimeSignalKind::Capability,
            Vec::new(),
        ));
        chat.apply_adaptive_terminal_signal(
            "failure-turn",
            AdaptiveOutcomeSignal::Failure(failure),
        );
        assert_eq!(chat.adaptive_effort.attempt_number, expected_attempt);
        assert_eq!(
            chat.adaptive_effort.transient_retry_consumed,
            expected_retry
        );
        assert_eq!(
            chat.adaptive_effort.workflow_terminal,
            (!expected_retry).then_some(AdaptiveWorkflowTerminal::Blocked)
        );
        assert_eq!(
            chat.adaptive_effort.current_family,
            Some(AdaptiveFamily::Luna)
        );
        assert_eq!(
            chat.adaptive_effort.current_effort,
            Some(AdaptiveEffort::Low)
        );
        assert_no_submit_op(&mut op_rx);
    }
}

#[tokio::test]
async fn every_existing_terminal_suppresses_later_capability() {
    for terminal in [
        AdaptiveWorkflowTerminal::ReadyForValidation,
        AdaptiveWorkflowTerminal::RepairRequired,
        AdaptiveWorkflowTerminal::ReadyForOwnerQa,
        AdaptiveWorkflowTerminal::Blocked,
    ] {
        let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
        chat.handle_thread_session(adaptive_test_session(
            ThreadId::new(),
            /*forked_from_id*/ None,
            "gpt-5.6-terra",
            Some(ReasoningEffort::Low),
        ));
        chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
        drain_events(&mut rx);
        chat.adaptive_effort.workflow_terminal = Some(terminal);
        chat.adaptive_effort.pending_signal = Some(pending_signal(
            "later-turn",
            AdaptiveRuntimeSignalKind::Capability,
            Vec::new(),
        ));
        let before = chat.adaptive_effort.clone();
        assert!(!chat.consume_adaptive_signal_at_terminal("later-turn"));
        assert_eq!(chat.adaptive_effort, before);
        assert_no_submit_op(&mut op_rx);
    }
}

#[tokio::test]
async fn capability_is_consumed_only_at_terminal_and_advances_one_existing_ladder_rung() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.thread_id = Some(thread_id);
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    drain_events(&mut rx);
    chat.adaptive_effort.worker_context = AdaptiveWorkerContext {
        role: AdaptiveWorkerRole::Implementation,
        authorized_scope: Some("slice-7c-c".to_string()),
    };
    chat.adaptive_effort.pending_signal = Some(pending_signal(
        "turn-capability",
        AdaptiveRuntimeSignalKind::Capability,
        Vec::new(),
    ));

    assert!(chat.consume_adaptive_signal_at_terminal("turn-capability"));

    assert_eq!(
        chat.adaptive_effort.current_family,
        Some(AdaptiveFamily::Luna)
    );
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Medium)
    );
    assert_eq!(chat.adaptive_effort.attempt_number, 2);
    assert_eq!(
        chat.adaptive_effort.worker_context,
        AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Implementation,
            authorized_scope: Some("slice-7c-c".to_string()),
        }
    );
    assert_eq!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Consumed {
            source_turn_id: "turn-capability".to_string(),
        })
    );
    assert_matches!(
        &chat.adaptive_effort.pending_attempt,
        Some(crate::chatwidget::adaptive_effort::AdaptivePendingAttempt {
            source_turn_id,
            attempt_number: 2,
            ..
        }) if source_turn_id == "turn-capability"
    );
    assert!(!chat.consume_adaptive_signal_at_terminal("turn-capability"));
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn workflow_signals_enforce_roles_and_native_evidence_outcomes() {
    let thread_id = ThreadId::new();
    for (kind, outcome, terminal) in [
        (
            AdaptiveRuntimeSignalKind::RepairRequired,
            AdaptiveEvidenceOutcome::Failure,
            AdaptiveWorkflowTerminal::RepairRequired,
        ),
        (
            AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
            AdaptiveEvidenceOutcome::Success,
            AdaptiveWorkflowTerminal::ReadyForOwnerQa,
        ),
    ] {
        let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
        chat.thread_id = Some(thread_id);
        chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
        drain_events(&mut rx);
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Validation;
        chat.adaptive_effort
            .evidence_registry
            .register(AdaptiveEvidenceRecord {
                evidence_id: "native-evidence".to_string(),
                thread_id,
                source_turn_id: "prior-validation-turn".to_string(),
                outcome,
                kind: AdaptiveEvidenceKind::CommandExecution,
            });
        let before_route = (
            chat.adaptive_effort.current_family,
            chat.adaptive_effort.current_effort,
            chat.adaptive_effort.attempt_number,
        );
        chat.adaptive_effort.pending_signal = Some(pending_signal(
            "terminal-turn",
            kind,
            vec!["native-evidence".to_string()],
        ));

        assert!(chat.consume_adaptive_signal_at_terminal("terminal-turn"));
        assert_eq!(chat.adaptive_effort.workflow_terminal, Some(terminal));
        assert_eq!(
            (
                chat.adaptive_effort.current_family,
                chat.adaptive_effort.current_effort,
                chat.adaptive_effort.attempt_number,
            ),
            before_route
        );
        assert_eq!(
            chat.adaptive_effort.worker_context.role,
            AdaptiveWorkerRole::Validation
        );
        assert_no_submit_op(&mut op_rx);
    }
}

#[tokio::test]
async fn invalid_workflow_signal_is_tombstoned_and_cannot_gain_authority() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.thread_id = Some(thread_id);
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    drain_events(&mut rx);
    chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Validation;
    chat.adaptive_effort.pending_signal = Some(pending_signal(
        "invalid-turn",
        AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
        vec!["missing".to_string()],
    ));

    assert!(!chat.consume_adaptive_signal_at_terminal("invalid-turn"));
    assert_eq!(chat.adaptive_effort.workflow_terminal, None);
    assert_eq!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Cancelled {
            source_turn_id: "invalid-turn".to_string(),
        })
    );
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn typed_runtime_signal_receipt_is_inert_and_preserves_native_identity() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.handle_thread_session(adaptive_test_session(
        thread_id,
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    ));
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    drain_events(&mut rx);
    handle_turn_started(&mut chat, "turn-native");
    let before = chat.adaptive_effort.clone();

    chat.handle_server_notification(
        adaptive_runtime_signal(
            thread_id,
            "turn-native",
            AdaptiveRuntimeSignalKind::Capability,
        ),
        None,
    );

    assert_eq!(
        chat.adaptive_effort,
        AdaptiveEffortState {
            pending_signal: Some(AdaptivePendingSignal::Pending(
                AdaptiveRuntimeSignalEnvelope {
                    source_turn_id: "turn-native".to_string(),
                    signal_kind: AdaptiveRuntimeSignalKind::Capability,
                    evidence_refs: Vec::new(),
                    diagnostic_note: None,
                },
            )),
            ..before
        }
    );
    assert!(drain_events(&mut rx).iter().all(|event| !matches!(
        event,
        AppEvent::UpdateModel(_) | AppEvent::UpdateReasoningEffort(_)
    )));
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn duplicate_is_idempotent_and_conflict_cannot_be_repaired() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.handle_thread_session(adaptive_test_session(
        thread_id,
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    ));
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    drain_events(&mut rx);
    handle_turn_started(&mut chat, "turn-1");
    let capability =
        adaptive_runtime_signal(thread_id, "turn-1", AdaptiveRuntimeSignalKind::Capability);
    chat.handle_server_notification(capability.clone(), None);
    drain_events(&mut rx);
    let pending = chat.adaptive_effort.clone();
    chat.handle_server_notification(capability.clone(), None);
    assert_eq!(chat.adaptive_effort, pending);
    assert!(drain_events(&mut rx).is_empty());

    chat.handle_server_notification(
        adaptive_runtime_signal(
            thread_id,
            "turn-1",
            AdaptiveRuntimeSignalKind::ReadyForValidation,
        ),
        None,
    );
    assert_eq!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Conflicted {
            source_turn_id: "turn-1".to_string(),
        })
    );
    chat.handle_server_notification(capability, None);
    assert_eq!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Conflicted {
            source_turn_id: "turn-1".to_string(),
        })
    );
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn signal_identity_canonicalizes_evidence_and_ignores_diagnostic_note() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.handle_thread_session(adaptive_test_session(
        thread_id,
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    ));
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    drain_events(&mut rx);
    handle_turn_started(&mut chat, "canonical-turn");
    let notification = |evidence_refs: Vec<&str>, diagnostic_note: Option<&str>| {
        ServerNotification::AdaptiveRuntimeSignal(AdaptiveRuntimeSignalNotification {
            thread_id: thread_id.to_string(),
            signal: AdaptiveRuntimeSignalEnvelope {
                source_turn_id: "canonical-turn".to_string(),
                signal_kind: AdaptiveRuntimeSignalKind::RepairRequired,
                evidence_refs: evidence_refs.into_iter().map(str::to_string).collect(),
                diagnostic_note: diagnostic_note.map(str::to_string),
            },
        })
    };
    chat.handle_server_notification(notification(vec!["b", "a", "a"], Some("first note")), None);
    assert_matches!(
        &chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Pending(signal))
            if signal.source_turn_id == "canonical-turn"
                && signal.signal_kind == AdaptiveRuntimeSignalKind::RepairRequired
                && signal.evidence_refs == ["a", "b"]
                && signal.diagnostic_note.as_deref() == Some("first note")
    );
    let canonical = chat.adaptive_effort.clone();
    chat.handle_server_notification(notification(vec!["a", "b"], Some("different note")), None);
    assert_eq!(chat.adaptive_effort, canonical);

    chat.handle_server_notification(notification(vec!["a", "c"], None), None);
    assert_eq!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Conflicted {
            source_turn_id: "canonical-turn".to_string(),
        })
    );
    chat.handle_server_notification(notification(vec!["a", "b"], None), None);
    assert_matches!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Conflicted { .. })
    );
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn runtime_signal_rejects_cross_thread_stale_and_inactive_control_states() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.handle_thread_session(adaptive_test_session(
        thread_id,
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    ));
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    drain_events(&mut rx);
    handle_turn_started(&mut chat, "active");

    for notification in [
        adaptive_runtime_signal(
            ThreadId::new(),
            "active",
            AdaptiveRuntimeSignalKind::Capability,
        ),
        adaptive_runtime_signal(thread_id, "stale", AdaptiveRuntimeSignalKind::Capability),
    ] {
        chat.handle_server_notification(notification, None);
    }
    assert_eq!(chat.adaptive_effort.pending_signal, None);

    chat.adaptive_effort.paused_by_user = true;
    chat.handle_server_notification(
        adaptive_runtime_signal(thread_id, "active", AdaptiveRuntimeSignalKind::Capability),
        None,
    );
    assert_eq!(chat.adaptive_effort.pending_signal, None);
    chat.adaptive_effort.paused_by_user = false;
    chat.adaptive_effort.workflow_terminal = Some(AdaptiveWorkflowTerminal::Blocked);
    chat.handle_server_notification(
        adaptive_runtime_signal(thread_id, "active", AdaptiveRuntimeSignalKind::Capability),
        None,
    );
    assert_eq!(chat.adaptive_effort.pending_signal, None);
    assert_no_submit_op(&mut op_rx);
}

#[test]
fn forks_inherit_stable_adaptive_state_but_never_pending_signal_authority() {
    let parent_id = ThreadId::new();
    let mut parent = adaptive_test_session(
        parent_id,
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    );
    parent.adaptive_effort.enabled = true;
    parent.adaptive_effort.attempt_number = 3;
    parent.adaptive_effort.pending_signal = Some(AdaptivePendingSignal::Pending(
        AdaptiveRuntimeSignalEnvelope {
            source_turn_id: "parent-turn".to_string(),
            signal_kind: AdaptiveRuntimeSignalKind::Capability,
            evidence_refs: Vec::new(),
            diagnostic_note: None,
        },
    ));
    let mut child = adaptive_test_session(
        ThreadId::new(),
        Some(parent_id),
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    );
    child.inherit_adaptive_effort_from(&parent);
    assert!(child.adaptive_effort.enabled);
    assert_eq!(child.adaptive_effort.attempt_number, 3);
    assert_eq!(child.adaptive_effort.pending_signal, None);

    let mut grandchild = adaptive_test_session(
        ThreadId::new(),
        Some(child.thread_id),
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    );
    grandchild.inherit_adaptive_effort_from(&child);
    assert_eq!(grandchild.adaptive_effort.pending_signal, None);
}

#[tokio::test]
async fn manual_esc_cancels_signal_and_resume_cannot_reactivate_it() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.handle_thread_session(adaptive_test_session(
        thread_id,
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    ));
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    drain_events(&mut rx);
    chat.submit_user_message(UserMessage::from("active task"));
    assert_matches!(next_submit_op(&mut op_rx), Op::UserTurn { .. });
    handle_turn_started(&mut chat, "turn-esc");
    chat.handle_server_notification(
        adaptive_runtime_signal(thread_id, "turn-esc", AdaptiveRuntimeSignalKind::Capability),
        None,
    );
    chat.bottom_pane.ensure_status_indicator();
    let route = (
        chat.adaptive_effort.current_family,
        chat.adaptive_effort.current_effort,
        chat.adaptive_effort.attempt_number,
    );

    chat.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(chat.adaptive_effort.paused_by_user);
    assert_eq!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Cancelled {
            source_turn_id: "turn-esc".to_string(),
        })
    );
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "resume".to_string(), Vec::new());
    chat.handle_server_notification(
        adaptive_runtime_signal(thread_id, "turn-esc", AdaptiveRuntimeSignalKind::Capability),
        None,
    );
    assert_eq!(
        chat.adaptive_effort.pending_signal,
        Some(AdaptivePendingSignal::Cancelled {
            source_turn_id: "turn-esc".to_string(),
        })
    );
    assert_eq!(
        (
            chat.adaptive_effort.current_family,
            chat.adaptive_effort.current_effort,
            chat.adaptive_effort.attempt_number,
        ),
        route
    );
}

#[tokio::test]
async fn live_bridge_is_idempotent_and_prepares_same_route_retry_without_churn() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.handle_thread_session(adaptive_test_session(
        ThreadId::new(),
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    ));
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    drain_events(&mut rx);

    let signal = AdaptiveOutcomeSignal::Failure(AdaptiveFailureKind::TransientInfrastructure);
    chat.apply_adaptive_terminal_signal("turn-1", signal);
    assert_eq!(chat.adaptive_effort.attempt_number, 2);
    assert!(chat.adaptive_effort.transient_retry_consumed);
    assert_eq!(
        chat.adaptive_effort.current_family,
        Some(AdaptiveFamily::Luna)
    );
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Low)
    );
    assert!(chat.adaptive_effort.pending_attempt.is_some());
    assert!(drain_events(&mut rx).iter().all(|event| !matches!(
        event,
        AppEvent::UpdateModel(_) | AppEvent::UpdateReasoningEffort(_)
    )));
    chat.apply_adaptive_terminal_signal("turn-1", signal);
    assert_eq!(chat.adaptive_effort.attempt_number, 2);
    assert!(drain_events(&mut rx).is_empty());
    assert_no_submit_op(&mut op_rx);

    chat.apply_adaptive_terminal_signal("turn-2", signal);
    assert_eq!(chat.adaptive_effort.attempt_number, 2);
    assert_eq!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::Blocked)
    );
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn injected_capability_applies_native_effort_then_model_routes_once() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.handle_thread_session(adaptive_test_session(
        ThreadId::new(),
        /*forked_from_id*/ None,
        "gpt-5.6-luna",
        Some(ReasoningEffort::Low),
    ));
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    drain_events(&mut rx);
    let worker = AdaptiveWorkerContext {
        role: AdaptiveWorkerRole::Repair,
        authorized_scope: Some("slice-7b".to_string()),
    };
    chat.adaptive_effort.worker_context = worker.clone();
    let signal = AdaptiveOutcomeSignal::Failure(AdaptiveFailureKind::Capability);

    chat.apply_adaptive_terminal_classification(
        "turn-effort",
        signal,
        AdaptiveClassification::EscalationEligible,
    );
    assert_eq!(chat.adaptive_effort.attempt_number, 2);
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Medium)
    );
    assert_eq!(chat.adaptive_effort.worker_context, worker);
    let events = drain_events(&mut rx);
    assert!(events.iter().any(|event| matches!(
        event,
        AppEvent::UpdateReasoningEffort(Some(ReasoningEffort::Medium))
    )));
    assert!(
        !events
            .iter()
            .any(|event| matches!(event, AppEvent::UpdateModel(_)))
    );

    chat.adaptive_effort.current_effort = Some(AdaptiveEffort::High);
    chat.set_reasoning_effort(Some(ReasoningEffort::High));
    chat.apply_adaptive_terminal_classification(
        "turn-model",
        signal,
        AdaptiveClassification::EscalationEligible,
    );
    assert_eq!(chat.adaptive_effort.attempt_number, 3);
    assert_eq!(
        chat.adaptive_effort.current_family,
        Some(AdaptiveFamily::Terra)
    );
    assert_eq!(
        chat.adaptive_effort.current_effort,
        Some(AdaptiveEffort::Low)
    );
    let events = drain_events(&mut rx);
    assert!(
        events
            .iter()
            .any(|event| matches!(event, AppEvent::UpdateModel(model) if model == "gpt-5.6-terra"))
    );
    assert!(events.iter().any(|event| matches!(
        event,
        AppEvent::UpdateReasoningEffort(Some(ReasoningEffort::Low))
    )));
    chat.apply_adaptive_terminal_classification(
        "turn-model",
        signal,
        AdaptiveClassification::EscalationEligible,
    );
    assert_eq!(chat.adaptive_effort.attempt_number, 3);
    assert!(drain_events(&mut rx).is_empty());
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn live_bridge_suppresses_off_paused_and_terminal_states() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let signal = AdaptiveOutcomeSignal::Failed;
    chat.apply_adaptive_terminal_signal("off", signal);
    assert_eq!(chat.adaptive_effort, AdaptiveEffortState::default());

    chat.dispatch_command_with_args(SlashCommand::Adaptive, "sol".to_string(), Vec::new());
    drain_events(&mut rx);
    chat.adaptive_effort.paused_by_user = true;
    let paused = chat.adaptive_effort.clone();
    chat.apply_adaptive_terminal_signal("paused", signal);
    assert_eq!(chat.adaptive_effort, paused);
    chat.adaptive_effort.paused_by_user = false;
    chat.adaptive_effort.workflow_terminal = Some(AdaptiveWorkflowTerminal::RepairRequired);
    let terminal = chat.adaptive_effort.clone();
    chat.apply_adaptive_terminal_classification(
        "terminal",
        AdaptiveOutcomeSignal::Failure(AdaptiveFailureKind::Capability),
        AdaptiveClassification::EscalationEligible,
    );
    assert_eq!(chat.adaptive_effort, terminal);
    assert!(drain_events(&mut rx).is_empty());
    assert_no_submit_op(&mut op_rx);
}

fn adaptive_test_session(
    thread_id: ThreadId,
    forked_from_id: Option<ThreadId>,
    model: &str,
    reasoning_effort: Option<ReasoningEffort>,
) -> crate::session_state::ThreadSessionState {
    crate::session_state::ThreadSessionState {
        thread_id,
        forked_from_id,
        fork_parent_title: None,
        thread_name: None,
        model: model.to_string(),
        model_provider_id: "openai".to_string(),
        service_tier: None,
        approval_policy: AskForApproval::Never,
        approvals_reviewer: ApprovalsReviewer::User,
        permission_profile: PermissionProfile::read_only(),
        active_permission_profile: None,
        cwd: test_path_buf("/tmp/adaptive-effort").abs(),
        runtime_workspace_roots: Vec::new(),
        instruction_source_paths: Vec::new(),
        reasoning_effort,
        collaboration_mode: None,
        personality: None,
        message_history: None,
        network_proxy: None,
        rollout_path: None,
        adaptive_effort: AdaptiveEffortState::default(),
    }
}

async fn enable_family(
    family: &str,
) -> (ChatWidget, tokio::sync::mpsc::UnboundedReceiver<AppEvent>) {
    let (mut chat, rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.dispatch_command_with_args(
        SlashCommand::from_str("adaptive").expect("adaptive command"),
        family.to_string(),
        Vec::new(),
    );
    (chat, rx)
}

#[tokio::test]
async fn adaptive_families_initialize_state_and_sync_model_and_effort() {
    for (family, preference) in [
        ("luna", AdaptiveFamily::Luna),
        ("terra", AdaptiveFamily::Terra),
        ("sol", AdaptiveFamily::Sol),
        ("astra", AdaptiveFamily::Astra),
    ] {
        let (mut chat, mut rx) = enable_family(family).await;
        assert!(chat.adaptive_effort.enabled);
        assert_eq!(chat.adaptive_effort.attempt_number, 1);
        assert_eq!(chat.adaptive_effort.starting_family, Some(preference));
        assert_eq!(
            chat.adaptive_effort.current_family,
            Some(AdaptiveFamily::Luna)
        );
        assert_eq!(
            chat.adaptive_effort.current_effort,
            Some(crate::adaptive_policy::AdaptiveEffort::Low)
        );

        let model_event = next_non_adaptive_state_event(&mut rx);
        match model_event {
            Ok(AppEvent::UpdateModel(value)) => {
                assert_eq!(value, "gpt-5.6-luna");
                chat.set_model(&value);
            }
            other => panic!("expected model update, got {other:?}"),
        }

        match rx.try_recv() {
            Ok(AppEvent::UpdateReasoningEffort(Some(ReasoningEffort::Low))) => {
                chat.set_reasoning_effort(Some(ReasoningEffort::Low));
            }
            other => panic!("expected low reasoning update, got {other:?}"),
        }

        assert_eq!(chat.current_model(), "gpt-5.6-luna");
        assert_eq!(chat.current_reasoning_effort(), Some(ReasoningEffort::Low));
    }
}

#[tokio::test]
async fn manual_adaptive_initialization_preserves_thread_worker_context_and_terminal() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.adaptive_effort.worker_context = AdaptiveWorkerContext {
        role: AdaptiveWorkerRole::Repair,
        authorized_scope: Some("gate/42c".to_string()),
    };
    chat.adaptive_effort.workflow_terminal = Some(AdaptiveWorkflowTerminal::RepairRequired);

    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());

    assert_eq!(
        chat.adaptive_effort.worker_context,
        AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Repair,
            authorized_scope: Some("gate/42c".to_string()),
        }
    );
    assert_eq!(
        chat.adaptive_effort.workflow_terminal,
        Some(AdaptiveWorkflowTerminal::RepairRequired)
    );
}

#[tokio::test]
async fn adaptive_controls_render_without_submitting_a_turn() {
    let (mut chat, mut rx) = enable_family("terra").await;
    let thread_id = ThreadId::new();
    chat.thread_id = Some(thread_id);
    while rx.try_recv().is_ok() {}

    chat.dispatch_command_with_args(SlashCommand::Adaptive, "status".to_string(), Vec::new());
    let status = match rx.try_recv() {
        Ok(AppEvent::InsertHistoryCell(cell)) => lines_to_single_string(&cell.display_lines(100)),
        other => panic!("expected adaptive status output, got {other:?}"),
    };
    assert!(status.contains("Preference: Terra"));
    assert!(status.contains("Current: Luna Low"));
    assert_eq!(chat.thread_id, Some(thread_id));

    chat.dispatch_command_with_args(SlashCommand::Adaptive, "pause".to_string(), Vec::new());
    assert!(chat.adaptive_effort.paused_by_user);
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "resume".to_string(), Vec::new());
    assert!(!chat.adaptive_effort.paused_by_user);
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "off".to_string(), Vec::new());
    assert!(!chat.adaptive_effort.enabled);
    assert!(rx.try_recv().is_ok());
}

#[tokio::test]
async fn adaptive_invalid_family_does_not_mutate_state_and_status_is_coherent() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;
    let before = chat.adaptive_effort.clone();
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "potato".to_string(), Vec::new());
    assert_eq!(chat.adaptive_effort, before);
    while rx.try_recv().is_ok() {}

    chat.dispatch_command(SlashCommand::Status);
    let rendered = match rx.try_recv() {
        Ok(AppEvent::InsertHistoryCell(cell)) => lines_to_single_string(&cell.display_lines(100)),
        other => panic!("expected status output, got {other:?}"),
    };
    assert!(rendered.contains("OpenAI Codex"));
    assert!(rendered.contains("Adaptive Effort"));
    assert!(rendered.contains("Enabled: no"));
}

#[tokio::test]
async fn manual_esc_pauses_adaptive_effort_without_changing_or_retrying_the_turn() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    while rx.try_recv().is_ok() {}
    chat.thread_id = Some(ThreadId::new());
    chat.submit_user_message(UserMessage::from("active task"));
    assert_matches!(next_submit_op(&mut op_rx), Op::UserTurn { .. });
    handle_turn_started(&mut chat, "turn-1");
    chat.bottom_pane.ensure_status_indicator();
    let before = chat.adaptive_effort.clone();

    let esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    assert!(chat.bottom_pane.should_interrupt_running_task(esc));
    chat.handle_key_event(esc);

    loop {
        match rx.try_recv() {
            Ok(AppEvent::CodexOp(Op::Interrupt)) => break,
            Ok(_) => {}
            Err(error) => panic!("expected Esc interrupt command, got {error:?}"),
        }
    }
    handle_turn_interrupted(&mut chat, "turn-1");
    assert_eq!(
        chat.adaptive_effort,
        AdaptiveEffortState {
            paused_by_user: true,
            last_outcome: Some(AdaptiveOutcome::UserInterrupted),
            last_failure_kind: None,
            ..before.clone()
        }
    );
    assert_no_submit_op(&mut op_rx);
    while rx.try_recv().is_ok() {}

    chat.dispatch_command(SlashCommand::Status);
    let status = match rx.try_recv() {
        Ok(AppEvent::InsertHistoryCell(cell)) => lines_to_single_string(&cell.display_lines(100)),
        other => panic!("expected status output, got {other:?}"),
    };
    assert!(status.contains("Paused: yes"));
    assert!(status.contains("Last outcome: USER_INTERRUPTED"));
    assert!(status.contains("Last failure: None"));

    chat.dispatch_command_with_args(SlashCommand::Adaptive, "status".to_string(), Vec::new());
    let adaptive_status = match rx.try_recv() {
        Ok(AppEvent::InsertHistoryCell(cell)) => lines_to_single_string(&cell.display_lines(100)),
        other => panic!("expected adaptive status output, got {other:?}"),
    };
    assert!(adaptive_status.contains("Paused: yes"));
    assert!(adaptive_status.contains("Last outcome: USER_INTERRUPTED"));

    chat.dispatch_command_with_args(SlashCommand::Adaptive, "resume".to_string(), Vec::new());
    assert_eq!(
        chat.adaptive_effort,
        AdaptiveEffortState {
            paused_by_user: false,
            last_outcome: Some(AdaptiveOutcome::UserInterrupted),
            ..before
        }
    );
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn descended_thread_inherits_paused_adaptive_effort_and_restores_settings() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let parent_id = ThreadId::new();
    chat.handle_thread_session(adaptive_test_session(
        parent_id,
        /*forked_from_id*/ None,
        "gpt-5.6-luna",
        Some(ReasoningEffort::Low),
    ));
    while rx.try_recv().is_ok() {}
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    while rx.try_recv().is_ok() {}
    chat.on_adaptive_user_interrupt();

    let expected = AdaptiveEffortState {
        enabled: true,
        starting_family: Some(AdaptiveFamily::Terra),
        current_family: Some(AdaptiveFamily::Terra),
        current_effort: Some(AdaptiveEffort::Low),
        attempt_number: 1,
        paused_by_user: true,
        last_outcome: Some(AdaptiveOutcome::UserInterrupted),
        last_failure_kind: None,
        worker_context: AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: Some("gate/42c".to_string()),
        },
        workflow_terminal: Some(AdaptiveWorkflowTerminal::RepairRequired),
        ..AdaptiveEffortState::default()
    };
    let child_id = ThreadId::new();
    let mut child = adaptive_test_session(
        child_id,
        Some(parent_id),
        "gpt-5.6-luna",
        Some(ReasoningEffort::Medium),
    );
    child.adaptive_effort = expected.clone();
    chat.handle_thread_session(child);
    assert_eq!(chat.adaptive_effort, expected);
    assert_matches!(next_non_adaptive_state_event(&mut rx), Ok(AppEvent::UpdateModel(model)) if model == "gpt-5.6-terra");
    assert_matches!(
        rx.try_recv(),
        Ok(AppEvent::UpdateReasoningEffort(Some(ReasoningEffort::Low)))
    );
    while rx.try_recv().is_ok() {}
    chat.dispatch_command(SlashCommand::Status);
    let status = match rx.try_recv() {
        Ok(AppEvent::InsertHistoryCell(cell)) => lines_to_single_string(&cell.display_lines(100)),
        other => panic!("expected inherited status output, got {other:?}"),
    };
    assert!(status.contains("Preference: Terra"));
    assert!(status.contains("Current: Terra Low"));
    assert!(status.contains("Attempt: 1"));
    assert!(status.contains("Paused: yes"));
    assert!(status.contains("Last outcome: USER_INTERRUPTED"));
    assert!(status.contains("Worker role: Validation"));
    assert!(status.contains("Workflow terminal: REPAIR_REQUIRED"));

    chat.dispatch_command_with_args(SlashCommand::Adaptive, "resume".to_string(), Vec::new());
    assert_eq!(
        chat.adaptive_effort,
        AdaptiveEffortState {
            paused_by_user: false,
            ..expected
        }
    );
    assert_no_submit_op(&mut op_rx);
}

#[test]
fn typed_forks_snapshot_immediate_parent_state_and_remain_independent() {
    let parent_id = ThreadId::new();
    let mut parent = adaptive_test_session(
        parent_id,
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    );
    parent.adaptive_effort = AdaptiveEffortState {
        enabled: true,
        starting_family: Some(AdaptiveFamily::Terra),
        current_family: Some(AdaptiveFamily::Terra),
        current_effort: Some(AdaptiveEffort::Low),
        attempt_number: 1,
        paused_by_user: true,
        last_outcome: Some(AdaptiveOutcome::UserInterrupted),
        last_failure_kind: None,
        worker_context: AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Implementation,
            authorized_scope: Some("milestone/42c".to_string()),
        },
        workflow_terminal: Some(AdaptiveWorkflowTerminal::ReadyForValidation),
        ..AdaptiveEffortState::default()
    };
    parent
        .adaptive_effort
        .evidence_registry
        .register(AdaptiveEvidenceRecord {
            evidence_id: "parent-native-call".to_string(),
            thread_id: parent_id,
            source_turn_id: "parent-turn".to_string(),
            outcome: AdaptiveEvidenceOutcome::Success,
            kind: AdaptiveEvidenceKind::CommandExecution,
        });
    let mut child = adaptive_test_session(
        ThreadId::new(),
        Some(parent_id),
        "gpt-5.6-luna",
        Some(ReasoningEffort::Low),
    );
    child.inherit_adaptive_effort_from(&parent);
    assert_eq!(child.adaptive_effort.evidence_registry.len(), 0);
    assert_eq!(
        child.adaptive_effort,
        AdaptiveEffortState {
            evidence_registry: Default::default(),
            ..parent.adaptive_effort.clone()
        }
    );
    assert_eq!(child.adaptive_effort.attempt_number, 1);

    parent.adaptive_effort.paused_by_user = false;
    parent.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Repair;
    assert!(child.adaptive_effort.paused_by_user);
    assert_eq!(
        child.adaptive_effort.worker_context.role,
        AdaptiveWorkerRole::Implementation
    );
    child.adaptive_effort.enabled = false;
    assert!(parent.adaptive_effort.enabled);

    child.inherit_adaptive_effort_from(&parent);
    assert_eq!(
        child.adaptive_effort.worker_context.role,
        AdaptiveWorkerRole::Implementation
    );

    let mut grandchild = adaptive_test_session(
        ThreadId::new(),
        Some(child.thread_id),
        "gpt-5.6-luna",
        Some(ReasoningEffort::Low),
    );
    grandchild.inherit_adaptive_effort_from(&child);
    assert_eq!(grandchild.adaptive_effort, child.adaptive_effort);
    assert_eq!(grandchild.adaptive_effort.evidence_registry.len(), 0);
}

#[test]
fn fresh_adaptive_state_has_no_worker_binding_or_workflow_terminal() {
    assert_eq!(
        AdaptiveEffortState::default(),
        AdaptiveEffortState {
            worker_context: AdaptiveWorkerContext {
                role: AdaptiveWorkerRole::Unspecified,
                authorized_scope: None,
            },
            workflow_terminal: None,
            ..AdaptiveEffortState::default()
        }
    );
}

#[tokio::test]
async fn evidence_receipt_is_inert_and_cannot_mutate_worker_authority() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    let thread_id = ThreadId::new();
    chat.handle_thread_session(adaptive_test_session(
        thread_id,
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    ));
    chat.adaptive_effort.worker_context = AdaptiveWorkerContext {
        role: AdaptiveWorkerRole::Validation,
        authorized_scope: Some("opaque/scope".to_string()),
    };
    let before = chat.adaptive_effort.clone();
    while rx.try_recv().is_ok() {}

    chat.register_adaptive_evidence(&ItemCompletedNotification {
        item: ThreadItem::CommandExecution {
            id: "native-call-1".to_string(),
            plugin_id: None,
            script_path: None,
            command: "printf success".to_string(),
            cwd: chat.config.cwd.clone().into(),
            process_id: None,
            source: CommandExecutionSource::Agent,
            status: CommandExecutionStatus::Completed,
            command_actions: Vec::new(),
            aggregated_output: Some("untrusted prose".to_string()),
            exit_code: Some(0),
            duration_ms: Some(1),
        },
        thread_id: thread_id.to_string(),
        turn_id: "turn-1".to_string(),
        completed_at_ms: 1,
    });

    let mut expected = before;
    expected.evidence_registry.register(AdaptiveEvidenceRecord {
        evidence_id: "native-call-1".to_string(),
        thread_id,
        source_turn_id: "turn-1".to_string(),
        outcome: AdaptiveEvidenceOutcome::Success,
        kind: AdaptiveEvidenceKind::CommandExecution,
    });
    assert_eq!(chat.adaptive_effort, expected);
    assert_matches!(
        rx.try_recv(),
        Ok(AppEvent::UpdateAdaptiveEffortState(state)) if state == expected
    );
    assert!(rx.try_recv().is_err());
    assert_no_submit_op(&mut op_rx);

    chat.handle_server_notification(
        ServerNotification::ItemCompleted(ItemCompletedNotification {
            item: ThreadItem::CommandExecution {
                id: "replayed-call".to_string(),
                plugin_id: None,
                script_path: None,
                command: "printf replay".to_string(),
                cwd: chat.config.cwd.clone().into(),
                process_id: None,
                source: CommandExecutionSource::Agent,
                status: CommandExecutionStatus::Completed,
                command_actions: Vec::new(),
                aggregated_output: Some("replayed transcript".to_string()),
                exit_code: Some(0),
                duration_ms: Some(1),
            },
            thread_id: thread_id.to_string(),
            turn_id: "old-turn".to_string(),
            completed_at_ms: 1,
        }),
        Some(ReplayKind::ResumeInitialMessages),
    );
    assert_eq!(chat.adaptive_effort.evidence_registry.len(), 1);
}

#[tokio::test]
async fn status_renders_each_worker_role_and_workflow_terminal() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;
    for role in [
        AdaptiveWorkerRole::Unspecified,
        AdaptiveWorkerRole::Implementation,
        AdaptiveWorkerRole::Validation,
        AdaptiveWorkerRole::Repair,
    ] {
        chat.adaptive_effort.worker_context.role = role;
        chat.adaptive_effort.workflow_terminal = None;
        chat.dispatch_command(SlashCommand::Status);
        let rendered = match rx.try_recv() {
            Ok(AppEvent::InsertHistoryCell(cell)) => {
                lines_to_single_string(&cell.display_lines(/*width*/ 100))
            }
            other => panic!("expected status output, got {other:?}"),
        };
        assert!(rendered.contains(match role {
            AdaptiveWorkerRole::Unspecified => "Worker role: Unspecified",
            AdaptiveWorkerRole::Implementation => "Worker role: Implementation",
            AdaptiveWorkerRole::Validation => "Worker role: Validation",
            AdaptiveWorkerRole::Repair => "Worker role: Repair",
        }));
        assert!(rendered.contains("Workflow terminal: None"));
    }

    for (terminal, expected) in [
        (
            AdaptiveWorkflowTerminal::ReadyForValidation,
            "READY_FOR_VALIDATION",
        ),
        (AdaptiveWorkflowTerminal::RepairRequired, "REPAIR_REQUIRED"),
        (
            AdaptiveWorkflowTerminal::ReadyForOwnerQa,
            "READY_FOR_OWNER_QA",
        ),
        (AdaptiveWorkflowTerminal::Blocked, "BLOCKED"),
    ] {
        chat.adaptive_effort.workflow_terminal = Some(terminal);
        chat.dispatch_command(SlashCommand::Status);
        let rendered = match rx.try_recv() {
            Ok(AppEvent::InsertHistoryCell(cell)) => {
                lines_to_single_string(&cell.display_lines(/*width*/ 100))
            }
            other => panic!("expected status output, got {other:?}"),
        };
        assert!(rendered.contains(&format!("Workflow terminal: {expected}")));
    }
}

#[tokio::test]
async fn unrelated_and_non_adaptive_threads_do_not_inherit_adaptive_effort() {
    let (mut chat, mut rx, _op_rx) = make_chatwidget_manual(None).await;
    let parent_id = ThreadId::new();
    chat.handle_thread_session(adaptive_test_session(
        parent_id,
        /*forked_from_id*/ None,
        "gpt-5.6-luna",
        Some(ReasoningEffort::Low),
    ));
    while rx.try_recv().is_ok() {}
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    while rx.try_recv().is_ok() {}
    let parent_adaptive_effort = chat.adaptive_effort.clone();

    chat.handle_thread_session(adaptive_test_session(
        ThreadId::new(),
        /*forked_from_id*/ None,
        "gpt-5.6-luna",
        Some(ReasoningEffort::Low),
    ));
    assert_eq!(chat.adaptive_effort, AdaptiveEffortState::default());

    let mut resumed_parent = adaptive_test_session(
        parent_id,
        /*forked_from_id*/ None,
        "gpt-5.6-terra",
        Some(ReasoningEffort::Low),
    );
    resumed_parent.adaptive_effort = parent_adaptive_effort.clone();
    chat.handle_thread_session(resumed_parent);
    assert_eq!(chat.adaptive_effort, parent_adaptive_effort);

    let non_adaptive_parent_id = ThreadId::new();
    chat.handle_thread_session(adaptive_test_session(
        non_adaptive_parent_id,
        /*forked_from_id*/ None,
        "gpt-5.6-luna",
        Some(ReasoningEffort::Low),
    ));
    chat.handle_thread_session(adaptive_test_session(
        ThreadId::new(),
        Some(non_adaptive_parent_id),
        "gpt-5.6-luna",
        Some(ReasoningEffort::Low),
    ));
    assert_eq!(chat.adaptive_effort, AdaptiveEffortState::default());
}

#[tokio::test]
async fn esc_does_not_mutate_adaptive_state_when_adaptive_effort_is_off() {
    let (mut chat, mut rx, mut op_rx) = make_chatwidget_manual(None).await;
    chat.thread_id = Some(ThreadId::new());
    chat.submit_user_message(UserMessage::from("active task"));
    assert_matches!(next_submit_op(&mut op_rx), Op::UserTurn { .. });
    handle_turn_started(&mut chat, "turn-1");
    chat.bottom_pane.ensure_status_indicator();
    let before = chat.adaptive_effort.clone();

    chat.handle_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    loop {
        match rx.try_recv() {
            Ok(AppEvent::CodexOp(Op::Interrupt)) => break,
            Ok(_) => {}
            Err(error) => panic!("expected Esc interrupt command, got {error:?}"),
        }
    }
    handle_turn_interrupted(&mut chat, "turn-1");
    assert_eq!(chat.adaptive_effort, before);
    assert_no_submit_op(&mut op_rx);
}

#[tokio::test]
async fn non_manual_turn_endings_do_not_record_user_interruption() {
    let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
    chat.dispatch_command_with_args(SlashCommand::Adaptive, "terra".to_string(), Vec::new());
    chat.thread_id = Some(ThreadId::new());
    handle_turn_started(&mut chat, "turn-1");

    chat.on_task_complete(None, None, /*from_replay*/ false);
    assert_eq!(chat.adaptive_effort.last_outcome, None);
    assert!(!chat.adaptive_effort.paused_by_user);

    handle_turn_started(&mut chat, "turn-2");
    chat.handle_non_retry_error("request failed".to_string(), None);
    assert_eq!(chat.adaptive_effort.last_outcome, None);
    assert!(!chat.adaptive_effort.paused_by_user);
}

#[tokio::test]
async fn adaptive_model_command_clears_composer_after_submit() {
    let (mut chat, _rx, mut op_rx) = make_chatwidget_manual(None).await;

    chat.bottom_pane
        .set_composer_text("/adaptive model".to_string(), Vec::new(), Vec::new());

    chat.handle_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(chat.bottom_pane.composer_text(), "");
    assert_no_submit_op(&mut op_rx);
}

#[test]
fn adaptive_is_discoverable_and_accepts_inline_args() {
    assert!(
        built_in_slash_commands()
            .into_iter()
            .any(|(name, command)| name == "adaptive" && command == SlashCommand::Adaptive)
    );
    assert!(SlashCommand::Adaptive.supports_inline_args());
}
