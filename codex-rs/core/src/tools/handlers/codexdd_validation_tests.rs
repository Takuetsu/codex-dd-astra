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
    fs::write(
        node_root.join("package.json"),
        r#"{"scripts":{"test":"vitest run"}}"#,
    )
    .expect("node manifest");
    fs::create_dir_all(node_root.join("scripts")).expect("node scripts");
    fs::create_dir_all(node_root.join("src")).expect("node source");
    fs::write(
        node_root
            .join("scripts")
            .join("codexdd-test-workpacket.ps1"),
        "",
    )
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
    fs::write(
        outer.join("scripts").join("codexdd-test-workpacket.ps1"),
        "",
    )
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
        Path::new("C:\\Temp\\validation.exit"),
    );

    assert!(command.contains("'C:\\work\\owner''s repo'"));
    assert!(command.contains("codexdd-test-targeted.ps1"));
    assert!(command.contains("AppendAllText"));
    assert!(command.contains("WriteAllText"));
    assert!(command.contains("'C:\\Temp\\validation.exit'"));
    assert!(!command.contains("CODEXDD_VALIDATION_RUNNER"));
    assert!(!command.contains("work_packet"));
    // Rust deletes stale temporary files before launching the command.
    // A second Remove-Item -Force in PowerShell causes the Windows
    // dangerous-command classifier to request approval or reject execution.
    assert!(!command.contains("Remove-Item"));
    let argv = vec![
        "powershell.exe".to_string(),
        "-NoProfile".to_string(),
        "-Command".to_string(),
        command,
    ];
    assert_eq!(
        codex_shell_command::is_dangerous_command::dangerous_command_match_for_platform(
            &argv,
            codex_shell_command::is_dangerous_command::DangerousCommandPlatform::Windows,
        ),
        None,
        "native validation capture must not trigger force-delete command policy",
    );
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
        parse_validation_summary(
            ValidationProfile::WorkPacket,
            log,
            Path::new("validation.log")
        ),
        Err("profile PASS did not match all declared, successful stages".to_string())
    );
}

