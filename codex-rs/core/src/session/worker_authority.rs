//! Core-owned first accepted user message Worker assignment.
//! This is an admission latch, NOT a durable native-validation credential.

use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_config::config_toml::AdaptiveWorkerRoleToml;
use codex_config::config_toml::parse_adaptive_worker_first_message;
use codex_protocol::protocol::CodexddFirstWorkerSnapshot;
use std::path::Path;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct CoreFirstWorkerAuthority {
    locked: bool,
    binding: Option<AdaptiveWorkerConfigToml>,
    // Launcher pins are not serialized as first-message credentials.
    accepted_user_origin: bool,
    // Only a flushed Core checkpoint (or a verified same-thread resume)
    // can enable a subsequent native TurnContext source credential.
    checkpointed: bool,
}

impl CoreFirstWorkerAuthority {
    pub(crate) fn from_startup(binding: Option<AdaptiveWorkerConfigToml>) -> Self {
        Self {
            locked: binding.is_some(),
            binding,
            accepted_user_origin: false,
            checkpointed: false,
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

    pub(crate) fn accept(&mut self, proposed: Option<Option<AdaptiveWorkerConfigToml>>) -> bool {
        if self.locked {
            return false;
        }
        let Some(binding) = proposed else {
            return false;
        };
        self.accepted_user_origin = binding.is_some();
        self.binding = binding;
        self.locked = true;
        self.checkpointed = false;
        true
    }

    /// Mark durable only after the native thread store confirmed append and flush.
    /// No model/tool argument may call this gate.
    pub(crate) fn mark_checkpointed(&mut self) -> bool {
        if !self.locked || !self.accepted_user_origin || self.binding.is_none() {
            return false;
        }
        self.checkpointed = true;
        true
    }

    /// Only persisted Core user authority may reach a subsequent native turn.
    /// Launcher-configured roles are already applied in the ordinary Config.
    pub(crate) fn runtime_binding(&self) -> Option<AdaptiveWorkerConfigToml> {
        if !self.locked || !self.accepted_user_origin || !self.checkpointed {
            return None;
        }
        self.binding.clone()
    }

    /// Only accepted first-user-message bindings go into trusted Core rollout
    /// settings. Startup bindings stay in launcher config, and plain prompts
    /// never acquire a stored role.
    pub(crate) fn first_user_snapshot(&self) -> Option<CodexddFirstWorkerSnapshot> {
        if !self.accepted_user_origin {
            return None;
        }
        let binding = self.binding.as_ref()?;
        let role = match binding.role {
            AdaptiveWorkerRoleToml::Implementation => "implementation",
            AdaptiveWorkerRoleToml::Validation => "validation",
            AdaptiveWorkerRoleToml::Repair => "repair",
            AdaptiveWorkerRoleToml::Unspecified => return None,
        };
        Some(CodexddFirstWorkerSnapshot {
            role: role.to_string(),
            authorized_scope: binding.authorized_scope.clone().unwrap_or_default(),
            source_root: binding.source_root.clone(),
            source_sha: binding.source_sha.clone(),
            task_kind: binding.task_kind.clone(),
        })
    }

    /// Rehydrate only an exact-thread native settings snapshot. The caller
    /// verifies its owner and the presence of previous user history first.
    /// An explicit launcher binding wins over a persisted first-message role.
    pub(crate) fn restore_first_user_snapshot(
        &mut self,
        snapshot: &CodexddFirstWorkerSnapshot,
    ) -> Result<bool, String> {
        if self.locked {
            return Ok(false);
        }
        let role = match snapshot.role.as_str() {
            "implementation" => AdaptiveWorkerRoleToml::Implementation,
            "validation" => AdaptiveWorkerRoleToml::Validation,
            "repair" => AdaptiveWorkerRoleToml::Repair,
            _ => return Err("persisted Worker role is invalid".to_string()),
        };
        let scope = snapshot.authorized_scope.trim();
        if scope.is_empty() {
            return Err("persisted Worker authorized_scope is empty".to_string());
        }
        let task_kind = snapshot.task_kind.as_deref().unwrap_or("source_change");
        if !matches!(
            task_kind,
            "source_change" | "operations_preview" | "benchmark"
        ) {
            return Err("persisted Worker task_kind is invalid".to_string());
        }
        let (source_root, source_sha) = match (
            snapshot.source_root.as_deref(),
            snapshot.source_sha.as_deref(),
        ) {
            (None, None) if task_kind == "source_change" => (None, None),
            (Some(root), Some(sha))
                if Path::new(root).is_absolute()
                    && sha.len() == 40
                    && sha.bytes().all(|ch| ch.is_ascii_hexdigit()) =>
            {
                (Some(root.to_string()), Some(sha.to_ascii_lowercase()))
            }
            _ => {
                return Err(
                    "persisted Worker source requires exact root and 40-digit SHA".to_string(),
                );
            }
        };
        self.binding = Some(AdaptiveWorkerConfigToml {
            role,
            authorized_scope: Some(scope.to_string()),
            source_root,
            source_sha,
            task_kind: snapshot.task_kind.clone(),
        });
        self.accepted_user_origin = true;
        self.locked = true;
        self.checkpointed = true;
        Ok(true)
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

    const IMPLEMENT: &str = "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"approved source\"\n\nImplement.";
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
        assert!(
            state
                .prepare(Some("[adaptive_worker]\nrole = \"invalid\""))
                .is_err()
        );
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
    #[test]
    fn codexdd_046_3b2g_first_user_snapshot_roundtrips_without_replaying_prompt() {
        let root = if cfg!(windows) {
            r"C:\authorized\worktree"
        } else {
            "/tmp/authorized/worktree"
        };
        let prompt = format!(
            "[adaptive_worker]\\nrole = \"implementation\"\\nauthorized_scope = \"source packet\"\\nsource_root = '{root}'\\nsource_sha = \"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\"\\n\\nImplement."
        );
        let prompt = prompt.replace("\\n", "\n");
        let mut original = CoreFirstWorkerAuthority::default();
        assert!(original.accept(original.prepare(Some(&prompt)).expect("parsed")));
        let snapshot = original
            .first_user_snapshot()
            .expect("native first-user binding");
        let serialized = serde_json::to_string(&snapshot).expect("encode");
        let decoded: CodexddFirstWorkerSnapshot =
            serde_json::from_str(&serialized).expect("decode");
        let mut resumed = CoreFirstWorkerAuthority::default();
        assert_eq!(resumed.restore_first_user_snapshot(&decoded), Ok(true));
        assert_eq!(resumed.prepare(Some(REPAIR)), Ok(None));
        assert_eq!(resumed.binding(), original.binding());
    }

    #[test]
    fn codexdd_046_3b2g_rejects_tampered_or_partial_native_snapshot() {
        let mut state = CoreFirstWorkerAuthority::default();
        let mut snapshot = CodexddFirstWorkerSnapshot {
            role: "implementation".to_string(),
            authorized_scope: "approved source".to_string(),
            source_root: Some(
                if cfg!(windows) {
                    r"C:\source\repo"
                } else {
                    "/tmp/source/repo"
                }
                .to_string(),
            ),
            source_sha: Some("a".repeat(40)),
            task_kind: Some("source_change".to_string()),
        };
        snapshot.source_sha = Some("not-a-commit".to_string());
        assert!(state.restore_first_user_snapshot(&snapshot).is_err());
        assert!(!state.locked());
        snapshot.source_sha = None;
        assert!(state.restore_first_user_snapshot(&snapshot).is_err());
        snapshot.source_root = None;
        snapshot.role = "unspecified".to_string();
        assert!(state.restore_first_user_snapshot(&snapshot).is_err());
        assert!(state.binding().is_none());
    }

    #[test]
    fn codexdd_046_3b2g_startup_and_plain_first_message_have_no_user_snapshot() {
        let mut plain = CoreFirstWorkerAuthority::default();
        assert!(
            plain.accept(
                plain
                    .prepare(Some("Ordinary first user task."))
                    .expect("plain")
            )
        );
        assert!(plain.first_user_snapshot().is_none());
        let startup = CoreFirstWorkerAuthority::from_startup(
            parse_adaptive_worker_first_message(IMPLEMENT).expect("launcher fixture"),
        );
        assert!(startup.first_user_snapshot().is_none());
        let mut pinned = startup.clone();
        let proposed = CodexddFirstWorkerSnapshot {
            role: "repair".to_string(),
            authorized_scope: "replayed override".to_string(),
            source_root: None,
            source_sha: None,
            task_kind: None,
        };
        assert_eq!(pinned.restore_first_user_snapshot(&proposed), Ok(false));
        assert_eq!(pinned.binding(), startup.binding());
    }

    #[test]
    fn codexdd_046_3b2h_unflushed_authority_cannot_enter_runtime() {
        let mut first = CoreFirstWorkerAuthority::default();
        let proposed = first.prepare(Some(IMPLEMENT)).expect("first header");
        assert!(first.accept(proposed));
        assert!(first.first_user_snapshot().is_some());
        assert!(first.runtime_binding().is_none());
        assert!(first.mark_checkpointed());
        assert_eq!(first.runtime_binding(), first.binding().cloned());
    }

    #[test]
    fn codexdd_046_3b2h_plain_or_launcher_authority_cannot_forge_checkpoint() {
        let mut plain = CoreFirstWorkerAuthority::default();
        let proposed = plain
            .prepare(Some("Ordinary first message"))
            .expect("plain");
        assert!(plain.accept(proposed));
        assert!(!plain.mark_checkpointed());
        assert!(plain.runtime_binding().is_none());

        let mut launcher = CoreFirstWorkerAuthority::from_startup(
            parse_adaptive_worker_first_message(REPAIR).expect("launcher binding"),
        );
        assert!(!launcher.mark_checkpointed());
        assert!(launcher.runtime_binding().is_none());
    }
}
