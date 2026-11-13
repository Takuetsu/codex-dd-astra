//! 0.4.6 3C.1b.2j3b2a — real OS app-server child, not an embedded runtime.
//! Source and gate authority must not be synthesized from an earlier READY.
//! The full genuine two-Worker/native-receipt App lifecycle remains a later gate.

use super::*;
use crate::adaptive_gate_prerequisites::AdaptiveGateLedger;
use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;
use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveFamily;
use crate::adaptive_worker::AdaptiveTaskKind;
use crate::adaptive_worker::AdaptiveWorkerContext;
use crate::adaptive_worker::AdaptiveWorkerPreflight;
use crate::adaptive_worker::AdaptiveWorkerRole;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use crate::chatwidget::adaptive_effort::AdaptiveEffortState;
use crate::legacy_core::config::ConfigBuilder;
use app_test_support::create_fake_rollout;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Child;
use tokio::process::Command;

fn git(root: &Path, args: &[&str]) -> Result<String> {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()?;
    assert!(
        output.status.success(),
        "disposable Git fixture failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

/// The test owns each child process and uses the public websocket client
/// transport exactly as the running TUI would. Do not replace this with an
/// embedded app-server: that separate evidence is 3C.1b.2j3b1.
pub(crate) async fn start_real_app_server(
    binary: &Path,
    codex_home: &Path,
    config: &Config,
) -> Result<(AppServerSession, Child)> {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
    let port = listener.local_addr()?.port();
    drop(listener);
    // The server --listen parser accepts a socket address, not a URL path.
    let url = format!("ws://127.0.0.1:{port}");
    let target = crate::AppServerTarget::Remote {
        endpoint: crate::resolve_remote_addr(&url)?,
    };
    let mut child = Command::new(binary)
        .arg("--listen")
        .arg(&url)
        .arg("--disable-plugin-startup-tasks-for-tests")
        .current_dir(codex_home)
        .env("CODEX_HOME", codex_home)
        .env("CODEX_APP_SERVER_DISABLE_MANAGED_CONFIG", "1")
        .env("RUST_LOG", "warn")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()?;
    let client = tokio::time::timeout(Duration::from_secs(40), async {
        loop {
            if let Some(status) = child.try_wait()? {
                color_eyre::eyre::bail!("real app-server exited before connection: {status}");
            }
            if let Ok(client) = crate::app_server_connection::connect(&target).await {
                break Ok::<_, color_eyre::Report>(client);
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    })
    .await??;
    Ok((
        AppServerSession::new(client, ThreadParamsMode::Remote)
            .with_local_codex_home(&config.codex_home),
        child,
    ))
}

async fn stop_real_app_server(session: AppServerSession, mut child: Child) -> Result<()> {
    session.shutdown().await?;
    if child.try_wait()?.is_none() {
        child.kill().await?;
    }
    Ok(())
}

#[tokio::test]
#[ignore = "requires GitHub Windows real-process binary and isolated CODEXDD_REAL_APP_SERVER_BIN"]
async fn codexdd_046_3c1b2j3b2a_real_os_restart_does_not_resurrect_ready_or_native_pass()
-> Result<()> {
    let executable = std::env::var_os("CODEXDD_REAL_APP_SERVER_BIN").ok_or_else(|| {
        color_eyre::eyre::eyre!("real-process CI must set CODEXDD_REAL_APP_SERVER_BIN")
    })?;
    let executable = std::path::PathBuf::from(executable);
    assert!(
        executable.is_file(),
        "real-process CI must build codex-app-server.exe before this test"
    );
    let home = tempfile::tempdir()?;
    let source = tempfile::tempdir()?;
    let source_root = source.path().canonicalize()?;
    git(&source_root, &["init", "-q"])?;
    git(
        &source_root,
        &[
            "-c",
            "user.name=CodexDD real-process test",
            "-c",
            "user.email=codexdd@example.invalid",
            "commit",
            "--allow-empty",
            "--no-gpg-sign",
            "-q",
            "-m",
            "approved source SHA",
        ],
    )?;
    let approved_sha = git(&source_root, &["rev-parse", "HEAD"])?;
    let config = ConfigBuilder::default()
        .codex_home(home.path().to_path_buf())
        .build()
        .await?;
    let thread_id = ThreadId::from_string(
        &create_fake_rollout(
            home.path(),
            "2025-01-05T12-00-00",
            "2025-01-05T12:00:00Z",
            "Saved independent Validator assignment",
            Some(config.model_provider_id.as_str()),
            /*git_info*/ None,
        )
        .map_err(|error| color_eyre::eyre::eyre!("real-process rollout fixture: {error}"))?,
    )?;
    let scope = "j3b2a source-pinned real-process Validator";
    let mut prior = AdaptiveEffortState {
        enabled: true,
        starting_family: Some(AdaptiveFamily::Astra),
        current_family: Some(AdaptiveFamily::Sol),
        current_effort: Some(AdaptiveEffort::High),
        attempt_number: 1,
        worker_context: AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: Some(scope.to_string()),
        },
        worker_assignment_locked: true,
        assignment_preflight: Some(AdaptiveWorkerPreflight {
            task_kind: AdaptiveTaskKind::SourceChange,
            source_root: source_root.display().to_string(),
            source_sha: approved_sha.clone(),
        }),
        workflow_terminal: Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa),
        ..Default::default()
    };
    prior.gate_ledger = Some(
        AdaptiveGateLedger::new(
            scope,
            &source_root,
            &approved_sha,
            vec![
                AdaptiveRequiredGate::SourceProfile("work_packet".to_owned()),
                AdaptiveRequiredGate::IndependentValidation,
            ],
        )
        .map_err(|error| color_eyre::eyre::eyre!("invalid real-process gate fixture: {error}"))?,
    );
    let checkpoint = prior.durable_workflow_snapshot(thread_id);
    assert!(
        checkpoint.gate_checkpoint.is_some(),
        "the original owner requirements must exist in the persisted record"
    );

    let settings = crate::local_settings::LocalSettings::from(&config);
    let (mut first, first_child) = start_real_app_server(&executable, home.path(), &config).await?;
    first
        .resume_thread(
            &settings,
            config.clone(),
            thread_id,
            ResumeModelSettings::RestoreFromThread,
        )
        .await?;
    first
        .thread_workflow_state_update(
            thread_id,
            ThreadWorkflowStateOperation::SetAdaptiveState,
            Some("previous-validator-owner-qa".to_owned()),
            Some(checkpoint),
        )
        .await?;
    stop_real_app_server(first, first_child).await?;

    let (mut cold, cold_child) = start_real_app_server(&executable, home.path(), &config).await?;
    let resumed = cold
        .resume_thread(
            &settings,
            config.clone(),
            thread_id,
            ResumeModelSettings::RestoreFromThread,
        )
        .await?;
    let state = &resumed.session.adaptive_effort;
    std::assert_eq!(resumed.session.thread_id, thread_id);
    std::assert_eq!(state.worker_context.role, AdaptiveWorkerRole::Validation);
    std::assert_eq!(
        state.worker_context.authorized_scope.as_deref(),
        Some(scope)
    );
    std::assert_eq!(
        state
            .assignment_preflight
            .as_ref()
            .expect("source pin from saved Validator")
            .source_sha,
        approved_sha
    );
    assert!(
        state.workflow_terminal.is_none() && state.paused_by_user,
        "old READY must never be reauthorized by process restart"
    );
    assert!(state.pending_signal.is_none());
    assert!(state.successor_admission.is_none());
    assert!(state.review_origin_thread_id.is_none());
    std::assert_eq!(state.evidence_registry.len(), 0);
    let pending = state
        .gate_ledger
        .as_ref()
        .expect("owner requirements must remain pending after cold restart")
        .outstanding(&state.evidence_registry);
    std::assert_eq!(pending.len(), 2);
    stop_real_app_server(cold, cold_child).await?;

    // A fresh OS process may display the old pin for historical inspection,
    // but cannot repin a new HEAD or admit an old native validation receipt.
    git(
        &source_root,
        &[
            "-c",
            "user.name=CodexDD real-process test",
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
    assert_ne!(git(&source_root, &["rev-parse", "HEAD"])?, approved_sha);
    let (mut drifted, drifted_child) =
        start_real_app_server(&executable, home.path(), &config).await?;
    let stale = drifted
        .resume_thread(
            &settings,
            config.clone(),
            thread_id,
            ResumeModelSettings::RestoreFromThread,
        )
        .await?;
    assert!(stale.session.adaptive_effort.workflow_terminal.is_none());
    std::assert_eq!(stale.session.adaptive_effort.evidence_registry.len(), 0);
    assert!(
        crate::adaptive_worker::preflight_mechanical_target(
            stale.session.adaptive_effort.assignment_preflight.as_ref(),
            &source_root,
        )
        .is_err(),
        "a new OS process must refuse validation against a changed Git HEAD"
    );
    stop_real_app_server(drifted, drifted_child).await?;

    std::fs::remove_dir_all(&source_root)?;
    let (mut missing, missing_child) =
        start_real_app_server(&executable, home.path(), &config).await?;
    assert!(
        missing
            .resume_thread(
                &settings,
                config,
                thread_id,
                ResumeModelSettings::RestoreFromThread,
            )
            .await
            .is_err(),
        "missing source root must not recover a trusted Validator gate checkpoint"
    );
    stop_real_app_server(missing, missing_child).await?;
    Ok(())
}
