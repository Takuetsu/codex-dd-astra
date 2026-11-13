use super::*;
use crate::adaptive_budget::AdaptiveBudgetMode;
use crate::adaptive_budget::mechanical_validation_route;
use crate::adaptive_budget::quality_review_route;
use crate::adaptive_complexity::AdaptiveComplexityClass;
use crate::adaptive_complexity::AdaptiveImplementationPhase;
use crate::adaptive_complexity::implementation_floor;
use crate::adaptive_controller::ADAPTIVE_UNFINISHED_TURN_THRESHOLD;
use crate::adaptive_evidence::ADAPTIVE_FAILURE_PRESSURE_THRESHOLD;
use crate::adaptive_evidence::AdaptiveEvidenceRegistry;
use crate::adaptive_gate_prerequisites::AdaptiveGateLedger;
pub(crate) use crate::adaptive_policy::AdaptiveEffort;
pub(crate) use crate::adaptive_policy::AdaptiveFailureKind;
pub(crate) use crate::adaptive_policy::AdaptiveFamily;
pub(crate) use crate::adaptive_policy::AdaptiveOutcome;
use crate::adaptive_worker::AdaptiveWorkerContext;
use crate::adaptive_worker::AdaptiveWorkerPreflight;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use crate::adaptive_worker::NewWorkerBinding;
use crate::adaptive_worker::parse_adaptive_owner_assignment;
use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;

pub(crate) const CODEXDD_VALIDATION_REPAIR_LIMIT: u8 = 2;
/// Consecutive unfinished Implementation turns without a completed native tool
/// result indicate a stalled Worker, not evidence that more model effort helps.
pub(crate) const CODEXDD_NO_NATIVE_PROGRESS_TURN_LIMIT: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptivePendingDecision {
    ContinueSameRoute,
    RetrySameLevel,
    EscalateEffort,
    EscalateModel,
    EnterMechanicalValidation,
    ResumeImplementation,
    ValidationTerminalization,
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
    /// Turn completion arrived before the separate trusted adaptive signal notification.
    /// The completed turn itself proves report_adaptive_signal ran, so successor admission
    /// must fail closed until that notification is delivered and validated.
    Awaiting {
        source_turn_id: String,
    },
    Pending(AdaptiveRuntimeSignalEnvelope),
    Conflicted {
        source_turn_id: String,
    },
    Cancelled {
        source_turn_id: String,
    },
    Consumed {
        source_turn_id: String,
    },
}

