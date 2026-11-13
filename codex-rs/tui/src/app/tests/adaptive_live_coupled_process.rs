//! 0.4.6 3C.1b.2j3b2b2b: source-matched, positive App-owned Validator
//! across a real app-server OS-process restart. All Core receipts and runtime
//! signals arrive from the actual remote WebSocket app-server, not test JSON.

use super::*;
use crate::adaptive_complexity::AdaptiveComplexityClass;
use crate::adaptive_complexity::AdaptiveImplementationPhase;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use crate::app_server_session::TurnPermissionsOverride;
use crate::app_server_session::real_process_restart_tests::start_real_app_server;
use crate::app_server_session::real_process_restart_tests::stop_real_app_server;
use crate::legacy_core::config::ConfigOverrides;
use color_eyre::eyre::Context;
use color_eyre::eyre::bail;
use color_eyre::eyre::ensure;
use app_test_support::MockResponsesConfig;
use app_test_support::create_final_assistant_message_sse_response;
use app_test_support::create_mock_responses_server_sequence_unchecked;
use codex_app_server_client::AppServerEvent;
use codex_app_server_protocol::AskForApproval;
use codex_app_server_protocol::ServerNotification;
use codex_app_server_protocol::ThreadItem;
use codex_app_server_protocol::UserInput;
use core_test_support::responses;
use serde_json::json;
use std::path::Path;
use std::time::Duration;
use tokio::time::timeout;

const LIVE_TIMEOUT: Duration = Duration::from_secs(90);

fn native_one_stage_script(profile: &str, stage: &str) -> String {
    format!(
        r#"[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"profile_begin","profile":"{profile}","stage_count":1}}')
[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"stage_begin","profile":"{profile}","stage":"{stage}","ordinal":1,"stage_count":1}}')
[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"stage_end","profile":"{profile}","stage":"{stage}","ordinal":1,"status":"pass","native_exit_code":0}}')
[Console]::Out.WriteLine('CODEXDD_VALIDATION_JSON {{"contract_version":1,"event":"profile_end","profile":"{profile}","status":"pass","exit_code":0,"completed_stages":1}}')
exit 0
"#
    )
}

fn fixture_git(root: &Path, args: &[&str]) -> Result<String> {
    let result = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()?;
    ensure!(
        result.status.success(),
        "disposable fixture git {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&result.stderr)
    );
    Ok(String::from_utf8(result.stdout)?.trim().to_string())
}

/// A test driver for the real TUI event multiplexer. The notification content
/// must originate from AppServerSession::next_event and is never authored here.
async fn drive_native_turn(
    app: &mut App,
    server: &mut AppServerSession,
    thread_id: ThreadId,
    turn_id: &str,
) -> Result<Vec<ServerNotification>> {
    let deadline = tokio::time::Instant::now() + LIVE_TIMEOUT;
    let mut delivered = Vec::new();
    let mut completed = false;
    for _ in 0..1000 {
        let next = tokio::time::timeout_at(deadline, server.next_event())
            .await
            .context("real WebSocket app-server turn event deadline")?
            .context("real app-server event stream ended")?;
        if let AppServerEvent::ServerNotification(note) = &next {
            if let ServerNotification::TurnCompleted(done) = note.as_ref() {
                completed = done.thread_id == thread_id.to_string() && done.turn.id == turn_id;
            }
            delivered.push(note.as_ref().clone());
        }
        app.handle_app_server_event(server, next).await;
        let mut buffered = Vec::new();
        if let Some(receiver) = app.active_thread_rx.as_mut() {
            while let Ok(event) = receiver.try_recv() {
                buffered.push(event);
            }
        }
        for event in buffered {
            app.handle_thread_event_now(event);
        }
        if completed {
            return Ok(delivered);
        }
    }
    bail!("real app-server stream exceeded bounded event count without completing turn")
}

