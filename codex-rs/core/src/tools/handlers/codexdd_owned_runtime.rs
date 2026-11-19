//! Core-owned D02 runtime health observation; not an Owner QA completion receipt.
//! The only model argument identifies an existing Core exec session. Original
//! argv, source and approved port are immutable owner config, never tool input.

use crate::function_tool::FunctionCallError;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::resolve_tool_environment;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_config::config_toml::AdaptiveWorkerRoleToml;
use codex_protocol::items::DynamicToolCallItem;
use codex_protocol::items::DynamicToolCallStatus;
use codex_protocol::items::TurnItem;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use serde_json::json;
use std::path::Path;
use uuid::Uuid;

const TOOL_NAME: &str = "run_codexdd_owned_runtime_health";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OwnedRuntimeArgs {
    session_id: i32,
}

struct ObservedCoreHealth {
    owner_pid: u32,
    port: u16,
    observed_at_ms: u64,
    root: String,
    sha: String,
}

pub struct CodexDDOwnedRuntimeHealthHandler;

/// Bind one exact executable argv and port only from the original owner gate.
/// An operations-preview gate without argv remains pending, never inferred.
pub(crate) fn original_owner_preview_gate(
    worker: &AdaptiveWorkerConfigToml,
) -> Result<(u16, &[String]), String> {
    let scope = worker.authorized_scope.as_deref().unwrap_or_default();
    let root = worker.source_root.as_deref().unwrap_or_default();
    let sha = worker.source_sha.as_deref().unwrap_or_default();
    if worker.role != AdaptiveWorkerRoleToml::Implementation
        || worker.task_kind.as_deref() != Some("operations_preview")
        || scope.trim().is_empty()
        || scope.len() > 1024
        || !Path::new(root).is_absolute()
        || sha.len() != 40
        || !sha.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(
            "SETUP_BLOCKED: runtime preview requires original Implementation scope and source"
                .to_string(),
        );
    }
    let required = worker.required_gates.as_deref().unwrap_or_default();
    if required
        .iter()
        .filter(|gate| gate.kind == "independent_validation")
        .count()
        != 1
        || required
            .iter()
            .any(|gate| gate.kind != "owned_runtime_health" && gate.command_argv.is_some())
    {
        return Err(
            "SETUP_BLOCKED: independent review or runtime command authority missing".to_string(),
        );
    }
    let mut runtime = required
        .iter()
        .filter(|gate| gate.kind == "owned_runtime_health");
    let gate = runtime
        .next()
        .ok_or_else(|| "SETUP_BLOCKED: original runtime-health gate missing".to_string())?;
    if runtime.next().is_some() || gate.profile.is_some() {
        return Err("SETUP_BLOCKED: ambiguous runtime-health gate".to_string());
    }
    let Some(ports) = gate.ports.as_ref() else {
        return Err("SETUP_BLOCKED: original health port missing".to_string());
    };
    if ports.len() != 1 || ports[0] == 0 {
        return Err("SETUP_BLOCKED: exactly one approved loopback port required".to_string());
    }
    let argv = gate.command_argv.as_deref().ok_or_else(|| {
        "SETUP_BLOCKED: original owner did not approve executable argv".to_string()
    })?;
    if argv.is_empty()
        || argv.len() > 32
        || argv.iter().any(|argument| {
            argument.is_empty() || argument.len() > 4096 || argument.chars().any(char::is_control)
        })
    {
        return Err("SETUP_BLOCKED: invalid original executable argv".to_string());
    }
    Ok((ports[0], argv))
}

impl ToolExecutor<ToolInvocation> for CodexDDOwnedRuntimeHealthHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let properties = [(
            "session_id".to_string(),
            JsonSchema::integer(Some(
                "Existing Core exec session ID; not an OS PID or process name.".to_string(),
            )),
        )]
        .into_iter()
        .collect();
        ToolSpec::Function(ResponsesApiTool {
            name: TOOL_NAME.to_string(),
            description: "Read-only Core check of an already-running, original-owner-approved Windows loopback preview. No new launch, process control, persistent receipt or READY authority.".to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["session_id".to_string()]),
                Some(false.into()),
            ),
            output_schema: None,
        })
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(async move {
            let ToolPayload::Function { arguments } = &invocation.payload else {
                return Err(FunctionCallError::RespondToModel(
                    "SETUP_BLOCKED: only fixed runtime-health arguments accepted".to_string(),
                ));
            };
            let args: OwnedRuntimeArgs = serde_json::from_str(arguments).map_err(|_| {
                FunctionCallError::RespondToModel(
                    "SETUP_BLOCKED: expected only existing Core session_id".to_string(),
                )
            })?;
            let result = observe(&invocation, args.session_id).await;
            emit(&invocation, args.session_id, result).await
        })
    }
}

