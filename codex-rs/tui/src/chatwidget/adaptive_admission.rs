//! Exact-once admission state for a controller-authorized adaptive successor.

use super::*;
use crate::adaptive_evidence::AdaptiveEvidenceOutcome;
use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveRoute;
use crate::adaptive_worker::AdaptiveWorkerRole;
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
    AwaitingTrustedSignal,
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
        if matches!(
            self.adaptive_effort.pending_signal.as_ref(),
            Some(crate::chatwidget::adaptive_effort::AdaptivePendingSignal::Awaiting { .. })
        ) {
            return false;
        }
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

        let requires_complexity_recovery = permit.worker_context.role
            == AdaptiveWorkerRole::Implementation
            && self.adaptive_effort.complexity_class.is_none();

        if !self.consume_adaptive_successor_admission(&permit) {
            return false;
        }
        self.adaptive_effort.pending_attempt = None;
        self.save_adaptive_effort_for_current_thread();

        let (validation_success_refs, validation_failure_refs) =
            if permit.decision
                == crate::chatwidget::adaptive_effort::AdaptivePendingDecision::ValidationTerminalization
            {
                (
                    self.adaptive_effort
                        .evidence_registry
                        .conclusive_refs_for_turn(
                            permit.thread_id,
                            &permit.source_turn_id,
                            AdaptiveEvidenceOutcome::Success,
                        ),
                    self.adaptive_effort
                        .evidence_registry
                        .conclusive_refs_for_turn(
                            permit.thread_id,
                            &permit.source_turn_id,
                            AdaptiveEvidenceOutcome::Failure,
                        ),
                )
            } else {
                (Vec::new(), Vec::new())
            };
        let message = UserMessage::from(adaptive_continuation_text(
            &permit,
            requires_complexity_recovery,
            &validation_success_refs,
            &validation_failure_refs,
        ));
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
        if matches!(
            state.pending_signal.as_ref(),
            Some(crate::chatwidget::adaptive_effort::AdaptivePendingSignal::Awaiting { .. })
        ) {
            return AdaptiveAdmissionResult::Suppressed(
                AdaptiveAdmissionSuppressedReason::AwaitingTrustedSignal,
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

fn adaptive_continuation_text(
    permit: &AdaptiveSuccessorPermit,
    requires_complexity_recovery: bool,
    validation_success_refs: &[String],
    validation_failure_refs: &[String],
) -> String {
    let decision = match permit.decision {
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::ContinueSameRoute => {
            "ContinueSameRoute"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::RetrySameLevel => {
            "RetrySameLevel"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::EscalateEffort => {
            "EscalateEffort"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::EscalateModel => {
            "EscalateModel"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::EnterMechanicalValidation => {
            "EnterMechanicalValidation"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::ResumeImplementation => {
            "ResumeImplementation"
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::ValidationTerminalization => {
            "ValidationTerminalization"
        }
    };
    let effort = match permit.route.effort {
        AdaptiveEffort::Low => "Low",
        AdaptiveEffort::Medium => "Medium",
        AdaptiveEffort::High => "High",
        AdaptiveEffort::XHigh => "XHigh",
        AdaptiveEffort::Max => "Max",
    };
    let model = permit.route.family.model();
    let attempt = permit.attempt_number;
    if requires_complexity_recovery {
        return format!(
            "[Adaptive continuation] The required initial Implementation complexity reconnaissance was not accepted. Attempt {attempt} remains authorized at {model} {effort} using {decision}. Before any further implementation edits, inspect the bounded task state, report kind=complexity with the complete structured complexity object, and end the turn. Preserve the existing task, scope, worktree, evidence, and acceptance criteria. Do not report ready_for_validation until a complexity class has been accepted."
        );
    }
    match permit.decision {
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::ValidationTerminalization => {
            let successful_refs = if validation_success_refs.is_empty() {
                "none".to_string()
            } else {
                validation_success_refs.join(", ")
            };
            let failing_refs = if validation_failure_refs.is_empty() {
                "none".to_string()
            } else {
                validation_failure_refs.join(", ")
            };
            format!(
                "[Adaptive continuation] Objective Validation evidence is already present, but the prior turn ended without the required workflow terminal. Attempt {attempt} remains authorized at {model} {effort} using {decision} for terminalization only. Do not modify the implementation and do not rerun validation that is already complete merely to recreate evidence. Existing successful native evidence refs: {successful_refs}. Existing failing native evidence refs: {failing_refs}. Decide the bounded Validation verdict from the current thread state. If green, call report_adaptive_signal with kind=ready_for_owner_qa and one or more successful native evidence refs. If a blocker is established, call report_adaptive_signal with kind=repair_required and one or more failing native evidence refs. If the existing evidence is insufficient to support the verdict, run only the minimum missing objective validation needed to produce the required native evidence, then report the terminal. End the turn immediately after reporting. Final-answer prose is non-authoritative and cannot close this Worker."
            )
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::EnterMechanicalValidation => {
            format!(
                "[Adaptive continuation] Source-changing implementation is complete. Attempt {attempt} is authorized at {model} {effort} using {decision} for mechanical validation only. Run tests, formatting checks, builds, diff inspection, and evidence collection without editing source. Submit each bounded validation command directly as its own exec_command; do not wrap validation in shell assignments, control-flow blocks, or command-chaining wrappers. If validation reveals that source changes or renewed implementation reasoning are required, report kind=implementation_work and end the turn before editing. Preserve the existing task, scope, worktree, and acceptance criteria."
            )
        }
        crate::chatwidget::adaptive_effort::AdaptivePendingDecision::ResumeImplementation => {
            format!(
                "[Adaptive continuation] Mechanical validation found source-changing implementation work. Attempt {attempt} is restored at {model} {effort} using {decision}; this route is at least the previously accepted complexity floor and preserves any stronger trusted route. Continue only the required bounded implementation edits. When source-changing work is complete again, report kind=mechanical_validation and end the turn before returning to tests/builds."
            )
        }
        _ => format!(
            "[Adaptive continuation] Continue the current assigned Worker task from the existing thread state. Attempt {attempt} is authorized at {model} {effort} using {decision}. Preserve the existing task, scope, worktree, evidence, and acceptance criteria. Continue from current progress; do not restart or broaden the task."
        ),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptive_policy::AdaptiveFamily;
    use crate::adaptive_worker::AdaptiveWorkerContext;
    use codex_protocol::ThreadId;

    #[test]
    fn validation_terminalization_prompt_reuses_existing_evidence_and_forbids_rerun() {
        let permit = AdaptiveSuccessorPermit {
            thread_id: ThreadId::new(),
            source_turn_id: "validation-evidence-turn".to_string(),
            decision:
                crate::chatwidget::adaptive_effort::AdaptivePendingDecision::ValidationTerminalization,
            route: AdaptiveRoute {
                family: AdaptiveFamily::Sol,
                effort: AdaptiveEffort::Low,
            },
            attempt_number: 2,
            worker_context: AdaptiveWorkerContext {
                role: AdaptiveWorkerRole::Validation,
                authorized_scope: Some("breakwater/Z-A0.52A-validation".to_string()),
            },
        };

        let text = adaptive_continuation_text(
            &permit,
            /*requires_complexity_recovery*/ false,
            &["green-proof".to_string()],
            &["failed-proof".to_string()],
        );

        assert!(text.contains("terminalization only"));
        assert!(text.contains("do not rerun validation"));
        assert!(text.contains("green-proof"));
        assert!(text.contains("failed-proof"));
        assert!(text.contains("kind=ready_for_owner_qa"));
        assert!(text.contains("kind=repair_required"));
        assert!(text.contains("Final-answer prose is non-authoritative"));
    }
}