#[test]
fn parser_refuses_fake_zero_stage_and_mismatched_node_passes() {
    let prefix = "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"work-packet\",\"stage_count\":0}\n";
    let terminal = "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_end\",\"profile\":\"work-packet\",\"status\":\"pass\",\"exit_code\":0,\"completed_stages\":0}\n";
    let empty_pass = format!("{prefix}{terminal}");
    assert!(
        parse_validation_summary(
            ValidationProfile::WorkPacket,
            &empty_pass,
            Path::new("validation.log"),
        )
        .is_err()
    );

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
        parse_validation_summary(
            ValidationProfile::Targeted,
            &unmatched,
            Path::new("validation.log")
        ),
        Err("stage_end does not match its stage_begin".to_string())
    );

    let accepted = concat!(
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"targeted\",\"stage_count\":1}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_begin\",\"profile\":\"targeted\",\"stage\":\"node-focused\",\"ordinal\":1}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_end\",\"profile\":\"targeted\",\"stage\":\"node-focused\",\"ordinal\":1,\"status\":\"pass\",\"native_exit_code\":0}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_end\",\"profile\":\"targeted\",\"status\":\"pass\",\"exit_code\":0,\"completed_stages\":1}\n",
    );
    assert!(
        parse_validation_summary(
            ValidationProfile::Targeted,
            accepted,
            Path::new("validation.log")
        )
        .is_ok()
    );
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
fn native_exit_consistency_refuses_fake_pass_and_missing_process_exit() {
    let original = ValidationSummary {
        contract_version: 1,
        profile: "targeted".to_string(),
        status: "pass".to_string(),
        exit_code: 0,
        stages: vec![ValidationStageSummary {
            stage: "node-focused".to_string(),
            status: "pass".to_string(),
            native_exit_code: Some(0),
            duration_ms: Some(4),
        }],
        failed_stage: None,
        error_type: None,
        message: None,
        log_path: "validation.log".to_string(),
    };
    let mut valid = original.clone();
    enforce_native_exit_consistency(&mut valid, Some(0));
    assert_eq!(valid, original);

    let mut fabricated = original.clone();
    enforce_native_exit_consistency(&mut fabricated, Some(1));
    assert_eq!(fabricated.status, "error");
    assert_eq!(fabricated.exit_code, 2);
    assert_eq!(
        fabricated.error_type.as_deref(),
        Some("native_exit_mismatch")
    );
    assert_eq!(
        validation_failure_fingerprint(ValidationProfile::Targeted, &fabricated),
        None
    );

    let mut missing = original;
    enforce_native_exit_consistency(&mut missing, None);
    assert_eq!(missing.status, "error");
    assert_eq!(missing.exit_code, 2);
    assert_eq!(missing.error_type.as_deref(), Some("native_exit_mismatch"));
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

#[test]
fn native_authorized_target_checks_real_nested_git_worktree_and_candidate_sha() {
    use codex_config::config_toml::AdaptiveWorkerConfigToml;
    use codex_config::config_toml::AdaptiveWorkerRoleToml;
    use std::process::Command;

    let temp = tempdir().expect("fixture");
    let milestone = temp.path().join("milestone");
    fs::create_dir_all(&milestone).expect("create milestone");
    let git = |root: &Path, args: &[&str]| -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .expect("git must be installed for preflight regression");
        assert!(
            output.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).expect("Git response UTF8")
    };
    git(&milestone, &["init", "-q"]);
    git(
        &milestone,
        &[
            "-c",
            "user.name=CodexDD Fixture",
            "-c",
            "user.email=codexdd@example.test",
            "commit",
            "--allow-empty",
            "-m",
            "source anchor",
        ],
    );
    let nested = milestone.join(".worktrees").join("candidate");
    git(
        &milestone,
        &[
            "worktree",
            "add",
            "--detach",
            nested.to_str().expect("UTF8 path"),
            "HEAD",
        ],
    );
    let sha = git(&nested, &["rev-parse", "HEAD"]).trim().to_string();
    let binding = AdaptiveWorkerConfigToml {
        role: AdaptiveWorkerRoleToml::Implementation,
        authorized_scope: Some("approved nested candidate".to_string()),
        source_root: Some(nested.display().to_string()),
        source_sha: Some(sha.clone()),
        task_kind: Some("source_change".to_string()),
    };
    assert_eq!(
        validate_native_authorized_target(&nested, Some(&binding), &sha),
        Ok(())
    );
    assert!(
        validate_native_authorized_target(&milestone, Some(&binding), &sha)
            .expect_err("wrong selected root must not fall back")
            .contains("native LVO selected")
    );

    let mut wrong_sha = binding.clone();
    wrong_sha.source_sha = Some("f".repeat(40));
    assert!(
        validate_native_authorized_target(&nested, Some(&wrong_sha), &sha)
            .expect_err("wrong candidate")
            .contains("authorized source SHA")
    );

    let mut benchmark = binding.clone();
    benchmark.task_kind = Some("benchmark".to_string());
    assert!(
        validate_native_authorized_target(&nested, Some(&benchmark), &sha)
            .expect_err("benchmark must not enter mechanical source validation")
            .contains("operations/benchmark Worker")
    );

    let mut incomplete = binding.clone();
    incomplete.source_sha = None;
    assert!(
        validate_native_authorized_target(&nested, Some(&incomplete), &sha)
            .expect_err("partial pin must not validate")
            .contains("source_root and source_sha together")
    );

    fs::write(nested.join("candidate.txt"), "preserve edits").expect("fixture edits");
    git(&nested, &["add", "candidate.txt"]);
    assert!(
        validate_native_authorized_target(&nested, Some(&binding), &sha)
            .expect_err("staged candidate cannot receive exact SHA proof")
            .contains("Preserve edits")
    );
    assert_eq!(
        fs::read_to_string(nested.join("candidate.txt")).expect("edit intact"),
        "preserve edits"
    );
    git(&nested, &["reset", "--quiet", "--", "candidate.txt"]);
    fs::remove_file(nested.join("candidate.txt")).expect("fixture cleanup");
    assert_eq!(
        validate_native_authorized_target(&nested, Some(&binding), &sha),
        Ok(())
    );

    git(
        &nested,
        &[
            "-c",
            "user.name=CodexDD Fixture",
            "-c",
            "user.email=codexdd@example.test",
            "commit",
            "--allow-empty",
            "-m",
            "new candidate HEAD",
        ],
    );
    assert!(
        validate_native_authorized_target(&nested, Some(&binding), &sha)
            .expect_err("post-exec SHA drift cannot retain PASS")
            .contains("authorized source SHA")
    );
}

