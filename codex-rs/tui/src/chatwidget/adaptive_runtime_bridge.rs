//! Live terminal-event bridge for the deterministic adaptive controller.

use super::*;
use crate::adaptive_controller::AdaptiveControllerDecision;
use crate::adaptive_controller::AdaptiveControllerState;
use crate::adaptive_controller::reduce_adaptive_controller;
use crate::adaptive_policy::AdaptiveClassification;
use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveFailureKind;
use crate::adaptive_policy::AdaptiveOutcome;
use crate::adaptive_policy::AdaptiveOutcomeSignal;
use crate::adaptive_policy::AdaptiveRoute;
use crate::adaptive_policy::classify_outcome;
use crate::chatwidget::adaptive_effort::AdaptivePendingAttempt;
use crate::chatwidget::adaptive_effort::AdaptivePendingDecision;

impl ChatWidget {
    pub(super) fn apply_adaptive_terminal_signal(
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

    pub(super) fn apply_adaptive_terminal_classification(
        &mut self,
        source_turn_id: &str,
        signal: AdaptiveOutcomeSignal,
        classification: AdaptiveClassification,
    ) {
        let state = &self.adaptive_effort;
        if !state.enabled
            || state.paused_by_user
            || state.workflow_terminal.is_some()
            || state.last_processed_terminal_turn_id.as_deref() == Some(source_turn_id)
        {
            return;
        }
        let (Some(starting_family), Some(current_family), Some(current_effort)) = (
            state.starting_family,
            state.current_family,
            state.current_effort,
        ) else {
            return;
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
        let reduction = reduce_adaptive_controller(controller, classification);
        self.adaptive_effort.last_processed_terminal_turn_id = Some(source_turn_id.to_string());
        self.adaptive_effort.current_family = Some(reduction.state.current_route.family);
        self.adaptive_effort.current_effort = Some(reduction.state.current_route.effort);
        self.adaptive_effort.attempt_number = reduction.state.attempt_number;
        self.adaptive_effort.transient_retry_consumed = reduction.state.transient_retry_consumed;
        self.adaptive_effort.paused_by_user = reduction.state.paused_by_user;
        self.adaptive_effort.workflow_terminal = reduction.state.terminal;
        // A new controller result supersedes any previous, unconsumed admission.
        self.adaptive_effort.successor_admission = None;
        self.adaptive_effort.pending_attempt = pending_attempt(
            self.thread_id(),
            &self.adaptive_effort.worker_context,
            source_turn_id,
            reduction.decision,
            reduction.state.current_route,
        );
        project_signal(&mut self.adaptive_effort, signal);
        self.save_adaptive_effort_for_current_thread();
        self.apply_adaptive_route(reduction.decision, reduction.state.current_route);
    }

    fn apply_adaptive_route(&self, decision: AdaptiveControllerDecision, route: AdaptiveRoute) {
        if !matches!(
            decision,
            AdaptiveControllerDecision::EscalateEffort { .. }
                | AdaptiveControllerDecision::EscalateModel { .. }
        ) {
            return;
        }
        let model = route.family.model();
        if self.current_model() != model {
            self.app_event_tx
                .send(AppEvent::UpdateModel(model.to_string()));
        }
        let effort = reasoning_effort(route.effort);
        if self.effective_reasoning_effort() != Some(effort.clone()) {
            self.app_event_tx
                .send(AppEvent::UpdateReasoningEffort(Some(effort)));
        }
    }
}

fn pending_attempt(
    thread_id: Option<codex_protocol::ThreadId>,
    worker_context: &crate::adaptive_worker::AdaptiveWorkerContext,
    source_turn_id: &str,
    decision: AdaptiveControllerDecision,
    route: AdaptiveRoute,
) -> Option<AdaptivePendingAttempt> {
    let thread_id = thread_id?;
    let (decision, attempt_number) = match decision {
        AdaptiveControllerDecision::RetrySameLevel { next_attempt, .. } => {
            (AdaptivePendingDecision::RetrySameLevel, next_attempt)
        }
        AdaptiveControllerDecision::EscalateEffort { next_attempt, .. } => {
            (AdaptivePendingDecision::EscalateEffort, next_attempt)
        }
        AdaptiveControllerDecision::EscalateModel { next_attempt, .. } => {
            (AdaptivePendingDecision::EscalateModel, next_attempt)
        }
        AdaptiveControllerDecision::NoAction
        | AdaptiveControllerDecision::ReadyForOwnerQa
        | AdaptiveControllerDecision::Blocked
        | AdaptiveControllerDecision::PausedByUser
        | AdaptiveControllerDecision::InvalidState => return None,
    };
    Some(AdaptivePendingAttempt {
        thread_id,
        source_turn_id: source_turn_id.to_string(),
        decision,
        route,
        attempt_number,
        worker_context: worker_context.clone(),
    })
}

fn project_signal(
    state: &mut crate::chatwidget::adaptive_effort::AdaptiveEffortState,
    signal: AdaptiveOutcomeSignal,
) {
    match signal {
        AdaptiveOutcomeSignal::Completed => {}
        AdaptiveOutcomeSignal::Failed => {
            state.last_outcome = Some(AdaptiveOutcome::Unknown);
            state.last_failure_kind = Some(AdaptiveFailureKind::Unknown);
        }
        AdaptiveOutcomeSignal::Failure(kind) => {
            state.last_failure_kind = Some(kind);
            state.last_outcome = match kind {
                AdaptiveFailureKind::UserInterrupted => Some(AdaptiveOutcome::UserInterrupted),
                AdaptiveFailureKind::ReadyForOwnerQa => Some(AdaptiveOutcome::ReadyForOwnerQa),
                AdaptiveFailureKind::Capability
                | AdaptiveFailureKind::TransientInfrastructure
                | AdaptiveFailureKind::Authentication
                | AdaptiveFailureKind::Quota
                | AdaptiveFailureKind::Environment
                | AdaptiveFailureKind::Permission
                | AdaptiveFailureKind::Dependency
                | AdaptiveFailureKind::ModelUnavailable
                | AdaptiveFailureKind::OwnerDecision
                | AdaptiveFailureKind::AuthorizationOrScope
                | AdaptiveFailureKind::Unknown => Some(AdaptiveOutcome::Unknown),
            };
        }
        AdaptiveOutcomeSignal::UserInterrupted => {
            state.last_outcome = Some(AdaptiveOutcome::UserInterrupted);
            state.last_failure_kind = None;
        }
        AdaptiveOutcomeSignal::ReadyForOwnerQa => {
            state.last_outcome = Some(AdaptiveOutcome::ReadyForOwnerQa);
            state.last_failure_kind = None;
        }
    }
}

fn reasoning_effort(effort: AdaptiveEffort) -> ReasoningEffortConfig {
    match effort {
        AdaptiveEffort::Low => ReasoningEffortConfig::Low,
        AdaptiveEffort::Medium => ReasoningEffortConfig::Medium,
        AdaptiveEffort::High => ReasoningEffortConfig::High,
    }
}
