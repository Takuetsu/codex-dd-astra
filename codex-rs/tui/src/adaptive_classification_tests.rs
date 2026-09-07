use super::*;
use crate::adaptive_policy::AdaptiveClassification;
use crate::adaptive_policy::AdaptiveFailureKind;
use crate::adaptive_policy::AdaptiveOutcomeSignal;
use crate::adaptive_policy::classify_outcome;
use codex_app_server_protocol::NonSteerableTurnKind;
use pretty_assertions::assert_eq;

#[test]
fn typed_codex_errors_normalize_without_text_inference() {
    let cases = [
        (
            CodexErrorInfo::ServerOverloaded,
            AdaptiveFailureKind::TransientInfrastructure,
        ),
        (
            CodexErrorInfo::Unauthorized,
            AdaptiveFailureKind::Authentication,
        ),
        (
            CodexErrorInfo::UsageLimitExceeded,
            AdaptiveFailureKind::Quota,
        ),
        (
            CodexErrorInfo::ContextWindowExceeded,
            AdaptiveFailureKind::Environment,
        ),
        (
            CodexErrorInfo::SandboxError,
            AdaptiveFailureKind::Permission,
        ),
        (
            CodexErrorInfo::CyberPolicy,
            AdaptiveFailureKind::AuthorizationOrScope,
        ),
        (CodexErrorInfo::Other, AdaptiveFailureKind::Unknown),
    ];

    for (error, expected) in cases {
        assert_eq!(
            signal_from_codex_error(&error),
            AdaptiveOutcomeSignal::Failure(expected)
        );
    }
}

#[test]
fn generic_turn_statuses_fail_closed() {
    assert_eq!(
        classify_outcome(signal_from_turn_status(&TurnStatus::Failed)),
        AdaptiveClassification::Blocked
    );
    assert_eq!(
        classify_outcome(signal_from_turn_status(&TurnStatus::Completed)),
        AdaptiveClassification::NoDecision
    );
    assert_eq!(
        classify_outcome(signal_from_turn_status(&TurnStatus::Interrupted)),
        AdaptiveClassification::Blocked
    );
}

#[test]
fn failed_tests_and_nonzero_tool_exits_have_no_authorizing_adapter() {
    let failed_test_turn = signal_from_turn_status(&TurnStatus::Failed);
    let nonzero_tool_exit_turn = signal_from_turn_status(&TurnStatus::Failed);
    assert_eq!(
        classify_outcome(failed_test_turn),
        AdaptiveClassification::Blocked
    );
    assert_eq!(
        classify_outcome(nonzero_tool_exit_turn),
        AdaptiveClassification::Blocked
    );
}

#[test]
fn assistant_text_has_no_classification_input() {
    for _untrusted_text in ["ready for owner QA", "capability failure"] {
        assert_eq!(
            classify_outcome(signal_from_turn_status(&TurnStatus::Completed)),
            AdaptiveClassification::NoDecision
        );
    }
}

#[test]
fn confirmed_user_interrupt_is_explicit_and_never_retries_or_escalates() {
    assert_eq!(
        classify_outcome(signal_from_user_interrupt()),
        AdaptiveClassification::UserInterrupted
    );
}

#[test]
fn ambiguous_typed_errors_fail_closed() {
    let cases = [
        CodexErrorInfo::BadRequest,
        CodexErrorInfo::ActiveTurnNotSteerable {
            turn_kind: NonSteerableTurnKind::Review,
        },
        CodexErrorInfo::Other,
    ];
    for error in cases {
        assert_eq!(
            classify_outcome(signal_from_codex_error(&error)),
            AdaptiveClassification::Blocked
        );
    }
}