impl CoreToolRuntime for CodexDDOwnedRuntimeHealthHandler {
    fn is_builtin_control_tool(&self) -> bool {
        true
    }
}

async fn observe(
    invocation: &ToolInvocation,
    session_id: i32,
) -> Result<ObservedCoreHealth, String> {
    if session_id <= 0 {
        return Err("SETUP_BLOCKED: invalid Core exec session identifier".to_string());
    }
    let worker = invocation
        .turn
        .config
        .adaptive_worker
        .as_ref()
        .ok_or_else(|| "SETUP_BLOCKED: original Worker authority missing".to_string())?;
    let (port, argv) = original_owner_preview_gate(worker)?;
    let root = Path::new(worker.source_root.as_deref().unwrap_or_default())
        .canonicalize()
        .map_err(|_| "SETUP_BLOCKED: owner worktree missing".to_string())?;
    let sha = worker.source_sha.as_deref().unwrap_or_default();
    let selected = resolve_tool_environment(
        &invocation.step_context,
        None,
        "CodexDD runtime health requires original local execution environment",
    )
    .map_err(|error| format!("SETUP_BLOCKED: {error}"))?;
    if selected.environment.is_remote() {
        return Err("SETUP_BLOCKED: remote execution is not a local Core handle".to_string());
    }
    let cwd = selected
        .cwd()
        .to_abs_path()
        .map_err(|_| "SETUP_BLOCKED: execution cwd is not local".to_string())?;
    let selected_root = cwd
        .as_path()
        .ancestors()
        .find(|path| path.join(".git").is_dir() || path.join(".git").is_file())
        .ok_or_else(|| "SETUP_BLOCKED: selected cwd lacks a Git worktree".to_string())?
        .canonicalize()
        .map_err(|_| "SETUP_BLOCKED: selected worktree unavailable".to_string())?;
    if selected_root != root {
        return Err("SETUP_BLOCKED: selected worktree differs from original root".to_string());
    }
    verify_head(&root, sha).await?;

    #[cfg(not(windows))]
    {
        let _ = (port, argv);
        Err("SETUP_BLOCKED: Windows native process observation unavailable".to_string())
    }
    #[cfg(windows)]
    {
        let observation = invocation
            .session
            .services
            .unified_exec_manager
            .observe_original_owner_approved_health(
                &invocation.session,
                session_id,
                argv,
                &root,
                port,
            )
            .await?;
        verify_head(&root, sha).await?;
        Ok(ObservedCoreHealth {
            owner_pid: observation.owner_pid,
            port: observation.approved_port,
            observed_at_ms: observation.observed_at_ms,
            root: root.display().to_string(),
            sha: sha.to_ascii_lowercase(),
        })
    }
}

async fn verify_head(root: &Path, sha: &str) -> Result<(), String> {
    let git = codex_git_utils::collect_git_info(root)
        .await
        .ok_or_else(|| "SETUP_BLOCKED: Git source identity unavailable".to_string())?;
    if git
        .commit_hash
        .as_ref()
        .is_none_or(|actual| !actual.0.eq_ignore_ascii_case(sha))
    {
        return Err("SETUP_BLOCKED: original owner source SHA drifted".to_string());
    }
    Ok(())
}