impl AdaptivePendingSignal {
    pub(super) fn source_turn_id(&self) -> &str {
        match self {
            Self::Pending(signal) => &signal.source_turn_id,
            Self::Awaiting { source_turn_id }
            | Self::Conflicted { source_turn_id }
            | Self::Cancelled { source_turn_id }
            | Self::Consumed { source_turn_id } => source_turn_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveValidationStatus {
    pub(crate) profile: String,
    pub(crate) result: String,
    pub(crate) run_id: String,
    pub(crate) branch: Option<String>,
    pub(crate) head_sha: String,
    pub(crate) failed_stage: Option<String>,
    pub(crate) failure_fingerprint: Option<String>,
    pub(crate) log_path: String,
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
    pub(crate) assignment_preflight: Option<AdaptiveWorkerPreflight>,
    pub(crate) worker_assignment_locked: bool,
    pub(crate) workflow_terminal: Option<AdaptiveWorkflowTerminal>,
    pub(crate) transient_retry_consumed: bool,
    pub(crate) unfinished_turn_pressure: u8,
    /// Stops runaway no-tool continuations independently of alternating effort pressure.
    pub(crate) consecutive_no_native_progress_turns: u8,
    pub(crate) last_processed_terminal_turn_id: Option<String>,
    pub(crate) pending_attempt: Option<AdaptivePendingAttempt>,
    pub(crate) successor_admission: Option<AdaptiveSuccessorAdmission>,
    pub(crate) pending_signal: Option<AdaptivePendingSignal>,
    pub(crate) evidence_registry: AdaptiveEvidenceRegistry,
    /// Pending source-/Worker-bound requirements, never cached success authority.
    pub(crate) gate_ledger: Option<AdaptiveGateLedger>,
    /// Ephemeral parent Implementation thread for a fresh, source-bound review.
    /// Never persisted or inferred from a model-provided thread ID.
    pub(crate) review_origin_thread_id: Option<codex_protocol::ThreadId>,
    pub(crate) complexity_class: Option<AdaptiveComplexityClass>,
    pub(crate) implementation_phase: Option<AdaptiveImplementationPhase>,
    pub(crate) validation_repair_fingerprint: Option<String>,
    pub(crate) validation_repair_cycles_used: u8,
    pub(crate) validation_targeted_retest_required: bool,
    pub(crate) validation_status: Option<AdaptiveValidationStatus>,
    pub(crate) budget_mode: AdaptiveBudgetMode,
}

impl AdaptiveFamily {
    pub(crate) fn parse(value: &str) -> Option<Self> {
        match value {
            "luna" => Some(Self::Luna),
            // 0.3.2 removes Terra from the active GPT-6 family ladder. Accept the old spelling
            // as a compatibility alias so persisted/operator input upgrades cleanly to Sol.
            "terra" | "sol" => Some(Self::Sol),
            "sol61" => Some(Self::Sol61),
            "astra" => Some(Self::Astra),
            _ => None,
        }
    }

    pub(crate) fn model(self) -> &'static str {
        match self {
            Self::Luna => "gpt-6-luna",
            Self::Sol => "gpt-6-sol",
            Self::Sol61 => "gpt-6.1-sol",
            Self::Astra => "gpt-6-astra",
        }
    }
}

impl AdaptiveEffortState {
    fn fresh_for_new_worker(&self, binding: Option<&NewWorkerBinding>) -> Self {
        let enabled = self.enabled || binding.is_some();
        let worker_context = binding.map_or_else(AdaptiveWorkerContext::default, |binding| {
            AdaptiveWorkerContext {
                role: binding.role,
                authorized_scope: Some(binding.authorized_scope.clone()),
            }
        });
        let default_initial_route = || {
            crate::adaptive_policy::initial_route(
                self.starting_family.unwrap_or(AdaptiveFamily::Luna),
            )
        };
        let initial_route = match worker_context.role {
            AdaptiveWorkerRole::Validation => quality_review_route(self.budget_mode),
            AdaptiveWorkerRole::Implementation => self
                .complexity_class
                .map(implementation_floor)
                .unwrap_or_else(default_initial_route),
            AdaptiveWorkerRole::Unspecified | AdaptiveWorkerRole::Repair => default_initial_route(),
        };
        // Preserve the same pinned source for the automatically spawned independent
        // validation of this exact scope, never for an unrelated fresh assignment.
        let assignment_preflight = if self.worker_assignment_locked
            && self.worker_context.role == AdaptiveWorkerRole::Implementation
            && self.workflow_terminal == Some(AdaptiveWorkflowTerminal::ReadyForValidation)
        {
            binding
                .filter(|binding| binding.role == AdaptiveWorkerRole::Validation)
                .and_then(|binding| {
                    self.worker_context
                        .authorized_scope
                        .as_deref()
                        .filter(|scope| scope.trim() == binding.authorized_scope.trim())
                        .and(self.assignment_preflight.clone())
                })
        } else {
            None
        };
        let implementation_phase = (worker_context.role == AdaptiveWorkerRole::Implementation
            && self.complexity_class.is_some())
        .then_some(AdaptiveImplementationPhase::Implementation);
        Self {
            enabled,
            starting_family: self.starting_family,
            current_family: enabled.then_some(initial_route.family),
            current_effort: enabled.then_some(initial_route.effort),
            attempt_number: enabled.then_some(1).unwrap_or_default(),
            worker_context,
            assignment_preflight,
            worker_assignment_locked: binding.is_some(),
            complexity_class: self.complexity_class,
            implementation_phase,
            budget_mode: self.budget_mode,
            ..Default::default()
        }
    }

    /// Handoff authority originates from the terminalized source Worker,
    /// not an arbitrary matching scope or a replayed checkpoint. In-memory
    /// registry refs are copied only so each exact gate can be revalidated.
    fn inherit_authorized_independent_review(
        &mut self,
        previous: &Self,
        binding: &NewWorkerBinding,
        origin_thread_id: codex_protocol::ThreadId,
    ) {
        let Some(pin) = self.assignment_preflight.as_ref() else {
            return;
        };
        let Some(ledger) = previous.gate_ledger.as_ref() else {
            return;
        };
        if binding.role != AdaptiveWorkerRole::Validation
            || previous.worker_context.role != AdaptiveWorkerRole::Implementation
            || !previous.worker_assignment_locked
            || previous.workflow_terminal != Some(AdaptiveWorkflowTerminal::ReadyForValidation)
            || previous.worker_context.authorized_scope.as_deref()
                != Some(binding.authorized_scope.as_str())
            || pin.task_kind != crate::adaptive_worker::AdaptiveTaskKind::SourceChange
            || !ledger.matches_authorized_source(
                &binding.authorized_scope,
                std::path::Path::new(&pin.source_root),
                &pin.source_sha,
            )
        {
            return;
        }
        self.gate_ledger = Some(ledger.clone());
        self.evidence_registry = previous.evidence_registry.clone();
        self.review_origin_thread_id = Some(origin_thread_id);
    }

    pub(crate) fn activate_adaptive_startup(&mut self, preferred_family: AdaptiveFamily) {
        if self.paused_by_user || self.workflow_terminal.is_some() {
            return;
        }
        self.activate_adaptive_family(preferred_family, false);
    }

    pub(crate) fn implementation_reconnaissance_required(&self) -> bool {
        self.enabled
            && !self.paused_by_user
            && self.workflow_terminal.is_none()
            && self.worker_context.role == AdaptiveWorkerRole::Implementation
            && self
                .worker_context
                .authorized_scope
                .as_deref()
                .is_some_and(|scope| !scope.trim().is_empty())
            && self.complexity_class.is_none()
    }

    /// Bind the direct-only lifecycle signal tool to the trusted Implementation
    /// turn. CodeModeOnly cannot call this control tool through its nested
    /// executor, but ordinary non-Worker turns must retain their tool mode.
    pub(crate) fn implementation_control_signal_required(&self) -> bool {
        self.enabled
            && !self.paused_by_user
            && self.workflow_terminal.is_none()
            && self.worker_context.role == AdaptiveWorkerRole::Implementation
            && self
                .worker_context
                .authorized_scope
                .as_deref()
                .is_some_and(|scope| !scope.trim().is_empty())
            && self.complexity_class.is_some()
            // Legacy/inconsistent state with accepted complexity but no phase
            // can request a trusted Mechanical Validation handoff. The phase
            // is never inferred to be Mechanical Validation by this predicate.
            && matches!(
                self.implementation_phase,
                None | Some(AdaptiveImplementationPhase::Implementation)
            )
    }

    pub(crate) fn implementation_mechanical_validation_active(&self) -> bool {
        self.enabled
            && !self.paused_by_user
            && self.workflow_terminal.is_none()
            && self.worker_context.role == AdaptiveWorkerRole::Implementation
            && self
                .worker_context
                .authorized_scope
                .as_deref()
                .is_some_and(|scope| !scope.trim().is_empty())
            && self.complexity_class.is_some()
            && self.implementation_phase == Some(AdaptiveImplementationPhase::MechanicalValidation)
    }

    pub(crate) fn record_validation_repair_cycle(&mut self, fingerprint: String) {
        self.validation_repair_fingerprint = Some(fingerprint);
        self.validation_repair_cycles_used = self.validation_repair_cycles_used.saturating_add(1);
        self.validation_targeted_retest_required = true;
    }

    pub(crate) fn validation_repair_budget_available(&self) -> bool {
        self.validation_repair_cycles_used < CODEXDD_VALIDATION_REPAIR_LIMIT
    }

    pub(crate) fn clear_validation_repair_state(&mut self) {
        self.validation_repair_fingerprint = None;
        self.validation_repair_cycles_used = 0;
        self.validation_targeted_retest_required = false;
    }

    pub(crate) fn observe_worker_assignment_text(&mut self, text: &str) -> Result<bool, String> {
        if self.worker_assignment_locked {
            return Ok(false);
        }
        self.worker_assignment_locked = true;
        match parse_adaptive_owner_assignment(text) {
            Ok(Some((worker_context, assignment_preflight, gate_ledger))) => {
                self.worker_context = worker_context;
                self.assignment_preflight = assignment_preflight;
                self.gate_ledger = gate_ledger;
                Ok(true)
            }
            Ok(None) => Ok(false),
            Err(err) => Err(err),
        }
    }

    fn activate_adaptive_family(&mut self, preferred_family: AdaptiveFamily, clear_pause: bool) {
        let worker_context = self.worker_context.clone();
        let assignment_preflight = self.assignment_preflight.clone();
        let worker_assignment_locked = self.worker_assignment_locked
            || worker_context.role != AdaptiveWorkerRole::Unspecified
            || worker_context.authorized_scope.is_some();
        let workflow_terminal = self.workflow_terminal;
        let evidence_registry = self.evidence_registry.clone();
        let gate_ledger = self.gate_ledger.clone();
        let review_origin_thread_id = self.review_origin_thread_id;
        let complexity_class = self.complexity_class;
        let implementation_phase = self.implementation_phase;
        let consecutive_no_native_progress_turns = self.consecutive_no_native_progress_turns;
        let validation_repair_fingerprint = self.validation_repair_fingerprint.clone();
        let validation_repair_cycles_used = self.validation_repair_cycles_used;
        let validation_targeted_retest_required = self.validation_targeted_retest_required;
        let validation_status = self.validation_status.clone();
        let budget_mode = self.budget_mode;
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
            assignment_preflight,
            worker_assignment_locked,
            workflow_terminal,
            transient_retry_consumed: false,
            unfinished_turn_pressure: 0,
            consecutive_no_native_progress_turns,
            last_processed_terminal_turn_id: None,
            pending_attempt: None,
            successor_admission: None,
            pending_signal: None,
            evidence_registry,
            gate_ledger,
            review_origin_thread_id,
            complexity_class,
            implementation_phase,
            validation_repair_fingerprint,
            validation_repair_cycles_used,
            validation_targeted_retest_required,
            validation_status,
            budget_mode,
        };
    }

    pub(crate) fn durable_workflow_snapshot(
        &self,
        owner_thread_id: codex_protocol::ThreadId,
    ) -> codex_app_server_protocol::ThreadAdaptiveWorkflowState {
        let family_name = |family: AdaptiveFamily| match family {
            AdaptiveFamily::Luna => "luna",
            AdaptiveFamily::Sol => "sol",
            AdaptiveFamily::Sol61 => "sol61",
            AdaptiveFamily::Astra => "astra",
        };
        let effort_name = |effort: AdaptiveEffort| match effort {
            AdaptiveEffort::Low => "low",
            AdaptiveEffort::Medium => "medium",
            AdaptiveEffort::High => "high",
            AdaptiveEffort::XHigh => "xhigh",
            AdaptiveEffort::Max => "max",
        };
        let worker_role = match self.worker_context.role {
            AdaptiveWorkerRole::Unspecified => "unspecified",
            AdaptiveWorkerRole::Implementation => "implementation",
            AdaptiveWorkerRole::Validation => "validation",
            AdaptiveWorkerRole::Repair => "repair",
        };
        let workflow_terminal = self.workflow_terminal.map(|terminal| match terminal {
            AdaptiveWorkflowTerminal::ReadyForValidation => "ready_for_validation",
            AdaptiveWorkflowTerminal::RepairRequired => "repair_required",
            AdaptiveWorkflowTerminal::ReadyForOwnerQa => "ready_for_owner_qa",
            AdaptiveWorkflowTerminal::Blocked => "blocked",
        });

        codex_app_server_protocol::ThreadAdaptiveWorkflowState {
            enabled: self.enabled,
            starting_family: self.starting_family.map(family_name).map(str::to_string),
            current_family: self.current_family.map(family_name).map(str::to_string),
            current_effort: self.current_effort.map(effort_name).map(str::to_string),
            complexity_class: self.complexity_class.map(|class| {
                match class {
                    AdaptiveComplexityClass::Routine => "routine",
                    AdaptiveComplexityClass::Standard => "standard",
                    AdaptiveComplexityClass::Complex => "complex",
                    AdaptiveComplexityClass::Architectural => "architectural",
                }
                .to_string()
            }),
            implementation_phase: self
                .implementation_phase
                .map(AdaptiveImplementationPhase::persisted_label)
                .map(str::to_string),
            validation_repair_fingerprint: self.validation_repair_fingerprint.clone(),
            validation_repair_cycles_used: Some(self.validation_repair_cycles_used),
            validation_targeted_retest_required: Some(self.validation_targeted_retest_required),
            validation_status: self.validation_status.as_ref().map(|status| {
                codex_app_server_protocol::ThreadAdaptiveValidationStatus {
                    profile: status.profile.clone(),
                    result: status.result.clone(),
                    run_id: status.run_id.clone(),
                    branch: status.branch.clone(),
                    head_sha: status.head_sha.clone(),
                    failed_stage: status.failed_stage.clone(),
                    failure_fingerprint: status.failure_fingerprint.clone(),
                    log_path: status.log_path.clone(),
                }
            }),
            attempt_number: self.attempt_number,
            paused_by_user: self.paused_by_user,
            worker_role: worker_role.to_string(),
            authorized_scope: self.worker_context.authorized_scope.clone(),
            assignment_task_kind: self.assignment_preflight.as_ref().map(|pin| {
                match pin.task_kind {
                    crate::adaptive_worker::AdaptiveTaskKind::SourceChange => "source_change",
                    crate::adaptive_worker::AdaptiveTaskKind::OperationsPreview => {
                        "operations_preview"
                    }
                    crate::adaptive_worker::AdaptiveTaskKind::Benchmark => "benchmark",
                }
                .to_string()
            }),
            authorized_source_root: self
                .assignment_preflight
                .as_ref()
                .map(|pin| pin.source_root.clone()),
            authorized_source_sha: self
                .assignment_preflight
                .as_ref()
                .map(|pin| pin.source_sha.clone()),
            gate_checkpoint: self.gate_ledger.as_ref().and_then(|ledger| {
                // The generated value contains requirements and blockers only.
                // If the Worker is unbound, never serialize an authority hint.
                ledger
                    .checkpoint_for_worker(owner_thread_id, self.worker_context.role)
                    .ok()
                    .and_then(|checkpoint| serde_json::to_value(checkpoint).ok())
            }),
            worker_assignment_locked: self.worker_assignment_locked,
            workflow_terminal: workflow_terminal.map(str::to_string),
        }
    }
}

impl ChatWidget {
    #[cfg(test)]
    pub(crate) fn adaptive_effort_for_test(&self) -> &AdaptiveEffortState {
        &self.adaptive_effort
    }

