//! Neutral Worker assignment and workflow handoff facts for adaptive state.

use crate::adaptive_gate_prerequisites::AdaptiveGateLedger;
use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;
use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_config::config_toml::AdaptiveWorkerRoleToml;
use codex_protocol::protocol::CodexddFirstWorkerSnapshot;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[cfg(test)]
const RAW_BRACKETED_PASTE_START: &str = "\u{1b}[200~";
#[cfg(test)]
const RAW_BRACKETED_PASTE_END: &str = "\u{1b}[201~";
#[cfg(test)]
const TEXT_BRACKETED_PASTE_START: &str = "[200~";
#[cfg(test)]
const TEXT_BRACKETED_PASTE_END: &str = "[201~";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum AdaptiveWorkerRole {
    #[default]
    Unspecified,
    Implementation,
    Validation,
    Repair,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NewWorkerBinding {
    pub(crate) role: AdaptiveWorkerRole,
    pub(crate) authorized_scope: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct AdaptiveWorkerContext {
    pub(crate) role: AdaptiveWorkerRole,
    pub(crate) authorized_scope: Option<String>,
}

/// A source pin is external first-assignment authority, never a model continuation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum AdaptiveTaskKind {
    #[default]
    SourceChange,
    OperationsPreview,
    Benchmark,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveWorkerPreflight {
    pub(crate) task_kind: AdaptiveTaskKind,
    pub(crate) source_root: String,
    pub(crate) source_sha: String,
}

pub(crate) fn validated_assignment_preflight(
    task_kind: Option<&str>,
    source_root: Option<&str>,
    source_sha: Option<&str>,
) -> Result<Option<AdaptiveWorkerPreflight>, String> {
    let task_kind = match task_kind {
        None | Some("source_change") => AdaptiveTaskKind::SourceChange,
        Some("operations_preview") => AdaptiveTaskKind::OperationsPreview,
        Some("benchmark") => AdaptiveTaskKind::Benchmark,
        Some(other) => {
            return Err(format!(
                "invalid Worker task_kind '{other}'; expected source_change, operations_preview or benchmark"
            ));
        }
    };
    match (source_root, source_sha) {
        (None, None) if task_kind == AdaptiveTaskKind::SourceChange => Ok(None),
        (Some(root), Some(sha)) => {
            let root = root.trim();
            let sha = sha.trim();
            if !Path::new(root).is_absolute() {
                return Err("Worker source_root must be an absolute authorized worktree path".to_string());
            }
            if sha.len() != 40 || !sha.bytes().all(|ch| ch.is_ascii_hexdigit()) {
                return Err("Worker source_sha must be an exact 40-character Git SHA".to_string());
            }
            Ok(Some(AdaptiveWorkerPreflight {
                task_kind,
                source_root: root.to_string(),
                source_sha: sha.to_ascii_lowercase(),
            }))
        }
        _ => Err("Worker target preflight requires source_root and source_sha together; operations/benchmark tasks must also bind a source".to_string()),
    }
}

/// Read-only TUI Git inspection with a deadline on the actual subprocess.
/// The outer async resume timeout alone cannot cancel a running blocking Git child.
/// Scratch-file stdout avoids waiting for inherited pipe handles; a bounded read
/// prevents unlimited output from an untrusted checkout. Failure never grants a pin.
fn run_bounded_tui_command(
    mut command: Command,
    step: &str,
    deadline: Duration,
) -> Result<(ExitStatus, String), String> {
    let output = tempfile::tempfile()
        .map_err(|err| format!("SETUP_BLOCKED: TUI Git {step} output unavailable: {err}"))?;
    let mut reader = output
        .try_clone()
        .map_err(|err| format!("SETUP_BLOCKED: TUI Git {step} output unreadable: {err}"))?;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::from(output))
        .stderr(Stdio::null());
    let started = Instant::now();
    let mut child = command
        .spawn()
        .map_err(|err| format!("SETUP_BLOCKED: TUI Git {step} unavailable: {err}"))?;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() >= deadline => {
                let terminated = child.kill().is_ok();
                if terminated {
                    let _ = child.wait();
                }
                return Err(format!(
                    "SETUP_BLOCKED: TUI Git {step} timed out after {}ms; process {}",
                    deadline.as_millis(),
                    if terminated {
                        "terminated"
                    } else {
                        "termination unconfirmed"
                    }
                ));
            }
            Ok(None) => thread::sleep(Duration::from_millis(25)),
            Err(err) => {
                let terminated = child.kill().is_ok();
                if terminated {
                    let _ = child.wait();
                }
                return Err(format!(
                    "SETUP_BLOCKED: TUI Git {step} could not be monitored: {err}; process {}",
                    if terminated {
                        "terminated"
                    } else {
                        "termination unconfirmed"
                    }
                ));
            }
        }
    };
    reader
        .seek(SeekFrom::Start(0))
        .map_err(|err| format!("SETUP_BLOCKED: TUI Git {step} output seek failed: {err}"))?;
    const MAX_OUTPUT: u64 = 1024 * 1024;
    let mut bytes = Vec::new();
    reader
        .take(MAX_OUTPUT + 1)
        .read_to_end(&mut bytes)
        .map_err(|err| format!("SETUP_BLOCKED: TUI Git {step} output read failed: {err}"))?;
    if bytes.len() as u64 > MAX_OUTPUT {
        return Err(format!(
            "SETUP_BLOCKED: TUI Git {step} exceeded bounded output size"
        ));
    }
    let output = String::from_utf8(bytes)
        .map_err(|_| format!("SETUP_BLOCKED: TUI Git {step} returned non-UTF8 output"))?;
    Ok((status, output))
}

