//! Pure deterministic routing for the D&D Bros adaptive effort ladder.
//!
//! This module deliberately has no TUI, session, event, or runtime dependencies. Callers decide
//! whether escalation is authorized; this policy only identifies the next legal route.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveFamily {
    Luna,
    Terra,
    Sol,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveEffort {
    Low,
    Medium,
    High,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveRoute {
    pub(crate) family: AdaptiveFamily,
    pub(crate) effort: AdaptiveEffort,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveTransition {
    EscalateEffort(AdaptiveRoute),
    EscalateModel(AdaptiveRoute),
    Blocked,
    InvalidState,
}

/// A normalized, typed attempt result supplied by a trusted runtime or controller producer.
///
/// Ordinary assistant prose and tool output are deliberately absent from this type, so they
/// cannot independently authorize retrying or spending stronger compute.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveOutcomeSignal {
    Completed,
    Failed,
    Failure(AdaptiveFailureKind),
    UserInterrupted,
    ReadyForOwnerQa,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveFailureKind {
    Capability,
    TransientInfrastructure,
    Authentication,
    Quota,
    Environment,
    Permission,
    Dependency,
    ModelUnavailable,
    OwnerDecision,
    AuthorizationOrScope,
    UserInterrupted,
    ReadyForOwnerQa,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveOutcome {
    Unknown,
    UserInterrupted,
    ReadyForOwnerQa,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveClassification {
    PassToOwnerQa,
    RetrySameLevel,
    EscalationEligible,
    Blocked,
    UserInterrupted,
    NoDecision,
}

pub(crate) fn classify_outcome(signal: AdaptiveOutcomeSignal) -> AdaptiveClassification {
    match signal {
        AdaptiveOutcomeSignal::Completed => AdaptiveClassification::NoDecision,
        AdaptiveOutcomeSignal::Failed => AdaptiveClassification::Blocked,
        AdaptiveOutcomeSignal::UserInterrupted
        | AdaptiveOutcomeSignal::Failure(AdaptiveFailureKind::UserInterrupted) => {
            AdaptiveClassification::UserInterrupted
        }
        AdaptiveOutcomeSignal::ReadyForOwnerQa
        | AdaptiveOutcomeSignal::Failure(AdaptiveFailureKind::ReadyForOwnerQa) => {
            AdaptiveClassification::PassToOwnerQa
        }
        AdaptiveOutcomeSignal::Failure(AdaptiveFailureKind::Capability) => {
            AdaptiveClassification::EscalationEligible
        }
        AdaptiveOutcomeSignal::Failure(AdaptiveFailureKind::TransientInfrastructure) => {
            AdaptiveClassification::RetrySameLevel
        }
        AdaptiveOutcomeSignal::Failure(
            AdaptiveFailureKind::Authentication
            | AdaptiveFailureKind::Quota
            | AdaptiveFailureKind::Environment
            | AdaptiveFailureKind::Permission
            | AdaptiveFailureKind::Dependency
            | AdaptiveFailureKind::ModelUnavailable
            | AdaptiveFailureKind::OwnerDecision
            | AdaptiveFailureKind::AuthorizationOrScope
            | AdaptiveFailureKind::Unknown,
        ) => AdaptiveClassification::Blocked,
    }
}

pub(crate) fn initial_route(starting_family: AdaptiveFamily) -> AdaptiveRoute {
    AdaptiveRoute {
        family: starting_family,
        effort: AdaptiveEffort::Low,
    }
}

pub(crate) fn next_route(
    starting_family: AdaptiveFamily,
    current_route: AdaptiveRoute,
) -> AdaptiveTransition {
    let ladder = match starting_family {
        AdaptiveFamily::Luna => &LUNA_LADDER[..],
        AdaptiveFamily::Terra => &TERRA_LADDER[..],
        AdaptiveFamily::Sol => &SOL_LADDER[..],
    };

    let Some(current_index) = ladder.iter().position(|route| *route == current_route) else {
        return AdaptiveTransition::InvalidState;
    };

    let Some(next_route) = ladder.get(current_index + 1).copied() else {
        return AdaptiveTransition::Blocked;
    };

    if next_route.family == current_route.family {
        AdaptiveTransition::EscalateEffort(next_route)
    } else {
        AdaptiveTransition::EscalateModel(next_route)
    }
}

const LUNA_LADDER: [AdaptiveRoute; 9] = [
    route(AdaptiveFamily::Luna, AdaptiveEffort::Low),
    route(AdaptiveFamily::Luna, AdaptiveEffort::Medium),
    route(AdaptiveFamily::Luna, AdaptiveEffort::High),
    route(AdaptiveFamily::Terra, AdaptiveEffort::Low),
    route(AdaptiveFamily::Terra, AdaptiveEffort::Medium),
    route(AdaptiveFamily::Terra, AdaptiveEffort::High),
    route(AdaptiveFamily::Sol, AdaptiveEffort::Low),
    route(AdaptiveFamily::Sol, AdaptiveEffort::Medium),
    route(AdaptiveFamily::Sol, AdaptiveEffort::High),
];

const TERRA_LADDER: [AdaptiveRoute; 6] = [
    route(AdaptiveFamily::Terra, AdaptiveEffort::Low),
    route(AdaptiveFamily::Terra, AdaptiveEffort::Medium),
    route(AdaptiveFamily::Terra, AdaptiveEffort::High),
    route(AdaptiveFamily::Sol, AdaptiveEffort::Low),
    route(AdaptiveFamily::Sol, AdaptiveEffort::Medium),
    route(AdaptiveFamily::Sol, AdaptiveEffort::High),
];

const SOL_LADDER: [AdaptiveRoute; 3] = [
    route(AdaptiveFamily::Sol, AdaptiveEffort::Low),
    route(AdaptiveFamily::Sol, AdaptiveEffort::Medium),
    route(AdaptiveFamily::Sol, AdaptiveEffort::High),
];

const fn route(family: AdaptiveFamily, effort: AdaptiveEffort) -> AdaptiveRoute {
    AdaptiveRoute { family, effort }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const LUNA_LOW: AdaptiveRoute = route(AdaptiveFamily::Luna, AdaptiveEffort::Low);
    const LUNA_MEDIUM: AdaptiveRoute = route(AdaptiveFamily::Luna, AdaptiveEffort::Medium);
    const LUNA_HIGH: AdaptiveRoute = route(AdaptiveFamily::Luna, AdaptiveEffort::High);
    const TERRA_LOW: AdaptiveRoute = route(AdaptiveFamily::Terra, AdaptiveEffort::Low);
    const TERRA_MEDIUM: AdaptiveRoute = route(AdaptiveFamily::Terra, AdaptiveEffort::Medium);
    const TERRA_HIGH: AdaptiveRoute = route(AdaptiveFamily::Terra, AdaptiveEffort::High);
    const SOL_LOW: AdaptiveRoute = route(AdaptiveFamily::Sol, AdaptiveEffort::Low);
    const SOL_MEDIUM: AdaptiveRoute = route(AdaptiveFamily::Sol, AdaptiveEffort::Medium);
    const SOL_HIGH: AdaptiveRoute = route(AdaptiveFamily::Sol, AdaptiveEffort::High);

    #[test]
    fn initial_routes_start_at_low_effort() {
        assert_eq!(initial_route(AdaptiveFamily::Luna), LUNA_LOW);
        assert_eq!(initial_route(AdaptiveFamily::Terra), TERRA_LOW);
        assert_eq!(initial_route(AdaptiveFamily::Sol), SOL_LOW);
    }

    #[test]
    fn luna_start_ladder_is_exact() {
        let transitions = [
            (LUNA_LOW, AdaptiveTransition::EscalateEffort(LUNA_MEDIUM)),
            (LUNA_MEDIUM, AdaptiveTransition::EscalateEffort(LUNA_HIGH)),
            (LUNA_HIGH, AdaptiveTransition::EscalateModel(TERRA_LOW)),
            (TERRA_LOW, AdaptiveTransition::EscalateEffort(TERRA_MEDIUM)),
            (TERRA_MEDIUM, AdaptiveTransition::EscalateEffort(TERRA_HIGH)),
            (TERRA_HIGH, AdaptiveTransition::EscalateModel(SOL_LOW)),
            (SOL_LOW, AdaptiveTransition::EscalateEffort(SOL_MEDIUM)),
            (SOL_MEDIUM, AdaptiveTransition::EscalateEffort(SOL_HIGH)),
            (SOL_HIGH, AdaptiveTransition::Blocked),
        ];

        for (current_route, expected) in transitions {
            assert_eq!(next_route(AdaptiveFamily::Luna, current_route), expected);
        }
    }

    #[test]
    fn terra_start_ladder_is_exact() {
        let transitions = [
            (TERRA_LOW, AdaptiveTransition::EscalateEffort(TERRA_MEDIUM)),
            (TERRA_MEDIUM, AdaptiveTransition::EscalateEffort(TERRA_HIGH)),
            (TERRA_HIGH, AdaptiveTransition::EscalateModel(SOL_LOW)),
            (SOL_LOW, AdaptiveTransition::EscalateEffort(SOL_MEDIUM)),
            (SOL_MEDIUM, AdaptiveTransition::EscalateEffort(SOL_HIGH)),
            (SOL_HIGH, AdaptiveTransition::Blocked),
        ];

        for (current_route, expected) in transitions {
            assert_eq!(next_route(AdaptiveFamily::Terra, current_route), expected);
        }
    }

    #[test]
    fn sol_start_ladder_is_exact() {
        let transitions = [
            (SOL_LOW, AdaptiveTransition::EscalateEffort(SOL_MEDIUM)),
            (SOL_MEDIUM, AdaptiveTransition::EscalateEffort(SOL_HIGH)),
            (SOL_HIGH, AdaptiveTransition::Blocked),
        ];

        for (current_route, expected) in transitions {
            assert_eq!(next_route(AdaptiveFamily::Sol, current_route), expected);
        }
    }

    #[test]
    fn transition_kinds_are_explicit() {
        assert_eq!(
            next_route(AdaptiveFamily::Luna, LUNA_LOW),
            AdaptiveTransition::EscalateEffort(LUNA_MEDIUM)
        );
        assert_eq!(
            next_route(AdaptiveFamily::Luna, LUNA_MEDIUM),
            AdaptiveTransition::EscalateEffort(LUNA_HIGH)
        );
        assert_eq!(
            next_route(AdaptiveFamily::Luna, LUNA_HIGH),
            AdaptiveTransition::EscalateModel(TERRA_LOW)
        );
        assert_eq!(
            next_route(AdaptiveFamily::Terra, TERRA_HIGH),
            AdaptiveTransition::EscalateModel(SOL_LOW)
        );
        assert_eq!(
            next_route(AdaptiveFamily::Sol, SOL_HIGH),
            AdaptiveTransition::Blocked
        );
    }

    #[test]
    fn lower_family_routes_are_invalid_for_higher_starting_families() {
        assert_eq!(
            next_route(AdaptiveFamily::Terra, LUNA_LOW),
            AdaptiveTransition::InvalidState
        );
        assert_eq!(
            next_route(AdaptiveFamily::Sol, TERRA_LOW),
            AdaptiveTransition::InvalidState
        );
        assert_eq!(
            next_route(AdaptiveFamily::Sol, LUNA_LOW),
            AdaptiveTransition::InvalidState
        );
    }

    #[test]
    fn lower_starting_family_routes_remain_valid_without_a_downgrade() {
        assert_eq!(
            next_route(
                AdaptiveFamily::Luna,
                route(AdaptiveFamily::Luna, AdaptiveEffort::High)
            ),
            AdaptiveTransition::EscalateModel(TERRA_LOW)
        );
        assert_eq!(
            next_route(
                AdaptiveFamily::Terra,
                route(AdaptiveFamily::Terra, AdaptiveEffort::Low)
            ),
            AdaptiveTransition::EscalateEffort(TERRA_MEDIUM)
        );
    }

    #[test]
    fn automatic_ladders_only_contain_low_medium_and_high_effort() {
        for route in LUNA_LADDER
            .iter()
            .chain(TERRA_LADDER.iter())
            .chain(SOL_LADDER.iter())
        {
            assert!(matches!(
                route.effort,
                AdaptiveEffort::Low | AdaptiveEffort::Medium | AdaptiveEffort::High
            ));
        }
    }

    #[test]
    fn no_transition_decreases_family_or_same_family_effort() {
        for (starting_family, ladder) in [
            (AdaptiveFamily::Luna, &LUNA_LADDER[..]),
            (AdaptiveFamily::Terra, &TERRA_LADDER[..]),
            (AdaptiveFamily::Sol, &SOL_LADDER[..]),
        ] {
            for (current_route, expected_route) in ladder.iter().zip(ladder.iter().skip(1)) {
                match next_route(starting_family, *current_route) {
                    AdaptiveTransition::EscalateEffort(next_route)
                    | AdaptiveTransition::EscalateModel(next_route) => {
                        assert_eq!(next_route, *expected_route);
                    }
                    transition => panic!("expected a route transition, got {transition:?}"),
                }
            }
        }
    }

    #[test]
    fn repeated_calls_are_deterministic() {
        let route = TERRA_HIGH;
        assert_eq!(
            next_route(AdaptiveFamily::Luna, route),
            next_route(AdaptiveFamily::Luna, route)
        );
    }
}
