use super::*;
use crate::app::THREAD_EVENT_CHANNEL_CAPACITY;
use crate::app::Turn;
use crate::app::TurnStatus;
use crate::test_support::PathBufExt;
use crate::test_support::test_path_buf;
use codex_app_server_protocol::AgentMessageDeltaNotification;
use codex_app_server_protocol::CommandExecutionRequestApprovalParams;
use codex_app_server_protocol::RequestId as AppServerRequestId;
use codex_app_server_protocol::ServerRequest;
use codex_app_server_protocol::TurnItemsView;
use codex_app_server_protocol::TurnStartedNotification;
use codex_protocol::ThreadId;
use pretty_assertions::assert_eq;

fn turn_started_notification(thread_id: ThreadId, turn_id: &str) -> ServerNotification {
    ServerNotification::TurnStarted(TurnStartedNotification {
        thread_id: thread_id.to_string(),
        turn: Turn {
            id: turn_id.to_string(),
            root_turn_id: None,
            items_view: TurnItemsView::Full,
            items: Vec::new(),
            status: TurnStatus::InProgress,
            error: None,
            started_at: Some(0),
            completed_at: None,
            duration_ms: None,
        },
    })
}

fn exec_approval_request(
    thread_id: ThreadId,
    turn_id: &str,
    item_id: &str,
    approval_id: Option<&str>,
) -> ServerRequest {
    ServerRequest::CommandExecutionRequestApproval {
        request_id: AppServerRequestId::Integer(1),
        params: CommandExecutionRequestApprovalParams {
            kind: Default::default(),
            thread_id: thread_id.to_string(),
            turn_id: turn_id.to_string(),
            item_id: item_id.to_string(),
            started_at_ms: 0,
            approval_id: approval_id.map(str::to_string),
            environment_id: None,
            reason: Some("needs approval".to_string()),
            network_approval_context: None,
            command: Some("echo hello".to_string()),
            cwd: Some(test_path_buf("/tmp/project").abs().into()),
            command_actions: None,
            additional_permissions: None,
            proposed_execpolicy_amendment: None,
            proposed_network_policy_amendments: None,
            available_decisions: None,
        },
    }
}

