//! 0.4.6 3C.1b.2j3b2b2a: a real app-server OS restart must not
//! recover App-owned automatic Validator or owner READY from old TUI state.
//! This is the cold-App negative seam; authentic Core native receipts across
//! the restart were separately accepted in 3C.1b.2j3b2b1.

use super::*;
use crate::adaptive_complexity::AdaptiveComplexityClass;
use crate::adaptive_complexity::AdaptiveImplementationPhase;
use crate::adaptive_gate_prerequisites::AdaptiveGateLedger;
use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;
use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveFamily;
use crate::adaptive_worker::AdaptiveTaskKind;
use crate::adaptive_worker::AdaptiveWorkerContext;
use crate::adaptive_worker::AdaptiveWorkerPreflight;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use crate::adaptive_worker::NewWorkerBinding;
use crate::app_server_session::ResumeModelSettings;
use crate::app_server_session::real_process_restart_tests::start_real_app_server;
use crate::app_server_session::real_process_restart_tests::stop_real_app_server;
use crate::chatwidget::adaptive_effort::AdaptiveEffortState;
use crate::legacy_core::config::ConfigBuilder;
use app_test_support::create_fake_rollout;
use codex_app_server_client::AppServerEvent;
use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;
use codex_app_server_protocol::AdaptiveRuntimeSignalNotification;
use codex_app_server_protocol::ThreadWorkflowStateOperation;
use codex_protocol::protocol::AdaptiveRuntimeSignalKind;
use std::path::Path;
use std::time::Duration;

