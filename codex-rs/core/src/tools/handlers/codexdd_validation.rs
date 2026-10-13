use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::function_tool::FunctionCallError;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::ExecCommandHandler;
use crate::tools::handlers::ExecCommandHandlerOptions;
use crate::tools::handlers::resolve_tool_environment;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use codex_protocol::items::DynamicToolCallItem;
use codex_protocol::items::DynamicToolCallStatus;
use codex_protocol::items::TurnItem;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

const TOOL_NAME: &str = "run_codexdd_validation";
const CONTRACT_VERSION: u64 = 1;
const EVENT_PREFIX: &str = "CODEXDD_VALIDATION_JSON ";
const MAX_RETAINED_RUNS_PER_REPOSITORY: usize = 20;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ValidationProfile {
    Targeted,
    WorkPacket,
    Release,
}

impl ValidationProfile {
    fn tool_name(self) -> &'static str {
        match self {
            Self::Targeted => "targeted",
            Self::WorkPacket => "work_packet",
            Self::Release => "release",
        }
    }

    fn contract_name(self) -> &'static str {
        match self {
            Self::Targeted => "targeted",
            Self::WorkPacket => "work-packet",
            Self::Release => "release",
        }
    }

    fn script_name(self) -> &'static str {
        match self {
            Self::Targeted => "codexdd-test-targeted.ps1",
            Self::WorkPacket => "codexdd-test-workpacket.ps1",
            Self::Release => "codexdd-test-release.ps1",
        }
    }

    fn timeout_ms(self) -> u64 {
        match self {
            Self::Targeted => 15 * 60 * 1_000,
            Self::WorkPacket => 45 * 60 * 1_000,
            Self::Release => 2 * 60 * 60 * 1_000,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ValidationArgs {
    profile: ValidationProfile,
}

#[derive(Clone, Debug, Deserialize)]
struct ValidationEvent {
    contract_version: u64,
    event: String,
    profile: Option<String>,
    stage: Option<String>,
    status: Option<String>,
    exit_code: Option<i32>,
    native_exit_code: Option<i32>,
    duration_ms: Option<u64>,
    ordinal: Option<u64>,
    stage_count: Option<u64>,
    completed_stages: Option<u64>,
    failed_stage: Option<String>,
    error_type: Option<String>,
    message: Option<String>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct ValidationStageSummary {
    stage: String,
    status: String,
    native_exit_code: Option<i32>,
    duration_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct ValidationSummary {
    contract_version: u64,
    profile: String,
    status: String,
    exit_code: i32,
    stages: Vec<ValidationStageSummary>,
    failed_stage: Option<String>,
    error_type: Option<String>,
    message: Option<String>,
    log_path: String,
}

#[derive(Serialize)]
struct ValidationOperatorSummary<'a> {
    #[serde(flatten)]
    summary: &'a ValidationSummary,
    run_id: &'a str,
    branch: Option<&'a str>,
    head_sha: &'a str,
}

pub struct CodexDDValidationHandler;

impl ToolExecutor<ToolInvocation> for CodexDDValidationHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let properties = [(
            "profile".to_string(),
            JsonSchema::string_enum(
                vec![json!("targeted"), json!("work_packet"), json!("release")],
                Some(
                    "Deterministic repository-owned validation profile. Ordinary implementation mechanical validation uses work_packet; targeted is for focused retest; release is an explicit broad pre-CI/pre-install gate."
                        .to_string(),
                ),
            ),
        )]
        .into_iter()
        .collect();

        ToolSpec::Function(ResponsesApiTool {
            name: TOOL_NAME.to_string(),
            description: "Run one deterministic CodexDD local-validation profile. The tool accepts only a fixed profile selector, resolves the conventional repo-owned PowerShell script, executes it through Codex's existing command/sandbox path, stores the full log outside the source worktree, and returns a compact structured PASS/FAIL/ERROR summary. It does not accept arbitrary shell commands."
                .to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["profile".to_string()]),
                Some(false.into()),
            ),
            output_schema: None,
        })
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(async move { self.handle_call(invocation).await })
    }
}

