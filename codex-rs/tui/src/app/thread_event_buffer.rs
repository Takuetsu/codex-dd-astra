//! Bounded replay-buffer policy for per-thread app-server events.

use super::ServerNotification;
use super::ServerRequest;
use super::THREAD_EVENT_REPLAY_MAX_WIRE_BYTES;
use super::ThreadBufferedEvent;
use super::ThreadEventStore;
use super::retained_replay_event_bytes;
use super::serialized_live_event_bytes;
use color_eyre::Result;
use std::borrow::Cow;

// Keep merged text finite so continued streaming still reaches bounded replay eviction.
const MAX_COALESCED_AGENT_MESSAGE_DELTA_BYTES: usize = 4 * 1024;
const MAX_BUFFERED_AGENT_MESSAGE_DELTA_BYTES: usize = 256 * 1024;

impl ThreadEventStore {
    fn is_interactive_request(request: &ServerRequest) -> bool {
        matches!(
            request,
            ServerRequest::CommandExecutionRequestApproval { .. }
                | ServerRequest::FileChangeRequestApproval { .. }
                | ServerRequest::McpServerElicitationRequest { .. }
                | ServerRequest::ToolRequestUserInput { .. }
                | ServerRequest::PermissionsRequestApproval { .. }
        )
    }

    fn is_unresolved_interactive_event(&self, event: &ThreadBufferedEvent) -> bool {
        matches!(
            event,
            ThreadBufferedEvent::Request(request)
                if Self::is_interactive_request(request)
                    && self
                        .pending_interactive_replay
                        .should_replay_snapshot_request(request.as_ref())
        )
    }

    /// Reject before changing pending state if a new request cannot coexist with
    /// all unresolved requests under both the entry-count and aggregate-byte caps.
    /// The app-server ingress caller propagates the error; it must not acknowledge
    /// a permission request it cannot preserve for a later thread switch.
    pub(super) fn push_request_checked(&mut self, request: ServerRequest) -> Result<()> {
        if Self::is_interactive_request(&request) {
            let mut remaining_entries = self.buffer.len().saturating_add(1);
            let mut remaining_bytes = self
                .retained_replay_wire_bytes
                .saturating_add(serialized_live_event_bytes(&request)?);
            let mut remaining_delta_bytes = self.buffered_agent_message_delta_bytes;
            for event in &self.buffer {
                if remaining_entries <= self.capacity
                    && remaining_bytes <= THREAD_EVENT_REPLAY_MAX_WIRE_BYTES
                    && remaining_delta_bytes <= MAX_BUFFERED_AGENT_MESSAGE_DELTA_BYTES
                {
                    break;
                }
                if self.is_unresolved_interactive_event(event) {
                    continue;
                }
                remaining_entries = remaining_entries.saturating_sub(1);
                remaining_bytes =
                    remaining_bytes.saturating_sub(retained_replay_event_bytes(event));
                if let ThreadBufferedEvent::Notification(notification) = event
                    && let ServerNotification::AgentMessageDelta(delta) = notification.as_ref()
                {
                    remaining_delta_bytes = remaining_delta_bytes.saturating_sub(delta.delta.len());
                }
            }
            if remaining_entries > self.capacity
                || remaining_bytes > THREAD_EVENT_REPLAY_MAX_WIRE_BYTES
                || remaining_delta_bytes > MAX_BUFFERED_AGENT_MESSAGE_DELTA_BYTES
            {
                return Err(color_eyre::eyre::eyre!(
                    "replay retention exhausted by unresolved interactive requests; refusing to lose a pending approval"
                ));
            }
        }
        self.push_request(request);
        Ok(())
    }