    #[cfg(test)]
    pub(crate) fn adaptive_effort_for_test_mut(&mut self) -> &mut AdaptiveEffortState {
        &mut self.adaptive_effort
    }

    pub(crate) fn adaptive_implementation_reconnaissance_required(&self) -> bool {
        self.adaptive_effort
            .implementation_reconnaissance_required()
    }

    pub(crate) fn adaptive_implementation_control_signal_required(&self) -> bool {
        self.adaptive_effort
            .implementation_control_signal_required()
    }

    pub(crate) fn adaptive_implementation_mechanical_validation_active(&self) -> bool {
        self.adaptive_effort
            .implementation_mechanical_validation_active()
    }

    pub(crate) fn adaptive_validation_terminalization_active(&self) -> bool {
        matches!(
            self.adaptive_effort.successor_admission.as_ref(),
            Some(AdaptiveSuccessorAdmission::Consumed(permit))
                if permit.decision == AdaptivePendingDecision::ValidationTerminalization
        ) && self.adaptive_effort.workflow_terminal.is_none()
    }

    pub(crate) fn automatic_validation_binding(&self) -> Option<NewWorkerBinding> {
        let state = &self.adaptive_effort;
        if state.workflow_terminal != Some(AdaptiveWorkflowTerminal::ReadyForValidation)
            || !matches!(
                state.worker_context.role,
                AdaptiveWorkerRole::Implementation | AdaptiveWorkerRole::Repair
            )
            || !matches!(
                state.complexity_class,
                Some(
                    AdaptiveComplexityClass::Standard
                        | AdaptiveComplexityClass::Complex
                        | AdaptiveComplexityClass::Architectural
                )
            )
        {
            return None;
        }
        let authorized_scope = state
            .worker_context
            .authorized_scope
            .as_deref()
            .map(str::trim)
            .filter(|scope| !scope.is_empty())?
            .to_string();
        Some(NewWorkerBinding {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope,
        })
    }

    /// A newly queued automatic handoff is only authoritative while its
    /// source Implementation Worker is still displayed and terminalized.
    /// A generic manually requested Validation session is not this event.
    pub(crate) fn automatic_validation_source_sha(
        &self,
        binding: &NewWorkerBinding,
    ) -> Option<&str> {
        let state = &self.adaptive_effort;
        if self.automatic_validation_binding().as_ref() != Some(binding)
            || state.worker_context.role != AdaptiveWorkerRole::Implementation
            || !state.worker_assignment_locked
        {
            return None;
        }
        let pin = state.assignment_preflight.as_ref()?;
        if pin.task_kind != crate::adaptive_worker::AdaptiveTaskKind::SourceChange
            || !state.gate_ledger.as_ref()?.matches_authorized_source(
                &binding.authorized_scope,
                std::path::Path::new(&pin.source_root),
                &pin.source_sha,
            )
        {
            return None;
        }
        Some(pin.source_sha.as_str())
    }

