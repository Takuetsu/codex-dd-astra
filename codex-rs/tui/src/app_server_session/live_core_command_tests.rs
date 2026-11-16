//! 0.4.6 3D.1c2b: a genuine agent-command lifecycle read from the local
//! embedded Core stream. This is an isolated, non-authoritative ingress test:
//! no benchmark admission, owner QA, or privileged process management.

use super::*;
use app_test_support::MockResponsesConfig;
use app_test_support::create_final_assistant_message_sse_response;
use app_test_support::create_mock_responses_server_sequence_unchecked;
use codex_app_server_protocol::CommandExecutionSource;
use codex_app_server_protocol::CommandExecutionStatus;
use codex_app_server_protocol::ServerNotification;
use codex_app_server_protocol::ThreadItem;
use core_test_support::responses;
use std::time::Duration;

#[tokio::test]
async fn codexdd_046_3d1c2b_real_embedded_core_exec_emits_authenticated_lifecycle() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("disposable-native-core-command");
    std::fs::create_dir(&root)?;
    let git = |args: &[&str]| -> Result<()> {
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
        Ok(())
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

    // The mock model requests exactly one harmless command. The Core app-server,
    // not the test, must execute it and emit both native lifecycle events.
    const COMMAND: &str = "echo CODEXDD_CORE_PROVENANCE";
    let model_call = responses::sse(vec![
        responses::ev_response_created("core-command-response"),
        responses::ev_function_call(
            "core-command-call",
            "exec_command",
            &serde_json::json!({
                "cmd": COMMAND,
                "yield_time_ms": 1000,
                "max_output_tokens": 128,
            })
            .to_string(),
        ),
        responses::ev_completed("core-command-response"),
    ]);
    let backend = create_mock_responses_server_sequence_unchecked(vec![
        model_call,
        create_final_assistant_message_sse_response("Core command finished.")?,
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
                text: "Run only the disposable echo command.".to_string(),
                text_elements: Vec::new(),
            }],
            None,
            root.clone(),
            Some(AskForApproval::Never),
            None,
            TurnPermissionsOverride::Preserve,
            &[],
            config.model.clone().unwrap_or_else(|| "mock-model".to_string()),
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
    let mut started_item: Option<(String, String)> = None;
    let mut completed_item: Option<(String, String)> = None;
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
            AppServerEvent::ServerNotification(notification) => {
                match notification.as_ref() {
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
                            color_eyre::eyre::ensure!(
                                *source == CommandExecutionSource::Agent
                                    && *status == CommandExecutionStatus::InProgress
                                    && command.contains(COMMAND),
                                "Core command start does not identify the expected agent execution"
                            );
                            color_eyre::eyre::ensure!(
                                started_item.is_none(),
                                "Core emitted duplicate command start events"
                            );
                            started_item = Some((id.clone(), command.clone()));
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
                            color_eyre::eyre::ensure!(
                                *source == CommandExecutionSource::Agent
                                    && *status == CommandExecutionStatus::Completed
                                    && *exit_code == Some(0)
                                    && command.contains(COMMAND),
                                "Core did not report successful genuine agent command execution"
                            );
                            color_eyre::eyre::ensure!(
                                completed_item.is_none(),
                                "Core emitted duplicate command completion events"
                            );
                            completed_item = Some((id.clone(), command.clone()));
                        }
                    }
                    ServerNotification::TurnCompleted(done)
                        if done.thread_id == thread_id.to_string()
                            && done.turn.id == turn_id =>
                    {
                        turn_completed = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        if turn_completed {
            break;
        }
    }
    color_eyre::eyre::ensure!(
        turn_completed && started_item.is_some() && started_item == completed_item,
        "live Core did not produce one matching agent item/started + item/completed pair"
    );
    server.shutdown().await?;
    Ok(())
}
