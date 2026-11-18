//! 0.4.6 3C.1b.2i — real Windows app-server JSON-RPC native events into
//! the TUI's production ChatWidget notification, gate, and terminal reducers.
//! Deterministic mock model + disposable committed scripts; no user worktrees.

use super::*;
use anyhow::Context;
use anyhow::Result;
use anyhow::ensure;
use app_test_support::MockResponsesConfig;
use app_test_support::TestAppServer;
use app_test_support::create_final_assistant_message_sse_response;
use app_test_support::create_mock_responses_server_sequence_unchecked;
use codex_app_server_protocol::AdaptiveRuntimeSignalNotification;
use codex_app_server_protocol::AskForApproval;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::JSONRPCNotification;
use codex_app_server_protocol::SandboxMode;
use codex_app_server_protocol::ServerNotification;
use codex_app_server_protocol::ThreadHistoryMode;
use codex_app_server_protocol::ThreadItem;
use codex_app_server_protocol::ThreadResumeParams;
use codex_app_server_protocol::ThreadResumeResponse;
use codex_app_server_protocol::ThreadStartParams;
use codex_app_server_protocol::ThreadStartResponse;
use codex_app_server_protocol::TurnCompletedNotification;
use codex_app_server_protocol::TurnStartParams;
use codex_app_server_protocol::TurnStartResponse;
use codex_app_server_protocol::TurnStartedNotification;
use codex_app_server_protocol::UserInput;
use core_test_support::responses;
use serde_json::json;
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::timeout;

const LIVE_TIMEOUT: Duration = Duration::from_secs(90);

fn d02_native_service_script(port: u16, stop_marker: &Path) -> String {
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

fn fixture_git(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()?;
    ensure!(
        output.status.success(),
        "disposable Git {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn one_stage_native_script(profile: &str, stage: &str) -> String {
    format!(
        r#"[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"profile_begin","profile":"{profile}","stage_count":1}}')
[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"stage_begin","profile":"{profile}","stage":"{stage}","ordinal":1,"stage_count":1}}')
[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"stage_end","profile":"{profile}","stage":"{stage}","ordinal":1,"status":"pass","native_exit_code":0}}')
[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"profile_end","profile":"{profile}","status":"pass","exit_code":0,"completed_stages":1}}')
exit 0
"#
    )
}

/// Only real app-server bytes may be passed into this TUI reducer. This
/// helper performs the same typed v2 protocol decoding that the TUI client
/// expects; it does not construct or modify evidence, run IDs, or gate status.
fn decode_live_notification(notification: JSONRPCNotification) -> Result<ServerNotification> {
    let method = notification.method;
    let params = notification
        .params
        .with_context(|| format!("live {method} missing server parameters"))?;
    match method.as_str() {
        "item/completed" => Ok(ServerNotification::ItemCompleted(serde_json::from_value(
            params,
        )?)),
        "turn/started" => Ok(ServerNotification::TurnStarted(serde_json::from_value(
            params,
        )?)),
        "turn/adaptiveRuntimeSignal" => Ok(ServerNotification::AdaptiveRuntimeSignal(
            serde_json::from_value(params)?,
        )),
        "turn/completed" => Ok(ServerNotification::TurnCompleted(serde_json::from_value(
            params,
        )?)),
        _ => anyhow::bail!("unrecognized native app-server event {method}"),
    }
}

async fn take_turn_event(
    server: &mut TestAppServer,
    method: &str,
    thread_id: &str,
    turn_id: &str,
) -> Result<ServerNotification> {
    let event = timeout(
        LIVE_TIMEOUT,
        server.read_stream_until_matching_notification(
            "exact source-bound native turn event",
            |n| {
                n.method == method
                    && n.params.as_ref().is_some_and(|params| {
                        params.get("threadId").and_then(|v| v.as_str()) == Some(thread_id)
                            && params.pointer("/turn/id").and_then(|v| v.as_str()) == Some(turn_id)
                    })
            },
        ),
    )
    .await??;
    decode_live_notification(event)
}

async fn take_item_event(
    server: &mut TestAppServer,
    thread_id: &str,
    turn_id: &str,
    expected_call: Option<&str>,
) -> Result<ServerNotification> {
    let note = timeout(
        LIVE_TIMEOUT,
        server.read_stream_until_matching_notification(
            "authentic app-server item/completed",
            |n| {
                n.method == "item/completed"
                    && n.params.as_ref().is_some_and(|params| {
                        serde_json::from_value::<ItemCompletedNotification>(params.clone())
                            .ok()
                            .is_some_and(|done| {
                                done.thread_id == thread_id
                                    && done.turn_id == turn_id
                                    && match &done.item {
                                        ThreadItem::UserMessage { .. } => expected_call.is_none(),
                                        ThreadItem::DynamicToolCall { id, .. } => {
                                            expected_call == Some(id.as_str())
                                        }
                                        _ => false,
                                    }
                            })
                    })
            },
        ),
    )
    .await??;
    decode_live_notification(note)
}