fn fixture_git(root: &Path, args: &[&str]) -> Result<String> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()?;
    assert!(
        output.status.success(),
        "disposable App process fixture Git failure: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

#[tokio::test]
#[ignore = "requires Windows real app-server binary and isolated disposable fixture"]
async fn codexdd_046_3c1b2j3b2b2a_cold_real_process_app_rejects_old_auto_handoff() -> Result<()> {
    let binary = std::path::PathBuf::from(
        std::env::var_os("CODEXDD_REAL_APP_SERVER_BIN")
            .ok_or_else(|| color_eyre::eyre::eyre!("missing real Windows app-server executable"))?,
    );
    assert!(
        binary.is_file(),
        "native Windows app-server executable required"
    );

    let home = tempfile::tempdir()?;
    let source = tempfile::tempdir()?;
    let root = source.path().canonicalize()?;
    fixture_git(&root, &["init", "-q"])?;
    fixture_git(
        &root,
        &[
            "-c",
            "user.name=CodexDD cold App test",
            "-c",
            "user.email=codexdd@example.invalid",
            "commit",
            "--allow-empty",
            "--no-gpg-sign",
            "-q",
            "-m",
            "owner source SHA",
        ],
    )?;
    let sha = fixture_git(&root, &["rev-parse", "HEAD"])?;
    let scope = "3C.1b.2j3b2b2a original implementation owner";
    let config = ConfigBuilder::default()
        .codex_home(home.path().to_path_buf())
        .build()
        .await?;
    let thread = ThreadId::from_string(
        &create_fake_rollout(
            home.path(),
            "2025-01-06T12-00-00",
            "2025-01-06T12:00:00Z",
            "Saved source Worker terminal to be refused after cold restart",
            Some(config.model_provider_id.as_str()),
            None,
        )
        .map_err(|error| color_eyre::eyre::eyre!("rollout fixture: {error}"))?,
    )?;
    let mut saved = AdaptiveEffortState {
        enabled: true,
        starting_family: Some(AdaptiveFamily::Astra),
        current_family: Some(AdaptiveFamily::Sol),
        current_effort: Some(AdaptiveEffort::High),
        attempt_number: 1,
        complexity_class: Some(AdaptiveComplexityClass::Standard),
        implementation_phase: Some(AdaptiveImplementationPhase::MechanicalValidation),
        worker_context: AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Implementation,
            authorized_scope: Some(scope.to_owned()),
        },
        worker_assignment_locked: true,
        assignment_preflight: Some(AdaptiveWorkerPreflight {
            task_kind: AdaptiveTaskKind::SourceChange,
            source_root: root.display().to_string(),
            source_sha: sha.clone(),
        }),
        workflow_terminal: Some(AdaptiveWorkflowTerminal::ReadyForValidation),
        ..Default::default()
    };
    saved.gate_ledger = Some(
        AdaptiveGateLedger::new(
            scope,
            &root,
            &sha,
            vec![
                AdaptiveRequiredGate::SourceProfile("work_packet".to_owned()),
                AdaptiveRequiredGate::IndependentValidation,
            ],
        )
        .map_err(|error| color_eyre::eyre::eyre!("invalid owner gate fixture: {error}"))?,
    );
    let checkpoint = saved.durable_workflow_snapshot(thread);
    assert!(checkpoint.gate_checkpoint.is_some());

    let local_settings = crate::local_settings::LocalSettings::from(&config);
    let (mut first, first_child) = start_real_app_server(&binary, home.path(), &config).await?;
    first
        .resume_thread(
            &local_settings,
            config.clone(),
            thread,
            ResumeModelSettings::RestoreFromThread,
        )
        .await?;
    first
        .thread_workflow_state_update(
            thread,
            ThreadWorkflowStateOperation::SetAdaptiveState,
            Some("historical-source-ready".to_string()),
            Some(checkpoint),
        )
        .await?;
    stop_real_app_server(first, first_child).await?;

    let (mut restarted, restarted_child) =
        start_real_app_server(&binary, home.path(), &config).await?;
    let resumed = restarted
        .resume_thread(
            &local_settings,
            config,
            thread,
            ResumeModelSettings::RestoreFromThread,
        )
        .await?;
    std::assert_eq!(resumed.session.thread_id, thread);
    std::assert_eq!(
        resumed.session.adaptive_effort.worker_context.role,
        AdaptiveWorkerRole::Implementation
    );
    assert!(
        resumed.session.adaptive_effort.workflow_terminal.is_none()
            && resumed.session.adaptive_effort.paused_by_user,
        "cold real-process resume cannot restore historical Implementation READY"
    );
    std::assert_eq!(
        resumed
            .session
            .adaptive_effort
            .gate_ledger
            .as_ref()
            .expect("original gates remain pending")
            .outstanding(&resumed.session.adaptive_effort.evidence_registry)
            .len(),
        2
    );

    // Attach this real remote process's mapped state to the actual App thread
    // event dispatcher, not a direct ChatWidget-only test.
    let (mut app, mut app_events, _ops) = make_test_app_with_channels().await;
    app.enqueue_primary_thread_session(resumed.session, resumed.turns)
        .await?;
    std::assert_eq!(app.current_displayed_thread_id(), Some(thread));
    std::assert_eq!(app.chat_widget.thread_id(), Some(thread));
    assert!(
        app.chat_widget
            .adaptive_effort_for_test()
            .workflow_terminal
            .is_none()
    );
    while app_events.try_recv().is_ok() {}

    // An old Core terminal signal is not authority for a newly loaded App
    // session. Deliver through the same foreground thread-buffer multiplexer.
    let stale = ServerNotification::AdaptiveRuntimeSignal(AdaptiveRuntimeSignalNotification {
        thread_id: thread.to_string(),
        signal: AdaptiveRuntimeSignalEnvelope {
            source_turn_id: "historical-source-ready".to_owned(),
            signal_kind: AdaptiveRuntimeSignalKind::ReadyForValidation,
            evidence_refs: vec!["prior-source-run".to_string()],
            diagnostic_note: None,
        },
    });
    app.handle_app_server_event(
        &restarted,
        AppServerEvent::ServerNotification(Box::new(stale)),
    )
    .await;
    let event = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Some(receiver) = app.active_thread_rx.as_mut()
                && let Ok(event) = receiver.try_recv()
            {
                break event;
            }
            tokio::task::yield_now().await;
        }
    })
    .await?;
    assert!(
        matches!(&event, ThreadBufferedEvent::Notification(notification)
            if matches!(notification.as_ref(), ServerNotification::AdaptiveRuntimeSignal(_))),
        "App foreground buffer changed or lost the old source notification"
    );
    app.handle_thread_event_now(event);
    assert!(
        app.chat_widget
            .adaptive_effort_for_test()
            .workflow_terminal
            .is_none(),
        "a historical source signal must not reauthorize the source Worker"
    );
    assert!(
        app.chat_widget
            .adaptive_effort_for_test()
            .evidence_registry
            .len()
            == 0,
        "old signal created imaginary native evidence"
    );

    let mut tui = crate::tui::test_support::make_test_tui()?;
    let binding = NewWorkerBinding {
        role: AdaptiveWorkerRole::Validation,
        authorized_scope: scope.to_string(),
    };
    app.handle_event(
        &mut tui,
        &mut restarted,
        AppEvent::StartAutomaticValidation {
            origin_thread_id: thread,
            source_sha: sha.clone(),
            worker_binding: binding.clone(),
        },
    )
    .await?;
    std::assert_eq!(app.current_displayed_thread_id(), Some(thread));
    assert!(
        app.chat_widget
            .adaptive_effort_for_test()
            .workflow_terminal
            .is_none(),
        "App must not resurrect old source terminal through queued Validator handoff"
    );

    fixture_git(
        &root,
        &[
            "-c",
            "user.name=CodexDD cold App test",
            "-c",
            "user.email=codexdd@example.invalid",
            "commit",
            "--allow-empty",
            "--no-gpg-sign",
            "-q",
            "-m",
            "unapproved new HEAD",
        ],
    )?;
    assert_ne!(fixture_git(&root, &["rev-parse", "HEAD"])?, sha);
    app.handle_event(
        &mut tui,
        &mut restarted,
        AppEvent::StartAutomaticValidation {
            origin_thread_id: thread,
            source_sha: sha,
            worker_binding: binding,
        },
    )
    .await?;
    std::assert_eq!(app.current_displayed_thread_id(), Some(thread));
    assert!(
        app.chat_widget
            .adaptive_effort_for_test()
            .workflow_terminal
            .is_none(),
        "changed Git HEAD must not authorize a cold queued Validator"
    );
    while let Ok(event) = app_events.try_recv() {
        assert!(
            !matches!(
                event,
                AppEvent::PersistWorkflowState {
                    operation: ThreadWorkflowStateOperation::SetReadyForOwnerQa,
                    ..
                }
            ),
            "cold replay must not request owner READY persistence"
        );
    }
    stop_real_app_server(restarted, restarted_child).await?;
    Ok(())
}