#[test]
fn thread_event_store_coalesces_only_adjacent_matching_agent_message_deltas() {
    let thread_id = ThreadId::new();
    let mut store = ThreadEventStore::new(/*capacity*/ 6);
    let delta = |turn_id: &str, item_id: &str, text: &str| {
        ServerNotification::AgentMessageDelta(AgentMessageDeltaNotification {
            thread_id: thread_id.to_string(),
            turn_id: turn_id.to_string(),
            item_id: item_id.to_string(),
            delta: text.to_string(),
        })
    };
    let approval =
        exec_approval_request(thread_id, "turn-2", "approval-1", /*approval_id*/ None);
    let other_thread_delta = ServerNotification::AgentMessageDelta(AgentMessageDeltaNotification {
        thread_id: ThreadId::new().to_string(),
        turn_id: "turn-2".to_string(),
        item_id: "item-2".to_string(),
        delta: "other thread".to_string(),
    });
    let snapshot_events = |store: &ThreadEventStore| {
        store
            .snapshot()
            .events
            .into_iter()
            .map(|event| match event {
                ThreadBufferedEvent::Notification(notification) => {
                    serde_json::to_value(notification).expect("notification should serialize")
                }
                ThreadBufferedEvent::Request(request) => {
                    serde_json::to_value(request).expect("request should serialize")
                }
                other => panic!("unexpected buffered event: {other:?}"),
            })
            .collect::<Vec<_>>()
    };

    store.push_notification(delta("turn-1", "item-1", "hello"));
    store.push_notification_ref(&delta("turn-1", "item-1", " world"));
    store.push_notification(delta("turn-1", "item-2", "another"));
    store.push_notification(delta("turn-2", "item-2", "next"));
    store.push_notification(other_thread_delta.clone());
    store.push_request(approval.clone());
    store.push_notification(delta("turn-2", "item-2", "after approval"));

    assert_eq!(
        snapshot_events(&store),
        vec![
            serde_json::to_value(delta("turn-1", "item-1", "hello world"))
                .expect("delta should serialize"),
            serde_json::to_value(delta("turn-1", "item-2", "another"))
                .expect("delta should serialize"),
            serde_json::to_value(delta("turn-2", "item-2", "next"))
                .expect("delta should serialize"),
            serde_json::to_value(other_thread_delta).expect("delta should serialize"),
            serde_json::to_value(approval.clone()).expect("request should serialize"),
            serde_json::to_value(delta("turn-2", "item-2", "after approval"))
                .expect("delta should serialize"),
        ]
    );
    assert!(store.has_pending_thread_approvals());

    let remaining = MAX_COALESCED_AGENT_MESSAGE_DELTA_BYTES - "after approval".len();
    let exact_limit = "é".repeat(remaining / "é".len());
    let full_chunk = "é".repeat(MAX_COALESCED_AGENT_MESSAGE_DELTA_BYTES / "é".len());
    store.push_notification(delta("turn-2", "item-2", &exact_limit));
    store.push_notification(delta("turn-2", "item-2", "🙂"));
    store.push_notification(delta("turn-2", "item-2", &full_chunk));
    store.push_notification(delta("turn-2", "item-2", "🙂"));
    store.push_notification(delta("turn-2", "item-2", &full_chunk));
    store.push_notification(delta("turn-2", "item-2", "🙂"));

    assert_eq!(
        snapshot_events(&store),
        vec![
            serde_json::to_value(approval.clone()).expect("approval should serialize"),
            serde_json::to_value(delta("turn-2", "item-2", "🙂")).expect("delta should serialize"),
            serde_json::to_value(delta("turn-2", "item-2", &full_chunk))
                .expect("delta should serialize"),
            serde_json::to_value(delta("turn-2", "item-2", "🙂")).expect("delta should serialize"),
            serde_json::to_value(delta("turn-2", "item-2", &full_chunk))
                .expect("delta should serialize"),
            serde_json::to_value(delta("turn-2", "item-2", "🙂")).expect("delta should serialize"),
        ]
    );
    assert!(store.has_pending_thread_approvals());

    let mut store = ThreadEventStore::new(/*capacity*/ THREAD_EVENT_CHANNEL_CAPACITY);
    store.push_request(exec_approval_request(
        thread_id,
        "turn-budget",
        "approval-budget",
        /*approval_id*/ None,
    ));
    for chunk in 0..65 {
        let fragment = format!("{chunk:08}");
        let notification = delta("turn-budget", "item-budget", &fragment);
        for _ in 0..MAX_COALESCED_AGENT_MESSAGE_DELTA_BYTES / fragment.len() {
            store.push_notification_ref(&notification);
        }
    }

    assert_eq!(
        snapshot_events(&store),
        std::iter::once(
            serde_json::to_value(exec_approval_request(
                thread_id,
                "turn-budget",
                "approval-budget",
                /*approval_id*/ None,
            ))
            .expect("approval should serialize"),
        )
        .chain((1..65).map(|chunk| {
            serde_json::to_value(delta(
                "turn-budget",
                "item-budget",
                &format!("{chunk:08}").repeat(MAX_COALESCED_AGENT_MESSAGE_DELTA_BYTES / 8),
            ))
            .expect("delta should serialize")
        }))
        .collect::<Vec<_>>()
    );
    assert!(store.has_pending_thread_approvals());

    let mut store = ThreadEventStore::new(/*capacity*/ 4);
    store.push_notification(turn_started_notification(thread_id, "turn-oversized"));
    store.push_request(exec_approval_request(
        thread_id,
        "turn-oversized",
        "approval-oversized",
        /*approval_id*/ None,
    ));
    store.push_notification(delta("turn-oversized", "item-oversized", "retained"));
    let expected_events = snapshot_events(&store);
    let oversized = "🙂".repeat(MAX_BUFFERED_AGENT_MESSAGE_DELTA_BYTES / "🙂".len() + 1);

    store.push_notification(delta("turn-oversized", "item-oversized", &oversized));
    store.push_notification_ref(&delta("turn-oversized", "item-oversized", &oversized));

    assert_eq!(snapshot_events(&store), expected_events);
    assert!(store.has_pending_thread_approvals());
    assert_eq!(store.buffered_agent_message_delta_bytes, "retained".len());

    store.push_notification(delta("turn-oversized", "item-oversized", " text"));
    assert_eq!(
        snapshot_events(&store),
        vec![
            serde_json::to_value(turn_started_notification(thread_id, "turn-oversized"))
                .expect("turn notification should serialize"),
            serde_json::to_value(exec_approval_request(
                thread_id,
                "turn-oversized",
                "approval-oversized",
                /*approval_id*/ None,
            ))
            .expect("approval should serialize"),
            serde_json::to_value(delta("turn-oversized", "item-oversized", "retained text"))
                .expect("delta should serialize"),
        ]
    );
    assert!(store.has_pending_thread_approvals());
}

