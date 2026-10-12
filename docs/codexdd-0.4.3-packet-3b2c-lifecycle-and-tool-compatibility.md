# CodexDD 0.4.3 — packet 3B.2c: child lifecycle and adaptive tool compatibility

**Status:** SOURCE RECONCILED / STATIC CONSTRAINTS PASSED; Rust native compilation and semantics **NOT** validated. Isolated GitHub integration work only.

## Exact version scope

- Integration branch based on production `4828e3b4232781963594cfaaebdc49c5531de8b1` (CodexDD 0.4.2, tracked Codex 0.159.2).
- Target official Codex 0.162.0 commit `c1382380de69521303b416720a52f42d51af6248`.
- The original [read-only discovery](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37960772252) remains **blocked_transplant_conflict**. These are source-merge checkpoints and not a promotable candidate.

## Nine files merged from full upstream source plus explicit CodexDD changes

| Source | Commit | Retained CodexDD invariant |
| --- | --- | --- |
| `codex-rs/core/src/agent/control/spawn.rs` | `14ce91a8e19a` | Child/agent history selection preserves durable `WorkflowState` rather than treating it as non-transferable transient data |
| `codex-rs/external-agent-migration/src/sessions/append.rs` | `679da8662c6a` | Adaptive workflow records are not mistaken for user/model response messages during external session migration |
| `codex-rs/thread-store/src/types.rs` | `9e7113c92756` | Prepared referenced forks bind an exact restored adaptive workflow snapshot alongside the inherited model-context prefix |
| `codex-rs/core/src/codex_thread.rs` | `ebf7612c1375` | `persist_workflow_state` appends and flushes adaptive workflow mutations on the same live thread |
| `codex-rs/core/src/client.rs` | `9c4914ff23a4` | Validation terminalization selects `tool_choice=required` unless the signal has already been submitted; otherwise keeps upstream's automatic selection |
| `codex-rs/core/src/session/turn.rs` | `1f2b0a16de49` | Private adaptive signal events do not become ordinary realtime messages |
| `codex-rs/core/src/tools/handlers/dynamic.rs` | `8415e8f9bea5` | Dynamic-tool outputs retain their original call evidence ID, while upstream's third-party tool classification is preserved |
| `codex-rs/core/src/tools/handlers/mod.rs` | `4131c9600f24` | Both CodexDD-specific trusted tool handlers remain in the upstream handler registry/module exports |
| `codex-rs/core/src/tools/spec_plan.rs` | `60d00c6aa0b0` | The adaptive reporting tool remains direct-model-only, and the native CodexDD validation tool is exposed only in its designated mechanical-validation turn |

The migration/agent/session source decisions are separate from the critical `registry.rs` dispatch guard in packet 3B.2d; **none of the tool-safety behavior is considered verified until that guard and its tests pass**.

## Source-integrity method

Each changed file started from the complete official 0.162.0 source rather than a 0.159.2 fork. Every fork-specific field, match arm, helper, import or validation tool registration was applied under unique source anchors. Reversing exactly those intentional additions reconstructed the entire official upstream file byte-for-byte before commit.

The fork-specific `CODEXDD_ADAPTIVE_VALIDATION_TERMINALIZATION_TURN_TRIGGER` symbol was verified in `codex-rs/protocol/src/lib.rs` on the actual isolated branch. The updated upstream request builder no longer carries the old explicit `instructions` field; the merge deliberately retained its new shape while preserving mandatory terminalization tool-choice semantics. Upstream's stable environment tools, new tool exposure pathways and dynamic-tool third-party labeling are retained.

**Not performed:** rustfmt, Cargo compilation, runtime execution, native PowerShell validation, protocol/schema regeneration, semantic acceptance.

## Required validation

1. Verify `client_tests.rs` existing `validation_terminalization_requests_required_tool_choice` on 0.162.0 API.
2. Run `registry_tests.rs` trusted signals, tool restrictions and source-edit denial tests on the native build after 3B.2d merge.
3. Run child fork/resume/persistence tests with the new upstream snapshot revision contract, including replay of approved vs unsupported workflow state.
4. Audit all new upstream extension/tool namespaces so they cannot bypass CodexDD mechanical or reconnaissance guards.
5. Review external-agent migration against the new protocol and generated schemas (3B.3/3B.4).
6. Final Windows-native validation and owner acceptance remain separate release gates.

No production, canonical candidate, product-version, tracked-upstream text file or installed binary was changed.
