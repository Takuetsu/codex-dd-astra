//! Neutral Worker assignment and workflow handoff facts for adaptive state.

use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_config::config_toml::AdaptiveWorkerRoleToml;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum AdaptiveWorkerRole {
    #[default]
    Unspecified,
    Implementation,
    Validation,
    Repair,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct AdaptiveWorkerContext {
    pub(crate) role: AdaptiveWorkerRole,
    pub(crate) authorized_scope: Option<String>,
}

impl From<AdaptiveWorkerConfigToml> for AdaptiveWorkerContext {
    fn from(binding: AdaptiveWorkerConfigToml) -> Self {
        let role = match binding.role {
            AdaptiveWorkerRoleToml::Unspecified => AdaptiveWorkerRole::Unspecified,
            AdaptiveWorkerRoleToml::Implementation => AdaptiveWorkerRole::Implementation,
            AdaptiveWorkerRoleToml::Validation => AdaptiveWorkerRole::Validation,
            AdaptiveWorkerRoleToml::Repair => AdaptiveWorkerRole::Repair,
        };
        Self {
            role,
            authorized_scope: binding.authorized_scope,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveWorkflowTerminal {
    ReadyForValidation,
    RepairRequired,
    ReadyForOwnerQa,
    Blocked,
}