async fn take_trusted_signal(
    server: &mut TestAppServer,
    thread_id: &str,
    turn_id: &str,
) -> Result<ServerNotification> {
    let note = timeout(
        LIVE_TIMEOUT,
        server.read_stream_until_matching_notification("real signal or premature terminal", |n| {
            if n.method == "turn/completed" {
                return n.params.as_ref().is_some_and(|params| {
                    params.get("threadId").and_then(|v| v.as_str()) == Some(thread_id)
                        && params.pointer("/turn/id").and_then(|v| v.as_str()) == Some(turn_id)
                });
            }
            n.method == "turn/adaptiveRuntimeSignal"
                && n.params.as_ref().is_some_and(|params| {
                    serde_json::from_value::<AdaptiveRuntimeSignalNotification>(params.clone())
                        .ok()
                        .is_some_and(|signal| {
                            signal.thread_id == thread_id && signal.signal.source_turn_id == turn_id
                        })
                })
        }),
    )
    .await??;
    ensure!(
        note.method == "turn/adaptiveRuntimeSignal",
        "turn terminated without a live trusted Core/adaptive signal"
    );
    decode_live_notification(note)
}

/// The mock model supplies only deterministic built-in tool calls. Both
/// validation scripts and both adaptive signals are actually executed and
/// emitted by Core inside the spawned Windows app-server process.
#[tokio::test]
async fn codexdd_046_3c1b2i_real_app_server_to_tui_native_ready_handoff() -> Result<()> {
    run_native_app_server_to_tui_handoff(false).await
}

/// Real Core-native source and independent-review tool receipts must remain
/// separately bound after the app-server OS process is replaced. Old source
/// events cannot complete a new Validation Worker or replay owner READY.
/// The App event-loop and persisted TUI UI checkpoint still need b2b2 proof.
#[cfg(windows)]
#[tokio::test]
async fn codexdd_046_3c1b2j3b2b1_real_native_receipts_across_restart_refuse_replay() -> Result<()> {
    run_native_app_server_to_tui_handoff(true).await
}

