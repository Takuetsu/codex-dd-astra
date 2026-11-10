//! Handles persistent thread-settings updates and serializes their persistence
//! with checkpoints written directly to storage.

use super::session::Session;
use super::session::SessionSettingsUpdate;
use super::step_settings::StepSettingsUpdate;
use super::worker_authority::CandidateShaAuthorization;
use crate::WithTurnExtensionData;
use crate::config::ConstraintResult;
use codex_history::RolloutItem;
use codex_protocol::capabilities::SelectedCapabilityRoot;
use codex_protocol::protocol::Event;
use codex_protocol::protocol::EventMsg;
use codex_protocol::protocol::ThreadSettingsAppliedEvent;
use codex_protocol::protocol::ThreadSettingsOverrides;
use codex_protocol::protocol::ThreadSettingsSnapshot;
use codex_thread_store::ThreadStoreResult;
use tokio::sync::SemaphorePermit;

impl Session {
    /// Retains the established thread-settings checkpoint API for callers that
    /// only need a successful write or error, without exposing Worker authority.
    pub(crate) async fn checkpoint_thread_settings(&self) -> ThreadStoreResult<()> {
        self.checkpoint_thread_settings_with_durability()
            .await
            .map(|_| ())
    }

    /// Captures and flushes current settings under the shared persistence permit.
    /// No live recorder means no durable first-message authority can be granted.
    pub(crate) async fn checkpoint_thread_settings_with_durability(
        &self,
    ) -> ThreadStoreResult<bool> {
        let _settings_guard = acquire_persistence_lock(self).await;
        let Some(live_thread) = self.live_thread() else {
            return Ok(false);
        };
        live_thread
            .append_items(&[RolloutItem::EventMsg(applied_event(self).await)])
            .await?;
        live_thread.flush().await?;
        Ok(true)
    }
    /// Bind an explicitly requested descendant only after the native thread
    /// store appends and flushes its exact same-thread settings snapshot.
    /// No Core/TUI execution authority is changed on validation or IO failure.
    pub(crate) async fn authorize_candidate_sha(
        &self,
        requested: &CandidateShaAuthorization,
    ) -> Result<(), String> {
        tracing::warn!("3B2e probe A0: entering candidate persistence");
        let _settings_guard = acquire_persistence_lock(self).await;
        tracing::warn!("3B2e probe A1: persistence permit acquired");
        let Some(live_thread) = self.live_thread() else {
            return Err(
                "SETUP_BLOCKED: candidate source requires a live durable thread store".to_string(),
            );
        };
        tracing::warn!("3B2e probe A2: before source preflight");
        let prospective = {
            let state = self.state.lock().await;
            state
                .first_worker_authority
                .verified_candidate_snapshot(requested)?
        };
        tracing::warn!("3B2e probe A3: source preflight completed");
        let mut snapshot = self.thread_settings_snapshot().await;
        tracing::warn!("3B2e probe A4: settings snapshot captured");
        snapshot.codexdd_first_worker = Some(prospective.clone());
        let msg = EventMsg::ThreadSettingsApplied(ThreadSettingsAppliedEvent {
            thread_id: Some(self.thread_id()),
            thread_settings: snapshot.clone(),
        });
        tracing::warn!("3B2e probe A5: before native append");
        live_thread
            .append_items(&[RolloutItem::EventMsg(msg)])
            .await
            .map_err(|error| {
                format!("SETUP_BLOCKED: candidate checkpoint append failed: {error}")
            })?;
        tracing::warn!("3B2e probe A6: append finished, before flush");
        live_thread.flush().await.map_err(|error| {
            format!("SETUP_BLOCKED: candidate checkpoint flush failed: {error}")
        })?;
        tracing::warn!("3B2e probe A7: native checkpoint flushed");
        // The persistence permit excludes competing settings checkpoints.
        // Any interrupted step after flush is recovered from the native record.
        self.state
            .lock()
            .await
            .first_worker_authority
            .apply_durable_candidate(requested, &prospective)?;
        tracing::warn!("3B2e probe A8: in-memory SHA promoted");
        // The matching embedded TUI needs a trusted Core-owned post-flush
        // settings notification. Never emit this before native persistence
        // succeeds; the event itself does not add another durable checkpoint.
        tracing::warn!("3B2e probe A9: before notification delivery");
        self.send_event_raw_without_materializing_rollout(Event {
            id: format!("codexdd-candidate-{}", requested.candidate_sha),
            msg: EventMsg::ThreadSettingsApplied(ThreadSettingsAppliedEvent {
                thread_id: Some(self.thread_id()),
                thread_settings: snapshot,
            }),
        })
        .await;
        tracing::warn!("3B2e probe A10: candidate authorization complete");
        Ok(())
    }
}

