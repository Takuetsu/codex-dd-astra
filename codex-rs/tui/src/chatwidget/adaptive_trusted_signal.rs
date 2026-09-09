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
        self.adaptive_effort.unfinished_turn_pressure = 0;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptive_policy::AdaptiveEffort;
    use crate::adaptive_policy::AdaptiveFamily;
    use crate::chatwidget::tests::make_chatwidget_manual;
    use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;
    use codex_app_server_protocol::AdaptiveRuntimeSignalNotification;
    use codex_app_server_protocol::CommandExecutionSource;
    use codex_app_server_protocol::CommandExecutionStatus;
    use codex_app_server_protocol::ItemCompletedNotification;
    use codex_app_server_protocol::ThreadItem;
    use codex_utils_absolute_path::AbsolutePathBuf;

    fn command_evidence(
        thread_id: ThreadId,
        turn_id: &str,
        item_id: &str,
        status: CommandExecutionStatus,
        exit_code: i32,
    ) -> ItemCompletedNotification {
        ItemCompletedNotification {
            item: ThreadItem::CommandExecution {
                id: item_id.to_string(),
                plugin_id: None,
                script_path: None,
                command: "review command".to_string(),
                cwd: AbsolutePathBuf::from_absolute_path(
                    std::env::current_dir().expect("cwd"),
                )
                .expect("absolute cwd")
                .into(),
                process_id: None,
                source: CommandExecutionSource::Agent,
                status,
                command_actions: Vec::new(),
                aggregated_output: None,
                exit_code: Some(exit_code),
                duration_ms: Some(1),
            },
            thread_id: thread_id.to_string(),
            turn_id: turn_id.to_string(),
            completed_at_ms: 1,
        }
    }

    #[tokio::test]
    async fn green_reviewer_terminal_overrides_armed_failure_pressure_and_admits_no_successor() {
        let (mut chat, _rx, _op_rx) = make_chatwidget_manual(None).await;
        let thread_id = ThreadId::new();
        let turn_id = "green-review-after-pressure";
        chat.thread_id = Some(thread_id);
        chat.dispatch_adaptive_command("astra");
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Validation;
        chat.adaptive_effort.worker_context.authorized_scope =
            Some("cycle-vengeance/A-B1-independent-review".to_string());
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "review-failure-1",
            CommandExecutionStatus::Failed,
            1,
        ));
        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "review-failure-2",
            CommandExecutionStatus::Failed,
            1,
        ));
        assert!(matches!(
            chat.adaptive_effort.pending_signal,
            Some(AdaptivePendingSignal::Pending(ref signal))
                if signal.signal_kind == AdaptiveRuntimeSignalKind::Capability
        ));

        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "review-green-proof",
            CommandExecutionStatus::Completed,
            0,
        ));
        chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
            thread_id: thread_id.to_string(),
            signal: AdaptiveRuntimeSignalEnvelope {
                source_turn_id: turn_id.to_string(),
                signal_kind: AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
                evidence_refs: vec!["review-green-proof".to_string()],
                diagnostic_note: Some("PASS - READY FOR REPOSITORY HANDOFF".to_string()),
            },
        });

        chat.turn_lifecycle.agent_turn_running = false;
        assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
        assert_eq!(
            chat.adaptive_effort.workflow_terminal,
            Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa)
        );
        assert_eq!(chat.adaptive_effort.current_family, Some(AdaptiveFamily::Luna));
        assert_eq!(chat.adaptive_effort.current_effort, Some(AdaptiveEffort::Low));
        assert_eq!(chat.adaptive_effort.attempt_number, 1);
        assert_eq!(chat.adaptive_effort.pending_attempt, None);
        assert_eq!(chat.adaptive_effort.successor_admission, None);
        assert!(!chat.maybe_submit_adaptive_successor());
    }
}