async fn run_native_app_server_to_tui_handoff(restart_between_workers: bool) -> Result<()> {
    use crate::adaptive_complexity::AdaptiveComplexityClass;
    use crate::adaptive_complexity::AdaptiveImplementationPhase;
    use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;
    use crate::adaptive_worker::AdaptiveWorkerRole;
    use crate::adaptive_worker::AdaptiveWorkflowTerminal;
    use crate::adaptive_worker::NewWorkerBinding;
    use crate::chatwidget::tests::make_chatwidget_manual_with_sender;

    let temp = TempDir::new()?;
    let root = temp.path().join("native-source");
    let scripts = root.join("scripts");
    std::fs::create_dir_all(&scripts)?;
    std::fs::write(
        scripts.join("codexdd-test-workpacket.ps1"),
        one_stage_native_script("work-packet", "real-tui-source-stage"),
    )?;
    std::fs::write(
        scripts.join("codexdd-test-independent.ps1"),
        one_stage_native_script("independent-review", "real-tui-independent-stage"),
    )?;
    fixture_git(&root, &["init", "-q"])?;
    fixture_git(
        &root,
        &[
            "add",
            "scripts/codexdd-test-workpacket.ps1",
            "scripts/codexdd-test-independent.ps1",
        ],
    )?;
    fixture_git(
        &root,
        &[
            "-c",
            "user.name=CodexDD native TUI test",
            "-c",
            "user.email=codexdd@example.test",
            "commit",
            "-q",
            "-m",
            "single verified native two-Worker TUI candidate",
        ],
    )?;
    let sha = fixture_git(&root, &["rev-parse", "HEAD"])?;
    ensure!(sha.len() == 40, "native fixture requires committed Git SHA");
    let canonical = root.canonicalize()?;
    let scope = "0.4.6 native app-server to TUI gate acceptance";
    let work = [
        (
            "implementation",
            "work_packet",
            "run_codexdd_validation",
            "native-tui-source-run",
            "ready_for_validation",
        ),
        (
            "validation",
            "independent_review",
            "run_codexdd_independent_validation",
            "native-tui-independent-run",
            "ready_for_owner_qa",
        ),
    ];
    let mut responses_sequence = Vec::new();
    for (role, profile, tool, call_id, signal_kind) in work {
        responses_sequence.push(create_final_assistant_message_sse_response(
            "First owner assignment acknowledged.",
        )?);
        responses_sequence.push(responses::sse(vec![
            responses::ev_response_created(&format!("native-{role}-validation")),
            responses::ev_function_call(call_id, tool, &json!({"profile": profile}).to_string()),
            responses::ev_completed(&format!("native-{role}-validation")),
        ]));
        responses_sequence.push(responses::sse(vec![
            responses::ev_response_created(&format!("native-{role}-signal")),
            responses::ev_function_call(
                &format!("{call_id}-signal"),
                "report_adaptive_signal",
                &json!({
                    "kind": signal_kind,
                    "evidence_refs": [call_id],
                    "diagnostic_note": "bounded native owner acceptance",
                })
                .to_string(),
            ),
            responses::ev_completed(&format!("native-{role}-signal")),
        ]));
        responses_sequence.push(create_final_assistant_message_sse_response("done")?);
    }
    let backend = create_mock_responses_server_sequence_unchecked(responses_sequence).await;
    let home = TempDir::new()?;
    MockResponsesConfig::new(&backend.uri()).write(home.path())?;
    let mut server = TestAppServer::builder()
        .with_codex_home(home.path())
        .without_auto_env()
        .build_initialized()
        .await?;

    let (mut chat, _sender, mut app_events, _op_rx) = make_chatwidget_manual_with_sender().await;
    let mut origin_thread: Option<ThreadId> = None;
    let mut run_ids = Vec::new();
    let mut previous_source_receipt: Option<ServerNotification> = None;
    let mut previous_source_signal: Option<ServerNotification> = None;
    for (role, profile, _tool, call_id, _) in work {
        let request_id = server
            .send_thread_start_request(ThreadStartParams {
                model: Some("mock-model".to_string()),
                cwd: Some(root.display().to_string()),
                history_mode: Some(ThreadHistoryMode::Legacy),
                approval_policy: Some(AskForApproval::Never),
                // CI fixture only: native PowerShell execution is restricted
                // to these two committed, source-pinned disposable scripts.
                sandbox: Some(SandboxMode::DangerFullAccess),
                ..Default::default()
            })
            .await?;
        let started: ThreadStartResponse =
            timeout(LIVE_TIMEOUT, server.read_response(request_id)).await??;
        let thread_id = started.thread.id;
        let thread = ThreadId::from_string(&thread_id)?;
        if role == "implementation" {
            chat.thread_id = Some(thread);
            chat.dispatch_adaptive_command("astra");
        } else {
            let origin = origin_thread.context("real source Worker missing")?;
            ensure!(
                origin != thread,
                "Validator must use a different app-server thread"
            );
            chat.thread_id = Some(thread);
            if restart_between_workers {
                let before = &chat.adaptive_effort;
                let gates = before
                    .gate_ledger
                    .as_ref()
                    .context("cold-restarted Validator missing original owner gates")?;
                let pending_before = gates.outstanding(&before.evidence_registry).len();
                let evidence_before = before.evidence_registry.len();
                ensure!(
                    pending_before == 1 && before.workflow_terminal.is_none(),
                    "fresh Validator must retain only the original source PASS, not owner READY"
                );
                chat.handle_server_notification(
                    previous_source_receipt
                        .clone()
                        .context("missing authentic prior source receipt")?,
                    None,
                );
                chat.handle_server_notification(
                    previous_source_signal
                        .clone()
                        .context("missing authentic prior source signal")?,
                    None,
                );
                let after = &chat.adaptive_effort;
                ensure!(
                    after.evidence_registry.len() == evidence_before
                        && after
                            .gate_ledger
                            .as_ref()
                            .context("replayed source event erased owner gates")?
                            .outstanding(&after.evidence_registry)
                            .len()
                            == pending_before
                        && after.workflow_terminal.is_none(),
                    "old Core source receipt/signal replay granted Validator gate or READY"
                );
            }
        }

        // Owner authority is encoded only by the actual first user message.
        // No test helper is allowed to assign the TUI ledger or import a PASS.
        let required = if role == "implementation" {
            "required_gates = [{ kind = \"source_profile\", profile = \"work_packet\" }, { kind = \"independent_validation\" }]\n"
        } else {
            ""
        };
        let owner_message = format!(
            "[adaptive_worker]\nrole = \"{role}\"\nauthorized_scope = \"{scope}\"\nsource_root = '{}'\nsource_sha = \"{sha}\"\ntask_kind = \"source_change\"\n{required}\nAuthorize the disposable {role} native profile.",
            canonical.display()
        );
        let first_req = server
            .send_turn_start_request(TurnStartParams {
                thread_id: thread_id.clone(),
                input: vec![UserInput::Text {
                    text: owner_message,
                    text_elements: Vec::new(),
                }],
                ..Default::default()
            })
            .await?;
        let first: TurnStartResponse =
            timeout(LIVE_TIMEOUT, server.read_response(first_req)).await??;
        let first_item = take_item_event(&mut server, &thread_id, &first.turn.id, None).await?;
        chat.handle_server_notification(first_item, None);
        if role == "implementation" {
            ensure!(
                chat.adaptive_effort.worker_context.role == AdaptiveWorkerRole::Implementation,
                "TUI ignored true owner-origin implementation header"
            );
            ensure!(
                chat.adaptive_effort
                    .gate_ledger
                    .as_ref()
                    .is_some_and(|ledger| {
                        ledger
                            .outstanding(&chat.adaptive_effort.evidence_registry)
                            .len()
                            == 2
                    }),
                "real first user message did not create both owner-required gates"
            );
        } else {
            ensure!(
                chat.adaptive_effort.worker_context.role == AdaptiveWorkerRole::Validation,
                "fresh Validator lost inherited owner role"
            );
            ensure!(
                chat.adaptive_effort.gate_ledger.is_some(),
                "fresh Validator lost immutable origin gate ledger"
            );
        }
        let first_done =
            take_turn_event(&mut server, "turn/completed", &thread_id, &first.turn.id).await?;
        ensure!(
            matches!(
                first_done,
                ServerNotification::TurnCompleted(TurnCompletedNotification {
                    turn: codex_app_server_protocol::Turn {
                        status: codex_app_server_protocol::TurnStatus::Completed,
                        ..
                    },
                    ..
                })
            ),
            "first owner-authenticated turn did not finish successfully"
        );

        if role == "implementation" {
            // Earlier authorizations have already moved the source Worker
            // into native MechanicalValidation; this packet tests the
            // live native receipt and terminal seam, not that earlier route.
            chat.adaptive_effort.complexity_class = Some(AdaptiveComplexityClass::Standard);
            chat.adaptive_effort.implementation_phase =
                Some(AdaptiveImplementationPhase::MechanicalValidation);
        }
        let native_req = server
            .send_turn_start_request(TurnStartParams {
                thread_id: thread_id.clone(),
                input: vec![UserInput::Text {
                    text: format!("Execute only the authorized {profile} native script."),
                    text_elements: Vec::new(),
                }],
                turn_trigger: (role == "implementation").then(|| {
                    codex_protocol::CODEXDD_ADAPTIVE_MECHANICAL_VALIDATION_TURN_TRIGGER.to_string()
                }),
                ..Default::default()
            })
            .await?;
        let native_turn: TurnStartResponse =
            timeout(LIVE_TIMEOUT, server.read_response(native_req)).await??;
        let native_started = take_turn_event(
            &mut server,
            "turn/started",
            &thread_id,
            &native_turn.turn.id,
        )
        .await?;
        chat.handle_server_notification(native_started, None);

        let native_item =
            take_item_event(&mut server, &thread_id, &native_turn.turn.id, Some(call_id)).await?;
        let ServerNotification::ItemCompleted(ref receipt) = native_item else {
            anyhow::bail!("native app-server did not deliver a typed tool receipt");
        };
        let ThreadItem::DynamicToolCall {
            arguments,
            status,
            success,
            ..
        } = &receipt.item
        else {
            anyhow::bail!("expected native Core tool completion, not synthetic text");
        };
        // Core receipts use the authorized tool profile ID (underscores),
        // not the human-readable PowerShell script profile label (hyphens).
        ensure!(
            *status == codex_app_server_protocol::DynamicToolCallStatus::Completed
                && *success == Some(true)
                && arguments["head_sha"] == sha
                && arguments["result"] == "pass"
                && arguments["profile"] == profile,
            "Core process receipt failed its native status or source pin"
        );
        let native_run = arguments["run_id"]
            .as_str()
            .context("native receipt missing Core run identity")?
            .to_string();
        ensure!(!native_run.is_empty() && !run_ids.contains(&native_run));
        run_ids.push(native_run);
        let native_item_replay = native_item.clone();
        chat.handle_server_notification(native_item, None);

        if role == "implementation" {
            ensure!(
                chat.adaptive_effort
                    .gate_ledger
                    .as_ref()
                    .context("missing first-owner ledger")?
                    .outstanding(&chat.adaptive_effort.evidence_registry)
                    .len()
                    == 1,
                "real native source item did not satisfy only the owner's source-profile gate"
            );
        } else {
            ensure!(
                chat.adaptive_effort
                    .gate_ledger
                    .as_ref()
                    .context("missing inherited owner ledger")?
                    .ready_for_owner_qa(&chat.adaptive_effort.evidence_registry),
                "real separate native Validator item did not close all owner gates"
            );
        }
        let live_signal =
            take_trusted_signal(&mut server, &thread_id, &native_turn.turn.id).await?;
        let live_signal_replay = live_signal.clone();
        chat.handle_server_notification(live_signal, None);
        let native_done = take_turn_event(
            &mut server,
            "turn/completed",
            &thread_id,
            &native_turn.turn.id,
        )
        .await?;
        chat.handle_server_notification(native_done, None);

        if role == "implementation" {
            ensure!(
                chat.adaptive_effort.workflow_terminal
                    == Some(AdaptiveWorkflowTerminal::ReadyForValidation),
                "authentic Core source receipt and real signal did not authorize READY_FOR_VALIDATION"
            );
            origin_thread = Some(thread);
            let binding = NewWorkerBinding {
                role: AdaptiveWorkerRole::Validation,
                authorized_scope: scope.to_string(),
            };
            let fresh = chat.fresh_adaptive_effort_for_new_worker(Some(&binding));
            ensure!(
                fresh.review_origin_thread_id == Some(thread)
                    && fresh.gate_ledger.is_some()
                    && fresh.assignment_preflight.is_some(),
                "TUI fresh Validator handoff lost original source pin, gate ledger, or origin"
            );
            chat.adaptive_effort = fresh;
            chat.turn_lifecycle.agent_turn_running = false;
            chat.turn_lifecycle.last_turn_id = None;
            if restart_between_workers {
                previous_source_receipt = Some(native_item_replay);
                previous_source_signal = Some(live_signal_replay);
                let inherited = &chat.adaptive_effort;
                ensure!(
                    inherited.review_origin_thread_id == Some(thread)
                        && inherited.evidence_registry.len() == 1
                        && inherited
                            .gate_ledger
                            .as_ref()
                            .context("lost owner ledger on cross-process handoff")?
                            .outstanding(&inherited.evidence_registry)
                            .len()
                            == 1
                        && inherited.workflow_terminal.is_none(),
                    "source receipt/owner gate inheritance granted forged native review"
                );
                timeout(LIVE_TIMEOUT, server.shutdown_gracefully()).await??;
                server = TestAppServer::builder()
                    .with_codex_home(home.path())
                    .without_auto_env()
                    .build_initialized()
                    .await?;
                let request_id = server
                    .send_thread_resume_request(ThreadResumeParams {
                        thread_id: thread_id.clone(),
                        ..Default::default()
                    })
                    .await?;
                let resumed: ThreadResumeResponse =
                    timeout(LIVE_TIMEOUT, server.read_response(request_id)).await??;
                let worker = resumed
                    .codexdd_first_worker
                    .context("new app-server OS process lost original Core first-user source")?;
                ensure!(
                    resumed.thread.id == thread_id
                        && worker.role == "implementation"
                        && worker.authorized_scope == scope
                        && worker.source_sha.as_deref() == Some(sha.as_str())
                        && worker.source_root.as_ref().is_some_and(|path| {
                            Path::new(path).canonicalize().ok().as_ref() == Some(&canonical)
                        }),
                    "cold resumed Core source/role identity does not match original user authority"
                );
            }
        } else {
            ensure!(
                chat.adaptive_effort.workflow_terminal
                    == Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa),
                "real separate Validator native PASS and real signal could not reach owner READY"
            );
            ensure!(
                chat.owner_qa_prerequisite_admission(&[call_id.to_string()])
                    .is_ok(),
                "post-terminal READY must cite actual separate native Validator receipt"
            );
            if restart_between_workers {
                let evidence_before = chat.adaptive_effort.evidence_registry.len();
                chat.handle_server_notification(native_item_replay, None);
                chat.handle_server_notification(live_signal_replay, None);
                ensure!(
                    chat.adaptive_effort.evidence_registry.len() == evidence_before
                        && chat.adaptive_effort.workflow_terminal
                            == Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa),
                    "replayed Validator receipt/signal changed the authorized READY evidence"
                );
            }
        }
    }

    ensure!(run_ids.len() == 2 && run_ids[0] != run_ids[1]);
    ensure!(
        fixture_git(
            &root,
            &["status", "--porcelain=v1", "--untracked-files=all"]
        )?
        .is_empty(),
        "native validation modified the committed disposable candidate"
    );
    let current = chat
        .thread_id
        .context("missing final TUI thread identity")?;
    ensure!(
        Some(current) != origin_thread,
        "READY cannot belong to Implementation thread"
    );
    let mut persisted_count = 0;
    while let Ok(event) = app_events.try_recv() {
        if let crate::app_event::AppEvent::PersistWorkflowState {
            thread_id,
            operation: codex_app_server_protocol::ThreadWorkflowStateOperation::SetReadyForOwnerQa,
            ..
        } = event
        {
            ensure!(thread_id == current, "foreign thread persisted READY");
            persisted_count += 1;
        }
    }
    ensure!(
        persisted_count == 1,
        "TUI must persist owner READY exactly once on the genuine Validator"
    );
    timeout(LIVE_TIMEOUT, server.shutdown_gracefully()).await??;
    Ok(())
}

