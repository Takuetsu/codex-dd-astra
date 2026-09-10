//! Neutral Worker assignment and workflow handoff facts for adaptive state.

use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_config::config_toml::AdaptiveWorkerRoleToml;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum AdaptiveWorkerRole {
    #[default]
    Unspecified,
    Implementation,
    Validation,
    Repair,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NewWorkerBinding {
    pub(crate) role: AdaptiveWorkerRole,
    pub(crate) authorized_scope: String,
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

pub(crate) fn parse_new_worker_role(value: &str) -> Option<AdaptiveWorkerRole> {
    match value {
        "implementation" => Some(AdaptiveWorkerRole::Implementation),
        "validation" => Some(AdaptiveWorkerRole::Validation),
        "repair" => Some(AdaptiveWorkerRole::Repair),
        _ => None,
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdaptiveWorkerAssignmentDocument {
    adaptive_worker: AdaptiveWorkerAssignmentTable,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AdaptiveWorkerAssignmentTable {
    role: String,
    authorized_scope: String,
}

/// Parses the authority header on the first user-supplied Worker assignment.
///
/// Authority is recognized only when the first non-blank line is exactly the
/// `[adaptive_worker]` table. The table must end at a blank line so ordinary
/// assignment prose can never be interpreted as TOML authority. A message
/// without the header returns `Ok(None)`; a malformed authority header fails
/// closed with an error.
pub(crate) fn parse_adaptive_worker_assignment(
    text: &str,
) -> Result<Option<AdaptiveWorkerContext>, String> {
    let mut lines = text.lines();
    let first_non_blank = lines.by_ref().find(|line| !line.trim().is_empty());
    let Some(first_non_blank) = first_non_blank else {
        return Ok(None);
    };
    if first_non_blank.trim() != "[adaptive_worker]" {
        return Ok(None);
    }

    let mut header = String::from("[adaptive_worker]\n");
    let mut found_delimiter = false;
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            found_delimiter = true;
            break;
        }
        header.push_str(line);
        header.push('\n');
    }
    if !found_delimiter {
        return Err(
            "[adaptive_worker] must be followed by a blank line before the Worker assignment"
                .to_string(),
        );
    }

    let body = lines.collect::<Vec<_>>().join("\n");
    if body.trim().is_empty() {
        return Err("Worker assignment body must not be empty".to_string());
    }

    let parsed: AdaptiveWorkerAssignmentDocument = toml::from_str(&header)
        .map_err(|err| format!("invalid [adaptive_worker] header: {err}"))?;
    let role = match parsed.adaptive_worker.role.as_str() {
        "implementation" => AdaptiveWorkerRole::Implementation,
        "validation" => AdaptiveWorkerRole::Validation,
        "repair" => AdaptiveWorkerRole::Repair,
        role => {
            return Err(format!(
                "invalid adaptive Worker role `{role}`; expected implementation, validation, or repair"
            ));
        }
    };
    let authorized_scope = parsed.adaptive_worker.authorized_scope.trim();
    if authorized_scope.is_empty() {
        return Err("adaptive Worker authorized_scope must not be empty".to_string());
    }

    Ok(Some(AdaptiveWorkerContext {
        role,
        authorized_scope: Some(authorized_scope.to_string()),
    }))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AdaptiveWorkflowTerminal {
    ReadyForValidation,
    RepairRequired,
    ReadyForOwnerQa,
    Blocked,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_worker_role_parser_accepts_only_role_values() {
        assert_eq!(
            parse_new_worker_role("implementation"),
            Some(AdaptiveWorkerRole::Implementation)
        );
        assert_eq!(
            parse_new_worker_role("validation"),
            Some(AdaptiveWorkerRole::Validation)
        );
        assert_eq!(
            parse_new_worker_role("repair"),
            Some(AdaptiveWorkerRole::Repair)
        );
        assert_eq!(parse_new_worker_role("nonsense"), None);
    }

    #[test]
    fn first_assignment_header_binds_each_authorized_worker_role() {
        for (role, expected) in [
            ("implementation", AdaptiveWorkerRole::Implementation),
            ("validation", AdaptiveWorkerRole::Validation),
            ("repair", AdaptiveWorkerRole::Repair),
        ] {
            let input = format!(
                "[adaptive_worker]\nrole = \"{role}\"\nauthorized_scope = \"Z-A0.45B-R2 exact bounded assignment\"\n\nPerform the assigned work."
            );
            assert_eq!(
                parse_adaptive_worker_assignment(&input),
                Ok(Some(AdaptiveWorkerContext {
                    role: expected,
                    authorized_scope: Some("Z-A0.45B-R2 exact bounded assignment".to_string()),
                }))
            );
        }
    }

    #[test]
    fn ordinary_prompt_has_no_worker_authority() {
        assert_eq!(
            parse_adaptive_worker_assignment("Review Z-A0.45B-R2 independently."),
            Ok(None)
        );
    }

    #[test]
    fn malformed_authority_header_fails_closed() {
        for input in [
            "[adaptive_worker]\nrole = \"reviewer\"\nauthorized_scope = \"R2\"\n\nReview it.",
            "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"   \"\n\nReview it.",
            "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\nextra = true\n\nReview it.",
            "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"",
            "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\n   ",
        ] {
            assert!(parse_adaptive_worker_assignment(input).is_err(), "{input}");
        }
    }
}