impl CodexDDValidationHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let ToolPayload::Function { arguments } = &invocation.payload else {
            return Err(FunctionCallError::RespondToModel(
                "run_codexdd_validation received unsupported payload".to_string(),
            ));
        };
        let args: ValidationArgs = serde_json::from_str(arguments).map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to parse run_codexdd_validation arguments: {err}"
            ))
        })?;

        let turn_environment = resolve_tool_environment(
            &invocation.step_context,
            None,
            "CodexDD validation requires a local execution environment",
        )?;
        if turn_environment.environment.is_remote() {
            return Err(FunctionCallError::RespondToModel(
                "CodexDD 0.4.0 local validation is Windows-local only".to_string(),
            ));
        }

        let cwd = turn_environment
            .cwd()
            .to_abs_path()
            .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
        let repository_root = find_active_repository_root(cwd.as_path())
            .map_err(FunctionCallError::RespondToModel)?;
        // Keep preflight errors as a typed, non-PASS LVO tool completion. A generic
        // tool-call exception would disappear from the native receipt registry,
        // leaving an unfinished Mechanical Validation Worker in retry pressure.
        let profile_error = validate_repository_profile(&repository_root, args.profile).err();
        let script_path = repository_root
            .join("scripts")
            .join(args.profile.script_name());

        let git_info = codex_git_utils::collect_git_info(&repository_root)
            .await
            .ok_or_else(|| {
                FunctionCallError::RespondToModel(
                    "CodexDD validation could not resolve Git branch/HEAD metadata for the active repository"
                        .to_string(),
                )
            })?;
        let head_sha = git_info.commit_hash.map(|sha| sha.0).ok_or_else(|| {
            FunctionCallError::RespondToModel(
                "CodexDD validation requires a resolvable Git HEAD commit".to_string(),
            )
        })?;
        let branch = git_info.branch;

        let run_id = validation_run_id(&invocation.call_id);
        if let Some(message) = profile_error {
            // No repository validation script is executed, and an ERROR can never
            // become a successful work-packet receipt or repair fingerprint.
            let codex_home = invocation.turn.config.codex_home.to_path_buf();
            let repo_id = repository_log_id(&repository_root);
            let final_log_path =
                persist_validation_log(&codex_home, &repo_id, &run_id, message.as_bytes()).await?;
            let summary = ValidationSummary {
                contract_version: CONTRACT_VERSION,
                profile: args.profile.contract_name().to_string(),
                status: "error".to_string(),
                exit_code: 2,
                stages: Vec::new(),
                failed_stage: None,
                error_type: Some("profile_unavailable".to_string()),
                message: Some(message),
                log_path: final_log_path.display().to_string(),
            };
            return emit_validation_result(
                &invocation,
                args.profile,
                &run_id,
                &branch,
                &head_sha,
                &summary,
            )
            .await;
        }

        let temp_log_path = std::env::temp_dir().join(format!("codexdd-validation-{run_id}.log"));
        let temp_exit_path = std::env::temp_dir().join(format!("codexdd-validation-{run_id}.exit"));
        for temporary_path in [&temp_log_path, &temp_exit_path] {
            if let Err(err) = tokio::fs::remove_file(temporary_path).await
                && err.kind() != std::io::ErrorKind::NotFound
            {
                return Err(FunctionCallError::RespondToModel(format!(
                    "failed to prepare CodexDD validation temp file {}: {err}",
                    temporary_path.display()
                )));
            }
        }

        let command = validation_powershell_command(
            &repository_root,
            &script_path,
            &temp_log_path,
            &temp_exit_path,
        );
        let inner_arguments = json!({
            "cmd": command,
            "shell": "powershell.exe",
            "login": false,
            "tty": false,
            "yield_time_ms": 30_000,
            "timeout_ms": args.profile.timeout_ms(),
            "max_output_tokens": 1_000
        })
        .to_string();

        let mut delegated = invocation.clone();
        delegated.call_id = format!("{}:exec", invocation.call_id);
        delegated.tool_name = ToolName::plain("exec_command");
        delegated.payload = ToolPayload::Function {
            arguments: inner_arguments,
        };

        let exec_handler = ExecCommandHandler::one_shot(ExecCommandHandlerOptions {
            include_login_parameter: false,
            allow_tty: false,
            exec_permission_approvals_enabled: invocation
                .session
                .features()
                .enabled(codex_features::Feature::ExecPermissionApprovals),
            include_environment_id: false,
            include_shell_parameter: true,
            include_windows_shell_guidance: true,
        });
        let execution_result = exec_handler.handle(delegated).await;

        let log_bytes = match tokio::fs::read(&temp_log_path).await {
            Ok(bytes) => bytes,
            Err(err) => {
                let _ = tokio::fs::remove_file(&temp_exit_path).await;
                let execution_note = execution_result
                    .as_ref()
                    .map(ToolOutput::log_output)
                    .unwrap_or_else(std::string::ToString::to_string);
                let message = format!(
                    "CodexDD validation did not produce its required full log {}: {err}. Execution detail: {execution_note}",
                    temp_log_path.display()
                );
                // Missing native logs are infrastructure errors, not a reason to
                // retry or reclassify model capability. Capture the diagnostic
                // out-of-tree and emit an explicitly unsuccessful tool result.
                let codex_home = invocation.turn.config.codex_home.to_path_buf();
                let repo_id = repository_log_id(&repository_root);
                let final_log_path =
                    persist_validation_log(&codex_home, &repo_id, &run_id, message.as_bytes())
                        .await?;
                let summary = ValidationSummary {
                    contract_version: CONTRACT_VERSION,
                    profile: args.profile.contract_name().to_string(),
                    status: "error".to_string(),
                    exit_code: 2,
                    stages: Vec::new(),
                    failed_stage: None,
                    error_type: Some("missing_native_log".to_string()),
                    message: Some(message),
                    log_path: final_log_path.display().to_string(),
                };
                return emit_validation_result(
                    &invocation,
                    args.profile,
                    &run_id,
                    &branch,
                    &head_sha,
                    &summary,
                )
                .await;
            }
        };
        // Invalid native output must yield a typed ERROR receipt instead of
        // abandoning Mechanical Validation without a completed tool record.
        let log_text = String::from_utf8_lossy(&log_bytes).into_owned();
        let native_exit_code = tokio::fs::read_to_string(&temp_exit_path)
            .await
            .ok()
            .and_then(|code| code.trim().parse::<i32>().ok());
        let _ = tokio::fs::remove_file(&temp_exit_path).await;

        let codex_home = invocation.turn.config.codex_home.to_path_buf();
        let repo_id = repository_log_id(&repository_root);
        let final_log_path =
            persist_validation_log(&codex_home, &repo_id, &run_id, &log_bytes).await?;
        let _ = tokio::fs::remove_file(&temp_log_path).await;

        let mut summary = parse_validation_summary(args.profile, &log_text, &final_log_path)
            .unwrap_or_else(|message| ValidationSummary {
                contract_version: CONTRACT_VERSION,
                profile: args.profile.contract_name().to_string(),
                status: "error".to_string(),
                exit_code: 2,
                stages: Vec::new(),
                failed_stage: None,
                error_type: Some("invalid_profile_output".to_string()),
                message: Some(message),
                log_path: final_log_path.display().to_string(),
            });

        if let Err(err) = execution_result {
            summary.status = "error".to_string();
            summary.exit_code = 2;
            summary.error_type = Some("execution_error".to_string());
            summary.message = Some(err.to_string());
        } else {
            enforce_native_exit_consistency(&mut summary, native_exit_code);
        }

        emit_validation_result(
            &invocation,
            args.profile,
            &run_id,
            &branch,
            &head_sha,
            &summary,
        )
        .await
    }
}

