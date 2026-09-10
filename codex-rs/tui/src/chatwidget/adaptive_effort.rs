use super::*;
use crate::adaptive_controller::ADAPTIVE_UNFINISHED_TURN_THRESHOLD;
use crate::adaptive_evidence::ADAPTIVE_FAILURE_PRESSURE_THRESHOLD;
use crate::adaptive_evidence::AdaptiveEvidenceRegistry;
pub(crate) use crate::adaptive_policy::AdaptiveEffort;
pub(crate) use crate::adaptive_policy::AdaptiveFailureKind;
pub(crate) use crate::adaptive_policy::AdaptiveFamily;
pub(crate) use crate::adaptive_policy::AdaptiveOutcome;
use crate::adaptive_worker::AdaptiveWorkerContext;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use crate::adaptive_worker::NewWorkerBinding;
use crate::adaptive_worker::parse_adaptive_worker_assignment;
use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptivePendingDecision {
    ContinueSameRoute,
    RetrySameLevel,
    EscalateEffort,
    EscalateModel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptivePendingAttempt {
    pub(crate) thread_id: codex_protocol::ThreadId,
    pub(crate) source_turn_id: String,
    pub(crate) decision: AdaptivePendingDecision,
    pub(crate) route: crate::adaptive_policy::AdaptiveRoute,
    pub(crate) attempt_number: u32,
    pub(crate) worker_context: AdaptiveWorkerContext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveSuccessorPermit {
    pub(crate) thread_id: codex_protocol::ThreadId,
    pub(crate) source_turn_id: String,
    pub(crate) decision: AdaptivePendingDecision,
    pub(crate) route: crate::adaptive_policy::AdaptiveRoute,
    pub(crate) attempt_number: u32,
    pub(crate) worker_context: AdaptiveWorkerContext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveSuccessorAdmission {
    Reserved(AdaptiveSuccessorPermit),
    Consumed(AdaptiveSuccessorPermit),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum AdaptivePendingSignal {
    Pending(AdaptiveRuntimeSignalEnvelope),
    Conflicted { source_turn_id: String },
    Cancelled { source_turn_id: String },
    Consumed { source_turn_id: String },
}

impl AdaptivePendingSignal {
    pub(super) fn source_turn_id(&self) -> &str {
        match self {
            Self::Pending(signal) => &signal.source_turn_id,
            Self::Conflicted { source_turn_id }
            | Self::Cancelled { source_turn_id }
            | Self::Consumed { source_turn_id } => source_turn_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct AdaptiveEffortState {
    pub(crate) enabled: bool,
    pub(crate) starting_family: Option<AdaptiveFamily>,
    pub(crate) current_family: Option<AdaptiveFamily>,
    pub(crate) current_effort: Option<AdaptiveEffort>,
    pub(crate) attempt_number: u32,
    pub(crate) paused_by_user: bool,
    pub(crate) last_outcome: Option<AdaptiveOutcome>,
    pub(crate) last_failure_kind: Option<AdaptiveFailureKind>,
    pub(crate) worker_context: AdaptiveWorkerContext,
    pub(crate) worker_assignment_locked: bool,
    pub(crate) workflow_terminal: Option<AdaptiveWorkflowTerminal>,
    pub(crate) transient_retry_consumed: bool,
    pub(crate) unfinished_turn_pressure: u8,
    pub(crate) last_processed_terminal_turn_id: Option<String>,
    pub(crate) pending_attempt: Option<AdaptivePendingAttempt>,
    pub(crate) successor_admission: Option<AdaptiveSuccessorAdmission>,
    pub(crate) pending_signal: Option<AdaptivePendingSignal>,
    pub(crate) evidence_registry: AdaptiveEvidenceRegistry,
}

impl AdaptiveFamily {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "luna" => Some(Self::Luna),
            "terra" => Some(Self::Terra),
            "sol" => Some(Self::Sol),
            "astra" => Some(Self::Astra),
            _ => None,
        }
    }

    pub(crate) fn model(self) -> &'static str {
        match self {
            Self::Luna => "gpt-5.6-luna",
            Self::Terra => "gpt-5.6-terra",
            Self::Sol => "gpt-5.6-sol",
            Self::Astra => "gpt-6-astra",
        }
    }
}

impl AdaptiveEffortState {
    pub(crate) fn activate_adaptive_startup(&mut self, preferred_family: AdaptiveFamily) {
        if self.paused_by_user || self.workflow_terminal.is_some() {
            return;
        }
        self.activate_adaptive_family(preferred_family, false);
    }

    pub(crate) fn observe_worker_assignment_text(&mut self, text: &str) -> Result<bool, String> {
        if self.worker_assignment_locked {
            return Ok(false);
        }
        self.worker_assignment_locked = true;
        match parse_adaptive_worker_assignment(text) {
            Ok(Some(worker_context)) => {
                self.worker_context = worker_context;
                Ok(true)
            }
            Ok(None) => Ok(false),
            Err(err) => Err(err),
        }
    }

    fn activate_adaptive_family(&mut self, preferred_family: AdaptiveFamily, clear_pause: bool) {
        let worker_context = self.worker_context.clone();
        let worker_assignment_locked = self.worker_assignment_locked
            || worker_context.role != AdaptiveWorkerRole::Unspecified
            || worker_context.authorized_scope.is_some();
        let workflow_terminal = self.workflow_terminal;
        let evidence_registry = self.evidence_registry.clone();
        *self = Self {
            enabled: true,
            starting_family: Some(preferred_family),
            current_family: Some(AdaptiveFamily::Luna),
            current_effort: Some(AdaptiveEffort::Low),
            attempt_number: 1,
            paused_by_user: if clear_pause {
                false
            } else {
                self.paused_by_user
            },
            last_outcome: None,
            last_failure_kind: None,
            worker_context,
            worker_assignment_locked,
            workflow_terminal,
            transient_retry_consumed: false,
            unfinished_turn_pressure: 0,
            last_processed_terminal_turn_id: None,
            pending_attempt: None,
            successor_admission: None,
            pending_signal: None,
            evidence_registry,
        };
    }
}

impl ChatWidget {
    pub(crate) fn fresh_adaptive_effort_for_new_worker(
        &self,
        binding: Option<&NewWorkerBinding>,
    ) -> AdaptiveEffortState {
        let enabled = self.adaptive_effort.enabled || binding.is_some();
        let worker_context = binding.map_or_else(AdaptiveWorkerContext::default, |binding| {
            AdaptiveWorkerContext {
                role: binding.role,
                authorized_scope: Some(binding.authorized_scope.clone()),
            }
        });
        AdaptiveEffortState {
            enabled,
            starting_family: self.adaptive_effort.starting_family,
            current_family: enabled.then_some(AdaptiveFamily::Luna),
            current_effort: enabled.then_some(AdaptiveEffort::Low),
            attempt_number: enabled.then_some(1).unwrap_or_default(),
            worker_context,
            worker_assignment_locked: binding.is_some(),
            ..Default::default()
        }
    }

    pub(super) fn save_adaptive_effort_for_current_thread(&self) {
        self.app_event_tx.send(AppEvent::UpdateAdaptiveEffortState(
            self.adaptive_effort.clone(),
        ));
    }

    /// Record the confirmed, locally initiated Esc interruption of the active turn.
    pub(super) fn on_adaptive_user_interrupt(&mut self) {
        if !self.adaptive_effort.enabled {
            return;
        }

        self.adaptive_effort.paused_by_user = true;
        self.adaptive_effort.last_outcome = Some(AdaptiveOutcome::UserInterrupted);
        self.adaptive_effort.last_failure_kind = None;
        self.adaptive_effort.unfinished_turn_pressure = 0;
        self.cancel_pending_adaptive_signal_for_active_turn();
        self.adaptive_effort.successor_admission = None;
        self.save_adaptive_effort_for_current_thread();
    }

    pub(crate) fn dispatch_adaptive_command(&mut self, args: &str) {
        match args.trim() {
            "" | "status" => self.add_adaptive_status_output(),
            "pause" => {
                self.cancel_pending_adaptive_signal_for_active_turn();
                self.adaptive_effort.paused_by_user = true;
                self.adaptive_effort.unfinished_turn_pressure = 0;
                self.adaptive_effort.successor_admission = None;
                self.save_adaptive_effort_for_current_thread();
                self.add_adaptive_status_output();
            }
            "resume" => {
                self.adaptive_effort.paused_by_user = false;
                self.save_adaptive_effort_for_current_thread();
                self.add_adaptive_status_output();
            }
            "off" => {
                self.cancel_pending_adaptive_signal_for_active_turn();
                self.adaptive_effort.enabled = false;
                self.adaptive_effort.paused_by_user = false;
                self.adaptive_effort.unfinished_turn_pressure = 0;
                self.adaptive_effort.successor_admission = None;
                self.save_adaptive_effort_for_current_thread();
                self.add_adaptive_status_output();
            }
            "reset" => {
                let Some(thread_id) = self.thread_id else {
                    self.add_error_message(
                        "Cannot reset workflow state before a thread is active.".to_string(),
                    );
                    return;
                };
                self.app_event_tx.send(AppEvent::PersistWorkflowState {
                    thread_id,
                    operation: codex_app_server_protocol::ThreadWorkflowStateOperation::Clear,
                    source_turn_id: None,
                });
            }
            family => match AdaptiveFamily::parse(family) {
                Some(preferred_family) => {
                    self.activate_adaptive_family(preferred_family, true);
                }
                None => self.add_error_message(
                    "Usage: /adaptive [luna|terra|sol|astra|status|pause|resume|off|reset]"
                        .to_string(),
                ),
            },
        }
    }

    fn activate_adaptive_family(&mut self, preferred_family: AdaptiveFamily, clear_pause: bool) {
        self.adaptive_effort
            .activate_adaptive_family(preferred_family, clear_pause);
        self.save_adaptive_effort_for_current_thread();
        self.app_event_tx.send(AppEvent::UpdateModel(
            AdaptiveFamily::Luna.model().to_string(),
        ));
        self.app_event_tx.send(AppEvent::UpdateReasoningEffort(Some(
            codex_protocol::openai_models::ReasoningEffort::Low,
        )));
        self.add_adaptive_status_output();
    }

    pub(crate) fn on_workflow_state_persisted(
        &mut self,
        thread_id: codex_protocol::ThreadId,
        operation: codex_app_server_protocol::ThreadWorkflowStateOperation,
    ) {
        if self.thread_id != Some(thread_id) {
            return;
        }
        if operation == codex_app_server_protocol::ThreadWorkflowStateOperation::Clear {
            self.adaptive_effort.workflow_terminal = None;
            self.adaptive_effort.unfinished_turn_pressure = 0;
            self.adaptive_effort.pending_attempt = None;
            self.adaptive_effort.pending_signal = None;
            self.adaptive_effort.successor_admission = None;
            self.save_adaptive_effort_for_current_thread();
            self.add_adaptive_status_output();
        }
    }

    pub(crate) fn on_workflow_state_persistence_failed(
        &mut self,
        thread_id: codex_protocol::ThreadId,
        operation: codex_app_server_protocol::ThreadWorkflowStateOperation,
        error: &str,
    ) {
        if self.thread_id != Some(thread_id) {
            return;
        }
        if operation == codex_app_server_protocol::ThreadWorkflowStateOperation::SetReadyForOwnerQa
        {
            self.adaptive_effort.workflow_terminal =
                Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa);
            self.adaptive_effort.unfinished_turn_pressure = 0;
            self.adaptive_effort.pending_attempt = None;
            self.adaptive_effort.pending_signal = None;
            self.adaptive_effort.successor_admission = None;
            self.save_adaptive_effort_for_current_thread();
        }
        self.add_error_message(format!(
            "Failed to persist workflow state; adaptive automation remains stopped: {error}"
        ));
    }

    pub(super) fn add_adaptive_status_output(&mut self) {
        self.add_info_message(self.adaptive_effort_status_text(), None);
    }

    pub(super) fn adaptive_effort_status_text(&self) -> String {
        let state = &self.adaptive_effort;
        let family = |value| match value {
            Some(AdaptiveFamily::Luna) => "Luna",
            Some(AdaptiveFamily::Terra) => "Terra",
            Some(AdaptiveFamily::Sol) => "Sol",
            Some(AdaptiveFamily::Astra) => "Astra",
            None => "-",
        };
        let effort = match state.current_effort {
            Some(AdaptiveEffort::Low) => "Low",
            Some(AdaptiveEffort::Medium) => "Medium",
            Some(AdaptiveEffort::High) => "High",
            Some(AdaptiveEffort::XHigh) => "XHigh",
            Some(AdaptiveEffort::Max) => "Max",
            None => "-",
        };
        let outcome = match state.last_outcome {
            Some(AdaptiveOutcome::Unknown) => "UNKNOWN",
            Some(AdaptiveOutcome::UserInterrupted) => "USER_INTERRUPTED",
            Some(AdaptiveOutcome::ReadyForOwnerQa) => "READY_FOR_OWNER_QA",
            None => "None",
        };
        let failure = match state.last_failure_kind {
            Some(AdaptiveFailureKind::Capability) => "CAPABILITY",
            Some(AdaptiveFailureKind::TransientInfrastructure) => "TRANSIENT_INFRASTRUCTURE",
            Some(AdaptiveFailureKind::Authentication) => "AUTHENTICATION",
            Some(AdaptiveFailureKind::Quota) => "QUOTA",
            Some(AdaptiveFailureKind::Environment) => "ENVIRONMENT",
            Some(AdaptiveFailureKind::Permission) => "PERMISSION",
            Some(AdaptiveFailureKind::Dependency) => "DEPENDENCY",
            Some(AdaptiveFailureKind::ModelUnavailable) => "MODEL_UNAVAILABLE",
            Some(AdaptiveFailureKind::OwnerDecision) => "OWNER_DECISION",
            Some(AdaptiveFailureKind::AuthorizationOrScope) => "AUTHORIZATION_OR_SCOPE",
            Some(AdaptiveFailureKind::UserInterrupted) => "USER_INTERRUPTED",
            Some(AdaptiveFailureKind::ReadyForOwnerQa) => "READY_FOR_OWNER_QA",
            Some(AdaptiveFailureKind::Unknown) => "UNKNOWN",
            None => "None",
        };
        let worker_role = match state.worker_context.role {
            AdaptiveWorkerRole::Unspecified => "Unspecified",
            AdaptiveWorkerRole::Implementation => "Implementation",
            AdaptiveWorkerRole::Validation => "Validation",
            AdaptiveWorkerRole::Repair => "Repair",
        };
        let worker_scope = state
            .worker_context
            .authorized_scope
            .as_deref()
            .filter(|scope| !scope.trim().is_empty())
            .unwrap_or("None");
        let worker_is_bound = matches!(
            state.worker_context.role,
            AdaptiveWorkerRole::Implementation
                | AdaptiveWorkerRole::Validation
                | AdaptiveWorkerRole::Repair
        ) && state
            .worker_context
            .authorized_scope
            .as_deref()
            .is_some_and(|scope| !scope.trim().is_empty());
        let worker_binding = if worker_is_bound {
            "Bound"
        } else if state.worker_assignment_locked {
            "Locked unbound"
        } else {
            "Awaiting first assignment"
        };
        let workflow_terminal = match state.workflow_terminal {
            Some(AdaptiveWorkflowTerminal::ReadyForValidation) => "READY_FOR_VALIDATION",
            Some(AdaptiveWorkflowTerminal::RepairRequired) => "REPAIR_REQUIRED",
            Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa) => "READY_FOR_OWNER_QA",
            Some(AdaptiveWorkflowTerminal::Blocked) => "BLOCKED",
            None => "None",
        };
        let failure_pressure = match (self.thread_id, self.turn_lifecycle.last_turn_id.as_deref()) {
            (Some(thread_id), Some(source_turn_id)) => state
                .evidence_registry
                .failure_pressure_for_turn(thread_id, source_turn_id),
            _ => 0,
        };
        let codexdd_identity = codex_build_info::codexdd_compact_identity();
        format!(
            "Adaptive Effort\n  codexdd: {}\n  Enabled: {}\n  Preference: {}\n  Current: {} {}\n  Attempt: {}\n  Failure pressure: {}/{}\n  Unfinished pressure: {}/{}\n  Paused: {}\n  Last outcome: {}\n  Last failure: {}\n  Worker role: {}\n  Worker scope: {}\n  Worker binding: {}\n  Workflow terminal: {}",
            codexdd_identity,
            if state.enabled { "yes" } else { "no" },
            family(state.starting_family),
            family(state.current_family),
            effort,
            state.attempt_number,
            failure_pressure,
            ADAPTIVE_FAILURE_PRESSURE_THRESHOLD,
            state.unfinished_turn_pressure,
            ADAPTIVE_UNFINISHED_TURN_THRESHOLD,
            if state.paused_by_user { "yes" } else { "no" },
            outcome,
            failure,
            worker_role,
            worker_scope,
            worker_binding,
            workflow_terminal
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALIDATION_ASSIGNMENT: &str = "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"Z-A0.45B-R2 independent verification\"\n\nReview the exact pushed R2 candidate.";

    #[test]
    fn plain_adaptive_startup_waits_for_external_worker_binding() {
        let mut state = AdaptiveEffortState::default();
        state.activate_adaptive_startup(AdaptiveFamily::Astra);

        assert!(state.enabled);
        assert_eq!(state.worker_context, AdaptiveWorkerContext::default());
        assert!(!state.worker_assignment_locked);
    }

    #[test]
    fn first_assignment_binds_once_and_later_messages_cannot_mutate_authority() {
        let mut state = AdaptiveEffortState::default();
        state.activate_adaptive_startup(AdaptiveFamily::Astra);

        assert_eq!(
            state.observe_worker_assignment_text(VALIDATION_ASSIGNMENT),
            Ok(true)
        );
        assert!(state.worker_assignment_locked);
        assert_eq!(state.worker_context.role, AdaptiveWorkerRole::Validation);
        assert_eq!(
            state.worker_context.authorized_scope.as_deref(),
            Some("Z-A0.45B-R2 independent verification")
        );

        assert_eq!(
            state.observe_worker_assignment_text(
                "[adaptive_worker]\nrole = \"repair\"\nauthorized_scope = \"different scope\"\n\nRepair it."
            ),
            Ok(false)
        );
        assert_eq!(state.worker_context.role, AdaptiveWorkerRole::Validation);
        assert_eq!(
            state.worker_context.authorized_scope.as_deref(),
            Some("Z-A0.45B-R2 independent verification")
        );
    }

    #[test]
    fn first_message_without_valid_header_locks_thread_unbound() {
        let mut state = AdaptiveEffortState::default();
        state.activate_adaptive_startup(AdaptiveFamily::Astra);

        assert_eq!(
            state.observe_worker_assignment_text("Review the candidate without a typed header."),
            Ok(false)
        );
        assert!(state.worker_assignment_locked);
        assert_eq!(state.worker_context, AdaptiveWorkerContext::default());
        assert_eq!(
            state.observe_worker_assignment_text(VALIDATION_ASSIGNMENT),
            Ok(false)
        );
        assert_eq!(state.worker_context, AdaptiveWorkerContext::default());
    }

    #[test]
    fn malformed_first_header_locks_thread_unbound() {
        let mut state = AdaptiveEffortState::default();
        state.activate_adaptive_startup(AdaptiveFamily::Astra);

        assert!(
            state
                .observe_worker_assignment_text(
                    "[adaptive_worker]\nrole = \"reviewer\"\nauthorized_scope = \"R2\"\n\nReview it."
                )
                .is_err()
        );
        assert!(state.worker_assignment_locked);
        assert_eq!(state.worker_context, AdaptiveWorkerContext::default());
        assert_eq!(
            state.observe_worker_assignment_text(VALIDATION_ASSIGNMENT),
            Ok(false)
        );
    }

    #[test]
    fn preconfigured_worker_authority_cannot_be_overridden_by_prompt() {
        let mut state = AdaptiveEffortState {
            worker_context: AdaptiveWorkerContext {
                role: AdaptiveWorkerRole::Implementation,
                authorized_scope: Some("preconfigured implementation gate".to_string()),
            },
            ..Default::default()
        };
        state.activate_adaptive_startup(AdaptiveFamily::Astra);

        assert!(state.worker_assignment_locked);
        assert_eq!(
            state.observe_worker_assignment_text(VALIDATION_ASSIGNMENT),
            Ok(false)
        );
        assert_eq!(
            state.worker_context.role,
            AdaptiveWorkerRole::Implementation
        );
        assert_eq!(
            state.worker_context.authorized_scope.as_deref(),
            Some("preconfigured implementation gate")
        );
    }
}
