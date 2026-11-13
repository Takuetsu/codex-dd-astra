//! 0.4.6 3C.1b.2j1 — exercise the normal App event multiplexer and
//! foreground thread buffer without granting an evidence/terminal bypass.
//! Native Core receipt execution is independently covered by 3C.1b.2i.

use super::*;
use pretty_assertions::assert_eq;
use codex_app_server_client::AppServerEvent;
use codex_app_server_protocol::AdaptiveRuntimeSignalEnvelope;
use codex_app_server_protocol::AdaptiveRuntimeSignalNotification;
use codex_app_server_protocol::DynamicToolCallStatus;
use codex_app_server_protocol::ItemCompletedNotification;
use codex_app_server_protocol::ThreadWorkflowStateOperation;
use codex_protocol::protocol::AdaptiveRuntimeSignalKind;

#[tokio::test]
async fn codexdd_046_3c1b2j1_app_routes_only_selected_thread_native_evidence() -> Result<()> {
    let (mut app, mut app_events, _ops) = make_test_app_with_channels().await;
    let app_server = Box::pin(crate::start_embedded_app_server_for_picker(&app.config)).await?;
    let selected = ThreadId::new();
    let foreign = ThreadId::new();

    app.enqueue_primary_thread_session(
        test_thread_session(selected, app.config.cwd.to_path_buf()),
        Vec::new(),
    )
    .await?;
    assert_eq!(app.current_displayed_thread_id(), Some(selected));
    assert_eq!(app.chat_widget.thread_id(), Some(selected));

    // The normal App event path must keep the exact Core tool receipt on its
    // selected foreground thread, not turn it into a global TUI notification.
    let native_receipt = ServerNotification::ItemCompleted(ItemCompletedNotification {
        thread_id: selected.to_string(),
        turn_id: "native-source-turn".to_string(),
        completed_at_ms: 42,
        item: ThreadItem::DynamicToolCall {
            id: "native-source-receipt".to_string(),
            namespace: None,
            tool: "run_codexdd_validation".to_string(),
            arguments: serde_json::json!({
                "profile": "work_packet",
                "result": "pass",
                "head_sha": "source-not-authorized-by-owner",
                "run_id": "native-source-run",
                "source_root": "fixture-not-an-authorized-worktree",
            }),
            status: DynamicToolCallStatus::Completed,
            content_items: None,
            success: Some(true),
            duration_ms: Some(1),
        },
    });
    app.handle_app_server_event(
        &app_server,
        AppServerEvent::ServerNotification(Box::new(native_receipt)),
    )
    .await;
    let queued = app
        .active_thread_rx
        .as_mut()
        .expect("selected Worker has a foreground receiver")
        .try_recv()
        .expect("native evidence must reach the selected Worker event queue");
    let ThreadBufferedEvent::Notification(notification) = &queued else {
        panic!("the foreground channel did not preserve a native item notification");
    };
    let ServerNotification::ItemCompleted(item) = notification.as_ref() else {
        panic!("the App event multiplexer changed the native item event type");
    };
    let ThreadItem::DynamicToolCall { id, arguments, .. } = &item.item else {
        panic!("native tool completion was rewritten into a different item");
    };
    assert_eq!(item.thread_id, selected.to_string());
    assert_eq!(id, "native-source-receipt");
    assert_eq!(arguments["profile"], "work_packet");
    assert_eq!(arguments["run_id"], "native-source-run");
    app.handle_thread_event_now(queued);
    assert!(
        app.chat_widget
            .adaptive_effort_for_test()
            .workflow_terminal
            .is_none(),
        "a native-looking receipt without an owner source/gate pin cannot grant READY"
    );

    // Neither a foreign review receipt nor a foreign trusted-signal shape
    // may be delivered to or admitted by the displayed source Worker.
    let foreign_receipt = ServerNotification::ItemCompleted(ItemCompletedNotification {
        thread_id: foreign.to_string(),
        turn_id: "foreign-turn".to_string(),
        completed_at_ms: 43,
        item: ThreadItem::DynamicToolCall {
            id: "foreign-independent-receipt".to_string(),
            namespace: None,
            tool: "run_codexdd_independent_validation".to_string(),
            arguments: serde_json::json!({
                "profile": "independent_review",
                "result": "pass",
                "head_sha": "source-not-authorized-by-owner",
                "run_id": "foreign-run",
            }),
            status: DynamicToolCallStatus::Completed,
            content_items: None,
            success: Some(true),
            duration_ms: Some(1),
        },
    });
    app.handle_app_server_event(
        &app_server,
        AppServerEvent::ServerNotification(Box::new(foreign_receipt)),
    )
    .await;
    app.handle_app_server_event(
        &app_server,
        AppServerEvent::ServerNotification(Box::new(ServerNotification::AdaptiveRuntimeSignal(
            AdaptiveRuntimeSignalNotification {
                thread_id: foreign.to_string(),
                signal: AdaptiveRuntimeSignalEnvelope {
                    source_turn_id: "foreign-turn".to_string(),
                    signal_kind: AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
                    evidence_refs: vec!["foreign-independent-receipt".to_string()],
                    diagnostic_note: None,
                },
            },
        ))),
    )
    .await;
    assert!(
        app.active_thread_rx
            .as_mut()
            .expect("selected receiver remains intact")
            .is_empty(),
        "foreign Worker evidence/signal must not enter selected Worker's event queue"
    );
    assert!(
        !app.thread_event_channels.contains_key(&foreign),
        "unowned foreign Worker must not become an owned session through a receipt"
    );
    assert_eq!(app.current_displayed_thread_id(), Some(selected));
    assert!(
        app.chat_widget
            .adaptive_effort_for_test()
            .workflow_terminal
            .is_none()
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
            "a foreign signal or unpinned source item requested owner READY persistence"
        );
    }

    app_server.shutdown().await?;
    Ok(())
}
