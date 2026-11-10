//! Core-owned first accepted user message Worker assignment.
//! This is an admission latch, NOT a durable native-validation credential.

use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_config::config_toml::AdaptiveWorkerRoleToml;
use codex_config::config_toml::parse_adaptive_worker_first_message;
use codex_protocol::protocol::CodexddFirstWorkerSnapshot;
use serde::Deserialize;
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

/// Owner candidate intent; parsing or preflight alone does not grant authority.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)] // 3B.2i-a: candidate authorizer is deliberately not yet wired.
pub(crate) struct CandidateShaAuthorization {
    pub(crate) previous_sha: String,
    pub(crate) candidate_sha: String,
}

/// Separate exact-owner proposal grammar. No model text can change a source pin.
#[allow(dead_code)] // 3B.2i-a: read-only fixture pending trusted turn-input wiring.
pub(crate) fn parse_candidate_sha_authorization(
    text: &str,
) -> Result<Option<CandidateShaAuthorization>, String> {
    let mut lines = text.lines();
    let Some(first) = lines.by_ref().find(|line| !line.trim().is_empty()) else {
        return Ok(None);
    };
    if first.trim() != "[codexdd_candidate]" {
        return Ok(None);
    }
    let mut header = String::from("[codexdd_candidate]\n");
    let mut delimiter = false;
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            delimiter = true;
            break;
        }
        header.push_str(line);
        header.push('\n');
    }
    if !delimiter || !lines.any(|line| !line.trim().is_empty()) {
        return Err(
            "candidate authorization requires a blank line and owner instruction".to_string(),
        );
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Envelope {
        codexdd_candidate: CandidateFields,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct CandidateFields {
        previous_sha: String,
        candidate_sha: String,
    }
    let parsed: Envelope = toml::from_str(&header)
        .map_err(|err| format!("invalid [codexdd_candidate] header: {err}"))?;
    let previous_sha = parsed
        .codexdd_candidate
        .previous_sha
        .trim()
        .to_ascii_lowercase();
    let candidate_sha = parsed
        .codexdd_candidate
        .candidate_sha
        .trim()
        .to_ascii_lowercase();
    for sha in [&previous_sha, &candidate_sha] {
        if sha.len() != 40 || !sha.bytes().all(|ch| ch.is_ascii_hexdigit()) {
            return Err(
                "candidate authorization requires two exact 40-character Git SHAs".to_string(),
            );
        }
    }
    if previous_sha == candidate_sha {
        return Err("candidate authorization cannot silently reuse the same SHA".to_string());
    }
    Ok(Some(CandidateShaAuthorization {
        previous_sha,
        candidate_sha,
    }))
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct CoreFirstWorkerAuthority {
    locked: bool,
    binding: Option<AdaptiveWorkerConfigToml>,
    // Launcher pins are not serialized as first-message credentials.
    accepted_user_origin: bool,
    // Only a flushed Core checkpoint (or a verified same-thread resume)
    // can enable a subsequent native TurnContext source credential.
    checkpointed: bool,
}

/// Read-only source inspection with a deadline for the actual Git child.
/// Unlike timeout(spawn_blocking(Command::output)), this kills and reaps a
/// blocked Git process. Redirecting stdout to a scratch file also prevents
/// inherited pipe handles from keeping Command::output pending indefinitely.
fn run_bounded_candidate_command(
    mut command: Command,
    step: &str,
    deadline: Duration,
) -> Result<(ExitStatus, String), String> {
    let output = tempfile::tempfile()
        .map_err(|err| format!("SETUP_BLOCKED: candidate Git {step} output unavailable: {err}"))?;
    let mut reader = output
        .try_clone()
        .map_err(|err| format!("SETUP_BLOCKED: candidate Git {step} output unreadable: {err}"))?;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::from(output))
        .stderr(Stdio::null());
    let started = Instant::now();
    tracing::debug!(git_step = step, "candidate Git inspection started");
    let mut child = command
        .spawn()
        .map_err(|err| format!("SETUP_BLOCKED: candidate Git {step} unavailable: {err}"))?;

    let exit = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() >= deadline => {
                let terminated = child.kill().is_ok();
                if terminated {
                    let _ = child.wait();
                }
                return Err(format!(
                    "SETUP_BLOCKED: candidate Git {step} timed out after {}ms; process {}",
                    deadline.as_millis(),
                    if terminated { "terminated" } else { "termination unconfirmed" }
                ));
            }
            Ok(None) => thread::sleep(Duration::from_millis(25)),
            Err(err) => {
                let terminated = child.kill().is_ok();
                if terminated {
                    let _ = child.wait();
                }
                return Err(format!(
                    "SETUP_BLOCKED: candidate Git {step} could not be monitored: {err}; process {}",
                    if terminated { "terminated" } else { "termination unconfirmed" }
                ));
            }
        }
    };
    tracing::debug!(
        git_step = step,
        elapsed_ms = started.elapsed().as_millis() as u64,
        "candidate Git inspection finished"
    );
    reader
        .seek(SeekFrom::Start(0))
        .map_err(|err| format!("SETUP_BLOCKED: candidate Git {step} output seek failed: {err}"))?;
    const MAX_OUTPUT: u64 = 1024 * 1024;
    let mut bytes = Vec::new();
    reader
        .take(MAX_OUTPUT + 1)
        .read_to_end(&mut bytes)
        .map_err(|err| format!("SETUP_BLOCKED: candidate Git {step} output read failed: {err}"))?;
    if bytes.len() as u64 > MAX_OUTPUT {
        return Err(format!(
            "SETUP_BLOCKED: candidate Git {step} exceeded bounded output size"
        ));
    }
    let output = String::from_utf8(bytes)
        .map_err(|_| format!("SETUP_BLOCKED: candidate Git {step} returned non-UTF8 output"))?;
    Ok((exit, output))
}

