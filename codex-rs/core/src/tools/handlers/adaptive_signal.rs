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
    Complexity,
    Capability,
    ReadyForValidation,
    RepairRequired,
    ReadyForOwnerQa,
    ReadyForRepositoryHandoff,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ComplexityReport {
    estimated_files: u16,
    cross_module: bool,
    public_api_or_data_model: bool,
    persistent_state_or_serialization: bool,
    concurrency_or_async: bool,
    build_release_or_toolchain: bool,
    uncertain_root_cause: bool,
    broad_test_surface: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdaptiveSignalArgs {
    kind: SignalKind,
    complexity: Option<ComplexityReport>,
    #[serde(default)]
    evidence_refs: Vec<String>,
    diagnostic_note: Option<String>,
}

pub struct AdaptiveSignalHandler;

fn classify_complexity(report: &ComplexityReport) -> &'static str {
    let mut score = 0u8;
    score = score.saturating_add(match report.estimated_files {
        0..=2 => 0,
        3..=5 => 1,
        6..=10 => 2,
        _ => 3,
    });
    score = score.saturating_add(u8::from(report.cross_module));
    score = score.saturating_add(u8::from(report.public_api_or_data_model) * 2);
    score = score.saturating_add(u8::from(report.persistent_state_or_serialization) * 2);
    score = score.saturating_add(u8::from(report.concurrency_or_async) * 2);
    score = score.saturating_add(u8::from(report.build_release_or_toolchain) * 2);
    score = score.saturating_add(u8::from(report.uncertain_root_cause));
    score = score.saturating_add(u8::from(report.broad_test_surface));

    match score {
        0..=1 => "routine",
        2..=4 => "standard",
        5..=7 => "complex",
        _ => "architectural",
    }
}

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
                    json!("complexity"),
                    json!("capability"),
                    json!("ready_for_validation"),
                    json!("repair_required"),
                    json!("ready_for_owner_qa"),
                    json!("ready_for_repository_handoff"),
                ],
                Some(
                    "The trusted adaptive fact to report. `capability` requests additional compute and therefore requires a diagnostic_note explaining the concrete capability limitation at the current route and why stronger model capability is required. A green Validation/Reviewer result such as PASS - READY FOR REPOSITORY HANDOFF must use ready_for_repository_handoff (or ready_for_owner_qa); both map to the READY_FOR_OWNER_QA hard terminal."
                        .to_string(),
                ),
            ),
        );
        let mut complexity_properties = BTreeMap::new();
        complexity_properties.insert(
            "estimated_files".to_string(),
            JsonSchema::integer(Some(
                "Estimated number of files the implementation is expected to touch.".to_string(),
            )),
        );
        for (name, description) in [
            (
                "cross_module",
                "Work crosses module or subsystem boundaries.",
            ),
            (
                "public_api_or_data_model",
                "Work changes a public API, shared contract, schema, or data model.",
            ),
            (
                "persistent_state_or_serialization",
                "Work changes persisted state, storage, serialization, or migrations.",
            ),
            (
                "concurrency_or_async",
                "Work changes concurrency, synchronization, async behavior, or ordering.",
            ),
            (
                "build_release_or_toolchain",
                "Work changes build, release, packaging, CI, or toolchain behavior.",
            ),
            (
                "uncertain_root_cause",
                "The root cause remains uncertain after reconnaissance.",
            ),
            (
                "broad_test_surface",
                "The expected validation surface spans multiple behaviors or subsystems.",
            ),
        ] {
            complexity_properties.insert(
                name.to_string(),
                JsonSchema::boolean(Some(description.to_string())),
            );
        }
        properties.insert(
            "complexity".to_string(),
            JsonSchema::object(
                complexity_properties,
                Some(vec![
                    "estimated_files".to_string(),
                    "cross_module".to_string(),
                    "public_api_or_data_model".to_string(),
                    "persistent_state_or_serialization".to_string(),
                    "concurrency_or_async".to_string(),
                    "build_release_or_toolchain".to_string(),
                    "uncertain_root_cause".to_string(),
                    "broad_test_surface".to_string(),
                ]),
                Some(false.into()),
            ),
        );
        properties.insert(
            "evidence_refs".to_string(),
            JsonSchema::array(
                JsonSchema::string(None),
                Some(
                    "Native completed tool/result IDs supporting a workflow terminal. Green Validation/Reviewer completion requires successful native evidence refs; repair_required requires failing native evidence refs."
                        .to_string(),
                ),
            ),
        );
        properties.insert(
            "diagnostic_note".to_string(),
            JsonSchema::string(Some(
                "Required and nonblank for kind=capability: explain the concrete limitation encountered at the current model/effort and why stronger model capability is warranted. Optional non-authoritative context for workflow-terminal signals."
                    .to_string(),
            )),
        );
        ToolSpec::Function(ResponsesApiTool {
            name: TOOL_NAME.to_string(),
            description: "Report the structured adaptive outcome before ending a bound Worker turn. On the first Luna Low turn of a bound Implementation Worker, perform reconnaissance only, then report kind=complexity with the complete structured complexity object and end the turn without editing; the runtime will continue automatically at the bounded implementation floor. A capability request must include a nonblank diagnostic report explaining the concrete current-route limitation and why stronger model capability is required; reasonless capability requests are rejected. When the assigned role is complete, report its workflow terminal before the final answer: Implementation/Repair completion uses ready_for_validation; a Validation/Reviewer with green objective validation, including a project verdict such as PASS - READY FOR REPOSITORY HANDOFF, uses ready_for_repository_handoff or ready_for_owner_qa with successful native evidence refs; an evidence-backed validation blocker uses repair_required. Do not report a workflow terminal while authorized work remains. Final-answer prose is non-authoritative and is not parsed by the runtime. Runtime validation occurs at the current turn's terminal boundary."
                .to_string(),
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

            let complexity_class = match (&args.kind, args.complexity.as_ref()) {
                (SignalKind::Complexity, Some(report)) => {
                    if !args.evidence_refs.is_empty() || args.diagnostic_note.is_some() {
                        return Err(FunctionCallError::RespondToModel(
                            "kind=complexity accepts only the structured complexity object; evidence_refs and diagnostic_note are not authoritative for reconnaissance"
                                .to_string(),
                        ));
                    }
                    Some(classify_complexity(report))
                }
                (SignalKind::Complexity, None) => {
                    return Err(FunctionCallError::RespondToModel(
                        "kind=complexity requires the complete structured complexity object"
                            .to_string(),
                    ));
                }
                (_, Some(_)) => {
                    return Err(FunctionCallError::RespondToModel(
                        "the complexity object is only valid for kind=complexity".to_string(),
                    ));
                }
                (_, None) => None,
            };

            if matches!(&args.kind, SignalKind::Capability) {
                let Some(diagnostic_note) = args
                    .diagnostic_note
                    .as_deref()
                    .map(str::trim)
                    .filter(|note| !note.is_empty())
                else {
                    return Err(FunctionCallError::RespondToModel(
                        "kind=capability requires a nonblank diagnostic_note explaining the concrete limitation at the current model/effort and why stronger model capability is required"
                            .to_string(),
                    ));
                };
                args.diagnostic_note = Some(diagnostic_note.to_string());
            }
            if let Some(complexity_class) = complexity_class {
                args.diagnostic_note = Some(complexity_class.to_string());
            }
            let signal_kind = match args.kind {
                SignalKind::Complexity => AdaptiveRuntimeSignalKind::Complexity,
                SignalKind::Capability => AdaptiveRuntimeSignalKind::Capability,
                SignalKind::ReadyForValidation => AdaptiveRuntimeSignalKind::ReadyForValidation,
                SignalKind::RepairRequired => AdaptiveRuntimeSignalKind::RepairRequired,
                SignalKind::ReadyForOwnerQa | SignalKind::ReadyForRepositoryHandoff => {
                    AdaptiveRuntimeSignalKind::ReadyForOwnerQa
                }
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
