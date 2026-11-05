//! Neutral Worker assignment and workflow handoff facts for adaptive state.

use codex_config::config_toml::AdaptiveWorkerConfigToml;
use codex_config::config_toml::AdaptiveWorkerRoleToml;
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

const RAW_BRACKETED_PASTE_START: &str = "\u{1b}[200~";
const RAW_BRACKETED_PASTE_END: &str = "\u{1b}[201~";
const TEXT_BRACKETED_PASTE_START: &str = "[200~";
const TEXT_BRACKETED_PASTE_END: &str = "[201~";

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

/// A source pin is external first-assignment authority, never a model continuation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum AdaptiveTaskKind {
    #[default]
    SourceChange,
    OperationsPreview,
    Benchmark,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveWorkerPreflight {
    pub(crate) task_kind: AdaptiveTaskKind,
    pub(crate) source_root: String,
    pub(crate) source_sha: String,
}

pub(crate) fn validated_assignment_preflight(
    task_kind: Option<&str>,
    source_root: Option<&str>,
    source_sha: Option<&str>,
) -> Result<Option<AdaptiveWorkerPreflight>, String> {
    let task_kind = match task_kind {
        None | Some("source_change") => AdaptiveTaskKind::SourceChange,
        Some("operations_preview") => AdaptiveTaskKind::OperationsPreview,
        Some("benchmark") => AdaptiveTaskKind::Benchmark,
        Some(other) => {
            return Err(format!("invalid Worker task_kind '{other}'; expected source_change, operations_preview or benchmark"));
        }
    };
    match (source_root, source_sha) {
        (None, None) if task_kind == AdaptiveTaskKind::SourceChange => Ok(None),
        (Some(root), Some(sha)) => {
            let root = root.trim();
            let sha = sha.trim();
            if !Path::new(root).is_absolute() {
                return Err("Worker source_root must be an absolute authorized worktree path".to_string());
            }
            if sha.len() != 40 || !sha.bytes().all(|ch| ch.is_ascii_hexdigit()) {
                return Err("Worker source_sha must be an exact 40-character Git SHA".to_string());
            }
            Ok(Some(AdaptiveWorkerPreflight {
                task_kind,
                source_root: root.to_string(),
                source_sha: sha.to_ascii_lowercase(),
            }))
        }
        _ => Err("Worker target preflight requires source_root and source_sha together; operations/benchmark tasks must also bind a source".to_string()),
    }
}