fn run_candidate_git_step(
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
    // Full untracked status needs more time than metadata-only checks on
    // large Windows source trees, but every step is bounded and fail-closed.
    let deadline = if step == "status" {
        Duration::from_secs(12)
    } else {
        Duration::from_secs(5)
    };
    run_bounded_candidate_command(command, step, deadline)
}

impl CoreFirstWorkerAuthority {
    pub(crate) fn from_startup(binding: Option<AdaptiveWorkerConfigToml>) -> Self {
        Self {
            locked: binding.is_some(),
            binding,
            accepted_user_origin: false,
            checkpointed: false,
        }
    }

    pub(crate) fn lock_after_prior_user_history(&mut self) {
        // No Core-authenticated persisted binding exists yet. Never parse
        // replayed prompt text as new authority after cold resume.
        self.locked = true;
    }

    pub(crate) fn prepare(
        &self,
        first_text: Option<&str>,
    ) -> Result<Option<Option<AdaptiveWorkerConfigToml>>, String> {
        if self.locked {
            return Ok(None);
        }
        parse_adaptive_worker_first_message(first_text.unwrap_or("")).map(Some)
    }

    pub(crate) fn accept(&mut self, proposed: Option<Option<AdaptiveWorkerConfigToml>>) -> bool {
        if self.locked {
            return false;
        }
        let Some(binding) = proposed else {
            return false;
        };
        self.accepted_user_origin = binding.is_some();
        self.binding = binding;
        self.locked = true;
        self.checkpointed = false;
        true
    }

