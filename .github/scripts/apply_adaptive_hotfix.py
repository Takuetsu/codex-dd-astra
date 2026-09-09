from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    p = Path(path)
    text = p.read_text()
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected exactly one match, found {count}: {old[:100]!r}")
    p.write_text(text.replace(old, new, 1))


def replace_all(path: str, old: str, new: str, minimum: int = 1) -> None:
    p = Path(path)
    text = p.read_text()
    count = text.count(old)
    if count < minimum:
        raise SystemExit(f"{path}: expected at least {minimum} matches, found {count}: {old[:100]!r}")
    p.write_text(text.replace(old, new))


controller = "codex-rs/tui/src/adaptive_controller.rs"
replace_once(
    controller,
    "use crate::adaptive_worker::AdaptiveWorkflowTerminal;\n",
    "use crate::adaptive_worker::AdaptiveWorkflowTerminal;\n\npub(crate) const ADAPTIVE_UNFINISHED_TURN_THRESHOLD: u8 = 2;\n",
)
replace_once(
    controller,
    "    RetrySameLevel {\n        route: AdaptiveRoute,\n        next_attempt: u32,\n    },\n    EscalateEffort {",
    "    RetrySameLevel {\n        route: AdaptiveRoute,\n        next_attempt: u32,\n    },\n    ContinueSameRoute {\n        route: AdaptiveRoute,\n        next_attempt: u32,\n    },\n    EscalateEffort {",
)
marker = "/// Deterministically reduces one normalized classification. It has no side effects.\n"
unfinished = '''/// Reduces an ordinary completed turn from an externally bound Worker that did not reach a
/// trusted workflow terminal. The first unfinished return authorizes one same-route continuation;
/// the next consecutive unfinished return converts that bounded pressure into one normal ladder
/// escalation.
pub(crate) fn reduce_unfinished_authorized_turn(
    state: AdaptiveControllerState,
    unfinished_turn_pressure: u8,
) -> (AdaptiveControllerReduction, u8) {
    if state.terminal.is_some() || state.paused_by_user {
        return (no_action(state), 0);
    }

    let pressure = unfinished_turn_pressure
        .saturating_add(1)
        .min(ADAPTIVE_UNFINISHED_TURN_THRESHOLD);
    if pressure < ADAPTIVE_UNFINISHED_TURN_THRESHOLD {
        let next_attempt = state.attempt_number + 1;
        return (
            AdaptiveControllerReduction {
                state: AdaptiveControllerState {
                    attempt_number: next_attempt,
                    transient_retry_consumed: false,
                    ..state
                },
                decision: AdaptiveControllerDecision::ContinueSameRoute {
                    route: state.current_route,
                    next_attempt,
                },
            },
            pressure,
        );
    }

    (
        reduce_adaptive_controller(state, AdaptiveClassification::EscalationEligible),
        0,
    )
}

'''
replace_once(controller, marker, unfinished + marker)

