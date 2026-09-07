//! Typed, thread-bound adaptive evidence derived from native tool completion items.

use std::collections::BTreeMap;

use codex_app_server_protocol::CommandExecutionStatus;
use codex_app_server_protocol::DynamicToolCallStatus;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::McpToolCallStatus;
use codex_app_server_protocol::ThreadItem;
use codex_protocol::ThreadId;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveEvidenceRecord {
    pub(crate) evidence_id: String,
    pub(crate) thread_id: ThreadId,
    pub(crate) source_turn_id: String,
    pub(crate) outcome: AdaptiveEvidenceOutcome,
    pub(crate) kind: AdaptiveEvidenceKind,
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

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }
}

pub(crate) fn record_from_item_completion(
    notification: &ItemCompletedNotification,
) -> Option<AdaptiveEvidenceRecord> {
    let thread_id = ThreadId::from_string(&notification.thread_id).ok()?;
    let (evidence_id, outcome, kind) = match &notification.item {
        ThreadItem::CommandExecution { id, status, .. } => (
            id.clone(),
            match status {
                CommandExecutionStatus::Completed => AdaptiveEvidenceOutcome::Success,
                CommandExecutionStatus::Failed => AdaptiveEvidenceOutcome::Failure,
                CommandExecutionStatus::Declined => AdaptiveEvidenceOutcome::Cancelled,
                CommandExecutionStatus::InProgress => AdaptiveEvidenceOutcome::Incomplete,
            },
            AdaptiveEvidenceKind::CommandExecution,
        ),
        ThreadItem::McpToolCall { id, status, .. } => (
            id.clone(),
            match status {
                McpToolCallStatus::Completed => AdaptiveEvidenceOutcome::Success,
                McpToolCallStatus::Failed => AdaptiveEvidenceOutcome::Failure,
                McpToolCallStatus::InProgress => AdaptiveEvidenceOutcome::Incomplete,
            },
            AdaptiveEvidenceKind::McpToolCall,
        ),
        ThreadItem::DynamicToolCall {
            id,
            status,
            success,
            ..
        } => (
            id.clone(),
            match (status, success) {
                (DynamicToolCallStatus::Completed, Some(true)) => AdaptiveEvidenceOutcome::Success,
                (DynamicToolCallStatus::Completed | DynamicToolCallStatus::Failed, Some(false))
                | (DynamicToolCallStatus::Failed, _) => AdaptiveEvidenceOutcome::Failure,
                (DynamicToolCallStatus::InProgress, _)
                | (DynamicToolCallStatus::Completed, None) => AdaptiveEvidenceOutcome::Incomplete,
            },
            AdaptiveEvidenceKind::DynamicToolCall,
        ),
        _ => return None,
    };
    Some(AdaptiveEvidenceRecord {
        evidence_id,
        thread_id,
        source_turn_id: notification.turn_id.clone(),
        outcome,
        kind,
    })
}
