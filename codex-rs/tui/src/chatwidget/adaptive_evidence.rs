//! Live native tool-completion ingestion for the per-thread adaptive evidence registry.

use super::*;
use crate::adaptive_evidence::ADAPTIVE_FAILURE_PRESSURE_THRESHOLD;
use crate::adaptive_evidence::AUTO_FAILURE_PRESSURE_DIAGNOSTIC;
use crate::adaptive_evidence::CODEXDD_VALIDATION_TOOL_NAME;
use crate::adaptive_evidence::record_from_item_completion;
use crate::chatwidget::adaptive_effort::AdaptivePendingSignal;
use crate::chatwidget::adaptive_effort::AdaptiveValidationStatus;
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
        self.observe_codexdd_validation_status(notification);

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

    fn observe_codexdd_validation_status(&mut self, notification: &ItemCompletedNotification) {
        if let Some(status) = validation_status_from_item_completion(notification) {
            self.adaptive_effort.validation_status = Some(status);
        }
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

fn validation_status_from_item_completion(
    notification: &ItemCompletedNotification,
) -> Option<AdaptiveValidationStatus> {
    let ThreadItem::DynamicToolCall {
        namespace,
        tool,
        arguments,
        status,
        ..
    } = &notification.item
    else {
        return None;
    };
    if namespace.is_some()
        || tool != CODEXDD_VALIDATION_TOOL_NAME
        || *status != codex_app_server_protocol::DynamicToolCallStatus::Completed
    {
        return None;
    }

    let required = |key| {
        arguments
            .get(key)
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(str::to_string)
    };
    let (Some(profile), Some(result), Some(run_id), Some(head_sha), Some(log_path)) = (
        required("profile"),
        required("result"),
        required("run_id"),
        required("head_sha"),
        required("log_path"),
    ) else {
        return None;
    };
    if !matches!(profile.as_str(), "targeted" | "work_packet" | "release")
        || !matches!(result.as_str(), "pass" | "fail" | "error")
    {
        return None;
    }

    Some(AdaptiveValidationStatus {
        profile,
        result,
        run_id,
        branch: required("branch"),
        head_sha,
        failed_stage: required("failed_stage"),
        log_path,
    })
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
    fn native_validation_receipt_extracts_operator_status() {
        let notification = ItemCompletedNotification {
            item: ThreadItem::DynamicToolCall {
                id: "validation-call".to_string(),
                namespace: None,
                tool: CODEXDD_VALIDATION_TOOL_NAME.to_string(),
                arguments: serde_json::json!({
                    "profile": "work_packet",
                    "result": "fail",
                    "run_id": "run-42",
                    "branch": "dd/status-evidence",
                    "head_sha": "0123456789abcdef",
                    "failed_stage": "core-adaptive-tests",
                    "log_path": "C:\\codexdd\\validation\\run-42\\validation.log",
                }),
                status: codex_app_server_protocol::DynamicToolCallStatus::Completed,
                content_items: None,
                success: Some(false),
                duration_ms: Some(1),
            },
            thread_id: "thread".to_string(),
            turn_id: "turn".to_string(),
            completed_at_ms: 1,
        };

        assert_eq!(
            validation_status_from_item_completion(&notification),
            Some(AdaptiveValidationStatus {
                profile: "work_packet".to_string(),
                result: "fail".to_string(),
                run_id: "run-42".to_string(),
                branch: Some("dd/status-evidence".to_string()),
                head_sha: "0123456789abcdef".to_string(),
                failed_stage: Some("core-adaptive-tests".to_string()),
                log_path: "C:\\codexdd\\validation\\run-42\\validation.log".to_string(),
            })
        );
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
