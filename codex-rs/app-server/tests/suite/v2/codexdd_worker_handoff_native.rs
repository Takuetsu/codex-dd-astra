//! 0.4.6 3C.1b.2g: real app-server receipt forwarding across two source-pinned
//! Core Worker threads. The test owns its only Git checkout and native scripts.
//! A green run is not TUI READY acceptance or a production review.

use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;
use app_test_support::MockResponsesConfig;
use app_test_support::TestAppServer;
use app_test_support::create_final_assistant_message_sse_response;
use app_test_support::create_mock_responses_server_sequence_unchecked;
use codex_app_server_protocol::AskForApproval;
use codex_app_server_protocol::DynamicToolCallStatus;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::SandboxMode;
use codex_app_server_protocol::ThreadHistoryMode;
use codex_app_server_protocol::ThreadItem;
use codex_app_server_protocol::ThreadResumeParams;
use codex_app_server_protocol::ThreadResumeResponse;
use codex_app_server_protocol::ThreadStartParams;
use codex_app_server_protocol::ThreadStartResponse;
use codex_app_server_protocol::TurnStartParams;
use codex_app_server_protocol::TurnStartResponse;
use codex_app_server_protocol::UserInput;
use core_test_support::responses;
use serde_json::json;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::timeout;

#[cfg(windows)]
const TEST_TIMEOUT: Duration = Duration::from_secs(120);

#[cfg(windows)]
fn git(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()?;
    ensure!(
        output.status.success(),
        "isolated Git {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

#[cfg(windows)]
fn one_stage_script(profile: &str, stage: &str) -> String {
    format!(
        r#"[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"profile_begin","profile":"{profile}","stage_count":1}}')
[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"stage_begin","profile":"{profile}","stage":"{stage}","ordinal":1,"stage_count":1}}')
[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"stage_end","profile":"{profile}","stage":"{stage}","ordinal":1,"status":"pass","native_exit_code":0}}')
[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"profile_end","profile":"{profile}","status":"pass","exit_code":0,"completed_stages":1}}')
exit 0
"#
    )
}