#[test]
fn native_authorized_target_keeps_legacy_unpinned_compatibility() {
    use codex_config::config_toml::AdaptiveWorkerConfigToml;
    use codex_config::config_toml::AdaptiveWorkerRoleToml;

    let temp = tempdir().expect("fixture");
    let old_binding = AdaptiveWorkerConfigToml {
        role: AdaptiveWorkerRoleToml::Implementation,
        authorized_scope: Some("legacy source work".to_string()),
        source_root: None,
        source_sha: None,
        task_kind: None,
    };
    let unpinned = temp.path();
    let sha = "a".repeat(40);
    assert_eq!(
        validate_native_authorized_target(unpinned, None, &sha),
        Ok(())
    );
    assert_eq!(
        validate_native_authorized_target(unpinned, Some(&old_binding), &sha),
        Ok(())
    );
}

#[test]
fn codexdd_046_3b2_native_role_gate_rejects_independent_workers() {
    use codex_config::config_toml::AdaptiveWorkerConfigToml;
    use codex_config::config_toml::AdaptiveWorkerRoleToml;

    let temp = tempdir().expect("role fixture");
    let sha = "a".repeat(40);
    let mut worker = AdaptiveWorkerConfigToml {
        role: AdaptiveWorkerRoleToml::Validation,
        authorized_scope: Some("independent owner QA".to_string()),
        source_root: None,
        source_sha: None,
        task_kind: None,
    };
    for role in [
        AdaptiveWorkerRoleToml::Validation,
        AdaptiveWorkerRoleToml::Repair,
    ] {
        worker.role = role;
        let blocker = validate_native_authorized_target(temp.path(), Some(&worker), &sha)
            .expect_err("independent role must not enter Implementation-only LVO");
        assert!(blocker.starts_with("SETUP_BLOCKED:"));
        assert!(blocker.contains("Validation/Repair Worker"));
        assert!(blocker.contains("No profile ran"));
    }

    worker.role = AdaptiveWorkerRoleToml::Unspecified;
    worker.source_root = Some(temp.path().display().to_string());
    worker.source_sha = Some(sha.clone());
    worker.task_kind = Some("source_change".to_string());
    let blocker = validate_native_authorized_target(temp.path(), Some(&worker), &sha)
        .expect_err("pinned source without Implementation role must fail closed");
    assert!(blocker.contains("explicitly bound Implementation Worker"));

    worker.role = AdaptiveWorkerRoleToml::Implementation;
    worker.source_sha = None;
    let blocker = validate_native_authorized_target(temp.path(), Some(&worker), &sha)
        .expect_err("partial pin must never be accepted");
    assert!(blocker.contains("source_root and source_sha together"));
}

