//! Canonical TUI session state shared across app-server routing, chat display, and status UI.
//!
//! The app-server API is the boundary for session lifecycle events. Once those responses enter
//! TUI, this module holds the small internal state shape used by app orchestration and widgets.

use std::path::PathBuf;

use crate::adaptive_complexity::AdaptiveComplexityClass;
use crate::adaptive_worker::AdaptiveWorkerContext;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use crate::chatwidget::adaptive_effort::AdaptiveEffort;
use crate::chatwidget::adaptive_effort::AdaptiveEffortState;
use crate::chatwidget::adaptive_effort::AdaptiveFamily;
use codex_app_server_protocol::AskForApproval;
use codex_protocol::ThreadId;
use codex_protocol::config_types::CollaborationMode;
use codex_protocol::config_types::Personality;
use codex_protocol::models::ActivePermissionProfile;
use codex_protocol::models::PermissionProfile;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_path_uri::PathUri;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SessionNetworkProxyRuntime {
    pub(crate) http_addr: String,
    pub(crate) socks_addr: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct MessageHistoryMetadata {
    pub(crate) log_id: u64,
    pub(crate) entry_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ThreadSessionState {
    pub(crate) thread_id: ThreadId,
    pub(crate) forked_from_id: Option<ThreadId>,
    pub(crate) fork_parent_title: Option<String>,
    pub(crate) thread_name: Option<String>,
    pub(crate) model: String,
    pub(crate) model_provider_id: String,
    pub(crate) service_tier: Option<String>,
    pub(crate) approval_policy: AskForApproval,
    pub(crate) approvals_reviewer: codex_protocol::config_types::ApprovalsReviewer,
    /// Permission snapshot used by TUI display surfaces. Legacy app-server
    /// responses are converted to a profile at ingestion time using the
    /// response cwd so cached sessions do not reinterpret cwd-bound grants.
    /// Turn requests must not treat this snapshot as a local permission
    /// override unless the user explicitly changed permissions in the TUI.
    pub(crate) permission_profile: PermissionProfile,
    /// Named or implicit built-in profile that produced `permission_profile`,
    /// when the server knows it.
    pub(crate) active_permission_profile: Option<ActivePermissionProfile>,
    pub(crate) cwd: AbsolutePathBuf,
    pub(crate) runtime_workspace_roots: Vec<AbsolutePathBuf>,
    pub(crate) instruction_source_paths: Vec<PathUri>,
    pub(crate) reasoning_effort: Option<codex_protocol::openai_models::ReasoningEffort>,
    pub(crate) collaboration_mode: Option<Box<CollaborationMode>>,
    pub(crate) personality: Option<Personality>,
    pub(crate) message_history: Option<MessageHistoryMetadata>,
    pub(crate) network_proxy: Option<SessionNetworkProxyRuntime>,
    pub(crate) rollout_path: Option<PathBuf>,
    /// Process-local adaptive controller state owned by this thread.
    pub(crate) adaptive_effort: AdaptiveEffortState,
}

impl ThreadSessionState {
    /// Copies adaptive state exactly once from the immediate typed fork parent.
    pub(crate) fn inherit_adaptive_effort_from(&mut self, parent: &Self) {
        if self.forked_from_id == Some(parent.thread_id)
            && self.adaptive_effort == AdaptiveEffortState::default()
        {
            self.adaptive_effort = parent.adaptive_effort.clone();
            self.adaptive_effort.pending_signal = None;
            self.adaptive_effort.pending_attempt = None;
            self.adaptive_effort.successor_admission = None;
            self.adaptive_effort.unfinished_turn_pressure = 0;
            self.adaptive_effort.evidence_registry = Default::default();
        }
    }

    pub(crate) fn set_cwd_retargeting_implicit_runtime_workspace_root(
        &mut self,
        cwd: AbsolutePathBuf,
    ) {
        let previous_cwd = std::mem::replace(&mut self.cwd, cwd.clone());
        if !self.runtime_workspace_roots.contains(&previous_cwd) {
            return;
        }

        let previous_roots = std::mem::take(&mut self.runtime_workspace_roots);
        self.runtime_workspace_roots.push(cwd);
        for root in previous_roots {
            if root != previous_cwd && !self.runtime_workspace_roots.contains(&root) {
                self.runtime_workspace_roots.push(root);
            }
        }
    }
}

fn restore_family(value: Option<String>) -> Result<Option<AdaptiveFamily>, String> {
    value
        .map(|value| {
            AdaptiveFamily::parse(&value)
                .ok_or_else(|| format!("unsupported persisted adaptive family `{value}`"))
        })
        .transpose()
}

fn restore_effort(value: Option<String>) -> Result<Option<AdaptiveEffort>, String> {
    value
        .map(|value| {
            match value.as_str() {
                "low" => Some(AdaptiveEffort::Low),
                "medium" => Some(AdaptiveEffort::Medium),
                "high" => Some(AdaptiveEffort::High),
                "xhigh" => Some(AdaptiveEffort::XHigh),
                "max" => Some(AdaptiveEffort::Max),
                _ => None,
            }
            .ok_or_else(|| format!("unsupported persisted adaptive effort `{value}`"))
        })
        .transpose()
}

fn restore_complexity_class(
    value: Option<String>,
) -> Result<Option<AdaptiveComplexityClass>, String> {
    value
        .map(|value| {
            match value.as_str() {
                "routine" => Some(AdaptiveComplexityClass::Routine),
                "standard" => Some(AdaptiveComplexityClass::Standard),
                "complex" => Some(AdaptiveComplexityClass::Complex),
                "architectural" => Some(AdaptiveComplexityClass::Architectural),
                _ => None,
            }
            .ok_or_else(|| format!("unsupported persisted adaptive complexity class `{value}`"))
        })
        .transpose()
}

fn restore_worker_role(value: &str) -> Result<AdaptiveWorkerRole, String> {
    match value {
        "unspecified" => Ok(AdaptiveWorkerRole::Unspecified),
        "implementation" => Ok(AdaptiveWorkerRole::Implementation),
        "validation" => Ok(AdaptiveWorkerRole::Validation),
        "repair" => Ok(AdaptiveWorkerRole::Repair),
        _ => Err(format!(
            "unsupported persisted adaptive Worker role `{value}`"
        )),
    }
}

fn restore_workflow_terminal(
    value: Option<String>,
) -> Result<Option<AdaptiveWorkflowTerminal>, String> {
    value
        .map(|value| {
            match value.as_str() {
                "ready_for_validation" => Some(AdaptiveWorkflowTerminal::ReadyForValidation),
                "repair_required" => Some(AdaptiveWorkflowTerminal::RepairRequired),
                "ready_for_owner_qa" => Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa),
                "blocked" => Some(AdaptiveWorkflowTerminal::Blocked),
                _ => None,
            }
            .ok_or_else(|| format!("unsupported persisted workflow terminal `{value}`"))
        })
        .transpose()
}

fn apply_persisted_adaptive_workflow_state(
    adaptive_effort: &mut AdaptiveEffortState,
    state: codex_history::AdaptiveWorkflowStateSnapshot,
    source_turn_id: Option<String>,
) -> Result<(), String> {
    let starting_family = restore_family(state.starting_family)?;
    let current_family = restore_family(state.current_family)?;
    let current_effort = restore_effort(state.current_effort)?;
    let worker_role = restore_worker_role(&state.worker_role)?;
    let workflow_terminal = restore_workflow_terminal(state.workflow_terminal)?;
    let complexity_class = restore_complexity_class(state.complexity_class)?;
    let budget_mode = adaptive_effort.budget_mode;

    if state.enabled
        && (current_family.is_none() || current_effort.is_none() || state.attempt_number == 0)
    {
        return Err(
            "persisted enabled adaptive state is missing its active route or attempt".to_string(),
        );
    }
    if current_family.is_some() != current_effort.is_some() {
        return Err("persisted adaptive family/effort route is incomplete".to_string());
    }

    *adaptive_effort = AdaptiveEffortState {
        enabled: state.enabled,
        starting_family,
        current_family,
        current_effort,
        attempt_number: state.attempt_number,
        paused_by_user: state.paused_by_user,
        last_outcome: None,
        last_failure_kind: None,
        worker_context: AdaptiveWorkerContext {
            role: worker_role,
            authorized_scope: state.authorized_scope,
        },
        worker_assignment_locked: state.worker_assignment_locked,
        workflow_terminal,
        transient_retry_consumed: false,
        unfinished_turn_pressure: 0,
        last_processed_terminal_turn_id: workflow_terminal.and(source_turn_id),
        pending_attempt: None,
        successor_admission: None,
        pending_signal: None,
        evidence_registry: Default::default(),
        complexity_class,
        budget_mode,
    };
    Ok(())
}

pub(crate) async fn restore_persisted_workflow_state(
    rollout_path: Option<&std::path::Path>,
    thread_id: ThreadId,
    adaptive_effort: &mut AdaptiveEffortState,
) -> Result<(), String> {
    let Some(rollout_path) = rollout_path else {
        return Ok(());
    };
    let (items, rollout_thread_id, _) =
        codex_rollout::RolloutRecorder::load_rollout_items(rollout_path)
            .await
            .map_err(|err| format!("failed to restore workflow state: {err}"))?;
    if rollout_thread_id != Some(thread_id) {
        return Err(format!(
            "workflow state rollout belongs to {rollout_thread_id:?}, not {thread_id}"
        ));
    }
    match codex_history::restored_workflow_state(items.iter()) {
        codex_history::RestoredWorkflowState::None => {}
        codex_history::RestoredWorkflowState::ReadyForOwnerQa => {
            adaptive_effort.workflow_terminal = Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa);
            adaptive_effort.unfinished_turn_pressure = 0;
            adaptive_effort.pending_attempt = None;
            adaptive_effort.pending_signal = None;
            adaptive_effort.successor_admission = None;
        }
        codex_history::RestoredWorkflowState::Adaptive {
            state,
            source_turn_id,
        } => {
            apply_persisted_adaptive_workflow_state(adaptive_effort, state, source_turn_id)?;
        }
        codex_history::RestoredWorkflowState::Unsupported => {
            adaptive_effort.enabled = false;
            adaptive_effort.paused_by_user = true;
            adaptive_effort.unfinished_turn_pressure = 0;
            adaptive_effort.pending_attempt = None;
            adaptive_effort.pending_signal = None;
            adaptive_effort.successor_admission = None;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptive_budget::AdaptiveBudgetMode;

    #[test]
    fn restored_terminal_snapshot_preserves_worker_route_and_clears_ephemeral_authority() {
        let mut adaptive = AdaptiveEffortState {
            budget_mode: AdaptiveBudgetMode::Surplus,
            ..AdaptiveEffortState::default()
        };
        apply_persisted_adaptive_workflow_state(
            &mut adaptive,
            codex_history::AdaptiveWorkflowStateSnapshot {
                enabled: true,
                starting_family: Some("astra".to_string()),
                current_family: Some("luna".to_string()),
                current_effort: Some("high".to_string()),
                complexity_class: None,
                attempt_number: 6,
                paused_by_user: false,
                worker_role: "repair".to_string(),
                authorized_scope: Some("Z-A0.47B bounded repair".to_string()),
                worker_assignment_locked: true,
                workflow_terminal: Some("ready_for_validation".to_string()),
            },
            Some("turn-6".to_string()),
        )
        .expect("snapshot should restore");

        assert!(adaptive.enabled);
        assert_eq!(adaptive.starting_family, Some(AdaptiveFamily::Astra));
        assert_eq!(adaptive.current_family, Some(AdaptiveFamily::Luna));
        assert_eq!(adaptive.current_effort, Some(AdaptiveEffort::High));
        assert_eq!(adaptive.attempt_number, 6);
        assert_eq!(adaptive.budget_mode, AdaptiveBudgetMode::Surplus);
        assert_eq!(adaptive.worker_context.role, AdaptiveWorkerRole::Repair);
        assert_eq!(
            adaptive.worker_context.authorized_scope.as_deref(),
            Some("Z-A0.47B bounded repair")
        );
        assert!(adaptive.worker_assignment_locked);
        assert_eq!(
            adaptive.workflow_terminal,
            Some(AdaptiveWorkflowTerminal::ReadyForValidation)
        );
        assert_eq!(
            adaptive.last_processed_terminal_turn_id.as_deref(),
            Some("turn-6")
        );
        assert_eq!(adaptive.unfinished_turn_pressure, 0);
        assert_eq!(adaptive.pending_attempt, None);
        assert_eq!(adaptive.pending_signal, None);
        assert_eq!(adaptive.successor_admission, None);
        assert_eq!(adaptive.evidence_registry, Default::default());
    }

    #[test]
    fn restored_snapshot_accepts_every_hard_workflow_terminal() {
        for (persisted, expected) in [
            (
                "ready_for_validation",
                AdaptiveWorkflowTerminal::ReadyForValidation,
            ),
            ("repair_required", AdaptiveWorkflowTerminal::RepairRequired),
            (
                "ready_for_owner_qa",
                AdaptiveWorkflowTerminal::ReadyForOwnerQa,
            ),
            ("blocked", AdaptiveWorkflowTerminal::Blocked),
        ] {
            let mut adaptive = AdaptiveEffortState::default();
            apply_persisted_adaptive_workflow_state(
                &mut adaptive,
                codex_history::AdaptiveWorkflowStateSnapshot {
                    enabled: true,
                    starting_family: Some("astra".to_string()),
                    current_family: Some("luna".to_string()),
                    current_effort: Some("low".to_string()),
                    complexity_class: None,
                    attempt_number: 1,
                    paused_by_user: false,
                    worker_role: "repair".to_string(),
                    authorized_scope: Some("bounded scope".to_string()),
                    worker_assignment_locked: true,
                    workflow_terminal: Some(persisted.to_string()),
                },
                Some("terminal-turn".to_string()),
            )
            .expect("terminal snapshot should restore");

            assert_eq!(adaptive.workflow_terminal, Some(expected));
            assert_eq!(adaptive.pending_attempt, None);
            assert_eq!(adaptive.pending_signal, None);
            assert_eq!(adaptive.successor_admission, None);
        }
    }

    #[test]
    fn restored_active_snapshot_keeps_attempt_but_never_restores_successor_authority() {
        let mut adaptive = AdaptiveEffortState::default();
        apply_persisted_adaptive_workflow_state(
            &mut adaptive,
            codex_history::AdaptiveWorkflowStateSnapshot {
                enabled: true,
                starting_family: Some("sol".to_string()),
                current_family: Some("luna".to_string()),
                current_effort: Some("medium".to_string()),
                complexity_class: None,
                attempt_number: 4,
                paused_by_user: true,
                worker_role: "validation".to_string(),
                authorized_scope: Some("independent validation".to_string()),
                worker_assignment_locked: true,
                workflow_terminal: None,
            },
            None,
        )
        .expect("snapshot should restore");

        assert_eq!(adaptive.attempt_number, 4);
        assert!(adaptive.paused_by_user);
        assert_eq!(adaptive.worker_context.role, AdaptiveWorkerRole::Validation);
        assert_eq!(adaptive.workflow_terminal, None);
        assert_eq!(adaptive.last_processed_terminal_turn_id, None);
        assert_eq!(adaptive.unfinished_turn_pressure, 0);
        assert_eq!(adaptive.pending_attempt, None);
        assert_eq!(adaptive.pending_signal, None);
        assert_eq!(adaptive.successor_admission, None);
    }
}

#[cfg(test)]
mod codexdd_complexity_persistence_regression {
    use super::*;
    use crate::adaptive_complexity::AdaptiveComplexityClass;

    #[test]
    fn restored_snapshot_recovers_complexity_class() {
        let snapshot: codex_history::AdaptiveWorkflowStateSnapshot = serde_json::from_str(
            r#"{
                    "enabled": true,
                    "starting_family": "astra",
                    "current_family": "sol",
                    "current_effort": "medium",
                    "attempt_number": 2,
                    "paused_by_user": false,
                    "worker_role": "implementation",
                    "authorized_scope": "codexdd/complexity-restore",
                    "worker_assignment_locked": true,
                    "workflow_terminal": null,
                    "complexity_class": "architectural"
                }"#,
        )
        .expect("snapshot should deserialize");

        let mut adaptive = AdaptiveEffortState::default();

        apply_persisted_adaptive_workflow_state(&mut adaptive, snapshot, None)
            .expect("snapshot should restore");

        assert_eq!(
            adaptive.complexity_class,
            Some(AdaptiveComplexityClass::Architectural)
        );
    }
}
