use super::adaptive_evidence::AdaptiveEvidenceKind;
use super::adaptive_evidence::AdaptiveEvidenceOutcome;
use super::adaptive_evidence::AdaptiveEvidenceRecord;
use super::adaptive_evidence::AdaptiveEvidenceRegistration;
use super::adaptive_evidence::AdaptiveEvidenceRegistry;
use super::adaptive_evidence::AdaptiveEvidenceResolveError;
use super::adaptive_evidence::AdaptiveSourceGateRequirement;
use super::adaptive_evidence::record_from_item_completion;
use super::adaptive_gate_prerequisites::AdaptiveGateCheckpoint;
use super::adaptive_gate_prerequisites::AdaptiveGateLedger;
use super::adaptive_gate_prerequisites::AdaptiveGateState;
use super::adaptive_gate_prerequisites::AdaptiveRequiredGate;
use super::adaptive_worker::AdaptiveTaskKind;
use super::adaptive_worker::AdaptiveWorkerContext;
use super::adaptive_worker::AdaptiveWorkerPreflight;
use super::adaptive_worker::AdaptiveWorkerRole;
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
        tool_name: None,
        validation_profile: None,
        validation_failure_fingerprint: None,
        native_source_provenance: None,
        worker_binding: None,
    }
}

#[test]
fn native_command_completion_becomes_small_typed_evidence() {
    let thread_id = ThreadId::new();
    let notification = ItemCompletedNotification {
        item: ThreadItem::CommandExecution {
            model_context: None,
            sandbox_type: None,
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
            tool_name: None,
            validation_profile: None,
            validation_failure_fingerprint: None,
            native_source_provenance: None,
            worker_binding: None,
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
            mcp_app_ui: None,
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
            tool_name: None,
            validation_profile: None,
            validation_failure_fingerprint: None,
            native_source_provenance: None,
            worker_binding: None,
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
            tool_name: Some("tool".to_string()),
            validation_profile: None,
            validation_failure_fingerprint: None,
            native_source_provenance: None,
            worker_binding: None,
        })
    );
}

#[test]
fn native_validation_tool_completion_becomes_typed_work_packet_receipt() {
    let thread_id = ThreadId::new();
    let notification = ItemCompletedNotification {
        item: ThreadItem::DynamicToolCall {
            id: "validation-call".to_string(),
            namespace: None,
            tool: super::adaptive_evidence::CODEXDD_VALIDATION_TOOL_NAME.to_string(),
            arguments: serde_json::json!({ "profile": "work_packet" }),
            status: DynamicToolCallStatus::Completed,
            content_items: None,
            success: Some(true),
            duration_ms: Some(1),
        },
        thread_id: thread_id.to_string(),
        turn_id: "turn-validation".to_string(),
        completed_at_ms: 1,
    };

    let record = record_from_item_completion(&notification).expect("validation receipt");
    assert_eq!(record.evidence_id, "validation-call");
    assert_eq!(record.outcome, AdaptiveEvidenceOutcome::Success);
    assert_eq!(record.kind, AdaptiveEvidenceKind::DynamicToolCall);
    assert_eq!(
        record.tool_name.as_deref(),
        Some(super::adaptive_evidence::CODEXDD_VALIDATION_TOOL_NAME)
    );
    assert_eq!(
        record.validation_profile.as_deref(),
        Some(super::adaptive_evidence::CODEXDD_WORK_PACKET_PROFILE)
    );

    let mut registry = AdaptiveEvidenceRegistry::default();
    registry.register(record);
    assert!(
        registry.has_successful_codexdd_work_packet_receipt_for_turn(
            &["validation-call".to_string()],
            thread_id,
            "turn-validation",
        )
    );
}

