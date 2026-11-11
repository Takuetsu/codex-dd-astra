//! Explicit candidate-bound gate prerequisites. Native source proof cannot
//! substitute for an official full gate, live service or independent Validator.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use crate::adaptive_evidence::AdaptiveEvidenceRegistry;
use crate::adaptive_evidence::AdaptiveEvidenceResolveError;
use crate::adaptive_evidence::AdaptiveSourceGateRequirement;
use crate::adaptive_worker::AdaptiveTaskKind;
use crate::adaptive_worker::AdaptiveWorkerRole;
use codex_protocol::ThreadId;
use serde::Deserialize;
use serde::Serialize;

/// A checkpoint is *not* proof: native evidence refs are deliberately excluded.
const CODEXDD_GATE_CHECKPOINT_VERSION: u8 = 1;
const MAX_GATE_REQUIREMENTS: usize = 16;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AdaptiveRequiredGate {
    SourceProfile(String),
    OwnedRuntimeHealth(Vec<u16>),
    IndependentValidation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveGateState {
    Pending,
    Blocked(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveOutstandingGate {
    pub(crate) gate: AdaptiveRequiredGate,
    pub(crate) state: AdaptiveGateState,
}

/// Bounded, non-authoritative task prerequisites saved for process restart.
/// This is never a trusted native PASS, runtime listener receipt or terminal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AdaptiveGateCheckpoint {
    pub(crate) schema_version: u8,
    pub(crate) owner_thread_id: String,
    pub(crate) owner_role: String,
    pub(crate) authorized_scope: String,
    pub(crate) source_root: String,
    pub(crate) source_sha: String,
    pub(crate) required_gates: Vec<AdaptiveRequiredGate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) blocked_gates: Vec<(AdaptiveRequiredGate, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NativeSourceRef {
    evidence_id: String,
    thread_id: ThreadId,
    turn_id: String,
    run_id: String,
    minimum_completed_at_ms: u64,
}

/// The owner-specified prerequisites for one exact candidate. Source receipts
/// remain references and are rechecked on every readiness evaluation, so a
/// later conflict/stale source cannot retain a cached PASS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveGateLedger {
    scope: String,
    root: PathBuf,
    sha: String,
    required: Vec<AdaptiveRequiredGate>,
    source_refs: BTreeMap<String, NativeSourceRef>,
    blockers: Vec<(AdaptiveRequiredGate, String)>,
}

impl AdaptiveGateLedger {
    pub(crate) fn new(
        scope: &str,
        root: &Path,
        sha: &str,
        required: Vec<AdaptiveRequiredGate>,
    ) -> Result<Self, String> {
        if scope.trim().is_empty()
            || scope.len() > 1024
            || required.len() > MAX_GATE_REQUIREMENTS
            || sha.len() != 40
            || !sha.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err("gate ledger needs exact candidate SHA and scope".to_string());
        }
        let root = root
            .canonicalize()
            .map_err(|_| "gate ledger target root is unavailable".to_string())?;
        if required
            .iter()
            .filter(|gate| matches!(gate, AdaptiveRequiredGate::IndependentValidation))
            .count()
            != 1
        {
            return Err("independent Validation gate is mandatory for owner QA".to_string());
        }
        for (index, gate) in required.iter().enumerate() {
            if required[..index].contains(gate) {
                return Err("duplicate required gate".to_string());
            }
            match gate {
                AdaptiveRequiredGate::SourceProfile(profile)
                    if matches!(profile.as_str(), "targeted" | "work_packet" | "release") => {}
                AdaptiveRequiredGate::SourceProfile(_) => {
                    return Err("unsupported source validation profile".to_string());
                }
                AdaptiveRequiredGate::OwnedRuntimeHealth(ports)
                    if !ports.is_empty()
                        && ports.len() <= 8
                        && ports.iter().all(|port| *port != 0)
                        && ports
                            .iter()
                            .collect::<std::collections::BTreeSet<_>>()
                            .len()
                            == ports.len() => {}
                AdaptiveRequiredGate::OwnedRuntimeHealth(_) => {
                    return Err("invalid required runtime listener ports".to_string());
                }
                AdaptiveRequiredGate::IndependentValidation => {}
            }
        }
        Ok(Self {
            scope: scope.to_string(),
            root,
            sha: sha.to_ascii_lowercase(),
            required,
            source_refs: BTreeMap::new(),
            blockers: Vec::new(),
        })
    }

    /// Serialize only requirements and bounded blockers. Revalidatable native
    /// receipts deliberately remain in the in-memory registry, never a
    /// persisted green flag that could become stale after restart.
    pub(crate) fn checkpoint_for_worker(
        &self,
        owner_thread_id: ThreadId,
        owner_role: AdaptiveWorkerRole,
    ) -> Result<AdaptiveGateCheckpoint, String> {
        let owner_role = Self::bound_role_name(owner_role)?;
        Ok(AdaptiveGateCheckpoint {
            schema_version: CODEXDD_GATE_CHECKPOINT_VERSION,
            owner_thread_id: owner_thread_id.to_string(),
            owner_role: owner_role.to_string(),
            authorized_scope: self.scope.clone(),
            source_root: self.root.display().to_string(),
            source_sha: self.sha.clone(),
            required_gates: self.required.clone(),
            blocked_gates: self.blockers.clone(),
        })
    }

    /// A legacy, foreign Worker, old SHA or moved root must never reacquire
    /// authority. Even a matching checkpoint restores *pending* source gates:
    /// no source proof ref or independent gate can be replayed from JSON.
    pub(crate) fn restore_pending(
        checkpoint: AdaptiveGateCheckpoint,
        expected_thread_id: ThreadId,
        expected_role: AdaptiveWorkerRole,
        expected_scope: &str,
        expected_root: &Path,
        expected_sha: &str,
    ) -> Result<Self, String> {
        let role = Self::bound_role_name(expected_role)?;
        if checkpoint.schema_version != CODEXDD_GATE_CHECKPOINT_VERSION
            || checkpoint.owner_thread_id != expected_thread_id.to_string()
            || checkpoint.owner_role != role
            || checkpoint.authorized_scope != expected_scope
            || !checkpoint.source_sha.eq_ignore_ascii_case(expected_sha)
            || checkpoint.source_root.len() > 4096
        {
            return Err("gate checkpoint worker, scope or source identity mismatch".to_string());
        }
        let actual_root = Path::new(&checkpoint.source_root)
            .canonicalize()
            .map_err(|_| "gate checkpoint root is no longer available".to_string())?;
        let expected_root = expected_root
            .canonicalize()
            .map_err(|_| "authorized gate worktree is unavailable".to_string())?;
        if actual_root != expected_root {
            return Err("gate checkpoint belongs to a different worktree".to_string());
        }
        let mut restored = Self::new(
            expected_scope,
            &expected_root,
            expected_sha,
            checkpoint.required_gates,
        )?;
        if checkpoint.blocked_gates.len() > restored.required.len() {
            return Err("gate checkpoint has excess blockers".to_string());
        }
        let mut seen = Vec::new();
        for (gate, reason) in checkpoint.blocked_gates {
            if seen.contains(&gate) {
                return Err("gate checkpoint has duplicate blockers".to_string());
            }
            restored.mark_blocked(&gate, &reason)?;
            seen.push(gate);
        }
        Ok(restored)
    }

    fn bound_role_name(role: AdaptiveWorkerRole) -> Result<&'static str, String> {
        match role {
            AdaptiveWorkerRole::Implementation => Ok("implementation"),
            AdaptiveWorkerRole::Validation => Ok("validation"),
            AdaptiveWorkerRole::Repair => Ok("repair"),
            AdaptiveWorkerRole::Unspecified => Err("unbound Worker cannot save gate authority".to_string()),
        }
    }

    /// Only an exact native LVO receipt from a pinned Implementation Worker
    /// can mark a source gate as observed. No model text or generic exec PASS.
    pub(crate) fn observe_source_pass(
        &mut self,
        registry: &AdaptiveEvidenceRegistry,
        profile: &str,
        evidence_id: &str,
        thread_id: ThreadId,
        turn_id: &str,
        run_id: &str,
        minimum_completed_at_ms: u64,
    ) -> Result<(), AdaptiveEvidenceResolveError> {
        if !self
            .required
            .contains(&AdaptiveRequiredGate::SourceProfile(profile.to_string()))
        {
            return Err(AdaptiveEvidenceResolveError::GateScopeMismatch);
        }
        let requirement =
            self.source_requirement(profile, thread_id, turn_id, run_id, minimum_completed_at_ms);
        registry.match_native_source_gate(evidence_id, &requirement)?;
        self.source_refs.insert(
            profile.to_string(),
            NativeSourceRef {
                evidence_id: evidence_id.to_string(),
                thread_id,
                turn_id: turn_id.to_string(),
                run_id: run_id.to_string(),
                minimum_completed_at_ms,
            },
        );
        Ok(())
    }

    fn source_requirement<'a>(
        &'a self,
        profile: &'a str,
        thread_id: ThreadId,
        turn_id: &'a str,
        run_id: &'a str,
        minimum_completed_at_ms: u64,
    ) -> AdaptiveSourceGateRequirement<'a> {
        AdaptiveSourceGateRequirement {
            thread_id,
            worker_role: AdaptiveWorkerRole::Implementation,
            task_kind: AdaptiveTaskKind::SourceChange,
            authorized_scope: &self.scope,
            source_root: &self.root,
            source_sha: &self.sha,
            required_profile: profile,
            source_turn_id: Some(turn_id),
            expected_run_id: Some(run_id),
            minimum_completed_at_ms,
        }
    }

    pub(crate) fn mark_blocked(
        &mut self,
        gate: &AdaptiveRequiredGate,
        reason_code: &str,
    ) -> Result<(), String> {
        if !self.required.contains(gate) || reason_code.trim().is_empty() || reason_code.len() > 128
        {
            return Err("invalid blocked gate or reason code".to_string());
        }
        self.blockers.retain(|(recorded, _)| recorded != gate);
        self.blockers.push((gate.clone(), reason_code.to_string()));
        Ok(())
    }

    pub(crate) fn outstanding(
        &self,
        registry: &AdaptiveEvidenceRegistry,
    ) -> Vec<AdaptiveOutstandingGate> {
        self.required
            .iter()
            .filter_map(|gate| {
                let state = if let Some((_, reason)) =
                    self.blockers.iter().find(|(recorded, _)| recorded == gate)
                {
                    Some(AdaptiveGateState::Blocked(reason.clone()))
                } else {
                    match gate {
                        AdaptiveRequiredGate::SourceProfile(profile) => {
                            let source_is_current =
                                self.source_refs.get(profile).is_some_and(|proof| {
                                    let requirement = self.source_requirement(
                                        profile,
                                        proof.thread_id,
                                        &proof.turn_id,
                                        &proof.run_id,
                                        proof.minimum_completed_at_ms,
                                    );
                                    registry
                                        .match_native_source_gate(&proof.evidence_id, &requirement)
                                        .is_ok()
                                });
                            (!source_is_current).then_some(AdaptiveGateState::Pending)
                        }
                        // Until the owned-health producer (3D.1) and independent
                        // Validator gate (3C.1) exist, these never auto-pass.
                        AdaptiveRequiredGate::OwnedRuntimeHealth(_)
                        | AdaptiveRequiredGate::IndependentValidation => {
                            Some(AdaptiveGateState::Pending)
                        }
                    }
                };
                state.map(|state| AdaptiveOutstandingGate {
                    gate: gate.clone(),
                    state,
                })
            })
            .collect()
    }

    pub(crate) fn ready_for_owner_qa(&self, registry: &AdaptiveEvidenceRegistry) -> bool {
        self.outstanding(registry).is_empty()
    }
}