#[test]
fn retained_replay_byte_budget_evicts_old_messages_but_keeps_pending_approval() {
    let thread_id = ThreadId::new();
    let mut store = ThreadEventStore::new(/*capacity*/ 128);
    let request = exec_approval_request(thread_id, "turn-approval", "item-approval", None);
    store
        .push_request_checked(request.clone())
        .expect("initial approval fits");

    for index in 0..12 {
        let event: ServerNotification = serde_json::from_value(serde_json::json!({
            "method": "item/completed",
            "params": {
                "threadId": thread_id.to_string(),
                "turnId": format!("turn-{index}"),
                "completedAtMs": 0,
                "item": {
                    "type": "agentMessage",
                    "id": format!("item-{index}"),
                    "text": "x".repeat(1024 * 1024),
                    "phase": null,
                    "memoryCitation": null,
                    "delivery": null,
                    "questions": null
                }
            }
        }))
        .expect("valid large agent notification");
        ThreadEventChannel::ensure_notification_budget(&event)
            .expect("each individual event is below live byte cap");
        store.push_notification(event);
        assert!(
            store.retained_replay_wire_bytes <= THREAD_EVENT_REPLAY_MAX_WIRE_BYTES,
            "aggregate replay payload must remain bounded after each event"
        );
    }

    let pending = store.pending_replay_requests();
    assert_eq!(pending.len(), 1);
    assert_eq!(
        serde_json::to_value(&pending[0]).expect("pending request serializes"),
        serde_json::to_value(request).expect("original request serializes")
    );
    assert!(store.has_pending_thread_approvals());
    assert!(
        store.buffer.len() < 13,
        "ordinary old messages should be evicted"
    );
}

#[test]
fn unresolved_request_retention_exhaustion_fails_before_mutating_approval_state() {
    let thread_id = ThreadId::new();
    let mut store = ThreadEventStore::new(/*capacity*/ 128);
    let mut rejected = false;
    let mut accepted = 0usize;

    for index in 0..16 {
        let mut request = exec_approval_request(
            thread_id,
            &format!("turn-{index}"),
            &format!("item-{index}"),
            Some(&format!("approval-{index}")),
        );
        let ServerRequest::CommandExecutionRequestApproval { request_id, params } = &mut request
        else {
            panic!("expected exec approval");
        };
        *request_id = AppServerRequestId::Integer(index);
        params.command = Some("x".repeat(1024 * 1024));
        ThreadEventChannel::ensure_request_budget(&request)
            .expect("each request fits the per-event ingress cap");
        if store.push_request_checked(request).is_err() {
            rejected = true;
            break;
        }
        accepted += 1;
    }

    assert!(
        rejected,
        "aggregate protected requests must hit a retention cap"
    );
    assert!(accepted > 1);
    assert_eq!(store.pending_replay_requests().len(), accepted);
    assert!(store.has_pending_thread_approvals());
    assert!(store.retained_replay_wire_bytes <= THREAD_EVENT_REPLAY_MAX_WIRE_BYTES);

    // Resolving one request makes its old replay entry evictable; further
    // requests must not require dropping any still-pending approval.
    store.push_notification(ServerNotification::ServerRequestResolved(
        codex_app_server_protocol::ServerRequestResolvedNotification {
            request_id: AppServerRequestId::Integer(0),
            thread_id: thread_id.to_string(),
        },
    ));
    let mut next =
        exec_approval_request(thread_id, "turn-next", "item-next", Some("approval-next"));
    let ServerRequest::CommandExecutionRequestApproval { request_id, params } = &mut next else {
        panic!("expected exec approval");
    };
    *request_id = AppServerRequestId::Integer(99);
    params.command = Some("x".repeat(1024 * 1024));
    store
        .push_request_checked(next)
        .expect("resolved request should free enough replay capacity");
    assert_eq!(store.pending_replay_requests().len(), accepted);
    assert!(store.retained_replay_wire_bytes <= THREAD_EVENT_REPLAY_MAX_WIRE_BYTES);
}

#[test]
fn refresh_recounts_replay_bytes_after_stripping_replayed_message_text() {
    let thread_id = ThreadId::new();
    let mut store = ThreadEventStore::new(/*capacity*/ 16);
    let event: ServerNotification = serde_json::from_value(serde_json::json!({
        "method": "item/completed",
        "params": {
            "threadId": thread_id.to_string(),
            "turnId": "turn-question",
            "completedAtMs": 0,
            "item": {
                "type": "agentMessage",
                "id": "question",
                "text": "x".repeat(1024 * 1024),
                "phase": null,
                "memoryCitation": null,
                "delivery": null,
                "questions": [{"title": "Choose?", "options": null}]
            }
        }
    }))
    .expect("valid live question");
    store.push_notification(event);
    let before = store.retained_replay_wire_bytes;
    store.rebase_buffer_after_session_refresh();
    assert!(store.retained_replay_wire_bytes < before);
    assert_eq!(
        store.retained_replay_wire_bytes,
        store
            .buffer
            .iter()
            .map(retained_replay_event_bytes)
            .sum::<usize>()
    );
}