    pub(super) fn recount_retained_replay_bytes(&mut self) {
        self.retained_replay_wire_bytes = self.buffer.iter().fold(0usize, |total, event| {
            total.saturating_add(retained_replay_event_bytes(event))
        });
        self.buffered_agent_message_delta_bytes = self
            .buffer
            .iter()
            .filter_map(|event| match event {
                ThreadBufferedEvent::Notification(notification) => match notification.as_ref() {
                    ServerNotification::AgentMessageDelta(delta) => Some(delta.delta.len()),
                    _ => None,
                },
                _ => None,
            })
            .fold(0usize, usize::saturating_add);
        self.evict_overflowing_events();
    }
    pub(super) fn push_replay_notification(&mut self, notification: Cow<'_, ServerNotification>) {
        if let ServerNotification::AgentMessageDelta(delta) = notification.as_ref()
            && delta.delta.len() > MAX_BUFFERED_AGENT_MESSAGE_DELTA_BYTES
        {
            return;
        }

        if let ServerNotification::AgentMessageDelta(delta) = notification.as_ref()
            && let Some(ThreadBufferedEvent::Notification(previous)) = self.buffer.back_mut()
            && let ServerNotification::AgentMessageDelta(previous) = previous.as_mut()
            && previous.thread_id == delta.thread_id
            && previous.turn_id == delta.turn_id
            && previous.item_id == delta.item_id
            && previous.delta.len().saturating_add(delta.delta.len())
                <= MAX_COALESCED_AGENT_MESSAGE_DELTA_BYTES
        {
            // JSON escaping can grow Unicode/text beyond the raw UTF-8 length;
            // account for the exact serialized change to a coalesced delta.
            let prior_wire_bytes = serialized_live_event_bytes(previous).unwrap_or(0);
            previous.delta.push_str(&delta.delta);
            let updated_wire_bytes = serialized_live_event_bytes(previous)
                .unwrap_or(THREAD_EVENT_REPLAY_MAX_WIRE_BYTES.saturating_add(1));
            self.retained_replay_wire_bytes = self
                .retained_replay_wire_bytes
                .saturating_add(updated_wire_bytes.saturating_sub(prior_wire_bytes));
            self.buffered_agent_message_delta_bytes = self
                .buffered_agent_message_delta_bytes
                .saturating_add(delta.delta.len());
            self.evict_overflowing_events();
            return;
        }

        self.push_buffered_event(ThreadBufferedEvent::Notification(Box::new(
            notification.into_owned(),
        )));
    }

    pub(super) fn push_buffered_event(&mut self, event: ThreadBufferedEvent) {
        self.retained_replay_wire_bytes = self
            .retained_replay_wire_bytes
            .saturating_add(retained_replay_event_bytes(&event));
        if let ThreadBufferedEvent::Notification(notification) = &event
            && let ServerNotification::AgentMessageDelta(delta) = notification.as_ref()
        {
            self.buffered_agent_message_delta_bytes = self
                .buffered_agent_message_delta_bytes
                .saturating_add(delta.delta.len());
        }
        self.buffer.push_back(event);
        self.evict_overflowing_events();
    }

    fn evict_overflowing_events(&mut self) {
        while self.buffer.len() > self.capacity
            || self.buffered_agent_message_delta_bytes > MAX_BUFFERED_AGENT_MESSAGE_DELTA_BYTES
            || self.retained_replay_wire_bytes > THREAD_EVENT_REPLAY_MAX_WIRE_BYTES
        {
            // Preserve the relative ordering of all survivors while refusing
            // to evict unresolved approvals, user-input or MCP prompts.
            let Some(index) = self
                .buffer
                .iter()
                .position(|event| !self.is_unresolved_interactive_event(event))
            else {
                // Production requests are preflighted by push_request_checked.
                // Never clear the unresolved authorization state merely to
                // satisfy a replay cap.
                tracing::error!(
                    "replay retention is full of unresolved requests; cannot evict safely"
                );
                break;
            };
            let Some(removed) = self.buffer.remove(index) else {
                break;
            };
            self.retained_replay_wire_bytes = self
                .retained_replay_wire_bytes
                .saturating_sub(retained_replay_event_bytes(&removed));
            match removed {
                ThreadBufferedEvent::Notification(notification) => {
                    if let ServerNotification::AgentMessageDelta(delta) = notification.as_ref() {
                        self.buffered_agent_message_delta_bytes = self
                            .buffered_agent_message_delta_bytes
                            .saturating_sub(delta.delta.len());
                    }
                }
                ThreadBufferedEvent::Request(request) => self
                    .pending_interactive_replay
                    .note_evicted_server_request(request.as_ref()),
                ThreadBufferedEvent::HistoryEntryResponse(_)
                | ThreadBufferedEvent::FeedbackSubmission(_) => {}
            }
        }
    }
}

#[cfg(test)]
#[path = "thread_event_buffer_tests.rs"]
mod tests;