/// A repo-owned script is not allowed to fabricate a PASS terminal while
/// exiting nonzero (or without completing the child process). The wrapper
/// independently captures the PowerShell child's exit code outside the log.
fn enforce_native_exit_consistency(summary: &mut ValidationSummary, observed: Option<i32>) {
    if observed == Some(summary.exit_code) {
        return;
    }
    let declared = summary.exit_code;
    summary.status = "error".to_string();
    summary.exit_code = 2;
    summary.error_type = Some("native_exit_mismatch".to_string());
    summary.message = Some(format!(
        "Validation script exit code did not match its contract-v1 terminal: declared {declared}, observed {observed:?}. No PASS or repair fingerprint is authorized."
    ));
    summary.failed_stage = None;
}

/// Emits a typed completed control-tool result for both native executions and
/// preflight-only profile errors. Errors are evidence of a blocker, never PASS.
async fn emit_validation_result(
    invocation: &ToolInvocation,
    profile: ValidationProfile,
    run_id: &str,
    branch: &Option<String>,
    head_sha: &str,
    summary: &ValidationSummary,
) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
    let success = summary.status == "pass" && summary.exit_code == 0;
    let failure_fingerprint = validation_failure_fingerprint(profile, summary);
    let receipt_arguments = json!({
        "profile": profile.tool_name(),
        "result": summary.status.clone(),
        "run_id": run_id,
        "branch": branch.clone(),
        "head_sha": head_sha,
        "log_path": summary.log_path.clone(),
        "failure_fingerprint": failure_fingerprint,
        "failed_stage": summary.failed_stage.clone(),
    });
    let receipt_started = TurnItem::DynamicToolCall(DynamicToolCallItem {
        id: invocation.call_id.clone(),
        namespace: None,
        tool: TOOL_NAME.to_string(),
        arguments: receipt_arguments.clone(),
        status: DynamicToolCallStatus::InProgress,
        content_items: None,
        success: None,
        error: None,
        duration: None,
    });
    invocation
        .session
        .emit_turn_item_started(invocation.turn.as_ref(), &receipt_started)
        .await;
    invocation
        .session
        .emit_turn_item_completed(
            invocation.turn.as_ref(),
            TurnItem::DynamicToolCall(DynamicToolCallItem {
                id: invocation.call_id.clone(),
                namespace: None,
                tool: TOOL_NAME.to_string(),
                arguments: receipt_arguments,
                status: DynamicToolCallStatus::Completed,
                content_items: None,
                success: Some(success),
                error: (!success).then(|| {
                    summary
                        .message
                        .clone()
                        .or_else(|| {
                            summary
                                .failed_stage
                                .as_ref()
                                .map(|stage| format!("validation failed at {stage}"))
                        })
                        .unwrap_or_else(|| "validation did not pass".to_string())
                }),
                duration: None,
            }),
        )
        .await;
    let operator_summary = ValidationOperatorSummary {
        summary,
        run_id,
        branch: branch.as_deref(),
        head_sha,
    };
    let text = serde_json::to_string(&operator_summary).map_err(|err| {
        FunctionCallError::RespondToModel(format!(
            "failed to serialize CodexDD validation summary: {err}"
        ))
    })?;
    Ok(boxed_tool_output(
        FunctionToolOutput::from_text(text, Some(success))
            .with_evidence_id(invocation.call_id.clone()),
    ))
}

