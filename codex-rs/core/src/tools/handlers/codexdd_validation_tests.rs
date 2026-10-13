use super::*;
use pretty_assertions::assert_eq;
use std::fs;
use tempfile::tempdir;

#[test]
fn profiles_map_to_fixed_scripts_and_bounded_timeouts() {
    assert_eq!(
        [
            (
                ValidationProfile::Targeted.tool_name(),
                ValidationProfile::Targeted.script_name(),
                ValidationProfile::Targeted.timeout_ms(),
            ),
            (
                ValidationProfile::WorkPacket.tool_name(),
                ValidationProfile::WorkPacket.script_name(),
                ValidationProfile::WorkPacket.timeout_ms(),
            ),
            (
                ValidationProfile::Release.tool_name(),
                ValidationProfile::Release.script_name(),
                ValidationProfile::Release.timeout_ms(),
            ),
        ],
        [
            ("targeted", "codexdd-test-targeted.ps1", 900_000),
            ("work_packet", "codexdd-test-workpacket.ps1", 2_700_000),
            ("release", "codexdd-test-release.ps1", 7_200_000),
        ]
    );
}

#[test]
fn repository_discovery_supports_explicit_rust_and_node_profiles() {
    let directory = tempdir().expect("tempdir");
    let root = directory.path();
    fs::create_dir(root.join(".git")).expect("git");
    fs::create_dir_all(root.join("codex-rs")).expect("codex-rs");
    fs::create_dir_all(root.join("scripts")).expect("scripts");
    fs::create_dir_all(root.join("nested").join("work")).expect("nested");
    fs::write(root.join("justfile"), "").expect("justfile");
    fs::write(root.join("codex-rs").join("Cargo.toml"), "").expect("cargo");
    fs::write(root.join("scripts").join("codexdd-test-targeted.ps1"), "").expect("profile");

    let nested = root.join("nested").join("work");
    assert_eq!(
        discover_repository_root(&nested, ValidationProfile::Targeted),
        Ok(root.to_path_buf()),
    );
    let missing = discover_repository_root(&nested, ValidationProfile::WorkPacket)
        .expect_err("missing profile must fail closed");
    assert!(missing.contains("is not configured"));
    assert!(missing.contains("No validation ran or PASS receipt was issued"));

    // A legitimate Node/TypeScript worktree can opt into the exact same
    // repository-owned contract without inheriting CodexDD's Rust markers.
    let node = tempdir().expect("node");
    let node_root = node.path();
    fs::write(node_root.join(".git"), "gitdir: ../metadata").expect("worktree marker");
    fs::write(node_root.join("package.json"), r#"{"scripts":{"test":"vitest run"}}"#)
        .expect("node manifest");
    fs::create_dir_all(node_root.join("scripts")).expect("node scripts");
    fs::create_dir_all(node_root.join("src")).expect("node source");
    fs::write(node_root.join("scripts").join("codexdd-test-workpacket.ps1"), "")
        .expect("node profile");
    assert_eq!(
        discover_repository_root(&node_root.join("src"), ValidationProfile::WorkPacket),
        Ok(node_root.to_path_buf())
    );
}

#[test]
fn missing_node_profile_cannot_fall_back_to_parent_repository() {
    let parent = tempdir().expect("parent");
    let outer = parent.path();
    fs::create_dir(outer.join(".git")).expect("outer git");
    fs::create_dir(outer.join("scripts")).expect("outer scripts");
    fs::write(outer.join("scripts").join("codexdd-test-workpacket.ps1"), "")
        .expect("outer profile");

    let inner = outer.join("breakwater");
    fs::create_dir(&inner).expect("inner");
    fs::create_dir(inner.join(".git")).expect("inner git");
    fs::write(inner.join("package.json"), r#"{"name":"breakwater"}"#).expect("manifest");
    fs::create_dir(inner.join("src")).expect("inner src");
    let error = discover_repository_root(&inner.join("src"), ValidationProfile::WorkPacket)
        .expect_err("must not use parent repo's profile");
    assert!(error.contains(&inner.display().to_string()));
    assert!(!error.contains("expected ") || error.contains("breakwater"));
}

#[test]
fn validation_requires_a_git_worktree() {
    let directory = tempdir().expect("tempdir");
    let error = discover_repository_root(directory.path(), ValidationProfile::Targeted)
        .expect_err("no git root");
    assert!(error.contains("no .git root was found"));
}

#[cfg(unix)]
#[test]
fn validation_rejects_profile_symlink_escape() {
    use std::os::unix::fs::symlink;
    let directory = tempdir().expect("tempdir");
    let root = directory.path();
    fs::create_dir(root.join(".git")).expect("git");
    fs::create_dir(root.join("scripts")).expect("scripts");
    let outside = tempdir().expect("outside");
    fs::write(outside.path().join("runner.ps1"), "").expect("outside script");
    symlink(
        outside.path().join("runner.ps1"),
        root.join("scripts").join("codexdd-test-workpacket.ps1"),
    )
    .expect("symlink");
    let error = discover_repository_root(root, ValidationProfile::WorkPacket)
        .expect_err("symlink must not cross repository boundary");
    assert!(error.contains("cross-repository script execution is forbidden"));
}

#[test]
fn powershell_command_escapes_trusted_paths_and_contains_no_model_shell_input() {
    let command = validation_powershell_command(
        Path::new("C:\\work\\owner's repo"),
        Path::new("C:\\work\\owner's repo\\scripts\\codexdd-test-targeted.ps1"),
        Path::new("C:\\Temp\\validation.log"),
    );

    assert!(command.contains("'C:\\work\\owner''s repo'"));
    assert!(command.contains("codexdd-test-targeted.ps1"));
    assert!(command.contains("AppendAllText"));
    assert!(!command.contains("CODEXDD_VALIDATION_RUNNER"));
    assert!(!command.contains("work_packet"));
}

#[test]
fn parser_returns_compact_pass_summary() {
    let log = concat!(
        "native output\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"work-packet\",\"stage_count\":2}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_begin\",\"profile\":\"work-packet\",\"stage\":\"format-check\",\"ordinal\":1,\"stage_count\":2}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_end\",\"profile\":\"work-packet\",\"stage\":\"format-check\",\"ordinal\":1,\"status\":\"pass\",\"native_exit_code\":0,\"duration_ms\":9}\n",
        "more output\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_begin\",\"profile\":\"work-packet\",\"stage\":\"tests\",\"ordinal\":2,\"stage_count\":2}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_end\",\"profile\":\"work-packet\",\"stage\":\"tests\",\"ordinal\":2,\"status\":\"pass\",\"native_exit_code\":0,\"duration_ms\":20}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_end\",\"profile\":\"work-packet\",\"status\":\"pass\",\"exit_code\":0,\"duration_ms\":29,\"completed_stages\":2}\n"
    );

    assert_eq!(
        parse_validation_summary(
            ValidationProfile::WorkPacket,
            log,
            Path::new("C:\\logs\\validation.log"),
        ),
        Ok(ValidationSummary {
            contract_version: 1,
            profile: "work-packet".to_string(),
            status: "pass".to_string(),
            exit_code: 0,
            stages: vec![
                ValidationStageSummary {
                    stage: "format-check".to_string(),
                    status: "pass".to_string(),
                    native_exit_code: Some(0),
                    duration_ms: Some(9),
                },
                ValidationStageSummary {
                    stage: "tests".to_string(),
                    status: "pass".to_string(),
                    native_exit_code: Some(0),
                    duration_ms: Some(20),
                },
            ],
            failed_stage: None,
            error_type: None,
            message: None,
            log_path: "C:\\logs\\validation.log".to_string(),
        })
    );
}

#[test]
fn parser_preserves_failed_stage_without_full_native_output() {
    let log = concat!(
        "thousands of compiler lines would remain only in the log\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"targeted\",\"stage_count\":1}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_begin\",\"profile\":\"targeted\",\"stage\":\"core-adaptive-tests\",\"ordinal\":1,\"stage_count\":1}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_end\",\"profile\":\"targeted\",\"stage\":\"core-adaptive-tests\",\"ordinal\":1,\"status\":\"fail\",\"native_exit_code\":101,\"duration_ms\":77}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_end\",\"profile\":\"targeted\",\"status\":\"fail\",\"exit_code\":1,\"duration_ms\":78,\"completed_stages\":0,\"failed_stage\":\"core-adaptive-tests\"}\n"
    );
    let summary = parse_validation_summary(
        ValidationProfile::Targeted,
        log,
        Path::new("validation.log"),
    )
    .expect("summary");

    assert_eq!(summary.status, "fail");
    assert_eq!(summary.exit_code, 1);
    assert_eq!(summary.failed_stage.as_deref(), Some("core-adaptive-tests"));
    assert_eq!(summary.stages.len(), 1);
}

#[test]
fn operator_summary_adds_run_and_candidate_identity_without_full_log_output() {
    let summary = ValidationSummary {
        contract_version: 1,
        profile: "work-packet".to_string(),
        status: "pass".to_string(),
        exit_code: 0,
        stages: vec![ValidationStageSummary {
            stage: "core-adaptive-tests".to_string(),
            status: "pass".to_string(),
            native_exit_code: Some(0),
            duration_ms: Some(42),
        }],
        failed_stage: None,
        error_type: None,
        message: None,
        log_path: "C:\\codexdd\\validation\\run-42\\validation.log".to_string(),
    };
    let operator = ValidationOperatorSummary {
        summary: &summary,
        run_id: "run-42",
        branch: Some("dd/status-evidence"),
        head_sha: "0123456789abcdef",
    };

    let value = serde_json::to_value(operator).expect("operator summary");
    assert_eq!(value["status"], "pass");
    assert_eq!(value["profile"], "work-packet");
    assert_eq!(value["run_id"], "run-42");
    assert_eq!(value["branch"], "dd/status-evidence");
    assert_eq!(value["head_sha"], "0123456789abcdef");
    assert_eq!(
        value["log_path"],
        "C:\\codexdd\\validation\\run-42\\validation.log"
    );
}

#[test]
fn validation_failure_fingerprint_is_stage_scoped_and_errors_are_not_repairable() {
    let failed = ValidationSummary {
        contract_version: 1,
        profile: "work-packet".to_string(),
        status: "fail".to_string(),
        exit_code: 1,
        stages: Vec::new(),
        failed_stage: Some("core-adaptive-tests".to_string()),
        error_type: None,
        message: None,
        log_path: "validation.log".to_string(),
    };
    assert_eq!(
        validation_failure_fingerprint(ValidationProfile::WorkPacket, &failed),
        Some("v1:work_packet:core-adaptive-tests".to_string())
    );

    let infrastructure_error = ValidationSummary {
        status: "error".to_string(),
        exit_code: 2,
        error_type: Some("execution_error".to_string()),
        ..failed
    };
    assert_eq!(
        validation_failure_fingerprint(ValidationProfile::WorkPacket, &infrastructure_error,),
        None
    );
}

#[test]
fn parser_refuses_profile_end_only_pass_without_running_declared_stages() {
    let log = concat!(
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"work-packet\",\"stage_count\":2}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_end\",\"profile\":\"work-packet\",\"status\":\"pass\",\"exit_code\":0,\"completed_stages\":2}\n",
    );
    assert_eq!(
        parse_validation_summary(ValidationProfile::WorkPacket, log, Path::new("validation.log")),
        Err("profile PASS did not match all declared, successful stages".to_string())
    );
}

#[test]
fn parser_refuses_fake_zero_stage_and_mismatched_node_passes() {
    let prefix =
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"work-packet\",\"stage_count\":0}\n";
    let terminal =
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_end\",\"profile\":\"work-packet\",\"status\":\"pass\",\"exit_code\":0,\"completed_stages\":0}\n";
    let empty_pass = format!("{prefix}{terminal}");
    assert!(parse_validation_summary(
        ValidationProfile::WorkPacket,
        &empty_pass,
        Path::new("validation.log"),
    ).is_err());

    let real_node_begin = concat!(
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"work-packet\",\"stage_count\":1}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_begin\",\"profile\":\"work-packet\",\"stage\":\"node-focused\",\"ordinal\":1,\"stage_count\":1}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_end\",\"profile\":\"work-packet\",\"stage\":\"node-focused\",\"ordinal\":1,\"status\":\"fail\",\"native_exit_code\":1}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_end\",\"profile\":\"work-packet\",\"status\":\"pass\",\"exit_code\":0,\"completed_stages\":1}\n",
    );
    assert_eq!(
        parse_validation_summary(
            ValidationProfile::WorkPacket,
            real_node_begin,
            Path::new("validation.log"),
        ),
        Err("profile PASS did not match all declared, successful stages".to_string())
    );
}

#[test]
fn parser_refuses_unmatched_node_stage_and_duplicate_terminal() {
    let prefix = concat!(
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"targeted\",\"stage_count\":1}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_begin\",\"profile\":\"targeted\",\"stage\":\"node-focused\",\"ordinal\":1}\n",
    );
    let unmatched = format!(
        "{prefix}CODEXDD_VALIDATION_JSON {{\"contract_version\":1,\"event\":\"stage_end\",\"profile\":\"targeted\",\"stage\":\"other-test\",\"ordinal\":1,\"status\":\"pass\",\"native_exit_code\":0}}\n"
    );
    assert_eq!(
        parse_validation_summary(ValidationProfile::Targeted, &unmatched, Path::new("validation.log")),
        Err("stage_end does not match its stage_begin".to_string())
    );

    let accepted = concat!(
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"targeted\",\"stage_count\":1}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_begin\",\"profile\":\"targeted\",\"stage\":\"node-focused\",\"ordinal\":1}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_end\",\"profile\":\"targeted\",\"stage\":\"node-focused\",\"ordinal\":1,\"status\":\"pass\",\"native_exit_code\":0}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_end\",\"profile\":\"targeted\",\"status\":\"pass\",\"exit_code\":0,\"completed_stages\":1}\n",
    );
    assert!(parse_validation_summary(
        ValidationProfile::Targeted, accepted, Path::new("validation.log")
    ).is_ok());
    let duplicate = format!(
        "{accepted}CODEXDD_VALIDATION_JSON {{\"contract_version\":1,\"event\":\"profile_end\",\"profile\":\"targeted\",\"status\":\"pass\",\"exit_code\":0,\"completed_stages\":1}}\n"
    );
    assert_eq!(
        parse_validation_summary(
            ValidationProfile::Targeted,
            &duplicate,
            Path::new("validation.log"),
        ),
        Err("validation event emitted after profile_end".to_string())
    );
}

#[test]
fn parser_fails_closed_without_terminal_event() {
    let log = "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"targeted\",\"stage_count\":1}\n";

    assert_eq!(
        parse_validation_summary(
            ValidationProfile::Targeted,
            log,
            Path::new("validation.log"),
        ),
        Err("profile_end event was not emitted".to_string())
    );
}

#[test]
fn path_component_sanitization_is_windows_safe() {
    assert_eq!(sanitize_path_component("call:42/abc?x"), "call_42_abc_x");
    assert_eq!(sanitize_path_component(""), "run");
}