fn run_tui_git_step(
    root: &Path,
    step: &'static str,
    args: &[&str],
) -> Result<(ExitStatus, String), String> {
    let mut command = Command::new("git");
    command
        .arg("--no-optional-locks")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GCM_INTERACTIVE", "Never");
    let deadline = if step == "status" {
        Duration::from_secs(12)
    } else {
        Duration::from_secs(5)
    };
    run_bounded_tui_command(command, step, deadline)
}

/// This local read-only pre-admission check never chooses another worktree.
/// Child-shell Set-Location does not change the native CodexDD runtime cwd.
pub(crate) fn preflight_mechanical_target(
    binding: Option<&AdaptiveWorkerPreflight>,
    runtime_cwd: &Path,
) -> Result<(), String> {
    let Some(binding) = binding else {
        // Legacy unpinned workers remain unchanged pending 3B.2 migration.
        return Ok(());
    };
    if binding.task_kind != AdaptiveTaskKind::SourceChange {
        return Err(
            "SETUP_BLOCKED: explicitly bound operations/benchmark task cannot enter protected source MechanicalValidation. Use an approved task-specific handoff, not a source work_packet."
                .to_string(),
        );
    }
    preflight_owner_candidate_identity(binding, runtime_cwd)
}

/// Bounded Git inspection of the selected, explicitly authorized candidate,
/// including non-source tasks. This does not grant operations permissions or
/// convert a source-only test into a live-runtime health receipt.
fn preflight_owner_candidate_identity(
    binding: &AdaptiveWorkerPreflight,
    runtime_cwd: &Path,
) -> Result<(), String> {
    let observed_root = runtime_cwd.ancestors().find(|candidate| {
        let marker = candidate.join(".git");
        marker.is_file() || marker.is_dir()
    }).ok_or_else(|| format!(
        "SETUP_BLOCKED: native cwd {} has no Git root; expected authorized {} at SHA {}. No validation ran.",
        runtime_cwd.display(), binding.source_root, binding.source_sha
    ))?;
    let expected_root = Path::new(&binding.source_root).canonicalize().map_err(|err| {
        format!("SETUP_BLOCKED: cannot resolve authorized worktree {} ({err}); preserve the candidate.", binding.source_root)
    })?;
    let actual_root = observed_root.canonicalize().map_err(|err| {
        format!(
            "SETUP_BLOCKED: cannot resolve native selected worktree {} ({err}); no validation ran.",
            observed_root.display()
        )
    })?;
    if actual_root != expected_root {
        return Err(format!(
            "SETUP_BLOCKED: native Git root {} differs from authorized {} at SHA {}. Keep the owner's milestone-root launch convention; bind the approved worktree before validation. No validation ran.",
            actual_root.display(),
            expected_root.display(),
            binding.source_sha
        ));
    }
    let git = |step: &'static str, args: &[&str]| -> Result<String, String> {
        let (status, output) = run_tui_git_step(&actual_root, step, args)?;
        if !status.success() {
            return Err(format!(
                "SETUP_BLOCKED: Git read-only preflight failed in {} (status {}). No validation ran.",
                actual_root.display(),
                status
            ));
        }
        Ok(output)
    };
    let git_root = git("root", &["rev-parse", "--show-toplevel"])?;
    let verified_git_root = Path::new(git_root.trim()).canonicalize().map_err(|err| {
        format!("SETUP_BLOCKED: Git-selected root cannot be resolved ({err}); no validation ran.")
    })?;
    if verified_git_root != actual_root {
        return Err(format!(
            "SETUP_BLOCKED: Git reports root {} but the native root is {}; no validation ran.",
            verified_git_root.display(),
            actual_root.display()
        ));
    }
    let current_sha = git("head", &["rev-parse", "HEAD"])?;
    if !current_sha.trim().eq_ignore_ascii_case(&binding.source_sha) {
        return Err(format!(
            "SETUP_BLOCKED: selected worktree {} has SHA {} but authorized candidate is {}; no validation ran.",
            actual_root.display(),
            current_sha.trim(),
            binding.source_sha
        ));
    }
    if !git(
        "status",
        &["status", "--porcelain=v1", "--untracked-files=all"],
    )?
    .trim()
    .is_empty()
    {
        return Err(format!(
            "SETUP_BLOCKED: authorized source {} at {} has staged/unstaged/untracked changes. Preserve all edits; no exact-SHA validation receipt can be claimed.",
            actual_root.display(),
            binding.source_sha
        ));
    }
    Ok(())
}

/// Reconcile an *already persisted* Core Worker source credential delivered by
/// the local embedded app-server. Caller must reject remote transports and
/// verify the notification belongs to the currently selected thread.
pub(crate) fn reconcile_core_candidate_preflight(
    current: Option<&AdaptiveWorkerPreflight>,
    worker: &AdaptiveWorkerContext,
    assignment_locked: bool,
    core: &CodexddFirstWorkerSnapshot,
    runtime_cwd: &Path,
) -> Result<Option<AdaptiveWorkerPreflight>, String> {
    let Some(current) = current else {
        // A notification never creates an initial Worker assignment.
        return Ok(None);
    };
    if !assignment_locked
        || worker.role != AdaptiveWorkerRole::Implementation
        || core.role != "implementation"
        || worker.authorized_scope.as_deref() != Some(core.authorized_scope.as_str())
        || current.task_kind != AdaptiveTaskKind::SourceChange
        || !matches!(core.task_kind.as_deref(), None | Some("source_change"))
        || core.source_root.as_deref() != Some(current.source_root.as_str())
    {
        return Err("SETUP_BLOCKED: Core candidate notification cannot change Worker role, task, root, or scope".to_string());
    }
    let Some(candidate_sha) = core.source_sha.as_deref() else {
        return Err("SETUP_BLOCKED: Core candidate notice lacks exact source SHA".to_string());
    };
    if candidate_sha.len() != 40 || !candidate_sha.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("SETUP_BLOCKED: Core candidate notice has an invalid source SHA".to_string());
    }
    if candidate_sha.eq_ignore_ascii_case(&current.source_sha) {
        return Ok(None);
    }
    let candidate = AdaptiveWorkerPreflight {
        task_kind: current.task_kind,
        source_root: current.source_root.clone(),
        source_sha: candidate_sha.to_ascii_lowercase(),
    };
    // No TUI source promotion from a stale, dirty, or wrong-root checkout.
    preflight_mechanical_target(Some(&candidate), runtime_cwd)?;
    let (descendant, _) = run_tui_git_step(
        Path::new(&candidate.source_root),
        "ancestry",
        &[
            "merge-base",
            "--is-ancestor",
            &current.source_sha,
            &candidate.source_sha,
        ],
    )?;
    if !descendant.success() {
        return Err(
            "SETUP_BLOCKED: Core candidate is not a verified descendant of the TUI source pin"
                .to_string(),
        );
    }
    Ok(Some(candidate))
}