#[test]
fn failed_validation_receipt_exposes_repairable_fingerprint() {
    let thread_id = ThreadId::new();
    let notification = ItemCompletedNotification {
        item: ThreadItem::DynamicToolCall {
            id: "failed-validation".to_string(),
            namespace: None,
            tool: super::adaptive_evidence::CODEXDD_VALIDATION_TOOL_NAME.to_string(),
            arguments: serde_json::json!({
                "profile": "work_packet",
                "failure_fingerprint": "v1:work_packet:core-adaptive-tests",
                "failed_stage": "core-adaptive-tests"
            }),
            status: DynamicToolCallStatus::Completed,
            content_items: None,
            success: Some(false),
            duration_ms: Some(1),
        },
        thread_id: thread_id.to_string(),
        turn_id: "turn-validation-failed".to_string(),
        completed_at_ms: 1,
    };

    let record = record_from_item_completion(&notification).expect("failed validation receipt");
    assert_eq!(
        record.validation_failure_fingerprint.as_deref(),
        Some("v1:work_packet:core-adaptive-tests")
    );

    let mut registry = AdaptiveEvidenceRegistry::default();
    registry.register(record);
    assert_eq!(
        registry.codexdd_repairable_validation_failure_for_turn(
            &["failed-validation".to_string()],
            thread_id,
            "turn-validation-failed",
        ),
        Some((
            "work_packet".to_string(),
            "v1:work_packet:core-adaptive-tests".to_string(),
        ))
    );
}

#[test]
fn work_packet_failure_is_preferred_over_targeted_regardless_of_ref_order() {
    let thread_id = ThreadId::new();
    let turn_id = "turn-multiple-validation-failures";
    let mut registry = AdaptiveEvidenceRegistry::default();

    for (id, profile, fingerprint) in [
        (
            "targeted-failure",
            "targeted",
            "v1:targeted:core-adaptive-tests",
        ),
        (
            "work-packet-failure",
            super::adaptive_evidence::CODEXDD_WORK_PACKET_PROFILE,
            "v1:work_packet:tui-adaptive-tests",
        ),
    ] {
        registry.register(AdaptiveEvidenceRecord {
            evidence_id: id.to_string(),
            thread_id,
            source_turn_id: turn_id.to_string(),
            outcome: AdaptiveEvidenceOutcome::Failure,
            kind: AdaptiveEvidenceKind::DynamicToolCall,
            tool_name: Some(super::adaptive_evidence::CODEXDD_VALIDATION_TOOL_NAME.to_string()),
            validation_profile: Some(profile.to_string()),
            validation_failure_fingerprint: Some(fingerprint.to_string()),
            native_source_provenance: None,
            worker_binding: None,
        });
    }

    for refs in [
        vec![
            "targeted-failure".to_string(),
            "work-packet-failure".to_string(),
        ],
        vec![
            "work-packet-failure".to_string(),
            "targeted-failure".to_string(),
        ],
    ] {
        assert_eq!(
            registry.codexdd_repairable_validation_failure_for_turn(&refs, thread_id, turn_id,),
            Some((
                super::adaptive_evidence::CODEXDD_WORK_PACKET_PROFILE.to_string(),
                "v1:work_packet:tui-adaptive-tests".to_string(),
            ))
        );
    }
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
                model_context: None,
                sandbox_type: None,
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

const SOURCE_SHA_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const SOURCE_SHA_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn scoped_native_source_receipt(
    thread_id: ThreadId,
    source_root: &std::path::Path,
    source_sha: &str,
    profile: &str,
) -> AdaptiveEvidenceRecord {
    let notification = ItemCompletedNotification {
        item: ThreadItem::DynamicToolCall {
            id: "native-lvo-evidence".to_string(),
            namespace: None,
            tool: super::adaptive_evidence::CODEXDD_VALIDATION_TOOL_NAME.to_string(),
            arguments: serde_json::json!({
                "profile": profile,
                "result": "pass",
                "run_id": "native-run-123",
                "head_sha": source_sha,
                "source_root": source_root.canonicalize().expect("canonical root").display().to_string(),
            }),
            status: DynamicToolCallStatus::Completed,
            content_items: None,
            success: Some(true),
            duration_ms: Some(1),
        },
        thread_id: thread_id.to_string(),
        turn_id: "native-source-turn".to_string(),
        completed_at_ms: 1_000,
    };
    let mut record = record_from_item_completion(&notification).expect("native record");
    record.bind_worker_identity(
        &AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Implementation,
            authorized_scope: Some("isolated source packet".to_string()),
        },
        Some(&AdaptiveWorkerPreflight {
            task_kind: AdaptiveTaskKind::SourceChange,
            source_root: source_root.display().to_string(),
            source_sha: source_sha.to_string(),
        }),
    );
    record
}