    pub(crate) fn automatic_validation_prompt_for_new_worker(
        &self,
        binding: &NewWorkerBinding,
    ) -> Option<UserMessage> {
        let expected = self.automatic_validation_binding()?;
        if &expected != binding {
            return None;
        }
        let complexity = self.adaptive_effort.complexity_class?.label();
        Some(
            format!(
                "[Adaptive quality review] Independently validate the completed implementation in the current working tree for the exact authorized scope: {}. Complexity class: {}. Do not assume the Implementation/Repair Worker was correct, and do not modify the implementation while acting as Validation. Inspect coupling, duplication, abstractions, architectural fit, unintended state/API/concurrency effects, unnecessary complexity, and test quality. Run objective validation appropriate to the scope. For an authorized pinned source, call the direct built-in run_codexdd_independent_validation tool with profile=independent_review; a generic command PASS or the Implementation-only runner is not an independent review. Include the native independent-review receipt ID in the successful evidence refs when reporting ready_for_owner_qa. Any remaining source, runtime or independent gate means READY must stay pending. If a blocker is found, report repair_required with failing native evidence refs.",
                binding.authorized_scope,
                complexity,
            )
            .into(),
        )
    }

    pub(crate) fn fresh_adaptive_effort_for_new_worker(
        &self,
        binding: Option<&NewWorkerBinding>,
    ) -> AdaptiveEffortState {
        let mut fresh = self.adaptive_effort.fresh_for_new_worker(binding);
        if let (Some(binding), Some(origin_thread_id)) = (binding, self.thread_id) {
            fresh.inherit_authorized_independent_review(
                &self.adaptive_effort,
                binding,
                origin_thread_id,
            );
        }
        fresh
    }

