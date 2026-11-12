//! Typed, thread-bound adaptive evidence derived from native tool completion items.

use std::collections::BTreeMap;
use std::path::Path;

use crate::adaptive_worker::AdaptiveTaskKind;
use crate::adaptive_worker::AdaptiveWorkerContext;
use crate::adaptive_worker::AdaptiveWorkerPreflight;
use crate::adaptive_worker::AdaptiveWorkerRole;
use codex_app_server_protocol::CommandExecutionStatus;
use codex_app_server_protocol::DynamicToolCallStatus;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::McpToolCallStatus;
use codex_app_server_protocol::ThreadItem;
use codex_protocol::ThreadId;

pub(crate) const ADAPTIVE_FAILURE_PRESSURE_THRESHOLD: usize = 2;
pub(crate) const AUTO_FAILURE_PRESSURE_DIAGNOSTIC: &str = "native_failure_pressure_threshold";
pub(crate) const CODEXDD_VALIDATION_TOOL_NAME: &str = "run_codexdd_validation";
/// Reserved for a Core-owned, Validation-only native producer (not yet registered).
/// A model report, shell command or external tool cannot mint this profile.
pub(crate) const CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME: &str =
    "run_codexdd_independent_validation";
pub(crate) const CODEXDD_INDEPENDENT_VALIDATION_PROFILE: &str = "independent_review";
pub(crate) const CODEXDD_WORK_PACKET_PROFILE: &str = "work_packet";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveEvidenceOutcome {
    Success,
    Failure,
    Cancelled,
    Incomplete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveEvidenceKind {
    CommandExecution,
    McpToolCall,
    DynamicToolCall,
}

/// Facts emitted by the built-in native LVO handler only on a complete PASS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveNativeSourceProvenance {
    pub(crate) source_root: String,
    pub(crate) source_sha: String,
    pub(crate) run_id: String,
    pub(crate) completed_at_ms: u64,
}

/// Worker identity captured from immutable first-assignment state on receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveEvidenceWorkerBinding {
    pub(crate) role: AdaptiveWorkerRole,
    pub(crate) authorized_scope: String,
    pub(crate) task_kind: AdaptiveTaskKind,
    pub(crate) source_root: String,
    pub(crate) source_sha: String,
}

/// Expectations must come from an authorized gate, not from the success itself.
/// Expected identity for a native review produced in a *different* Worker
/// thread. A raw Validation agent's green exec is not this receipt.
pub(crate) struct AdaptiveIndependentReviewRequirement<'a> {
    pub(crate) validation_thread_id: ThreadId,
    pub(crate) implementation_thread_id: ThreadId,
    pub(crate) validation_turn_id: &'a str,
    pub(crate) authorized_scope: &'a str,
    pub(crate) source_root: &'a Path,
    pub(crate) source_sha: &'a str,
    pub(crate) expected_run_id: &'a str,
    pub(crate) minimum_completed_at_ms: u64,
}

pub(crate) struct AdaptiveSourceGateRequirement<'a> {
    pub(crate) thread_id: ThreadId,
    pub(crate) worker_role: AdaptiveWorkerRole,
    pub(crate) task_kind: AdaptiveTaskKind,
    pub(crate) authorized_scope: &'a str,
    pub(crate) source_root: &'a Path,
    pub(crate) source_sha: &'a str,
    pub(crate) required_profile: &'a str,
    pub(crate) source_turn_id: Option<&'a str>,
    pub(crate) expected_run_id: Option<&'a str>,
    pub(crate) minimum_completed_at_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveEvidenceRecord {
    pub(crate) evidence_id: String,
    pub(crate) thread_id: ThreadId,
    pub(crate) source_turn_id: String,
    pub(crate) outcome: AdaptiveEvidenceOutcome,
    pub(crate) kind: AdaptiveEvidenceKind,
    pub(crate) tool_name: Option<String>,
    pub(crate) validation_profile: Option<String>,
    pub(crate) validation_failure_fingerprint: Option<String>,
    pub(crate) native_source_provenance: Option<AdaptiveNativeSourceProvenance>,
    pub(crate) worker_binding: Option<AdaptiveEvidenceWorkerBinding>,
}

