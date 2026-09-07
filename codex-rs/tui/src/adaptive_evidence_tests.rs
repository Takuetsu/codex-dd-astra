use super::adaptive_evidence::AdaptiveEvidenceKind;
use super::adaptive_evidence::AdaptiveEvidenceOutcome;
use super::adaptive_evidence::AdaptiveEvidenceRecord;
use super::adaptive_evidence::AdaptiveEvidenceRegistration;
use super::adaptive_evidence::AdaptiveEvidenceRegistry;
use super::adaptive_evidence::AdaptiveEvidenceResolveError;
use super::adaptive_evidence::record_from_item_completion;
use codex_app_server_protocol::CommandExecutionSource;
use codex_app_server_protocol::CommandExecutionStatus;
use codex_app_server_protocol::DynamicToolCallStatus;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::McpToolCallStatus;
use codex_app_server_protocol::ThreadItem;
use codex_protocol::ThreadId;
use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;

fn record(
    evidence_id: &str,
    thread_id: ThreadId,
    source_turn_id: &str,
    outcome: AdaptiveEvidenceOutcome,
) -> AdaptiveEvidenceRecord {
    AdaptiveEvidenceRecord {
        evidence_id: evidence_id.to_string(),
        thread_id,
        source_turn_id: source_turn_id.to_string(),
        outcome,
        kind: AdaptiveEvidenceKind::CommandExecution,
    }
}

#[test]
fn native_command_completion_becomes_small_typed_evidence() {
    let thread_id = ThreadId::new();
    let notification = ItemCompletedNotification {
        item: ThreadItem::CommandExecution {
            id: "native-call-1".to_string(),
            plugin_id: None,
            script_path: None,
            command: "secret command text that is not evidence".to_string(),
            cwd: AbsolutePathBuf::from_absolute_path(std::env::current_dir().expect("cwd"))
                .expect("absolute cwd")
                .into(),
            process_id: None,
            source: CommandExecutionSource::Agent,
            status: CommandExecutionStatus::Completed,
            command_actions: Vec::new(),
            aggregated_output: Some("bulk stdout that must not be stored".to_string()),
            exit_code: Some(0),
            duration_ms: Some(1),
        },
        thread_id: thread_id.to_string(),
        turn_id: "turn-1".to_string(),
        completed_at_ms: 999,
    };

    assert_eq!(
        record_from_item_completion(&notification),
        Some(AdaptiveEvidenceRecord {
            evidence_id: "native-call-1".to_string(),
            thread_id,
            source_turn_id: "turn-1".to_string(),
            outcome: AdaptiveEvidenceOutcome::Success,
            kind: AdaptiveEvidenceKind::CommandExecution,
        })
    );
}

#[test]
fn mcp_and_dynamic_completions_reuse_their_runtime_call_ids() {
    let thread_id = ThreadId::new();
    let mcp_notification = ItemCompletedNotification {
        item: ThreadItem::McpToolCall {
            id: "runtime-mcp-call".to_string(),
            server: "server".to_string(),
            tool: "tool".to_string(),
            status: McpToolCallStatus::Completed,
            arguments: serde_json::json!({}),
            app_context: None,
            mcp_app_resource_uri: None,
            plugin_id: None,
            read_only_hint: None,
            result: None,
            error: None,
            duration_ms: Some(1),
        },
        thread_id: thread_id.to_string(),
        turn_id: "turn-1".to_string(),
        completed_at_ms: 1,
    };
    let dynamic_notification = ItemCompletedNotification {
        item: ThreadItem::DynamicToolCall {
            id: "runtime-dynamic-call".to_string(),
            namespace: Some("namespace".to_string()),
            tool: "tool".to_string(),
            arguments: serde_json::json!({}),
            status: DynamicToolCallStatus::Completed,
            content_items: None,
            success: Some(true),
            duration_ms: Some(1),
        },
        thread_id: thread_id.to_string(),
        turn_id: "turn-1".to_string(),
        completed_at_ms: 1,
    };

    assert_eq!(
        record_from_item_completion(&mcp_notification),
        Some(AdaptiveEvidenceRecord {
            evidence_id: "runtime-mcp-call".to_string(),
            thread_id,
            source_turn_id: "turn-1".to_string(),
            outcome: AdaptiveEvidenceOutcome::Success,
            kind: AdaptiveEvidenceKind::McpToolCall,
        })
    );
    assert_eq!(
        record_from_item_completion(&dynamic_notification),
        Some(AdaptiveEvidenceRecord {
            evidence_id: "runtime-dynamic-call".to_string(),
            thread_id,
            source_turn_id: "turn-1".to_string(),
            outcome: AdaptiveEvidenceOutcome::Success,
            kind: AdaptiveEvidenceKind::DynamicToolCall,
        })
    );
}

