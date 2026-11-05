//! NONRELEASE 3F.95: exercise the real TUI reconnect path across two actual
//! codex-app-server operating-system processes, without the managed updater.
//!
//! Unlike the 3F.90 loopback fixture, this test uses the production app-server
//! executable, a persisted rollout and a crash/replacement process boundary.
//! The local test socket is declared as a remote fixture transport so that
//! this test cannot be mistaken for 3F.94's privileged local daemon supervisor.

use super::*;
use crate::app::reconnect::ReconnectPresentation;
use crate::app::reconnect::reconnect;
use crate::app_server_session::ThreadParamsMode;
use codex_app_server_protocol::ThreadLoadedListParams;
use pretty_assertions::assert_eq;
use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Child;
use tokio::process::Command;
use tokio::time::Duration;
use tokio::time::sleep;
use tokio::time::timeout;

fn spawn_real_app_server(home: &Path, socket: &Path) -> Result<Child> {
    let binary = std::env::var_os("CODEXDD_3F95_APP_SERVER_BIN")
        .map(PathBuf::from)
        .ok_or_else(|| {
            color_eyre::eyre::eyre!(
                "3F.95 requires CODEXDD_3F95_APP_SERVER_BIN pointing to a compiled codex-app-server"
            )
        })?;
    if !binary.is_file() {
        color_eyre::eyre::bail!("3F.95 compiled app-server binary is missing");
    }
    Ok(Command::new(binary)
        .args(["--listen", &format!("unix://{}", socket.display())])
        .arg(app_test_support::DISABLE_PLUGIN_STARTUP_TASKS_ARG)
        .arg("--managed-daemon")
        .env("CODEX_HOME", home)
        .env("CODEX_DAEMON_SHUTDOWN_SOCKET", socket)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()?)
}

#[tokio::test]
async fn real_daemon_process_restart_restores_tui_history_and_unsent_draft() -> Result<()> {
    let (mut app, mut events, mut ops) = make_test_app_with_channels().await;
    let home = app.config.codex_home.to_path_buf();
    let saved_message = "3F.95 persisted message from the real app-server process";
    let unsent_draft = "3F.95 unsent draft must not be submitted during reconnect";
    // All process and rollout files are confined to the test app's temporary CODEX_HOME.
    std::fs::write(
        home.join("config.toml"),
        r#"model = "gpt-5.2"
approval_policy = "never"
sandbox_mode = "read-only"
model_provider = "mock_provider"
[model_providers.mock_provider]
name = "3F.95 offline provider"
base_url = "http://127.0.0.1:1/v1"
wire_api = "responses"
request_max_retries = 0
stream_max_retries = 0
"#,
    )?;
    app.config.model = Some("gpt-5.2".into());
    let thread_id = ThreadId::from_string(&app_test_support::create_fake_rollout(
        &home,
        "2026-11-01T11-00-00",
        "2026-11-01T11:00:00Z",
        saved_message,
        Some("mock_provider"),
        /*git_info*/ None,
    )?)?;
    let control = home.join("3f95-control");
    std::fs::create_dir_all(&control)?;
    let socket = control.join("server.sock");
    let endpoint = crate::resolve_remote_addr(&format!("unix://{}", socket.display()))?;
    let target = AppServerTarget::Remote {
        endpoint: endpoint.clone(),
    };
    app.app_server_target = target.clone();

    let mut original = spawn_real_app_server(&home, &socket)?;
    let original_pid = original.id().expect("real original app-server PID");
    // Fail closed if the real app-server did not start. No fake WebSocket endpoint.
    let client = timeout(Duration::from_secs(20), async {
        loop {
            if let Ok(client) = crate::app_server_connection::connect(&target).await {
                return client;
            }
            sleep(Duration::from_millis(50)).await;
        }
    })
    .await?;
    let mut session = AppServerSession::new(client, ThreadParamsMode::Remote);
    let before = session
        .thread_read(thread_id, /*include_turns*/ true)
        .await?;
    assert!(
        format!("{:?}", before.turns).contains(saved_message),
        "3F.95 original real daemon did not read persisted conversation"
    );
    let initial_loaded = session
        .thread_loaded_list(ThreadLoadedListParams {
            cursor: None,
            limit: Some(100),
        })
        .await?;
    assert!(
        initial_loaded.data.is_empty(),
        "reading a saved rollout should not automatically resume a thread"
    );

    // A visible conversation with an unsent user draft survives a dead server.
    app.primary_thread_id = Some(thread_id);
    app.active_thread_id = Some(thread_id);
    let cached = test_thread_session(thread_id, app.config.cwd.to_path_buf());
    app.ensure_thread_channel(thread_id)
        .store
        .lock()
        .await
        .set_session(cached.clone(), Vec::new());
    app.chat_widget.handle_thread_session(cached);
    app.chat_widget
        .restore_user_message_to_composer(unsent_draft.to_string());
    app.store_active_thread_receiver().await;
    assert_eq!(app.chat_widget.composer_text_with_pending(), unsent_draft);

    original.kill().await?;
    let old_exit = original.wait().await?;
    assert!(
        !old_exit.success(),
        "3F.95 original was not forcibly terminated"
    );
    assert!(app.begin_reconnect());
    assert!(app.reconnect.offline);
    let draft_offline = app.chat_widget.composer_text_with_pending();
    assert!(
        draft_offline.contains(unsent_draft),
        "the real-process disconnection lost unsent TUI input"
    );

    let mut successor = spawn_real_app_server(&home, &socket)?;
    let successor_pid = successor.id().expect("real successor app-server PID");
    assert_ne!(
        original_pid, successor_pid,
        "restart must replace the OS PID"
    );
    let mut tui = crate::tui::test_support::make_test_tui()?;
    let connected = timeout(
        Duration::from_secs(60),
        reconnect(
            target,
            app.config.clone(),
            app.local_settings.clone(),
            Some(thread_id),
            /*remote_cwd*/ None,
            session.thread_tool_transport(),
            ReconnectPresentation::Conversation,
        ),
    )
    .await??;
    app.finish_reconnect(
        &mut tui,
        &mut session,
        &mut events,
        connected,
        CODEX_CLI_VERSION,
    )
    .await?;

    assert!(
        !app.reconnect.offline,
        "TUI remained offline after real restart"
    );
    assert_eq!(app.current_displayed_thread_id(), Some(thread_id));
    assert_eq!(
        app.chat_widget.composer_text_with_pending(),
        unsent_draft,
        "TUI must preserve the unsent draft, not replay it as a new turn"
    );
    let history = drain_history(&mut app, &mut tui, &mut session, &mut events).await?;
    assert!(
        history.contains(saved_message),
        "TUI did not render persisted history from successor process: {history}"
    );
    assert!(
        !std::iter::from_fn(|| ops.try_recv().ok())
            .any(|op| matches!(op, AppCommand::UserTurn { .. })),
        "cold reconnect must not manufacture or resubmit a user turn"
    );
    let after = session
        .thread_read(thread_id, /*include_turns*/ true)
        .await?;
    assert!(
        format!("{:?}", after.turns).contains(saved_message),
        "successor lost the original persisted rollout"
    );
    successor.kill().await?;
    successor.wait().await?;
    Ok(())
}
