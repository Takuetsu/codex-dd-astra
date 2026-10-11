# CodexDD 0.4.3 — packet 3F supplemental security-sensitive overlap review

**Status: SOURCE-SPECIFIC TECHNICAL REVIEW; NO OWNER SEMANTIC ACCEPTANCE OR NATIVE RELEASE.** This reviews the twelve original-classifier category-empty paths in the independently checked 136-overlap ledger. They are not new textual conflicts. Original `blocked_transplant_conflict`, 22 textual conflicts and proposed dispositions remain unchanged.

**Post-audit source note:** [`343be968e5c10b5703f95517eb6ed4be955f4f91`](https://github.com/Takuetsu/codex-dd-astra/commit/343be968e5c10b5703f95517eb6ed4be955f4f91) adds fail-closed rejection of `WorkflowState` in external *source* append items, while retaining native destination history handling. [Review and pending test gate](codexdd-0.4.3-packet-3f-import-workflow-authority.md). The original 12-path hash table below remains a historical `32ab09a141d626df5851965a557e0dce877ac2ec` snapshot; it is **not** silently revised or source-accepted.

## Immutable identity and scope

- Frozen product source: [`32ab09a141d626df5851965a557e0dce877ac2ec`](https://github.com/Takuetsu/codex-dd-astra/commit/32ab09a141d626df5851965a557e0dce877ac2ec), tree `a0bce7166cccb52e9b4fc8d658e0c08616752e62`.
- Exact current blob SHA references are from [136/136 postformat Git-object verification](codexdd-0.4.3-packet-3f-postformat-136-blob-audit.json). Earlier [628 Linux runtime tests](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292769), [core and CLI](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292765), [native hosted Windows focused F2/unit tests](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38158359012), and [13 generated output drift](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38157430174) are bounded **earlier-source** evidence, not per-path semantic acceptance.
- Source-specific findings below are review targets, **not assertions that a security defect is proven**.

## Twelve source findings and targeted missing evidence

| Path and frozen blob | Inspected source predicate | Remaining proof before acceptance |
| --- | --- | --- |
| 1. [`codex-rs/app-server-daemon/src/backend/windows.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/app-server-daemon/src/backend/windows.rs#L92-L151); `1cc1889712a5daa85642739bc9d17869967f8ac5` | Single Windows token elevation probe and typed denial | Real elevated SSH and failed-probe diagnostic untested |
| 2. [`codex-rs/app-server-daemon/src/backend/windows_tests.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/app-server-daemon/src/backend/windows_tests.rs#L18-L28); `66cbaf83bc698641dd858de643a09bbfc8b21495` | Probe/guard parity test; detached job assertions separately | Hosted focused test does not prove real elevated SSH or detached daemon lifecycle |
| 3. [`codex-rs/app-server-daemon/src/lib.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/app-server-daemon/src/lib.rs#L232-L301); `bc3cf057bc8b39dfec1e67dfd4fb0a736f41a61f` | Explicit privileged start/restart/bootstrap/update guards | Verify actual elevated CLI denial and non-elevated reuse/stop |
| 4. [`codex-rs/cli/src/main.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/cli/src/main.rs#L1294-L1348); `173d95554ec555bd4ea5e04408a95d350f209015` | CLI delegates daemon commands to guarded library | Test actual elevated resume/fork and no implicit daemon start |
| 5. [`codex-rs/cli/tests/daemon_startup.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/cli/tests/daemon_startup.rs#L62-L121); `e55abb91643cc93e0e401e0db851ac8777bc7f9b` | Elevated startup/resume/fork and restrictive-job test cases | Elevated case returns OK without assertions when runner is not elevated |
| 6. [`codex-rs/core/tests/suite/agent_websocket.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/core/tests/suite/agent_websocket.rs#L733-L835); `16b42c6a4709adb420b49c79e529266c8cdcb764` | Nested WebSocket Code Mode exec output reaches final request | Tool-output provenance is not equivalent to authorization for adaptive Worker state |
| 7. [`codex-rs/core/tests/suite/scenarios.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/core/tests/suite/scenarios.rs#L272-L288); `6021db64243290630dc436c120d26cf428e9294f` | MCP schema override and other agent/tool scenario tests | Test malformed/untrusted MCP payload against Worker scope admission |
| 8. [`codex-rs/core/tests/suite/unified_exec.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/core/tests/suite/unified_exec.rs#L1954-L2036); `02ae9e4f864ad69514f87a8ce3eab35ac6ed0df7` | Executed-tool evidence ID survives downstream model requests | Validate denied/interrupted background evidence cannot create trusted WorkflowState |
| 9. [`codex-rs/external-agent-migration/src/sessions/append.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/external-agent-migration/src/sessions/append.rs#L27-L159); `4efe3d065c832f11846207a7ec1976efcebc7458` | Cold-thread append checks archived/active/divergent states and full transcript prefix; WorkflowState ignored for transcript identity | Targeted import test should reject untrusted WorkflowState-based Worker authority, including suffix appends |
| 10. [`codex-rs/rollout-trace/src/protocol_event.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/rollout-trace/src/protocol_event.rs#L95-L145); `c7d7a51775430045e358d89f7c4ef61f9d7addf2` | Serializes borrowed exec/MCP trace payload and filters UserShell events | Trace privacy/redaction for commands and MCP payloads needs targeted proof |
| 11. [`sdk/python/src/openai_codex/generated/notification_registry.py`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/sdk/python/src/openai_codex/generated/notification_registry.py#L248-L266); `b527068e804a5e7bbe697464c331b21424cdafa6` | Generated registry maps turn/adaptiveRuntimeSignal notification | Require exact source-bound 13-artifact generator and SDK contract drift test |
| 12. [`sdk/python/src/openai_codex/generated/v2_all.py`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/sdk/python/src/openai_codex/generated/v2_all.py#L6837-L6843); `79e2b3a2e3b7620102ab12252efc7fd0a3d2bf81` | Generated typed adaptive runtime signal and thread identifier | Schema round-trip and source-origin trust are distinct from type generation |

## Technical risk synthesis

**Windows/F2 (five paths):** Source guards use typed elevation denial and early embedded selection. The Windows CLI integration test explicitly returns successful without assertions if it runs without elevation, so green hosted tests do **not** certify real elevated SSH, non-elevated detached-daemon lifetime or F2 observation.

**Trusted tool and Worker boundary (three core test paths):** Existing WebSocket, scenario and unified-exec coverage supplies narrow execution evidence. Tool results or successful tests do not grant trusted authority to untrusted output. Verify actual origin and admission of adaptive runtime signals and Worker scopes.

**Migration/replay (one path):** `external-agent-migration/.../append.rs` ignores `WorkflowState` while computing model-visible transcript identity; this does not itself establish whether imported metadata can affect authority. Test the imported suffix containing a forged WorkflowState, including cold thread append and resume/fork, before source approval.

**Rollout trace/privacy (one path):** `ToolRuntimePayload` serializes the full borrowed MCP event and selected command metadata, while suppressing UserShell trace events. Inspect concrete privacy/redaction behavior; a typed serializer is not a privacy guarantee.

**Generated Python SDK (two paths):** Notification registry and `v2_all.py` encode the adaptive-runtime-signal types. Keep generated files untouched and reproduce the complete 13-output artifact set against the frozen source at the final gate.

## Required gate sequence

1. Keep the prior 136 overlap/22 conflict classifications immutable. Review and separately accept each proposed semantic disposition and all six sensitive domains.
2. For further source changes, run the smallest applicable native **Daniel-CL SSH** tests first; the prior Rustfmt check and PowerShell fixtures are green but do not replace elevated CLI and imported-state security tests. Do not alter active `E:\codexdd` installation; use isolated validation checkout and exclude unrelated untracked file `M`.
3. After explicit owner source acceptance, stage canonical v2 evidence under guarded production identity, obtain separate candidate-publication permission, complete the **real candidate-bound 21-stage native Windows release profile**, then final GitHub CI and independent merge/install authorization.

**Result:** 12/12 original unclassified risk paths technically inspected; 0/12 owner accepted. No production update, release receipt, candidate publication, CI acceptance or installation is claimed.
