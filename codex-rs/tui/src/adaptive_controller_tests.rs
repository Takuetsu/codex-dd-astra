use crate::adaptive_controller::*;
use crate::adaptive_policy::*;
use crate::adaptive_worker::AdaptiveWorkflowTerminal;
use pretty_assertions::assert_eq;

fn route(family: AdaptiveFamily, effort: AdaptiveEffort) -> AdaptiveRoute {
    AdaptiveRoute { family, effort }
}

#[test]
fn every_preference_starts_at_luna_low_and_capability_ladder_is_exact() {
    for family in [
        AdaptiveFamily::Luna,
        AdaptiveFamily::Terra,
        AdaptiveFamily::Sol,
        AdaptiveFamily::Astra,
    ] {
        assert_eq!(
            AdaptiveControllerState::initial(family).current_route,
            route(AdaptiveFamily::Luna, AdaptiveEffort::Low)
        );
    }

    let mut state = AdaptiveControllerState::initial(AdaptiveFamily::Astra);
    for expected in [
        route(AdaptiveFamily::Luna, AdaptiveEffort::Medium),
        route(AdaptiveFamily::Luna, AdaptiveEffort::High),
        route(AdaptiveFamily::Terra, AdaptiveEffort::Low),
        route(AdaptiveFamily::Terra, AdaptiveEffort::Medium),
        route(AdaptiveFamily::Terra, AdaptiveEffort::High),
        route(AdaptiveFamily::Sol, AdaptiveEffort::Low),
        route(AdaptiveFamily::Sol, AdaptiveEffort::Medium),
        route(AdaptiveFamily::Sol, AdaptiveEffort::High),
        route(AdaptiveFamily::Astra, AdaptiveEffort::Low),
        route(AdaptiveFamily::Astra, AdaptiveEffort::Medium),
        route(AdaptiveFamily::Astra, AdaptiveEffort::High),
        route(AdaptiveFamily::Astra, AdaptiveEffort::XHigh),
        route(AdaptiveFamily::Astra, AdaptiveEffort::Max),
    ] {
        let reduced = reduce_adaptive_controller(state, AdaptiveClassification::EscalationEligible);
        assert_eq!(reduced.state.current_route, expected);
        assert_eq!(reduced.state.attempt_number, state.attempt_number + 1);
        state = reduced.state;
    }
    let reduced = reduce_adaptive_controller(state, AdaptiveClassification::EscalationEligible);
    assert_eq!(reduced.decision, AdaptiveControllerDecision::Blocked);
    assert_eq!(reduced.state.attempt_number, 14);
}

#[test]
fn malformed_route_fails_closed_and_retry_is_route_local() {
    let state = AdaptiveControllerState {
        current_route: route(AdaptiveFamily::Luna, AdaptiveEffort::XHigh),
        ..AdaptiveControllerState::initial(AdaptiveFamily::Terra)
    };
    let invalid = reduce_adaptive_controller(state, AdaptiveClassification::EscalationEligible);
    assert_eq!(invalid.decision, AdaptiveControllerDecision::InvalidState);
    assert_eq!(invalid.state.attempt_number, 1);

    let state = AdaptiveControllerState::initial(AdaptiveFamily::Terra);
    let retry = reduce_adaptive_controller(state, AdaptiveClassification::RetrySameLevel);
    assert_eq!(
        retry.decision,
        AdaptiveControllerDecision::RetrySameLevel {
            route: route(AdaptiveFamily::Luna, AdaptiveEffort::Low),
            next_attempt: 2
        }
    );
    let blocked = reduce_adaptive_controller(retry.state, AdaptiveClassification::RetrySameLevel);
    assert_eq!(blocked.decision, AdaptiveControllerDecision::Blocked);
    assert_eq!(blocked.state.attempt_number, 2);
    let moved = reduce_adaptive_controller(state, AdaptiveClassification::EscalationEligible);
    assert!(!moved.state.transient_retry_consumed);
    assert_eq!(
        reduce_adaptive_controller(moved.state, AdaptiveClassification::RetrySameLevel).decision,
        AdaptiveControllerDecision::RetrySameLevel {
            route: route(AdaptiveFamily::Luna, AdaptiveEffort::Medium),
            next_attempt: 3
        }
    );
}

#[test]
fn terminals_pause_resume_and_no_decision_authorize_nothing() {
    let state = AdaptiveControllerState::initial(AdaptiveFamily::Terra);
    let qa = reduce_adaptive_controller(state, AdaptiveClassification::PassToOwnerQa);
    assert_eq!(qa.decision, AdaptiveControllerDecision::ReadyForOwnerQa);
    for signal in [
        AdaptiveClassification::EscalationEligible,
        AdaptiveClassification::RetrySameLevel,
        AdaptiveClassification::PassToOwnerQa,
    ] {
        assert_eq!(
            reduce_adaptive_controller(qa.state, signal).decision,
            AdaptiveControllerDecision::NoAction
        );
    }
    let paused = reduce_adaptive_controller(state, AdaptiveClassification::UserInterrupted);
    assert_eq!(paused.state.attempt_number, 1);
    assert_eq!(
        reduce_adaptive_controller(paused.state, AdaptiveClassification::RetrySameLevel).decision,
        AdaptiveControllerDecision::NoAction
    );
    assert_eq!(paused.state.resume(), state);
    assert_eq!(
        reduce_adaptive_controller(state, AdaptiveClassification::NoDecision),
        AdaptiveControllerReduction {
            state,
            decision: AdaptiveControllerDecision::NoAction
        }
    );
}

#[test]
fn every_workflow_terminal_is_an_idempotent_hard_stop() {
    for terminal in [
        AdaptiveWorkflowTerminal::ReadyForValidation,
        AdaptiveWorkflowTerminal::RepairRequired,
        AdaptiveWorkflowTerminal::ReadyForOwnerQa,
        AdaptiveWorkflowTerminal::Blocked,
    ] {
        let state = AdaptiveControllerState {
            terminal: Some(terminal),
            ..AdaptiveControllerState::initial(AdaptiveFamily::Terra)
        };
        for classification in [
            AdaptiveClassification::RetrySameLevel,
            AdaptiveClassification::EscalationEligible,
            AdaptiveClassification::PassToOwnerQa,
            AdaptiveClassification::Blocked,
        ] {
            assert_eq!(
                reduce_adaptive_controller(state, classification),
                AdaptiveControllerReduction {
                    state,
                    decision: AdaptiveControllerDecision::NoAction,
                }
            );
        }
    }
}

#[test]
fn repair_required_never_becomes_capability_or_repair_authorization() {
    let state = AdaptiveControllerState {
        terminal: Some(AdaptiveWorkflowTerminal::RepairRequired),
        ..AdaptiveControllerState::initial(AdaptiveFamily::Luna)
    };
    assert_eq!(
        reduce_adaptive_controller(state, AdaptiveClassification::EscalationEligible),
        AdaptiveControllerReduction {
            state,
            decision: AdaptiveControllerDecision::NoAction,
        }
    );
}

#[test]
fn reduction_is_deterministic() {
    let state = AdaptiveControllerState::initial(AdaptiveFamily::Astra);
    assert_eq!(
        reduce_adaptive_controller(state, AdaptiveClassification::EscalationEligible),
        reduce_adaptive_controller(state, AdaptiveClassification::EscalationEligible)
    );
}
