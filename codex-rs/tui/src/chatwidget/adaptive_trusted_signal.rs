//! Terminal-bound validation and consumption of trusted adaptive signals.

use super::*;
use crate::adaptive_evidence::ADAPTIVE_FAILURE_PRESSURE_THRESHOLD;
use crate::adaptive_evidence::AdaptiveEvidenceOutcome;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use crate::chatwidget::adaptive_effort::AdaptivePendingSignal;
use codex_protocol::protocol::AdaptiveRuntimeSignalKind;

impl ChatWidget {
    pub(super) fn consume_adaptive_signal_at_terminal(&mut self, source_turn_id: &str) -> bool {
        if self.adaptive_effort.workflow_terminal.is_some() {
            return false;
        }
        let Some(pending_signal) = self.adaptive_effort.pending_signal.as_ref() else {
            return false;
        };
        let envelope = match pending_signal {
            AdaptivePendingSignal::Pending(envelope)
                if envelope.source_turn_id == source_turn_id =>
            {
                envelope
            }
            AdaptivePendingSignal::Conflicted {
                source_turn_id: pending_turn_id,
            } if pending_turn_id == source_turn_id => {
                if !self.native_failure_pressure_available(source_turn_id) {
                    return false;
                }
                self.adaptive_effort.pending_signal = Some(AdaptivePendingSignal::Consumed {
                    source_turn_id: source_turn_id.to_string(),
                });
                self.apply_adaptive_terminal_signal(
                    source_turn_id,
                    crate::adaptive_policy::AdaptiveOutcomeSignal::Failure(
                        crate::adaptive_policy::AdaptiveFailureKind::Capability,
                    ),
                );
                return true;
            }
            _ => return false,
        };

        let accepted = match envelope.signal_kind {
            AdaptiveRuntimeSignalKind::Capability => envelope.evidence_refs.is_empty(),
            AdaptiveRuntimeSignalKind::ReadyForValidation => {
                envelope.evidence_refs.is_empty()
                    && matches!(
                        self.adaptive_effort.worker_context.role,
                        AdaptiveWorkerRole::Implementation | AdaptiveWorkerRole::Repair
                    )
            }
            AdaptiveRuntimeSignalKind::RepairRequired => {
                self.adaptive_effort.worker_context.role == AdaptiveWorkerRole::Validation
                    && self.valid_evidence_refs(
                        &envelope.evidence_refs,
                        AdaptiveEvidenceOutcome::Failure,
                    )
            }
            AdaptiveRuntimeSignalKind::ReadyForOwnerQa => {
                self.adaptive_effort.worker_context.role == AdaptiveWorkerRole::Validation
                    && self.valid_evidence_refs(
                        &envelope.evidence_refs,
                        AdaptiveEvidenceOutcome::Success,
                    )
            }
        };
        let signal_kind = envelope.signal_kind;
        self.adaptive_effort.pending_signal = Some(if accepted {
            AdaptivePendingSignal::Consumed {
                source_turn_id: source_turn_id.to_string(),
            }
        } else {
            AdaptivePendingSignal::Cancelled {
                source_turn_id: source_turn_id.to_string(),
            }
        });
        if !accepted {
            if self.native_failure_pressure_available(source_turn_id) {
                self.adaptive_effort.pending_signal = Some(AdaptivePendingSignal::Consumed {
                    source_turn_id: source_turn_id.to_string(),
                });
                self.apply_adaptive_terminal_signal(
                    source_turn_id,
                    crate::adaptive_policy::AdaptiveOutcomeSignal::Failure(
                        crate::adaptive_policy::AdaptiveFailureKind::Capability,
                    ),
                );
                return true;
            }
            self.save_adaptive_effort_for_current_thread();
            return false;
        }

        match signal_kind {
            AdaptiveRuntimeSignalKind::Capability => self.apply_adaptive_terminal_signal(
                source_turn_id,
                crate::adaptive_policy::AdaptiveOutcomeSignal::Failure(
                    crate::adaptive_policy::AdaptiveFailureKind::Capability,
                ),
            ),
            AdaptiveRuntimeSignalKind::ReadyForValidation => {
                self.begin_adaptive_validation(source_turn_id)
            }
            AdaptiveRuntimeSignalKind::RepairRequired => self
                .latch_workflow_terminal(source_turn_id, AdaptiveWorkflowTerminal::RepairRequired),
            AdaptiveRuntimeSignalKind::ReadyForOwnerQa => self
                .latch_workflow_terminal(source_turn_id, AdaptiveWorkflowTerminal::ReadyForOwnerQa),
        }
        true
    }

    fn native_failure_pressure_available(&self, source_turn_id: &str) -> bool {
        self.thread_id.is_some_and(|thread_id| {
            self.adaptive_effort.enabled
                && !self.adaptive_effort.paused_by_user
                && self.adaptive_effort.workflow_terminal.is_none()
                && self
                    .adaptive_effort
                    .evidence_registry
                    .failure_pressure_for_turn(thread_id, source_turn_id)
                    >= ADAPTIVE_FAILURE_PRESSURE_THRESHOLD
        })
    }

    fn valid_evidence_refs(
        &self,
        evidence_refs: &[String],
        expected: AdaptiveEvidenceOutcome,
    ) -> bool {
        let Some(thread_id) = self.thread_id else {
            return false;
        };
        !evidence_refs.is_empty()
            && evidence_refs.iter().all(|evidence_id| {
                self.adaptive_effort
                    .evidence_registry
                    .validate_outcome(evidence_id, thread_id, expected)
                    .is_ok()
            })
    }

    fn latch_workflow_terminal(
        &mut self,
        source_turn_id: &str,
        terminal: AdaptiveWorkflowTerminal,
    ) {
        self.adaptive_effort.workflow_terminal = Some(terminal);
        self.adaptive_effort.last_processed_terminal_turn_id = Some(source_turn_id.to_string());
        self.adaptive_effort.pending_attempt = None;
        self.adaptive_effort.successor_admission = None;
        self.save_adaptive_effort_for_current_thread();
        if terminal == AdaptiveWorkflowTerminal::ReadyForOwnerQa
            && let Some(thread_id) = self.thread_id
        {
            self.app_event_tx.send(AppEvent::PersistWorkflowState {
                thread_id,
                operation:
                    codex_app_server_protocol::ThreadWorkflowStateOperation::SetReadyForOwnerQa,
                source_turn_id: Some(source_turn_id.to_string()),
            });
        }
    }
}
