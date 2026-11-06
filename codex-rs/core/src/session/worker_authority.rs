//! Core-owned first accepted user message Worker assignment.
//! This is an admission latch, NOT a durable native-validation credential.

use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_config::config_toml::parse_adaptive_worker_first_message;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct CoreFirstWorkerAuthority {
    locked: bool,
    binding: Option<AdaptiveWorkerConfigToml>,
}

impl CoreFirstWorkerAuthority {
    pub(crate) fn from_startup(binding: Option<AdaptiveWorkerConfigToml>) -> Self {
        Self {
            locked: binding.is_some(),
            binding,
        }
    }

    pub(crate) fn lock_after_prior_user_history(&mut self) {
        // No Core-authenticated persisted binding exists yet. Never parse
        // replayed prompt text as new authority after cold resume.
        self.locked = true;
    }

    pub(crate) fn prepare(
        &self,
        first_text: Option<&str>,
    ) -> Result<Option<Option<AdaptiveWorkerConfigToml>>, String> {
        if self.locked {
            return Ok(None);
        }
        parse_adaptive_worker_first_message(first_text.unwrap_or("")).map(Some)
    }

    pub(crate) fn accept(
        &mut self,
        proposed: Option<Option<AdaptiveWorkerConfigToml>>,
    ) -> bool {
        if self.locked {
            return false;
        }
        let Some(binding) = proposed else {
            return false;
        };
        self.binding = binding;
        self.locked = true;
        true
    }

    #[cfg(test)]
    pub(crate) fn binding(&self) -> Option<&AdaptiveWorkerConfigToml> {
        self.binding.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn locked(&self) -> bool {
        self.locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_config::config_toml::AdaptiveWorkerRoleToml;

    const IMPLEMENT: &str =
        "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"approved source\"\n\nImplement.";
    const REPAIR: &str =
        "[adaptive_worker]\nrole = \"repair\"\nauthorized_scope = \"late spoof\"\n\nRepair.";

    #[test]
    fn codexdd_046_3b2f_first_accepted_role_is_immutable() {
        let mut state = CoreFirstWorkerAuthority::default();
        let proposal = state.prepare(Some(IMPLEMENT)).expect("valid header");
        assert!(!state.locked());
        assert!(state.accept(proposal));
        assert!(state.locked());
        assert_eq!(
            state.binding().map(|binding| binding.role),
            Some(AdaptiveWorkerRoleToml::Implementation)
        );
        assert_eq!(state.prepare(Some(REPAIR)), Ok(None));
        assert!(!state.accept(None));
    }

    #[test]
    fn codexdd_046_3b2f_plain_or_rejected_first_turn_is_safe() {
        let mut state = CoreFirstWorkerAuthority::default();
        assert!(state.prepare(Some("[adaptive_worker]\nrole = \"invalid\"")).is_err());
        assert!(!state.locked());
        let plain = state.prepare(Some("Normal user prompt.")).expect("plain");
        assert_eq!(plain, Some(None));
        assert!(state.accept(plain));
        assert!(state.locked());
        assert_eq!(state.prepare(Some(IMPLEMENT)), Ok(None));
        assert!(state.binding().is_none());
    }

    #[test]
    fn codexdd_046_3b2f_startup_and_cold_resume_do_not_rebind() {
        let mut startup = CoreFirstWorkerAuthority::from_startup(
            parse_adaptive_worker_first_message(REPAIR).expect("header"),
        );
        assert_eq!(startup.prepare(Some(IMPLEMENT)), Ok(None));
        startup.lock_after_prior_user_history();
        assert_eq!(
            startup.binding().map(|binding| binding.role),
            Some(AdaptiveWorkerRoleToml::Repair)
        );
        let mut resume = CoreFirstWorkerAuthority::default();
        resume.lock_after_prior_user_history();
        assert_eq!(resume.prepare(Some(IMPLEMENT)), Ok(None));
        assert!(resume.binding().is_none());
    }
}