impl CoreToolRuntime for CodexDDValidationHandler {
    fn is_builtin_control_tool(&self) -> bool {
        true
    }
}

fn validation_failure_fingerprint(
    profile: ValidationProfile,
    summary: &ValidationSummary,
) -> Option<String> {
    if summary.status != "fail" || summary.exit_code != 1 {
        return None;
    }
    let failed_stage = summary.failed_stage.as_deref()?;
    Some(format!("v1:{}:{failed_stage}", profile.tool_name()))
}

/// Resolve a validation profile only inside the *active* Git worktree.
///
/// A worktree can contain a .git directory or a worktree .git file. In particular,
/// the nearest Git root takes precedence over any ancestor: a missing profile in
/// a Node/TypeScript worktree must not fall back to another repository's scripts.
/// The profile contract is language-neutral, but always explicit and repo-owned.
fn find_active_repository_root(cwd: &Path) -> Result<PathBuf, String> {
    cwd.ancestors()
        .find(|candidate| {
            let git = candidate.join(".git");
            git.is_dir() || git.is_file()
        })
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            "CodexDD validation requires a Git worktree; no .git root was found above the active working directory. No validation was run."
                .to_string()
        })
}

fn validate_repository_profile(
    repository_root: &Path,
    profile: ValidationProfile,
) -> Result<(), String> {
    let script_path = repository_root.join("scripts").join(profile.script_name());
    if !script_path.is_file() {
        return Err(format!(
            "CodexDD validation profile '{}' is not configured in the active repository {} (expected {}). No validation ran or PASS receipt was issued. Register the repository-owned contract-v1 PowerShell profile for this project; stop for operator configuration rather than retrying or escalating model effort.",
            profile.tool_name(),
            repository_root.display(),
            script_path.display(),
        ));
    }

    let canonical_root = repository_root.canonicalize().map_err(|err| {
        format!(
            "CodexDD could not resolve the active Git repository root {}: {err}",
            repository_root.display()
        )
    })?;
    let canonical_script = script_path.canonicalize().map_err(|err| {
        format!(
            "CodexDD could not resolve the configured validation profile {}: {err}",
            script_path.display()
        )
    })?;
    if !canonical_script.starts_with(&canonical_root) {
        return Err(format!(
            "CodexDD validation profile '{}' resolves outside the active Git repository {}; cross-repository script execution is forbidden.",
            profile.tool_name(),
            repository_root.display()
        ));
    }
    Ok(())
}

