//! Inert receipt and per-thread lifecycle for private adaptive runtime signals.

use super::*;
use crate::adaptive_evidence::AUTO_FAILURE_PRESSURE_DIAGNOSTIC;
use crate::chatwidget::adaptive_effort::AdaptivePendingSignal;
use codex_app_server_protocol::AdaptiveRuntimeSignalNotification;
use codex_protocol::protocol::AdaptiveRuntimeSignalKind;

impl ChatWidget {
    pub(super) fn handle_adaptive_runtime_signal(
        &mut self,
        mut notification: AdaptiveRuntimeSignalNotification,
    ) {
        let Ok(thread_id) = ThreadId::from_string(&notification.thread_id) else {
            return;
        };
        let source_turn_id = notification.signal.source_turn_id.as_str();
        notification.signal.evidence_refs.sort();
        notification.signal.evidence_refs.dedup();
        if self.thread_id != Some(thread_id)
            || !self.turn_lifecycle.agent_turn_running
            || self.turn_lifecycle.last_turn_id.as_deref() != Some(source_turn_id)
            || !self.adaptive_effort.enabled
            || self.adaptive_effort.paused_by_user
            || self.adaptive_effort.workflow_terminal.is_some()
        {
            return;
        }

        match self.adaptive_effort.pending_signal.as_ref() {
            None => {
                self.adaptive_effort.pending_signal =
                    Some(AdaptivePendingSignal::Pending(notification.signal));
                self.save_adaptive_effort_for_current_thread();
            }
            // A real capability report received later in the same turn upgrades the synthetic
            // two-native-failure signal. Diagnostic text is part of the authorization boundary:
            // the synthetic sentinel alone cannot justify spending into a stronger model family.
            Some(AdaptivePendingSignal::Pending(existing))
                if existing.source_turn_id == source_turn_id
                    && existing.diagnostic_note.as_deref()
                        == Some(AUTO_FAILURE_PRESSURE_DIAGNOSTIC)
                    && existing.signal_kind == AdaptiveRuntimeSignalKind::Capability
                    && notification.signal.signal_kind == AdaptiveRuntimeSignalKind::Capability
                    && notification
                        .signal
                        .diagnostic_note
                        .as_deref()
                        .map(str::trim)
                        .is_some_and(|note| {
                            !note.is_empty() && note != AUTO_FAILURE_PRESSURE_DIAGNOSTIC
                        }) =>
            {
                self.adaptive_effort.pending_signal =
                    Some(AdaptivePendingSignal::Pending(notification.signal));
                self.save_adaptive_effort_for_current_thread();
            }
            // A workflow terminal reported later in the same turn outranks the synthetic
            // two-failure capability signal. This lets a worker recover after two failed tools and
            // still finish cleanly without forcing an unnecessary escalation.
            Some(AdaptivePendingSignal::Pending(existing))
                if existing.source_turn_id == source_turn_id
                    && existing.diagnostic_note.as_deref()
                        == Some(AUTO_FAILURE_PRESSURE_DIAGNOSTIC)
                    && existing.signal_kind == AdaptiveRuntimeSignalKind::Capability
                    && notification.signal.signal_kind != AdaptiveRuntimeSignalKind::Capability =>
            {
                self.adaptive_effort.pending_signal =
                    Some(AdaptivePendingSignal::Pending(notification.signal));
                self.save_adaptive_effort_for_current_thread();
            }
            Some(AdaptivePendingSignal::Pending(existing))
                if existing.source_turn_id == notification.signal.source_turn_id
                    && existing.signal_kind == notification.signal.signal_kind
                    && existing.evidence_refs == notification.signal.evidence_refs
                    && existing.diagnostic_note == notification.signal.diagnostic_note => {}
            Some(AdaptivePendingSignal::Pending(existing))
                if existing.source_turn_id == source_turn_id =>
            {
                self.adaptive_effort.pending_signal = Some(AdaptivePendingSignal::Conflicted {
                    source_turn_id: source_turn_id.to_string(),
                });
                self.save_adaptive_effort_for_current_thread();
            }
            Some(
                AdaptivePendingSignal::Conflicted {
                    source_turn_id: existing_turn_id,
                }
                | AdaptivePendingSignal::Cancelled {
                    source_turn_id: existing_turn_id,
                }
                | AdaptivePendingSignal::Consumed {
                    source_turn_id: existing_turn_id,
                },
            ) if existing_turn_id == source_turn_id => {}
            Some(_) => {}
        }
    }

    pub(super) fn clear_stale_pending_adaptive_signal(&mut self, active_turn_id: &str) {
        if self
            .adaptive_effort
            .pending_signal
            .as_ref()
            .is_some_and(|signal| signal.source_turn_id() != active_turn_id)
        {
            self.adaptive_effort.pending_signal = None;
            self.save_adaptive_effort_for_current_thread();
        }
    }

    pub(super) fn cancel_pending_adaptive_signal_for_active_turn(&mut self) {
        let Some(source_turn_id) = self.turn_lifecycle.last_turn_id.as_deref() else {
            return;
        };
        if matches!(
            self.adaptive_effort.pending_signal,
            Some(AdaptivePendingSignal::Pending(ref signal))
                if signal.source_turn_id == source_turn_id
        ) {
            self.adaptive_effort.pending_signal = Some(AdaptivePendingSignal::Cancelled {
                source_turn_id: source_turn_id.to_string(),
            });
        }
    }
}