/// Build exactly the requirements supplied in the original owner assignment.
/// A missing manifest leaves READY blocked: there is no safe implicit gate
/// list based on model prose, incidental tests or a later Worker continuation.
fn bound_owner_gate_ledger(
    binding: &AdaptiveWorkerConfigToml,
    context: &AdaptiveWorkerContext,
    pin: Option<&AdaptiveWorkerPreflight>,
) -> Result<Option<AdaptiveGateLedger>, String> {
    let Some(definitions) = binding.required_gates.as_ref() else {
        return Ok(None);
    };
    if context.role != AdaptiveWorkerRole::Implementation {
        return Err("SETUP_BLOCKED: only an owner-bound Implementation Worker may issue candidate gate requirements".to_string());
    }
    let scope = context
        .authorized_scope
        .as_deref()
        .filter(|scope| !scope.trim().is_empty() && scope.len() <= 1024)
        .ok_or_else(|| "SETUP_BLOCKED: gate requirements need a bounded owner-authorized scope".to_string())?;
    let pin = pin.ok_or_else(|| {
        "SETUP_BLOCKED: owner gate requirements require both an exact source root and SHA".to_string()
    })?;
    if pin.source_root.len() > 4096 || definitions.is_empty() || definitions.len() > 16 {
        return Err("SETUP_BLOCKED: invalid owner gate manifest size or source path".to_string());
    }
    let mut gates = Vec::with_capacity(definitions.len());
    for definition in definitions {
        let gate = match definition.kind.as_str() {
            "source_profile" if definition.ports.is_none() => {
                let profile = definition.profile.as_deref().ok_or_else(|| {
                    "SETUP_BLOCKED: source_profile gate must explicitly name its native profile".to_string()
                })?;
                AdaptiveRequiredGate::SourceProfile(profile.to_string())
            }
            "owned_runtime_health" if definition.profile.is_none() => {
                let ports = definition.ports.clone().ok_or_else(|| {
                    "SETUP_BLOCKED: owned_runtime_health requires explicit approved ports".to_string()
                })?;
                AdaptiveRequiredGate::OwnedRuntimeHealth(ports)
            }
            "independent_validation"
                if definition.profile.is_none() && definition.ports.is_none() =>
            {
                AdaptiveRequiredGate::IndependentValidation
            }
            _ => {
                return Err("SETUP_BLOCKED: unknown or ambiguous owner gate kind/fields".to_string());
            }
        };
        gates.push(gate);
    }
    match pin.task_kind {
        AdaptiveTaskKind::SourceChange => {
            if !gates.iter().any(|gate| matches!(
                gate,
                AdaptiveRequiredGate::SourceProfile(profile)
                    if profile == "work_packet" || profile == "release"
            )) {
                return Err("SETUP_BLOCKED: source-change QA requires an explicitly authorized work_packet or release gate; targeted alone is insufficient".to_string());
            }
        }
        AdaptiveTaskKind::OperationsPreview => {
            if gates.iter().any(|gate| matches!(gate, AdaptiveRequiredGate::SourceProfile(_)))
                || !gates.iter().any(|gate| matches!(gate, AdaptiveRequiredGate::OwnedRuntimeHealth(_)))
            {
                return Err("SETUP_BLOCKED: operations-preview requires an owned-runtime-health gate, not source-only PASS".to_string());
            }
        }
        AdaptiveTaskKind::Benchmark => {
            return Err("SETUP_BLOCKED: benchmark owner gates await a native benchmark-evidence contract; source or runtime PASS cannot substitute".to_string());
        }
    }
    let ledger = AdaptiveGateLedger::new(scope, Path::new(&pin.source_root), &pin.source_sha, gates)
        .map_err(|error| format!("SETUP_BLOCKED: invalid owner gate manifest: {error}"))?;
    // Validate the actual selected Git root, clean HEAD and SHA *before*
    // granting even pending source/operations gate requirements.
    preflight_owner_candidate_identity(pin, Path::new(&pin.source_root))?;
    Ok(Some(ledger))
}

/// Bind both the immutable startup Worker identity and its independently
/// requested pending gate ledger before the first app-server thread starts.
pub(crate) fn configured_worker_authority(
    binding: &AdaptiveWorkerConfigToml,
) -> Result<
    (
        AdaptiveWorkerContext,
        Option<AdaptiveWorkerPreflight>,
        Option<AdaptiveGateLedger>,
    ),
    String,