async fn emit(
    invocation: &ToolInvocation,
    session_id: i32,
    result: Result<ObservedCoreHealth, String>,
) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
    let success = result.is_ok();
    let run_id = Uuid::new_v4().to_string();
    let (arguments, message) = match result {
        Ok(health) => (
            json!({
                "result": "observation_only",
                "run_id": run_id,
                "source_root": health.root,
                "source_sha": health.sha,
                "session_id": session_id,
                "owner_pid": health.owner_pid,
                "approved_port": health.port,
                "observed_at_ms": health.observed_at_ms,
            }),
            format!(
                "Core observed original owner PID {}, loopback port {} and HTTP 200 at {} ms; native QA gate remains PENDING.",
                health.owner_pid, health.port, health.observed_at_ms
            ),
        ),
        Err(reason) => (
            json!({
                "result": "setup_blocked",
                "run_id": run_id,
                "session_id": session_id,
            }),
            reason,
        ),
    };
    let item = |status, success, error| {
        TurnItem::DynamicToolCall(DynamicToolCallItem {
            id: invocation.call_id.clone(),
            namespace: None,
            tool: TOOL_NAME.to_string(),
            arguments: arguments.clone(),
            status,
            content_items: None,
            success,
            error,
            duration: None,
        })
    };
    invocation
        .session
        .emit_turn_item_started(
            invocation.turn.as_ref(),
            &item(DynamicToolCallStatus::InProgress, None, None),
        )
        .await;
    invocation
        .session
        .emit_turn_item_completed(
            invocation.turn.as_ref(),
            item(
                DynamicToolCallStatus::Completed,
                Some(success),
                (!success).then(|| message.clone()),
            ),
        )
        .await;
    Ok(boxed_tool_output(
        FunctionToolOutput::from_text(message, Some(success))
            .with_evidence_id(invocation.call_id.clone()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_config::config_toml::AdaptiveOwnerGateToml;

    fn worker() -> AdaptiveWorkerConfigToml {
        AdaptiveWorkerConfigToml {
            role: AdaptiveWorkerRoleToml::Implementation,
            authorized_scope: Some("disposable preview".to_string()),
            source_root: Some(std::env::temp_dir().display().to_string()),
            source_sha: Some("1".repeat(40)),
            task_kind: Some("operations_preview".to_string()),
            required_gates: Some(vec![
                AdaptiveOwnerGateToml {
                    kind: "owned_runtime_health".to_string(),
                    profile: None,
                    ports: Some(vec![4331]),
                    command_argv: Some(vec![
                        "powershell.exe".to_string(),
                        "-NoProfile".to_string(),
                    ]),
                },
                AdaptiveOwnerGateToml {
                    kind: "independent_validation".to_string(),
                    profile: None,
                    ports: None,
                    command_argv: None,
                },
            ]),
            benchmark: None,
        }
    }

    #[test]
    fn codexdd_046_3d1e_original_owner_command_scope_and_independence_required() {
        let valid = worker();
        let (port, argv) = original_owner_preview_gate(&valid).unwrap();
        assert_eq!(port, 4331);
        assert_eq!(
            argv,
            &["powershell.exe".to_string(), "-NoProfile".to_string()]
        );
        let mut wrong = worker();
        wrong.role = AdaptiveWorkerRoleToml::Validation;
        assert!(original_owner_preview_gate(&wrong).is_err());
        wrong = worker();
        wrong.task_kind = Some("source_change".to_string());
        assert!(original_owner_preview_gate(&wrong).is_err());
        wrong = worker();
        wrong.required_gates.as_mut().unwrap().pop();
        assert!(original_owner_preview_gate(&wrong).is_err());
        wrong = worker();
        wrong.authorized_scope = None;
        assert!(original_owner_preview_gate(&wrong).is_err());
    }

    #[test]
    fn codexdd_046_3d1f_model_cannot_supply_health_pid_source_or_port() {
        let valid: OwnedRuntimeArgs =
            serde_json::from_str(r#"{"session_id":101}"#).expect("one bounded lookup");
        assert_eq!(valid.session_id, 101);
        for forged in [
            r#"{"session_id":101,"owner_pid":123}"#,
            r#"{"session_id":101,"approved_port":4331}"#,
            r#"{"session_id":101,"source_sha":"forged"}"#,
            r#"{"session_id":101,"command_argv":["arbitrary.exe"]}"#,
        ] {
            assert!(
                serde_json::from_str::<OwnedRuntimeArgs>(forged).is_err(),
                "model must never enlarge Core authority: {forged}"
            );
        }
    }

    #[test]
    fn codexdd_046_3d1e_model_cannot_enlarge_owned_port_or_command() {
        let mut wrong = worker();
        wrong.required_gates.as_mut().unwrap()[0].command_argv = None;
        assert!(original_owner_preview_gate(&wrong).is_err());
        wrong = worker();
        wrong.required_gates.as_mut().unwrap()[0].ports = Some(vec![4331, 4332]);
        assert!(original_owner_preview_gate(&wrong).is_err());
        wrong = worker();
        wrong.required_gates.as_mut().unwrap()[0].command_argv = Some(vec![
            "powershell.exe".to_string(),
            "Get-Date\nRestart-Service".to_string(),
        ]);
        assert!(original_owner_preview_gate(&wrong).is_err());
        wrong = worker();
        wrong.required_gates.as_mut().unwrap()[1].command_argv =
            Some(vec!["powershell.exe".to_string()]);
        assert!(original_owner_preview_gate(&wrong).is_err());
        wrong = worker();
        let duplicate = wrong.required_gates.as_ref().unwrap()[0].clone();
        wrong.required_gates.as_mut().unwrap().push(duplicate);
        assert!(original_owner_preview_gate(&wrong).is_err());
    }
}
