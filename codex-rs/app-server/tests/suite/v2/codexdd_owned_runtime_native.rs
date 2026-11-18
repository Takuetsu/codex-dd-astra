//! 0.4.6 D02: actual Windows Core-held HTTP listener and app-server QA review.
//! The disposable owner worktree is committed before any model tool call.
//! No fixture executable or process is adopted from the user's machine.
//! This verifies the Core/AppServer positive health path, not full TUI READY.

#![cfg(windows)]

use anyhow::Result;
use anyhow::ensure;
use app_test_support::MockResponsesConfig;
use app_test_support::TestAppServer;
use app_test_support::create_final_assistant_message_sse_response;
use app_test_support::create_mock_responses_server_sequence_unchecked;
use codex_app_server_protocol::AskForApproval;
use codex_app_server_protocol::RequestId;
use codex_app_server_protocol::SandboxMode;
use codex_app_server_protocol::ThreadHistoryMode;
use codex_app_server_protocol::ThreadOwnedRuntimeHealthReviewResponse;
use codex_app_server_protocol::ThreadResumeParams;
use codex_app_server_protocol::ThreadResumeResponse;
use codex_app_server_protocol::ThreadStartParams;
use codex_app_server_protocol::ThreadStartResponse;
use codex_app_server_protocol::TurnStartParams;
use codex_app_server_protocol::TurnStartResponse;
use codex_app_server_protocol::UserInput;
use core_test_support::responses;
use serde_json::json;
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::timeout;

const DEADLINE: Duration = Duration::from_secs(100);

fn fixture_git(root: &Path, args: &[&str]) -> Result<String> {
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

fn server_script(port: u16, stop_marker: &Path) -> String {
    // The listener runs in the *same PowerShell process* retained by Core:
    // spawning a second Powershell child would invalidate the exact OS PID.
    r#"$ErrorActionPreference = 'Stop'
$marker = 'STOP_MARKER'
$listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Loopback, PORT_VALUE)
$listener.Start()
try {
    $deadline = (Get-Date).AddSeconds(90)
    while ((Get-Date) -lt $deadline -and -not (Test-Path -LiteralPath $marker)) {
        if (-not $listener.Pending()) {
            Start-Sleep -Milliseconds 20
            continue
        }
        $client = $listener.AcceptTcpClient()
        try {
            $stream = $client.GetStream()
            $stream.ReadTimeout = 1000
            $bytes = New-Object byte[] 1024
            [void]$stream.Read($bytes, 0, $bytes.Length)
            $crlf = [string][char]13 + [string][char]10
            $response = 'HTTP/1.1 200 OK' + $crlf +
                'Content-Length: 2' + $crlf +
                'Connection: close' + $crlf + $crlf + 'OK'
            $reply = [Text.Encoding]::ASCII.GetBytes($response)
            $stream.Write($reply, 0, $reply.Length)
            $stream.Flush()
        } finally {
            $client.Close()
        }
    }
} finally {
    $listener.Stop()
}
"#
    .replace("PORT_VALUE", &port.to_string())
    .replace(
        "STOP_MARKER",
        &stop_marker.display().to_string().replace('\'', "''"),
    )
}

async fn assert_live_review(
    app_server: &mut TestAppServer,
    thread_id: &str,
) -> Result<ThreadOwnedRuntimeHealthReviewResponse> {
    let request = app_server
        .send_request(
            "thread/ownedRuntimeHealth/review",
            Some(json!({"threadId": thread_id})),
        )
        .await?;
    timeout(DEADLINE, app_server.read_response(request)).await?
}

async fn assert_refused_review(app_server: &mut TestAppServer, thread_id: &str) -> Result<()> {
    let request = app_server
        .send_request(
            "thread/ownedRuntimeHealth/review",
            Some(json!({"threadId": thread_id})),
        )
        .await?;
    let refused = timeout(
        DEADLINE,
        app_server.read_stream_until_error_message(RequestId::Integer(request)),
    )
    .await??;
    ensure!(
        format!("{refused:?}").contains("SETUP_BLOCKED"),
        "live Core QA review must refuse without original held listener: {refused:?}"
    );
    Ok(())
}

