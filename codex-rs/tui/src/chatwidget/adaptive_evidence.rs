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
        // Native source PASS must enter the owner-issued ledger from the
        // actual app-server item/completed event, not from model text or a
        // hand-edited test fixture. The gate resolver rechecks it on READY.
        let source_receipt = match (
            record.tool_name.as_deref(),
            record.validation_profile.as_deref(),
            record.native_source_provenance.as_ref(),
        ) {
            (Some(CODEXDD_VALIDATION_TOOL_NAME), Some(profile), Some(native)) => Some((
                record.evidence_id.clone(),
                profile.to_string(),
                native.run_id.clone(),
                native.completed_at_ms,
            )),
            _ => None,
        };
        let independent_receipt =
            if record.tool_name.as_deref() == Some(CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME) {
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
        if let Some((evidence_id, profile, run_id, completed_at_ms)) = source_receipt
            && self.adaptive_effort.worker_context.role
                == crate::adaptive_worker::AdaptiveWorkerRole::Implementation
            && let Some(ledger) = self.adaptive_effort.gate_ledger.as_mut()
        {
            // No cached green status: every source gate remains a reference
            // to the exact native tool ID, source turn, SHA and run.
            let _ = ledger.observe_source_pass(
                &self.adaptive_effort.evidence_registry,
                &profile,
                &evidence_id,
                thread_id,
                &notification.turn_id,
                &run_id,
                completed_at_ms,
            );
        }
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
        assert_eq!(
            chat.adaptive_effort
                .gate_ledger
                .as_ref()
                .expect("owner ledger")
                .outstanding(&chat.adaptive_effort.evidence_registry)
                .len(),
            1,
            "live native source item must automatically satisfy only its exact source gate"
        );
        chat.adaptive_effort.workflow_terminal = Some(AdaptiveWorkflowTerminal::ReadyForValidation);
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
        let ledger = chat
            .adaptive_effort
            .gate_ledger
            .as_ref()
            .expect("review ledger");
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

    /// Exercises the same typed app-server item/completed notification surface
    /// that the real two-Worker Windows process test now proves. In contrast
    /// to the independent native producer test, these events are fixture
    /// notifications; this is the TUI boundary, not a second native execution.
    #[tokio::test]
    async fn codexdd_046_3c1b2h_tui_live_wire_handoff_ready_and_conflict_refusal() {
        use crate::adaptive_complexity::AdaptiveComplexityClass;
        use crate::adaptive_complexity::AdaptiveImplementationPhase;
        use crate::adaptive_gate_prerequisites::AdaptiveGateLedger;
        use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;
        use crate::adaptive_worker::AdaptiveTaskKind;
        use crate::adaptive_worker::AdaptiveWorkerContext;
        use crate::adaptive_worker::AdaptiveWorkerPreflight;
        use crate::adaptive_worker::AdaptiveWorkerRole;
        use crate::adaptive_worker::AdaptiveWorkflowTerminal;
        use crate::adaptive_worker::NewWorkerBinding;
        use crate::chatwidget::tests::make_chatwidget_manual_with_sender;
        use codex_app_server_protocol::AdaptiveRuntimeSignalNotification;
        use codex_app_server_protocol::DynamicToolCallStatus;
        use codex_app_server_protocol::ServerNotification;
        use codex_protocol::ThreadId;

        let (mut chat, _sender, mut app_events, _op_rx) =
            make_chatwidget_manual_with_sender().await;
        let source = tempfile::tempdir().expect("disposable authorized source");
        let scope = "3C.1b.2h TUI exact native handoff";
        let sha = "a".repeat(40);
        let implementation = ThreadId::new();
        let validation = ThreadId::new();
        let native_item = |thread: ThreadId,
                           turn: &str,
                           id: &str,
                           tool: &str,
                           profile: &str,
                           run: &str,
                           observed_sha: &str|
         -> ItemCompletedNotification {
            let item = ItemCompletedNotification {
                item: ThreadItem::DynamicToolCall {
                    id: id.to_string(),
                    namespace: None,
                    tool: tool.to_string(),
                    arguments: serde_json::json!({
                        "profile": profile,
                        "result": "pass",
                        "run_id": run,
                        "head_sha": observed_sha,
                        "source_root": source.path().display().to_string(),
                    }),
                    status: DynamicToolCallStatus::Completed,
                    content_items: None,
                    success: Some(true),
                    duration_ms: Some(1),
                },
                thread_id: thread.to_string(),
                turn_id: turn.to_string(),
                completed_at_ms: 2_000,
            };
            // Prove JSON-RPC wire serialization retains the identity fields.
            serde_json::from_value(serde_json::to_value(item).expect("app-server encode"))
                .expect("app-server decode")
        };

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
            .expect("owner's exact source requirements"),
        );
        let source_turn = "owner-implementation-native-turn";
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(source_turn.to_string());
        chat.handle_server_notification(
            ServerNotification::ItemCompleted(native_item(
                implementation,
                source_turn,
                "native-source-receipt",
                CODEXDD_VALIDATION_TOOL_NAME,
                "work_packet",
                "native-source-run",
                &sha,
            )),
            None,
        );
        assert_eq!(
            chat.adaptive_effort
                .gate_ledger
                .as_ref()
                .expect("owner gates")
                .outstanding(&chat.adaptive_effort.evidence_registry)
                .len(),
            1,
            "source gate is observed via live TUI notification without test-only injection"
        );
        chat.adaptive_effort.complexity_class = Some(AdaptiveComplexityClass::Standard);
        chat.adaptive_effort.implementation_phase =
            Some(AdaptiveImplementationPhase::MechanicalValidation);
        chat.handle_server_notification(
            ServerNotification::AdaptiveRuntimeSignal(AdaptiveRuntimeSignalNotification {
                thread_id: implementation.to_string(),
                signal: AdaptiveRuntimeSignalEnvelope {
                    source_turn_id: source_turn.to_string(),
                    signal_kind: AdaptiveRuntimeSignalKind::ReadyForValidation,
                    evidence_refs: vec!["native-source-receipt".to_string()],
                    diagnostic_note: Some("READY_FOR_VALIDATION".to_string()),
                },
            }),
            None,
        );
        chat.turn_lifecycle.agent_turn_running = false;
        assert!(chat.consume_adaptive_signal_at_terminal(source_turn));
        assert_eq!(
            chat.adaptive_effort.workflow_terminal,
            Some(AdaptiveWorkflowTerminal::ReadyForValidation)
        );
        let binding = NewWorkerBinding {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: scope.to_string(),
        };
        let fresh = chat.fresh_adaptive_effort_for_new_worker(Some(&binding));
        assert_eq!(fresh.review_origin_thread_id, Some(implementation));
        assert!(fresh.gate_ledger.is_some());
        assert!(
            fresh
                .gate_ledger
                .as_ref()
                .expect("inherited owner ledger")
                .outstanding(&fresh.evidence_registry)
                .iter()
                .any(|gate| gate.gate == AdaptiveRequiredGate::IndependentValidation)
        );

        chat.thread_id = Some(validation);
        chat.adaptive_effort = fresh;
        let review_turn = "separate-validator-native-turn";
        chat.turn_lifecycle.agent_turn_running = true;
        chat.turn_lifecycle.last_turn_id = Some(review_turn.to_string());

        // A foreign-thread completion and a matching-tool, wrong-SHA
        // completion cannot close the original owner's Validator gate.
        chat.handle_server_notification(
            ServerNotification::ItemCompleted(native_item(
                implementation,
                review_turn,
                "foreign-review",
                CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME,
                "independent_review",
                "foreign-run",
                &sha,
            )),
            None,
        );
        assert!(
            chat.adaptive_effort
                .evidence_registry
                .resolve("foreign-review")
                .is_err()
        );
        chat.handle_server_notification(
            ServerNotification::ItemCompleted(native_item(
                validation,
                review_turn,
                "wrong-sha-review",
                CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME,
                "independent_review",
                "wrong-sha-run",
                &"b".repeat(40),
            )),
            None,
        );
        assert!(
            !chat
                .adaptive_effort
                .gate_ledger
                .as_ref()
                .expect("inherited owner gates")
                .ready_for_owner_qa(&chat.adaptive_effort.evidence_registry)
        );

        chat.handle_server_notification(
            ServerNotification::ItemCompleted(native_item(
                validation,
                review_turn,
                "native-independent-review",
                CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME,
                "independent_review",
                "independent-review-run",
                &sha,
            )),
            None,
        );
        assert!(
            chat.adaptive_effort
                .gate_ledger
                .as_ref()
                .expect("inherited owner gates")
                .ready_for_owner_qa(&chat.adaptive_effort.evidence_registry)
        );
        assert!(
            chat.owner_qa_prerequisite_admission(&["wrong-sha-review".to_string()])
                .is_err(),
            "matching schema alone must not authorize a mismatched-SHA signal"
        );
        assert!(
            chat.owner_qa_prerequisite_admission(&["generic-command".to_string()])
                .is_err(),
            "a green nonnative result cannot cite the owner's independent gate"
        );
        let original_pin = chat.adaptive_effort.assignment_preflight.clone();
        chat.adaptive_effort
            .assignment_preflight
            .as_mut()
            .expect("review pin")
            .source_sha = "c".repeat(40);
        assert!(
            chat.owner_qa_prerequisite_admission(&["native-independent-review".to_string()])
                .is_err(),
            "an active Validator pin cannot be swapped after review"
        );
        chat.adaptive_effort.assignment_preflight = original_pin;
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Implementation;
        assert!(
            chat.owner_qa_prerequisite_admission(&["native-independent-review".to_string()])
                .is_err(),
            "Implementation role can never claim Validator READY"
        );
        chat.adaptive_effort.worker_context.role = AdaptiveWorkerRole::Validation;

        // Conflicting re-delivery poisons the previous receipt; even a
        // matching first PASS must no longer be reusable at READY.
        chat.handle_server_notification(
            ServerNotification::ItemCompleted(native_item(
                validation,
                review_turn,
                "native-independent-review",
                CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME,
                "independent_review",
                "conflicted-review-run",
                &"c".repeat(40),
            )),
            None,
        );
        assert!(
            !chat
                .adaptive_effort
                .gate_ledger
                .as_ref()
                .expect("owner ledger")
                .ready_for_owner_qa(&chat.adaptive_effort.evidence_registry),
            "conflicting same-ID review invalidates cached provenance"
        );

        // A new, independently emitted native run is needed after conflict.
        chat.handle_server_notification(
            ServerNotification::ItemCompleted(native_item(
                validation,
                review_turn,
                "native-independent-review-retry",
                CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME,
                "independent_review",
                "independent-review-retry-run",
                &sha,
            )),
            None,
        );
        assert!(
            chat.owner_qa_prerequisite_admission(&["native-independent-review-retry".to_string()])
                .is_ok()
        );

        chat.handle_server_notification(
            ServerNotification::AdaptiveRuntimeSignal(AdaptiveRuntimeSignalNotification {
                thread_id: validation.to_string(),
                signal: AdaptiveRuntimeSignalEnvelope {
                    source_turn_id: review_turn.to_string(),
                    signal_kind: AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
                    evidence_refs: vec!["native-independent-review-retry".to_string()],
                    diagnostic_note: Some("verified independent source gate".to_string()),
                },
            }),
            None,
        );
        chat.turn_lifecycle.agent_turn_running = false;
        assert!(chat.consume_adaptive_signal_at_terminal(review_turn));
        assert_eq!(
            chat.adaptive_effort.workflow_terminal,
            Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa),
            "only fresh Core-native review and all owner gates can admit terminal"
        );
        assert!(!chat.maybe_submit_adaptive_successor());

        // READY may be persisted only for the exact Validation Worker.
        let mut ready_persisted = false;
        while let Ok(event) = app_events.try_recv() {
            if let crate::app_event::AppEvent::PersistWorkflowState {
                thread_id,
                operation:
                    codex_app_server_protocol::ThreadWorkflowStateOperation::SetReadyForOwnerQa,
                ..
            } = event
            {
                assert_eq!(thread_id, validation);
                ready_persisted = true;
            }
        }
        assert!(ready_persisted);
    }

    #[tokio::test]
    async fn codexdd_046_3c1b2h_health_gate_and_replay_are_not_native_ready() {
        use crate::adaptive_gate_prerequisites::AdaptiveGateLedger;
        use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;
        use crate::adaptive_worker::AdaptiveTaskKind;
        use crate::adaptive_worker::AdaptiveWorkerContext;
        use crate::adaptive_worker::AdaptiveWorkerPreflight;
        use crate::adaptive_worker::AdaptiveWorkerRole;
        use crate::chatwidget::tests::make_chatwidget_manual_with_sender;
        use codex_app_server_protocol::DynamicToolCallStatus;
        use codex_app_server_protocol::ServerNotification;
        use codex_protocol::ThreadId;

        let (mut chat, _sender, _app_rx, _op_rx) = make_chatwidget_manual_with_sender().await;
        let source = tempfile::tempdir().expect("disposable worktree");
        let sha = "a".repeat(40);
        let scope = "owner-mandated release and owned health";
        let implementation = ThreadId::new();
        let validation = ThreadId::new();
        chat.thread_id = Some(validation);
        chat.dispatch_adaptive_command("astra");
        chat.adaptive_effort.worker_context = AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: Some(scope.to_string()),
        };
        chat.adaptive_effort.assignment_preflight = Some(AdaptiveWorkerPreflight {
            task_kind: AdaptiveTaskKind::SourceChange,
            source_root: source.path().display().to_string(),
            source_sha: sha.clone(),
        });
        chat.adaptive_effort.worker_assignment_locked = true;
        chat.adaptive_effort.review_origin_thread_id = Some(implementation);
        chat.adaptive_effort.gate_ledger = Some(
            AdaptiveGateLedger::new(
                scope,
                source.path(),
                &sha,
                vec![
                    AdaptiveRequiredGate::SourceProfile("release".to_string()),
                    AdaptiveRequiredGate::OwnedRuntimeHealth(vec![4331]),
                    AdaptiveRequiredGate::IndependentValidation,
                ],
            )
            .expect("mandatory health gate"),
        );
        let review = ItemCompletedNotification {
            item: ThreadItem::DynamicToolCall {
                id: "real-independent-review".to_string(),
                namespace: None,
                tool: CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME.to_string(),
                arguments: serde_json::json!({
                    "profile": "independent_review",
                    "result": "pass",
                    "head_sha": sha,
                    "source_root": source.path().display().to_string(),
                    "run_id": "independent-real-run",
                }),
                status: DynamicToolCallStatus::Completed,
                content_items: None,
                success: Some(true),
                duration_ms: Some(1),
            },
            thread_id: validation.to_string(),
            turn_id: "review-turn".to_string(),
            completed_at_ms: 2_000,
        };
        // Historical snapshot/replay never creates native Validator authority.
        chat.handle_server_notification(
            ServerNotification::ItemCompleted(review.clone()),
            Some(ReplayKind::ThreadSnapshot),
        );
        assert!(
            !chat
                .adaptive_effort
                .gate_ledger
                .as_ref()
                .expect("owner gates")
                .ready_for_owner_qa(&chat.adaptive_effort.evidence_registry)
        );
        chat.handle_server_notification(ServerNotification::ItemCompleted(review), None);
        let pending = chat
            .adaptive_effort
            .gate_ledger
            .as_ref()
            .expect("owner gates")
            .outstanding(&chat.adaptive_effort.evidence_registry);
        assert_eq!(
            pending.len(),
            2,
            "even valid independent Core PASS leaves unrun release and owned listener gates pending"
        );
        assert!(
            chat.owner_qa_prerequisite_admission(&["real-independent-review".to_string()])
                .is_err()
        );
    }
}
