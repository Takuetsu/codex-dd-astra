//! Pure deterministic routing for the D&D Bros adaptive effort ladder.
//!
//! This module deliberately has no TUI, session, event, or runtime dependencies. Callers decide
//! whether escalation is authorized; this policy only identifies the next legal route.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveFamily {
    Luna,
    Terra,
    Sol,
    Astra,
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

/// Every adaptive run starts on Luna Low. `starting_family` is retained as preference metadata,
/// but it does not bypass the cheap first attempt.
pub(crate) fn initial_route(_starting_family: AdaptiveFamily) -> AdaptiveRoute {
    route(AdaptiveFamily::Luna, AdaptiveEffort::Low)
}

/// Advance through the single automatic ladder. The preference does not remove lower rungs.
pub(crate) fn next_route(
    _starting_family: AdaptiveFamily,
    current_route: AdaptiveRoute,
) -> AdaptiveTransition {
    let Some(current_index) = AUTOMATIC_LADDER
        .iter()
        .position(|route| *route == current_route)
    else {
        return AdaptiveTransition::InvalidState;
    };

    let Some(next_route) = AUTOMATIC_LADDER.get(current_index + 1).copied() else {
        return AdaptiveTransition::Blocked;
    };

    if next_route.family == current_route.family {
        AdaptiveTransition::EscalateEffort(next_route)
    } else {
        AdaptiveTransition::EscalateModel(next_route)
    }
}

/// Phase one ends at Astra High. Astra XHigh/Max are added after the native failure-pressure
/// mechanism passes its end-to-end validation, so widening the effort enum cannot obscure the
/// core escalation test. Ultra remains separately gated and will not be part of this ladder.
const AUTOMATIC_LADDER: [AdaptiveRoute; 12] = [
    route(AdaptiveFamily::Luna, AdaptiveEffort::Low),
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
    const ASTRA_LOW: AdaptiveRoute = route(AdaptiveFamily::Astra, AdaptiveEffort::Low);
    const ASTRA_MEDIUM: AdaptiveRoute = route(AdaptiveFamily::Astra, AdaptiveEffort::Medium);
    const ASTRA_HIGH: AdaptiveRoute = route(AdaptiveFamily::Astra, AdaptiveEffort::High);

    #[test]
    fn every_preference_starts_at_luna_low() {
        for family in [
            AdaptiveFamily::Luna,
            AdaptiveFamily::Terra,
            AdaptiveFamily::Sol,
            AdaptiveFamily::Astra,
        ] {
            assert_eq!(initial_route(family), LUNA_LOW);
        }
    }

    #[test]
    fn automatic_ladder_is_exact_for_every_preference() {
        let transitions = [
            (LUNA_LOW, AdaptiveTransition::EscalateEffort(LUNA_MEDIUM)),
            (LUNA_MEDIUM, AdaptiveTransition::EscalateEffort(LUNA_HIGH)),
            (LUNA_HIGH, AdaptiveTransition::EscalateModel(TERRA_LOW)),
            (TERRA_LOW, AdaptiveTransition::EscalateEffort(TERRA_MEDIUM)),
            (TERRA_MEDIUM, AdaptiveTransition::EscalateEffort(TERRA_HIGH)),
            (TERRA_HIGH, AdaptiveTransition::EscalateModel(SOL_LOW)),
            (SOL_LOW, AdaptiveTransition::EscalateEffort(SOL_MEDIUM)),
            (SOL_MEDIUM, AdaptiveTransition::EscalateEffort(SOL_HIGH)),
            (SOL_HIGH, AdaptiveTransition::EscalateModel(ASTRA_LOW)),
            (ASTRA_LOW, AdaptiveTransition::EscalateEffort(ASTRA_MEDIUM)),
            (ASTRA_MEDIUM, AdaptiveTransition::EscalateEffort(ASTRA_HIGH)),
            (ASTRA_HIGH, AdaptiveTransition::Blocked),
        ];

        for preference in [
            AdaptiveFamily::Luna,
            AdaptiveFamily::Terra,
            AdaptiveFamily::Sol,
            AdaptiveFamily::Astra,
        ] {
            for (current_route, expected) in transitions {
                assert_eq!(next_route(preference, current_route), expected);
            }
        }
    }

    #[test]
    fn non_ladder_routes_fail_closed() {
        let invalid = AdaptiveRoute {
            family: AdaptiveFamily::Astra,
            effort: AdaptiveEffort::Low,
        };
        assert_eq!(
            next_route(AdaptiveFamily::Luna, invalid),
            AdaptiveTransition::EscalateEffort(ASTRA_MEDIUM)
        );
    }

    #[test]
    fn no_transition_decreases_family_or_same_family_effort() {
        for (current_route, expected_route) in AUTOMATIC_LADDER
            .iter()
            .zip(AUTOMATIC_LADDER.iter().skip(1))
        {
            match next_route(AdaptiveFamily::Luna, *current_route) {
                AdaptiveTransition::EscalateEffort(next_route)
                | AdaptiveTransition::EscalateModel(next_route) => {
                    assert_eq!(next_route, *expected_route);
                }
                transition => panic!("expected a route transition, got {transition:?}"),
            }
        }
    }

    #[test]
    fn repeated_calls_are_deterministic() {
        assert_eq!(
            next_route(AdaptiveFamily::Astra, SOL_HIGH),
            next_route(AdaptiveFamily::Astra, SOL_HIGH)
        );
    }
}