controller_tests = "codex-rs/tui/src/adaptive_controller_tests.rs"
p = Path(controller_tests)
text = p.read_text()
if "unfinished_authorized_turn_continues_once_then_escalates_one_rung" not in text:
    text += r'''

#[test]
fn unfinished_authorized_turn_continues_once_then_escalates_one_rung() {
    let state = AdaptiveControllerState::initial(AdaptiveFamily::Astra);
    let (first, pressure) = reduce_unfinished_authorized_turn(state, 0);
    assert_eq!(pressure, 1);
    assert_eq!(
        first.decision,
        AdaptiveControllerDecision::ContinueSameRoute {
            route: route(AdaptiveFamily::Luna, AdaptiveEffort::Low),
            next_attempt: 2,
        }
    );
    assert_eq!(first.state.current_route, state.current_route);
    assert_eq!(first.state.attempt_number, 2);
    assert!(!first.state.transient_retry_consumed);

    let (second, pressure) = reduce_unfinished_authorized_turn(first.state, pressure);
    assert_eq!(pressure, 0);
    assert_eq!(
        second.decision,
        AdaptiveControllerDecision::EscalateEffort {
            from: route(AdaptiveFamily::Luna, AdaptiveEffort::Low),
            to: route(AdaptiveFamily::Luna, AdaptiveEffort::Medium),
            next_attempt: 3,
        }
    );
    assert_eq!(
        second.state.current_route,
        route(AdaptiveFamily::Luna, AdaptiveEffort::Medium)
    );
    assert_eq!(second.state.attempt_number, 3);

    let (third, pressure) = reduce_unfinished_authorized_turn(second.state, pressure);
    assert_eq!(pressure, 1);
    assert_eq!(
        third.decision,
        AdaptiveControllerDecision::ContinueSameRoute {
            route: route(AdaptiveFamily::Luna, AdaptiveEffort::Medium),
            next_attempt: 4,
        }
    );
}

#[test]
fn unfinished_pressure_is_suppressed_by_existing_hard_stops() {
    let paused = AdaptiveControllerState {
        paused_by_user: true,
        ..AdaptiveControllerState::initial(AdaptiveFamily::Astra)
    };
    let (reduced, pressure) = reduce_unfinished_authorized_turn(paused, 1);
    assert_eq!(reduced.decision, AdaptiveControllerDecision::NoAction);
    assert_eq!(pressure, 0);

    let terminal = AdaptiveControllerState {
        terminal: Some(AdaptiveWorkflowTerminal::RepairRequired),
        ..AdaptiveControllerState::initial(AdaptiveFamily::Astra)
    };
    let (reduced, pressure) = reduce_unfinished_authorized_turn(terminal, 1);
    assert_eq!(reduced.decision, AdaptiveControllerDecision::NoAction);
    assert_eq!(pressure, 0);
}
'''
    p.write_text(text)

effort = "codex-rs/tui/src/chatwidget/adaptive_effort.rs"
replace_once(
    effort,
    "use super::*;\nuse crate::adaptive_evidence::ADAPTIVE_FAILURE_PRESSURE_THRESHOLD;\n",
    "use super::*;\nuse crate::adaptive_controller::ADAPTIVE_UNFINISHED_TURN_THRESHOLD;\nuse crate::adaptive_evidence::ADAPTIVE_FAILURE_PRESSURE_THRESHOLD;\n",
)
replace_once(
    effort,
    "pub(crate) enum AdaptivePendingDecision {\n    BeginValidation,\n    RetrySameLevel,\n",
    "pub(crate) enum AdaptivePendingDecision {\n    BeginValidation,\n    ContinueSameRoute,\n    RetrySameLevel,\n",
)
replace_once(
    effort,
    "    pub(crate) transient_retry_consumed: bool,\n    pub(crate) last_processed_terminal_turn_id: Option<String>,\n",
    "    pub(crate) transient_retry_consumed: bool,\n    pub(crate) unfinished_turn_pressure: u8,\n    pub(crate) last_processed_terminal_turn_id: Option<String>,\n",
)
replace_once(
    effort,
    "            transient_retry_consumed: false,\n            last_processed_terminal_turn_id: None,\n",
    "            transient_retry_consumed: false,\n            unfinished_turn_pressure: 0,\n            last_processed_terminal_turn_id: None,\n",
)
replace_once(
    effort,
    "        self.adaptive_effort.last_failure_kind = None;\n        self.cancel_pending_adaptive_signal_for_active_turn();\n",
    "        self.adaptive_effort.last_failure_kind = None;\n        self.adaptive_effort.unfinished_turn_pressure = 0;\n        self.cancel_pending_adaptive_signal_for_active_turn();\n",
)
replace_once(
    effort,
    "            \"pause\" => {\n                self.cancel_pending_adaptive_signal_for_active_turn();\n                self.adaptive_effort.paused_by_user = true;\n",
    "            \"pause\" => {\n                self.cancel_pending_adaptive_signal_for_active_turn();\n                self.adaptive_effort.paused_by_user = true;\n                self.adaptive_effort.unfinished_turn_pressure = 0;\n",
)
replace_once(
    effort,
    "            \"off\" => {\n                self.cancel_pending_adaptive_signal_for_active_turn();\n                self.adaptive_effort.enabled = false;\n                self.adaptive_effort.paused_by_user = false;\n",
    "            \"off\" => {\n                self.cancel_pending_adaptive_signal_for_active_turn();\n                self.adaptive_effort.enabled = false;\n                self.adaptive_effort.paused_by_user = false;\n                self.adaptive_effort.unfinished_turn_pressure = 0;\n",
)
replace_once(
    effort,
    "            self.adaptive_effort.workflow_terminal = None;\n            self.adaptive_effort.pending_attempt = None;\n",
    "            self.adaptive_effort.workflow_terminal = None;\n            self.adaptive_effort.unfinished_turn_pressure = 0;\n            self.adaptive_effort.pending_attempt = None;\n",
)
replace_once(
    effort,
    "            self.adaptive_effort.workflow_terminal =\n                Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa);\n            self.adaptive_effort.pending_attempt = None;\n",
    "            self.adaptive_effort.workflow_terminal =\n                Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa);\n            self.adaptive_effort.unfinished_turn_pressure = 0;\n            self.adaptive_effort.pending_attempt = None;\n",
)
replace_once(
    effort,
    "            \"Adaptive Effort\\n  Enabled: {}\\n  Preference: {}\\n  Current: {} {}\\n  Attempt: {}\\n  Failure pressure: {}/{}\\n  Paused: {}\\n  Last outcome: {}\\n  Last failure: {}\\n  Worker role: {}\\n  Workflow terminal: {}\",\n",
    "            \"Adaptive Effort\\n  Enabled: {}\\n  Preference: {}\\n  Current: {} {}\\n  Attempt: {}\\n  Failure pressure: {}/{}\\n  Unfinished pressure: {}/{}\\n  Paused: {}\\n  Last outcome: {}\\n  Last failure: {}\\n  Worker role: {}\\n  Workflow terminal: {}\",\n",
)
replace_once(
    effort,
    "            failure_pressure,\n            ADAPTIVE_FAILURE_PRESSURE_THRESHOLD,\n            if state.paused_by_user { \"yes\" } else { \"no\" },\n",
    "            failure_pressure,\n            ADAPTIVE_FAILURE_PRESSURE_THRESHOLD,\n            state.unfinished_turn_pressure,\n            ADAPTIVE_UNFINISHED_TURN_THRESHOLD,\n            if state.paused_by_user { \"yes\" } else { \"no\" },\n",
)

