//! Live terminal-event bridge for the deterministic adaptive controller.

use super::*;
use crate::adaptive_budget::mechanical_validation_route;
use crate::adaptive_complexity::AdaptiveComplexityClass;
use crate::adaptive_complexity::AdaptiveImplementationPhase;
use crate::adaptive_complexity::implementation_floor;
use crate::adaptive_controller::AdaptiveControllerDecision;
use crate::adaptive_controller::AdaptiveControllerState;
use crate::adaptive_controller::reduce_adaptive_controller;
use crate::adaptive_controller::reduce_unfinished_authorized_turn;
use crate::adaptive_policy::AdaptiveClassification;
use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveFailureKind;
use crate::adaptive_policy::AdaptiveOutcome;
use crate::adaptive_policy::AdaptiveOutcomeSignal;
use crate::adaptive_policy::AdaptiveRoute;
use crate::adaptive_policy::classify_outcome;
use crate::adaptive_policy::stronger_route;
use crate::chatwidget::adaptive_effort::AdaptivePendingAttempt;
use crate::chatwidget::adaptive_effort::AdaptivePendingDecision;

impl ChatWidget {
    pub(super) fn apply_adaptive_complexity_floor(
        &mut self,
        source_turn_id: &str,
        complexity_class: AdaptiveComplexityClass,
    ) {
        let Some(thread_id) = self.thread_id() else {
            return;
        };
        let (Some(current_family), Some(current_effort)) = (
            self.adaptive_effort.current_family,
            self.adaptive_effort.current_effort,
        ) else {
            return;
        };

        let current_route = AdaptiveRoute {
            family: current_family,
            effort: current_effort,
        };
        let route = implementation_floor(complexity_class);
        let decision = if route == current_route {
            AdaptivePendingDecision::ContinueSameRoute
        } else if route.family == current_route.family {
            AdaptivePendingDecision::EscalateEffort
        } else {
            AdaptivePendingDecision::EscalateModel
        };
        let next_attempt = self.adaptive_effort.attempt_number.saturating_add(1);

        self.adaptive_effort.complexity_class = Some(complexity_class);
        self.adaptive_effort.implementation_phase =
            Some(AdaptiveImplementationPhase::Implementation);
        self.adaptive_effort.current_family = Some(route.family);
        self.adaptive_effort.current_effort = Some(route.effort);
        self.adaptive_effort.attempt_number = next_attempt;
        self.adaptive_effort.unfinished_turn_pressure = 0;
        self.adaptive_effort.last_processed_terminal_turn_id = Some(source_turn_id.to_string());
        self.adaptive_effort.last_outcome = None;
        self.adaptive_effort.last_failure_kind = None;
        self.adaptive_effort.successor_admission = None;
        self.adaptive_effort.pending_attempt = Some(AdaptivePendingAttempt {
            thread_id,
            source_turn_id: source_turn_id.to_string(),
            decision,
            route,
            attempt_number: next_attempt,
            worker_context: self.adaptive_effort.worker_context.clone(),
        });
        self.save_adaptive_effort_for_current_thread();

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

    pub(super) fn apply_adaptive_implementation_phase(
        &mut self,
        source_turn_id: &str,
        phase: AdaptiveImplementationPhase,
    ) {
        let Some(thread_id) = self.thread_id() else {
            return;
        };
        let Some(complexity_class) = self.adaptive_effort.complexity_class else {
            return;
        };
        let (Some(current_family), Some(current_effort)) = (
            self.adaptive_effort.current_family,
            self.adaptive_effort.current_effort,
        ) else {
            return;
        };
        let current_route = AdaptiveRoute {
            family: current_family,
            effort: current_effort,
        };
        let (route, decision) = match phase {
            AdaptiveImplementationPhase::MechanicalValidation => (
                mechanical_validation_route(self.adaptive_effort.budget_mode),
                AdaptivePendingDecision::EnterMechanicalValidation,
            ),
            AdaptiveImplementationPhase::Implementation => {
                let floor = implementation_floor(complexity_class);
                (
                    stronger_route(current_route, floor).unwrap_or(floor),
                    AdaptivePendingDecision::ResumeImplementation,
                )
            }
        };
        let next_attempt = self.adaptive_effort.attempt_number.saturating_add(1);

        self.adaptive_effort.implementation_phase = Some(phase);
        self.adaptive_effort.current_family = Some(route.family);
        self.adaptive_effort.current_effort = Some(route.effort);
        self.adaptive_effort.attempt_number = next_attempt;
        self.adaptive_effort.unfinished_turn_pressure = 0;
        self.adaptive_effort.last_processed_terminal_turn_id = Some(source_turn_id.to_string());
        self.adaptive_effort.last_outcome = None;
        self.adaptive_effort.last_failure_kind = None;
        self.adaptive_effort.successor_admission = None;
        self.adaptive_effort.pending_attempt = Some(AdaptivePendingAttempt {
            thread_id,
            source_turn_id: source_turn_id.to_string(),
            decision,
            route,
            attempt_number: next_attempt,
            worker_context: self.adaptive_effort.worker_context.clone(),
        });
        self.save_adaptive_effort_for_current_thread();

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

    pub(super) fn apply_adaptive_unfinished_authorized_turn(
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
        let current_route = AdaptiveRoute {
            family: current_family,
            effort: current_effort,
        };
        if state.worker_context.role
            == crate::adaptive_worker::AdaptiveWorkerRole::Validation
            && let Some(thread_id) = self.thread_id()
        {
            let previous_turn_had_conclusive_evidence = state
                .last_processed_terminal_turn_id
                .as_deref()
                .is_some_and(|turn_id| {
                    state
                        .evidence_registry
                        .has_conclusive_evidence_for_turn(thread_id, turn_id)
                });
            let current_turn_has_conclusive_evidence = state
                .evidence_registry
                .has_conclusive_evidence_for_turn(thread_id, source_turn_id);

            // A Validation Worker that already produced native evidence but omitted its required
            // workflow terminal gets exactly one same-route terminalization turn. Do not spend
            // additional budget rerunning already-complete validation just because the model
            // forgot to close the adaptive contract.
            if current_turn_has_conclusive_evidence
                && !(state.unfinished_turn_pressure > 0
                    && previous_turn_had_conclusive_evidence)
            {
                let next_attempt = state.attempt_number.saturating_add(1);
                self.adaptive_effort.last_processed_terminal_turn_id =
                    Some(source_turn_id.to_string());
                self.adaptive_effort.current_family = Some(current_route.family);
                self.adaptive_effort.current_effort = Some(current_route.effort);
                self.adaptive_effort.attempt_number = next_attempt;
                self.adaptive_effort.transient_retry_consumed = false;
                self.adaptive_effort.unfinished_turn_pressure = 1;
                self.adaptive_effort.last_outcome = None;
                self.adaptive_effort.last_failure_kind = None;
                self.adaptive_effort.successor_admission = None;
                self.adaptive_effort.pending_attempt = Some(AdaptivePendingAttempt {
                    thread_id,
                    source_turn_id: source_turn_id.to_string(),
                    decision: AdaptivePendingDecision::ValidationTerminalization,
                    route: current_route,
                    attempt_number: next_attempt,
                    worker_context: self.adaptive_effort.worker_context.clone(),
                });
                self.save_adaptive_effort_for_current_thread();
                return true;
            }

            // If the dedicated terminalization turn itself ends without ready_for_owner_qa or
            // repair_required, stop deterministically rather than entering the ordinary
            // unfinished-pressure escalation ladder. The owner can inspect the preserved
            // evidence without paying for repeated validation reruns.
            if state.unfinished_turn_pressure > 0 && previous_turn_had_conclusive_evidence {
                self.apply_adaptive_terminal_classification(
                    source_turn_id,
                    AdaptiveOutcomeSignal::Failure(AdaptiveFailureKind::OwnerDecision),
                    AdaptiveClassification::Blocked,
                );
                self.add_info_message(
                    "Validation completed with native evidence but the dedicated terminalization turn ended without ready_for_owner_qa or repair_required. CodexDD stopped the Worker at BLOCKED instead of rerunning validation or escalating compute."
                        .to_string(),
                    None,
                );
                return true;
            }
        }

        let controller = AdaptiveControllerState {
            starting_family,
            current_route,
            attempt_number: state.attempt_number,
            transient_retry_consumed: state.transient_retry_consumed,
            paused_by_user: state.paused_by_user,
            terminal: state.workflow_terminal,
        };
        let (reduction, unfinished_turn_pressure) =
            reduce_unfinished_authorized_turn(controller, state.unfinished_turn_pressure);

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
            AdaptiveControllerDecision::ContinueSameRoute { .. }
            | AdaptiveControllerDecision::EscalateEffort { .. }
            | AdaptiveControllerDecision::RequireModelEscalationReport { .. } => {
                // An ordinary unfinished turn is not capability evidence. Keep status projection
                // neutral even when bounded unfinished pressure raises effort inside one family.
                self.adaptive_effort.last_outcome = None;
                self.adaptive_effort.last_failure_kind = None;
            }
            AdaptiveControllerDecision::EscalateModel { .. } => {
                // This function never authorizes model-family escalation from unfinished pressure.
                // Keep this arm defensive in case the controller contract changes later.
                self.adaptive_effort.last_outcome = Some(AdaptiveOutcome::Unknown);
                self.adaptive_effort.last_failure_kind = Some(AdaptiveFailureKind::Capability);
            }
            AdaptiveControllerDecision::Blocked => {
                self.adaptive_effort.last_outcome = Some(AdaptiveOutcome::Unknown);
                self.adaptive_effort.last_failure_kind = Some(AdaptiveFailureKind::Unknown);
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
        if let AdaptiveControllerDecision::RequireModelEscalationReport { from, to } =
            reduction.decision
        {
            self.add_info_message(
                format!(
                    "Adaptive model escalation withheld\n  From: {} {:?}\n  Proposed: {} {:?}\n  Required: fresh trusted Capability signal with a nonblank diagnostic report explaining why the stronger model family is warranted.",
                    from.family.model(),
                    from.effort,
                    to.family.model(),
                    to.effort,
                ),
                None,
            );
        }
        self.save_adaptive_effort_for_current_thread();
        self.apply_adaptive_route(reduction.decision, reduction.state.current_route);
        true
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
        self.adaptive_effort.unfinished_turn_pressure = 0;
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
        AdaptiveControllerDecision::ContinueSameRoute { next_attempt, .. } => {
            (AdaptivePendingDecision::ContinueSameRoute, next_attempt)
        }
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
        | AdaptiveControllerDecision::RequireModelEscalationReport { .. }
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
        AdaptiveEffort::XHigh => ReasoningEffortConfig::XHigh,
        AdaptiveEffort::Max => ReasoningEffortConfig::Max,
    }
}
