# CodexDD 0.4.3 — packet 3B.2d: fail-closed tool dispatch governance

**Status:** CRITICAL SOURCE RECONCILIATION CHECKPOINT. **Source-level placement reviewed**; NO compiler, test, Windows-native smoke, or release-acceptance proof yet. The 0.4.3 candidate is still unpublished and original v1 discovery remains **blocked_transplant_conflict**.

## Change and recorded identity

- Isolated integration branch from production `4828e3b4232781963594cfaaebdc49c5531de8b1` to pinned official `rust-v0.162.0` commit `c1382380de69521303b416720a52f42d51af6248`.
- Source: `codex-rs/core/src/tools/registry.rs` — commit `e1a2dbb0a8ed7ad1f116bdebbbc3782849601a6b`.
- Imported **the complete official 0.162.0 source file**, retaining only CodexDD's three trigger imports, two shell-parser imports, the ~367-line fail-closed helper/predicate suite and a pre-dispatch rejection block. Removing exactly those additions reconstructs the official 0.162.0 file.
- All fork-specific trigger string constants were found in the staged `codex-rs/protocol/src/lib.rs`; `parse_command`, the bounded PowerShell parser and `shell_script_for_invocation` are present in the merged source dependencies.

## Security-critical retained behavior

1. **Complexity reconnaissance**: read-only by default. Reject unfamiliar tool namespaces, write-capable shell commands and unsafe PowerShell expressions; allow only explicitly enumerated inspection commands, planning/context utilities and trusted reporting. No edit before reconnaissance acceptance.
2. **Mechanical validation**: allow bounded native validation, trusted `report_adaptive_signal`, context/planning and safe inspections; block source patching or other edit-capable tool calls. If the job needs new source work, it must report it for the implementation phase.
3. **Validation terminalization**: only `report_adaptive_signal` may execute. Other tools or operations are denied.
4. **Where enforced**: the rejection chain runs **after the registry has verified the tool's registered kind but before `run_pre_tool_use_hooks`, any tool-start notification and the handler execution**. Failed requests are visible to the model and logged with a rejected dispatch trace. This preserves the fork's previous early-denial boundary while retaining upstream tool lifecycle/telemetry changes.
5. **No authorization widening via newly introduced tools**: the allowlist checks the default namespace and enumerated names. New upstream tool names or plugins do not become permitted merely by being registered. Exact native dynamic behavior still requires tests.

This gate is an **important security compatibility invariant**, not evidence that arbitrary command syntax is safely parsed in all situations. Retained static policies must be verified against the new 0.162.0 tool router, payloads, shell runner and expansion rules.

## Existing regression tests confirmed present (not executed)

- `codex-rs/core/src/tools/registry_tests.rs`: `adaptive_reconnaissance_shell_gate_allows_inspection_commands`, `adaptive_reconnaissance_shell_gate_rejects_write_capable_commands`, `adaptive_mechanical_validation_shell_gate_allows_bounded_validation_commands`, `adaptive_mechanical_validation_shell_gate_blocks_source_edit_capable_commands`, and dispatch-level tests for read-only, patch block, and terminalization.
- `codex-rs/core/src/client_tests.rs`: `validation_terminalization_requests_required_tool_choice`.
- `codex-rs/core/src/tools/handlers/adaptive_signal.rs` and `codexdd_validation.rs` remain present from the current production fork on the isolated WIP branch.
- Shell/powershell parsing helpers are present in the exact target source. These dependency checks and source-body comparisons are **static checks only**, never a substitute for running the tests.

## Required Gate 3 evidence and rejection scenarios

- Compile against the reconciled core `registry.rs`, `spec_plan.rs`, `dynamic.rs`, source hooks/telemetry, and 0.162.0 tool routing.
- Pass the existing reconnaissance and mechanical allow/deny unit/dispatch tests on Windows; add explicit negative checks for newly introduced 0.162.0 tool names, indirectly routed tools, alternative shell invocations, mixed read/write pipeline, PowerShell aliases, and write-capable command redirection where coverage is missing.
- Preserve legitimate read-only command paths; avoid broad false positives that block routine status or search operations.
- Assert `ValidationTerminalization` cannot execute non-signal tools and `ResponsesApiRequest.tool_choice` remains required only for pending signal submissions.
- Audit whether `run_pre_tool_use_hooks` input mutation could otherwise have bypassed gating: here the fail-closed rejection is evaluated on the **original** input before hook rewriting; any unsafe rewrite path must be prevented or re-evaluated in the later security test packet.
- Any test failure is an integration blocker, never an acceptable validation waiver.

**No runtime claims** until native compilation and security regressions are green, and no candidate publication until the distinct additive recovery-bridge tooling PR is validated and owner-merged.

## Ownership and next work

This completes the 3B.2d **initial source-level** merge of the last owned core dispatch path. `core/src/session/tests.rs` remains **constructor-compatibility-only**, requiring complete 0.162.0 semantic test reconciliation. Phase 3B.3 app-server source/protocol alignment and 3B.4 generated schema regeneration are also outstanding before a meaningful Daniel-CL release-shaped native gate.

All 136 overlap paths, original 22 conflicts and one-time Windows release decisions remain governed by the separate phase/gate records.