impl AdaptiveEvidenceRecord {
    /// Snapshot authoritative first-assignment identity only for a genuine
    /// native-source receipt. Unpinned/legacy Workers cannot authorize it.
    pub(crate) fn bind_worker_identity(
        &mut self,
        worker: &AdaptiveWorkerContext,
        preflight: Option<&AdaptiveWorkerPreflight>,
    ) {
        let (Some(_native), Some(pin), Some(scope)) = (
            self.native_source_provenance.as_ref(),
            preflight,
            worker.authorized_scope.as_deref(),
        ) else {
            return;
        };
        let correct_native_producer = matches!(
            (worker.role, self.tool_name.as_deref()),
            (
                AdaptiveWorkerRole::Implementation,
                Some(CODEXDD_VALIDATION_TOOL_NAME)
            ) | (
                AdaptiveWorkerRole::Validation,
                Some(CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME)
            )
        );
        if !correct_native_producer
            || pin.task_kind != AdaptiveTaskKind::SourceChange
            || scope.trim().is_empty()
            || pin.source_sha.len() != 40
            || !pin.source_sha.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return;
        }
        self.worker_binding = Some(AdaptiveEvidenceWorkerBinding {
            role: worker.role,
            authorized_scope: scope.to_string(),
            task_kind: pin.task_kind,
            source_root: pin.source_root.clone(),
            source_sha: pin.source_sha.clone(),
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum AdaptiveEvidenceEntry {
    Valid(AdaptiveEvidenceRecord),
    Conflicted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveEvidenceRegistration {
    Registered,
    Duplicate,
    Conflicted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveEvidenceResolveError {
    NotFound,
    Conflicted,
    CrossThread,
    SourceTurnMismatch,
    OutcomeMismatch,
    MissingProvenance,
    WorkerBindingMismatch,
    SourceIdentityMismatch,
    GateScopeMismatch,
    RunIdMismatch,
    StaleEvidence,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct AdaptiveEvidenceRegistry {
    entries: BTreeMap<String, AdaptiveEvidenceEntry>,
}

impl AdaptiveEvidenceRegistry {
    pub(crate) fn register(
        &mut self,
        record: AdaptiveEvidenceRecord,
    ) -> AdaptiveEvidenceRegistration {
        match self.entries.get(&record.evidence_id) {
            None => {
                self.entries.insert(
                    record.evidence_id.clone(),
                    AdaptiveEvidenceEntry::Valid(record),
                );
                AdaptiveEvidenceRegistration::Registered
            }
            Some(AdaptiveEvidenceEntry::Valid(existing)) if existing == &record => {
                AdaptiveEvidenceRegistration::Duplicate
            }
            Some(AdaptiveEvidenceEntry::Valid(_)) => {
                self.entries
                    .insert(record.evidence_id, AdaptiveEvidenceEntry::Conflicted);
                AdaptiveEvidenceRegistration::Conflicted
            }
            Some(AdaptiveEvidenceEntry::Conflicted) => AdaptiveEvidenceRegistration::Conflicted,
        }
    }

    pub(crate) fn resolve(
        &self,
        evidence_id: &str,
    ) -> Result<&AdaptiveEvidenceRecord, AdaptiveEvidenceResolveError> {
        match self.entries.get(evidence_id) {
            Some(AdaptiveEvidenceEntry::Valid(record)) => Ok(record),
            Some(AdaptiveEvidenceEntry::Conflicted) => {
                Err(AdaptiveEvidenceResolveError::Conflicted)
            }
            None => Err(AdaptiveEvidenceResolveError::NotFound),
        }
    }

    pub(crate) fn resolve_for_thread(
        &self,
        evidence_id: &str,
        thread_id: ThreadId,
    ) -> Result<&AdaptiveEvidenceRecord, AdaptiveEvidenceResolveError> {
        let record = self.resolve(evidence_id)?;
        if record.thread_id != thread_id {
            return Err(AdaptiveEvidenceResolveError::CrossThread);
        }
        Ok(record)
    }

    pub(crate) fn resolve_for_turn(
        &self,
        evidence_id: &str,
        thread_id: ThreadId,
        source_turn_id: &str,
    ) -> Result<&AdaptiveEvidenceRecord, AdaptiveEvidenceResolveError> {
        let record = self.resolve_for_thread(evidence_id, thread_id)?;
        if record.source_turn_id != source_turn_id {
            return Err(AdaptiveEvidenceResolveError::SourceTurnMismatch);
        }
        Ok(record)
    }

    pub(crate) fn validate_outcome(
        &self,
        evidence_id: &str,
        thread_id: ThreadId,
        expected: AdaptiveEvidenceOutcome,
    ) -> Result<&AdaptiveEvidenceRecord, AdaptiveEvidenceResolveError> {
        let record = self.resolve_for_thread(evidence_id, thread_id)?;
        if record.outcome != expected {
            return Err(AdaptiveEvidenceResolveError::OutcomeMismatch);
        }
        Ok(record)
    }

    /// Strict source-gate proof: successful commands and stale/partial LVO
    /// receipts never substitute for exact native profile evidence.
    pub(crate) fn match_native_source_gate<'a>(
        &'a self,
        evidence_id: &str,
        requirement: &AdaptiveSourceGateRequirement<'_>,
    ) -> Result<&'a AdaptiveEvidenceRecord, AdaptiveEvidenceResolveError> {
        let record = match requirement.source_turn_id {
            Some(turn_id) => self.resolve_for_turn(evidence_id, requirement.thread_id, turn_id)?,
            None => self.resolve_for_thread(evidence_id, requirement.thread_id)?,
        };
        if record.outcome != AdaptiveEvidenceOutcome::Success {
            return Err(AdaptiveEvidenceResolveError::OutcomeMismatch);
        }
        if record.kind != AdaptiveEvidenceKind::DynamicToolCall
            || record.tool_name.as_deref() != Some(CODEXDD_VALIDATION_TOOL_NAME)
        {
            return Err(AdaptiveEvidenceResolveError::MissingProvenance);
        }
        let native = record
            .native_source_provenance
            .as_ref()
            .ok_or(AdaptiveEvidenceResolveError::MissingProvenance)?;
        let bound = record
            .worker_binding
            .as_ref()
            .ok_or(AdaptiveEvidenceResolveError::WorkerBindingMismatch)?;
        if bound.role != AdaptiveWorkerRole::Implementation
            || bound.role != requirement.worker_role
            || bound.task_kind != requirement.task_kind
            || bound.task_kind != AdaptiveTaskKind::SourceChange
            || bound.authorized_scope != requirement.authorized_scope
            || bound.authorized_scope.trim().is_empty()
        {
            return Err(AdaptiveEvidenceResolveError::WorkerBindingMismatch);
        }
        if requirement.source_sha.len() != 40
            || !requirement
                .source_sha
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
            || !bound
                .source_sha
                .eq_ignore_ascii_case(requirement.source_sha)
            || !native
                .source_sha
                .eq_ignore_ascii_case(requirement.source_sha)
            || !same_canonical_root(&bound.source_root, requirement.source_root)
            || !same_canonical_root(&native.source_root, requirement.source_root)
        {
            return Err(AdaptiveEvidenceResolveError::SourceIdentityMismatch);
        }
        if record.validation_profile.as_deref() != Some(requirement.required_profile)
            || !matches!(
                requirement.required_profile,
                "targeted" | CODEXDD_WORK_PACKET_PROFILE | "release"
            )
        {
            return Err(AdaptiveEvidenceResolveError::GateScopeMismatch);
        }
        if requirement
            .expected_run_id
            .is_some_and(|run_id| run_id != native.run_id)
        {
            return Err(AdaptiveEvidenceResolveError::RunIdMismatch);
        }
        if native.completed_at_ms < requirement.minimum_completed_at_ms {
            return Err(AdaptiveEvidenceResolveError::StaleEvidence);
        }
        Ok(record)
    }

    /// Contract for a separately bound Core native Validator producer.
    /// This is deliberately not called from READY yet: the native producer
    /// and authenticated cross-Worker gate handoff are separate work packets.
    pub(crate) fn match_native_independent_review<'a>(
        &'a self,
        evidence_id: &str,
        requirement: &AdaptiveIndependentReviewRequirement<'_>,
    ) -> Result<&'a AdaptiveEvidenceRecord, AdaptiveEvidenceResolveError> {
        if requirement.validation_thread_id == requirement.implementation_thread_id
            || requirement.validation_turn_id.trim().is_empty()
            || requirement.expected_run_id.trim().is_empty()
        {
            return Err(AdaptiveEvidenceResolveError::WorkerBindingMismatch);
        }
        let record = self.resolve_for_turn(
            evidence_id,
            requirement.validation_thread_id,
            requirement.validation_turn_id,
        )?;
        if record.outcome != AdaptiveEvidenceOutcome::Success {
            return Err(AdaptiveEvidenceResolveError::OutcomeMismatch);
        }
        if record.kind != AdaptiveEvidenceKind::DynamicToolCall
            || record.tool_name.as_deref() != Some(CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME)
            || record.validation_profile.as_deref() != Some(CODEXDD_INDEPENDENT_VALIDATION_PROFILE)
        {
            return Err(AdaptiveEvidenceResolveError::MissingProvenance);
        }
        let native = record
            .native_source_provenance
            .as_ref()
            .ok_or(AdaptiveEvidenceResolveError::MissingProvenance)?;
        let bound = record
            .worker_binding
            .as_ref()
            .ok_or(AdaptiveEvidenceResolveError::WorkerBindingMismatch)?;
        if bound.role != AdaptiveWorkerRole::Validation
            || bound.task_kind != AdaptiveTaskKind::SourceChange
            || bound.authorized_scope != requirement.authorized_scope
            || bound.authorized_scope.trim().is_empty()
        {
            return Err(AdaptiveEvidenceResolveError::WorkerBindingMismatch);
        }
        if requirement.source_sha.len() != 40
            || !requirement
                .source_sha
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
            || !bound
                .source_sha
                .eq_ignore_ascii_case(requirement.source_sha)
            || !native
                .source_sha
                .eq_ignore_ascii_case(requirement.source_sha)
            || !same_canonical_root(&bound.source_root, requirement.source_root)
            || !same_canonical_root(&native.source_root, requirement.source_root)
        {
            return Err(AdaptiveEvidenceResolveError::SourceIdentityMismatch);
        }
        if native.run_id != requirement.expected_run_id {
            return Err(AdaptiveEvidenceResolveError::RunIdMismatch);
        }
        if native.completed_at_ms < requirement.minimum_completed_at_ms
            || native.completed_at_ms == 0
        {
            return Err(AdaptiveEvidenceResolveError::StaleEvidence);
        }
        Ok(record)
    }

    fn has_successful_codexdd_profile_receipt_for_turn(
        &self,
        evidence_refs: &[String],
        thread_id: ThreadId,
        source_turn_id: &str,
        profile: &str,
    ) -> bool {
        !evidence_refs.is_empty()
            && evidence_refs.iter().any(|evidence_id| {
                self.resolve_for_turn(evidence_id, thread_id, source_turn_id)
                    .is_ok_and(|record| {
                        record.outcome == AdaptiveEvidenceOutcome::Success
                            && record.kind == AdaptiveEvidenceKind::DynamicToolCall
                            && record.tool_name.as_deref() == Some(CODEXDD_VALIDATION_TOOL_NAME)
                            && record.validation_profile.as_deref() == Some(profile)
                    })
            })
    }

    pub(crate) fn has_successful_codexdd_work_packet_receipt_for_turn(
        &self,
        evidence_refs: &[String],
        thread_id: ThreadId,
        source_turn_id: &str,
    ) -> bool {
        self.has_successful_codexdd_profile_receipt_for_turn(
            evidence_refs,
            thread_id,
            source_turn_id,
            CODEXDD_WORK_PACKET_PROFILE,
        )
    }

    pub(crate) fn has_successful_codexdd_targeted_receipt_for_turn(
        &self,
        evidence_refs: &[String],
        thread_id: ThreadId,
        source_turn_id: &str,
    ) -> bool {
        self.has_successful_codexdd_profile_receipt_for_turn(
            evidence_refs,
            thread_id,
            source_turn_id,
            "targeted",
        )
    }

    /// Only the native LVO runner can count as progress in protected Mechanical
    /// Validation. Inspection commands and ad-hoc tests cannot substitute for
    /// a structured validation receipt or extend an unavailable-tool loop.
    pub(crate) fn has_codexdd_validation_run_for_turn(
        &self,
        thread_id: ThreadId,
        source_turn_id: &str,
    ) -> bool {
        self.entries.values().any(|entry| {
            matches!(
                entry,
                AdaptiveEvidenceEntry::Valid(record)
                    if record.thread_id == thread_id
                        && record.source_turn_id == source_turn_id
                        && matches!(
                            record.outcome,
                            AdaptiveEvidenceOutcome::Success | AdaptiveEvidenceOutcome::Failure
                        )
                        && record.kind == AdaptiveEvidenceKind::DynamicToolCall
                        && record.tool_name.as_deref() == Some(CODEXDD_VALIDATION_TOOL_NAME)
            )
        })
    }

    /// Infrastructure/profile errors never grant repair authority or model
    /// escalation. A completed built-in LVO call with a failed outcome and no
    /// product-failure fingerprint is a deterministic nonrepairable blocker.
    /// Scope this to the current Worker turn and the two mechanical profiles.
    pub(crate) fn has_nonrepairable_codexdd_validation_error_for_turn(
        &self,
        thread_id: ThreadId,
        source_turn_id: &str,
    ) -> bool {
        self.entries.values().any(|entry| {
            let AdaptiveEvidenceEntry::Valid(record) = entry else {
                return false;
            };
            record.thread_id == thread_id
                && record.source_turn_id == source_turn_id
                && record.outcome == AdaptiveEvidenceOutcome::Failure
                && record.kind == AdaptiveEvidenceKind::DynamicToolCall
                && record.tool_name.as_deref() == Some(CODEXDD_VALIDATION_TOOL_NAME)
                && matches!(
                    record.validation_profile.as_deref(),
                    Some("targeted" | CODEXDD_WORK_PACKET_PROFILE)
                )
                && record.validation_failure_fingerprint.is_none()
        })
    }

    pub(crate) fn codexdd_repairable_validation_failure_for_turn(
        &self,
        evidence_refs: &[String],
        thread_id: ThreadId,
        source_turn_id: &str,
    ) -> Option<(String, String)> {
        let mut targeted = None;
        for evidence_id in evidence_refs {
            let Ok(record) = self.resolve_for_turn(evidence_id, thread_id, source_turn_id) else {
                continue;
            };
            if record.outcome != AdaptiveEvidenceOutcome::Failure
                || record.kind != AdaptiveEvidenceKind::DynamicToolCall
                || record.tool_name.as_deref() != Some(CODEXDD_VALIDATION_TOOL_NAME)
            {
                continue;
            }
            let (Some(profile), Some(fingerprint)) = (
                record.validation_profile.as_deref(),
                record.validation_failure_fingerprint.clone(),
            ) else {
                continue;
            };
            if profile == CODEXDD_WORK_PACKET_PROFILE {
                return Some((profile.to_string(), fingerprint));
            }
            if profile == "targeted" && targeted.is_none() {
                targeted = Some((profile.to_string(), fingerprint));
            }
        }
        targeted
    }

    pub(crate) fn failure_count_for_turn(
        &self,
        thread_id: ThreadId,
        source_turn_id: &str,
    ) -> usize {
        self.entries
            .values()
            .filter(|entry| {
                matches!(
                    entry,
                    AdaptiveEvidenceEntry::Valid(record)
                        if record.thread_id == thread_id
                            && record.source_turn_id == source_turn_id
                            && record.outcome == AdaptiveEvidenceOutcome::Failure
                )
            })
            .count()
    }

    pub(crate) fn failure_pressure_for_turn(
        &self,
        thread_id: ThreadId,
        source_turn_id: &str,
    ) -> usize {
        self.failure_count_for_turn(thread_id, source_turn_id)
            .min(ADAPTIVE_FAILURE_PRESSURE_THRESHOLD)
    }

    pub(crate) fn conclusive_refs_for_turn(
        &self,
        thread_id: ThreadId,
        source_turn_id: &str,
        expected: AdaptiveEvidenceOutcome,
    ) -> Vec<String> {
        self.entries
            .values()
            .filter_map(|entry| match entry {
                AdaptiveEvidenceEntry::Valid(record)
                    if record.thread_id == thread_id
                        && record.source_turn_id == source_turn_id
                        && record.outcome == expected =>
                {
                    Some(record.evidence_id.clone())
                }
                _ => None,
            })
            .collect()
    }

    pub(crate) fn has_conclusive_evidence_for_turn(
        &self,
        thread_id: ThreadId,
        source_turn_id: &str,
    ) -> bool {
        self.entries.values().any(|entry| {
            matches!(
                entry,
                AdaptiveEvidenceEntry::Valid(record)
                    if record.thread_id == thread_id
                        && record.source_turn_id == source_turn_id
                        && matches!(
                            record.outcome,
                            AdaptiveEvidenceOutcome::Success | AdaptiveEvidenceOutcome::Failure
                        )
            )
        })
    }

    /// Control-plane tools can finish successfully while the Implementation
    /// Worker makes no actual progress. Do not let tool discovery, status
    /// queries or repeated signal attempts reset the no-progress watchdog.
    pub(crate) fn has_native_implementation_work_for_turn(
        &self,
        thread_id: ThreadId,
        source_turn_id: &str,
    ) -> bool {
        self.entries.values().any(|entry| {
            let AdaptiveEvidenceEntry::Valid(record) = entry else {
                return false;
            };
            if record.thread_id != thread_id
                || record.source_turn_id != source_turn_id
                || !matches!(
                    record.outcome,
                    AdaptiveEvidenceOutcome::Success | AdaptiveEvidenceOutcome::Failure
                )
            {
                return false;
            }
            match record.kind {
                AdaptiveEvidenceKind::CommandExecution | AdaptiveEvidenceKind::McpToolCall => true,
                AdaptiveEvidenceKind::DynamicToolCall => {
                    record.tool_name.as_deref().is_some_and(|name| {
                        !matches!(
                            name,
                            "report_adaptive_signal"
                                | "get_context_remaining"
                                | "update_plan"
                                | "tool_search"
                                | "current_time"
                                | "sleep"
                                | "send_message_to_user"
                        )
                    })
                }
            }
        })
    }

    pub(crate) fn conclusive_refs_for_thread(
        &self,
        thread_id: ThreadId,
        expected: AdaptiveEvidenceOutcome,
    ) -> Vec<String> {
        self.entries
            .values()
            .filter_map(|entry| match entry {
                AdaptiveEvidenceEntry::Valid(record)
                    if record.thread_id == thread_id && record.outcome == expected =>
                {
                    Some(record.evidence_id.clone())
                }
                _ => None,
            })
            .collect()
    }

    pub(crate) fn has_conclusive_evidence_for_thread(&self, thread_id: ThreadId) -> bool {
        self.entries.values().any(|entry| {
            matches!(
                entry,
                AdaptiveEvidenceEntry::Valid(record)
                    if record.thread_id == thread_id
                        && matches!(
                            record.outcome,
                            AdaptiveEvidenceOutcome::Success | AdaptiveEvidenceOutcome::Failure
                        )
            )
        })
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }
}

pub(crate) fn record_from_item_completion(
    notification: &ItemCompletedNotification,
) -> Option<AdaptiveEvidenceRecord> {
    let thread_id = ThreadId::from_string(&notification.thread_id).ok()?;
    let (evidence_id, outcome, kind, tool_name, validation_profile, validation_failure_fingerprint) =
        match &notification.item {
            ThreadItem::CommandExecution { id, status, .. } => (
                id.clone(),
                match status {
                    CommandExecutionStatus::Completed => AdaptiveEvidenceOutcome::Success,
                    CommandExecutionStatus::Failed => AdaptiveEvidenceOutcome::Failure,
                    CommandExecutionStatus::Declined => AdaptiveEvidenceOutcome::Cancelled,
                    CommandExecutionStatus::InProgress => AdaptiveEvidenceOutcome::Incomplete,
                },
                AdaptiveEvidenceKind::CommandExecution,
                None,
                None,
                None,
            ),
            ThreadItem::McpToolCall { id, status, .. } => (
                id.clone(),
                match status {
                    McpToolCallStatus::Completed => AdaptiveEvidenceOutcome::Success,
                    McpToolCallStatus::Failed => AdaptiveEvidenceOutcome::Failure,
                    McpToolCallStatus::InProgress => AdaptiveEvidenceOutcome::Incomplete,
                },
                AdaptiveEvidenceKind::McpToolCall,
                None,
                None,
                None,
            ),
            ThreadItem::DynamicToolCall {
                id,
                namespace,
                tool,
                arguments,
                status,
                success,
                ..
            } => (
                id.clone(),
                match (status, success) {
                    (DynamicToolCallStatus::Completed, Some(true)) => {
                        AdaptiveEvidenceOutcome::Success
                    }
                    (
                        DynamicToolCallStatus::Completed | DynamicToolCallStatus::Failed,
                        Some(false),
                    )
                    | (DynamicToolCallStatus::Failed, _) => AdaptiveEvidenceOutcome::Failure,
                    (DynamicToolCallStatus::InProgress, _)
                    | (DynamicToolCallStatus::Completed, None) => {
                        AdaptiveEvidenceOutcome::Incomplete
                    }
                },
                AdaptiveEvidenceKind::DynamicToolCall,
                Some(tool.clone()),
                (namespace.is_none()
                    && matches!(
                        tool.as_str(),
                        CODEXDD_VALIDATION_TOOL_NAME | CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME
                    ))
                .then(|| {
                    arguments
                        .get("profile")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_string)
                })
                .flatten(),
                (namespace.is_none() && tool == CODEXDD_VALIDATION_TOOL_NAME)
                    .then(|| {
                        arguments
                            .get("failure_fingerprint")
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_string)
                    })
                    .flatten(),
            ),
            _ => return None,
        };
    Some(AdaptiveEvidenceRecord {
        evidence_id,
        thread_id,
        source_turn_id: notification.turn_id.clone(),
        outcome,
        kind,
        tool_name,
        validation_profile,
        validation_failure_fingerprint,
        native_source_provenance: native_source_provenance(notification),
        worker_binding: None,
    })
}

