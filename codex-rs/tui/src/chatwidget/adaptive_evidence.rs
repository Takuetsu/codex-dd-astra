//! Live native tool-completion ingestion for the per-thread adaptive evidence registry.

use super::*;
use crate::adaptive_evidence::record_from_item_completion;
use codex_app_server_protocol::ItemCompletedNotification;

impl ChatWidget {
    pub(super) fn register_adaptive_evidence(&mut self, notification: &ItemCompletedNotification) {
        let Some(thread_id) = self.thread_id else {
            return;
        };
        let Some(record) = record_from_item_completion(notification) else {
            return;
        };
        if record.thread_id != thread_id {
            return;
        }
        self.adaptive_effort.evidence_registry.register(record);
        self.save_adaptive_effort_for_current_thread();
    }
}