/// This local read-only pre-admission check never chooses another worktree.
/// Child-shell Set-Location does not change the native CodexDD runtime cwd.
pub(crate) fn preflight_mechanical_target(
    binding: Option<&AdaptiveWorkerPreflight>,
    runtime_cwd: &Path,
) -> Result<(), String> {
    let Some(binding) = binding else {
        // Legacy unpinned workers remain unchanged pending 3B.2 migration.
        return Ok(());
    };
    if binding.task_kind != AdaptiveTaskKind::SourceChange {
        return Err(
            "SETUP_BLOCKED: explicitly bound operations/benchmark task cannot enter protected source MechanicalValidation. Use an approved task-specific handoff, not a source work_packet."
                .to_string(),
        );
    }
    let observed_root = runtime_cwd.ancestors().find(|candidate| {
        let marker = candidate.join(".git");
        marker.is_file() || marker.is_dir()
    }).ok_or_else(|| format!(
        "SETUP_BLOCKED: native cwd {} has no Git root; expected authorized {} at SHA {}. No validation ran.",
        runtime_cwd.display(), binding.source_root, binding.source_sha
    ))?;
    let expected_root = Path::new(&binding.source_root).canonicalize().map_err(|err| {
        format!("SETUP_BLOCKED: cannot resolve authorized worktree {} ({err}); preserve the candidate.", binding.source_root)
    })?;
    let actual_root = observed_root.canonicalize().map_err(|err| {
        format!("SETUP_BLOCKED: cannot resolve native selected worktree {} ({err}); no validation ran.", observed_root.display())
    })?;
    if actual_root != expected_root {
        return Err(format!(
            "SETUP_BLOCKED: native Git root {} differs from authorized {} at SHA {}. Keep the owner's milestone-root launch convention; bind the approved worktree before validation. No validation ran.",
            actual_root.display(), expected_root.display(), binding.source_sha
        ));
    }
    let git = |args: &[&str]| -> Result<String, String> {
        let output = Command::new("git")
            .arg("-C").arg(&actual_root).args(args).output()
            .map_err(|err| format!("SETUP_BLOCKED: cannot inspect selected worktree with Git ({err}); no validation ran."))?;
        if !output.status.success() {
            return Err(format!(
                "SETUP_BLOCKED: Git read-only preflight failed in {} (status {}). No validation ran.",
                actual_root.display(), output.status
            ));
        }
        String::from_utf8(output.stdout)
            .map_err(|_| "SETUP_BLOCKED: Git returned non-UTF8 preflight data; no validation ran.".to_string())
    };
    let git_root = git(&["rev-parse", "--show-toplevel"])?;
    let verified_git_root = Path::new(git_root.trim()).canonicalize()
        .map_err(|err| format!("SETUP_BLOCKED: Git-selected root cannot be resolved ({err}); no validation ran."))?;
    if verified_git_root != actual_root {
        return Err(format!(
            "SETUP_BLOCKED: Git reports root {} but the native root is {}; no validation ran.",
            verified_git_root.display(), actual_root.display()
        ));
    }
    let current_sha = git(&["rev-parse", "HEAD"])?;
    if !current_sha.trim().eq_ignore_ascii_case(&binding.source_sha) {
        return Err(format!(
            "SETUP_BLOCKED: selected worktree {} has SHA {} but authorized candidate is {}; no validation ran.",
            actual_root.display(), current_sha.trim(), binding.source_sha
        ));
    }
    if !git(&["status", "--porcelain=v1", "--untracked-files=all"])?.trim().is_empty() {
        return Err(format!(
            "SETUP_BLOCKED: authorized source {} at {} has staged/unstaged/untracked changes. Preserve all edits; no exact-SHA validation receipt can be claimed.",
            actual_root.display(), binding.source_sha
        ));
    }
    Ok(())
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
    source_root: Option<String>,
    source_sha: Option<String>,
    task_kind: Option<String>,
}

fn normalize_bracketed_paste_framing(text: &str) -> (String, bool) {
    let mut normalized = text.to_string();
    let mut removed_leading_marker = false;

    if let Some((offset, _)) = normalized
        .char_indices()
        .find(|(_, ch)| !ch.is_whitespace())
    {
        for marker in [RAW_BRACKETED_PASTE_START, TEXT_BRACKETED_PASTE_START] {
            if normalized[offset..].starts_with(marker) {
                normalized.replace_range(offset..offset + marker.len(), "");
                removed_leading_marker = true;
                break;
            }
        }
    }

    let trailing_non_whitespace_len = normalized.trim_end_matches(char::is_whitespace).len();
    for marker in [RAW_BRACKETED_PASTE_END, TEXT_BRACKETED_PASTE_END] {
        if normalized[..trailing_non_whitespace_len].ends_with(marker) {
            let marker_start = trailing_non_whitespace_len - marker.len();
            normalized.replace_range(marker_start..trailing_non_whitespace_len, "");
            break;
        }
    }

    (normalized, removed_leading_marker)
}

/// Parses the authority header on the first user-supplied Worker assignment.
///
/// Authority is recognized only when the first non-blank line is exactly the
/// `[adaptive_worker]` table after removing a single recognized bracketed-paste
/// framing marker from the message boundary. The table must end at a blank line
/// so ordinary assignment prose can never be interpreted as TOML authority. A
/// message without the header returns `Ok(None)`; a malformed authority header
/// fails closed with an error.
pub(crate) fn parse_adaptive_worker_assignment(
    text: &str,
) -> Result<Option<AdaptiveWorkerContext>, String> {
    parse_adaptive_worker_assignment_with_preflight(text)
        .map(|assignment| assignment.map(|(context, _)| context))
}