#[tokio::test]
async fn codexdd_046_3d1h_real_core_owned_http_review_and_cold_resume() -> Result<()> {
    let source = TempDir::new()?;
    let root = source.path().join("authorized");
    let scripts = root.join("scripts");
    std::fs::create_dir_all(&scripts)?;
    let stop_marker = source.path().join("stop-owned-service");
    let reserved = TcpListener::bind(("127.0.0.1", 0))?;
    let port = reserved.local_addr()?.port();
    drop(reserved);

    let pwsh = Path::new(&std::env::var("ProgramFiles")?)
        .join("PowerShell")
        .join("7")
        .join("pwsh.exe");
    ensure!(pwsh.is_file(), "native Windows pwsh.exe required");
    let script = scripts.join("owned-health.ps1");
    std::fs::write(&script, server_script(port, &stop_marker))?;
    fixture_git(&root, &["init", "-q"])?;
    fixture_git(&root, &["add", "scripts/owned-health.ps1"])?;
    fixture_git(
        &root,
        &[
            "-c",
            "user.name=CodexDD isolated runtime fixture",
            "-c",
            "user.email=codexdd@example.test",
            "commit",
            "--no-gpg-sign",
            "-q",
            "-m",
            "owner approved live health service",
        ],
    )?;
    let sha = fixture_git(&root, &["rev-parse", "HEAD"])?;
    let root = root.canonicalize()?;
    // PowerShell -Command runs the script in this same OS process, retaining
    // the original Core-held PID as the actual TCP listener owner.
    let command = format!("& '{}'", script.display().to_string().replace('\'', "''"));
    let launch_argv = vec![
        pwsh.display().to_string(),
        "-NoProfile".to_string(),
        "-Command".to_string(),
        command.clone(),
    ];
    let argv_toml = serde_json::to_string(&launch_argv)?;
    let owner_message = format!(
        "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"isolated D02 live service acceptance\"\nsource_root = '{}'\nsource_sha = \"{sha}\"\ntask_kind = \"operations_preview\"\nrequired_gates = [{{ kind = \"owned_runtime_health\", ports = [{port}], command_argv = {argv_toml} }}, {{ kind = \"independent_validation\" }}]\n\nLaunch only the authorized disposable service.",
        root.display()
    );
    ensure!(
        matches!(
            codex_config::config_toml::parse_adaptive_worker_first_message(&owner_message),
            Ok(Some(_))
        ),
        "original owner argv/port authorization must parse as one strict Worker"
    );

    let sequence = vec![
        create_final_assistant_message_sse_response("Owner binding recorded.")?,
        responses::sse(vec![
            responses::ev_response_created("native-owned-service-launch"),
            responses::ev_function_call(
                "d02-verified-launch",
                "exec_command",
                &json!({
                    "cmd": command,
                    "shell": pwsh.display().to_string(),
                    "login": false,
                    "tty": false,
                    "yield_time_ms": 1000
                })
                .to_string(),
            ),
            responses::ev_completed("native-owned-service-launch"),
        ]),
        create_final_assistant_message_sse_response("Service launched.")?,
    ];
    let mock = create_mock_responses_server_sequence_unchecked(sequence).await;
    let home = TempDir::new()?;
    MockResponsesConfig::new(&mock.uri()).write(home.path())?;
    let mut app_server = TestAppServer::builder()
        .with_codex_home(home.path())
        .without_auto_env()
        .build_initialized()
        .await?;
    let start_id = app_server
        .send_thread_start_request(ThreadStartParams {
            model: Some("mock-model".to_string()),
            cwd: Some(root.display().to_string()),
            history_mode: Some(ThreadHistoryMode::Legacy),
            approval_policy: Some(AskForApproval::Never),
            // This is confined to the temporary, committed CI fixture.
            sandbox: Some(SandboxMode::DangerFullAccess),
            ..Default::default()
        })
        .await?;
    let started: ThreadStartResponse =
        timeout(DEADLINE, app_server.read_response(start_id)).await??;
    let thread_id = started.thread.id;

    let first_id = app_server
        .send_turn_start_request(TurnStartParams {
            thread_id: thread_id.clone(),
            input: vec![UserInput::Text {
                text: owner_message,
                text_elements: Vec::new(),
            }],
            ..Default::default()
        })
        .await?;
    let _: TurnStartResponse = timeout(DEADLINE, app_server.read_response(first_id)).await??;
    timeout(
        DEADLINE,
        app_server.read_stream_until_notification_message("turn/completed"),
    )
    .await??;

    // Before the Core tool launches a service, owner approval is not health.
    assert_refused_review(&mut app_server, &thread_id).await?;

    let launch_id = app_server
        .send_turn_start_request(TurnStartParams {
            thread_id: thread_id.clone(),
            input: vec![UserInput::Text {
                text: "Execute the exact original Core-owned preview launch.".to_string(),
                text_elements: Vec::new(),
            }],
            ..Default::default()
        })
        .await?;
    let _: TurnStartResponse = timeout(DEADLINE, app_server.read_response(launch_id)).await??;
    timeout(
        DEADLINE,
        app_server.read_stream_until_notification_message("turn/completed"),
    )
    .await??;

    let observed = assert_live_review(&mut app_server, &thread_id).await?;
    ensure!(
        observed.owner_pid > 0,
        "Core omitted actual OS listener PID"
    );
    ensure!(
        observed.approved_port == port,
        "Core reviewed unapproved port"
    );
    ensure!(observed.source_sha.eq_ignore_ascii_case(&sha));
    ensure!(Path::new(&observed.source_root).canonicalize()? == root);
    ensure!(
        uuid::Uuid::parse_str(&observed.observation_id).is_ok(),
        "Core did not return a fresh observation identity"
    );
    let again = assert_live_review(&mut app_server, &thread_id).await?;
    ensure!(
        again.observation_id != observed.observation_id,
        "fresh Core QA review must never replay an old observation ID"
    );

    // Exit the disposable service without changing the source worktree.
    std::fs::write(&stop_marker, b"stop")?;
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert_refused_review(&mut app_server, &thread_id).await?;
    ensure!(
        fixture_git(
            &root,
            &["status", "--porcelain=v1", "--untracked-files=all"]
        )?
        .is_empty(),
        "native QA fixture modified its pinned Git candidate"
    );
    timeout(DEADLINE, app_server.shutdown_gracefully()).await??;

    let mut restarted = TestAppServer::builder()
        .with_codex_home(home.path())
        .without_auto_env()
        .build_initialized()
        .await?;
    let resume_id = restarted
        .send_thread_resume_request(ThreadResumeParams {
            thread_id: thread_id.clone(),
            ..Default::default()
        })
        .await?;
    let resumed: ThreadResumeResponse =
        timeout(DEADLINE, restarted.read_response(resume_id)).await??;
    ensure!(resumed.thread.id == thread_id);
    assert_refused_review(&mut restarted, &thread_id).await?;
    timeout(DEADLINE, restarted.shutdown_gracefully()).await??;
    Ok(())
}
