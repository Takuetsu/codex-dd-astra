//! Exact-once admission state for a controller-authorized adaptive successor.

use super::*;
use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveRoute;
use crate::chatwidget::adaptive_effort::AdaptiveSuccessorAdmission;
use crate::chatwidget::adaptive_effort::AdaptiveSuccessorPermit;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveAdmissionWaitingReason {
    EffectiveRouteSynchronization,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveAdmissionSuppressedReason {
    AdaptiveDisabled,
    PausedByUser,
    WorkflowTerminal,
    ThreadLifecycle,
    AlreadyReserved,
    AlreadyConsumed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveAdmissionRejectedReason {
    MissingPendingAttempt,
    SourceTurnMismatch,
    ThreadMismatch,
    AttemptMismatch,
    RouteMismatch,
    WorkerContextMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveAdmissionResult {
    Authorized(AdaptiveSuccessorPermit),
    Waiting(AdaptiveAdmissionWaitingReason),
    Suppressed(AdaptiveAdmissionSuppressedReason),
    Rejected(AdaptiveAdmissionRejectedReason),
}

impl ChatWidget {
    pub(crate) fn maybe_submit_adaptive_successor(&mut self) -> bool {
        let Some(source_turn_id) = self.adaptive_effort.last_processed_terminal_turn_id.clone()
        else {
            return false;
        };

        let permit = match self.adaptive_effort.successor_admission.clone() {
            Some(AdaptiveSuccessorAdmission::Reserved(permit)) => permit,
            Some(AdaptiveSuccessorAdmission::Consumed(_)) => return false,
            None => match self.reserve_adaptive_successor_admission(&source_turn_id) {
                AdaptiveAdmissionResult::Authorized(permit) => permit,
                AdaptiveAdmissionResult::Waiting(_)
                | AdaptiveAdmissionResult::Suppressed(_)
                | AdaptiveAdmissionResult::Rejected(_) => return false,
            },
        };

        if !self.adaptive_successor_permit_is_still_valid(&permit) {
            self.adaptive_effort.successor_admission = None;
            self.adaptive_effort.pending_attempt = None;
            self.save_adaptive_effort_for_current_thread();
            return false;
        }

        if !self.consume_adaptive_successor_admission(&permit) {
            return false;
        }
        self.adaptive_effort.pending_attempt = None;
        self.save_adaptive_effort_for_current_thread();

        let message = UserMessage::from(adaptive_continuation_text(&permit));
        let accepted = self.submit_user_message_with_history_record(
            message,
            UserMessageHistoryRecord::UserMessageText,
        );
        if !accepted {
            tracing::error!(
                source_turn_id = %permit.source_turn_id,
                attempt = permit.attempt_number,
                "failed to submit consumed adaptive successor permit"
            );
        }
        accepted
    }

    fn adaptive_successor_permit_is_still_valid(&self, permit: &AdaptiveSuccessorPermit) -> bool {
        let state = &self.adaptive_effort;
        state.enabled
            && !state.paused_by_user
            && state.workflow_terminal.is_none()
            && !self.is_user_turn_pending_or_running()
            && !self.has_queued_follow_up_messages()
            && self.thread_id() == Some(permit.thread_id)
            && state.last_processed_terminal_turn_id.as_deref()
                == Some(permit.source_turn_id.as_str())
            && self.turn_lifecycle.last_turn_id.as_deref() == Some(permit.source_turn_id.as_str())
            && state.attempt_number == permit.attempt_number
            && state.worker_context == permit.worker_context
            && state.pending_attempt.as_ref().is_some_and(|pending| {
                pending.thread_id == permit.thread_id
                    && pending.source_turn_id == permit.source_turn_id
                    && pending.decision == permit.decision
                    && pending.route == permit.route
                    && pending.attempt_number == permit.attempt_number
                    && pending.worker_context == permit.worker_context
            })
            && state.current_family == Some(permit.route.family)
            && state.current_effort == Some(permit.route.effort)
            && self.current_model() == permit.route.family.model()
            && self.effective_reasoning_effort() == Some(reasoning_effort(permit.route.effort))
    }

    /// Reserves the one admission permit authorized by the current pending attempt.
    ///
    /// Slice 7D-B may trust the immutable permit facts, but must atomically consume the same
    /// canonical permit and revalidate thread lifecycle, pause, terminal, Worker binding, and
    /// effective route immediately before submitting a native continuation.
    pub(crate) fn reserve_adaptive_successor_admission(
        &mut self,
        source_turn_id: &str,
    ) -> AdaptiveAdmissionResult {
        let state = &self.adaptive_effort;
        if !state.enabled {
            return AdaptiveAdmissionResult::Suppressed(
                AdaptiveAdmissionSuppressedReason::AdaptiveDisabled,
            );
        }
        if state.paused_by_user {
            return AdaptiveAdmissionResult::Suppressed(
                AdaptiveAdmissionSuppressedReason::PausedByUser,
            );
        }
        if state.workflow_terminal.is_some() {
            return AdaptiveAdmissionResult::Suppressed(
                AdaptiveAdmissionSuppressedReason::WorkflowTerminal,
            );
        }
        if self.turn_lifecycle.agent_turn_running || self.bottom_pane.is_task_running() {
            return AdaptiveAdmissionResult::Suppressed(
                AdaptiveAdmissionSuppressedReason::ThreadLifecycle,
            );
        }
        if let Some(admission) = &state.successor_admission {
            return AdaptiveAdmissionResult::Suppressed(match admission {
                AdaptiveSuccessorAdmission::Reserved(_) => {
                    AdaptiveAdmissionSuppressedReason::AlreadyReserved
                }
                AdaptiveSuccessorAdmission::Consumed(_) => {
                    AdaptiveAdmissionSuppressedReason::AlreadyConsumed
                }
            });
        }
        let Some(pending) = state.pending_attempt.as_ref() else {
            return AdaptiveAdmissionResult::Rejected(
                AdaptiveAdmissionRejectedReason::MissingPendingAttempt,
            );
        };
        if pending.source_turn_id != source_turn_id
            || state.last_processed_terminal_turn_id.as_deref() != Some(source_turn_id)
            || self.turn_lifecycle.last_turn_id.as_deref() != Some(source_turn_id)
        {
            return AdaptiveAdmissionResult::Rejected(
                AdaptiveAdmissionRejectedReason::SourceTurnMismatch,
            );
        }
        if self.thread_id() != Some(pending.thread_id) {
            return AdaptiveAdmissionResult::Rejected(
                AdaptiveAdmissionRejectedReason::ThreadMismatch,
            );
        }
        if pending.attempt_number != state.attempt_number {
            return AdaptiveAdmissionResult::Rejected(
                AdaptiveAdmissionRejectedReason::AttemptMismatch,
            );
        }
        let canonical_route = match (state.current_family, state.current_effort) {
            (Some(family), Some(effort)) => AdaptiveRoute { family, effort },
            _ => {
                return AdaptiveAdmissionResult::Rejected(
                    AdaptiveAdmissionRejectedReason::RouteMismatch,
                );
            }
        };
        if pending.route != canonical_route {
            return AdaptiveAdmissionResult::Rejected(
                AdaptiveAdmissionRejectedReason::RouteMismatch,
            );
        }
        if pending.worker_context != state.worker_context {
            return AdaptiveAdmissionResult::Rejected(
                AdaptiveAdmissionRejectedReason::WorkerContextMismatch,
            );
        }
        if self.current_model() != pending.route.family.model()
            || self.effective_reasoning_effort() != Some(reasoning_effort(pending.route.effort))
        {
            return AdaptiveAdmissionResult::Waiting(
                AdaptiveAdmissionWaitingReason::EffectiveRouteSynchronization,
            );
        }

        let permit = AdaptiveSuccessorPermit {
            thread_id: pending.thread_id,
            source_turn_id: pending.source_turn_id.clone(),
            decision: pending.decision,
            route: pending.route,
            attempt_number: pending.attempt_number,
            worker_context: pending.worker_context.clone(),
        };
        self.adaptive_effort.successor_admission =
            Some(AdaptiveSuccessorAdmission::Reserved(permit.clone()));
        self.save_adaptive_effort_for_current_thread();
        AdaptiveAdmissionResult::Authorized(permit)
    }

    pub(crate) fn consume_adaptive_successor_admission(
        &mut self,
        permit: &AdaptiveSuccessorPermit,
    ) -> bool {
        if self.adaptive_effort.successor_admission
            != Some(AdaptiveSuccessorAdmission::Reserved(permit.clone()))
        {
            return false;
        }
        self.adaptive_effort.successor_admission =
            Some(AdaptiveSuccessorAdmission::Consumed(permit.clone()));
        self.save_adaptive_effort_for_current_thread();
        true
    }
}

fn adaptive_continuation_text(permit: &AdaptiveSuccessorPermit) -> String {
    let decision = match permit.decision {
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::RetrySameLevel => {
            "RetrySameLevel"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::EscalateEffort => {
            "EscalateEffort"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::EscalateModel => {
            "EscalateModel"
        }
    };
    let effort = match permit.route.effort {
        AdaptiveEffort::Low => "Low",
        AdaptiveEffort::Medium => "Medium",
        AdaptiveEffort::High => "High",
    };
    let model = permit.route.family.model();
    let attempt = permit.attempt_number;
    format!(
        "[Adaptive continuation] Continue the current assigned Worker task from the existing thread state. Attempt {attempt} is authorized at {model} {effort} using {decision}. Preserve the existing task, scope, worktree, evidence, and acceptance criteria. Continue from current progress; do not restart or broaden the task."
    )
}

fn reasoning_effort(effort: AdaptiveEffort) -> ReasoningEffortConfig {
    match effort {
        AdaptiveEffort::Low => ReasoningEffortConfig::Low,
        AdaptiveEffort::Medium => ReasoningEffortConfig::Medium,
        AdaptiveEffort::High => ReasoningEffortConfig::High,
    }
}