fn native_source_requirement<'a>(
    thread_id: ThreadId,
    source_root: &'a std::path::Path,
    source_sha: &'a str,
    required_profile: &'a str,
) -> AdaptiveSourceGateRequirement<'a> {
    AdaptiveSourceGateRequirement {
        thread_id,
        worker_role: AdaptiveWorkerRole::Implementation,
        task_kind: AdaptiveTaskKind::SourceChange,
        authorized_scope: "isolated source packet",
        source_root,
        source_sha,
        required_profile,
        source_turn_id: Some("native-source-turn"),
        expected_run_id: Some("native-run-123"),
        minimum_completed_at_ms: 999,
    }
}

#[test]
fn codexdd_046_3b3b_a03_requires_exact_source_sha_and_worktree() {
    let root = tempfile::tempdir().expect("root");
    let other_root = tempfile::tempdir().expect("other root");
    let thread_id = ThreadId::new();
    let mut registry = AdaptiveEvidenceRegistry::default();
    assert_eq!(
        registry.register(scoped_native_source_receipt(
            thread_id,
            root.path(),
            SOURCE_SHA_A,
            "work_packet",
        )),
        AdaptiveEvidenceRegistration::Registered
    );
    let current = native_source_requirement(thread_id, root.path(), SOURCE_SHA_A, "work_packet");
    assert!(
        registry
            .match_native_source_gate("native-lvo-evidence", &current)
            .is_ok()
    );
    let changed_source =
        native_source_requirement(thread_id, root.path(), SOURCE_SHA_B, "work_packet");
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &changed_source),
        Err(AdaptiveEvidenceResolveError::SourceIdentityMismatch)
    );
    let changed_worktree =
        native_source_requirement(thread_id, other_root.path(), SOURCE_SHA_A, "work_packet");
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &changed_worktree),
        Err(AdaptiveEvidenceResolveError::SourceIdentityMismatch)
    );
}

#[test]
fn codexdd_046_3b3b_a04_rejects_other_worker_scope_and_narrower_gate() {
    let root = tempfile::tempdir().expect("root");
    let thread_id = ThreadId::new();
    let other_worker = ThreadId::new();
    let mut registry = AdaptiveEvidenceRegistry::default();
    registry.register(scoped_native_source_receipt(
        thread_id,
        root.path(),
        SOURCE_SHA_A,
        "targeted",
    ));
    let focused = native_source_requirement(thread_id, root.path(), SOURCE_SHA_A, "targeted");
    assert!(
        registry
            .match_native_source_gate("native-lvo-evidence", &focused)
            .is_ok()
    );
    let whole_gate = native_source_requirement(thread_id, root.path(), SOURCE_SHA_A, "work_packet");
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &whole_gate),
        Err(AdaptiveEvidenceResolveError::GateScopeMismatch)
    );
    let other_thread =
        native_source_requirement(other_worker, root.path(), SOURCE_SHA_A, "targeted");
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &other_thread),
        Err(AdaptiveEvidenceResolveError::CrossThread)
    );
    let mut other_scope =
        native_source_requirement(thread_id, root.path(), SOURCE_SHA_A, "targeted");
    other_scope.authorized_scope = "other packet";
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &other_scope),
        Err(AdaptiveEvidenceResolveError::WorkerBindingMismatch)
    );
    let mut wrong_role =
        native_source_requirement(thread_id, root.path(), SOURCE_SHA_A, "targeted");
    wrong_role.worker_role = AdaptiveWorkerRole::Validation;
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &wrong_role),
        Err(AdaptiveEvidenceResolveError::WorkerBindingMismatch)
    );
}

