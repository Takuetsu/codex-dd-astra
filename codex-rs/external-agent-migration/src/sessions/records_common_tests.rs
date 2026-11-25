use super::*;
use pretty_assertions::assert_eq;

#[test]
fn imported_timestamp_keeps_signed_epoch_and_offset() {
    assert_eq!(
        parse_timestamp("1969-12-31T21:59:59.999999999-02:00"),
        Some(-1)
    );
    assert_eq!(parse_timestamp("1970-01-01 05:30:00+05:30"), Some(0));
    assert_eq!(parse_timestamp("1970-01-01T00:00:00"), None);
    assert_eq!(parse_timestamp("1970-01-01q00:00:00Z"), None);
}

#[test]
fn converts_tool_use_blocks_to_bounded_external_agent_tags() {
    let block = serde_json::json!({
        "type": "tool_use",
        "name": "Bash",
        "input": {
            "description": "Check repo status",
            "command": "git status --short"
        }
    });

    assert_eq!(
        tool_call_note(&block),
        "[external_agent_tool_call: Bash]\n\
         description: Check repo status\n\
         command: git status --short\n\
         [/external_agent_tool_call]"
    );
}

#[test]
fn converts_tool_result_blocks_to_bounded_external_agent_tags() {
    let block = serde_json::json!({
        "type": "tool_result",
        "content": "codex-rs/external-agent-migration/src/sessions/records_common.rs"
    });

    assert_eq!(
        tool_result_note(&block),
        "[external_agent_tool_result]\n\
         codex-rs/external-agent-migration/src/sessions/records_common.rs\n\
         [/external_agent_tool_result]"
    );
}

#[test]
fn converts_error_tool_result_blocks_to_bounded_external_agent_tags() {
    let block = serde_json::json!({
        "type": "tool_result",
        "is_error": true,
        "content": "command failed"
    });

    assert_eq!(
        tool_result_note(&block),
        "[external_agent_tool_result: error]\n\
         command failed\n\
         [/external_agent_tool_result]"
    );
}