    /// Preflight an explicit same-root descendant without changing the active
    /// binding. A later owner-origin turn must persist and flush before promotion.
    #[allow(dead_code)] // 3B.2i-a: deliberately no authority mutation.
    pub(crate) fn verified_candidate_snapshot(
        &self,
        request: &CandidateShaAuthorization,
    ) -> Result<CodexddFirstWorkerSnapshot, String> {
        let Some(worker) = self.runtime_binding() else {
            return Err("SETUP_BLOCKED: no durable first-user Worker source authority".to_string());
        };
        if worker.role != AdaptiveWorkerRoleToml::Implementation
            || !matches!(worker.task_kind.as_deref(), None | Some("source_change"))
        {
            return Err(
                "SETUP_BLOCKED: candidate must belong to a source-changing Implementation Worker"
                    .to_string(),
            );
        }
        let root = worker
            .source_root
            .as_deref()
            .ok_or_else(|| "SETUP_BLOCKED: existing Worker has no exact source root".to_string())?;
        let previous = worker
            .source_sha
            .as_deref()
            .ok_or_else(|| "SETUP_BLOCKED: existing Worker has no pinned source SHA".to_string())?;
        if !previous.eq_ignore_ascii_case(&request.previous_sha) {
            return Err(
                "SETUP_BLOCKED: candidate request has the wrong previous source SHA".to_string(),
            );
        }
        let selected = Path::new(root)
            .canonicalize()
            .map_err(|err| format!("SETUP_BLOCKED: cannot resolve authorized worktree: {err}"))?;
        let git = |step: &'static str, args: &[&str]| -> Result<String, String> {
            let (status, output) = run_candidate_git_step(&selected, step, args)?;
            if !status.success() {
                return Err(format!(
                    "SETUP_BLOCKED: candidate Git {step} failed with status {status}"
                ));
            }
            Ok(output)
        };
        let reported_root = git("root", &["rev-parse", "--show-toplevel"])?;
        let reported = Path::new(reported_root.trim())
            .canonicalize()
            .map_err(|err| format!("SETUP_BLOCKED: invalid Git-reported root: {err}"))?;
        if reported != selected {
            return Err(
                "SETUP_BLOCKED: candidate is not the exact authorized worktree".to_string(),
            );
        }
        let observed = git("head", &["rev-parse", "HEAD"])?;
        if !observed.trim().eq_ignore_ascii_case(&request.candidate_sha) {
            return Err(format!(
                "SETUP_BLOCKED: observed candidate HEAD {} differs from requested {}",
                observed.trim(),
                request.candidate_sha
            ));
        }
        if !git("status", &["status", "--porcelain=v1", "--untracked-files=all"])?
            .trim()
            .is_empty()
        {
            return Err(
                "SETUP_BLOCKED: preserve staged, unstaged, or untracked edits; candidate is dirty"
                    .to_string(),
            );
        }
        let (ancestry, _) = run_candidate_git_step(
            &selected,
            "ancestry",
            &[
                "merge-base",
                "--is-ancestor",
                &request.previous_sha,
                &request.candidate_sha,
            ],
        )?;
        if !ancestry.success() {
            return Err(
                "SETUP_BLOCKED: candidate is not a descendant of the authorized SHA".to_string(),
            );
        }
        let mut snapshot = self.first_user_snapshot().ok_or_else(|| {
            "SETUP_BLOCKED: missing original first-user authority snapshot".to_string()
        })?;
        // Proposal changes only SHA. Role, scope, root and task kind stay fixed.
        snapshot.source_sha = Some(request.candidate_sha.clone());
        Ok(snapshot)
    }

    /// This method is called ONLY after a matching proposed source was
    /// persisted and flushed by Core under the settings persistence permit.
    /// Rejects stale proposals without changing the original Worker scope.
    pub(crate) fn apply_durable_candidate(
        &mut self,
        requested: &CandidateShaAuthorization,
        persisted: &CodexddFirstWorkerSnapshot,
    ) -> Result<(), String> {
        let Some(current) = self.runtime_binding() else {
            return Err(
                "SETUP_BLOCKED: cannot promote uncheckpointed Worker authority".to_string(),
            );
        };
        let Some(original) = self.first_user_snapshot() else {
            return Err("SETUP_BLOCKED: no Core-owned first-user Worker snapshot".to_string());
        };
        if current.role != AdaptiveWorkerRoleToml::Implementation
            || !matches!(current.task_kind.as_deref(), None | Some("source_change"))
            || current.source_sha.as_deref() != Some(requested.previous_sha.as_str())
            || requested.candidate_sha == requested.previous_sha
            || persisted.source_sha.as_deref() != Some(requested.candidate_sha.as_str())
            || persisted.role != original.role
            || persisted.authorized_scope != original.authorized_scope
            || persisted.source_root != original.source_root
            || persisted.task_kind != original.task_kind
        {
            return Err(
                "SETUP_BLOCKED: stale or scope-changing durable candidate promotion".to_string(),
            );
        }
        let Some(binding) = self.binding.as_mut() else {
            return Err("SETUP_BLOCKED: missing original Worker binding".to_string());
        };
        binding.source_sha = Some(requested.candidate_sha.clone());
        Ok(())
    }

    /// Mark durable only after the native thread store confirmed append and flush.
    /// No model/tool argument may call this gate.
    pub(crate) fn mark_checkpointed(&mut self) -> bool {
        if !self.locked || !self.accepted_user_origin || self.binding.is_none() {
            return false;
        }
        self.checkpointed = true;
        true
    }

    /// Only persisted Core user authority may reach a subsequent native turn.
    /// Launcher-configured roles are already applied in the ordinary Config.
    pub(crate) fn runtime_binding(&self) -> Option<AdaptiveWorkerConfigToml> {
        if !self.locked || !self.accepted_user_origin || !self.checkpointed {
            return None;
        }
        self.binding.clone()
    }

    /// Only accepted first-user-message bindings go into trusted Core rollout
    /// settings. Startup bindings stay in launcher config, and plain prompts
    /// never acquire a stored role.
    pub(crate) fn first_user_snapshot(&self) -> Option<CodexddFirstWorkerSnapshot> {
        if !self.accepted_user_origin {
            return None;
        }
        let binding = self.binding.as_ref()?;
        let role = match binding.role {
            AdaptiveWorkerRoleToml::Implementation => "implementation",
            AdaptiveWorkerRoleToml::Validation => "validation",
            AdaptiveWorkerRoleToml::Repair => "repair",
            AdaptiveWorkerRoleToml::Unspecified => return None,
        };
        Some(CodexddFirstWorkerSnapshot {
            role: role.to_string(),
            authorized_scope: binding.authorized_scope.clone().unwrap_or_default(),
            source_root: binding.source_root.clone(),
            source_sha: binding.source_sha.clone(),
            task_kind: binding.task_kind.clone(),
        })
    }

    /// Rehydrate only an exact-thread native settings snapshot. The caller
    /// verifies its owner and the presence of previous user history first.
    /// An explicit launcher binding wins over a persisted first-message role.
    pub(crate) fn restore_first_user_snapshot(
        &mut self,
        snapshot: &CodexddFirstWorkerSnapshot,
    ) -> Result<bool, String> {
        if self.locked {
            return Ok(false);
        }
        let role = match snapshot.role.as_str() {
            "implementation" => AdaptiveWorkerRoleToml::Implementation,
            "validation" => AdaptiveWorkerRoleToml::Validation,
            "repair" => AdaptiveWorkerRoleToml::Repair,
            _ => return Err("persisted Worker role is invalid".to_string()),
        };
        let scope = snapshot.authorized_scope.trim();
        if scope.is_empty() {
            return Err("persisted Worker authorized_scope is empty".to_string());
        }
        let task_kind = snapshot.task_kind.as_deref().unwrap_or("source_change");
        if !matches!(
            task_kind,
            "source_change" | "operations_preview" | "benchmark"
        ) {
            return Err("persisted Worker task_kind is invalid".to_string());
        }
        let (source_root, source_sha) = match (
            snapshot.source_root.as_deref(),
            snapshot.source_sha.as_deref(),
        ) {
            (None, None) if task_kind == "source_change" => (None, None),
            (Some(root), Some(sha))
                if Path::new(root).is_absolute()
                    && sha.len() == 40
                    && sha.bytes().all(|ch| ch.is_ascii_hexdigit()) =>
            {
                (Some(root.to_string()), Some(sha.to_ascii_lowercase()))
            }
            _ => {
                return Err(
                    "persisted Worker source requires exact root and 40-digit SHA".to_string(),
                );
            }
        };
        self.binding = Some(AdaptiveWorkerConfigToml {
            role,
            authorized_scope: Some(scope.to_string()),
            source_root,
            source_sha,
            task_kind: snapshot.task_kind.clone(),
        });
        self.accepted_user_origin = true;
        self.locked = true;
        self.checkpointed = true;
        Ok(true)
    }

    #[cfg(test)]
    pub(crate) fn binding(&self) -> Option<&AdaptiveWorkerConfigToml> {
        self.binding.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn locked(&self) -> bool {
        self.locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_config::config_toml::AdaptiveWorkerRoleToml;

    const IMPLEMENT: &str = "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"approved source\"\n\nImplement.";
    const REPAIR: &str =
        "[adaptive_worker]\nrole = \"repair\"\nauthorized_scope = \"late spoof\"\n\nRepair.";

    #[test]
    fn codexdd_046_3b2f_first_accepted_role_is_immutable() {
        let mut state = CoreFirstWorkerAuthority::default();
        let proposal = state.prepare(Some(IMPLEMENT)).expect("valid header");
        assert!(!state.locked());
        assert!(state.accept(proposal));
        assert!(state.locked());
        assert_eq!(
            state.binding().map(|binding| binding.role),
            Some(AdaptiveWorkerRoleToml::Implementation)
        );
        assert_eq!(state.prepare(Some(REPAIR)), Ok(None));
        assert!(!state.accept(None));
    }

    #[test]
    fn codexdd_046_3b2f_plain_or_rejected_first_turn_is_safe() {
        let mut state = CoreFirstWorkerAuthority::default();
        assert!(
            state
                .prepare(Some("[adaptive_worker]\nrole = \"invalid\""))
                .is_err()
        );
        assert!(!state.locked());
        let plain = state.prepare(Some("Normal user prompt.")).expect("plain");
        assert_eq!(plain, Some(None));
        assert!(state.accept(plain));
        assert!(state.locked());
        assert_eq!(state.prepare(Some(IMPLEMENT)), Ok(None));
        assert!(state.binding().is_none());
    }

    #[test]
    fn codexdd_046_3b2f_startup_and_cold_resume_do_not_rebind() {
        let mut startup = CoreFirstWorkerAuthority::from_startup(
            parse_adaptive_worker_first_message(REPAIR).expect("header"),
        );
        assert_eq!(startup.prepare(Some(IMPLEMENT)), Ok(None));
        startup.lock_after_prior_user_history();
        assert_eq!(
            startup.binding().map(|binding| binding.role),
            Some(AdaptiveWorkerRoleToml::Repair)
        );
        let mut resume = CoreFirstWorkerAuthority::default();
        resume.lock_after_prior_user_history();
        assert_eq!(resume.prepare(Some(IMPLEMENT)), Ok(None));
        assert!(resume.binding().is_none());
    }
    #[test]
    fn codexdd_046_3b2g_first_user_snapshot_roundtrips_without_replaying_prompt() {
        let root = if cfg!(windows) {
            r"C:\authorized\worktree"
        } else {
            "/tmp/authorized/worktree"
        };
        let prompt = format!(
            "[adaptive_worker]\\nrole = \"implementation\"\\nauthorized_scope = \"source packet\"\\nsource_root = '{root}'\\nsource_sha = \"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"\\n\\nImplement."
        );
        let prompt = prompt.replace("\\n", "\n");
        let mut original = CoreFirstWorkerAuthority::default();
        assert!(original.accept(original.prepare(Some(&prompt)).expect("parsed")));
        let snapshot = original
            .first_user_snapshot()
            .expect("native first-user binding");
        let serialized = serde_json::to_string(&snapshot).expect("encode");
        let decoded: CodexddFirstWorkerSnapshot =
            serde_json::from_str(&serialized).expect("decode");
        let mut resumed = CoreFirstWorkerAuthority::default();
        assert_eq!(resumed.restore_first_user_snapshot(&decoded), Ok(true));
        assert_eq!(resumed.prepare(Some(REPAIR)), Ok(None));
        assert_eq!(resumed.binding(), original.binding());
    }

    #[test]
    fn codexdd_046_3b2g_rejects_tampered_or_partial_native_snapshot() {
        let mut state = CoreFirstWorkerAuthority::default();
        let mut snapshot = CodexddFirstWorkerSnapshot {
            role: "implementation".to_string(),
            authorized_scope: "approved source".to_string(),
            source_root: Some(
                if cfg!(windows) {
                    r"C:\source\repo"
                } else {
                    "/tmp/source/repo"
                }
                .to_string(),
            ),
            source_sha: Some("a".repeat(40)),
            task_kind: Some("source_change".to_string()),
        };
        snapshot.source_sha = Some("not-a-commit".to_string());
        assert!(state.restore_first_user_snapshot(&snapshot).is_err());
        assert!(!state.locked());
        snapshot.source_sha = None;
        assert!(state.restore_first_user_snapshot(&snapshot).is_err());
        snapshot.source_root = None;
        snapshot.role = "unspecified".to_string();
        assert!(state.restore_first_user_snapshot(&snapshot).is_err());
        assert!(state.binding().is_none());
    }

    #[test]
    fn codexdd_046_3b2g_startup_and_plain_first_message_have_no_user_snapshot() {
        let mut plain = CoreFirstWorkerAuthority::default();
        assert!(
            plain.accept(
                plain
                    .prepare(Some("Ordinary first user task."))
                    .expect("plain")
            )
        );
        assert!(plain.first_user_snapshot().is_none());
        let startup = CoreFirstWorkerAuthority::from_startup(
            parse_adaptive_worker_first_message(IMPLEMENT).expect("launcher fixture"),
        );
        assert!(startup.first_user_snapshot().is_none());
        let mut pinned = startup.clone();
        let proposed = CodexddFirstWorkerSnapshot {
            role: "repair".to_string(),
            authorized_scope: "replayed override".to_string(),
            source_root: None,
            source_sha: None,
            task_kind: None,
        };
        assert_eq!(pinned.restore_first_user_snapshot(&proposed), Ok(false));
        assert_eq!(pinned.binding(), startup.binding());
    }

    #[test]
    fn codexdd_046_3b2h_unflushed_authority_cannot_enter_runtime() {
        let mut first = CoreFirstWorkerAuthority::default();
        let proposed = first.prepare(Some(IMPLEMENT)).expect("first header");
        assert!(first.accept(proposed));
        assert!(first.first_user_snapshot().is_some());
        assert!(first.runtime_binding().is_none());
        assert!(first.mark_checkpointed());
        assert_eq!(first.runtime_binding(), first.binding().cloned());
    }

    #[test]
    fn codexdd_046_3b2h_plain_or_launcher_authority_cannot_forge_checkpoint() {
        let mut plain = CoreFirstWorkerAuthority::default();
        let proposed = plain
            .prepare(Some("Ordinary first message"))
            .expect("plain");
        assert!(plain.accept(proposed));
        assert!(!plain.mark_checkpointed());
        assert!(plain.runtime_binding().is_none());

        let mut launcher = CoreFirstWorkerAuthority::from_startup(
            parse_adaptive_worker_first_message(REPAIR).expect("launcher binding"),
        );
        assert!(!launcher.mark_checkpointed());
        assert!(launcher.runtime_binding().is_none());
    }
    fn candidate_fixture() -> (
        tempfile::TempDir,
        CoreFirstWorkerAuthority,
        CandidateShaAuthorization,
    ) {
        let tmp = tempfile::tempdir().expect("temporary Git fixture");
        let root = tmp.path();
        let git = |args: &[&str]| -> String {
            let output = Command::new("git")
                .arg("-C")
                .arg(root)
                .args(args)
                .output()
                .expect("Git available");
            assert!(
                output.status.success(),
                "Git fixture failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            String::from_utf8(output.stdout)
                .expect("Git output")
                .trim()
                .to_string()
        };
        git(&["init"]);
        git(&["config", "user.email", "codexdd-fixture@example.invalid"]);
        git(&["config", "user.name", "CodexDD regression"]);
        std::fs::write(root.join("candidate.txt"), "original").expect("first file");
        git(&["add", "candidate.txt"]);
        git(&["commit", "-m", "authorized source"]);
        let previous_sha = git(&["rev-parse", "HEAD"]);
        std::fs::write(root.join("candidate.txt"), "next").expect("next file");
        git(&["add", "candidate.txt"]);
        git(&["commit", "-m", "reviewed candidate"]);
        let candidate_sha = git(&["rev-parse", "HEAD"]);
        let prompt = format!(
            "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"approved packet\"\nsource_root = '{}'\nsource_sha = \"{}\"\n\nImplement.",
            root.display(),
            previous_sha
        );
        let mut authority = CoreFirstWorkerAuthority::default();
        let proposed = authority.prepare(Some(&prompt)).expect("first binding");
        assert!(authority.accept(proposed));
        assert!(authority.mark_checkpointed());
        (
            tmp,
            authority,
            CandidateShaAuthorization {
                previous_sha,
                candidate_sha,
            },
        )
    }

    #[test]
    fn codexdd_046_3b2p_stalled_git_process_is_bounded_and_reaped() {
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
        let error = run_bounded_candidate_command(
            command,
            "synthetic-stall",
            Duration::from_millis(150),
        )
        .expect_err("a stalled child must be terminated");
        assert!(error.contains("Git synthetic-stall timed out"));
        assert!(error.contains("process terminated"));
    }

    #[test]
    fn codexdd_046_3b2i_candidate_header_requires_explicit_shas() {
        assert_eq!(
            parse_candidate_sha_authorization("Progress update"),
            Ok(None)
        );
        assert_eq!(
            parse_candidate_sha_authorization("[adaptive_worker]\nrole = \"implementation\""),
            Ok(None)
        );
        let previous = "a".repeat(40);
        let candidate = "b".repeat(40);
        let header = format!(
            "[codexdd_candidate]\nprevious_sha = \"{previous}\"\ncandidate_sha = \"{candidate}\"\n\nApprove this exact source."
        );
        assert_eq!(
            parse_candidate_sha_authorization(&header).expect("valid proposal"),
            Some(CandidateShaAuthorization {
                previous_sha: previous.clone(),
                candidate_sha: candidate.clone()
            })
        );
        assert!(
            parse_candidate_sha_authorization(
                &header.replace("candidate_sha =", "source_root = '/other'\ncandidate_sha =")
            )
            .is_err()
        );
        assert!(parse_candidate_sha_authorization(&header.replace(&candidate, &previous)).is_err());
        assert!(
            parse_candidate_sha_authorization(&header.replace(&candidate, "not-a-sha")).is_err()
        );
        assert!(
            parse_candidate_sha_authorization(
                &header.replace("\n\nApprove this exact source.", "")
            )
            .is_err()
        );
    }

    #[test]
    fn codexdd_046_3b2i_descendant_preflight_does_not_rebind() {
        let (_tmp, authority, request) = candidate_fixture();
        let old = authority.runtime_binding().expect("original authority");
        let proposed = authority
            .verified_candidate_snapshot(&request)
            .expect("clean descendant");
        assert_eq!(
            proposed.source_sha.as_deref(),
            Some(request.candidate_sha.as_str())
        );
        assert_eq!(proposed.source_root, old.source_root);
        assert_eq!(proposed.authorized_scope, old.authorized_scope.unwrap());
        assert_eq!(
            authority
                .runtime_binding()
                .expect("unchanged authority")
                .source_sha
                .as_deref(),
            Some(request.previous_sha.as_str())
        );
    }

    #[test]
    fn codexdd_046_3b2i_rejects_unbound_wrong_sha_and_dirty_candidate() {
        let (tmp, authority, request) = candidate_fixture();
        assert!(
            CoreFirstWorkerAuthority::default()
                .verified_candidate_snapshot(&request)
                .is_err()
        );
        let launcher = CoreFirstWorkerAuthority::from_startup(authority.binding().cloned());
        assert!(launcher.verified_candidate_snapshot(&request).is_err());
        let wrong_parent = CandidateShaAuthorization {
            previous_sha: "f".repeat(40),
            candidate_sha: request.candidate_sha.clone(),
        };
        assert!(
            authority
                .verified_candidate_snapshot(&wrong_parent)
                .is_err()
        );
        let wrong_head = CandidateShaAuthorization {
            previous_sha: request.previous_sha.clone(),
            candidate_sha: "e".repeat(40),
        };
        assert!(authority.verified_candidate_snapshot(&wrong_head).is_err());
        std::fs::write(tmp.path().join("untracked.txt"), "preserve").expect("untracked file");
        let blocker = authority
            .verified_candidate_snapshot(&request)
            .expect_err("dirty candidate");
        assert!(blocker.contains("staged, unstaged, or untracked"));
        assert_eq!(
            std::fs::read_to_string(tmp.path().join("untracked.txt")).expect("preserved file"),
            "preserve"
        );
    }

    #[test]
    fn codexdd_046_3b2m_successor_snapshot_survives_cold_restore_without_reassignment() {
        let (_tmp, mut original, requested) = candidate_fixture();
        let proposed = original
            .verified_candidate_snapshot(&requested)
            .expect("verified exact successor");
        original
            .apply_durable_candidate(&requested, &proposed)
            .expect("simulated completed native flush");
        let successor = original.first_user_snapshot().expect("successor snapshot");
        let encoded = serde_json::to_string(&successor).expect("serialize");
        let decoded: CodexddFirstWorkerSnapshot =
            serde_json::from_str(&encoded).expect("deserialize");
        let mut resumed = CoreFirstWorkerAuthority::default();
        assert_eq!(resumed.restore_first_user_snapshot(&decoded), Ok(true));
        resumed.lock_after_prior_user_history();
        let binding = resumed.runtime_binding().expect("restored Worker");
        assert_eq!(
            binding.source_sha.as_deref(),
            Some(requested.candidate_sha.as_str())
        );
        assert_eq!(binding.authorized_scope.as_deref(), Some("approved packet"));
        assert_eq!(binding.role, AdaptiveWorkerRoleToml::Implementation);
        assert_eq!(
            binding.source_root,
            original.runtime_binding().unwrap().source_root
        );
        assert_eq!(resumed.prepare(Some(REPAIR)), Ok(None));
        let mut fork = CoreFirstWorkerAuthority::default();
        fork.lock_after_prior_user_history();
        assert!(fork.runtime_binding().is_none());
    }

    #[test]
    fn codexdd_046_3b2i_candidate_promotion_preserves_role_and_scope() {
        let (_tmp, mut authority, request) = candidate_fixture();
        let prospective = authority
            .verified_candidate_snapshot(&request)
            .expect("verified");
        let mut altered = prospective.clone();
        altered.authorized_scope = "unapproved change".to_string();
        assert!(
            authority
                .apply_durable_candidate(&request, &altered)
                .is_err()
        );
        assert_eq!(
            authority
                .runtime_binding()
                .expect("original binding")
                .source_sha
                .as_deref(),
            Some(request.previous_sha.as_str())
        );
        authority
            .apply_durable_candidate(&request, &prospective)
            .expect("apply flushed candidate");
        let current = authority.runtime_binding().expect("new binding");
        assert_eq!(
            current.source_sha.as_deref(),
            Some(request.candidate_sha.as_str())
        );
        assert_eq!(current.authorized_scope.as_deref(), Some("approved packet"));
        assert_eq!(current.role, AdaptiveWorkerRoleToml::Implementation);
        assert!(
            authority
                .apply_durable_candidate(&request, &prospective)
                .is_err()
        );
    }
}