/// Both native handlers are invoked by the mock-model tool-call path of a
/// *real* app-server process. Unlike a synthesized completion this exercises
/// the Core->app-server->client JSON-RPC item/completed translation.
#[cfg(windows)]
#[tokio::test]
async fn codexdd_046_3c1b2g_real_app_server_two_worker_receipts_and_cold_resume() -> Result<()> {
    let checkout = TempDir::new()?;
    let root = checkout.path().join("candidate");
    let scripts = root.join("scripts");
    std::fs::create_dir_all(&scripts)?;
    std::fs::write(
        scripts.join("codexdd-test-workpacket.ps1"),
        one_stage_script("work-packet", "source-acceptance"),
    )?;
    std::fs::write(
        scripts.join("codexdd-test-independent.ps1"),
        one_stage_script("independent-review", "review-acceptance"),
    )?;
    git(&root, &["init", "-q"])?;
    git(
        &root,
        &[
            "add",
            "scripts/codexdd-test-workpacket.ps1",
            "scripts/codexdd-test-independent.ps1",
        ],
    )?;
    git(
        &root,
        &[
            "-c",
            "user.name=CodexDD separate Worker app-server fixture",
            "-c",
            "user.email=codexdd@example.test",
            "commit",
            "-q",
            "-m",
            "native source and independent fixture",
        ],
    )?;
    let sha = git(&root, &["rev-parse", "HEAD"])?;
    let canonical = root.canonicalize()?;
    let scope = "owner-authorized 3C.1b.2g disposable source";

    let calls = [
        (
            "implementation",
            "appserver-source-3c1b2g",
            "run_codexdd_validation",
            "work_packet",
        ),
        (
            "validation",
            "appserver-independent-3c1b2g",
            "run_codexdd_independent_validation",
            "independent_review",
        ),
    ];
    let mut response_sequence = Vec::new();
    for (role, call_id, tool, profile) in calls {
        // First direct user turn durably binds the original owner identity.
        // This must be a separate turn from the protected native successor.
        response_sequence.push(create_final_assistant_message_sse_response(
            "initial owner identity accepted",
        )?);
        let arguments = json!({ "profile": profile }).to_string();
        response_sequence.push(responses::sse(vec![
            responses::ev_response_created(&format!("mock-{role}-tool")),
            responses::ev_function_call(call_id, tool, &arguments),
            responses::ev_completed(&format!("mock-{role}-tool")),
        ]));
        response_sequence.push(create_final_assistant_message_sse_response("done")?);
    }
    let backend = create_mock_responses_server_sequence_unchecked(response_sequence).await;
    let home = TempDir::new()?;
    MockResponsesConfig::new(&backend.uri()).write(home.path())?;
    let mut app_server = TestAppServer::builder()
        .with_codex_home(home.path())
        .without_auto_env()
        .build_initialized()
        .await?;

    let mut thread_ids = Vec::new();
    let mut native_run_ids = Vec::new();
    for (role, call_id, tool, profile) in calls {
        let id = app_server
            .send_thread_start_request(ThreadStartParams {
                model: Some("mock-model".to_string()),
                cwd: Some(root.display().to_string()),
                history_mode: Some(ThreadHistoryMode::Legacy),
                // Original Worker authority must come from an actual direct
                // user turn. Launcher config is intentionally not persisted
                // as the Core first-user authorization checkpoint.
                approval_policy: Some(AskForApproval::Never),
                // A deliberately approved no-sandbox policy exists only inside
                // this self-contained Windows CI fixture and its Git directory.
                sandbox: Some(SandboxMode::DangerFullAccess),
                ..Default::default()
            })
            .await?;
        let started: ThreadStartResponse =
            timeout(TEST_TIMEOUT, app_server.read_response(id)).await??;
        let thread_id = started.thread.id;
        ensure!(
            !thread_ids.contains(&thread_id),
            "fresh Validation Worker reused the Implementation thread identity"
        );

        // The first user turn is a strict TOML Worker assignment, not a model
        // tool call. Core must durably flush that exact-thread authority before
        // a later, separately marked validation turn can use it.
        let first_message = format!(
            "[adaptive_worker]\nrole = \"{role}\"\nauthorized_scope = \"{scope}\"\nsource_root = '{}'\nsource_sha = \"{sha}\"\ntask_kind = \"source_change\"\n\nAuthorize this isolated {role} Worker for one bounded native profile.",
            canonical.display()
        );
        let first_id = app_server
            .send_turn_start_request(TurnStartParams {
                thread_id: thread_id.clone(),
                input: vec![UserInput::Text {
                    text: first_message,
                    text_elements: Vec::new(),
                }],
                ..Default::default()
            })
            .await?;
        let first_started: TurnStartResponse =
            timeout(TEST_TIMEOUT, app_server.read_response(first_id)).await??;
        let first_terminal = timeout(
            TEST_TIMEOUT,
            app_server.read_stream_until_notification_message("turn/completed"),
        )
        .await??;
        ensure!(
            first_terminal.params.as_ref().and_then(|params| params.get("threadId"))
                .and_then(|id| id.as_str()) == Some(thread_id.as_str()),
            "owner assignment turn finished on a foreign thread"
        );
        ensure!(
            first_terminal
                .params
                .as_ref()
                .and_then(|params| params.pointer("/turn/id"))
                .and_then(|id| id.as_str()) == Some(first_started.turn.id.as_str()),
            "owner assignment turn was not the expected Core turn"
        );
        ensure!(
            first_terminal
                .params
                .as_ref()
                .and_then(|params| params.pointer("/turn/status"))
                .and_then(|status| status.as_str()) == Some("completed"),
            "initial direct user Worker assignment must finish successfully"
        );

        let turn_id = app_server
            .send_turn_start_request(TurnStartParams {
                thread_id: thread_id.clone(),
                input: vec![UserInput::Text {
                    text: format!("Run the preauthorized {profile} native profile."),
                    text_elements: Vec::new(),
                }],
                // Source LVO is intentionally absent on an ordinary model turn.
                // Only the original owner-approved Implementation Worker may
                // request the bounded MechanicalValidation lifecycle.
                turn_trigger: (role == "implementation").then(|| {
                    codex_protocol::CODEXDD_ADAPTIVE_MECHANICAL_VALIDATION_TURN_TRIGGER.to_string()
                }),
                ..Default::default()
            })
            .await?;
        let started_turn: TurnStartResponse =
            timeout(TEST_TIMEOUT, app_server.read_response(turn_id)).await??;

        let item = timeout(
            TEST_TIMEOUT,
            app_server.read_stream_until_matching_notification(
                "matching genuine native validation completion",
                |notice| {
                    // Fail immediately on a completed turn that never issued
                    // the required native receipt (e.g. a wrongly tagged turn)
                    // rather than spending the entire 120s deadline.
                    if notice.method == "turn/completed" {
                        return notice
                            .params
                            .as_ref()
                            .and_then(|params| params.get("threadId"))
                            .and_then(|id| id.as_str())
                            == Some(thread_id.as_str());
                    }
                    notice.method == "item/completed"
                        && notice
                            .params
                            .as_ref()
                            .and_then(|value| {
                                serde_json::from_value::<ItemCompletedNotification>(value.clone())
                                    .ok()
                            })
                            .is_some_and(|done| {
                                done.thread_id == thread_id
                                    && matches!(done.item,
                                        ThreadItem::DynamicToolCall { ref id, .. }
                                        if id == call_id
                                    )
                            })
                },
            ),
        )
        .await??;
        ensure!(
            item.method == "item/completed",
            "{role} turn completed without a real {tool} receipt; refusing synthetic PASS",
        );
        let completion: ItemCompletedNotification =
            serde_json::from_value(item.params.context("native completion payload missing")?)?;
        ensure!(completion.thread_id == thread_id);
        ensure!(completion.turn_id == started_turn.turn.id);
        ensure!(completion.completed_at_ms > 0);
        let ThreadItem::DynamicToolCall {
            id,
            namespace,
            tool: actual_tool,
            arguments,
            status,
            success,
            ..
        } = completion.item
        else {
            anyhow::bail!("expected real dynamic tool completion across app-server");
        };
        ensure!(id == call_id && actual_tool == tool && namespace.is_none());
        ensure!(status == DynamicToolCallStatus::Completed && success == Some(true));
        ensure!(arguments["profile"] == profile);
        ensure!(arguments["result"] == "pass");
        ensure!(arguments["head_sha"] == sha);
        let reported_root = arguments["source_root"]
            .as_str()
            .context("Core must supply authoritative source_root")?;
        ensure!(Path::new(reported_root).canonicalize()? == canonical);
        let run_id = arguments["run_id"]
            .as_str()
            .context("real native run_id missing")?
            .to_string();
        ensure!(!run_id.trim().is_empty());
        ensure!(
            !native_run_ids.contains(&run_id),
            "separate Worker reused native run ID"
        );
        native_run_ids.push(run_id);
        thread_ids.push(thread_id);

        timeout(
            TEST_TIMEOUT,
            app_server.read_stream_until_notification_message("turn/completed"),
        )
        .await??;
    }

    ensure!(thread_ids.len() == 2 && native_run_ids.len() == 2);
    ensure!(
        git(
            &root,
            &["status", "--porcelain=v1", "--untracked-files=all"]
        )?
        .is_empty(),
        "real native process should leave source candidate unchanged"
    );
    timeout(TEST_TIMEOUT, app_server.shutdown_gracefully()).await??;

    // Cold resume in a distinct app-server process must not lose the original
    // separately Core-bound source/role identity or promote the Validator.
    let mut restarted = TestAppServer::builder()
        .with_codex_home(home.path())
        .without_auto_env()
        .build_initialized()
        .await?;
    for (index, (role, _, _, _)) in calls.iter().enumerate() {
        let id = restarted
            .send_thread_resume_request(ThreadResumeParams {
                thread_id: thread_ids[index].clone(),
                ..Default::default()
            })
            .await?;
        let resumed: ThreadResumeResponse =
            timeout(TEST_TIMEOUT, restarted.read_response(id)).await??;
        ensure!(resumed.thread.id == thread_ids[index]);
        let worker = resumed
            .codexdd_first_worker
            .context("cold resumed thread lost its Core-owned Worker source identity")?;
        ensure!(worker.role == *role);
        ensure!(worker.authorized_scope == scope);
        ensure!(worker.source_sha.as_deref() == Some(sha.as_str()));
        ensure!(
            worker
                .source_root
                .as_ref()
                .is_some_and(
                    |path| Path::new(path).canonicalize().ok().as_ref() == Some(&canonical)
                )
        );
    }
    timeout(TEST_TIMEOUT, restarted.shutdown_gracefully()).await??;
    Ok(())
}
