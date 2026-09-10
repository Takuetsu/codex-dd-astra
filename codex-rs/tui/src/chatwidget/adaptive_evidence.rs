//! Live native tool-completion ingestion for the per-thread adaptive evidence registry.

use super::*;
use crate::adaptive_evidence::ADAPTIVE_FAILURE_PRESSURE_THRESHOLD;
use crate::adaptive_evidence::AUTO_FAILURE_PRESSURE_DIAGNOSTIC;
use crate::adaptive_evidence::record_from_item_completion;
use crate::chatwidget::adaptive_effort::AdaptivePendingSignal;
use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::ThreadItem;
use codex_app_server_protocol::UserInput;
use codex_protocol::protocol::AdaptiveRuntimeSignalKind;

impl ChatWidget {
    pub(super) fn register_adaptive_evidence(&mut self, notification: &ItemCompletedNotification) {
        let Some(thread_id) = self.thread_id else {
            return;
        };
        if notification.thread_id != thread_id.to_string() {
            return;
        }

        self.observe_adaptive_worker_assignment(notification);

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

    fn observe_adaptive_worker_assignment(&mut self, notification: &ItemCompletedNotification) {
        if !self.adaptive_effort.enabled || self.adaptive_effort.worker_assignment_locked {
            return;
        }
        let ThreadItem::UserMessage { content, .. } = &notification.item else {
            return;
        };

        let assignment_text = content
            .iter()
            .filter_map(|input| match input {
                UserInput::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        let result = self
            .adaptive_effort
            .observe_worker_assignment_text(&assignment_text);
        self.save_adaptive_effort_for_current_thread();

        if let Err(err) = result {
            self.add_error_message(format!(
                "Adaptive Worker assignment rejected: {err}. This thread is locked unbound; start a fresh Worker thread with a valid [adaptive_worker] header."
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_message_notification(text: &str) -> ItemCompletedNotification {
        ItemCompletedNotification {
            item: ThreadItem::UserMessage {
                id: "user-message".to_string(),
                client_id: None,
                content: vec![UserInput::Text {
                    text: text.to_string(),
                    text_elements: Vec::new(),
                }],
            },
            thread_id: "thread".to_string(),
            turn_id: "turn".to_string(),
            completed_at_ms: 1,
        }
    }

    #[test]
    fn user_assignment_item_preserves_typed_header_text_for_binding() {
        let notification = user_message_notification(
            "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2 review\"\n\nReview the candidate.",
        );
        let ThreadItem::UserMessage { content, .. } = notification.item else {
            panic!("expected user message");
        };
        let text = content
            .iter()
            .filter_map(|input| match input {
                UserInput::Text { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(text.starts_with("[adaptive_worker]"));
        assert!(text.contains("authorized_scope = \"R2 review\""));
    }
}
