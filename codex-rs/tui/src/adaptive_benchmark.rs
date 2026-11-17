//! Phase 3D.1c: original-owner benchmark authorization and native completion admission.
//!
//! This module does not execute shell commands, allocate Worker seats, or mint a
//! QA PASS. Commands run only through existing guarded Core command execution.
//! Callers must supply events from the *local trusted app-server connection*;
//! model prose, generic Validation PASS, and raw shell text are never evidence.

use crate::adaptive_worker::AdaptiveTaskKind;
use crate::adaptive_worker::AdaptiveWorkerContext;
use crate::adaptive_worker::AdaptiveWorkerPreflight;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::preflight_owner_candidate_identity;
use codex_app_server_protocol::CommandExecutionSource;
use codex_app_server_protocol::CommandExecutionStatus;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::ItemStartedNotification;
use codex_app_server_protocol::ThreadItem;
use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_protocol::ThreadId;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const MAX_COMMAND_BYTES: usize = 4096;
const MAX_BENCHMARK_DURATION_MS: u64 = 120_000;
const MAX_EVENT_AGE_MS: u64 = 300_000;

/// Only values from immutable original owner configuration may create this.
#[derive(Debug)]
pub(crate) struct AdaptiveApprovedBenchmark {
    scope: String,
    root: PathBuf,
    sha: String,
    command_2: String,
    command_4: String,
    lock_path: PathBuf,
    max_duration_ms: u64,
    // Fresh in-memory authorization bounds native event freshness and replay.
    bound_at_ms: u64,
    consumed: bool,
}

/// A temporary exclusive reservation at the owner's exact scratch lock path.
/// This does not grant command-execution or host-process privileges.
#[derive(Debug)]
pub(crate) struct AdaptiveBenchmarkLock {
    path: PathBuf,
    file: Option<File>,
    acquired_at_ms: u64,
}

impl AdaptiveBenchmarkLock {
    /// A path that merely exists is not proof that the original owner's
    /// *locked file handle* still refers to that path. Reject a removed or
    /// replaced lock rather than accepting another process's scratch file.
    fn owns_original_path(&self) -> bool {
        let Some(file) = self.file.as_ref() else {
            return false;
        };
        let (Ok(held), Ok(at_path)) = (file.metadata(), std::fs::metadata(&self.path)) else {
            return false;
        };
        if !held.is_file() || !at_path.is_file() {
            return false;
        }
        #[cfg(windows)]
        {
            use std::os::windows::io::AsRawHandle;
            use windows_sys::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION;
            use windows_sys::Win32::Storage::FileSystem::GetFileInformationByHandle;

            let Ok(at_path_file) = OpenOptions::new().read(true).open(&self.path) else {
                return false;
            };
            let identity = |opened: &File| {
                let mut info = std::mem::MaybeUninit::<BY_HANDLE_FILE_INFORMATION>::uninit();
                // SAFETY: the raw handle belongs to a live File, and the Win32
                // function initializes info only when it returns nonzero.
                let found = unsafe {
                    GetFileInformationByHandle(opened.as_raw_handle(), info.as_mut_ptr())
                };
                (found != 0).then(|| {
                    // SAFETY: successful GetFileInformationByHandle initialized info.
                    let info = unsafe { info.assume_init() };
                    (
                        info.dwVolumeSerialNumber,
                        info.nFileIndexHigh,
                        info.nFileIndexLow,
                    )
                })
            };
            identity(file).is_some_and(|original| identity(&at_path_file) == Some(original))
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            held.dev() == at_path.dev() && held.ino() == at_path.ino()
        }
        #[cfg(not(any(windows, unix)))]
        {
            false
        }
    }
}

