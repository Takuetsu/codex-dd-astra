use crate::adaptive_policy::AdaptiveClassification;
use crate::adaptive_policy::AdaptiveFailureKind;
use crate::adaptive_policy::AdaptiveOutcomeSignal;
use crate::adaptive_policy::classify_outcome;
use pretty_assertions::assert_eq;

#[test]
fn typed_failure_taxonomy_maps_to_safe_decisions() {
    let cases = [
        (
            AdaptiveFailureKind::Capability,
            AdaptiveClassification::EscalationEligible,
        ),
        (
            AdaptiveFailureKind::TransientInfrastructure,
            AdaptiveClassification::RetrySameLevel,
        ),
        (
            AdaptiveFailureKind::Authentication,
            AdaptiveClassification::Blocked,
        ),
        (AdaptiveFailureKind::Quota, AdaptiveClassification::Blocked),
        (
            AdaptiveFailureKind::Environment,
            AdaptiveClassification::Blocked,
        ),
        (
            AdaptiveFailureKind::Permission,
            AdaptiveClassification::Blocked,
        ),
        (
            AdaptiveFailureKind::Dependency,
            AdaptiveClassification::Blocked,
        ),
        (
            AdaptiveFailureKind::ModelUnavailable,
            AdaptiveClassification::Blocked,
        ),
        (
            AdaptiveFailureKind::OwnerDecision,
            AdaptiveClassification::Blocked,
        ),
        (
            AdaptiveFailureKind::AuthorizationOrScope,
            AdaptiveClassification::Blocked,
        ),
        (
            AdaptiveFailureKind::Unknown,
            AdaptiveClassification::Blocked,
        ),
        (
            AdaptiveFailureKind::UserInterrupted,
            AdaptiveClassification::UserInterrupted,
        ),
        (
            AdaptiveFailureKind::ReadyForOwnerQa,
            AdaptiveClassification::PassToOwnerQa,
        ),
    ];

    for (failure, expected) in cases {
        assert_eq!(
            classify_outcome(AdaptiveOutcomeSignal::Failure(failure)),
            expected
        );
    }
}

#[test]
fn only_explicit_typed_signals_authorize_capability_or_owner_qa() {
    assert_eq!(
        classify_outcome(AdaptiveOutcomeSignal::Failed),
        AdaptiveClassification::Blocked
    );
    assert_eq!(
        classify_outcome(AdaptiveOutcomeSignal::Completed),
        AdaptiveClassification::NoDecision
    );
    assert_eq!(
        classify_outcome(AdaptiveOutcomeSignal::Failure(
            AdaptiveFailureKind::Capability
        )),
        AdaptiveClassification::EscalationEligible
    );
    assert_eq!(
        classify_outcome(AdaptiveOutcomeSignal::ReadyForOwnerQa),
        AdaptiveClassification::PassToOwnerQa
    );
}

#[test]
fn classification_is_deterministic_side_effect_free_and_does_not_mutate_attempt_number() {
    let signal = AdaptiveOutcomeSignal::Failure(AdaptiveFailureKind::Capability);
    let attempt_number = 7;
    assert_eq!(classify_outcome(signal), classify_outcome(signal));
    assert_eq!(attempt_number, 7);
}