bridge = "codex-rs/tui/src/chatwidget/adaptive_runtime_bridge.rs"
replace_once(
    bridge,
    "use crate::adaptive_controller::reduce_adaptive_controller;\n",
    "use crate::adaptive_controller::reduce_adaptive_controller;\nuse crate::adaptive_controller::reduce_unfinished_authorized_turn;\n",
)
replace_once(
    bridge,
    "        state.transient_retry_consumed = false;\n        state.last_failure_kind = None;\n",
    "        state.transient_retry_consumed = false;\n        state.unfinished_turn_pressure = 0;\n        state.last_failure_kind = None;\n",
)
anchor = '''    pub(super) fn apply_adaptive_terminal_signal(
        &mut self,
        source_turn_id: &str,
        signal: AdaptiveOutcomeSignal,
    ) {
        self.apply_adaptive_terminal_classification(
            source_turn_id,
            signal,
            classify_outcome(signal),
        );
    }

'''
method = '''    pub(super) fn apply_adaptive_unfinished_authorized_turn(
        &mut self,
        source_turn_id: &str,
    ) -> bool {
        let state = &self.adaptive_effort;
        let bound_worker = matches!(
            state.worker_context.role,
            crate::adaptive_worker::AdaptiveWorkerRole::Implementation
                | crate::adaptive_worker::AdaptiveWorkerRole::Validation
                | crate::adaptive_worker::AdaptiveWorkerRole::Repair
        ) && state
            .worker_context
            .authorized_scope
            .as_deref()
            .is_some_and(|scope| !scope.trim().is_empty());
        if !state.enabled
            || state.paused_by_user
            || state.workflow_terminal.is_some()
            || state.last_processed_terminal_turn_id.as_deref() == Some(source_turn_id)
            || !bound_worker
        {
            return false;
        }
        let (Some(starting_family), Some(current_family), Some(current_effort)) = (
            state.starting_family,
            state.current_family,
            state.current_effort,
        ) else {
            return false;
        };
        let controller = AdaptiveControllerState {
            starting_family,
            current_route: AdaptiveRoute {
                family: current_family,
                effort: current_effort,
            },
            attempt_number: state.attempt_number,
            transient_retry_consumed: state.transient_retry_consumed,
            paused_by_user: state.paused_by_user,
            terminal: state.workflow_terminal,
        };
        let (reduction, unfinished_turn_pressure) = reduce_unfinished_authorized_turn(
            controller,
            state.unfinished_turn_pressure,
        );

        self.adaptive_effort.last_processed_terminal_turn_id = Some(source_turn_id.to_string());
        self.adaptive_effort.current_family = Some(reduction.state.current_route.family);
        self.adaptive_effort.current_effort = Some(reduction.state.current_route.effort);
        self.adaptive_effort.attempt_number = reduction.state.attempt_number;
        self.adaptive_effort.transient_retry_consumed = reduction.state.transient_retry_consumed;
        self.adaptive_effort.unfinished_turn_pressure = unfinished_turn_pressure;
        self.adaptive_effort.paused_by_user = reduction.state.paused_by_user;
        self.adaptive_effort.workflow_terminal = reduction.state.terminal;
        self.adaptive_effort.successor_admission = None;
        self.adaptive_effort.pending_attempt = pending_attempt(
            self.thread_id(),
            &self.adaptive_effort.worker_context,
            source_turn_id,
            reduction.decision,
            reduction.state.current_route,
        );
        match reduction.decision {
            AdaptiveControllerDecision::ContinueSameRoute { .. } => {
                self.adaptive_effort.last_outcome = None;
                self.adaptive_effort.last_failure_kind = None;
            }
            AdaptiveControllerDecision::EscalateEffort { .. }
            | AdaptiveControllerDecision::EscalateModel { .. }
            | AdaptiveControllerDecision::Blocked => {
                self.adaptive_effort.last_outcome = Some(AdaptiveOutcome::Unknown);
                self.adaptive_effort.last_failure_kind = Some(AdaptiveFailureKind::Capability);
            }
            AdaptiveControllerDecision::InvalidState => {
                self.adaptive_effort.last_outcome = Some(AdaptiveOutcome::Unknown);
                self.adaptive_effort.last_failure_kind = Some(AdaptiveFailureKind::Unknown);
            }
            AdaptiveControllerDecision::NoAction
            | AdaptiveControllerDecision::RetrySameLevel { .. }
            | AdaptiveControllerDecision::ReadyForOwnerQa
            | AdaptiveControllerDecision::PausedByUser => {}
        }
        self.save_adaptive_effort_for_current_thread();
        self.apply_adaptive_route(reduction.decision, reduction.state.current_route);
        true
    }

'''
replace_once(bridge, anchor, anchor + method)
replace_once(
    bridge,
    "        let reduction = reduce_adaptive_controller(controller, classification);\n        self.adaptive_effort.last_processed_terminal_turn_id = Some(source_turn_id.to_string());\n",
    "        let reduction = reduce_adaptive_controller(controller, classification);\n        self.adaptive_effort.unfinished_turn_pressure = 0;\n        self.adaptive_effort.last_processed_terminal_turn_id = Some(source_turn_id.to_string());\n",
)
replace_once(
    bridge,
    "    let (decision, attempt_number) = match decision {\n        AdaptiveControllerDecision::RetrySameLevel { next_attempt, .. } => {\n",
    "    let (decision, attempt_number) = match decision {\n        AdaptiveControllerDecision::ContinueSameRoute { next_attempt, .. } => {\n            (AdaptivePendingDecision::ContinueSameRoute, next_attempt)\n        }\n        AdaptiveControllerDecision::RetrySameLevel { next_attempt, .. } => {\n",
)