    pub(super) fn save_adaptive_effort_for_current_thread(&self) {
        self.app_event_tx.send(AppEvent::UpdateAdaptiveEffortState(
            self.adaptive_effort.clone(),
        ));
        if let Some(thread_id) = self.thread_id {
            self.app_event_tx.send(AppEvent::PersistWorkflowState {
                thread_id,
                operation:
                    codex_app_server_protocol::ThreadWorkflowStateOperation::SetAdaptiveState,
                source_turn_id: self
                    .adaptive_effort
                    .workflow_terminal
                    .and_then(|_| self.adaptive_effort.last_processed_terminal_turn_id.clone()),
                adaptive_state: Some(self.adaptive_effort.durable_workflow_snapshot(thread_id)),
            });
        }
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
                    adaptive_state: None,
                });
            }
            family => match AdaptiveFamily::parse(family) {
                Some(preferred_family) => {
                    self.activate_adaptive_family(preferred_family, true);
                }
                None => self.add_error_message(
                    "Usage: /adaptive [luna|sol|astra|status|pause|resume|off|reset]".to_string(),
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
            self.adaptive_effort.consecutive_no_native_progress_turns = 0;
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
        let message = match operation {
            codex_app_server_protocol::ThreadWorkflowStateOperation::SetAdaptiveState => {
                if self.adaptive_effort.workflow_terminal.is_some() {
                    format!(
                        "Failed to persist adaptive workflow state; the in-memory workflow terminal remains authoritative for this session, but resume may not restore it: {error}"
                    )
                } else {
                    format!(
                        "Failed to persist adaptive workflow state; resume may not restore the latest Worker state: {error}"
                    )
                }
            }
            _ => format!(
                "Failed to persist workflow state; adaptive automation remains stopped: {error}"
            ),
        };
        self.add_error_message(message);
    }

    pub(super) fn add_adaptive_status_output(&mut self) {
        self.add_info_message(self.adaptive_effort_status_text(), None);
    }

    pub(super) fn adaptive_effort_status_text(&self) -> String {
        let state = &self.adaptive_effort;
        let family = |value| match value {
            Some(AdaptiveFamily::Luna) => "Luna",
            Some(AdaptiveFamily::Sol) => "Sol",
            Some(AdaptiveFamily::Sol61) => "Sol 6.1",
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
        let complexity = state
            .complexity_class
            .map(AdaptiveComplexityClass::label)
            .unwrap_or("None");
        let implementation_phase = state
            .implementation_phase
            .map(AdaptiveImplementationPhase::label)
            .unwrap_or_else(|| {
                if state.worker_context.role == AdaptiveWorkerRole::Implementation
                    && state.complexity_class.is_none()
                {
                    "Reconnaissance"
                } else {
                    "None"
                }
            });
        let complexity_floor = state.complexity_class.map_or_else(
            || "None".to_string(),
            |class| {
                let route = if state.implementation_phase
                    == Some(AdaptiveImplementationPhase::MechanicalValidation)
                {
                    mechanical_validation_route(state.budget_mode)
                } else {
                    implementation_floor(class)
                };
                let effort = match route.effort {
                    AdaptiveEffort::Low => "Low",
                    AdaptiveEffort::Medium => "Medium",
                    AdaptiveEffort::High => "High",
                    AdaptiveEffort::XHigh => "XHigh",
                    AdaptiveEffort::Max => "Max",
                };
                format!("{} {effort}", family(Some(route.family)))
            },
        );
        let validation_last = state.validation_status.as_ref().map_or_else(
            || "not run".to_string(),
            |status| format!("{} {}", status.result.to_ascii_uppercase(), status.profile),
        );
        let validation_run = state
            .validation_status
            .as_ref()
            .map(|status| status.run_id.as_str())
            .unwrap_or("None");
        let validation_candidate = state.validation_status.as_ref().map_or_else(
            || "None".to_string(),
            |status| {
                format!(
                    "{} @ {}",
                    status.branch.as_deref().unwrap_or("(detached)"),
                    status.head_sha
                )
            },
        );
        let validation_failed_stage = state
            .validation_status
            .as_ref()
            .and_then(|status| status.failed_stage.as_deref())
            .unwrap_or("None");
        let validation_log = state
            .validation_status
            .as_ref()
            .map(|status| status.log_path.as_str())
            .unwrap_or("None");
        let validation_failure_fingerprint = state
            .validation_status
            .as_ref()
            .and_then(|status| status.failure_fingerprint.as_deref())
            .or(state.validation_repair_fingerprint.as_deref())
            .unwrap_or("None");
        let validation_repair_remaining =
            CODEXDD_VALIDATION_REPAIR_LIMIT.saturating_sub(state.validation_repair_cycles_used);
        let validation_next = match state.workflow_terminal {
            Some(AdaptiveWorkflowTerminal::ReadyForValidation) => "handoff ready",
            Some(AdaptiveWorkflowTerminal::Blocked) => "owner review required",
            _ if state.worker_context.role != AdaptiveWorkerRole::Implementation => "n/a",
            _ => match state.implementation_phase {
                Some(AdaptiveImplementationPhase::MechanicalValidation)
                    if state.validation_targeted_retest_required =>
                {
                    "targeted -> work_packet"
                }
                Some(AdaptiveImplementationPhase::MechanicalValidation) => "work_packet",
                Some(AdaptiveImplementationPhase::Implementation)
                    if state.validation_repair_cycles_used > 0 =>
                {
                    "repair -> mechanical_validation"
                }
                Some(AdaptiveImplementationPhase::Implementation) => {
                    "implementation -> mechanical_validation"
                }
                None if state.complexity_class.is_none() => "reconnaissance",
                None => "implementation",
            },
        };
        let source_pin = state.assignment_preflight.as_ref().map_or_else(
            || "None (do not infer a worktree from the launch cwd)".to_string(),
            |pin| format!("{} @ {}", pin.source_root, pin.source_sha),
        );
        let pending_gates = state.gate_ledger.as_ref().map_or_else(
            || "Not specified by an authorized owner".to_string(),
            |ledger| {
                let outstanding = ledger.outstanding(&state.evidence_registry);
                if outstanding.is_empty() {
                    "None (still requires a trusted terminal signal)".to_string()
                } else {
                    outstanding
                        .iter()
                        .map(|item| {
                            let name = match &item.gate {
                                crate::adaptive_gate_prerequisites::AdaptiveRequiredGate::SourceProfile(profile) => {
                                    format!("source:{profile}")
                                }
                                crate::adaptive_gate_prerequisites::AdaptiveRequiredGate::OwnedRuntimeHealth(ports) => {
                                    format!("runtime:{ports:?}")
                                }
                                crate::adaptive_gate_prerequisites::AdaptiveRequiredGate::IndependentValidation => {
                                    "independent_review".to_string()
                                }
                            };
                            let status = match &item.state {
                                crate::adaptive_gate_prerequisites::AdaptiveGateState::Pending => {
                                    "PENDING"
                                }
                                crate::adaptive_gate_prerequisites::AdaptiveGateState::Blocked(_) => {
                                    "BLOCKED"
                                }
                            };
                            format!("{name} {status}")
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            },
        );
        let native_tool = match state.worker_context.role {
            AdaptiveWorkerRole::Implementation => {
                "run_codexdd_validation (Core live tool/profile availability not inferred from historical LVO status)"
            }
            AdaptiveWorkerRole::Validation => {
                "run_codexdd_independent_validation (Core live tool/profile availability must be verified)"
            }
            AdaptiveWorkerRole::Repair => {
                "Repair cannot substitute for a fresh independent Validator"
            }
            AdaptiveWorkerRole::Unspecified => {
                "Not authorized until the first user-origin Worker assignment"
            }
        };
        let next_action = if !state.enabled {
            "Enable adaptive mode, then supply one owner-origin Worker assignment."
        } else if !worker_is_bound {
            "Start the first nonblank user message with [adaptive_worker], role and authorized_scope; source changes also require source_root, source_sha, task_kind and required_gates."
        } else if state.paused_by_user {
            "Hold: confirm the original source and owner prerequisites; resume only with fresh native proof. Historic READY is not authority."
        } else if state.workflow_terminal == Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa) {
            "Await independent owner QA approval; do not auto-release or reuse this READY after restart."
        } else if state.workflow_terminal == Some(AdaptiveWorkflowTerminal::ReadyForValidation) {
            "Start exactly one fresh independent Validator for the current source SHA and owner gates."
        } else if state.workflow_terminal == Some(AdaptiveWorkflowTerminal::Blocked) {
            "Owner review required; preserve task state and do not escalate or fabricate evidence."
        } else if state.assignment_preflight.as_ref().is_some_and(|pin| {
            pin.task_kind != crate::adaptive_worker::AdaptiveTaskKind::SourceChange
        }) {
            "Non-source operations/benchmark task: wait for an explicitly authorized operations-specific path. Do not run source work_packet or claim mechanical source PASS."
        } else if state.gate_ledger.is_none() && state.assignment_preflight.is_some() {
            "Source-pinned Worker needs an explicit owner gate manifest; no READY or Validator handoff."
        } else if state.worker_context.role == AdaptiveWorkerRole::Validation {
            "Run a fresh Core-native independent_review against the exact pinned source; old source receipts cannot satisfy this gate."
        } else if state.worker_context.role == AdaptiveWorkerRole::Implementation
            && state.implementation_phase == Some(AdaptiveImplementationPhase::MechanicalValidation)
        {
            "Run current Core-native work_packet (and targeted if required) for the pinned SHA; inspect tool/profile setup blockers."
        } else {
            "Continue the authorized Worker route; check first-message role, source pin and actual Core tool/profile readiness before claiming completion."
        };
        let codexdd_identity = codex_build_info::codexdd_compact_identity();
        let report = format!(
            "Adaptive Effort\n  codexdd: {}\n  Enabled: {}\n  Preference: {}\n  Current: {} {}\n  Budget mode: {}\n  Complexity: {}\n  Implementation phase: {}\n  Implementation floor: {}\n  Attempt: {}\n  Failure pressure: {}/{}\n  Unfinished pressure: {}/{}\n  Paused: {}\n  Last outcome: {}\n  Last failure: {}\n  Worker role: {}\n  Worker scope: {}\n  Worker binding: {}\n  Workflow terminal: {}\n  LVO last: {}\n  LVO run: {}\n  LVO candidate: {}\n  LVO failed stage: {}\n  LVO failure fingerprint: {}\n  LVO repair budget: {}/{} used ({} remaining)\n  LVO next: {}\n  LVO log: {}",
            codexdd_identity,
            if state.enabled { "yes" } else { "no" },
            family(state.starting_family),
            family(state.current_family),
            effort,
            state.budget_mode.label(),
            complexity,
            implementation_phase,
            complexity_floor,
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
            workflow_terminal,
            validation_last,
            validation_run,
            validation_candidate,
            validation_failed_stage,
            validation_failure_fingerprint,
            state.validation_repair_cycles_used,
            CODEXDD_VALIDATION_REPAIR_LIMIT,
            validation_repair_remaining,
            validation_next,
            validation_log
        );
        format!(
            "{report}\n  Authorized source: {source_pin}\n  Pending prerequisites: {pending_gates}\n  Native tool required: {native_tool}\n  Next action: {next_action}"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALIDATION_ASSIGNMENT: &str = "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"Z-A0.45B-R2 independent verification\"\n\nReview the exact pushed R2 candidate.";

    fn codexdd_046_3c1b2c_source_parent(
        root: &std::path::Path,
        scope: &str,
    ) -> AdaptiveEffortState {
        use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;
        let sha = "a".repeat(40);
        let mut state = AdaptiveEffortState {
            enabled: true,
            worker_context: AdaptiveWorkerContext {
                role: AdaptiveWorkerRole::Implementation,
                authorized_scope: Some(scope.to_string()),
            },
            worker_assignment_locked: true,
            assignment_preflight: Some(AdaptiveWorkerPreflight {
                task_kind: crate::adaptive_worker::AdaptiveTaskKind::SourceChange,
                source_root: root.display().to_string(),
                source_sha: sha.clone(),
            }),
            workflow_terminal: Some(AdaptiveWorkflowTerminal::ReadyForValidation),
            ..Default::default()
        };
        state.gate_ledger = Some(
            AdaptiveGateLedger::new(
                scope,
                root,
                &sha,
                vec![
                    AdaptiveRequiredGate::SourceProfile("work_packet".to_string()),
                    AdaptiveRequiredGate::IndependentValidation,
                ],
            )
            .expect("owner-specified gate requirements"),
        );
        state
    }

    #[test]
    fn codexdd_046_3c1b2c_live_handoff_preserves_exact_source_gates_without_terminal() {
        let root = tempfile::tempdir().expect("isolated candidate");
        let scope = "isolated authorized task";
        let parent = codexdd_046_3c1b2c_source_parent(root.path(), scope);
        let parent_thread_id = codex_protocol::ThreadId::new();
        let binding = NewWorkerBinding {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: scope.to_string(),
        };
        let mut validator = parent.fresh_for_new_worker(Some(&binding));
        validator.inherit_authorized_independent_review(&parent, &binding, parent_thread_id);
        assert_eq!(validator.review_origin_thread_id, Some(parent_thread_id));
        assert_eq!(validator.assignment_preflight, parent.assignment_preflight);
        assert_eq!(validator.gate_ledger, parent.gate_ledger);
        assert_eq!(validator.workflow_terminal, None);
        assert_eq!(
            validator
                .gate_ledger
                .as_ref()
                .expect("transferred requirements")
                .outstanding(&validator.evidence_registry)
                .len(),
            2,
            "no source or independent success is inferred from the transfer"
        );
        let persisted = serde_json::to_value(
            validator.durable_workflow_snapshot(codex_protocol::ThreadId::new()),
        )
        .expect("requirements-only snapshot");
        assert!(
            persisted.get("reviewOriginThreadId").is_none(),
            "source Worker identity is ephemeral, never restored as READY authority"
        );
    }

    #[test]
    fn codexdd_046_3c1b2c_handoff_rejects_unterminalized_foreign_or_unpinned_source() {
        let root = tempfile::tempdir().expect("isolated candidate");
        let scope = "isolated authorized task";
        let parent = codexdd_046_3c1b2c_source_parent(root.path(), scope);
        let parent_thread_id = codex_protocol::ThreadId::new();
        let binding = NewWorkerBinding {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: scope.to_string(),
        };

        let mut not_ready = parent.clone();
        not_ready.workflow_terminal = None;
        let mut validator = not_ready.fresh_for_new_worker(Some(&binding));
        validator.inherit_authorized_independent_review(&not_ready, &binding, parent_thread_id);
        assert_eq!(validator.review_origin_thread_id, None);
        assert!(validator.gate_ledger.is_none());

        let wrong_scope = NewWorkerBinding {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: "other task".to_string(),
        };
        let mut foreign = parent.fresh_for_new_worker(Some(&wrong_scope));
        foreign.inherit_authorized_independent_review(&parent, &wrong_scope, parent_thread_id);
        assert_eq!(foreign.review_origin_thread_id, None);
        assert!(foreign.gate_ledger.is_none());
        assert!(foreign.assignment_preflight.is_none());

        let mut wrong_source = parent.clone();
        wrong_source
            .assignment_preflight
            .as_mut()
            .expect("pin")
            .source_sha = "b".repeat(40);
        let mut rejected = wrong_source.fresh_for_new_worker(Some(&binding));
        rejected.inherit_authorized_independent_review(&wrong_source, &binding, parent_thread_id);
        assert_eq!(rejected.review_origin_thread_id, None);
        assert!(rejected.gate_ledger.is_none());
    }

    #[test]
    fn plain_adaptive_startup_waits_for_external_worker_binding() {
        let mut state = AdaptiveEffortState::default();
        state.activate_adaptive_startup(AdaptiveFamily::Astra);

        assert!(state.enabled);
        assert_eq!(state.worker_context, AdaptiveWorkerContext::default());
        assert!(!state.worker_assignment_locked);
    }

    #[test]
    fn fresh_roleful_worker_preserves_preference_but_resets_lifecycle_state() {
        let mut previous = AdaptiveEffortState::default();
        previous.activate_adaptive_startup(AdaptiveFamily::Astra);
        previous.worker_context = AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Implementation,
            authorized_scope: Some("old scope".to_string()),
        };
        previous.worker_assignment_locked = true;
        previous.workflow_terminal = Some(AdaptiveWorkflowTerminal::ReadyForValidation);
        previous.last_outcome = Some(AdaptiveOutcome::ReadyForOwnerQa);
        previous.last_failure_kind = Some(AdaptiveFailureKind::Capability);
        previous.unfinished_turn_pressure = 1;
        previous
            .evidence_registry
            .register(crate::adaptive_evidence::AdaptiveEvidenceRecord {
                evidence_id: "old-evidence".to_string(),
                thread_id: codex_protocol::ThreadId::new(),
                source_turn_id: "old-turn".to_string(),
                outcome: crate::adaptive_evidence::AdaptiveEvidenceOutcome::Failure,
                kind: crate::adaptive_evidence::AdaptiveEvidenceKind::CommandExecution,
                tool_name: None,
                validation_profile: None,
                validation_failure_fingerprint: None,
                native_source_provenance: None,
                worker_binding: None,
            });
        let binding = NewWorkerBinding {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: " exact new scope ".to_string(),
        };

        assert_eq!(
            previous.fresh_for_new_worker(Some(&binding)),
            AdaptiveEffortState {
                enabled: true,
                starting_family: Some(AdaptiveFamily::Astra),
                current_family: Some(AdaptiveFamily::Sol),
                current_effort: Some(AdaptiveEffort::Low),
                attempt_number: 1,
                worker_context: AdaptiveWorkerContext {
                    role: AdaptiveWorkerRole::Validation,
                    authorized_scope: Some(" exact new scope ".to_string()),
                },
                worker_assignment_locked: true,
                ..Default::default()
            }
        );
    }

    #[test]
    fn validation_worker_floor_tracks_budget_without_affecting_implementation() {
        let binding = |role| NewWorkerBinding {
            role,
            authorized_scope: "quality gate".to_string(),
        };

        for (budget_mode, family, effort) in [
            (
                AdaptiveBudgetMode::Conserve,
                AdaptiveFamily::Luna,
                AdaptiveEffort::High,
            ),
            (
                AdaptiveBudgetMode::Balanced,
                AdaptiveFamily::Sol,
                AdaptiveEffort::Low,
            ),
            (
                AdaptiveBudgetMode::Surplus,
                AdaptiveFamily::Sol61,
                AdaptiveEffort::Medium,
            ),
        ] {
            let previous = AdaptiveEffortState {
                enabled: true,
                starting_family: Some(AdaptiveFamily::Astra),
                current_family: Some(AdaptiveFamily::Luna),
                current_effort: Some(AdaptiveEffort::Low),
                attempt_number: 1,
                budget_mode,
                ..Default::default()
            };
            let validation =
                previous.fresh_for_new_worker(Some(&binding(AdaptiveWorkerRole::Validation)));
            assert_eq!(validation.current_family, Some(family));
            assert_eq!(validation.current_effort, Some(effort));
            assert_eq!(validation.budget_mode, budget_mode);

            let implementation =
                previous.fresh_for_new_worker(Some(&binding(AdaptiveWorkerRole::Implementation)));
            assert_eq!(
                implementation.current_family,
                Some(AdaptiveFamily::Luna),
                "budget mode must not silently raise implementation before complexity authorization"
            );
            assert_eq!(implementation.current_effort, Some(AdaptiveEffort::Low));
        }
    }

    #[test]
    fn implementation_worker_uses_bounded_complexity_floor() {
        let binding = |role| NewWorkerBinding {
            role,
            authorized_scope: "implementation gate".to_string(),
        };

        for (complexity_class, family, effort) in [
            (
                AdaptiveComplexityClass::Routine,
                AdaptiveFamily::Luna,
                AdaptiveEffort::Low,
            ),
            (
                AdaptiveComplexityClass::Standard,
                AdaptiveFamily::Luna,
                AdaptiveEffort::High,
            ),
            (
                AdaptiveComplexityClass::Complex,
                AdaptiveFamily::Sol,
                AdaptiveEffort::Low,
            ),
            (
                AdaptiveComplexityClass::Architectural,
                AdaptiveFamily::Sol61,
                AdaptiveEffort::Low,
            ),
        ] {
            let previous = AdaptiveEffortState {
                enabled: true,
                starting_family: Some(AdaptiveFamily::Astra),
                current_family: Some(AdaptiveFamily::Luna),
                current_effort: Some(AdaptiveEffort::Low),
                attempt_number: 1,
                complexity_class: Some(complexity_class),
                ..Default::default()
            };
            let implementation =
                previous.fresh_for_new_worker(Some(&binding(AdaptiveWorkerRole::Implementation)));
            assert_eq!(implementation.current_family, Some(family));
            assert_eq!(implementation.current_effort, Some(effort));
            assert_eq!(
                implementation.complexity_class,
                Some(complexity_class),
                "complexity authorization should survive the Worker handoff"
            );
            assert_eq!(
                implementation.implementation_phase,
                Some(AdaptiveImplementationPhase::Implementation)
            );

            let repair = previous.fresh_for_new_worker(Some(&binding(AdaptiveWorkerRole::Repair)));
            assert_eq!(repair.current_family, Some(AdaptiveFamily::Luna));
            assert_eq!(repair.current_effort, Some(AdaptiveEffort::Low));
        }
    }

    #[test]
    fn fresh_bare_worker_is_unbound_and_unlocked() {
        let mut previous = AdaptiveEffortState::default();
        previous.activate_adaptive_startup(AdaptiveFamily::Astra);
        previous.worker_context = AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Repair,
            authorized_scope: Some("old scope".to_string()),
        };
        previous.worker_assignment_locked = true;

        assert_eq!(
            previous.fresh_for_new_worker(None),
            AdaptiveEffortState {
                enabled: true,
                starting_family: Some(AdaptiveFamily::Astra),
                current_family: Some(AdaptiveFamily::Luna),
                current_effort: Some(AdaptiveEffort::Low),
                attempt_number: 1,
                ..Default::default()
            }
        );
    }

    #[test]
    fn implementation_reconnaissance_gate_requires_bound_scope_and_missing_complexity() {
        let mut state = AdaptiveEffortState::default();
        state.activate_adaptive_startup(AdaptiveFamily::Astra);
        state.worker_context = AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Implementation,
            authorized_scope: Some("bounded implementation".to_string()),
        };
        state.worker_assignment_locked = true;

        assert!(state.implementation_reconnaissance_required());

        state.complexity_class = Some(AdaptiveComplexityClass::Routine);
        assert!(!state.implementation_reconnaissance_required());

        state.complexity_class = None;
        state.paused_by_user = true;
        assert!(!state.implementation_reconnaissance_required());

        state.paused_by_user = false;
        state.worker_context.authorized_scope = Some("   ".to_string());
        assert!(!state.implementation_reconnaissance_required());
    }

    #[test]
    fn implementation_control_signal_requires_bound_nonterminal_worker() {
        let mut state = AdaptiveEffortState {
            enabled: true,
            starting_family: Some(AdaptiveFamily::Astra),
            current_family: Some(AdaptiveFamily::Luna),
            current_effort: Some(AdaptiveEffort::Medium),
            attempt_number: 2,
            worker_context: AdaptiveWorkerContext {
                role: AdaptiveWorkerRole::Implementation,
                authorized_scope: Some("isolated native acceptance".to_string()),
            },
            complexity_class: Some(AdaptiveComplexityClass::Standard),
            implementation_phase: Some(AdaptiveImplementationPhase::Implementation),
            ..Default::default()
        };
        assert!(state.implementation_control_signal_required());
        state.implementation_phase = None;
        assert!(state.implementation_control_signal_required());
        assert!(!state.implementation_mechanical_validation_active());
        state.implementation_phase = Some(AdaptiveImplementationPhase::MechanicalValidation);
        assert!(!state.implementation_control_signal_required());
        assert!(state.implementation_mechanical_validation_active());
        state.complexity_class = None;
        assert!(!state.implementation_control_signal_required());
        state.complexity_class = Some(AdaptiveComplexityClass::Standard);
        state.implementation_phase = Some(AdaptiveImplementationPhase::Implementation);
        state.worker_context.role = AdaptiveWorkerRole::Validation;
        assert!(!state.implementation_control_signal_required());
        state.worker_context.role = AdaptiveWorkerRole::Implementation;
        state.worker_context.authorized_scope = Some(" ".to_string());
        assert!(!state.implementation_control_signal_required());
        state.worker_context.authorized_scope = Some("isolated native acceptance".to_string());
        state.workflow_terminal = Some(AdaptiveWorkflowTerminal::Blocked);
        assert!(!state.implementation_control_signal_required());
    }

    #[test]
    fn mechanical_validation_gate_requires_bound_implementation_phase() {
        let mut state = AdaptiveEffortState {
            enabled: true,
            starting_family: Some(AdaptiveFamily::Astra),
            current_family: Some(AdaptiveFamily::Luna),
            current_effort: Some(AdaptiveEffort::Low),
            attempt_number: 3,
            worker_context: AdaptiveWorkerContext {
                role: AdaptiveWorkerRole::Implementation,
                authorized_scope: Some("bounded implementation".to_string()),
            },
            complexity_class: Some(AdaptiveComplexityClass::Architectural),
            implementation_phase: Some(AdaptiveImplementationPhase::MechanicalValidation),
            ..Default::default()
        };

        assert!(state.implementation_mechanical_validation_active());

        state.implementation_phase = Some(AdaptiveImplementationPhase::Implementation);
        assert!(!state.implementation_mechanical_validation_active());

        state.implementation_phase = Some(AdaptiveImplementationPhase::MechanicalValidation);
        state.paused_by_user = true;
        assert!(!state.implementation_mechanical_validation_active());
    }

    #[test]
    fn codexdd_046_3c1b2e_owner_first_turn_gates_survive_route_but_cannot_be_rewritten() {
        use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;
        use std::process::Command;

        let root = tempfile::tempdir().expect("isolated pinned gate fixture");
        for args in [
            vec!["init", "-q"],
            vec![
                "-c",
                "user.name=CodexDD Gate Fixture",
                "-c",
                "user.email=codexdd@example.test",
                "commit",
                "--allow-empty",
                "-q",
                "-m",
                "owner gate authority",
            ],
        ] {
            let status = Command::new("git")
                .arg("-C")
                .arg(root.path())
                .args(&args)
                .status()
                .expect("Git available");
            assert!(status.success(), "fixture Git command failed");
        }
        let output = Command::new("git")
            .arg("-C")
            .arg(root.path())
            .args(["rev-parse", "HEAD"])
            .output()
            .expect("candidate HEAD");
        assert!(output.status.success());
        let sha = String::from_utf8(output.stdout).expect("UTF8 HEAD");
        let manifest = r#"[{ kind = "source_profile", profile = "work_packet" }, { kind = "independent_validation" }]"#;
        let first = format!(
            "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"locked owner QA task\"\nsource_root = '{}'\nsource_sha = \"{}\"\ntask_kind = \"source_change\"\nrequired_gates = {manifest}\n\nImplement the exact assigned task.",
            root.path().display(),
            sha.trim()
        );
        let mut state = AdaptiveEffortState::default();
        assert_eq!(state.observe_worker_assignment_text(&first), Ok(true));
        let original_ledger = state
            .gate_ledger
            .clone()
            .expect("issued by first owner turn");
        let outstanding = original_ledger.outstanding(&state.evidence_registry);
        assert_eq!(outstanding.len(), 2);
        assert!(
            outstanding
                .iter()
                .any(|item| { item.gate == AdaptiveRequiredGate::IndependentValidation })
        );
        state.activate_adaptive_startup(AdaptiveFamily::Astra);
        state.activate_adaptive_family(AdaptiveFamily::Sol61, false);
        assert_eq!(state.gate_ledger, Some(original_ledger.clone()));
        assert_eq!(
            state.observe_worker_assignment_text(
                "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"different scope\"\nrequired_gates = []\n\nRemove gates."
            ),
            Ok(false),
            "later user turns cannot clear or replace the original requirements"
        );
        assert_eq!(state.gate_ledger, Some(original_ledger));
        let snapshot = state.durable_workflow_snapshot(codex_protocol::ThreadId::new());
        let requirements = snapshot.gate_checkpoint.expect("pending gates are durable");
        assert!(requirements.get("requiredGates").is_some());
        assert!(requirements.get("sourceRefs").is_none());
        assert!(requirements.get("independentReviewRef").is_none());
        assert_eq!(state.workflow_terminal, None);
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
    fn bracketed_paste_first_assignment_binds_and_locks_authority() {
        let mut state = AdaptiveEffortState::default();
        state.activate_adaptive_startup(AdaptiveFamily::Astra);

        let pasted = format!("[200~{VALIDATION_ASSIGNMENT}[201~");
        assert_eq!(state.observe_worker_assignment_text(&pasted), Ok(true));
        assert!(state.worker_assignment_locked);
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
    fn source_pin_survives_adaptive_activation_and_new_model_route() {
        let root = if cfg!(windows) {
            r"C:\approved\candidate"
        } else {
            "/tmp/approved/candidate"
        };
        let first = format!(
            "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"pinned source\"\nsource_root = '{root}'\nsource_sha = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\n\nImplement against the approved checkout."
        );
        let mut state = AdaptiveEffortState::default();
        assert_eq!(state.observe_worker_assignment_text(&first), Ok(true));
        let expected = state
            .assignment_preflight
            .clone()
            .expect("first assignment pin");

        state.activate_adaptive_startup(AdaptiveFamily::Astra);
        assert_eq!(state.assignment_preflight.as_ref(), Some(&expected));
        assert!(state.worker_assignment_locked);
        state.activate_adaptive_family(AdaptiveFamily::Sol61, false);
        assert_eq!(state.assignment_preflight.as_ref(), Some(&expected));
        assert_eq!(
            state.worker_context.authorized_scope.as_deref(),
            Some("pinned source")
        );
    }

    #[test]
    fn pinned_first_assignment_is_immutable_across_later_turns() {
        let root = if cfg!(windows) {
            r"D:\bw58\.worktrees\candidate"
        } else {
            "/tmp/bw58/.worktrees/candidate"
        };
        let first = format!(
            "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"source worktree\"\nsource_root = '{root}'\nsource_sha = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\n\nImplement within the authorized target."
        );
        let mut state = AdaptiveEffortState::default();
        assert_eq!(state.observe_worker_assignment_text(&first), Ok(true));
        assert_eq!(
            state
                .assignment_preflight
                .as_ref()
                .expect("pin")
                .source_root,
            root
        );
        assert_eq!(
            state.observe_worker_assignment_text(
                "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"other\"\n\nSwitch source."
            ),
            Ok(false)
        );
        assert_eq!(
            state
                .assignment_preflight
                .as_ref()
                .expect("immutable pin")
                .source_root,
            root
        );
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

    #[tokio::test]
    async fn codexdd_046_3c2a_status_unbound_explains_first_user_role_and_missing_tool_proof() {
        let (mut chat, _sender, _app_events, _ops) =
            crate::chatwidget::tests::make_chatwidget_manual_with_sender().await;
        chat.adaptive_effort
            .activate_adaptive_startup(AdaptiveFamily::Astra);
        let status = chat.adaptive_effort_status_text();
        assert!(status.contains("Worker binding: Awaiting first assignment"));
        assert!(status.contains("Pending prerequisites: Not specified by an authorized owner"));
        assert!(status.contains(
            "Native tool required: Not authorized until the first user-origin Worker assignment"
        ));
        assert!(
            status.contains(
                "Next action: Start the first nonblank user message with [adaptive_worker]"
            )
        );
        assert!(status.contains("source_root, source_sha, task_kind and required_gates"));
        assert!(!status.contains("READY_FOR_OWNER_QA\n  Authorized source:"));
    }

    #[tokio::test]
    async fn codexdd_046_3c2a_status_reports_exact_pin_pending_gates_and_role_specific_tool() {
        let (mut chat, _sender, _app_events, _ops) =
            crate::chatwidget::tests::make_chatwidget_manual_with_sender().await;
        let source = tempfile::tempdir().expect("disposable source");
        let sha = "a".repeat(40);
        let mut state = codexdd_046_3c1b2c_source_parent(source.path(), "bounded status scope");
        state.complexity_class = Some(AdaptiveComplexityClass::Standard);
        state.implementation_phase = Some(AdaptiveImplementationPhase::MechanicalValidation);
        state.workflow_terminal = None;
        chat.adaptive_effort = state.clone();
        let source_status = chat.adaptive_effort_status_text();
        assert!(source_status.contains(&format!(
            "Authorized source: {} @ {sha}",
            source.path().display()
        )));
        assert!(source_status.contains("source:work_packet PENDING"));
        assert!(source_status.contains("independent_review PENDING"));
        assert!(source_status.contains("Native tool required: run_codexdd_validation"));
        assert!(source_status.contains("Core live tool/profile availability not inferred"));
        assert!(source_status.contains("Next action: Run current Core-native work_packet"));

        state.worker_context.role = AdaptiveWorkerRole::Validation;
        state.review_origin_thread_id = Some(codex_protocol::ThreadId::new());
        chat.adaptive_effort = state;
        let validation_status = chat.adaptive_effort_status_text();
        assert!(
            validation_status.contains("Native tool required: run_codexdd_independent_validation")
        );
        assert!(
            validation_status.contains("Next action: Run a fresh Core-native independent_review")
        );
        assert!(validation_status.contains("independent_review PENDING"));
        assert!(
            !validation_status
                .contains("Next action: Start exactly one fresh independent Validator")
        );
    }

    #[tokio::test]
    async fn codexdd_046_3c2a_cold_paused_status_never_implies_historic_ready() {
        let (mut chat, _sender, _app_events, _ops) =
            crate::chatwidget::tests::make_chatwidget_manual_with_sender().await;
        let source = tempfile::tempdir().expect("disposable source");
        let mut state = codexdd_046_3c1b2c_source_parent(source.path(), "paused source");
        state.workflow_terminal = None;
        state.paused_by_user = true;
        chat.adaptive_effort = state;
        let status = chat.adaptive_effort_status_text();
        assert!(status.contains("Next action: Hold: confirm the original source"));
        assert!(status.contains("Historic READY is not authority"));
        assert!(status.contains("source:work_packet PENDING"));
        assert!(!status.contains("Next action: Start exactly one fresh independent Validator"));
    }
}