#[test]
fn codexdd_046_3b3b_legacy_unpinned_and_generic_success_are_not_gate_proof() {
    let root = tempfile::tempdir().expect("root");
    let thread_id = ThreadId::new();
    let requirement =
        native_source_requirement(thread_id, root.path(), SOURCE_SHA_A, "work_packet");
    let legacy_notification = ItemCompletedNotification {
        item: ThreadItem::DynamicToolCall {
            id: "legacy-call".to_string(),
            namespace: None,
            tool: super::adaptive_evidence::CODEXDD_VALIDATION_TOOL_NAME.to_string(),
            arguments: serde_json::json!({
                "profile": "work_packet",
                "result": "pass",
                "head_sha": SOURCE_SHA_A,
            }),
            status: DynamicToolCallStatus::Completed,
            content_items: None,
            success: Some(true),
            duration_ms: Some(1),
        },
        thread_id: thread_id.to_string(),
        turn_id: "native-source-turn".to_string(),
        completed_at_ms: 1_000,
    };
    let mut registry = AdaptiveEvidenceRegistry::default();
    registry.register(record_from_item_completion(&legacy_notification).expect("legacy"));
    assert_eq!(
        registry.match_native_source_gate("legacy-call", &requirement),
        Err(AdaptiveEvidenceResolveError::MissingProvenance)
    );

    let mut unpinned =
        scoped_native_source_receipt(thread_id, root.path(), SOURCE_SHA_A, "work_packet");
    unpinned.worker_binding = None;
    registry.register(unpinned);
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &requirement),
        Err(AdaptiveEvidenceResolveError::WorkerBindingMismatch)
    );

    registry.register(record(
        "generic-success",
        thread_id,
        "native-source-turn",
        AdaptiveEvidenceOutcome::Success,
    ));
    assert_eq!(
        registry.match_native_source_gate("generic-success", &requirement),
        Err(AdaptiveEvidenceResolveError::MissingProvenance)
    );
}

#[test]
fn codexdd_046_3b3b_run_timestamp_and_turn_must_match_gate() {
    let root = tempfile::tempdir().expect("root");
    let thread_id = ThreadId::new();
    let mut registry = AdaptiveEvidenceRegistry::default();
    registry.register(scoped_native_source_receipt(
        thread_id,
        root.path(),
        SOURCE_SHA_A,
        "work_packet",
    ));
    let mut requirement =
        native_source_requirement(thread_id, root.path(), SOURCE_SHA_A, "work_packet");
    requirement.expected_run_id = Some("other-run");
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &requirement),
        Err(AdaptiveEvidenceResolveError::RunIdMismatch)
    );
    requirement.expected_run_id = Some("native-run-123");
    requirement.minimum_completed_at_ms = 1_001;
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &requirement),
        Err(AdaptiveEvidenceResolveError::StaleEvidence)
    );
    requirement.minimum_completed_at_ms = 999;
    requirement.source_turn_id = Some("other-turn");
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &requirement),
        Err(AdaptiveEvidenceResolveError::SourceTurnMismatch)
    );
}

#[test]
fn codexdd_046_3b3b_provenance_conflict_is_permanently_inadmissible() {
    let root = tempfile::tempdir().expect("root");
    let thread_id = ThreadId::new();
    let mut registry = AdaptiveEvidenceRegistry::default();
    let original =
        scoped_native_source_receipt(thread_id, root.path(), SOURCE_SHA_A, "work_packet");
    assert_eq!(
        registry.register(original.clone()),
        AdaptiveEvidenceRegistration::Registered
    );
    assert_eq!(
        registry.register(original.clone()),
        AdaptiveEvidenceRegistration::Duplicate
    );
    let conflicting = AdaptiveEvidenceRecord {
        native_source_provenance: original.native_source_provenance.as_ref().map(|p| {
            super::adaptive_evidence::AdaptiveNativeSourceProvenance {
                source_sha: SOURCE_SHA_B.to_string(),
                ..p.clone()
            }
        }),
        ..original.clone()
    };
    assert_eq!(
        registry.register(conflicting),
        AdaptiveEvidenceRegistration::Conflicted
    );
    let requirement =
        native_source_requirement(thread_id, root.path(), SOURCE_SHA_A, "work_packet");
    assert_eq!(
        registry.match_native_source_gate("native-lvo-evidence", &requirement),
        Err(AdaptiveEvidenceResolveError::Conflicted)
    );
    assert_eq!(
        registry.register(original),
        AdaptiveEvidenceRegistration::Conflicted
    );
}

