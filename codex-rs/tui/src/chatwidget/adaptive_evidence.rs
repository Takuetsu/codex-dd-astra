//! Live native tool-completion ingestion for the per-thread adaptive evidence registry.

use super::*;
use crate::adaptive_evidence::ADAPTIVE_FAILURE_PRESSURE_THRESHOLD;
use crate::adaptive_evidence::AUTO_FAILURE_PRESSURE_DIAGNOSTIC;
use crate::adaptive_evidence::record_from_item_completion;
use crate::chatwidget::adaptive_effort::AdaptivePendingSignal;
use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_protocol::protocol::AdaptiveRuntimeSignalKind;

impl ChatWidget {
    pub(super) fn register_adaptive_evidence(&mut self, notification: &ItemCompletedNotification) {
        let Some(thread_id) = self.thread_id else {
            return;
        };
        let Some(record) = record_from_item_completion(notification) else {
            return;
        };
        if record.thread_id != thread_id {
            return;
        }
        self.adaptive_effort.evidence_registry.register(record);

        let source_turn_id = notification.turn_id.as_str();
        let pressure = self
            .adaptive_effort
            .evidence_registry
            .failure_pressure_for_turn(thread_id, source_turn_id);
        let current_turn_matches = self.turn_lifecycle.agent_turn_running
            && self.turn_lifecycle.last_turn_id.as_deref() == Some(source_turn_id);
        if self.adaptive_effort.enabled
            && !self.adaptive_effort.paused_by_user
            && self.adaptive_effort.workflow_terminal.is_none()
            && current_turn_matches
            && pressure >= ADAPTIVE_FAILURE_PRESSURE_THRESHOLD
            && self.adaptive_effort.pending_signal.is_none()
        {
            self.adaptive_effort.pending_signal = Some(AdaptivePendingSignal::Pending(
                AdaptiveRuntimeSignalEnvelope {
                    source_turn_id: source_turn_id.to_string(),
                    signal_kind: AdaptiveRuntimeSignalKind::Capability,
                    evidence_refs: Vec::new(),
                    diagnostic_note: Some(AUTO_FAILURE_PRESSURE_DIAGNOSTIC.to_string()),
                },
            ));
        }

        self.save_adaptive_effort_for_current_thread();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptive_policy::AdaptiveEffort;
    use crate::adaptive_policy::AdaptiveFamily;
    use crate::adaptive_worker::AdaptiveWorkerRole;
    use crate::adaptive_worker::AdaptiveWorkflowTerminal;
    use crate::chatwidget::tests::make_chatwidget_manual;
    use codex_app_server_protocol::AdaptiveRuntimeSignalNotification;
    use codex_app_server_protocol::CommandExecutionSource;
    use codex_app_server_protocol::CommandExecutionStatus;
    use codex_app_server_protocol::ThreadItem;
    use codex_protocol::ThreadId;
    use codex_utils_absolute_path::AbsolutePathBuf;

    fn failed_command(
        thread_id: ThreadId,
        turn_id: &str,
        item_id: &str,
    ) -> ItemCompletedNotification {
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
        assert_eq!(chat.adaptive_effort.current_family, Some(AdaptiveFamily::Luna));
        assert_eq!(chat.adaptive_effort.current_effort, Some(AdaptiveEffort::Low));
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(turn_id.to_string());

        chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-1"));
        assert!(chat.adaptive_effort.pending_signal.is_none());
        assert!(chat.adaptive_effort_status_text().contains("Failure pressure: 1/2"));

        chat.register_adaptive_evidence(&failed_command(thread_id, turn_id, "failure-2"));
        assert!(chat.adaptive_effort_status_text().contains("Failure pressure: 2/2"));
        assert!(matches!(
            chat.adaptive_effort.pending_signal,
            Some(AdaptivePendingSignal::Pending(ref signal))
                if signal.signal_kind == AdaptiveRuntimeSignalKind::Capability
                    && signal.diagnostic_note.as_deref()
                        == Some(AUTO_FAILURE_PRESSURE_DIAGNOSTIC)
        ));

        chat.turn_lifecycle.agent_turn_running = false;
        assert!(chat.consume_adaptive_signal_at_terminal(turn_id));
        assert_eq!(chat.adaptive_effort.current_family, Some(AdaptiveFamily::Luna));
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
        assert_eq!(chat.adaptive_effort.current_family, Some(AdaptiveFamily::Luna));
        assert_eq!(chat.adaptive_effort.current_effort, Some(AdaptiveEffort::Low));
    }
}