protocol = "codex-rs/tui/src/chatwidget/protocol.rs"
replace_once(
    protocol,
    '''            let trusted_signal_consumed = matches!(notification.turn.status, TurnStatus::Completed)
                && self.consume_adaptive_signal_at_terminal(&notification.turn.id);
            if !trusted_signal_consumed {
                self.apply_adaptive_terminal_signal(&notification.turn.id, signal);
            }
''',
    '''            let completed = matches!(notification.turn.status, TurnStatus::Completed);
            let trusted_signal_consumed =
                completed && self.consume_adaptive_signal_at_terminal(&notification.turn.id);
            let unfinished_worker_consumed = completed
                && !trusted_signal_consumed
                && self.apply_adaptive_unfinished_authorized_turn(&notification.turn.id);
            if !trusted_signal_consumed && !unfinished_worker_consumed {
                self.apply_adaptive_terminal_signal(&notification.turn.id, signal);
            }
''',
)

admission = "codex-rs/tui/src/chatwidget/adaptive_admission.rs"
replace_once(
    admission,
    '''        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::BeginValidation => {
            "BeginValidation"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::RetrySameLevel => {
''',
    '''        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::BeginValidation => {
            "BeginValidation"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::ContinueSameRoute => {
            "ContinueSameRoute"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::RetrySameLevel => {
''',
)

