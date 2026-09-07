use crate::function_tool::FunctionCallError;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use codex_protocol::protocol::AdaptiveRuntimeSignalEvent;
use codex_protocol::protocol::AdaptiveRuntimeSignalKind;
use codex_protocol::protocol::EventMsg;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;

const TOOL_NAME: &str = "report_adaptive_signal";

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum SignalKind {
    Capability,
    ReadyForValidation,
    RepairRequired,
    ReadyForOwnerQa,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdaptiveSignalArgs {
    kind: SignalKind,
    #[serde(default)]
    evidence_refs: Vec<String>,
    diagnostic_note: Option<String>,
}

pub struct AdaptiveSignalHandler;

impl ToolExecutor<ToolInvocation> for AdaptiveSignalHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        let mut properties = BTreeMap::new();
        properties.insert(
            "kind".to_string(),
            JsonSchema::string_enum(
                vec![
                    json!("capability"),
                    json!("ready_for_validation"),
                    json!("repair_required"),
                    json!("ready_for_owner_qa"),
                ],
                Some("The trusted signal fact to report.".to_string()),
            ),
        );
        properties.insert(
            "evidence_refs".to_string(),
            JsonSchema::array(
                JsonSchema::string(None),
                Some(
                    "Native completed tool/result IDs supporting a workflow terminal.".to_string(),
                ),
            ),
        );
        properties.insert(
            "diagnostic_note".to_string(),
            JsonSchema::string(Some(
                "Optional short non-authoritative diagnostic note.".to_string(),
            )),
        );
        ToolSpec::Function(ResponsesApiTool {
            name: TOOL_NAME.to_string(),
            description: "Report a structured adaptive capability or workflow-terminal signal. Runtime validation occurs only at the current turn's terminal boundary.".to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["kind".to_string()]),
                /*additional_properties*/ Some(false.into()),
            ),
            output_schema: Some(json!({
                "type": "object",
                "properties": { "status": { "type": "string", "enum": ["request_emitted"] } },
                "required": ["status"],
                "additionalProperties": false
            })),
        })
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(async move {
            let ToolPayload::Function { arguments } = invocation.payload else {
                return Err(FunctionCallError::RespondToModel(format!(
                    "{TOOL_NAME} handler received unsupported payload"
                )));
            };
            let mut args: AdaptiveSignalArgs = serde_json::from_str(&arguments).map_err(|err| {
                FunctionCallError::RespondToModel(format!(
                    "failed to parse function arguments: {err}"
                ))
            })?;
            args.evidence_refs.sort();
            args.evidence_refs.dedup();
            let signal_kind = match args.kind {
                SignalKind::Capability => AdaptiveRuntimeSignalKind::Capability,
                SignalKind::ReadyForValidation => AdaptiveRuntimeSignalKind::ReadyForValidation,
                SignalKind::RepairRequired => AdaptiveRuntimeSignalKind::RepairRequired,
                SignalKind::ReadyForOwnerQa => AdaptiveRuntimeSignalKind::ReadyForOwnerQa,
            };
            invocation
                .session
                .send_event(
                    invocation.turn.as_ref(),
                    EventMsg::AdaptiveRuntimeSignal(AdaptiveRuntimeSignalEvent {
                        signal_kind,
                        evidence_refs: args.evidence_refs,
                        diagnostic_note: args.diagnostic_note,
                    }),
                )
                .await;
            Ok(boxed_tool_output(FunctionToolOutput::from_text(
                "{\"status\":\"request_emitted\"}".to_string(),
                Some(true),
            )))
        })
    }
}

impl CoreToolRuntime for AdaptiveSignalHandler {
    fn is_builtin_control_tool(&self) -> bool {
        true
    }
}

#[cfg(test)]
#[path = "adaptive_signal_tests.rs"]
mod tests;