#[cfg(windows)]
#[tokio::test]
async fn codexdd_046_3d1i_real_ops_preview_independent_validator_live_qa_ready() -> Result<()> {
    use crate::adaptive_complexity::AdaptiveComplexityClass;
    use crate::adaptive_complexity::AdaptiveImplementationPhase;
    use crate::adaptive_worker::AdaptiveWorkerRole;
    use crate::adaptive_worker::AdaptiveWorkflowTerminal;
    use crate::adaptive_worker::NewWorkerBinding;
    use crate::app_event::AppEvent;
    use crate::chatwidget::tests::make_chatwidget_manual_with_sender;
    use std::net::TcpListener;

    let temp = TempDir::new()?;
    let root = temp.path().join("owned-preview");
    let scripts = root.join("scripts");
    std::fs::create_dir_all(&scripts)?;
    let stop_marker = temp.path().join("stop-d02-service");
    let reserved = TcpListener::bind(("127.0.0.1", 0))?;
    let port = reserved.local_addr()?.port();
    drop(reserved);
    let pwsh = Path::new(&std::env::var("ProgramFiles")?)
        .join("PowerShell")
        .join("7")
        .join("pwsh.exe");
    ensure!(pwsh.is_file(), "native Windows PowerShell 7 is required");
    let script = scripts.join("owned-health.ps1");
    std::fs::write(&script, d02_native_service_script(port, &stop_marker))?;
    std::fs::write(
        scripts.join("codexdd-test-workpacket.ps1"),
        one_stage_native_script("work-packet", "d02-owned-preview-source"),
    )?;
    std::fs::write(
        scripts.join("codexdd-test-independent.ps1"),
        one_stage_native_script("independent-review", "d02-distinct-validator"),
    )?;
    fixture_git(&root, &["init", "-q"])?;
    fixture_git(&root, &["add", "scripts"])?;
    fixture_git(
        &root,
        &[
            "-c",
            "user.name=CodexDD D02 QA fixture",
            "-c",
            "user.email=codexdd@example.test",
            "commit",
            "--no-gpg-sign",
            "-q",
            "-m",
            "one source-pinned Core-owned preview",
        ],
    )?;
    let sha = fixture_git(&root, &["rev-parse", "HEAD"])?;
    let canonical = root.canonicalize()?;
    let scope = "D02 one owned preview, separate Validator and fresh QA";
    let command = format!("& '{}'", script.display().to_string().replace('\'', "''"));
    let argv = [
        pwsh.display().to_string(),
        "-NoProfile".to_string(),
        "-Command".to_string(),
        command.clone(),
    ];
    let command_argv = serde_json::to_string(&argv)?;
    let owner_message = format!(
        "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"{scope}\"\nsource_root = '{}'\nsource_sha = \"{sha}\"\ntask_kind = \"operations_preview\"\nrequired_gates = [{{ kind = \"owned_runtime_health\", ports = [{port}], command_argv = {command_argv} }}, {{ kind = \"independent_validation\" }}]\n\nRun this one original approved disposable service.",
        canonical.display(),
    );
    ensure!(
        codex_config::config_toml::parse_adaptive_worker_first_message(&owner_message)
            .map_err(anyhow::Error::msg)?
            .is_some(),
        "original owner health permission must be parsed and locked"
    );

    let mut sequence = vec![
        create_final_assistant_message_sse_response("Bounded owner accepted.")?,
        responses::sse(vec![
            responses::ev_response_created("d02-real-launch"),
            responses::ev_function_call(
                "d02-real-launch-call",
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
            responses::ev_completed("d02-real-launch"),
        ]),
        create_final_assistant_message_sse_response("Original service launched.")?,
    ];
    for (role, profile, tool, call, signal) in [
        (
            "implementation",
            "work_packet",
            "run_codexdd_validation",
            "d02-native-source-receipt",
            "ready_for_validation",
        ),
        (
            "validation",
            "independent_review",
            "run_codexdd_independent_validation",
            "d02-native-independent-receipt",
            "ready_for_owner_qa",
        ),
    ] {
        if role == "validation" {
            sequence.push(create_final_assistant_message_sse_response(
                "Separate Validation Worker bound.",
            )?);
        }
        sequence.push(responses::sse(vec![
            responses::ev_response_created(&format!("d02-{role}-native")),
            responses::ev_function_call(call, tool, &json!({"profile": profile}).to_string()),
            responses::ev_completed(&format!("d02-{role}-native")),
        ]));
        sequence.push(responses::sse(vec![
            responses::ev_response_created(&format!("d02-{role}-signal")),
            responses::ev_function_call(
                &format!("{call}-signal"),
                "report_adaptive_signal",
                &json!({
                    "kind": signal,
                    "evidence_refs": [call],
                    "diagnostic_note": "native D02 exact owner QA",
                })
                .to_string(),
            ),
            responses::ev_completed(&format!("d02-{role}-signal")),
        ]));
        sequence.push(create_final_assistant_message_sse_response("done")?);
    }
    let backend = create_mock_responses_server_sequence_unchecked(sequence).await;
    let home = TempDir::new()?;
    MockResponsesConfig::new(&backend.uri()).write(home.path())?;
    let mut server = TestAppServer::builder()
        .with_codex_home(home.path())
        .without_auto_env()
        .build_initialized()
        .await?;
    let (mut chat, _sender, mut app_events, _op_rx) = make_chatwidget_manual_with_sender().await;
    let mut origin_thread = None;
    let mut validator_thread = None;
    let mut last_validation_turn = None;
    for (role, profile, call_id) in [
        ("implementation", "work_packet", "d02-native-source-receipt"),
        (
            "validation",
            "independent_review",
            "d02-native-independent-receipt",
        ),
    ] {
        let thread_start_id = server
            .send_thread_start_request(ThreadStartParams {
                model: Some("mock-model".to_string()),
                cwd: Some(canonical.display().to_string()),
                history_mode: Some(ThreadHistoryMode::Legacy),
                approval_policy: Some(AskForApproval::Never),
                // Only disposable temp Git source and loopback CI service.
                sandbox: Some(SandboxMode::DangerFullAccess),
                ..Default::default()
            })
            .await?;
        let started: ThreadStartResponse =
            timeout(LIVE_TIMEOUT, server.read_response(thread_start_id)).await??;
        let thread_id = started.thread.id;
        let thread = ThreadId::from_string(&thread_id)?;
        chat.thread_id = Some(thread);
        if role == "implementation" {
            chat.dispatch_adaptive_command("astra");
            origin_thread = Some(thread);
        } else {
            let parent = origin_thread.context("original Core Implementation missing")?;
            ensure!(
                thread != parent,
                "Validator must run in separate Core thread"
            );
            validator_thread = Some(thread);
        }
        let header = if role == "implementation" {
            owner_message.clone()
        } else {
            format!(
                "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"{scope}\"\nsource_root = '{}'\nsource_sha = \"{sha}\"\ntask_kind = \"operations_preview\"\n\nIndependently validate only the source-pinned original owner preview.",
                canonical.display(),
            )
        };
        let bind_id = server
            .send_turn_start_request(TurnStartParams {
                thread_id: thread_id.clone(),
                input: vec![UserInput::Text {
                    text: header,
                    text_elements: Vec::new(),
                }],
                ..Default::default()
            })
            .await?;
        let bound: TurnStartResponse =
            timeout(LIVE_TIMEOUT, server.read_response(bind_id)).await??;
        let first_item = take_item_event(&mut server, &thread_id, &bound.turn.id, None).await?;
        chat.handle_server_notification(first_item, None);
        ensure!(
            chat.adaptive_effort.worker_context.role
                == if role == "implementation" {
                    AdaptiveWorkerRole::Implementation
                } else {
                    AdaptiveWorkerRole::Validation
                },
            "live first user message did not preserve Worker role"
        );
        let binding_done =
            take_turn_event(&mut server, "turn/completed", &thread_id, &bound.turn.id).await?;
        ensure!(
            matches!(binding_done, ServerNotification::TurnCompleted(_)),
            "original Worker header did not complete"
        );
        if role == "implementation" {
            ensure!(
                chat.adaptive_effort
                    .gate_ledger
                    .as_ref()
                    .is_some_and(|ledger| {
                        ledger
                            .outstanding(&chat.adaptive_effort.evidence_registry)
                            .len()
                            == 2
                    }),
                "original owner must have runtime plus independent pending gates"
            );
            let launch_id = server
                .send_turn_start_request(TurnStartParams {
                    thread_id: thread_id.clone(),
                    input: vec![UserInput::Text {
                        text: "Launch only the original owner-approved service.".to_string(),
                        text_elements: Vec::new(),
                    }],
                    ..Default::default()
                })
                .await?;
            let launched: TurnStartResponse =
                timeout(LIVE_TIMEOUT, server.read_response(launch_id)).await??;
            let done =
                take_turn_event(&mut server, "turn/completed", &thread_id, &launched.turn.id)
                    .await?;
            ensure!(
                matches!(done, ServerNotification::TurnCompleted(_)),
                "Core did not retain live approved service turn"
            );
            let health_request = server
                .send_request(
                    "thread/ownedRuntimeHealth/review",
                    Some(json!({"threadId": thread_id})),
                )
                .await?;
            let early: codex_app_server_protocol::ThreadOwnedRuntimeHealthReviewResponse =
                timeout(LIVE_TIMEOUT, server.read_response(health_request)).await??;
            ensure!(
                early.owner_pid > 0
                    && early.approved_port == port
                    && early.source_sha.eq_ignore_ascii_case(&sha),
                "native Core listener health did not match owner authorization"
            );
            ensure!(
                chat.adaptive_effort.workflow_terminal.is_none(),
                "Core-owned HTTP health alone cannot grant QA READY"
            );
            chat.adaptive_effort.complexity_class = Some(AdaptiveComplexityClass::Standard);
            chat.adaptive_effort.implementation_phase =
                Some(AdaptiveImplementationPhase::MechanicalValidation);
        }
        let native_id = server
            .send_turn_start_request(TurnStartParams {
                thread_id: thread_id.clone(),
                input: vec![UserInput::Text {
                    text: format!("Run only original {role} {profile} native validation."),
                    text_elements: Vec::new(),
                }],
                turn_trigger: (role == "implementation").then(|| {
                    codex_protocol::CODEXDD_ADAPTIVE_MECHANICAL_VALIDATION_TURN_TRIGGER.to_string()
                }),
                ..Default::default()
            })
            .await?;
        let native: TurnStartResponse =
            timeout(LIVE_TIMEOUT, server.read_response(native_id)).await??;
        let started =
            take_turn_event(&mut server, "turn/started", &thread_id, &native.turn.id).await?;
        chat.handle_server_notification(started, None);
        let item = take_item_event(&mut server, &thread_id, &native.turn.id, Some(call_id)).await?;
        let ServerNotification::ItemCompleted(ref receipt) = item else {
            anyhow::bail!("expected actual Core-native tool completion");
        };
        let ThreadItem::DynamicToolCall {
            arguments,
            success,
            status,
            ..
        } = &receipt.item
        else {
            anyhow::bail!("native validation must emit dynamic Core tool receipt");
        };
        ensure!(
            *success == Some(true)
                && *status == codex_app_server_protocol::DynamicToolCallStatus::Completed
                && arguments["profile"] == profile
                && arguments["result"] == "pass"
                && arguments["head_sha"] == sha,
            "independent/native Worker's real Core validation receipt failed"
        );
        chat.handle_server_notification(item, None);
        let signal = take_trusted_signal(&mut server, &thread_id, &native.turn.id).await?;
        chat.handle_server_notification(signal, None);
        let completed =
            take_turn_event(&mut server, "turn/completed", &thread_id, &native.turn.id).await?;
        chat.handle_server_notification(completed, None);
        if role == "implementation" {
            ensure!(
                chat.adaptive_effort.workflow_terminal
                    == Some(AdaptiveWorkflowTerminal::ReadyForValidation),
                "Core-owned Implementation did not reach authorized independent handoff"
            );
            let review_binding = NewWorkerBinding {
                role: AdaptiveWorkerRole::Validation,
                authorized_scope: scope.to_string(),
            };
            let fresh = chat.fresh_adaptive_effort_for_new_worker(Some(&review_binding));
            ensure!(
                fresh.review_origin_thread_id == origin_thread
                    && fresh.gate_ledger.is_some()
                    && fresh.assignment_preflight.is_some(),
                "original Core owner identity was not transferred to Validator"
            );
            chat.adaptive_effort = fresh;
            chat.turn_lifecycle.agent_turn_running = false;
            chat.turn_lifecycle.last_turn_id = None;
        } else {
            last_validation_turn = Some(native.turn.id.clone());
            ensure!(
                chat.adaptive_effort.workflow_terminal.is_none(),
                "Validator bypassed pending live Core health QA"
            );
            let pending = chat
                .adaptive_effort
                .pending_owned_runtime_qa
                .as_ref()
                .context("real independent Validator did not schedule live host QA recheck")?;
            ensure!(
                pending.origin_thread_id == origin_thread.context("missing origin")?
                    && pending.validation_thread_id
                        == validator_thread.context("missing Validator")?
                    && pending.approved_port == port
                    && pending.source_turn_id == native.turn.id,
                "pending QA approval is not source, port and turn bound"
            );
        }
    }

    let original = origin_thread.context("missing original Core Worker")?;
    let validator = validator_thread.context("missing independent Validator")?;
    let source_turn = last_validation_turn.context("missing actual Validator turn")?;
    let mut queued_review = None;
    let mut premature_ready = 0;
    while let Ok(event) = app_events.try_recv() {
        match event {
            AppEvent::ReviewOwnedRuntimeHealthAtQa {
                validation_thread_id,
                origin_thread_id,
                source_turn_id,
                review_request_id,
            } => {
                ensure!(
                    validation_thread_id == validator
                        && origin_thread_id == original
                        && source_turn_id == source_turn,
                    "host QA request escaped current independent Validator source turn"
                );
                ensure!(queued_review.replace(review_request_id).is_none());
            }
            AppEvent::PersistWorkflowState {
                operation:
                    codex_app_server_protocol::ThreadWorkflowStateOperation::SetReadyForOwnerQa,
                ..
            } => premature_ready += 1,
            _ => {}
        }
    }
    ensure!(
        premature_ready == 0,
        "model READY must not be persisted before Core recheck"
    );
    let review_id = queued_review.context("no trusted host QA request was scheduled")?;
    let result_request = server
        .send_request(
            "thread/ownedRuntimeHealth/review",
            Some(json!({"threadId": original.to_string()})),
        )
        .await?;
    let healthy: codex_app_server_protocol::ThreadOwnedRuntimeHealthReviewResponse =
        timeout(LIVE_TIMEOUT, server.read_response(result_request)).await??;
    chat.complete_owned_runtime_qa_review(
        validator,
        original,
        &source_turn,
        review_id,
        Ok(healthy.clone()),
    );
    ensure!(
        chat.adaptive_effort.workflow_terminal == Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa),
        "fresh Core-owned HTTP 200 plus genuine independent Validator did not reach READY"
    );
    ensure!(
        chat.adaptive_effort.pending_owned_runtime_qa.is_none(),
        "completed QA request must be consumed once and never saved"
    );
    chat.complete_owned_runtime_qa_review(
        validator,
        original,
        &source_turn,
        review_id,
        Ok(healthy),
    );
    let mut persisted_ready = 0;
    while let Ok(event) = app_events.try_recv() {
        if let AppEvent::PersistWorkflowState {
            operation: codex_app_server_protocol::ThreadWorkflowStateOperation::SetReadyForOwnerQa,
            thread_id,
            ..
        } = event
        {
            ensure!(thread_id == validator, "READY persisted to wrong Worker");
            persisted_ready += 1;
        }
    }
    ensure!(
        persisted_ready == 1,
        "authentic Core QA recheck must persist exactly one READY, never replay"
    );
    std::fs::write(&stop_marker, b"stop")?;
    ensure!(
        fixture_git(
            &root,
            &["status", "--porcelain=v1", "--untracked-files=all"]
        )?
        .is_empty(),
        "native QA process changed the pinned candidate"
    );
    timeout(LIVE_TIMEOUT, server.shutdown_gracefully()).await??;
    Ok(())
}