#[test]
fn codexdd_046_3b3b_native_pass_without_exact_fields_cannot_bind() {
    let root = tempfile::tempdir().expect("root");
    let thread_id = ThreadId::new();
    for (idx, result, sha, run_id, finished) in [
        (0, "fail", SOURCE_SHA_A, "native-run-123", 1_000),
        (1, "pass", "not-a-sha", "native-run-123", 1_000),
        (2, "pass", SOURCE_SHA_A, "", 1_000),
        (3, "pass", SOURCE_SHA_A, "native-run-123", 0),
    ] {
        let notification = ItemCompletedNotification {
            item: ThreadItem::DynamicToolCall {
                id: format!("invalid-{idx}"),
                namespace: None,
                tool: super::adaptive_evidence::CODEXDD_VALIDATION_TOOL_NAME.to_string(),
                arguments: serde_json::json!({
                    "profile": "work_packet",
                    "result": result,
                    "run_id": run_id,
                    "head_sha": sha,
                    "source_root": root.path().display().to_string(),
                }),
                status: DynamicToolCallStatus::Completed,
                content_items: None,
                success: Some(true),
                duration_ms: Some(1),
            },
            thread_id: thread_id.to_string(),
            turn_id: "native-source-turn".to_string(),
            completed_at_ms: finished,
        };
        let mut record = record_from_item_completion(&notification).expect("diagnostic record");
        record.bind_worker_identity(
            &AdaptiveWorkerContext {
                role: AdaptiveWorkerRole::Implementation,
                authorized_scope: Some("isolated source packet".to_string()),
            },
            Some(&AdaptiveWorkerPreflight {
                task_kind: AdaptiveTaskKind::SourceChange,
                source_root: root.path().display().to_string(),
                source_sha: SOURCE_SHA_A.to_string(),
            }),
        );
        assert!(record.native_source_provenance.is_none());
        assert!(record.worker_binding.is_none());
    }
}

#[test]
fn codexdd_046_3b3c_a01_focused_pass_retains_required_full_gate() {
    let root = tempfile::tempdir().expect("isolated root");
    let worker = ThreadId::new();
    let mut registry = AdaptiveEvidenceRegistry::default();
    registry.register(scoped_native_source_receipt(
        worker,
        root.path(),
        SOURCE_SHA_A,
        "targeted",
    ));
    let mut gates = AdaptiveGateLedger::new(
        "isolated source packet",
        root.path(),
        SOURCE_SHA_A,
        vec![
            AdaptiveRequiredGate::SourceProfile("targeted".to_string()),
            AdaptiveRequiredGate::SourceProfile("release".to_string()),
            AdaptiveRequiredGate::IndependentValidation,
        ],
    )
    .expect("explicit requirements");
    gates
        .observe_source_pass(
            &registry,
            "targeted",
            "native-lvo-evidence",
            worker,
            "native-source-turn",
            "native-run-123",
            999,
        )
        .expect("focused source PASS");
    let pending = gates.outstanding(&registry);
    assert_eq!(pending.len(), 2);
    assert_eq!(
        pending[0].gate,
        AdaptiveRequiredGate::SourceProfile("release".to_string())
    );
    assert_eq!(pending[0].state, AdaptiveGateState::Pending);
    assert_eq!(pending[1].gate, AdaptiveRequiredGate::IndependentValidation);
    assert!(!gates.ready_for_owner_qa(&registry));
    assert_eq!(
        gates.observe_source_pass(
            &registry,
            "release",
            "native-lvo-evidence",
            worker,
            "native-source-turn",
            "native-run-123",
            999,
        ),
        Err(AdaptiveEvidenceResolveError::GateScopeMismatch)
    );
}

