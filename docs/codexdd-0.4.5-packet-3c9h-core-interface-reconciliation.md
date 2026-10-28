# CodexDD 0.4.5 — 3C.9h coordinated 0.163 core-interface repair

Date: 2026-10-19

**Status: SOURCE INTEGRATION ONLY — NO NATIVE RUST PASS, NO RELEASE AUTHORIZATION.**

## Failed native Windows evidence

The GitHub Actions read-only compile for commit `76734b65a6c5ee4cff92cc7542e51658653c7161` at run [39291001441](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39291001441) failed in `codex-core` with **36 compiler errors**. The job printed only the final 180 log lines, exposing 10 error headings. Its full diagnostic artifact was retained by the GitHub runner.

Visible failures concerned stale:

- `GuardianReviewSessionOutcome::StaleAuthorization` and `ReviewerTurn::cyber_access_program`;
- plugin-policy-aware `McpServerContributor::selected_plugins(context, plugins_config)`;
- MCP `PreparedMcpCall::augment_sandbox_metadata`, `McpConfig.plugins` and `McpConfig.codex_self_exe`;
- Code Mode collection's `tool_description_first` parameter;
- model-owned `functions_namespace_functions_description_prefixes` accessor.

## Source reconciliation (same-blob official 0.162.0 baseline)

The following staging targets were verified **byte-for-byte equal to the official `rust-v0.162.0` versions before integration**. They contain no fork-only CodexDD changes, so the review uses the exact official `rust-v0.163.0` blobs, not hand-edited copies.

- `ext/guardian-reviewer/src/{assessment.rs,assessment_tests.rs,lib.rs,outcome.rs,settings.rs}`: preserve fail-closed stale-authorization, Daybreak Blue ceiling on reviewer access and updated assessment parsing.
- `ext/extension-api/src/contributors.rs`: propagate the explicit plugin policy through the contribution trait, so callers cannot implicitly bypass per-step admission.
- `tools/src/code_mode.rs`: propagate tool-description ordering through nested tools, retaining the original sorted/deduplicated tool catalog behavior.
- `codex-mcp/src/{lib.rs,mcp/mod.rs,binding.rs,runtime.rs,rmcp_client.rs,connection_manager.rs,connection_manager_tests.rs}`: coordinate captured MCP call authority, explicit plugin policy, verified local sandbox launcher and reconnection behavior.
- Stage new official `codex-mcp/src/{sandbox_metadata.rs,sandbox_metadata_tests.rs,sandbox_executable.rs}`: provide opaque sandbox-state metadata and verified executable detection. Never search PATH for a launcher, infer a local host from a remote connection, or widen sandbox permissions.

Unlike prior one-file hotfixes, these interdependent interfaces must be merged atomically: a new `PreparedMcpCall` method without its `sandbox_metadata` implementation, or a new plugin selection parameter without the trait declaration, would not compile.

## Bounded additional compatibility change

`prompts/src/model_messages.rs` gains `functions_namespace_functions_description_prefixes()`, a read-only accessor for the already-staged `ToolMessages.functions_namespace_functions_description_prefixes: Option<BTreeMap<String,String>>` schema. The tool registry remains responsible for validating and applying prefix rules; this does not grant tool execution or modify policy.

## Build diagnostics improvement

`.github/workflows/codexdd-045-windows-cargo-check.yml` now extracts **every** Rust compiler error heading and its next 22 source lines into both the job log and a 7-day diagnostic text artifact. The workflow remains restricted to the `dd/codexdd-v0.4.5-upstream-0.163.0-integration` branch, with `contents: read` and no packaging, merging, installer or publishing authority.

## Still outstanding

This packet is not a success report for the 36-error build. Next: inspect the automatically triggered focused native `cargo check -p codex-tui --lib` on Windows and resolve any remaining errors as a group; then compile native tests and perform Windows security/approval/TUI validation. `Cargo.lock` is still unaccepted pending exact resolver and security audits. Other protected source conflicts, generated schemas, the final 21-stage Windows suite, release version advancement, CI acceptance, and automatic upstream publication remain separate gates. Current released product remains CodexDD `0.4.4` and the tracked released upstream remains `rust-v0.162.0`.
