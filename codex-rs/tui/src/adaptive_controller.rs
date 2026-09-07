//! Pure adaptive controller state reduction.
//!
//! The TUI's per-thread `AdaptiveEffortState` remains canonical lifecycle state. A future runtime
//! bridge may translate it to this neutral snapshot and execute an authorized decision.

use crate::adaptive_policy::AdaptiveClassification;
use crate::adaptive_policy::AdaptiveFamily;
use crate::adaptive_policy::AdaptiveRoute;
use crate::adaptive_policy::AdaptiveTransition;
use crate::adaptive_policy::initial_route;
use crate::adaptive_policy::next_route;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveControllerState {
    pub(crate) starting_family: AdaptiveFamily,
    pub(crate) current_route: AdaptiveRoute,
    pub(crate) attempt_number: u32,
    pub(crate) transient_retry_consumed: bool,
    pub(crate) paused_by_user: bool,
    pub(crate) terminal: Option<AdaptiveWorkflowTerminal>,
}

impl AdaptiveControllerState {
    pub(crate) fn initial(starting_family: AdaptiveFamily) -> Self {
        Self {
            starting_family,
            current_route: initial_route(starting_family),
            attempt_number: 1,
            transient_retry_consumed: false,
            paused_by_user: false,
            terminal: None,
        }
    }
    /// Clears only a user pause. It does not authorize or record a work attempt.
    pub(crate) fn resume(mut self) -> Self {
        self.paused_by_user = false;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveControllerDecision {
    NoAction,
    RetrySameLevel {
        route: AdaptiveRoute,
        next_attempt: u32,
    },
    EscalateEffort {
        from: AdaptiveRoute,
        to: AdaptiveRoute,
        next_attempt: u32,
    },
    EscalateModel {
        from: AdaptiveRoute,
        to: AdaptiveRoute,
        next_attempt: u32,
    },
    ReadyForOwnerQa,
    Blocked,
    PausedByUser,
    InvalidState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveControllerReduction {
    pub(crate) state: AdaptiveControllerState,
    pub(crate) decision: AdaptiveControllerDecision,
}

/// Deterministically reduces one normalized classification. It has no side effects.
pub(crate) fn reduce_adaptive_controller(
    state: AdaptiveControllerState,
    classification: AdaptiveClassification,
) -> AdaptiveControllerReduction {
    if state.terminal.is_some() || state.paused_by_user {
        return no_action(state);
    }
    match classification {
        AdaptiveClassification::NoDecision => no_action(state),
        AdaptiveClassification::UserInterrupted => AdaptiveControllerReduction {
            state: AdaptiveControllerState {
                paused_by_user: true,
                ..state
            },
            decision: AdaptiveControllerDecision::PausedByUser,
        },
        AdaptiveClassification::PassToOwnerQa => AdaptiveControllerReduction {
            state: AdaptiveControllerState {
                terminal: Some(AdaptiveWorkflowTerminal::ReadyForOwnerQa),
                ..state
            },
            decision: AdaptiveControllerDecision::ReadyForOwnerQa,
        },
        AdaptiveClassification::Blocked => block(state),
        AdaptiveClassification::RetrySameLevel if state.transient_retry_consumed => block(state),
        AdaptiveClassification::RetrySameLevel => {
            let next_attempt = state.attempt_number + 1;
            AdaptiveControllerReduction {
                state: AdaptiveControllerState {
                    attempt_number: next_attempt,
                    transient_retry_consumed: true,
                    ..state
                },
                decision: AdaptiveControllerDecision::RetrySameLevel {
                    route: state.current_route,
                    next_attempt,
                },
            }
        }
        AdaptiveClassification::EscalationEligible => {
            match next_route(state.starting_family, state.current_route) {
                AdaptiveTransition::EscalateEffort(to) => escalate(state, to, false),
                AdaptiveTransition::EscalateModel(to) => escalate(state, to, true),
                AdaptiveTransition::Blocked => block(state),
                AdaptiveTransition::InvalidState => AdaptiveControllerReduction {
                    state: AdaptiveControllerState {
                        terminal: Some(AdaptiveWorkflowTerminal::Blocked),
                        ..state
                    },
                    decision: AdaptiveControllerDecision::InvalidState,
                },
            }
        }
    }
}

fn no_action(state: AdaptiveControllerState) -> AdaptiveControllerReduction {
    AdaptiveControllerReduction {
        state,
        decision: AdaptiveControllerDecision::NoAction,
    }
}
fn block(state: AdaptiveControllerState) -> AdaptiveControllerReduction {
    AdaptiveControllerReduction {
        state: AdaptiveControllerState {
            terminal: Some(AdaptiveWorkflowTerminal::Blocked),
            ..state
        },
        decision: AdaptiveControllerDecision::Blocked,
    }
}
fn escalate(
    state: AdaptiveControllerState,
    to: AdaptiveRoute,
    changes_model: bool,
) -> AdaptiveControllerReduction {
    let next_attempt = state.attempt_number + 1;
    let decision = if changes_model {
        AdaptiveControllerDecision::EscalateModel {
            from: state.current_route,
            to,
            next_attempt,
        }
    } else {
        AdaptiveControllerDecision::EscalateEffort {
            from: state.current_route,
            to,
            next_attempt,
        }
    };
    AdaptiveControllerReduction {
        state: AdaptiveControllerState {
            current_route: to,
            attempt_number: next_attempt,
            transient_retry_consumed: false,
            ..state
        },
        decision,
    }
}
