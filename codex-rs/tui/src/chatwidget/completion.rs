//! Completion metadata for live and restored turns. Saved timestamps are authoritative;
//! only a live completion may fall back to the local clock. Labels are deduplicated per thread.

use super::*;

impl ChatWidget {
    pub(crate) fn completion_cell(
        &mut self,
        turn: &Turn,
        replay_kind: Option<ReplayKind>,
    ) -> Option<history_cell::FinalMessageSeparator> {
        if self
            .turn_lifecycle
            .rendered_completion_turn_ids
            .contains(&turn.id)
        {
            return None;
        }
        let duration_ms = turn
            .duration_ms
            .and_then(|duration| u64::try_from(duration).ok())
            .or_else(|| {
                if replay_kind.is_none() {
                    self.bottom_pane
                        .status_elapsed()
                        .map(|elapsed| u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX))
                } else {
                    None
                }
            });
        let elapsed_seconds = duration_ms
            .map(|duration| duration / 1_000)
            .filter(|seconds| *seconds > 60);
        let worker_bound = self.adaptive_effort.worker_context.role
            != crate::adaptive_worker::AdaptiveWorkerRole::Unspecified;
        if worker_bound && let Some(duration_ms) = duration_ms {
            self.turn_lifecycle
                .record_worker_duration(&turn.id, duration_ms);
        }
        let completed_at = turn
            .completed_at
            .and_then(|timestamp| chrono::DateTime::from_timestamp(timestamp, /*nsecs*/ 0))
            .map(|timestamp| timestamp.with_timezone(&Local))
            .or_else(|| replay_kind.is_none().then(Local::now));
        let worker_done = worker_bound
            && self.adaptive_effort.workflow_terminal.is_some()
            && self
                .adaptive_effort
                .last_processed_terminal_turn_id
                .as_deref()
                == Some(turn.id.as_str())
            && completed_at.is_some();
        if completed_at.is_none() && elapsed_seconds.is_none() && !worker_done {
            return None;
        }
        self.turn_lifecycle
            .rendered_completion_turn_ids
            .insert(turn.id.clone());
        let mut cell = history_cell::FinalMessageSeparator::new(
            elapsed_seconds,
            /*runtime_metrics*/ None,
        );
        if worker_done {
            cell =
                cell.with_worker_completion(self.turn_lifecycle.worker_total_duration_ms() / 1_000);
        }
        Some(match completed_at {
            Some(completed_at) => {
                cell.with_completed_at(completed_at, crate::clock_format::ClockFormat::system())
            }
            None => cell,
        })
    }
}
