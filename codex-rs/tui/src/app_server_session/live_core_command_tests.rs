//! 0.4.6 3D.1c2b: a genuine agent-command lifecycle read from the local
//! embedded Core stream. This is an isolated, non-authoritative ingress test:
//! no benchmark admission, owner QA, or privileged process management.

use super::*;
use crate::adaptive_benchmark::AdaptiveApprovedBenchmark;
use crate::adaptive_worker::AdaptiveWorkerContext;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::validated_assignment_preflight;
use crate::legacy_core::config::ConfigBuilder;
use crate::legacy_core::config::ConfigOverrides;
use app_test_support::MockResponsesConfig;
use app_test_support::create_final_assistant_message_sse_response;
use app_test_support::create_mock_responses_server_sequence_unchecked;
use codex_app_server_protocol::CommandExecutionSource;
use codex_app_server_protocol::CommandExecutionStatus;
use codex_app_server_protocol::ServerNotification;
use codex_app_server_protocol::ThreadItem;
use codex_config::config_toml::AdaptiveOwnerBenchmarkToml;
use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_config::config_toml::AdaptiveWorkerRoleToml;
use core_test_support::responses;
use std::collections::BTreeMap;
use std::time::Duration;

#[tokio::test]
async fn codexdd_046_3d1c2b_real_embedded_core_exec_emits_authenticated_lifecycle() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("disposable-native-core-command");
    std::fs::create_dir(&root)?;
    let git = |args: &[&str]| -> Result<String> {
        let result = std::process::Command::new("git")
            .arg("-C")
            .arg(&root)
            .args(args)
            .output()?;
        color_eyre::eyre::ensure!(
            result.status.success(),
            "disposable Git fixture failed: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())
    };
    git(&["init", "-q"])?;
    git(&[
        "-c",
        "user.name=CodexDD native Core fixture",
        "-c",
        "user.email=codexdd@example.test",
        "commit",
        "--allow-empty",
        "-q",
        "-m",
        "disposable Core command stream source",
    ])?;
    let sha = git(&["rev-parse", "HEAD"])?;
    const COMMAND_2: &str = "pwsh -NoProfile -NonInteractive -Command '1..2 | ForEach-Object -Parallel { Start-Sleep -Milliseconds 25 } -ThrottleLimit 2; Write-Output CODEXDD_CORE_PROVENANCE_2'";
    const COMMAND_4: &str = "pwsh -NoProfile -NonInteractive -Command '1..4 | ForEach-Object -Parallel { Start-Sleep -Milliseconds 25 } -ThrottleLimit 4; Write-Output CODEXDD_CORE_PROVENANCE_4'";
    const CALL_2: &str = "core-command-call-2";
    const CALL_4: &str = "core-command-call-4";
    let original = AdaptiveWorkerConfigToml {
        role: AdaptiveWorkerRoleToml::Implementation,
        authorized_scope: Some("owner-approved disposable Core benchmark".to_string()),
        source_root: Some(root.to_string_lossy().to_string()),
        source_sha: Some(sha),
        task_kind: Some("benchmark".to_string()),
        required_gates: None,
        benchmark: Some(AdaptiveOwnerBenchmarkToml {
            command_2: COMMAND_2.to_string(),
            command_4: COMMAND_4.to_string(),
            resource_lock: std::env::temp_dir()
                .join(format!(
                    "codexdd-core-ingress-{}.lock",
                    uuid::Uuid::new_v4()
                ))
                .to_string_lossy()
                .to_string(),
            max_duration_ms: 30_000,
        }),
    };
    let worker = AdaptiveWorkerContext {
        role: AdaptiveWorkerRole::Implementation,
        authorized_scope: original.authorized_scope.clone(),
    };
    let pin = validated_assignment_preflight(
        original.task_kind.as_deref(),
        original.source_root.as_deref(),
        original.source_sha.as_deref(),
    )
    .map_err(|error| color_eyre::eyre::eyre!("{error}"))?
    .ok_or_else(|| color_eyre::eyre::eyre!("missing immutable owner source pin"))?;
    let approved =
        AdaptiveApprovedBenchmark::from_original_owner(&original, &worker, Some(&pin), &root)
            .map_err(|error| color_eyre::eyre::eyre!("{error}"))?;
    // Acquire before either Core command is dispatched; keep the actual OS
    // lock handle alive until the two source-bound Core lifecycles finish.
    let held_lock = approved
        .reserve_resource_lock()
        .map_err(|error| color_eyre::eyre::eyre!("{error}"))?;
    color_eyre::eyre::ensure!(
        approved.reserve_resource_lock().is_err(),
        "second contender acquired the original owner's exclusive scratch lock"
    );

    // Two sequential model tool calls, one actual Core turn and two distinct
    // child command executions. Neither a shaped fixture nor a generic PASS
    // can manufacture these server-origin item/started and item/completed pairs.
    let model_call = |response_id: &str, call_id: &str, command: &str| {
        responses::sse(vec![
            responses::ev_response_created(response_id),
            responses::ev_function_call(
                call_id,
                "exec_command",
                &serde_json::json!({
                    "cmd": command,
                    "yield_time_ms": 5000,
                    "max_output_tokens": 128,
                })
                .to_string(),
            ),
            responses::ev_completed(response_id),
        ])
    };
    let backend = create_mock_responses_server_sequence_unchecked(vec![
        model_call("core-response-2", CALL_2, COMMAND_2),
        model_call("core-response-4", CALL_4, COMMAND_4),
        create_final_assistant_message_sse_response("Both Core commands finished.")
            .map_err(|error| color_eyre::eyre::eyre!("{error:#}"))?,
    ])
    .await;
    let home = tempfile::tempdir()?;
    MockResponsesConfig::new(&backend.uri())
        .with_approval_policy("never")
        .with_sandbox_mode("danger-full-access")
        .write(home.path())?;
    let config = ConfigBuilder::default()
        .codex_home(home.path().to_path_buf())
        .harness_overrides(ConfigOverrides {
            cwd: Some(root.clone()),
            ..ConfigOverrides::default()
        })
        .build()
        .await?;
    let mut server = crate::start_embedded_app_server_for_picker(&config).await?;
    let started = server.start_thread(&config).await?;
    let thread_id = started.session.thread_id;
    let response = server
        .turn_start(
            thread_id,
            uuid::Uuid::new_v4().to_string(),
            vec![UserInput::Text {
                text: "Run only the original two approved disposable Core benchmark commands."
                    .to_string(),
                text_elements: Vec::new(),
            }],
            None,
            root.clone(),
            Some(AskForApproval::Never),
            None,
            TurnPermissionsOverride::Preserve,
            &[],
            config
                .model
                .clone()
                .unwrap_or_else(|| "mock-model".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    let turn_id = response.turn.id;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(90);
    let approved_calls = [(CALL_2, COMMAND_2), (CALL_4, COMMAND_4)];
    let mut started_items = BTreeMap::<String, String>::new();
    let mut completed_items = BTreeMap::<String, String>::new();
    let mut turn_completed = false;

    for _ in 0..400 {
        let observed = tokio::time::timeout_at(deadline, server.next_observed_event())
            .await?
            .ok_or_else(|| color_eyre::eyre::eyre!("local Core stream closed"))?;
        color_eyre::eyre::ensure!(
            observed.is_from_local_core_session(&server),
            "Core event lost its exact in-process origin"
        );
        match observed.event() {
            AppServerEvent::ServerNotification(notification) => match notification.as_ref() {
                ServerNotification::ItemStarted(item)
                    if observed.is_local_core_command_lifecycle(&server)
                        && item.thread_id == thread_id.to_string()
                        && item.turn_id == turn_id =>
                {
                    if let ThreadItem::CommandExecution {
                        id,
                        command,
                        source,
                        status,
                        ..
                    } = &item.item
                    {
                        // Startup/approval traffic can contain other real Core
                        // command items in the same turn. Only the literal mock
                        // tool-call ID is authoritative for this regression.
                        let Some((_, approved_command)) = approved_calls
                            .iter()
                            .find(|(call_id, _)| id.as_str() == *call_id)
                        else {
                            continue;
                        };
                        color_eyre::eyre::ensure!(
                            *source == CommandExecutionSource::UnifiedExecStartup
                                && *status == CommandExecutionStatus::InProgress
                                && command.contains(*approved_command),
                            "Core start mismatched original command: source={source:?}, status={status:?}, approved_command_present={}",
                            command.contains(*approved_command)
                        );
                        color_eyre::eyre::ensure!(
                            started_items.insert(id.clone(), command.clone()).is_none(),
                            "Core emitted duplicate command-start events"
                        );
                    }
                }
                ServerNotification::ItemCompleted(item)
                    if observed.is_local_core_command_lifecycle(&server)
                        && item.thread_id == thread_id.to_string()
                        && item.turn_id == turn_id =>
                {
                    if let ThreadItem::CommandExecution {
                        id,
                        command,
                        source,
                        status,
                        exit_code,
                        ..
                    } = &item.item
                    {
                        let Some((_, approved_command)) = approved_calls
                            .iter()
                            .find(|(call_id, _)| id.as_str() == *call_id)
                        else {
                            continue;
                        };
                        color_eyre::eyre::ensure!(
                            *source == CommandExecutionSource::UnifiedExecStartup
                                && *status == CommandExecutionStatus::Completed
                                && *exit_code == Some(0)
                                && command.contains(*approved_command),
                            "Core completion mismatched original command: source={source:?}, status={status:?}, exit_code={exit_code:?}, approved_command_present={}",
                            command.contains(*approved_command)
                        );
                        color_eyre::eyre::ensure!(
                            completed_items
                                .insert(id.clone(), command.clone())
                                .is_none(),
                            "Core emitted duplicate command-completion events"
                        );
                    }
                }
                ServerNotification::TurnCompleted(done)
                    if done.thread_id == thread_id.to_string() && done.turn.id == turn_id =>
                {
                    turn_completed = true;
                }
                _ => {}
            },
            _ => {}
        }
        if turn_completed {
            break;
        }
    }
    color_eyre::eyre::ensure!(
        turn_completed && started_items.len() == 2 && started_items == completed_items,
        "live Core did not produce two distinct matching source-bound command lifecycles"
    );
    color_eyre::eyre::ensure!(
        approved.reserve_resource_lock().is_err(),
        "original scratch lock was released before Core completed the benchmark turn"
    );
    server.shutdown().await?;
    drop(held_lock);
    approved
        .reserve_resource_lock()
        .map_err(|error| color_eyre::eyre::eyre!("{error}"))?;
    Ok(())
}
