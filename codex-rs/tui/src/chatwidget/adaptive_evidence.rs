//! Live native tool-completion ingestion for the per-thread adaptive evidence registry.

use super::*;
use crate::adaptive_evidence::ADAPTIVE_FAILURE_PRESSURE_THRESHOLD;
use crate::adaptive_evidence::AUTO_FAILURE_PRESSURE_DIAGNOSTIC;
use crate::adaptive_evidence::CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME;
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

        let Some(mut record) = record_from_item_completion(notification) else {
            return;
        };
        if record.thread_id != thread_id {
            return;
        }
        record.bind_worker_identity(
            &self.adaptive_effort.worker_context,
            self.adaptive_effort.assignment_preflight.as_ref(),
        );
        let independent_receipt = if record.tool_name.as_deref()
            == Some(CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME)
        {
            record.native_source_provenance.as_ref().map(|native| {
                (
                    record.evidence_id.clone(),
                    native.run_id.clone(),
                    native.completed_at_ms,
                )
            })
        } else {
            None
        };
        self.adaptive_effort.evidence_registry.register(record);
        // Only a real fresh-Worker handoff has the parent identity, the
        // original owner-gate ledger and the source receipts for rechecking.
        if let (Some(origin_thread_id), Some((evidence_id, run_id, completed_at_ms))) = (
            self.adaptive_effort.review_origin_thread_id,
            independent_receipt,
        ) && let Some(ledger) = self.adaptive_effort.gate_ledger.as_mut()
        {
            let _ = ledger.observe_independent_review_pass(
                &self.adaptive_effort.evidence_registry,
                &evidence_id,
                thread_id,
                origin_thread_id,
                &notification.turn_id,
                &run_id,
                completed_at_ms,
            );
        }

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
    let failure_fingerprint = required("failure_fingerprint");
    if failure_fingerprint.is_some() && result != "fail" {
        return None;
    }

    Some(AdaptiveValidationStatus {
        profile,
        result,
        run_id,
        branch: required("branch"),
        head_sha,
        failed_stage: required("failed_stage"),
        failure_fingerprint,
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

    #[tokio::test]
    async fn codexdd_046_3c1b2c_native_completion_handoff_rechecks_source_and_review() {
        use crate::adaptive_gate_prerequisites::AdaptiveGateLedger;
        use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;
        use crate::adaptive_worker::AdaptiveTaskKind;
        use crate::adaptive_worker::AdaptiveWorkerContext;
        use crate::adaptive_worker::AdaptiveWorkerPreflight;
        use crate::adaptive_worker::AdaptiveWorkerRole;
        use crate::adaptive_worker::AdaptiveWorkflowTerminal;
        use crate::adaptive_worker::NewWorkerBinding;
        use crate::chatwidget::tests::make_chatwidget_manual_with_sender;
        use codex_app_server_protocol::DynamicToolCallStatus;
        use codex_protocol::ThreadId;

        let (mut chat, _sender, _rx, _op_rx) = make_chatwidget_manual_with_sender().await;
        let source = tempfile::tempdir().expect("isolated source");
        let scope = "isolated independent review";
        let sha = "a".repeat(40);
        let implementation = ThreadId::new();
        let validation = ThreadId::new();
        chat.thread_id = Some(implementation);
        chat.dispatch_adaptive_command("astra");
        chat.adaptive_effort.worker_context = AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Implementation,
            authorized_scope: Some(scope.to_string()),
        };
        chat.adaptive_effort.worker_assignment_locked = true;
        chat.adaptive_effort.assignment_preflight = Some(AdaptiveWorkerPreflight {
            task_kind: AdaptiveTaskKind::SourceChange,
            source_root: source.path().display().to_string(),
            source_sha: sha.clone(),
        });
        chat.adaptive_effort.gate_ledger = Some(
            AdaptiveGateLedger::new(
                scope,
                source.path(),
                &sha,
                vec![
                    AdaptiveRequiredGate::SourceProfile("work_packet".to_string()),
                    AdaptiveRequiredGate::IndependentValidation,
                ],
            )
            .expect("exact owner requirements"),
        );

        let completed = |thread: ThreadId,
                         turn_id: &str,
                         id: &str,
                         tool: &str,
                         profile: &str,
                         run_id: &str|
         -> ItemCompletedNotification {
            ItemCompletedNotification {
                item: ThreadItem::DynamicToolCall {
                    id: id.to_string(),
                    namespace: None,
                    tool: tool.to_string(),
                    arguments: serde_json::json!({
                        "profile": profile,
                        "result": "pass",
                        "run_id": run_id,
                        "head_sha": sha,
                        "source_root": source.path().display().to_string(),
                    }),
                    status: DynamicToolCallStatus::Completed,
                    content_items: None,
                    success: Some(true),
                    duration_ms: Some(1),
                },
                thread_id: thread.to_string(),
                turn_id: turn_id.to_string(),
                completed_at_ms: 2_000,
            }
        };
        let source_receipt = completed(
            implementation,
            "source-turn",
            "native-source",
            CODEXDD_VALIDATION_TOOL_NAME,
            "work_packet",
            "source-run",
        );
        chat.register_adaptive_evidence(&source_receipt);
        chat.adaptive_effort
            .gate_ledger
            .as_mut()
            .expect("owner ledger")
            .observe_source_pass(
                &chat.adaptive_effort.evidence_registry,
                "work_packet",
                "native-source",
                implementation,
                "source-turn",
                "source-run",
                1_999,
            )
            .expect("source receipt matches exact Implementation Worker");
        chat.adaptive_effort.workflow_terminal =
            Some(AdaptiveWorkflowTerminal::ReadyForValidation);
        let binding = NewWorkerBinding {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: scope.to_string(),
        };
        let fresh = chat.fresh_adaptive_effort_for_new_worker(Some(&binding));
        assert_eq!(fresh.review_origin_thread_id, Some(implementation));
        assert!(fresh.gate_ledger.is_some());
        assert!(fresh.evidence_registry.len() > 0);

        chat.thread_id = Some(validation);
        chat.adaptive_effort = fresh;
        let review_receipt = completed(
            validation,
            "review-turn",
            "native-review",
            CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME,
            "independent_review",
            "review-run",
        );
        chat.register_adaptive_evidence(&review_receipt);
        let ledger = chat.adaptive_effort.gate_ledger.as_ref().expect("review ledger");
        assert!(
            ledger
                .outstanding(&chat.adaptive_effort.evidence_registry)
                .is_empty(),
            "gates are cleared only by matching source and separately bound Core receipts"
        );
        assert!(ledger.independent_review_referenced_by(
            &chat.adaptive_effort.evidence_registry,
            &["native-review".to_string()],
        ));
        assert!(!ledger.independent_review_referenced_by(
            &chat.adaptive_effort.evidence_registry,
            &["generic-command".to_string()],
        ));
        assert_eq!(chat.adaptive_effort.workflow_terminal, None);
        // Source + native review receipts are accepted by the TUI observer
        // but never auto-latch READY without a separate trusted signal.
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
                    "failure_fingerprint": "v1:work_packet:core-adaptive-tests",
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
                failure_fingerprint: Some("v1:work_packet:core-adaptive-tests".to_string()),
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
