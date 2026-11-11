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

#[derive(Clone, Debug, PartialEq, Eq)]
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