trusted = "codex-rs/tui/src/chatwidget/adaptive_trusted_signal.rs"
replace_once(
    trusted,
    "        self.adaptive_effort.workflow_terminal = Some(terminal);\n        self.adaptive_effort.last_processed_terminal_turn_id = Some(source_turn_id.to_string());\n",
    "        self.adaptive_effort.workflow_terminal = Some(terminal);\n        self.adaptive_effort.unfinished_turn_pressure = 0;\n        self.adaptive_effort.last_processed_terminal_turn_id = Some(source_turn_id.to_string());\n",
)

session = "codex-rs/tui/src/session_state.rs"
replace_once(
    session,
    "            self.adaptive_effort.successor_admission = None;\n            self.adaptive_effort.evidence_registry = Default::default();\n",
    "            self.adaptive_effort.successor_admission = None;\n            self.adaptive_effort.unfinished_turn_pressure = 0;\n            self.adaptive_effort.evidence_registry = Default::default();\n",
)
replace_once(
    session,
    "            adaptive_effort.workflow_terminal = Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa);\n            adaptive_effort.pending_attempt = None;\n",
    "            adaptive_effort.workflow_terminal = Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa);\n            adaptive_effort.unfinished_turn_pressure = 0;\n            adaptive_effort.pending_attempt = None;\n",
)
replace_once(
    session,
    "            adaptive_effort.enabled = false;\n            adaptive_effort.paused_by_user = true;\n            adaptive_effort.pending_attempt = None;\n",
    "            adaptive_effort.enabled = false;\n            adaptive_effort.paused_by_user = true;\n            adaptive_effort.unfinished_turn_pressure = 0;\n            adaptive_effort.pending_attempt = None;\n",
)

effort_tests = "codex-rs/tui/src/chatwidget/tests/adaptive_effort_tests.rs"
entry = '''        (
            AdaptivePendingDecision::RetrySameLevel,
            admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Low),
        ),
'''
continuation_entry = '''        (
            AdaptivePendingDecision::ContinueSameRoute,
            admission_route(AdaptiveFamily::Terra, AdaptiveEffort::Low),
        ),
'''
replace_all(effort_tests, entry, continuation_entry + entry, minimum=2)
replace_all(
    effort_tests,
    '''                        AdaptivePendingDecision::BeginValidation => "BeginValidation",
                        AdaptivePendingDecision::RetrySameLevel => "RetrySameLevel",
''',
    '''                        AdaptivePendingDecision::BeginValidation => "BeginValidation",
                        AdaptivePendingDecision::ContinueSameRoute => "ContinueSameRoute",
                        AdaptivePendingDecision::RetrySameLevel => "RetrySameLevel",
''',
)

pressure_tests = "codex-rs/tui/src/chatwidget/tests/adaptive_pressure_tests.rs"
p = Path(pressure_tests)
text = p.read_text()
if "successful_work_without_handoff_continues_once_then_escalates" not in text:
    text += r'''

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
    assert_eq!(chat.adaptive_effort.current_family, Some(AdaptiveFamily::Luna));
    assert_eq!(chat.adaptive_effort.current_effort, Some(AdaptiveEffort::Low));
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
    assert_eq!(chat.adaptive_effort.current_family, Some(AdaptiveFamily::Luna));
    assert_eq!(chat.adaptive_effort.current_effort, Some(AdaptiveEffort::Medium));
    assert_matches!(
        chat.adaptive_effort.pending_attempt,
        Some(ref pending)
            if pending.decision == AdaptivePendingDecision::EscalateEffort
                && pending.attempt_number == 3
                && pending.worker_context == worker_context
    );
    assert_eq!(
        chat.adaptive_effort.last_failure_kind,
        Some(crate::adaptive_policy::AdaptiveFailureKind::Capability)
    );
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
    assert_eq!(chat.adaptive_effort.worker_context.role, AdaptiveWorkerRole::Validation);
}
'''
    p.write_text(text)