> {
    let context = AdaptiveWorkerContext::from(binding.clone());
    let preflight = validated_assignment_preflight(
        binding.task_kind.as_deref(),
        binding.source_root.as_deref(),
        binding.source_sha.as_deref(),
    )?;
    if preflight.is_some()
        && (context.role == AdaptiveWorkerRole::Unspecified
            || context
                .authorized_scope
                .as_deref()
                .is_none_or(|scope| scope.trim().is_empty()))
    {
        return Err(
            "SETUP_BLOCKED: pinned startup Worker requires a bound role and nonempty authorized_scope"
                .to_string(),
        );
    }
    let ledger = bound_owner_gate_ledger(binding, &context, preflight.as_ref())?;
    Ok((context, preflight, ledger))
}

/// Validate immutable startup-config Worker binding before starting a thread.
/// A configured scope cannot be replaced by a later first-message header.
pub(crate) fn configured_worker_binding(
    binding: &AdaptiveWorkerConfigToml,
) -> Result<(AdaptiveWorkerContext, Option<AdaptiveWorkerPreflight>), String> {
    configured_worker_authority(binding).map(|(context, preflight, _)| (context, preflight))
}

impl From<AdaptiveWorkerConfigToml> for AdaptiveWorkerContext {
    fn from(binding: AdaptiveWorkerConfigToml) -> Self {
        let role = match binding.role {
            AdaptiveWorkerRoleToml::Unspecified => AdaptiveWorkerRole::Unspecified,
            AdaptiveWorkerRoleToml::Implementation => AdaptiveWorkerRole::Implementation,
            AdaptiveWorkerRoleToml::Validation => AdaptiveWorkerRole::Validation,
            AdaptiveWorkerRoleToml::Repair => AdaptiveWorkerRole::Repair,
        };
        Self {
            role,
            authorized_scope: binding.authorized_scope,
        }
    }
}

pub(crate) fn parse_new_worker_role(value: &str) -> Option<AdaptiveWorkerRole> {
    match value {
        "implementation" => Some(AdaptiveWorkerRole::Implementation),
        "validation" => Some(AdaptiveWorkerRole::Validation),
        "repair" => Some(AdaptiveWorkerRole::Repair),
        _ => None,
    }
}

/// Parses the authority header on the first user-supplied Worker assignment.
///
/// Authority is recognized only when the first non-blank line is exactly the
/// `[adaptive_worker]` table after removing a single recognized bracketed-paste
/// framing marker from the message boundary. The table must end at a blank line
/// so ordinary assignment prose can never be interpreted as TOML authority. A
/// message without the header returns `Ok(None)`; a malformed authority header
/// fails closed with an error.
pub(crate) fn parse_adaptive_worker_assignment(
    text: &str,
) -> Result<Option<AdaptiveWorkerContext>, String> {
    parse_adaptive_worker_assignment_with_preflight(text)
        .map(|assignment| assignment.map(|(context, _)| context))
}

pub(crate) fn parse_adaptive_worker_assignment_with_preflight(
    text: &str,
) -> Result<Option<(AdaptiveWorkerContext, Option<AdaptiveWorkerPreflight>)>, String> {
    parse_adaptive_owner_assignment(text).map(|assignment| {
        assignment.map(|(context, preflight, _)| (context, preflight))
    })
}

pub(crate) fn parse_adaptive_owner_assignment(
    text: &str,
) -> Result<
    Option<(
        AdaptiveWorkerContext,
        Option<AdaptiveWorkerPreflight>,
        Option<AdaptiveGateLedger>,
    )>,
    String,
