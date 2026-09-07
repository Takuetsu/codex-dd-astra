use super::adaptive_worker::AdaptiveWorkerContext;
use super::adaptive_worker::AdaptiveWorkerRole;
use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_config::config_toml::AdaptiveWorkerRoleToml;
use codex_config::config_toml::ConfigToml;
use pretty_assertions::assert_eq;

#[test]
fn absent_external_binding_defaults_to_unspecified() {
    assert_eq!(
        AdaptiveWorkerContext::default().role,
        AdaptiveWorkerRole::Unspecified
    );
    assert_eq!(AdaptiveWorkerContext::default().authorized_scope, None);
}

#[test]
fn every_external_role_and_scope_binds_canonically() {
    for (external, expected) in [
        (
            AdaptiveWorkerRoleToml::Unspecified,
            AdaptiveWorkerRole::Unspecified,
        ),
        (
            AdaptiveWorkerRoleToml::Implementation,
            AdaptiveWorkerRole::Implementation,
        ),
        (
            AdaptiveWorkerRoleToml::Validation,
            AdaptiveWorkerRole::Validation,
        ),
        (AdaptiveWorkerRoleToml::Repair, AdaptiveWorkerRole::Repair),
    ] {
        assert_eq!(
            AdaptiveWorkerContext::from(AdaptiveWorkerConfigToml {
                role: external,
                authorized_scope: Some("opaque/scope".to_string()),
            }),
            AdaptiveWorkerContext {
                role: expected,
                authorized_scope: Some("opaque/scope".to_string()),
            }
        );
    }
}

#[test]
fn invalid_external_role_fails_closed_during_config_parsing() {
    let error = toml::from_str::<ConfigToml>(
        r#"
        [adaptive_worker]
        role = "owner"
        authorized_scope = "opaque/scope"
        "#,
    )
    .expect_err("unknown role must be rejected");

    assert!(error.to_string().contains("unknown variant"));
}
