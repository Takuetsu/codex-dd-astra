use super::*;
use crate::session::step_context::StepContext;
use crate::session::tests::make_session_and_context_with_rx;
use crate::tools::context::ToolCallSource;
use crate::turn_diff_tracker::TurnDiffTracker;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn production_handler_emits_signal_with_active_turn_identity() {
    let (session, turn, events) = make_session_and_context_with_rx().await;
    let source_turn_id = turn.sub_id.clone();
    let output = AdaptiveSignalHandler
        .handle(ToolInvocation {
            session,
            step_context: StepContext::for_test(Arc::clone(&turn)),
            turn,
            cancellation_token: tokio_util::sync::CancellationToken::new(),
            tracker: Arc::new(Mutex::new(TurnDiffTracker::default())),
            call_id: "adaptive-call".to_string(),
            tool_name: ToolName::plain(TOOL_NAME),
            source: ToolCallSource::Direct,
            payload: ToolPayload::Function {
                arguments: json!({
                    "kind": "repair_required",
                    "evidence_refs": ["evidence-b", "evidence-a", "evidence-a"],
                    "diagnostic_note": "non-authoritative"
                })
                .to_string(),
            },
        })
        .await
        .expect("handler output");

    let event = events.recv().await.expect("adaptive signal event");
    assert_eq!(event.id, source_turn_id);
    let EventMsg::AdaptiveRuntimeSignal(signal) = event.msg else {
        panic!("expected adaptive runtime signal");
    };
    assert_eq!(
        signal.signal_kind,
        AdaptiveRuntimeSignalKind::RepairRequired
    );
    assert_eq!(
        signal.evidence_refs,
        vec!["evidence-a".to_string(), "evidence-b".to_string()]
    );
    assert_eq!(signal.diagnostic_note.as_deref(), Some("non-authoritative"));
    assert_eq!(output.log_output(), "{\"status\":\"request_emitted\"}");
    assert!(output.success_for_logging());
}

#[tokio::test]
async fn repository_handoff_alias_emits_owner_qa_terminal_signal() {
    let (session, turn, events) = make_session_and_context_with_rx().await;
    let source_turn_id = turn.sub_id.clone();
    let output = AdaptiveSignalHandler
        .handle(ToolInvocation {
            session,
            step_context: StepContext::for_test(Arc::clone(&turn)),
            turn,
            cancellation_token: tokio_util::sync::CancellationToken::new(),
            tracker: Arc::new(Mutex::new(TurnDiffTracker::default())),
            call_id: "adaptive-handoff-call".to_string(),
            tool_name: ToolName::plain(TOOL_NAME),
            source: ToolCallSource::Direct,
            payload: ToolPayload::Function {
                arguments: json!({
                    "kind": "ready_for_repository_handoff",
                    "evidence_refs": ["success-b", "success-a", "success-a"],
                    "diagnostic_note": "PASS - READY FOR REPOSITORY HANDOFF"
                })
                .to_string(),
            },
        })
        .await
        .expect("handler output");

    let event = events.recv().await.expect("adaptive signal event");
    assert_eq!(event.id, source_turn_id);
    let EventMsg::AdaptiveRuntimeSignal(signal) = event.msg else {
        panic!("expected adaptive runtime signal");
    };
    assert_eq!(
        signal.signal_kind,
        AdaptiveRuntimeSignalKind::ReadyForOwnerQa
    );
    assert_eq!(
        signal.evidence_refs,
        vec!["success-a".to_string(), "success-b".to_string()]
    );
    assert_eq!(
        signal.diagnostic_note.as_deref(),
        Some("PASS - READY FOR REPOSITORY HANDOFF")
    );
    assert_eq!(output.log_output(), "{\"status\":\"request_emitted\"}");
    assert!(output.success_for_logging());
}

#[test]
fn tool_spec_documents_repository_handoff_terminal_mapping() {
    let ToolSpec::Function(spec) = AdaptiveSignalHandler.spec() else {
        panic!("expected function tool spec");
    };
    assert!(spec.description.contains("READY FOR REPOSITORY HANDOFF"));
    assert!(spec.description.contains("ready_for_repository_handoff"));
    assert!(
        spec.description
            .contains("Final-answer prose is non-authoritative")
    );
}

#[tokio::test]
async fn production_handler_rejects_authority_fields() {
    let (session, turn, _events) = make_session_and_context_with_rx().await;
    for field in [
        "thread_id",
        "source_turn_id",
        "terminal_turn_id",
        "role",
        "scope",
        "model",
        "effort",
        "attempt",
        "requested_next_route",
    ] {
        let mut arguments = json!({ "kind": "capability" });
        arguments[field] = json!("caller authority");
        let result = AdaptiveSignalHandler
            .handle(ToolInvocation {
                session: Arc::clone(&session),
                step_context: StepContext::for_test(Arc::clone(&turn)),
                turn: Arc::clone(&turn),
                cancellation_token: tokio_util::sync::CancellationToken::new(),
                tracker: Arc::new(Mutex::new(TurnDiffTracker::default())),
                call_id: "adaptive-call".to_string(),
                tool_name: ToolName::plain(TOOL_NAME),
                source: ToolCallSource::Direct,
                payload: ToolPayload::Function {
                    arguments: arguments.to_string(),
                },
            })
            .await;
        assert!(result.is_err(), "field {field} must be rejected");
    }
}
