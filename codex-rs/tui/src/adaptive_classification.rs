//! Narrow adapters from existing typed Codex facts into the pure adaptive classifier.
//!
//! This module does not inspect error messages, assistant prose, command output, or test output.

use crate::adaptive_policy::AdaptiveFailureKind;
use crate::adaptive_policy::AdaptiveOutcomeSignal;
use codex_app_server_protocol::CodexErrorInfo;
use codex_app_server_protocol::TurnStatus;

pub(crate) fn signal_from_turn_status(status: &TurnStatus) -> AdaptiveOutcomeSignal {
    match status {
        TurnStatus::Completed => AdaptiveOutcomeSignal::Completed,
        TurnStatus::Failed | TurnStatus::Interrupted | TurnStatus::InProgress => {
            AdaptiveOutcomeSignal::Failed
        }
    }
}

pub(crate) fn signal_from_user_interrupt() -> AdaptiveOutcomeSignal {
    AdaptiveOutcomeSignal::UserInterrupted
}

pub(crate) fn signal_from_codex_error(error: &CodexErrorInfo) -> AdaptiveOutcomeSignal {
    AdaptiveOutcomeSignal::Failure(match error {
        CodexErrorInfo::RateLimitExceeded
        | CodexErrorInfo::ServerOverloaded
        | CodexErrorInfo::HttpConnectionFailed { .. }
        | CodexErrorInfo::ResponseStreamConnectionFailed { .. }
        | CodexErrorInfo::InternalServerError
        | CodexErrorInfo::ResponseStreamDisconnected { .. }
        | CodexErrorInfo::ResponseTooManyFailedAttempts { .. } => {
            AdaptiveFailureKind::TransientInfrastructure
        }
        CodexErrorInfo::Unauthorized => AdaptiveFailureKind::Authentication,
        CodexErrorInfo::SessionBudgetExceeded | CodexErrorInfo::UsageLimitExceeded => {
            AdaptiveFailureKind::Quota
        }
        CodexErrorInfo::ContextWindowExceeded | CodexErrorInfo::ThreadRollbackFailed => {
            AdaptiveFailureKind::Environment
        }
        CodexErrorInfo::SandboxError => AdaptiveFailureKind::Permission,
        CodexErrorInfo::CyberPolicy | CodexErrorInfo::MisalignmentPolicyViolation => {
            AdaptiveFailureKind::AuthorizationOrScope
        }
        CodexErrorInfo::BadRequest
        | CodexErrorInfo::ActiveTurnNotSteerable { .. }
        | CodexErrorInfo::Other => AdaptiveFailureKind::Unknown,
    })
}

#[cfg(test)]
#[path = "adaptive_classification_tests.rs"]
mod tests;