pub(crate) fn parse_adaptive_worker_assignment_with_preflight(
    text: &str,
) -> Result<Option<(AdaptiveWorkerContext, Option<AdaptiveWorkerPreflight>)>, String> {
    let (normalized, removed_leading_paste_marker) = normalize_bracketed_paste_framing(text);
    let mut lines = normalized.lines();
    let first_non_blank = lines.by_ref().find(|line| !line.trim().is_empty());
    let Some(first_non_blank) = first_non_blank else {
        return Ok(None);
    };
    if first_non_blank.trim() != "[adaptive_worker]" {
        if removed_leading_paste_marker {
            return Err(
                "Worker binding rejected: bracketed-paste marker detected before [adaptive_worker], but the normalized first nonblank line was not [adaptive_worker]. Retry with a clean first assignment."
                    .to_string(),
            );
        }
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

    let preflight = validated_assignment_preflight(
        parsed.adaptive_worker.task_kind.as_deref(),
        parsed.adaptive_worker.source_root.as_deref(),
        parsed.adaptive_worker.source_sha.as_deref(),
    )?;
    Ok(Some((AdaptiveWorkerContext {
        role,
        authorized_scope: Some(authorized_scope.to_string()),
    }, preflight)))
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

    fn validation_assignment() -> String {
        "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"Z-A0.45B-R2 exact bounded assignment\"\n\nPerform the assigned work."
            .to_string()
    }

    fn expected_validation_context() -> AdaptiveWorkerContext {
        AdaptiveWorkerContext {
            role: AdaptiveWorkerRole::Validation,
            authorized_scope: Some("Z-A0.45B-R2 exact bounded assignment".to_string()),
        }
    }

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
    fn bracketed_paste_framing_binds_first_assignment() {
        let assignment = validation_assignment();
        let cases = [
            format!("{RAW_BRACKETED_PASTE_START}{assignment}"),
            format!("{assignment}{RAW_BRACKETED_PASTE_END}"),
            format!("{RAW_BRACKETED_PASTE_START}{assignment}{RAW_BRACKETED_PASTE_END}"),
            format!("{TEXT_BRACKETED_PASTE_START}{assignment}"),
            format!("{assignment}{TEXT_BRACKETED_PASTE_END}"),
            format!("{TEXT_BRACKETED_PASTE_START}{assignment}{TEXT_BRACKETED_PASTE_END}"),
        ];

        for input in cases {
            assert_eq!(
                parse_adaptive_worker_assignment(&input),
                Ok(Some(expected_validation_context())),
                "{input:?}"
            );
        }
    }

    #[test]
    fn bracketed_paste_start_after_leading_blank_lines_binds() {
        let input = format!(
            "\n  \n{TEXT_BRACKETED_PASTE_START}{}",
            validation_assignment()
        );
        assert_eq!(
            parse_adaptive_worker_assignment(&input),
            Ok(Some(expected_validation_context()))
        );
    }

    #[test]
    fn bracketed_paste_marker_without_exact_header_reports_binding_error() {
        let err = parse_adaptive_worker_assignment(
            "[200~not-the-header\n[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\nReview it.",
        )
        .expect_err("recognized paste framing without the exact authority header should fail closed");

        assert!(err.contains("bracketed-paste marker detected before [adaptive_worker]"));
    }

    #[test]
    fn unrelated_malformed_prefix_still_has_no_worker_authority() {
        for input in [
            "[199~[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\nReview it.",
            "[200x~[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\nReview it.",
            "prefix[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\nReview it.",
        ] {
            assert_eq!(parse_adaptive_worker_assignment(input), Ok(None), "{input}");
        }
    }

    #[test]
    fn trailing_paste_marker_cannot_substitute_for_assignment_body() {
        for marker in [RAW_BRACKETED_PASTE_END, TEXT_BRACKETED_PASTE_END] {
            let input = format!(
                "[adaptive_worker]\nrole = \"validation\"\nauthorized_scope = \"R2\"\n\n{marker}"
            );
            assert_eq!(
                parse_adaptive_worker_assignment(&input),
                Err("Worker assignment body must not be empty".to_string())
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

    #[test]
    fn pinned_authority_classifies_benchmark_and_rejects_partial_binding() {
        let root = if cfg!(windows) { r"D:\bw58\.worktrees\candidate" } else { "/tmp/bw58/.worktrees/candidate" };
        let sha = "a".repeat(40);
        let input = format!(
            "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"benchmark case\"\nsource_root = '{root}'\nsource_sha = \"{sha}\"\ntask_kind = \"benchmark\"\n\nMeasure safely."
        );
        let (context, binding) = parse_adaptive_worker_assignment_with_preflight(&input)
            .expect("parsed").expect("bound");
        assert_eq!(context.role, AdaptiveWorkerRole::Implementation);
        let binding = binding.expect("pin");
        assert_eq!(binding.source_root, root);
        assert_eq!(binding.source_sha, sha);
        assert_eq!(binding.task_kind, AdaptiveTaskKind::Benchmark);
        assert!(preflight_mechanical_target(Some(&binding), Path::new(root))
            .expect_err("not source work").contains("SETUP_BLOCKED"));
        for (kind, root, sha) in [
            (Some("benchmark"), None, None),
            (None, Some(root), None),
            (None, None, Some("a".repeat(40))),
            (None, Some("relative"), Some("a".repeat(40))),
            (None, Some(root), Some("invalid".to_string())),
            (Some("elevated"), Some(root), Some("a".repeat(40))),
        ] {
            assert!(validated_assignment_preflight(kind, root, sha.as_deref()).is_err());
        }
    }

    #[test]
    fn pinned_worktree_identity_rejects_wrong_root_sha_and_dirty_edits() {
        use std::fs;
        let temp = tempfile::tempdir().expect("tempdir");
        let milestone = temp.path().join("milestone");
        fs::create_dir(&milestone).expect("milestone");
        let git = |root: &Path, args: &[&str]| -> String {
            let output = Command::new("git").arg("-C").arg(root).args(args)
                .output().expect("git installed");
            assert!(output.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
            String::from_utf8(output.stdout).expect("utf8")
        };
        git(&milestone, &["init"]);
        git(&milestone, &["-c", "user.name=CodexDD Test", "-c", "user.email=codexdd@example.test",
            "commit", "--allow-empty", "-m", "baseline"]);
        let nested = milestone.join(".worktrees").join("candidate");
        git(&milestone, &["worktree", "add", "--detach", nested.to_str().expect("utf8"), "HEAD"]);
        let binding = AdaptiveWorkerPreflight {
            task_kind: AdaptiveTaskKind::SourceChange,
            source_root: nested.display().to_string(),
            source_sha: git(&nested, &["rev-parse", "HEAD"]).trim().to_string(),
        };
        assert!(preflight_mechanical_target(Some(&binding), &milestone)
            .expect_err("wrong selected root").contains("differs from authorized"));
        assert_eq!(preflight_mechanical_target(Some(&binding), &nested), Ok(()));
        let mut wrong_sha = binding.clone();
        wrong_sha.source_sha = "f".repeat(40);
        assert!(preflight_mechanical_target(Some(&wrong_sha), &nested)
            .expect_err("wrong SHA").contains("authorized candidate"));
        fs::write(nested.join("candidate.txt"), "preserve me").expect("write");
        git(&nested, &["add", "candidate.txt"]);
        assert!(preflight_mechanical_target(Some(&binding), &nested)
            .expect_err("staged candidate").contains("Preserve all edits"));
        assert_eq!(fs::read_to_string(nested.join("candidate.txt")).expect("read"), "preserve me");
    }
}