impl Drop for AdaptiveBenchmarkLock {
    fn drop(&mut self) {
        // Only a successfully acquired owner lease may clean up its own
        // scratch path. A failed lock attempt must not delete a file another
        // process has since acquired, even if the handle inode is unchanged.
        let owns_path = self.acquired_at_ms != 0 && self.owns_original_path();
        drop(self.file.take());
        if owns_path {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveNativeBenchmarkMeasurement {
    pub(crate) threads: u8,
    pub(crate) item_id: String,
    pub(crate) process_id: String,
    pub(crate) duration_ms: u64,
    pub(crate) completed_at_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveNativeBenchmarkComparison {
    pub(crate) source_sha: String,
    pub(crate) two: AdaptiveNativeBenchmarkMeasurement,
    pub(crate) four: AdaptiveNativeBenchmarkMeasurement,
}

impl AdaptiveApprovedBenchmark {
    /// Bind a pair of *literal* commands and a scratch resource lock from
    /// first-assignment or startup config. No continuation can add authority.
    pub(crate) fn from_original_owner(
        original: &AdaptiveWorkerConfigToml,
        worker: &AdaptiveWorkerContext,
        pin: Option<&AdaptiveWorkerPreflight>,
        runtime_cwd: &Path,
    ) -> Result<Self, String> {
        let plan = original.benchmark.as_ref().ok_or_else(|| {
            "SETUP_BLOCKED: original owner did not specify two benchmark commands and lock"
                .to_string()
        })?;
        let pin = pin.ok_or_else(|| {
            "SETUP_BLOCKED: benchmark is missing its original source pin".to_string()
        })?;
        let scope = worker.authorized_scope.as_deref().unwrap_or_default();
        if worker.role != AdaptiveWorkerRole::Implementation
            || original.role != codex_config::config_toml::AdaptiveWorkerRoleToml::Implementation
            || original.task_kind.as_deref() != Some("benchmark")
            || pin.task_kind != AdaptiveTaskKind::Benchmark
            || scope.is_empty()
            || original.authorized_scope.as_deref() != Some(scope)
            || original.source_root.as_deref() != Some(pin.source_root.as_str())
            || original.source_sha.as_deref() != Some(pin.source_sha.as_str())
        {
            return Err(
                "SETUP_BLOCKED: benchmark must match immutable Implementation owner and exact source"
                    .to_string(),
            );
        }
        preflight_owner_candidate_identity(pin, runtime_cwd)?;
        for command in [&plan.command_2, &plan.command_4] {
            if command.trim().is_empty()
                || command.len() > MAX_COMMAND_BYTES
                || command.chars().any(char::is_control)
            {
                return Err("SETUP_BLOCKED: invalid original benchmark command".to_string());
            }
        }
        if plan.command_2 == plan.command_4
            || plan.max_duration_ms == 0
            || plan.max_duration_ms > MAX_BENCHMARK_DURATION_MS
        {
            return Err(
                "SETUP_BLOCKED: benchmark needs two distinct bounded original commands".to_string(),
            );
        }
        let root = Path::new(&pin.source_root)
            .canonicalize()
            .map_err(|_| "SETUP_BLOCKED: approved benchmark root disappeared".to_string())?;
        let lock_path = PathBuf::from(&plan.resource_lock);
        let parent = lock_path
            .parent()
            .ok_or_else(|| "SETUP_BLOCKED: missing benchmark lock parent".to_string())?;
        let scratch = std::env::temp_dir()
            .canonicalize()
            .map_err(|_| "SETUP_BLOCKED: scratch directory unavailable".to_string())?;
        let lock_parent = parent
            .canonicalize()
            .map_err(|_| "SETUP_BLOCKED: approved lock parent unavailable".to_string())?;
        if !lock_path.is_absolute()
            || lock_path.file_name().is_none()
            || !lock_parent.starts_with(&scratch)
            || lock_parent.starts_with(&root)
            || lock_path.exists()
        {
            return Err(
                "SETUP_BLOCKED: benchmark lock must name an unoccupied owner-approved scratch file"
                    .to_string(),
            );
        }
        let bound_at_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "SETUP_BLOCKED: benchmark admission clock unavailable".to_string())?
            .as_millis() as u64;
        Ok(Self {
            scope: scope.to_string(),
            root,
            sha: pin.source_sha.clone(),
            command_2: plan.command_2.clone(),
            command_4: plan.command_4.clone(),
            lock_path,
            max_duration_ms: plan.max_duration_ms,
            bound_at_ms,
            consumed: false,
        })
    }

    /// Open with exclusive create: a pre-existing lock is not an invitation to
    /// replace, reuse or remove a different owner's lock file.
    pub(crate) fn reserve_resource_lock(&self) -> Result<AdaptiveBenchmarkLock, String> {
        if self.lock_path.exists() {
            return Err("SETUP_BLOCKED: benchmark resource lock is already occupied".to_string());
        }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&self.lock_path)
            .map_err(|_| {
                "SETUP_BLOCKED: cannot exclusively acquire approved benchmark lock".to_string()
            })?;
        // Own the file handle before trying to lock or read the clock. All
        // error paths then use Drop's same-file identity check, never a blind
        // pathname unlink that could delete another owner's replacement lock.
        let mut held = AdaptiveBenchmarkLock {
            path: self.lock_path.clone(),
            file: Some(file),
            acquired_at_ms: 0,
        };
        let held_file = held
            .file
            .as_ref()
            .ok_or_else(|| "SETUP_BLOCKED: benchmark lock handle disappeared".to_string())?;
        if let Err(error) = held_file.try_lock() {
            return Err(format!(
                "SETUP_BLOCKED: resource lock cannot be held exclusively: {error}"
            ));
        }
        held.acquired_at_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "SETUP_BLOCKED: benchmark lock clock unavailable".to_string())?
            .as_millis() as u64;
        Ok(held)
    }

    /// Correlate a command's native start and completion shapes before
    /// admitting a benchmark. This verifies item identity and event ordering,
    /// not the authenticity of the source connection: the caller must prove
    /// that both events came from the live, local embedded Core stream.
    #[allow(clippy::too_many_arguments)]
    fn match_native_start(
        &self,
        start: Option<&ItemStartedNotification>,
        done: Option<&ItemCompletedNotification>,
        expected_command: &str,
        thread_id: ThreadId,
        turn_id: &str,
        run_started_at_ms: u64,
        lock: &AdaptiveBenchmarkLock,
    ) -> Result<(), String> {
        let start = start.ok_or_else(|| {
            "SETUP_BLOCKED: benchmark lacks a native command-start event".to_string()
        })?;
        let done = done.ok_or_else(|| {
            "SETUP_BLOCKED: benchmark lacks a native command-completion event".to_string()
        })?;
        let ThreadItem::CommandExecution {
            id,
            command,
            cwd,
            plugin_id,
            script_path,
            source,
            status,
            process_id,
            exit_code,
            duration_ms,
            ..
        } = &start.item
        else {
            return Err("SETUP_BLOCKED: benchmark start is not a native command".to_string());
        };
        let ThreadItem::CommandExecution {
            id: done_id,
            command: done_command,
            cwd: done_cwd,
            process_id: done_process_id,
            source: done_source,
            ..
        } = &done.item
        else {
            return Err("SETUP_BLOCKED: benchmark completion is not a native command".to_string());
        };
        let started = u64::try_from(start.started_at_ms)
            .map_err(|_| "SETUP_BLOCKED: benchmark native start time invalid".to_string())?;
        let completed = u64::try_from(done.completed_at_ms)
            .map_err(|_| "SETUP_BLOCKED: benchmark native completion time invalid".to_string())?;
        if !lock.owns_original_path()
            || started <= lock.acquired_at_ms
            || started < run_started_at_ms
            || started > completed
            || start.thread_id != thread_id.to_string()
            || start.turn_id != turn_id
            || done.thread_id != start.thread_id
            || done.turn_id != start.turn_id
            || id.is_empty()
            || id != done_id
            || command != expected_command
            || command != done_command
            || cwd != done_cwd
            || Path::new(cwd.as_str()).canonicalize().ok().as_ref() != Some(&self.root)
            || plugin_id.is_some()
            || script_path.is_some()
            || !matches!(
                source,
                CommandExecutionSource::Agent | CommandExecutionSource::UnifiedExecStartup
            )
            || source != done_source
            || *status != CommandExecutionStatus::InProgress
            || exit_code.is_some()
            || duration_ms.is_some()
            || process_id
                .as_ref()
                .is_some_and(|process_id| Some(process_id) != done_process_id.as_ref())
        {
            return Err(
                "SETUP_BLOCKED: native benchmark start does not match owner command, lock, thread, turn or completion"
                    .to_string(),
            );
        }
        Ok(())
    }

    /// Require two matching start/completion pairs. The existing completion
    /// admission still performs exact source, role, duration, exit and one-use
    /// checks. Neither function may mint QA without trusted Core ingress.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn admit_native_lifecycles(
        &mut self,
        worker: &AdaptiveWorkerContext,
        pin: Option<&AdaptiveWorkerPreflight>,
        runtime_cwd: &Path,
        held_lock: Option<&AdaptiveBenchmarkLock>,
        thread_id: ThreadId,
        turn_id: &str,
        started_at_ms: u64,
        now_ms: u64,
        two_start: Option<&ItemStartedNotification>,
        two_done: Option<&ItemCompletedNotification>,
        four_start: Option<&ItemStartedNotification>,
        four_done: Option<&ItemCompletedNotification>,
    ) -> Result<AdaptiveNativeBenchmarkComparison, String> {
        if self.consumed {
            return Err(
                "SETUP_BLOCKED: benchmark outcomes already consumed in this owner session"
                    .to_string(),
            );
        }
        let lock = held_lock
            .ok_or_else(|| "SETUP_BLOCKED: benchmark resource lock is not held".to_string())?;
        self.match_native_start(
            two_start,
            two_done,
            &self.command_2,
            thread_id,
            turn_id,
            started_at_ms,
            lock,
        )?;
        self.match_native_start(
            four_start,
            four_done,
            &self.command_4,
            thread_id,
            turn_id,
            started_at_ms,
            lock,
        )?;
        // A 2-vs-4 comparison is not meaningful when the two measured runs
        // overlap or execute out of order. Native event timestamps, rather
        // than model prose or a reconstructed test fixture, establish order.
        if four_start.is_none_or(|four| {
            two_done.is_none_or(|two| four.started_at_ms < two.completed_at_ms)
        }) {
            return Err(
                "SETUP_BLOCKED: benchmark runs overlap or are out of required 2-then-4 order"
                    .to_string(),
            );
        }
        self.admit_native_completions(
            worker,
            pin,
            runtime_cwd,
            held_lock,
            thread_id,
            turn_id,
            started_at_ms,
            now_ms,
            two_done,
            four_done,
        )
    }