#[test]
fn codexdd_046_3b2_native_preflight_without_git_returns_nonrepairable_setup_blocker() {
    let tmp = tempdir().expect("no-git fixture");
    let missing_git = find_active_repository_root(tmp.path())
        .expect_err("missing repo must not select an ancestor");
    assert!(missing_git.contains("no .git root was found"));

    let summary = native_setup_blocker_summary(
        ValidationProfile::WorkPacket,
        "repository_unavailable",
        format!("SETUP_BLOCKED: {missing_git}"),
        Path::new("isolated-validation.log"),
    );
    assert_eq!(summary.profile, "work-packet");
    assert_eq!(summary.status, "error");
    assert_eq!(summary.exit_code, 2);
    assert!(summary.stages.is_empty());
    assert_eq!(
        summary.error_type.as_deref(),
        Some("repository_unavailable")
    );
    assert!(
        summary
            .message
            .as_deref()
            .unwrap_or_default()
            .starts_with("SETUP_BLOCKED:")
    );
    assert_eq!(
        validation_failure_fingerprint(ValidationProfile::WorkPacket, &summary),
        None
    );

    for kind in [
        "source_identity_unavailable",
        "profile_unavailable",
        "source_identity_mismatch",
    ] {
        let setup = native_setup_blocker_summary(
            ValidationProfile::Targeted,
            kind,
            "SETUP_BLOCKED: source/profile not ready".to_string(),
            Path::new("isolated-validation.log"),
        );
        assert_eq!(setup.status, "error");
        assert_eq!(setup.failed_stage, None);
        assert_eq!(
            validation_failure_fingerprint(ValidationProfile::Targeted, &setup),
            None
        );
    }
}

#[test]
fn codexdd_046_3b2d_protected_mechanical_requires_core_bound_source() {
    use codex_config::config_toml::AdaptiveWorkerConfigToml;
    use codex_config::config_toml::AdaptiveWorkerRoleToml;

    let trigger = codex_protocol::CODEXDD_ADAPTIVE_MECHANICAL_VALIDATION_TURN_TRIGGER;
    let legacy = AdaptiveWorkerConfigToml {
        role: AdaptiveWorkerRoleToml::Implementation,
        authorized_scope: Some("legacy without native pin".to_string()),
        source_root: None,
        source_sha: None,
        task_kind: None,
    };
    for worker in [None, Some(&legacy)] {
        let blocker = native_mechanical_source_authority_blocker(Some(trigger), worker)
            .expect("missing Core binding must stop protected native validation");
        assert!(blocker.starts_with("SETUP_BLOCKED:"));
        assert!(blocker.contains("Core-bound"));
        assert!(blocker.contains("No profile ran"));
    }
    let mut pinned = legacy;
    pinned.source_root = Some("C:\\authorized\\worktree".to_string());
    pinned.source_sha = Some("a".repeat(40));
    assert_eq!(
        native_mechanical_source_authority_blocker(Some(trigger), Some(&pinned)),
        None
    );
    // A pinned Validation or Repair role cannot acquire Implementation authority.
    pinned.role = AdaptiveWorkerRoleToml::Validation;
    assert!(
        native_mechanical_source_authority_blocker(Some(trigger), Some(&pinned))
            .is_some()
    );
    pinned.role = AdaptiveWorkerRoleToml::Repair;
    assert!(
        native_mechanical_source_authority_blocker(Some(trigger), Some(&pinned))
            .is_some()
    );
}

#[test]
fn codexdd_046_3b2d_unbound_adaptive_receipt_cannot_authorize_repair() {
    let trigger = codex_protocol::CODEXDD_ADAPTIVE_MECHANICAL_VALIDATION_TURN_TRIGGER;
    let message = native_mechanical_source_authority_blocker(Some(trigger), None)
        .expect("unbound protected validation must block");
    let summary = native_setup_blocker_summary(
        ValidationProfile::WorkPacket,
        "source_authority_unavailable",
        message,
        Path::new("isolated-validation.log"),
    );
    assert_eq!(summary.status, "error");
    assert_eq!(summary.exit_code, 2);
    assert_eq!(summary.failed_stage, None);
    assert!(summary.stages.is_empty());
    assert_eq!(
        summary.error_type.as_deref(),
        Some("source_authority_unavailable")
    );
    assert_eq!(
        validation_failure_fingerprint(ValidationProfile::WorkPacket, &summary),
        None
    );
    assert_eq!(
        native_mechanical_source_authority_blocker(Some("user"), None),
        None
    );
    assert_eq!(native_mechanical_source_authority_blocker(None, None), None);
}