> {
    let Some(binding) = codex_config::config_toml::parse_adaptive_worker_first_message(text)?
    else {
        return Ok(None);
    };
    configured_worker_authority(&binding).map(Some)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveWorkflowTerminal {
    ReadyForValidation,
    RepairRequired,
    ReadyForOwnerQa,
    Blocked,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validation_assignment() -> String {
        "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"Z-A0.45B-R2 exact bounded assignment\"\n\nPerform the assigned work."
            .to_string()
    }

    fn expected_validation_context() -> AdaptiveWorkerContext {
        AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: Some("Z-A0.45B-R2 exact bounded assignment".to_string()),
        }
    }

    fn codexdd_046_3c1b2e_git_fixture() -> (tempfile::TempDir, String) {
        let root = tempfile::tempdir().expect("isolated owner candidate");
        let git = |args: &[&str]| -> String {
            let result = Command::new("git")
                .arg("-C")
                .arg(root.path())
                .args(args)
                .output()
                .expect("Git installed on native Windows test runner");
            assert!(
                result.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            String::from_utf8(result.stdout).expect("UTF-8 Git output")
        };
        git(&["init", "-q"]);
        git(&[
            "-c",
            "user.name=CodexDD 0.4.6 Fixture",
            "-c",
            "user.email=codexdd@example.test",
            "commit",
            "--allow-empty",
            "-q",
            "-m",
            "owner-approved candidate",
        ]);
        let sha = git(&["rev-parse", "HEAD"]).trim().to_string();
        (root, sha)
    }

    fn codexdd_046_3c1b2e_owner_message(
        root: &Path,
        sha: &str,
        role: &str,
        kind: &str,
        gates: &str,
    ) -> String {
        format!(
            "[adaptive_worker]\nrole = \"{role}\"\nauthorized_scope = \"owner-approved isolated task\"\nsource_root = '{}'\nsource_sha = \"{sha}\"\ntask_kind = \"{kind}\"\nrequired_gates = {gates}\n\nExecute only the owner-approved assignment.",
            root.display()
        )
    }

    #[test]
    fn codexdd_046_3c1b2e_exact_first_message_creates_pending_owner_gates() {
        use crate::adaptive_evidence::AdaptiveEvidenceRegistry;
        use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;

        let (source, sha) = codexdd_046_3c1b2e_git_fixture();
        let gates = r#"[{ kind = "source_profile", profile = "work_packet" }, { kind = "independent_validation" }]"#;
        let message = codexdd_046_3c1b2e_owner_message(
            source.path(),
            &sha,
            "implementation",
            "source_change",
            gates,
        );
        let (context, pin, ledger) = parse_adaptive_owner_assignment(&message)
            .expect("first owner assignment valid")
            .expect("owner authority present");
        assert_eq!(context.role, AdaptiveWorkerRole::Implementation);
        let pin = pin.expect("exact source pin");
        let ledger = ledger.expect("owner-issued requirements");
        assert!(ledger.matches_authorized_source(
            "owner-approved isolated task",
            Path::new(&pin.source_root),
            &pin.source_sha,
        ));
        let registry = AdaptiveEvidenceRegistry::default();
        let outstanding = ledger.outstanding(&registry);
        assert_eq!(outstanding.len(), 2);
        assert!(outstanding.iter().any(|pending| {
            pending.gate == AdaptiveRequiredGate::SourceProfile("work_packet".to_string())
        }));
        assert!(outstanding.iter().any(|pending| {
            pending.gate == AdaptiveRequiredGate::IndependentValidation
        }));
        assert!(!ledger.ready_for_owner_qa(&registry));

        let binding = codex_config::config_toml::parse_adaptive_worker_first_message(&message)
            .expect("typed owner TOML")
            .expect("owner binding");
        let (_, _, startup_ledger) =
            configured_worker_authority(&binding).expect("same gates on startup config");
        assert_eq!(startup_ledger.expect("startup requirements"), ledger);
        let legacy = message.replace(&format!("required_gates = {gates}\n"), "");
        let (_, _, no_inference) = parse_adaptive_owner_assignment(&legacy)
            .expect("legacy assignment may still start")
            .expect("legacy Worker bound");
        assert!(no_inference.is_none(), "no manifest means no owner QA gates or READY");
    }

    #[test]
    fn codexdd_046_3c1b2e_rejects_missing_weakened_or_forged_owner_gates() {
        let (source, sha) = codexdd_046_3c1b2e_git_fixture();
        let bad_manifests = [
            "[]",
            r#"[{ kind = "independent_validation" }]"#,
            r#"[{ kind = "source_profile", profile = "targeted" }, { kind = "independent_validation" }]"#,
            r#"[{ kind = "source_profile", profile = "work_packet" }]"#,
            r#"[{ kind = "source_profile", profile = "release" }, { kind = "independent_validation" }, { kind = "independent_validation" }]"#,
            r#"[{ kind = "source_profile", profile = "arbitrary_shell" }, { kind = "independent_validation" }]"#,
            r#"[{ kind = "source_profile", profile = "work_packet", ports = [8080] }, { kind = "independent_validation" }]"#,
            r#"[{ kind = "source_profile", profile = "work_packet" }, { kind = "independent_validation", profile = "release" }]"#,
            r#"[{ kind = "unknown" }, { kind = "independent_validation" }]"#,
            r#"[{ kind = "source_profile", profile = "work_packet", extra = true }, { kind = "independent_validation" }]"#,
        ];
        for gates in bad_manifests {
            let message = codexdd_046_3c1b2e_owner_message(
                source.path(), &sha, "implementation", "source_change", gates,
            );
            assert!(
                parse_adaptive_owner_assignment(&message).is_err(),
                "must refuse incomplete/ambiguous owner manifest: {gates}"
            );
        }
        let valid = r#"[{ kind = "source_profile", profile = "work_packet" }, { kind = "independent_validation" }]"#;
        for role in ["validation", "repair"] {
            let message = codexdd_046_3c1b2e_owner_message(
                source.path(), &sha, role, "source_change", valid,
            );
            assert!(
                parse_adaptive_owner_assignment(&message)
                    .expect_err("only Implementation owner can issue gate authority")
                    .contains("Implementation"),
            );
        }
        let unpinned = format!(
            "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"owner-approved isolated task\"\nrequired_gates = {valid}\n\nNo source pin."
        );
        assert!(parse_adaptive_owner_assignment(&unpinned).is_err());
    }

    #[test]
    fn codexdd_046_3c1b2e_operations_health_remains_pending_and_benchmark_is_blocked() {
        use crate::adaptive_evidence::AdaptiveEvidenceRegistry;
        use crate::adaptive_gate_prerequisites::AdaptiveGateState;
        use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;

        let (source, sha) = codexdd_046_3c1b2e_git_fixture();
        let gates = r#"[{ kind = "owned_runtime_health", ports = [4879, 4880] }, { kind = "independent_validation" }]"#;
        let message = codexdd_046_3c1b2e_owner_message(
            source.path(), &sha, "implementation", "operations_preview", gates,
        );
        let (_, pin, ledger) = parse_adaptive_owner_assignment(&message)
            .expect("bounded operations requirements")
            .expect("owner authority");
        let pin = pin.expect("explicit operations source");
        let ledger = ledger.expect("runtime gate must be present");
        let outstanding = ledger.outstanding(&AdaptiveEvidenceRegistry::default());
        assert_eq!(outstanding.len(), 2);
        assert!(outstanding.iter().any(|entry| {
            entry.gate == AdaptiveRequiredGate::OwnedRuntimeHealth(vec![4879, 4880])
                && entry.state == AdaptiveGateState::Pending
        }));
        assert!(!ledger.ready_for_owner_qa(&AdaptiveEvidenceRegistry::default()));
        assert!(
            preflight_mechanical_target(Some(&pin), source.path())
                .expect_err("operations must not enter source MechanicalValidation")
                .contains("operations/benchmark")
        );
        let benchmark = codexdd_046_3c1b2e_owner_message(
            source.path(), &sha, "implementation", "benchmark", gates,
        );
        assert!(
            parse_adaptive_owner_assignment(&benchmark)
                .expect_err("benchmark lacks independent native evidence lane")
                .contains("benchmark owner gates")
        );
        let source_only = r#"[{ kind = "source_profile", profile = "work_packet" }, { kind = "independent_validation" }]"#;
        let bad_operation = codexdd_046_3c1b2e_owner_message(
            source.path(), &sha, "implementation", "operations_preview", source_only,
        );
        assert!(parse_adaptive_owner_assignment(&bad_operation).is_err());
    }

    #[test]
    fn codexdd_046_3c1b2e_rejects_dirty_foreign_and_wrong_sha_before_issuance() {
        use std::fs;
        let (source, sha) = codexdd_046_3c1b2e_git_fixture();
        let gates = r#"[{ kind = "source_profile", profile = "work_packet" }, { kind = "independent_validation" }]"#;
        let wrong_sha = "f".repeat(40);
        let wrong = codexdd_046_3c1b2e_owner_message(
            source.path(), &wrong_sha, "implementation", "source_change", gates,
        );
        assert!(
            parse_adaptive_owner_assignment(&wrong)
                .expect_err("wrong HEAD must be setup blocked")
                .contains("authorized candidate")
        );

        let nested = source.path().join("not-an-independent-worktree");
        fs::create_dir(&nested).expect("nested source directory");
        let foreign = codexdd_046_3c1b2e_owner_message(
            &nested, &sha, "implementation", "source_change", gates,
        );
        assert!(
            parse_adaptive_owner_assignment(&foreign)
                .expect_err("do not use ancestor Git root")
                .contains("native Git root")
        );

        let dirty_file = source.path().join("preserve-this-untracked-file.txt");
        fs::write(&dirty_file, "operator source edits").expect("untracked edit");
        let dirty = codexdd_046_3c1b2e_owner_message(
            source.path(), &sha, "implementation", "source_change", gates,
        );
        assert!(
            parse_adaptive_owner_assignment(&dirty)
                .expect_err("dirty source must not get a gate ledger")
                .contains("staged/unstaged/untracked")
        );
        assert_eq!(fs::read_to_string(dirty_file).expect("edits retained"), "operator source edits");
    }

    #[test]
    fn new_worker_role_parser_accepts_only_role_values() {
        assert_eq!(
            parse_new_worker_role("implementation"),
            Some(AdaptiveWorkerRole::Implementation)
        );
        assert_eq!(
            parse_new_worker_role("validation"),
            Some(AdaptiveWorkerRole::Validation)
        );
        assert_eq!(
            parse_new_worker_role("repair"),
            Some(AdaptiveWorkerRole::Repair)
        );
        assert_eq!(parse_new_worker_role("nonsense"), None);
    }

    #[test]
    fn first_assignment_header_binds_each_authorized_worker_role() {
        for (role, expected) in [
            ("implementation", AdaptiveWorkerRole::Implementation),
            ("validation", AdaptiveWorkerRole::Validation),
            ("repair", AdaptiveWorkerRole::Repair),
        ] {
            let input = format!(
                "[adaptive_worker]\nrole = \"{role}\"\nauthorized_scope = \"Z-A0.45B-R2 exact bounded assignment\"\n\nPerform the assigned work."
            );
            assert_eq!(
                parse_adaptive_worker_assignment(&input),
                Ok(Some(AdaptiveWorkerContext {
                    role: expected,
                    authorized_scope: Some("Z-A0.45B-R2 exact bounded assignment".to_string()),
                }))
            );
        }
    }

    #[test]
    fn bracketed_paste_framing_binds_first_assignment() {
        let assignment = validation_assignment();
        let cases = [
            format!("{RAW_BRACKETED_PASTE_START}{assignment}"),
            format!("{assignment}{RAW_BRACKETED_PASTE_END}"),
            format!("{RAW_BRACKETED_PASTE_START}{assignment}{RAW_BRACKETED_PASTE_END}"),
            format!("{TEXT_BRACKETED_PASTE_START}{assignment}"),
            format!("{assignment}{TEXT_BRACKETED_PASTE_END}"),
            format!("{TEXT_BRACKETED_PASTE_START}{assignment}{TEXT_BRACKETED_PASTE_END}"),
        ];

        for input in cases {
            assert_eq!(
                parse_adaptive_worker_assignment(&input),
                Ok(Some(expected_validation_context())),
                "{input:?}"
            );
        }
    }

    #[test]
    fn bracketed_paste_start_after_leading_blank_lines_binds() {
        let input = format!(
            "\n  \n{TEXT_BRACKETED_PASTE_START}{}",
            validation_assignment()
        );
        assert_eq!(
            parse_adaptive_worker_assignment(&input),
            Ok(Some(expected_validation_context()))
        );
    }

    #[test]
    fn bracketed_paste_marker_without_exact_header_reports_binding_error() {
        let err = parse_adaptive_worker_assignment(
            "[200~not-the-header\n[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\nReview it.",
        )
        .expect_err("recognized paste framing without the exact authority header should fail closed");

        assert!(err.contains("bracketed-paste marker detected before [adaptive_worker]"));
    }

    #[test]
    fn unrelated_malformed_prefix_still_has_no_worker_authority() {
        for input in [
            "[199~[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\nReview it.",
            "[200x~[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\nReview it.",
            "prefix[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\nReview it.",
        ] {
            assert_eq!(parse_adaptive_worker_assignment(input), Ok(None), "{input}");
        }
    }

    #[test]
    fn trailing_paste_marker_cannot_substitute_for_assignment_body() {
        for marker in [RAW_BRACKETED_PASTE_END, TEXT_BRACKETED_PASTE_END] {
            let input = format!(
                "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\n{marker}"
            );
            assert_eq!(
                parse_adaptive_worker_assignment(&input),
                Err("Worker assignment body must not be empty".to_string())
            );
        }
    }

    #[test]
    fn ordinary_prompt_has_no_worker_authority() {
        assert_eq!(
            parse_adaptive_worker_assignment("Review Z-A0.45B-R2 independently."),
            Ok(None)
        );
    }

    #[test]
    fn malformed_authority_header_fails_closed() {
        for input in [
            "[adaptive_worker]\nrole = \"reviewer\"\nauthorized_scope = \"R2\"\n\nReview it.",
            "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"   \"\n\nReview it.",
            "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\nextra = true\n\nReview it.",
            "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"",
            "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\n   ",
        ] {
            assert!(parse_adaptive_worker_assignment(input).is_err(), "{input}");
        }
    }

    #[test]
    fn startup_worker_authority_is_checked_before_any_thread_starts() {
        let pin = if cfg!(windows) {
            r"C:\approved\candidate"
        } else {
            "/tmp/approved/candidate"
        };
        let config = AdaptiveWorkerConfigToml {
            role: AdaptiveWorkerRoleToml::Implementation,
            authorized_scope: Some("approved isolated candidate".to_string()),
            source_root: Some(pin.to_string()),
            source_sha: Some("A".repeat(40)),
            task_kind: Some("benchmark".to_string()),
            required_gates: None,
        };
        let (context, preflight) = configured_worker_binding(&config).expect("valid startup pin");
        assert_eq!(context.role, AdaptiveWorkerRole::Implementation);
        let pinned = preflight.expect("pinned");
        assert_eq!(pinned.source_root, pin);
        assert_eq!(pinned.source_sha, "a".repeat(40));
        assert_eq!(pinned.task_kind, AdaptiveTaskKind::Benchmark);
        for role in [
            AdaptiveWorkerRoleToml::Unspecified,
            AdaptiveWorkerRoleToml::Validation,
        ] {
            let mut blocked = config.clone();
            blocked.role = role;
            if role == AdaptiveWorkerRoleToml::Validation {
                blocked.source_sha = None;
            }
            assert!(
                configured_worker_binding(&blocked).is_err(),
                "invalid startup source authority must not start a thread"
            );
        }
        let mut unscoped = config.clone();
        unscoped.authorized_scope = None;
        assert!(configured_worker_binding(&unscoped).is_err());
    }

    #[test]
    fn pinned_authority_classifies_benchmark_and_rejects_partial_binding() {
        let root = if cfg!(windows) {
            r"D:\bw58\.worktrees\candidate"
        } else {
            "/tmp/bw58/.worktrees/candidate"
        };
        let sha = "a".repeat(40);
        let input = format!(
            "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"benchmark case\"\nsource_root = '{root}'\nsource_sha = \"{sha}\"\ntask_kind = \"benchmark\"\n\nMeasure safely."
        );
        let (context, binding) = parse_adaptive_worker_assignment_with_preflight(&input)
            .expect("parsed")
            .expect("bound");
        assert_eq!(context.role, AdaptiveWorkerRole::Implementation);
        let binding = binding.expect("pin");
        assert_eq!(binding.source_root, root);
        assert_eq!(binding.source_sha, sha);
        assert_eq!(binding.task_kind, AdaptiveTaskKind::Benchmark);
        assert!(
            preflight_mechanical_target(Some(&binding), Path::new(root))
                .expect_err("not source work")
                .contains("SETUP_BLOCKED")
        );
        for (kind, root, sha) in [
            (Some("benchmark"), None, None),
            (None, Some(root), None),
            (None, None, Some("a".repeat(40))),
            (None, Some("relative"), Some("a".repeat(40))),
            (None, Some(root), Some("invalid".to_string())),
            (Some("elevated"), Some(root), Some("a".repeat(40))),
        ] {
            assert!(validated_assignment_preflight(kind, root, sha.as_deref()).is_err());
        }
    }

    #[test]
    fn pinned_worktree_identity_rejects_wrong_root_sha_and_dirty_edits() {
        use std::fs;
        let temp = tempfile::tempdir().expect("tempdir");
        let milestone = temp.path().join("milestone");
        fs::create_dir(&milestone).expect("milestone");
        let git = |root: &Path, args: &[&str]| -> String {
            let output = Command::new("git")
                .arg("-C")
                .arg(root)
                .args(args)
                .output()
                .expect("git installed");
            assert!(
                output.status.success(),
                "git {args:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            String::from_utf8(output.stdout).expect("utf8")
        };
        git(&milestone, &["init"]);
        git(
            &milestone,
            &[
                "-c",
                "user.name=CodexDD Test",
                "-c",
                "user.email=codexdd@example.test",
                "commit",
                "--allow-empty",
                "-m",
                "baseline",
            ],
        );
        let nested = milestone.join(".worktrees").join("candidate");
        git(
            &milestone,
            &[
                "worktree",
                "add",
                "--detach",
                nested.to_str().expect("utf8"),
                "HEAD",
            ],
        );
        let binding = AdaptiveWorkerPreflight {
            task_kind: AdaptiveTaskKind::SourceChange,
            source_root: nested.display().to_string(),
            source_sha: git(&nested, &["rev-parse", "HEAD"]).trim().to_string(),
        };
        assert!(
            preflight_mechanical_target(Some(&binding), &milestone)
                .expect_err("wrong selected root")
                .contains("differs from authorized")
        );
        assert_eq!(preflight_mechanical_target(Some(&binding), &nested), Ok(()));
        let mut wrong_sha = binding.clone();
        wrong_sha.source_sha = "f".repeat(40);
        assert!(
            preflight_mechanical_target(Some(&wrong_sha), &nested)
                .expect_err("wrong SHA")
                .contains("authorized candidate")
        );
        fs::write(nested.join("candidate.txt"), "preserve me").expect("write");
        git(&nested, &["add", "candidate.txt"]);
        assert!(
            preflight_mechanical_target(Some(&binding), &nested)
                .expect_err("staged candidate")
                .contains("Preserve all edits")
        );
        assert_eq!(
            fs::read_to_string(nested.join("candidate.txt")).expect("read"),
            "preserve me"
        );
    }

    #[test]
    fn codexdd_046_3b2k_durable_core_candidate_updates_only_source_sha() {
        let tmp = tempfile::tempdir().expect("isolated Git fixture");
        let root = tmp.path();
        let git = |args: &[&str]| -> String {
            let output = Command::new("git")
                .arg("-C")
                .arg(root)
                .args(args)
                .output()
                .expect("Git fixture");
            assert!(
                output.status.success(),
                "Git failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            String::from_utf8(output.stdout)
                .expect("Git output")
                .trim()
                .to_string()
        };
        git(&["init"]);
        git(&["config", "user.email", "codexdd@example.invalid"]);
        git(&["config", "user.name", "CodexDD test"]);
        std::fs::write(root.join("source.txt"), "first").expect("write");
        git(&["add", "source.txt"]);
        git(&["commit", "-m", "first"]);
        let original_sha = git(&["rev-parse", "HEAD"]);
        std::fs::write(root.join("source.txt"), "candidate").expect("write");
        git(&["add", "source.txt"]);
        git(&["commit", "-m", "candidate"]);
        let candidate_sha = git(&["rev-parse", "HEAD"]);
        let current = AdaptiveWorkerPreflight {
            task_kind: AdaptiveTaskKind::SourceChange,
            source_root: root.display().to_string(),
            source_sha: original_sha.clone(),
        };
        let worker = AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Implementation,
            authorized_scope: Some("source review".to_string()),
        };
        let notice = CodexddFirstWorkerSnapshot {
            role: "implementation".to_string(),
            authorized_scope: "source review".to_string(),
            source_root: Some(root.display().to_string()),
            source_sha: Some(candidate_sha.clone()),
            task_kind: Some("source_change".to_string()),
        };
        let proposed =
            reconcile_core_candidate_preflight(Some(&current), &worker, true, &notice, root)
                .expect("matching durable candidate")
                .expect("new pin");
        assert_eq!(proposed.source_sha, candidate_sha);
        assert_eq!(current.source_sha, original_sha);
        assert_eq!(proposed.source_root, current.source_root);
        assert_eq!(
            reconcile_core_candidate_preflight(Some(&proposed), &worker, true, &notice, root),
            Ok(None)
        );
        assert_eq!(
            reconcile_core_candidate_preflight(None, &worker, true, &notice, root),
            Ok(None)
        );
    }

    #[test]
    fn codexdd_046_3b2k_core_candidate_refuses_role_scope_and_worktree_drift() {
        let root = if cfg!(windows) {
            r"C:\missing\codexdd-worktree"
        } else {
            "/nonexistent/codexdd-worktree"
        };
        let current = AdaptiveWorkerPreflight {
            task_kind: AdaptiveTaskKind::SourceChange,
            source_root: root.to_string(),
            source_sha: "a".repeat(40),
        };
        let worker = AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Implementation,
            authorized_scope: Some("approved packet".to_string()),
        };
        let notice = CodexddFirstWorkerSnapshot {
            role: "implementation".to_string(),
            authorized_scope: "approved packet".to_string(),
            source_root: Some(root.to_string()),
            source_sha: Some("b".repeat(40)),
            task_kind: Some("source_change".to_string()),
        };
        let mut invalid = notice.clone();
        invalid.authorized_scope = "other work".to_string();
        assert!(
            reconcile_core_candidate_preflight(
                Some(&current),
                &worker,
                true,
                &invalid,
                Path::new(root)
            )
            .is_err()
        );
        invalid = notice.clone();
        invalid.role = "repair".to_string();
        assert!(
            reconcile_core_candidate_preflight(
                Some(&current),
                &worker,
                true,
                &invalid,
                Path::new(root)
            )
            .is_err()
        );
        invalid = notice.clone();
        invalid.source_root = Some("/different-repository".to_string());
        assert!(
            reconcile_core_candidate_preflight(
                Some(&current),
                &worker,
                true,
                &invalid,
                Path::new(root)
            )
            .is_err()
        );
        invalid = notice.clone();
        invalid.source_sha = Some("broken".to_string());
        assert!(
            reconcile_core_candidate_preflight(
                Some(&current),
                &worker,
                true,
                &invalid,
                Path::new(root)
            )
            .is_err()
        );
        assert!(
            reconcile_core_candidate_preflight(
                Some(&current),
                &worker,
                false,
                &notice,
                Path::new(root)
            )
            .is_err()
        );
        assert!(
            reconcile_core_candidate_preflight(
                Some(&current),
                &worker,
                true,
                &notice,
                Path::new(root)
            )
            .is_err()
        ); // missing worktree must never grant new authority
    }

    #[test]
    fn codexdd_046_3b2r_stalled_tui_git_child_is_bounded_and_reaped() {
        #[cfg(windows)]
        let command = {
            let mut command = Command::new("powershell.exe");
            command.args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "Start-Sleep -Seconds 5",
            ]);
            command
        };
        #[cfg(not(windows))]
        let command = {
            let mut command = Command::new("sleep");
            command.arg("5");
            command
        };
        let error = run_bounded_tui_command(command, "synthetic-stall", Duration::from_millis(150))
            .expect_err("stalled child must be terminated and reaped");
        assert!(error.contains("TUI Git synthetic-stall timed out"));
        assert!(error.contains("process terminated"));
    }
}