#[test]
fn codexdd_046_3b3c_a02_native_source_cannot_establish_live_service_health() {
    let root = tempfile::tempdir().expect("isolated root");
    let worker = ThreadId::new();
    let mut registry = AdaptiveEvidenceRegistry::default();
    registry.register(scoped_native_source_receipt(
        worker,
        root.path(),
        SOURCE_SHA_A,
        "work_packet",
    ));
    let mut gates = AdaptiveGateLedger::new(
        "isolated source packet",
        root.path(),
        SOURCE_SHA_A,
        vec![
            AdaptiveRequiredGate::SourceProfile("work_packet".to_string()),
            AdaptiveRequiredGate::OwnedRuntimeHealth(vec![4331, 4332]),
            AdaptiveRequiredGate::IndependentValidation,
        ],
    )
    .expect("explicit source and operations gates");
    gates
        .observe_source_pass(
            &registry,
            "work_packet",
            "native-lvo-evidence",
            worker,
            "native-source-turn",
            "native-run-123",
            999,
        )
        .expect("source profile passed");
    // Even a successful generic exec never becomes owned health evidence.
    registry.register(record(
        "unverified-health-exec",
        worker,
        "native-source-turn",
        AdaptiveEvidenceOutcome::Success,
    ));
    let outstanding = gates.outstanding(&registry);
    assert_eq!(outstanding.len(), 2);
    assert_eq!(
        outstanding[0].gate,
        AdaptiveRequiredGate::OwnedRuntimeHealth(vec![4331, 4332])
    );
    assert_eq!(outstanding[0].state, AdaptiveGateState::Pending);
    assert!(!gates.ready_for_owner_qa(&registry));
    gates
        .mark_blocked(
            &AdaptiveRequiredGate::OwnedRuntimeHealth(vec![4331, 4332]),
            "service_health_unavailable",
        )
        .expect("typed setup blocker");
    assert_eq!(
        gates.outstanding(&registry)[0].state,
        AdaptiveGateState::Blocked("service_health_unavailable".to_string())
    );
}

#[test]
fn codexdd_046_3b3c_a07_implementation_pass_is_not_independent_validation() {
    let root = tempfile::tempdir().expect("isolated root");
    let worker = ThreadId::new();
    let mut registry = AdaptiveEvidenceRegistry::default();
    let source = scoped_native_source_receipt(worker, root.path(), SOURCE_SHA_A, "work_packet");
    registry.register(source.clone());
    let mut gates = AdaptiveGateLedger::new(
        "isolated source packet",
        root.path(),
        SOURCE_SHA_A,
        vec![
            AdaptiveRequiredGate::SourceProfile("work_packet".to_string()),
            AdaptiveRequiredGate::IndependentValidation,
        ],
    )
    .expect("explicit independent Validator");
    gates
        .observe_source_pass(
            &registry,
            "work_packet",
            "native-lvo-evidence",
            worker,
            "native-source-turn",
            "native-run-123",
            999,
        )
        .expect("Implementation native source validation");
    assert_eq!(
        gates.outstanding(&registry)[0].gate,
        AdaptiveRequiredGate::IndependentValidation
    );
    assert!(!gates.ready_for_owner_qa(&registry));

    let mut conflicting = source;
    conflicting
        .native_source_provenance
        .as_mut()
        .expect("native source proof")
        .source_sha = SOURCE_SHA_B.to_string();
    assert_eq!(
        registry.register(conflicting),
        AdaptiveEvidenceRegistration::Conflicted
    );
    // Previously accepted evidence must be rechecked, not cached forever.
    assert_eq!(gates.outstanding(&registry).len(), 2);
}