    /// Completion-only matching is deliberately private: an external caller
    /// must not bypass admit_native_lifecycles and its required start-event,
    /// original-thread and pre-lock refusal checks. Trusted Core ingress and
    /// real guarded execution remain separate mandatory proof boundaries.
    #[allow(clippy::too_many_arguments)]
    fn admit_native_completions(
        &mut self,
        worker: &AdaptiveWorkerContext,
        pin: Option<&AdaptiveWorkerPreflight>,
        runtime_cwd: &Path,
        held_lock: Option<&AdaptiveBenchmarkLock>,
        thread_id: ThreadId,
        turn_id: &str,
        started_at_ms: u64,
        now_ms: u64,
        two: Option<&ItemCompletedNotification>,
        four: Option<&ItemCompletedNotification>,
    ) -> Result<AdaptiveNativeBenchmarkComparison, String> {
        if self.consumed {
            return Err(
                "SETUP_BLOCKED: benchmark outcomes already consumed in this owner session"
                    .to_string(),
            );
        }
        let pin = pin.ok_or_else(|| "SETUP_BLOCKED: missing benchmark source pin".to_string())?;
        if worker.role != AdaptiveWorkerRole::Implementation
            || worker.authorized_scope.as_deref() != Some(self.scope.as_str())
            || pin.task_kind != AdaptiveTaskKind::Benchmark
            || pin.source_sha != self.sha
            || Path::new(&pin.source_root).canonicalize().ok().as_ref() != Some(&self.root)
        {
            return Err("SETUP_BLOCKED: original benchmark source or role drifted".to_string());
        }
        preflight_owner_candidate_identity(pin, runtime_cwd)?;
        if !held_lock.is_some_and(|lock| lock.path == self.lock_path && lock.owns_original_path()) {
            return Err("SETUP_BLOCKED: benchmark resource lock is not held".to_string());
        }
        if turn_id.is_empty()
            || started_at_ms == 0
            || started_at_ms < self.bound_at_ms
            // A post-hoc resource lock cannot authorize already-started work.
            // Equal millisecond timestamps are ambiguous, so refuse them too.
            || held_lock.is_some_and(|lock| started_at_ms <= lock.acquired_at_ms)
            || now_ms < started_at_ms
            || now_ms - started_at_ms > MAX_EVENT_AGE_MS
        {
            return Err("SETUP_BLOCKED: benchmark session is stale or unbound".to_string());
        }
        let two = self.match_measurement(
            two,
            &self.command_2,
            2,
            thread_id,
            turn_id,
            started_at_ms,
            now_ms,
        )?;
        let four = self.match_measurement(
            four,
            &self.command_4,
            4,
            thread_id,
            turn_id,
            started_at_ms,
            now_ms,
        )?;
        if two.item_id == four.item_id || two.process_id == four.process_id {
            return Err("SETUP_BLOCKED: benchmark reuses an item or OS process".to_string());
        }
        preflight_owner_candidate_identity(pin, runtime_cwd)?;
        self.consumed = true;
        Ok(AdaptiveNativeBenchmarkComparison {
            source_sha: self.sha.clone(),
            two,
            four,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn match_measurement(
        &self,
        event: Option<&ItemCompletedNotification>,
        expected_command: &str,
        threads: u8,
        thread_id: ThreadId,
        turn_id: &str,
        started_at_ms: u64,
        now_ms: u64,
    ) -> Result<AdaptiveNativeBenchmarkMeasurement, String> {
        let event = event.ok_or_else(|| {
            "SETUP_BLOCKED: both genuine native benchmark outcomes are mandatory".to_string()
        })?;
        let ThreadItem::CommandExecution {
            id,
            plugin_id,
            script_path,
            command,
            cwd,
            process_id,
            source,
            status,
            exit_code,
            duration_ms,
            ..
        } = &event.item
        else {
            return Err(
                "SETUP_BLOCKED: benchmark requires native Core command completion".to_string(),
            );
        };
        let Some(pid) = process_id
            .as_deref()
            .filter(|value| value.len() <= 10 && value.parse::<u32>().is_ok_and(|pid| pid != 0))
        else {
            return Err(
                "SETUP_BLOCKED: benchmark lacks a positive decimal OS process identifier"
                    .to_string(),
            );
        };
        let duration = (*duration_ms)
            .and_then(|value| u64::try_from(value).ok())
            .ok_or_else(|| "SETUP_BLOCKED: benchmark has no nonnegative duration".to_string())?;
        let completed_at_ms = u64::try_from(event.completed_at_ms)
            .map_err(|_| "SETUP_BLOCKED: benchmark completion time is invalid".to_string())?;
        if id.is_empty()
            || id.len() > 256
            || plugin_id.is_some()
            || script_path.is_some()
            || command != expected_command
            || !matches!(
                source,
                CommandExecutionSource::Agent | CommandExecutionSource::UnifiedExecStartup
            )
            || *status != CommandExecutionStatus::Completed
            || *exit_code != Some(0)
            || duration == 0
            || duration > self.max_duration_ms
            || duration
                > completed_at_ms
                    .saturating_sub(started_at_ms)
                    .saturating_add(1)
            || event.thread_id != thread_id.to_string()
            || event.turn_id != turn_id
            || completed_at_ms < started_at_ms
            || completed_at_ms > now_ms
            || now_ms - completed_at_ms > MAX_EVENT_AGE_MS
            || Path::new(cwd.as_str()).canonicalize().ok().as_ref() != Some(&self.root)
        {
            return Err("SETUP_BLOCKED: native benchmark item does not match approved command, source, time or exit".to_string());
        }
        Ok(AdaptiveNativeBenchmarkMeasurement {
            threads,
            item_id: id.clone(),
            process_id: pid.to_string(),
            duration_ms: duration,
            completed_at_ms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptive_worker::validated_assignment_preflight;
    use codex_config::config_toml::AdaptiveOwnerBenchmarkToml;
    use codex_config::config_toml::AdaptiveWorkerRoleToml;
    use std::process::Command;
    use std::time::Instant;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    fn clock_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("test clock")
            .as_millis() as u64
    }

    fn fixture() -> (
        tempfile::TempDir,
        AdaptiveWorkerConfigToml,
        AdaptiveWorkerContext,
        AdaptiveWorkerPreflight,
    ) {
        let root = tempfile::tempdir().expect("disposable owner root");
        let git = |args: &[&str]| {
            let output = Command::new("git")
                .arg("-C")
                .arg(root.path())
                .args(args)
                .output()
                .expect("Git available");
            assert!(output.status.success(), "{:?}", output);
            String::from_utf8(output.stdout).expect("Git stdout")
        };
        git(&["init", "-q"]);
        git(&[
            "-c",
            "user.name=CodexDD test",
            "-c",
            "user.email=codexdd@example.test",
            "commit",
            "--allow-empty",
            "-q",
            "-m",
            "benchmark source",
        ]);
        let sha = git(&["rev-parse", "HEAD"]).trim().to_string();
        let root_text = root.path().to_string_lossy().to_string();
        let binding = AdaptiveWorkerConfigToml {
            role: AdaptiveWorkerRoleToml::Implementation,
            authorized_scope: Some("owner-approved D01 isolated benchmark".to_string()),
            source_root: Some(root_text.clone()),
            source_sha: Some(sha.clone()),
            task_kind: Some("benchmark".to_string()),
            required_gates: None,
            benchmark: Some(AdaptiveOwnerBenchmarkToml {
                command_2: "pwsh -NoProfile -NonInteractive -Command '1..2 | ForEach-Object -Parallel { Start-Sleep -Milliseconds 25 } -ThrottleLimit 2'".to_string(),
                command_4: "pwsh -NoProfile -NonInteractive -Command '1..4 | ForEach-Object -Parallel { Start-Sleep -Milliseconds 25 } -ThrottleLimit 4'".to_string(),
                resource_lock: std::env::temp_dir()
                    .join(format!("codexdd-046-3d1c-{}.lock", uuid::Uuid::new_v4()))
                    .to_string_lossy()
                    .to_string(),
                max_duration_ms: 20_000,
            }),
        };
        let worker = AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Implementation,
            authorized_scope: binding.authorized_scope.clone(),
        };
        let pin = validated_assignment_preflight(
            binding.task_kind.as_deref(),
            binding.source_root.as_deref(),
            binding.source_sha.as_deref(),
        )
        .expect("valid pin")
        .expect("bound");
        (root, binding, worker, pin)
    }

    fn real_command(
        command: &str,
        script: &str,
        thread: ThreadId,
        turn: &str,
        cwd: &Path,
        id: &str,
    ) -> ItemCompletedNotification {
        let start = Instant::now();
        let mut child = Command::new("pwsh")
            .args(["-NoProfile", "-NonInteractive", "-Command", script])
            .current_dir(cwd)
            .spawn()
            .expect("disposable Windows benchmark child");
        let pid = child.id().to_string();
        let status = child.wait().expect("reap benchmark child");
        assert!(status.success(), "real benchmark command failed");
        let duration = i64::try_from(start.elapsed().as_millis()).expect("bounded duration");
        let item = serde_json::from_value(serde_json::json!({
            "type": "commandExecution",
            "id": id,
            "command": command,
            "cwd": cwd.to_string_lossy(),
            "processId": pid,
            "source": "agent",
            "status": "completed",
            "commandActions": [],
            "aggregatedOutput": null,
            "exitCode": 0,
            "durationMs": duration,
        }))
        .expect("actual OS result represented as an app-server item");
        ItemCompletedNotification {
            item,
            thread_id: thread.to_string(),
            turn_id: turn.to_string(),
            completed_at_ms: clock_ms() as i64,
        }
    }

    #[cfg(windows)]
    #[test]
    fn codexdd_046_3d1c_real_two_and_four_os_processes_are_measured_under_lock() {
        let (root, config, worker, pin) = fixture();
        let mut approval = AdaptiveApprovedBenchmark::from_original_owner(
            &config,
            &worker,
            Some(&pin),
            root.path(),
        )
        .expect("original approved benchmark");
        let lease = approval
            .reserve_resource_lock()
            .expect("exclusive fixture lock");
        assert!(
            approval.reserve_resource_lock().is_err(),
            "another benchmark must not reuse the live lock"
        );
        let thread = ThreadId::new();
        // The measured owner session must start at or after admission, never before it.
        std::thread::sleep(std::time::Duration::from_millis(10));
        let start = clock_ms();
        assert!(start > lease.acquired_at_ms && start >= approval.bound_at_ms);
        let plan = config.benchmark.as_ref().expect("benchmark");
        // The original first-message parser must retain exact typed commands
        // and the lock; assignment prose cannot supply or upgrade them.
        let toml_string = |value: &str| format!("{value:?}");
        let header = format!(
            "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"owner-approved D01 isolated benchmark\"\nsource_root = {}\nsource_sha = \"{}\"\ntask_kind = \"benchmark\"\nbenchmark = {{ command_2 = {}, command_4 = {}, resource_lock = {}, max_duration_ms = 20000 }}\n\nRun only the authorized benchmark.",
            toml_string(&pin.source_root),
            pin.source_sha,
            toml_string(&plan.command_2),
            toml_string(&plan.command_4),
            toml_string(&plan.resource_lock),
        );
        let parsed = codex_config::config_toml::parse_adaptive_worker_first_message(&header)
            .expect("typed original header")
            .expect("first-owner assignment");
        assert_eq!(parsed.benchmark, config.benchmark);
        let two = real_command(
            &plan.command_2,
            "1..2 | ForEach-Object -Parallel { Start-Sleep -Milliseconds 25 } -ThrottleLimit 2",
            thread,
            "owner-turn",
            root.path(),
            "native-bench-2",
        );
        let four = real_command(
            &plan.command_4,
            "1..4 | ForEach-Object -Parallel { Start-Sleep -Milliseconds 25 } -ThrottleLimit 4",
            thread,
            "owner-turn",
            root.path(),
            "native-bench-4",
        );
        // This test makes native-shaped start records from actual OS-process
        // completions; only a future trusted Core event-stream integration may
        // prove that the start events themselves were authentically observed.
        let make_fixture_start = |done: &ItemCompletedNotification| {
            let mut item = done.item.clone();
            if let ThreadItem::CommandExecution {
                status,
                process_id,
                exit_code,
                duration_ms,
                ..
            } = &mut item
            {
                *status = CommandExecutionStatus::InProgress;
                *process_id = None;
                *exit_code = None;
                *duration_ms = None;
            }
            ItemStartedNotification {
                item,
                thread_id: done.thread_id.clone(),
                turn_id: done.turn_id.clone(),
                started_at_ms: i64::try_from(start).expect("bounded test timestamp"),
            }
        };
        let two_start = make_fixture_start(&two);
        let mut four_start = make_fixture_start(&four);
        // This legacy fixture synthesizes start shapes from real completed
        // OS runs. Reflect the actually sequential execution order; only the
        // embedded Core stream integration test authenticates real starts.
        four_start.started_at_ms = two.completed_at_ms;
        // Core exec_command emits UnifiedExecStartup for its initial process,
        // not Agent. Its native source must remain checked against the matching
        // completion; a user-shell or cross-source event is never D01 evidence.
        let mut core_start = two_start.clone();
        let mut core_done = two.clone();
        if let ThreadItem::CommandExecution { source, .. } = &mut core_start.item {
            *source = CommandExecutionSource::UnifiedExecStartup;
        }
        if let ThreadItem::CommandExecution { source, .. } = &mut core_done.item {
            *source = CommandExecutionSource::UnifiedExecStartup;
        }
        approval
            .match_native_start(
                Some(&core_start),
                Some(&core_done),
                &plan.command_2,
                thread,
                "owner-turn",
                start,
                &lease,
            )
            .expect("real Core unified-exec startup source matches its completion");
        approval
            .match_measurement(
                Some(&core_done),
                &plan.command_2,
                2,
                thread,
                "owner-turn",
                start,
                clock_ms(),
            )
            .expect("native Core unified-exec completion is an eligible measurement");
        let mut foreign_done = core_done.clone();
        if let ThreadItem::CommandExecution { source, .. } = &mut foreign_done.item {
            *source = CommandExecutionSource::UserShell;
        }
        assert!(
            approval
                .match_native_start(
                    Some(&core_start),
                    Some(&foreign_done),
                    &plan.command_2,
                    thread,
                    "owner-turn",
                    start,
                    &lease,
                )
                .is_err(),
            "a different command source cannot borrow genuine Core-start evidence"
        );
        if let ThreadItem::CommandExecution { source, .. } = &mut core_start.item {
            *source = CommandExecutionSource::UserShell;
        }
        assert!(
            approval
                .match_native_start(
                    Some(&core_start),
                    Some(&foreign_done),
                    &plan.command_2,
                    thread,
                    "owner-turn",
                    start,
                    &lease,
                )
                .is_err(),
            "user-shell events must not gain benchmark authority"
        );
        let mut foreign_start = four_start.clone();
        foreign_start.turn_id = "foreign-turn".to_string();
        assert!(
            approval
                .admit_native_lifecycles(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    start,
                    clock_ms(),
                    Some(&two_start),
                    Some(&two),
                    Some(&foreign_start),
                    Some(&four),
                )
                .expect_err("foreign start cannot authorize an otherwise valid completion")
                .contains("native benchmark start does not match")
        );
        let mut stale_start = four_start.clone();
        stale_start.started_at_ms =
            i64::try_from(lease.acquired_at_ms).expect("bounded lock timestamp");
        assert!(
            approval
                .admit_native_lifecycles(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    start,
                    clock_ms(),
                    Some(&two_start),
                    Some(&two),
                    Some(&stale_start),
                    Some(&four),
                )
                .expect_err("pre-lock start is not authorized by post-hoc lock")
                .contains("native benchmark start does not match")
        );
        assert!(
            approval
                .admit_native_lifecycles(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    start,
                    clock_ms(),
                    None,
                    Some(&two),
                    Some(&four_start),
                    Some(&four),
                )
                .expect_err("completion-only evidence must not bypass native starts")
                .contains("lacks a native command-start")
        );
        let mut wrong_start_id = four_start.clone();
        if let ThreadItem::CommandExecution { id, .. } = &mut wrong_start_id.item {
            *id = "unrelated-command-start".to_string();
        }
        assert!(
            approval
                .admit_native_lifecycles(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    start,
                    clock_ms(),
                    Some(&two_start),
                    Some(&two),
                    Some(&wrong_start_id),
                    Some(&four),
                )
                .expect_err("a different native item ID cannot satisfy the completion")
                .contains("native benchmark start does not match")
        );
        let mut overlapping_start = four_start.clone();
        overlapping_start.started_at_ms = two_start.started_at_ms;
        assert!(
            approval
                .admit_native_lifecycles(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    start,
                    clock_ms(),
                    Some(&two_start),
                    Some(&two),
                    Some(&overlapping_start),
                    Some(&four),
                )
                .expect_err("overlapping measurements cannot compare 2 against 4")
                .contains("out of required 2-then-4 order")
        );
        let matched = approval
            .admit_native_lifecycles(
                &worker,
                Some(&pin),
                root.path(),
                Some(&lease),
                thread,
                "owner-turn",
                start,
                clock_ms(),
                Some(&two_start),
                Some(&two),
                Some(&four_start),
                Some(&four),
            )
            .expect("matched native lifecycle records for two real OS processes");
        assert_eq!((matched.two.threads, matched.four.threads), (2, 4));
        assert_ne!(matched.two.process_id, matched.four.process_id);
        assert_eq!(matched.source_sha, pin.source_sha);
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    start,
                    clock_ms(),
                    Some(&two),
                    Some(&four),
                )
                .expect_err("same owned session cannot replay a completed pair")
                .contains("already consumed")
        );
        drop(lease);
        assert!(!Path::new(&plan.resource_lock).exists());
    }

    #[cfg(windows)]
    #[test]
    fn codexdd_046_3d1c_missing_owner_lock_source_or_real_receipt_refuses() {
        let (root, config, worker, pin) = fixture();
        let mut approval = AdaptiveApprovedBenchmark::from_original_owner(
            &config,
            &worker,
            Some(&pin),
            root.path(),
        )
        .expect("owner plan");
        let thread = ThreadId::new();
        let pre_lock_start = clock_ms().max(approval.bound_at_ms);
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    None,
                    thread,
                    "owner-turn",
                    pre_lock_start,
                    clock_ms(),
                    None,
                    None,
                )
                .expect_err("no lock")
                .contains("resource lock")
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
        let lease = approval.reserve_resource_lock().expect("scratch lock");
        assert!(pre_lock_start < lease.acquired_at_ms);
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    pre_lock_start,
                    clock_ms(),
                    None,
                    None,
                )
                .expect_err("post-hoc resource lock cannot justify pre-lock execution")
                .contains("session is stale or unbound")
        );
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    lease.acquired_at_ms,
                    clock_ms(),
                    None,
                    None,
                )
                .expect_err("same-millisecond start is not ordered after lock acquisition")
                .contains("session is stale or unbound")
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
        let started = clock_ms();
        assert!(started > lease.acquired_at_ms);
        // A live file at the expected pathname is not enough: its locked
        // handle must identify the same OS file, not an unrelated scratch file.
        // Keep the foreign fixture outside the pinned Git checkout so the
        // refusal exercises the held-handle identity check, not source dirt.
        let foreign_path =
            std::env::temp_dir().join(format!("codexdd-046-3d1c-foreign-{}", uuid::Uuid::new_v4()));
        std::fs::write(&foreign_path, b"not the approved lock").expect("foreign fixture");
        let mut impostor = AdaptiveBenchmarkLock {
            path: lease.path.clone(),
            file: Some(
                OpenOptions::new()
                    .read(true)
                    .open(&foreign_path)
                    .expect("foreign open handle"),
            ),
            acquired_at_ms: lease.acquired_at_ms,
        };
        assert!(!impostor.owns_original_path());
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&impostor),
                    thread,
                    "owner-turn",
                    started,
                    clock_ms(),
                    None,
                    None,
                )
                .expect_err("foreign handle must not authorize the original scratch lock")
                .contains("resource lock")
        );
        // Restore the test-only handle's own path before Drop so it cannot
        // remove the genuine owner's lock.
        impostor.path = foreign_path;
        drop(impostor);
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    approval.bound_at_ms.saturating_sub(1),
                    clock_ms(),
                    None,
                    None,
                )
                .expect_err("benchmark begun before original owner authorization")
                .contains("session is stale or unbound")
        );
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    started,
                    clock_ms(),
                    None,
                    None,
                )
                .expect_err("missing measurements")
                .contains("both genuine native")
        );
        let mut wrong = pin.clone();
        wrong.source_sha = "f".repeat(40);
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&wrong),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    started,
                    clock_ms(),
                    None,
                    None,
                )
                .is_err()
        );
        let missing_owner = AdaptiveWorkerConfigToml {
            benchmark: None,
            ..config.clone()
        };
        assert!(
            AdaptiveApprovedBenchmark::from_original_owner(
                &missing_owner,
                &worker,
                Some(&pin),
                root.path()
            )
            .expect_err("no original authority")
            .contains("original owner")
        );
        let plan = config.benchmark.as_ref().expect("plan");
        let two = real_command(
            &plan.command_2,
            "1..2 | ForEach-Object -Parallel { Start-Sleep -Milliseconds 25 } -ThrottleLimit 2",
            thread,
            "owner-turn",
            root.path(),
            "native-2",
        );
        let four = real_command(
            &plan.command_4,
            "1..4 | ForEach-Object -Parallel { Start-Sleep -Milliseconds 25 } -ThrottleLimit 4",
            thread,
            "owner-turn",
            root.path(),
            "native-4",
        );
        let mut changed = four.clone();
        if let ThreadItem::CommandExecution {
            ref mut command, ..
        } = changed.item
        {
            *command = "unapproved command".to_string();
        }
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    started,
                    clock_ms(),
                    Some(&two),
                    Some(&changed),
                )
                .is_err()
        );
        let mut nonnumeric = four.clone();
        if let ThreadItem::CommandExecution { process_id, .. } = &mut nonnumeric.item {
            *process_id = Some("pid-from-untrusted-prose".to_string());
        }
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    started,
                    clock_ms(),
                    Some(&two),
                    Some(&nonnumeric),
                )
                .expect_err("arbitrary process text is not a native PID")
                .contains("decimal OS process")
        );
        // A correctly shaped completion from another App thread or turn
        // must not be used as evidence for this owner's measured session.
        let mut foreign_thread = four.clone();
        foreign_thread.thread_id = ThreadId::new().to_string();
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    started,
                    clock_ms(),
                    Some(&two),
                    Some(&foreign_thread),
                )
                .expect_err("foreign-thread event cannot satisfy D01")
                .contains("native benchmark item does not match")
        );
        let mut foreign_turn = four.clone();
        foreign_turn.turn_id = "foreign-turn".to_string();
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    started,
                    clock_ms(),
                    Some(&two),
                    Some(&foreign_turn),
                )
                .expect_err("foreign-turn event cannot satisfy D01")
                .contains("native benchmark item does not match")
        );
        let mut earlier_completion = four.clone();
        earlier_completion.completed_at_ms =
            i64::try_from(started.saturating_sub(1)).expect("bounded test clock");
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    started,
                    clock_ms(),
                    Some(&two),
                    Some(&earlier_completion),
                )
                .expect_err("pre-authorization completion cannot satisfy D01")
                .contains("native benchmark item does not match")
        );
        let mut duplicate = four;
        if let (
            ThreadItem::CommandExecution { process_id: p2, .. },
            ThreadItem::CommandExecution { process_id: p4, .. },
        ) = (&two.item, &mut duplicate.item)
        {
            *p4 = p2.clone();
        }
        assert!(
            approval
                .admit_native_completions(
                    &worker,
                    Some(&pin),
                    root.path(),
                    Some(&lease),
                    thread,
                    "owner-turn",
                    started,
                    clock_ms(),
                    Some(&two),
                    Some(&duplicate),
                )
                .expect_err("foreign/reused PID")
                .contains("reuses")
        );

        // Even a handle opened at the correct scratch pathname cannot remove
        // that path if this owner never successfully acquired its OS lock.
        let unacquired_path = std::env::temp_dir().join(format!(
            "codexdd-046-3d1c-unacquired-{}",
            uuid::Uuid::new_v4()
        ));
        let unacquired_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&unacquired_path)
            .expect("disposable unacquired scratch file");
        drop(AdaptiveBenchmarkLock {
            path: unacquired_path.clone(),
            file: Some(unacquired_file),
            acquired_at_ms: 0,
        });
        assert!(
            unacquired_path.exists(),
            "failed or unacquired handle must not unlink another holder's scratch path"
        );
        std::fs::remove_file(&unacquired_path).expect("clean up isolated unacquired fixture");
    }
}
