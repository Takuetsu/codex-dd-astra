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
fn repository_discovery_requires_markers_and_selected_profile() {
    let directory = tempdir().expect("tempdir");
    let root = directory.path();
    fs::create_dir_all(root.join("codex-rs")).expect("codex-rs");
    fs::create_dir_all(root.join("scripts")).expect("scripts");
    fs::create_dir_all(root.join("nested").join("work")).expect("nested");
    fs::write(root.join("justfile"), "").expect("justfile");
    fs::write(root.join("codex-rs").join("Cargo.toml"), "").expect("cargo");
    fs::write(root.join("scripts").join("codexdd-test-targeted.ps1"), "").expect("profile");

    assert_eq!(
        discover_repository_root(
            &root.join("nested").join("work"),
            ValidationProfile::Targeted,
        ),
        Some(root.to_path_buf())
    );
    assert_eq!(
        discover_repository_root(
            &root.join("nested").join("work"),
            ValidationProfile::WorkPacket,
        ),
        None
    );
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
    assert!(!command.contains("work_packet"));
}

#[test]
fn parser_returns_compact_pass_summary() {
    let log = concat!(
        "native output\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"profile_begin\",\"profile\":\"work-packet\",\"stage_count\":2}\n",
        "CODEXDD_VALIDATION_JSON {\"contract_version\":1,\"event\":\"stage_end\",\"profile\":\"work-packet\",\"stage\":\"format-check\",\"ordinal\":1,\"status\":\"pass\",\"native_exit_code\":0,\"duration_ms\":9}\n",
        "more output\n",
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