#[cfg(test)]
fn discover_repository_root(cwd: &Path, profile: ValidationProfile) -> Result<PathBuf, String> {
    let repository_root = find_active_repository_root(cwd)?;
    validate_repository_profile(&repository_root, profile)?;
    Ok(repository_root)
}

fn validation_powershell_command(
    repository_root: &Path,
    script_path: &Path,
    log_path: &Path,
    exit_path: &Path,
) -> String {
    let repository_root = powershell_single_quoted(repository_root);
    let script_path = powershell_single_quoted(script_path);
    let log_path = powershell_single_quoted(log_path);
    let exit_path = powershell_single_quoted(exit_path);
    format!(
        "$ErrorActionPreference = 'Stop'; Set-Location -LiteralPath {repository_root}; $log = {log_path}; $exitLog = {exit_path}; $utf8 = New-Object System.Text.UTF8Encoding($false); if (Test-Path -LiteralPath $log) {{ Remove-Item -LiteralPath $log -Force }}; $ErrorActionPreference = 'Continue'; & powershell.exe -NoProfile -ExecutionPolicy Bypass -File {script_path} 2>&1 | ForEach-Object {{ [System.IO.File]::AppendAllText($log, $_.ToString() + [Environment]::NewLine, $utf8) }}; $code = $LASTEXITCODE; if ($null -eq $code) {{ $code = 2 }}; [System.IO.File]::WriteAllText($exitLog, [string]$code, $utf8); exit $code"
    )
}

fn powershell_single_quoted(path: &Path) -> String {
    format!("'{}'", path.to_string_lossy().replace('\'', "''"))
}

fn validation_run_id(call_id: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("{timestamp}-{}", sanitize_path_component(call_id))
}

fn repository_log_id(repository_root: &Path) -> String {
    let name = repository_root
        .file_name()
        .and_then(|name| name.to_str())
        .map(sanitize_path_component)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "repository".to_string());
    let mut hasher = DefaultHasher::new();
    repository_root.hash(&mut hasher);
    format!("{name}-{:08x}", hasher.finish() as u32)
}

fn sanitize_path_component(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '_'
            }
        })
        .collect();
    if sanitized.is_empty() {
        "run".to_string()
    } else {
        sanitized
    }
}

async fn persist_validation_log(
    codex_home: &Path,
    repo_id: &str,
    run_id: &str,
    log_bytes: &[u8],
) -> Result<PathBuf, FunctionCallError> {
    let repository_log_root = codex_home.join("codexdd").join("validation").join(repo_id);
    let run_directory = repository_log_root.join(run_id);
    tokio::fs::create_dir_all(&run_directory)
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to create CodexDD validation log directory {}: {err}",
                run_directory.display()
            ))
        })?;

    let log_path = run_directory.join("validation.log");
    tokio::fs::write(&log_path, log_bytes)
        .await
        .map_err(|err| {
            FunctionCallError::RespondToModel(format!(
                "failed to persist CodexDD validation log {}: {err}",
                log_path.display()
            ))
        })?;
    prune_validation_logs(&repository_log_root).await;
    Ok(log_path)
}

async fn prune_validation_logs(repository_log_root: &Path) {
    let Ok(mut entries) = tokio::fs::read_dir(repository_log_root).await else {
        return;
    };
    let mut directories = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        if entry
            .file_type()
            .await
            .is_ok_and(|file_type| file_type.is_dir())
        {
            directories.push(entry.path());
        }
    }
    directories.sort();
    let excess = directories
        .len()
        .saturating_sub(MAX_RETAINED_RUNS_PER_REPOSITORY);
    for directory in directories.into_iter().take(excess) {
        let _ = tokio::fs::remove_dir_all(directory).await;
    }
}

