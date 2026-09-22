//! Budget-aware routing helpers for codexdd 0.3.2.
//!
//! This module is deliberately deterministic. It consumes backend-provided rate-limit windows and
//! computes whether codexdd should conserve, stay balanced, or spend surplus allowance on quality.
//! It never authorizes failure-driven escalation and never bypasses the existing capability-report
//! guard for model-family jumps during an active Worker attempt.

use crate::adaptive_policy::AdaptiveEffort;
use crate::adaptive_policy::AdaptiveFamily;
use crate::adaptive_policy::AdaptiveRoute;
use codex_app_server_protocol::RateLimitWindow;

const CONSERVE_SURPLUS_POINTS: f64 = -10.0;
const SURPLUS_SURPLUS_POINTS: f64 = 20.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum AdaptiveBudgetMode {
    Conserve,
    #[default]
    Balanced,
    Surplus,
}

impl AdaptiveBudgetMode {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Conserve => "Conserve",
            Self::Balanced => "Balanced",
            Self::Surplus => "Surplus",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct AdaptiveBudgetAssessment {
    pub(crate) mode: AdaptiveBudgetMode,
    pub(crate) primary_surplus_points: Option<f64>,
    pub(crate) secondary_surplus_points: Option<f64>,
}

/// Compare remaining quota against remaining wall-clock time in each rolling window.
///
/// Positive surplus means quota is being consumed more slowly than the window is expiring.
/// Negative surplus means usage is ahead of pace. The most restrictive known window wins.
pub(crate) fn assess_budget(
    primary: Option<&RateLimitWindow>,
    secondary: Option<&RateLimitWindow>,
    now_unix_seconds: i64,
) -> AdaptiveBudgetAssessment {
    let primary_surplus_points =
        primary.and_then(|window| window_surplus_points(window, now_unix_seconds));
    let secondary_surplus_points =
        secondary.and_then(|window| window_surplus_points(window, now_unix_seconds));

    let mode = combine_window_modes([
        primary_surplus_points.map(mode_for_surplus_points),
        secondary_surplus_points.map(mode_for_surplus_points),
    ]);

    AdaptiveBudgetAssessment {
        mode,
        primary_surplus_points,
        secondary_surplus_points,
    }
}

fn window_surplus_points(window: &RateLimitWindow, now_unix_seconds: i64) -> Option<f64> {
    let duration_minutes = window.window_duration_mins?;
    let resets_at = window.resets_at?;
    if duration_minutes <= 0 || resets_at <= now_unix_seconds {
        return None;
    }

    let duration_seconds = duration_minutes.saturating_mul(60);
    if duration_seconds <= 0 {
        return None;
    }

    let seconds_remaining = resets_at.saturating_sub(now_unix_seconds);
    let remaining_time_percent =
        (seconds_remaining as f64 / duration_seconds as f64 * 100.0).clamp(0.0, 100.0);
    let used_percent = f64::from(window.used_percent).clamp(0.0, 100.0);
    let remaining_quota_percent = 100.0 - used_percent;

    Some(remaining_quota_percent - remaining_time_percent)
}

fn mode_for_surplus_points(points: f64) -> AdaptiveBudgetMode {
    if points <= CONSERVE_SURPLUS_POINTS {
        AdaptiveBudgetMode::Conserve
    } else if points >= SURPLUS_SURPLUS_POINTS {
        AdaptiveBudgetMode::Surplus
    } else {
        AdaptiveBudgetMode::Balanced
    }
}

fn combine_window_modes<const N: usize>(
    modes: [Option<AdaptiveBudgetMode>; N],
) -> AdaptiveBudgetMode {
    let mut saw_surplus = false;
    for mode in modes.into_iter().flatten() {
        match mode {
            AdaptiveBudgetMode::Conserve => return AdaptiveBudgetMode::Conserve,
            AdaptiveBudgetMode::Balanced => return AdaptiveBudgetMode::Balanced,
            AdaptiveBudgetMode::Surplus => saw_surplus = true,
        }
    }
    if saw_surplus {
        AdaptiveBudgetMode::Surplus
    } else {
        AdaptiveBudgetMode::Balanced
    }
}

/// Independent Validation/Reviewer Workers are where surplus quota buys the most quality.
///
/// Implementation and repair attempts retain the existing Luna-low start until the separate
/// complexity-floor slice authorizes a higher starting route.
pub(crate) fn quality_review_route(mode: AdaptiveBudgetMode) -> AdaptiveRoute {
    match mode {
        AdaptiveBudgetMode::Conserve => AdaptiveRoute {
            family: AdaptiveFamily::Luna,
            effort: AdaptiveEffort::High,
        },
        AdaptiveBudgetMode::Balanced => AdaptiveRoute {
            family: AdaptiveFamily::Sol,
            effort: AdaptiveEffort::Low,
        },
        AdaptiveBudgetMode::Surplus => AdaptiveRoute {
            family: AdaptiveFamily::Sol,
            effort: AdaptiveEffort::High,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn window(used_percent: i32, duration_minutes: i64, seconds_remaining: i64) -> RateLimitWindow {
        let now = 1_000_000;
        RateLimitWindow {
            used_percent,
            window_duration_mins: Some(duration_minutes),
            resets_at: Some(now + seconds_remaining),
        }
    }

    #[test]
    fn classifies_conserve_when_usage_is_ahead_of_time() {
        let now = 1_000_000;
        let primary = window(70, 300, 180 * 60);
        let assessment = assess_budget(Some(&primary), None, now);
        assert_eq!(assessment.mode, AdaptiveBudgetMode::Conserve);
        assert!(
            assessment
                .primary_surplus_points
                .is_some_and(|points| points < -10.0)
        );
    }

    #[test]
    fn classifies_balanced_near_even_pace() {
        let now = 1_000_000;
        let primary = window(50, 300, 150 * 60);
        let assessment = assess_budget(Some(&primary), None, now);
        assert_eq!(assessment.mode, AdaptiveBudgetMode::Balanced);
        assert_eq!(assessment.primary_surplus_points, Some(0.0));
    }

    #[test]
    fn classifies_surplus_when_allowance_would_expire_underused() {
        let now = 1_000_000;
        let weekly = window(45, 7 * 24 * 60, (14 * 7 * 24 * 60 * 60) / 100);
        let assessment = assess_budget(None, Some(&weekly), now);
        assert_eq!(assessment.mode, AdaptiveBudgetMode::Surplus);
        assert!(
            assessment
                .secondary_surplus_points
                .is_some_and(|points| points > 20.0)
        );
    }

    #[test]
    fn most_restrictive_known_window_wins() {
        let now = 1_000_000;
        let primary = window(75, 300, 210 * 60);
        let secondary = window(20, 7 * 24 * 60, 24 * 60 * 60);
        let assessment = assess_budget(Some(&primary), Some(&secondary), now);
        assert_eq!(assessment.mode, AdaptiveBudgetMode::Conserve);
    }

    #[test]
    fn stale_or_incomplete_windows_do_not_create_fake_surplus() {
        let now = 1_000_000;
        let stale = RateLimitWindow {
            used_percent: 10,
            window_duration_mins: Some(300),
            resets_at: Some(now - 1),
        };
        let incomplete = RateLimitWindow {
            used_percent: 10,
            window_duration_mins: None,
            resets_at: Some(now + 100),
        };
        assert_eq!(
            assess_budget(Some(&stale), Some(&incomplete), now).mode,
            AdaptiveBudgetMode::Balanced
        );
    }

    #[test]
    fn review_floor_spends_surplus_on_quality_not_implementation() {
        assert_eq!(
            quality_review_route(AdaptiveBudgetMode::Conserve),
            AdaptiveRoute {
                family: AdaptiveFamily::Luna,
                effort: AdaptiveEffort::High,
            }
        );
        assert_eq!(
            quality_review_route(AdaptiveBudgetMode::Balanced),
            AdaptiveRoute {
                family: AdaptiveFamily::Sol,
                effort: AdaptiveEffort::Low,
            }
        );
        assert_eq!(
            quality_review_route(AdaptiveBudgetMode::Surplus),
            AdaptiveRoute {
                family: AdaptiveFamily::Sol,
                effort: AdaptiveEffort::High,
            }
        );
    }
}