async fn send_real_turn(
    server: &mut AppServerSession,
    thread_id: ThreadId,
    root: &Path,
    text: String,
    trigger: Option<String>,
    model: String,
) -> Result<String> {
    let response = server
        .turn_start(
            thread_id,
            uuid::Uuid::new_v4().to_string(),
            vec![UserInput::Text {
                text,
                text_elements: Vec::new(),
            }],
            trigger,
            root.to_path_buf(),
            Some(AskForApproval::Never),
            None,
            TurnPermissionsOverride::Preserve,
            &[],
            model,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    Ok(response.turn.id)
}

fn require_real_native_receipt(
    events: &[ServerNotification],
    thread: ThreadId,
    expected_profile: &str,
    expected_sha: &str,
) -> Result<String> {
    let mut matches = Vec::new();
    for notification in events {
        let ServerNotification::ItemCompleted(item) = notification else {
            continue;
        };
        if item.thread_id != thread.to_string() {
            continue;
        }
        let ThreadItem::DynamicToolCall {
            arguments,
            status,
            success,
            ..
        } = &item.item
        else {
            continue;
        };
        if arguments["profile"] != expected_profile {
            continue;
        }
        ensure!(
            *status == codex_app_server_protocol::DynamicToolCallStatus::Completed
                && *success == Some(true)
                && arguments["head_sha"] == expected_sha
                && arguments["result"] == "pass",
            "real Core receipt did not pass authorized native profile and SHA"
        );
        matches.push(
            arguments["run_id"]
                .as_str()
                .context("Core receipt missing authentic run ID")?
                .to_string(),
        );
    }
    ensure!(matches.len() == 1 && !matches[0].is_empty(), "expected one exact Core-native receipt");
    Ok(matches.remove(0))
}

#[tokio::test]
#[ignore = "requires isolated Windows real app-server binary and mock Core provider"]
async fn codexdd_046_3c1b2j3b2b2b_real_app_owned_native_validator_after_restart() -> Result<()> {
    let binary = std::path::PathBuf::from(
        std::env::var_os("CODEXDD_REAL_APP_SERVER_BIN")
            .context("real Windows app-server binary required")?,
    );
    ensure!(binary.is_file(), "missing real app-server Windows binary");

    let checkout = tempfile::tempdir()?;
    let root = checkout.path().join("source");
    let scripts = root.join("scripts");
    std::fs::create_dir_all(&scripts)?;
    std::fs::write(
        scripts.join("codexdd-test-workpacket.ps1"),
        native_one_stage_script("work-packet", "coupled-source"),
    )?;
    std::fs::write(
        scripts.join("codexdd-test-independent.ps1"),
        native_one_stage_script("independent-review", "coupled-independent"),
    )?;
    fixture_git(&root, &["init", "-q"])?;
    fixture_git(&root, &["add", "scripts/codexdd-test-workpacket.ps1", "scripts/codexdd-test-independent.ps1"])?;
    fixture_git(
        &root,
        &[
            "-c", "user.name=CodexDD coupled test",
            "-c", "user.email=codexdd@example.invalid",
            "commit", "-q", "--no-gpg-sign", "-m", "approved coupled native candidate",
        ],
    )?;
    let sha = fixture_git(&root, &["rev-parse", "HEAD"])?;
    ensure!(sha.len() == 40);
    let root = root.canonicalize()?;
    let scope = "0.4.6 App-owned real source-to-independent-Validator after restart";

    let mut sequence = Vec::new();
    for (role, profile, tool, call_id, signal_kind) in [
        ("implementation", "work_packet", "run_codexdd_validation", "coupled-source-call", "ready_for_validation"),
        ("validation", "independent_review", "run_codexdd_independent_validation", "coupled-independent-call", "ready_for_owner_qa"),
    ] {
        sequence.push(create_final_assistant_message_sse_response("Owner assignment accepted.")?);
        sequence.push(responses::sse(vec![
            responses::ev_response_created(&format!("{role}-native-profile")),
            responses::ev_function_call(call_id, tool, &json!({"profile": profile}).to_string()),
            responses::ev_completed(&format!("{role}-native-profile")),
        ]));
        sequence.push(responses::sse(vec![
            responses::ev_response_created(&format!("{role}-native-signal")),
            responses::ev_function_call(
                &format!("{call_id}-signal"),
                "report_adaptive_signal",
                &json!({
                    "kind": signal_kind,
                    "evidence_refs": [call_id],
                    "diagnostic_note": "bounded native owner acceptance",
                }).to_string(),
            ),
            responses::ev_completed(&format!("{role}-native-signal")),
        ]));
        sequence.push(create_final_assistant_message_sse_response("done")?);
    }
    let mock = create_mock_responses_server_sequence_unchecked(sequence).await;
    let home = tempfile::tempdir()?;
    MockResponsesConfig::new(&mock.uri())
        .with_sandbox_mode("danger-full-access")
        .write(home.path())?;
    let config = ConfigBuilder::default()
        .codex_home(home.path().to_path_buf())
        .harness_overrides(ConfigOverrides {
            cwd: Some(root.clone()),
            ..Default::default()
        })
        .build()
        .await?;
    let (mut server, child) = start_real_app_server(&binary, home.path(), &config).await?;
    let started = server.start_thread(&config).await?;
    let implementation = started.session.thread_id;
    let (mut app, mut app_events, _ops) = make_test_app_with_channels().await;
    app.config = config.clone();
    app.local_settings = crate::local_settings::LocalSettings::from(&config);
    app.launch_cwd = root.clone();
    let mut tui = crate::tui::test_support::make_test_tui()?;
    app.replace_chat_widget_with_app_server_thread(
        &mut tui,
        started,
        session_lifecycle::ThreadAttachPresentation::Fresh,
        None,
    )
    .await?;
    app.chat_widget.dispatch_adaptive_command("astra");
    while app_events.try_recv().is_ok() {}

    let owner_text = format!(
        "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"{scope}\"\nsource_root = '{}'\nsource_sha = \"{sha}\"\ntask_kind = \"source_change\"\nrequired_gates = [{{ kind = \"source_profile\", profile = \"work_packet\" }}, {{ kind = \"independent_validation\" }}]\nAuthorize this disposable source.",
        root.display()
    );
    let turn_id = send_real_turn(
        &mut server,
        implementation,
        &root,
        owner_text,
        None,
        config.model.clone().unwrap_or_else(|| "mock-model".to_string()),
    )
    .await?;
    let _original = drive_native_turn(&mut app, &mut server, implementation, &turn_id).await?;
    ensure!(
        app.chat_widget.adaptive_effort_for_test().worker_context.role
            == AdaptiveWorkerRole::Implementation,
        "actual initial user origin was not admitted"
    );
    ensure!(
        app.chat_widget
            .adaptive_effort_for_test()
            .gate_ledger
            .as_ref()
            .is_some_and(|ledger| {
                ledger.outstanding(
                    &app.chat_widget.adaptive_effort_for_test().evidence_registry
                ).len() == 2
            }),
        "first user source failed to create both pending owner gates"
    );
    // The earlier accepted workflow packets establish the complexity floor.
    // This fixture starts at their terminal MechanicalValidation boundary.
    app.chat_widget.adaptive_effort_for_test_mut().complexity_class =
        Some(AdaptiveComplexityClass::Standard);
    app.chat_widget.adaptive_effort_for_test_mut().implementation_phase =
        Some(AdaptiveImplementationPhase::MechanicalValidation);

    let turn_id = send_real_turn(
        &mut server,
        implementation,
        &root,
        "Execute only the authorized source work_packet native profile.".to_string(),
        Some(codex_protocol::CODEXDD_ADAPTIVE_MECHANICAL_VALIDATION_TURN_TRIGGER.to_string()),
        app.chat_widget.current_model(),
    )
    .await?;
    let events = drive_native_turn(&mut app, &mut server, implementation, &turn_id).await?;
    let source_run = require_real_native_receipt(&events, implementation, "work_packet", &sha)?;
    ensure!(
        app.chat_widget.adaptive_effort_for_test().workflow_terminal
            == Some(AdaptiveWorkflowTerminal::ReadyForValidation),
        "Core source native PASS and signal did not finish in the App ChatWidget"
    );
    ensure!(
        app.chat_widget
            .adaptive_effort_for_test()
            .gate_ledger
            .as_ref()
            .is_some_and(|ledger| {
                ledger.outstanding(
                    &app.chat_widget.adaptive_effort_for_test().evidence_registry
                ).len() == 1
            }),
        "real source PASS must close only source profile, not independent review"
    );

    let queued = std::iter::from_fn(|| app_events.try_recv().ok()).collect::<Vec<_>>();
    let mut handoffs = queued.into_iter().filter(|event| matches!(
        event, AppEvent::StartAutomaticValidation { .. }
    ));
    let handoff = handoffs.next().context("production App did not queue automatic Validator")?;
    ensure!(handoffs.next().is_none(), "duplicate automatic Validator event");
    if let AppEvent::StartAutomaticValidation {
        origin_thread_id,
        source_sha,
        ..
    } = &handoff {
        ensure!(*origin_thread_id == implementation && source_sha == &sha);
    }

    // The original native source process is genuinely torn down here; the
    // App retains the live, just-observed Core source proof in its own memory.
    stop_real_app_server(server, child).await?;
    let (mut restarted, restarted_child) =
        start_real_app_server(&binary, home.path(), &config).await?;
    app.handle_event(&mut tui, &mut restarted, handoff).await?;
    let validator = app.chat_widget.thread_id().context("App did not attach a new Validator")?;
    ensure!(validator != implementation, "source and Validator share a thread");
    ensure!(app.current_displayed_thread_id() == Some(validator));
    let fresh = app.chat_widget.adaptive_effort_for_test();
    ensure!(
        fresh.worker_context.role == AdaptiveWorkerRole::Validation
            && fresh.review_origin_thread_id == Some(implementation)
            && fresh.assignment_preflight.as_ref().is_some_and(|pin| pin.source_sha == sha)
            && fresh.workflow_terminal.is_none(),
        "App-owned Validator lost original source, role, or fresh pending terminal"
    );
    ensure!(
        fresh.gate_ledger.as_ref().is_some_and(|ledger| {
            ledger.outstanding(&fresh.evidence_registry).len() == 1
        }),
        "new Validator reused READY or lost the only pending independent gate"
    );
    let auto_events = std::iter::from_fn(|| app_events.try_recv().ok()).collect::<Vec<_>>();
    let mut initial = auto_events.into_iter().filter_map(|event| match event {
        AppEvent::CodexOp(Op::UserTurn { items, cwd, .. }) => Some((items, cwd)),
        AppEvent::SubmitThreadOp {
            thread_id,
            op: Op::UserTurn { items, cwd, .. },
        } if thread_id == validator => Some((items, cwd)),
        _ => None,
    });
    let (prompt, cwd) = initial.next().context("App did not submit fresh Validator prompt")?;
    ensure!(initial.next().is_none(), "App submitted more than one Validator first turn");
    ensure!(cwd.canonicalize()? == root, "App-created Validator selected wrong Git root");
    let assignment = send_real_turn(
        &mut restarted,
        validator,
        &root,
        match prompt.as_slice() {
            [UserInput::Text { text, .. }] => text.clone(),
            _ => bail!("App first Validator turn is not the generated text assignment"),
        },
        None,
        app.chat_widget.current_model(),
    )
    .await?;
    let _ = drive_native_turn(&mut app, &mut restarted, validator, &assignment).await?;
    let review_turn = send_real_turn(
        &mut restarted,
        validator,
        &root,
        "Run the authorized independent_review native profile and report receipt.".to_string(),
        None,
        app.chat_widget.current_model(),
    )
    .await?;
    let events = drive_native_turn(&mut app, &mut restarted, validator, &review_turn).await?;
    let validation_run =
        require_real_native_receipt(&events, validator, "independent_review", &sha)?;
    ensure!(source_run != validation_run, "source and independent native run IDs reused");
    let terminal = app.chat_widget.adaptive_effort_for_test();
    ensure!(
        terminal.workflow_terminal == Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa)
            && terminal.gate_ledger.as_ref().is_some_and(|ledger| {
                ledger.ready_for_owner_qa(&terminal.evidence_registry)
            }),
        "actual independent Core-native receipt did not authorize App owner READY"
    );
    let mut ready_events = Vec::new();
    for event in std::iter::from_fn(|| app_events.try_recv().ok()) {
        if matches!(
            &event,
            AppEvent::PersistWorkflowState {
                operation: codex_app_server_protocol::ThreadWorkflowStateOperation::SetReadyForOwnerQa,
                ..
            }
        ) {
            ready_events.push(event);
        }
    }
    ensure!(
        ready_events.len() == 1,
        "App must persist exactly one owner READY on the separate native Validator"
    );
    app.handle_event(&mut tui, &mut restarted, ready_events.remove(0))
        .await?;
    ensure!(app.current_displayed_thread_id() == Some(validator));
    stop_real_app_server(restarted, restarted_child).await?;
    Ok(())
}