fn parse_validation_summary(
    profile: ValidationProfile,
    log: &str,
    log_path: &Path,
) -> Result<ValidationSummary, String> {
    let mut stages = Vec::new();
    let mut stage_begins = Vec::new();
    let mut profile_stage_count = None;
    let mut profile_end = None;
    let mut pending_stage: Option<(String, u64)> = None;

    for line in log.lines() {
        let Some(payload) = line.strip_prefix(EVENT_PREFIX) else {
            continue;
        };
        let event: ValidationEvent = serde_json::from_str(payload)
            .map_err(|err| format!("invalid validation event JSON: {err}"))?;
        if event.contract_version != CONTRACT_VERSION {
            return Err(format!(
                "unsupported validation contract version {}",
                event.contract_version
            ));
        }
        if event.profile.as_deref() != Some(profile.contract_name()) {
            return Err(format!(
                "validation event profile mismatch: expected {}",
                profile.contract_name()
            ));
        }
        if profile_end.is_some() {
            return Err("validation event emitted after profile_end".to_string());
        }

        match event.event.as_str() {
            "profile_begin" => {
                if profile_stage_count.is_some() {
                    return Err("duplicate profile_begin event".to_string());
                }
                profile_stage_count = Some(
                    event
                        .stage_count
                        .ok_or_else(|| "profile_begin event omitted stage_count".to_string())?,
                );
            }
            "stage_begin" => {
                if profile_stage_count.is_none() || pending_stage.is_some() {
                    return Err("stage_begin outside a valid validation stage".to_string());
                }
                let name = event
                    .stage
                    .ok_or_else(|| "stage_begin event omitted stage".to_string())?;
                let ordinal = event
                    .ordinal
                    .ok_or_else(|| "stage_begin event omitted ordinal".to_string())?;
                if ordinal != (stage_begins.len() as u64) + 1 {
                    return Err("stage_begin ordinal is not sequential".to_string());
                }
                stage_begins.push(name.clone());
                pending_stage = Some((name, ordinal));
            }
            "stage_end" => {
                let (expected_name, expected_ordinal) = pending_stage
                    .take()
                    .ok_or_else(|| "stage_end event has no matching stage_begin".to_string())?;
                let stage = event
                    .stage
                    .ok_or_else(|| "stage_end event omitted stage".to_string())?;
                if stage != expected_name || event.ordinal != Some(expected_ordinal) {
                    return Err("stage_end does not match its stage_begin".to_string());
                }
                let status = event
                    .status
                    .ok_or_else(|| "stage_end event omitted status".to_string())?;
                if !matches!(status.as_str(), "pass" | "fail" | "error") {
                    return Err(format!("unrecognized validation stage status {status}"));
                }
                if status == "pass" && event.native_exit_code != Some(0) {
                    return Err("PASS stage must have native exit code 0".to_string());
                }
                stages.push(ValidationStageSummary {
                    stage,
                    status,
                    native_exit_code: event.native_exit_code,
                    duration_ms: event.duration_ms,
                });
            }
            "profile_end" => {
                if profile_stage_count.is_none() {
                    return Err("profile_end without profile_begin".to_string());
                }
                if pending_stage.is_some() {
                    return Err("profile_end while a stage is incomplete".to_string());
                }
                profile_end = Some(event);
            }
            other => return Err(format!("unknown validation event '{other}'")),
        }
    }

    let profile_end = profile_end.ok_or_else(|| "profile_end event was not emitted".to_string())?;
    let status = profile_end
        .status
        .ok_or_else(|| "profile_end event omitted status".to_string())?;
    let exit_code = profile_end
        .exit_code
        .ok_or_else(|| "profile_end event omitted exit_code".to_string())?;
    let declared_count =
        profile_stage_count.ok_or_else(|| "profile_begin event was not emitted".to_string())?;
    if stage_begins.len() as u64 > declared_count {
        return Err("more stages began than the declared profile stage_count".to_string());
    }
    let completed_passes = stages.iter().filter(|stage| stage.status == "pass").count() as u64;
    if status == "pass" {
        if exit_code != 0
            || declared_count == 0
            || stages.len() as u64 != declared_count
            || stage_begins.len() as u64 != declared_count
            || completed_passes != declared_count
            || profile_end.completed_stages != Some(declared_count)
        {
            return Err("profile PASS did not match all declared, successful stages".to_string());
        }
    } else if status == "fail" {
        if exit_code != 1
            || stages.last().is_none_or(|stage| stage.status != "fail")
            || profile_end.failed_stage.as_deref()
                != stages.last().map(|stage| stage.stage.as_str())
            || profile_end.completed_stages != Some(completed_passes)
        {
            return Err("profile FAIL did not match its failed stage".to_string());
        }
    } else if status != "error" || exit_code != 2 {
        return Err(
            "validation profile returned an invalid terminal status or exit code".to_string(),
        );
    }

    Ok(ValidationSummary {
        contract_version: CONTRACT_VERSION,
        profile: profile.contract_name().to_string(),
        status,
        exit_code,
        stages,
        failed_stage: profile_end.failed_stage,
        error_type: profile_end.error_type,
        message: profile_end.message,
        log_path: log_path.display().to_string(),
    })
}

#[cfg(test)]
#[path = "codexdd_validation_tests.rs"]
mod tests;