#[test]
fn codexdd_046_3b3c_requirements_reject_missing_or_duplicate_authority() {
    let root = tempfile::tempdir().expect("isolated root");
    let cases = vec![
        vec![AdaptiveRequiredGate::SourceProfile("targeted".to_string())],
        vec![
            AdaptiveRequiredGate::IndependentValidation,
            AdaptiveRequiredGate::IndependentValidation,
        ],
        vec![
            AdaptiveRequiredGate::SourceProfile("not-a-profile".to_string()),
            AdaptiveRequiredGate::IndependentValidation,
        ],
        vec![
            AdaptiveRequiredGate::OwnedRuntimeHealth(vec![4331, 4331]),
            AdaptiveRequiredGate::IndependentValidation,
        ],
        vec![
            AdaptiveRequiredGate::OwnedRuntimeHealth(Vec::new()),
            AdaptiveRequiredGate::IndependentValidation,
        ],
    ];
    for requirements in cases {
        assert!(
            AdaptiveGateLedger::new(
                "isolated source packet",
                root.path(),
                SOURCE_SHA_A,
                requirements
            )
            .is_err()
        );
    }
    assert!(
        AdaptiveGateLedger::new(
            " ",
            root.path(),
            SOURCE_SHA_A,
            vec![AdaptiveRequiredGate::IndependentValidation]
        )
        .is_err()
    );
}


#[test]
fn codexdd_046_3b3d_checkpoint_roundtrip_drops_all_success_authority() {
    let root = tempfile::tempdir().expect("isolated root");
    let source_worker = ThreadId::new();
    let owner_thread = ThreadId::new();
    let mut registry = AdaptiveEvidenceRegistry::default();
    registry.register(scoped_native_source_receipt(
        source_worker,
        root.path(),
        SOURCE_SHA_A,
        "work_packet",
    ));
    let mut ledger = AdaptiveGateLedger::new(
        "isolated source packet",
        root.path(),
        SOURCE_SHA_A,
        vec![
            AdaptiveRequiredGate::SourceProfile("work_packet".to_string()),
            AdaptiveRequiredGate::IndependentValidation,
        ],
    )
    .expect("required gates");
    ledger
        .observe_source_pass(
            &registry,
            "work_packet",
            "native-lvo-evidence",
            source_worker,
            "native-source-turn",
            "native-run-123",
            999,
        )
        .expect("source LVO green");
    assert_eq!(ledger.outstanding(&registry).len(), 1);
    let checkpoint = ledger
        .checkpoint_for_worker(owner_thread, AdaptiveWorkerRole::Validation)
        .expect("bound Validator checkpoint");
    let json = serde_json::to_string(&checkpoint).expect("checkpoint JSON");
    assert!(!json.contains("native-lvo-evidence"));
    assert!(!json.contains("native-run-123"));
    assert!(!json.contains("sourceRefs"));
    let decoded: AdaptiveGateCheckpoint = serde_json::from_str(&json).expect("round trip");
    assert_eq!(decoded, checkpoint);
    let restored = AdaptiveGateLedger::restore_pending(
        decoded,
        owner_thread,
        AdaptiveWorkerRole::Validation,
        "isolated source packet",
        root.path(),
        SOURCE_SHA_A,
    )
    .expect("matching Worker and source");
    let outstanding = restored.outstanding(&registry);
    assert_eq!(outstanding.len(), 2);
    assert_eq!(
        outstanding[0].gate,
        AdaptiveRequiredGate::SourceProfile("work_packet".to_string())
    );
    assert_eq!(outstanding[0].state, AdaptiveGateState::Pending);
    assert_eq!(
        outstanding[1].gate,
        AdaptiveRequiredGate::IndependentValidation
    );
    assert!(!restored.ready_for_owner_qa(&registry));
}