fn same_canonical_root(observed: &str, required: &Path) -> bool {
    Path::new(observed)
        .canonicalize()
        .ok()
        .zip(required.canonicalize().ok())
        .is_some_and(|(observed, required)| observed == required)
}

/// Legacy or partial tool items are still recorded for diagnostics but cannot
/// carry source gate authority. Only a full native PASS has these facts.
fn native_source_provenance(
    notification: &ItemCompletedNotification,
) -> Option<AdaptiveNativeSourceProvenance> {
    let ThreadItem::DynamicToolCall {
        namespace,
        tool,
        arguments,
        status,
        success,
        ..
    } = &notification.item
    else {
        return None;
    };
    let profile = arguments.get("profile").and_then(serde_json::Value::as_str);
    let expected_native_profile = match tool.as_str() {
        CODEXDD_VALIDATION_TOOL_NAME => {
            matches!(
                profile,
                Some("targeted" | CODEXDD_WORK_PACKET_PROFILE | "release")
            )
        }
        CODEXDD_INDEPENDENT_VALIDATION_TOOL_NAME => {
            profile == Some(CODEXDD_INDEPENDENT_VALIDATION_PROFILE)
        }
        _ => false,
    };
    if namespace.is_some()
        || !expected_native_profile
        || *status != DynamicToolCallStatus::Completed
        || *success != Some(true)
        || arguments.get("result").and_then(serde_json::Value::as_str) != Some("pass")
    {
        return None;
    }
    let required = |name| {
        arguments
            .get(name)
            .and_then(serde_json::Value::as_str)
            .filter(|value| !value.trim().is_empty())
    };
    let source_root = required("source_root")?;
    let source_sha = required("head_sha")?;
    let run_id = required("run_id")?;
    let completed_at_ms = u64::try_from(notification.completed_at_ms).ok()?;
    if !Path::new(source_root).is_absolute()
        || source_root.len() > 4096
        || source_sha.len() != 40
        || !source_sha.bytes().all(|byte| byte.is_ascii_hexdigit())
        || run_id.len() > 256
        || completed_at_ms == 0
    {
        return None;
    }
    Some(AdaptiveNativeSourceProvenance {
        source_root: source_root.to_string(),
        source_sha: source_sha.to_ascii_lowercase(),
        run_id: run_id.to_string(),
        completed_at_ms,
    })
}