/// Applies standalone thread settings. The caller holds the persistence permit through notification.
pub(super) async fn update(
    session: &Session,
    overrides: impl Into<WithTurnExtensionData<ThreadSettingsOverrides>>,
) -> ConstraintResult<ThreadSettingsSnapshot> {
    let updates = prepare_update(overrides, &session.services.selected_capability_roots);
    let commit = session.update_settings(updates).await?;
    // Standalone settings changes supersede a pending automatic continuation.
    session.state.lock().await.last_started_turn_id = None;
    Ok(commit.snapshot)
}

/// Converts protocol overrides into the internal settings update shape.
pub(super) fn prepare_update(
    overrides: impl Into<WithTurnExtensionData<ThreadSettingsOverrides>>,
    roots: &[SelectedCapabilityRoot],
) -> SessionSettingsUpdate {
    let WithTurnExtensionData {
        request: overrides,
        turn_extension_init,
    } = overrides.into();
    let ThreadSettingsOverrides {
        environments: environment_requests,
        runtime_workspace_roots,
        profile_workspace_roots,
        approval_policy,
        approvals_reviewer,
        sandbox_policy,
        permission_profile,
        active_permission_profile,
        windows_sandbox_level,
        model,
        effort,
        summary,
        service_tier,
        collaboration_mode,
        personality,
        disabled_plugin_ids,
    } = overrides;
    SessionSettingsUpdate {
        turn_extension_init,
        step_settings: StepSettingsUpdate {
            model,
            effort,
            collaboration_mode,
            reasoning_summary: summary,
            service_tier,
            personality,
            approval_policy,
            approvals_reviewer,
        },
        environments: environment_requests.map(|requests| requests.select(roots)),
        runtime_workspace_roots,
        profile_workspace_roots,
        sandbox_policy,
        permission_profile,
        active_permission_profile,
        windows_sandbox_level,
        disabled_plugin_ids,
        ..Default::default()
    }
}

/// Acquires the shared permit before capturing or changing persistent settings.
pub(super) async fn acquire_persistence_lock(session: &Session) -> SemaphorePermit<'_> {
    session
        .thread_settings_persistence
        .acquire()
        .await
        .unwrap_or_else(|_| unreachable!("thread settings persistence semaphore is never closed"))
}

/// Applies persistent settings and emits the resulting thread-owned snapshot.
pub(super) async fn apply_update(
    session: &Session,
    submission_id: String,
    updates: SessionSettingsUpdate,
) -> ConstraintResult<()> {
    let _settings_guard = acquire_persistence_lock(session).await;
    let commit = session.update_settings(updates).await?;
    emit_applied(session, submission_id, commit.snapshot).await;
    Ok(())
}

/// Emits the snapshot published by one successful settings update.
pub(super) async fn emit_applied(
    session: &Session,
    submission_id: String,
    snapshot: ThreadSettingsSnapshot,
) {
    let msg = EventMsg::ThreadSettingsApplied(ThreadSettingsAppliedEvent {
        thread_id: Some(session.thread_id()),
        thread_settings: snapshot,
    });
    session
        .send_event_raw_without_materializing_rollout(Event {
            id: submission_id,
            msg,
        })
        .await;
}

/// Builds a current thread-owned snapshot for storage checkpoints.
pub(super) async fn applied_event(session: &Session) -> EventMsg {
    EventMsg::ThreadSettingsApplied(ThreadSettingsAppliedEvent {
        thread_id: Some(session.thread_id()),
        thread_settings: session.thread_settings_snapshot().await,
    })
}