#[test]
fn codexdd_046_3b3d_rejects_cross_worker_prior_sha_and_wrong_worktree() {
    let root = tempfile::tempdir().expect("source");
    let other_root = tempfile::tempdir().expect("foreign source");
    let owner = ThreadId::new();
    let other_thread = ThreadId::new();
    let scope = "isolated source packet";
    let ledger = AdaptiveGateLedger::new(
        scope,
        root.path(),
        SOURCE_SHA_A,
        vec![AdaptiveRequiredGate::IndependentValidation],
    )
    .expect("bounded gates");
    assert!(ledger
        .checkpoint_for_worker(owner, AdaptiveWorkerRole::Unspecified)
        .is_err());
    let checkpoint = ledger
        .checkpoint_for_worker(owner, AdaptiveWorkerRole::Validation)
        .expect("bound Worker");
    for (thread, role, expected_scope, expected_root, sha) in [
        (other_thread, AdaptiveWorkerRole::Validation, scope, root.path(), SOURCE_SHA_A),
        (owner, AdaptiveWorkerRole::Implementation, scope, root.path(), SOURCE_SHA_A),
        (owner, AdaptiveWorkerRole::Validation, "foreign packet", root.path(), SOURCE_SHA_A),
        (owner, AdaptiveWorkerRole::Validation, scope, other_root.path(), SOURCE_SHA_A),
        (owner, AdaptiveWorkerRole::Validation, scope, root.path(), SOURCE_SHA_B),
    ] {
        assert!(AdaptiveGateLedger::restore_pending(
            checkpoint.clone(),
            thread,
            role,
            expected_scope,
            expected_root,
            sha,
        )
        .is_err());
    }
}

#[test]
fn codexdd_046_3b3d_restores_only_valid_pending_blockers_and_fails_closed() {
    let root = tempfile::tempdir().expect("source root");
    let owner = ThreadId::new();
    let health_gate = AdaptiveRequiredGate::OwnedRuntimeHealth(vec![4331, 4332]);
    let mut ledger = AdaptiveGateLedger::new(
        "isolated source packet",
        root.path(),
        SOURCE_SHA_A,
        vec![
            AdaptiveRequiredGate::SourceProfile("work_packet".to_string()),
            health_gate.clone(),
            AdaptiveRequiredGate::IndependentValidation,
        ],
    )
    .expect("gate list");
    ledger
        .mark_blocked(&health_gate, "service_health_missing")
        .expect("typed blocker");
    let checkpoint = ledger
        .checkpoint_for_worker(owner, AdaptiveWorkerRole::Validation)
        .expect("checkpoint");
    let restored = AdaptiveGateLedger::restore_pending(
        checkpoint.clone(),
        owner,
        AdaptiveWorkerRole::Validation,
        "isolated source packet",
        root.path(),
        SOURCE_SHA_A,
    )
    .expect("bound pending restore");
    let outstanding = restored.outstanding(&AdaptiveEvidenceRegistry::default());
    assert_eq!(outstanding.len(), 3);
    assert_eq!(outstanding[0].state, AdaptiveGateState::Pending);
    assert_eq!(
        outstanding[1].state,
        AdaptiveGateState::Blocked("service_health_missing".to_string())
    );
    assert_eq!(outstanding[2].state, AdaptiveGateState::Pending);

    let mut altered = checkpoint.clone();
    altered.schema_version = 42;
    assert!(AdaptiveGateLedger::restore_pending(
        altered,
        owner,
        AdaptiveWorkerRole::Validation,
        "isolated source packet",
        root.path(),
        SOURCE_SHA_A,
    )
    .is_err());
    let mut altered = checkpoint.clone();
    altered.required_gates.clear();
    assert!(AdaptiveGateLedger::restore_pending(
        altered,
        owner,
        AdaptiveWorkerRole::Validation,
        "isolated source packet",
        root.path(),
        SOURCE_SHA_A,
    )
    .is_err());
    let mut altered = checkpoint.clone();
    altered.blocked_gates.push((health_gate, "duplicate_blocker".to_string()));
    assert!(AdaptiveGateLedger::restore_pending(
        altered,
        owner,
        AdaptiveWorkerRole::Validation,
        "isolated source packet",
        root.path(),
        SOURCE_SHA_A,
    )
    .is_err());
    let mut json = serde_json::to_value(checkpoint).expect("checkpoint JSON");
    json["foreignAuthority"] = serde_json::json!("not approved");
    assert!(serde_json::from_value::<AdaptiveGateCheckpoint>(json).is_err());
}