#[test]
fn native_command_outcomes_are_not_upgraded_from_output() {
    let thread_id = ThreadId::new();
    for (status, expected) in [
        (
            CommandExecutionStatus::Failed,
            AdaptiveEvidenceOutcome::Failure,
        ),
        (
            CommandExecutionStatus::Declined,
            AdaptiveEvidenceOutcome::Cancelled,
        ),
        (
            CommandExecutionStatus::InProgress,
            AdaptiveEvidenceOutcome::Incomplete,
        ),
    ] {
        let notification = ItemCompletedNotification {
            item: ThreadItem::CommandExecution {
                id: format!("native-{status:?}"),
                plugin_id: None,
                script_path: None,
                command: "printf success".to_string(),
                cwd: AbsolutePathBuf::from_absolute_path(std::env::current_dir().expect("cwd"))
                    .expect("absolute cwd")
                    .into(),
                process_id: None,
                source: CommandExecutionSource::Agent,
                status,
                command_actions: Vec::new(),
                aggregated_output: Some("SUCCESS all checks passed".to_string()),
                exit_code: Some(1),
                duration_ms: Some(1),
            },
            thread_id: thread_id.to_string(),
            turn_id: "turn-1".to_string(),
            completed_at_ms: 1,
        };
        assert_eq!(
            record_from_item_completion(&notification)
                .expect("command completion should produce evidence")
                .outcome,
            expected
        );
    }
}

#[test]
fn registry_is_idempotent_and_conflicts_fail_closed_permanently() {
    let thread_id = ThreadId::new();
    let original = record(
        "native-call-1",
        thread_id,
        "turn-1",
        AdaptiveEvidenceOutcome::Success,
    );
    let mut registry = AdaptiveEvidenceRegistry::default();

    assert_eq!(
        registry.register(original.clone()),
        AdaptiveEvidenceRegistration::Registered
    );
    assert_eq!(
        registry.register(original.clone()),
        AdaptiveEvidenceRegistration::Duplicate
    );
    assert_eq!(registry.len(), 1);

    let conflicting = AdaptiveEvidenceRecord {
        outcome: AdaptiveEvidenceOutcome::Failure,
        ..original.clone()
    };
    assert_eq!(
        registry.register(conflicting),
        AdaptiveEvidenceRegistration::Conflicted
    );
    assert_eq!(
        registry.resolve("native-call-1"),
        Err(AdaptiveEvidenceResolveError::Conflicted)
    );
    assert_eq!(
        registry.register(original),
        AdaptiveEvidenceRegistration::Conflicted
    );
    assert_eq!(
        registry.resolve("native-call-1"),
        Err(AdaptiveEvidenceResolveError::Conflicted)
    );
}

#[test]
fn resolution_rejects_missing_cross_thread_stale_and_wrong_outcome() {
    let owning_thread = ThreadId::new();
    let other_thread = ThreadId::new();
    let evidence = record(
        "native-call-1",
        owning_thread,
        "turn-1",
        AdaptiveEvidenceOutcome::Failure,
    );
    let mut registry = AdaptiveEvidenceRegistry::default();
    registry.register(evidence.clone());

    assert_eq!(registry.resolve("native-call-1"), Ok(&evidence));
    assert_eq!(
        registry.resolve("missing"),
        Err(AdaptiveEvidenceResolveError::NotFound)
    );
    assert_eq!(
        registry.resolve_for_thread("native-call-1", other_thread),
        Err(AdaptiveEvidenceResolveError::CrossThread)
    );
    assert_eq!(
        registry.resolve_for_turn("native-call-1", owning_thread, "turn-2"),
        Err(AdaptiveEvidenceResolveError::SourceTurnMismatch)
    );
    assert_eq!(
        registry.validate_outcome(
            "native-call-1",
            owning_thread,
            AdaptiveEvidenceOutcome::Success,
        ),
        Err(AdaptiveEvidenceResolveError::OutcomeMismatch)
    );
}

#[test]
fn conflict_on_source_turn_is_also_permanent() {
    let thread_id = ThreadId::new();
    let original = record(
        "native-call-1",
        thread_id,
        "turn-1",
        AdaptiveEvidenceOutcome::Success,
    );
    let mut registry = AdaptiveEvidenceRegistry::default();
    registry.register(original.clone());
    registry.register(AdaptiveEvidenceRecord {
        source_turn_id: "turn-2".to_string(),
        ..original.clone()
    });

    assert_eq!(
        registry.resolve("native-call-1"),
        Err(AdaptiveEvidenceResolveError::Conflicted)
    );
    registry.register(original);
    assert_eq!(
        registry.resolve("native-call-1"),
        Err(AdaptiveEvidenceResolveError::Conflicted)
    );
}
