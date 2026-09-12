//! Terminal-bound validation and consumption of trusted adaptive signals.

use super::*;
use crate::adaptive_evidence::ADAPTIVE_FAILURE_PRESSURE_THRESHOLD;
use crate::adaptive_evidence::AUTO_FAILURE_PRESSURE_DIAGNOSTIC;
use crate::adaptive_evidence::AdaptiveEvidenceOutcome;
use crate::adaptive_policy::AdaptiveRoute;
use crate::adaptive_policy::AdaptiveTransition;
use crate::adaptive_policy::next_route;
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
                self.apply_capability_signal_with_report(
                    source_turn_id,
                    AUTO_FAILURE_PRESSURE_DIAGNOSTIC,
                );
                return true;
            }
            _ => return false,
        };

        let capability_diagnostic = envelope
            .diagnostic_note
            .as_deref()
            .map(str::trim)
            .filter(|note| !note.is_empty())
            .map(str::to_string);
        let accepted = match envelope.signal_kind {
            AdaptiveRuntimeSignalKind::Capability => {
                envelope.evidence_refs.is_empty() && capability_diagnostic.is_some()
            }
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
                self.apply_capability_signal_with_report(
                    source_turn_id,
                    AUTO_FAILURE_PRESSURE_DIAGNOSTIC,
                );
                return true;
            }
            self.save_adaptive_effort_for_current_thread();
            return false;
        }

        match signal_kind {
            AdaptiveRuntimeSignalKind::Capability => self.apply_capability_signal_with_report(
                source_turn_id,
                capability_diagnostic
                    .as_deref()
                    .expect("accepted capability signal has a diagnostic report"),
            ),
            // READY_FOR_VALIDATION is a hard handoff boundary for the current
            // Implementation/Repair Worker. Validation must run in a fresh,
            // independently bound Worker thread; never mutate this Worker's
            // authority or admit another same-thread adaptive successor.
            AdaptiveRuntimeSignalKind::ReadyForValidation => self.latch_workflow_terminal(
                source_turn_id,
                AdaptiveWorkflowTerminal::ReadyForValidation,
            ),
            AdaptiveRuntimeSignalKind::RepairRequired => self
                .latch_workflow_terminal(source_turn_id, AdaptiveWorkflowTerminal::RepairRequired),
            AdaptiveRuntimeSignalKind::ReadyForOwnerQa => self
                .latch_workflow_terminal(source_turn_id, AdaptiveWorkflowTerminal::ReadyForOwnerQa),
        }
        true
    }

    fn apply_capability_signal_with_report(&mut self, source_turn_id: &str, diagnostic: &str) {
        if diagnostic == AUTO_FAILURE_PRESSURE_DIAGNOSTIC
            && let Some((from, to)) = self.model_escalation_boundary()
        {
            self.adaptive_effort.unfinished_turn_pressure = 0;
            self.adaptive_effort.last_processed_terminal_turn_id = Some(source_turn_id.to_string());
            self.adaptive_effort.pending_attempt = None;
            self.adaptive_effort.successor_admission = None;
            self.adaptive_effort.last_outcome = None;
            self.adaptive_effort.last_failure_kind = None;
            self.add_info_message(
                format!(
                    "Adaptive model escalation withheld\n  From: {} {:?}\n  Proposed: {} {:?}\n  Native failure pressure: {}/{}\n  Required: a fresh trusted Capability report explaining why stronger model capability is required. Two failed tools alone can raise effort inside one model family, but cannot authorize a model-family jump.",
                    from.family.model(),
                    from.effort,
                    to.family.model(),
                    to.effort,
                    ADAPTIVE_FAILURE_PRESSURE_THRESHOLD,
                    ADAPTIVE_FAILURE_PRESSURE_THRESHOLD,
                ),
                None,
            );
            self.save_adaptive_effort_for_current_thread();
            return;
        }

        if let Some(report) = self.model_escalation_report(diagnostic) {
            self.add_info_message(report, None);
        }
        self.apply_adaptive_terminal_signal(
            source_turn_id,
            crate::adaptive_policy::AdaptiveOutcomeSignal::Failure(
                crate::adaptive_policy::AdaptiveFailureKind::Capability,
            ),
        );
    }

    fn model_escalation_boundary(&self) -> Option<(AdaptiveRoute, AdaptiveRoute)> {
        let state = &self.adaptive_effort;
        let (Some(starting_family), Some(current_family), Some(current_effort)) = (
            state.starting_family,
            state.current_family,
            state.current_effort,
        ) else {
            return None;
        };
        let from = AdaptiveRoute {
            family: current_family,
            effort: current_effort,
        };
        let AdaptiveTransition::EscalateModel(to) = next_route(starting_family, from) else {
            return None;
        };
        Some((from, to))
    }

    fn model_escalation_report(&self, diagnostic: &str) -> Option<String> {
        let (from, to) = self.model_escalation_boundary()?;
        Some(format!(
            "Adaptive model escalation report\n  From: {} {:?}\n  To: {} {:?}\n  Why: {}",
            from.family.model(),
            from.effort,
            to.family.model(),
            to.effort,
            diagnostic.trim(),
        ))
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
    use crate::chatwidget::tests::make_chatwidget_manual_with_sender;
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
                cwd: AbsolutePathBuf::from_absolute_path(std::env::current_dir().expect("cwd"))
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
    async fn capability_signal_requires_nonblank_diagnostic_report() {
        let (mut chat, _sender, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
        let thread_id = ThreadId::new();
        let turn_id = "capability-without-report";
        chat.thread_id = Some(thread_id);
        chat.dispatch_adaptive_command("astra");
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
        chat.adaptive_effort.worker_context.authorized_scope = Some("bounded repair".to_string());
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

        chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
            thread_id: thread_id.to_string(),
            signal: AdaptiveRuntimeSignalEnvelope {
                source_turn_id: turn_id.to_string(),
                signal_kind: AdaptiveRuntimeSignalKind::Capability,
                evidence_refs: Vec::new(),
                diagnostic_note: Some("   ".to_string()),
            },
        });

        chat.turn_lifecycle.agent_turn_running = false;
        assert!(!chat.consume_adaptive_signal_at_terminal(turn_id));
        assert_eq!(
            chat.adaptive_effort.current_family,
            Some(AdaptiveFamily::Luna)
        );
        assert_eq!(
            chat.adaptive_effort.current_effort,
            Some(AdaptiveEffort::Low)
        );
        assert_eq!(chat.adaptive_effort.attempt_number, 1);
        assert!(matches!(
            chat.adaptive_effort.pending_signal,
            Some(AdaptivePendingSignal::Cancelled { .. })
        ));
    }

    #[tokio::test]
    async fn diagnostic_capability_signal_can_cross_model_family_boundary() {
        let (mut chat, _sender, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
        let thread_id = ThreadId::new();
        let turn_id = "luna-high-capability-report";
        chat.thread_id = Some(thread_id);
        chat.dispatch_adaptive_command("astra");
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
        chat.adaptive_effort.worker_context.authorized_scope = Some("bounded repair".to_string());
        chat.adaptive_effort.current_family = Some(AdaptiveFamily::Luna);
        chat.adaptive_effort.current_effort = Some(AdaptiveEffort::High);
        chat.adaptive_effort.attempt_number = 5;
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

        chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
            thread_id: thread_id.to_string(),
            signal: AdaptiveRuntimeSignalEnvelope {
                source_turn_id: turn_id.to_string(),
                signal_kind: AdaptiveRuntimeSignalKind::Capability,
                evidence_refs: Vec::new(),
                diagnostic_note: Some(
                    "Current model repeatedly failed the bounded implementation step.".to_string(),
                ),
            },
        });

        chat.turn_lifecycle.agent_turn_running = false;
        assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
        assert_eq!(
            chat.adaptive_effort.current_family,
            Some(AdaptiveFamily::Terra)
        );
        assert_eq!(
            chat.adaptive_effort.current_effort,
            Some(AdaptiveEffort::Low)
        );
        assert_eq!(chat.adaptive_effort.attempt_number, 6);
        assert_eq!(
            chat.adaptive_effort.last_failure_kind,
            Some(crate::adaptive_policy::AdaptiveFailureKind::Capability)
        );
    }

    #[tokio::test]
    async fn native_failure_pressure_still_escalates_effort_inside_one_family() {
        let (mut chat, _sender, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
        let thread_id = ThreadId::new();
        let turn_id = "luna-low-native-pressure";
        chat.thread_id = Some(thread_id);
        chat.dispatch_adaptive_command("astra");
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
        chat.adaptive_effort.worker_context.authorized_scope = Some("bounded repair".to_string());
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "same-family-failure-1",
            CommandExecutionStatus::Failed,
            1,
        ));
        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "same-family-failure-2",
            CommandExecutionStatus::Failed,
            1,
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
        assert_eq!(chat.adaptive_effort.attempt_number, 2);
        assert_eq!(
            chat.adaptive_effort.last_failure_kind,
            Some(crate::adaptive_policy::AdaptiveFailureKind::Capability)
        );
    }

    #[tokio::test]
    async fn native_failure_pressure_cannot_cross_model_family_boundary_without_report() {
        let (mut chat, _sender, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
        let thread_id = ThreadId::new();
        let turn_id = "luna-high-native-pressure";
        chat.thread_id = Some(thread_id);
        chat.dispatch_adaptive_command("astra");
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
        chat.adaptive_effort.worker_context.authorized_scope = Some("bounded repair".to_string());
        chat.adaptive_effort.current_family = Some(AdaptiveFamily::Luna);
        chat.adaptive_effort.current_effort = Some(AdaptiveEffort::High);
        chat.adaptive_effort.attempt_number = 5;
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "boundary-failure-1",
            CommandExecutionStatus::Failed,
            1,
        ));
        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "boundary-failure-2",
            CommandExecutionStatus::Failed,
            1,
        ));

        chat.turn_lifecycle.agent_turn_running = false;
        assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
        assert_eq!(
            chat.adaptive_effort.current_family,
            Some(AdaptiveFamily::Luna)
        );
        assert_eq!(
            chat.adaptive_effort.current_effort,
            Some(AdaptiveEffort::High)
        );
        assert_eq!(chat.adaptive_effort.attempt_number, 5);
        assert_eq!(
            chat.adaptive_effort
                .last_processed_terminal_turn_id
                .as_deref(),
            Some(turn_id)
        );
        assert_eq!(chat.adaptive_effort.pending_attempt, None);
        assert_eq!(chat.adaptive_effort.successor_admission, None);
        assert_eq!(chat.adaptive_effort.last_failure_kind, None);
        assert!(!chat.maybe_submit_adaptive_successor());
    }

    #[tokio::test]
    async fn explicit_capability_report_supersedes_synthetic_pressure_and_crosses_boundary() {
        let (mut chat, _sender, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
        let thread_id = ThreadId::new();
        let turn_id = "luna-high-pressure-then-explicit-report";
        let report = "Luna High cannot resolve the bounded implementation constraint; stronger model capability is required.";
        chat.thread_id = Some(thread_id);
        chat.dispatch_adaptive_command("astra");
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
        chat.adaptive_effort.worker_context.authorized_scope = Some("bounded repair".to_string());
        chat.adaptive_effort.current_family = Some(AdaptiveFamily::Luna);
        chat.adaptive_effort.current_effort = Some(AdaptiveEffort::High);
        chat.adaptive_effort.attempt_number = 5;
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "upgrade-failure-1",
            CommandExecutionStatus::Failed,
            1,
        ));
        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "upgrade-failure-2",
            CommandExecutionStatus::Failed,
            1,
        ));
        assert!(matches!(
            chat.adaptive_effort.pending_signal,
            Some(AdaptivePendingSignal::Pending(ref signal))
                if signal.diagnostic_note.as_deref() == Some(AUTO_FAILURE_PRESSURE_DIAGNOSTIC)
        ));

        chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
            thread_id: thread_id.to_string(),
            signal: AdaptiveRuntimeSignalEnvelope {
                source_turn_id: turn_id.to_string(),
                signal_kind: AdaptiveRuntimeSignalKind::Capability,
                evidence_refs: Vec::new(),
                diagnostic_note: Some(report.to_string()),
            },
        });
        assert!(matches!(
            chat.adaptive_effort.pending_signal,
            Some(AdaptivePendingSignal::Pending(ref signal))
                if signal.diagnostic_note.as_deref() == Some(report)
        ));

        chat.turn_lifecycle.agent_turn_running = false;
        assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
        assert_eq!(
            chat.adaptive_effort.current_family,
            Some(AdaptiveFamily::Terra)
        );
        assert_eq!(
            chat.adaptive_effort.current_effort,
            Some(AdaptiveEffort::Low)
        );
        assert_eq!(chat.adaptive_effort.attempt_number, 6);
        assert_eq!(
            chat.adaptive_effort.last_failure_kind,
            Some(crate::adaptive_policy::AdaptiveFailureKind::Capability)
        );
    }

    #[tokio::test]
    async fn implementation_ready_for_validation_is_hard_handoff_and_admits_no_successor() {
        let (mut chat, _sender, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
        let thread_id = ThreadId::new();
        let turn_id = "implementation-ready-for-independent-validation";
        chat.thread_id = Some(thread_id);
        chat.dispatch_adaptive_command("astra");
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
        chat.adaptive_effort.worker_context.authorized_scope =
            Some("breakwater/Z-A0.46A-buildstamp-repair".to_string());
        // Mirror the observed runaway at the top of the ladder. A valid handoff
        // must preserve the completed Worker's route/attempt rather than reset
        // the same thread to a Validation Worker at Luna Low.
        chat.adaptive_effort.current_family = Some(AdaptiveFamily::Astra);
        chat.adaptive_effort.current_effort = Some(AdaptiveEffort::Max);
        chat.adaptive_effort.attempt_number = 28;
        chat.adaptive_effort.unfinished_turn_pressure = 1;
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

        chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
            thread_id: thread_id.to_string(),
            signal: AdaptiveRuntimeSignalEnvelope {
                source_turn_id: turn_id.to_string(),
                signal_kind: AdaptiveRuntimeSignalKind::ReadyForValidation,
                evidence_refs: Vec::new(),
                diagnostic_note: Some("READY_FOR_VALIDATION".to_string()),
            },
        });

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
        assert_eq!(
            chat.adaptive_effort
                .worker_context
                .authorized_scope
                .as_deref(),
            Some("breakwater/Z-A0.46A-buildstamp-repair")
        );
        assert_eq!(
            chat.adaptive_effort.current_family,
            Some(AdaptiveFamily::Astra)
        );
        assert_eq!(
            chat.adaptive_effort.current_effort,
            Some(AdaptiveEffort::Max)
        );
        assert_eq!(chat.adaptive_effort.attempt_number, 28);
        assert_eq!(chat.adaptive_effort.unfinished_turn_pressure, 0);
        assert_eq!(chat.adaptive_effort.pending_attempt, None);
        assert_eq!(chat.adaptive_effort.successor_admission, None);
        assert!(!chat.maybe_submit_adaptive_successor());
    }

    #[tokio::test]
    async fn green_reviewer_terminal_overrides_armed_failure_pressure_and_admits_no_successor() {
        let (mut chat, _sender, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
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
        assert_eq!(
            chat.adaptive_effort.current_family,
            Some(AdaptiveFamily::Luna)
        );
        assert_eq!(
            chat.adaptive_effort.current_effort,
            Some(AdaptiveEffort::Low)
        );
        assert_eq!(chat.adaptive_effort.attempt_number, 1);
        assert_eq!(chat.adaptive_effort.pending_attempt, None);
        assert_eq!(chat.adaptive_effort.successor_admission, None);
        assert!(!chat.maybe_submit_adaptive_successor());
    }

    #[tokio::test]
    async fn repair_required_terminal_overrides_armed_failure_pressure_and_admits_no_successor() {
        let (mut chat, _sender, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
        let thread_id = ThreadId::new();
        let turn_id = "failed-validation-repair-required";
        chat.thread_id = Some(thread_id);
        chat.dispatch_adaptive_command("astra");
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Validation;
        chat.adaptive_effort.worker_context.authorized_scope =
            Some("breakwater/Z-A0.46A-independent-validation".to_string());
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "validation-failure-1",
            CommandExecutionStatus::Failed,
            1,
        ));
        chat.register_adaptive_evidence(&command_evidence(
            thread_id,
            turn_id,
            "validation-failure-2",
            CommandExecutionStatus::Failed,
            1,
        ));
        assert!(matches!(
            chat.adaptive_effort.pending_signal,
            Some(AdaptivePendingSignal::Pending(ref signal))
                if signal.signal_kind == AdaptiveRuntimeSignalKind::Capability
        ));

        chat.handle_adaptive_runtime_signal(AdaptiveRuntimeSignalNotification {
            thread_id: thread_id.to_string(),
            signal: AdaptiveRuntimeSignalEnvelope {
                source_turn_id: turn_id.to_string(),
                signal_kind: AdaptiveRuntimeSignalKind::RepairRequired,
                evidence_refs: vec![
                    "validation-failure-1".to_string(),
                    "validation-failure-2".to_string(),
                ],
                diagnostic_note: Some("FAIL Z-A0.46A REPAIR_REQUIRED".to_string()),
            },
        });

        chat.turn_lifecycle.agent_turn_running = false;
        assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
        assert_eq!(
            chat.adaptive_effort.workflow_terminal,
            Some(AdaptiveWorkflowTerminal::RepairRequired)
        );
        assert_eq!(
            chat.adaptive_effort.current_family,
            Some(AdaptiveFamily::Luna)
        );
        assert_eq!(
            chat.adaptive_effort.current_effort,
            Some(AdaptiveEffort::Low)
        );
        assert_eq!(chat.adaptive_effort.attempt_number, 1);
        assert_eq!(chat.adaptive_effort.pending_attempt, None);
        assert_eq!(chat.adaptive_effort.successor_admission, None);
        assert!(!chat.maybe_submit_adaptive_successor());
    }
}
