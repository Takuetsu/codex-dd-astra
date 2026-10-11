# CodexDD 0.4.3 — 3F imported WorkflowState authority boundary

**Status: LOCAL DANIEL-CL TARGETED TEST PASSED (3/3); SOURCE INVENTORY UPDATED; NO OWNER SEMANTIC ACCEPTANCE OR CANDIDATE RELEASE RECEIPT.**

## Scope and exact ancestry

- Production remains `16fdc9826e800ebabab920eadd8e1d976d2f645f` (CodexDD 0.4.2). Official upstream target is `rust-v0.162.0`, peeled `c1382380de69521303b416720a52f42d51af6248`.
- Previous fully inventoried 136-overlap Rust snapshot: `32ab09a141d626df5851965a557e0dce877ac2ec`, full tree `a0bce7166cccb52e9b4fc8d658e0c08616752e62`.
- Latest modified product source: [`343be968e5c10b5703f95517eb6ed4be955f4f91`](https://github.com/Takuetsu/codex-dd-astra/commit/343be968e5c10b5703f95517eb6ed4be955f4f91), full tree `4f58649164ea5ffcfa782b8c9075d68eafe5db65`. The previous inventory remains immutable and **historical**; [updated 136-path Git-object audit](codexdd-0.4.3-packet-3f-posthardening-source-audit.md) now reconciles the current tree against its verified blobs. The new source differs from the previous Rust snapshot in **exactly two files**, verified by GitHub compare; no updater, CLI, daemon, model router, generated SDK, version or release marker edits.
- Production and installed Windows clients are unchanged. No release candidate or CI job is intentionally launched by this work packet.

## Reachable authority and original failure mode

1. [External session parser](https://github.com/Takuetsu/codex-dd-astra/blob/343be968e5c10b5703f95517eb6ed4be955f4f91/codex-rs/external-agent-migration/src/sessions/export.rs#L39-L85) consumes Claude/Cursor records, converts them into `ConversationMessage`, then reconstructs source `RolloutItem` values from user/assistant messages, turn markers and token usage. **It does not import raw native `WorkflowState` items from external JSONL.**
2. [The app-server importer](https://github.com/Takuetsu/codex-dd-astra/blob/343be968e5c10b5703f95517eb6ed4be955f4f91/codex-rs/app-server/src/external_agent_migration/session_importer.rs#L261-L324) calls the public `append_existing_session` API with `session.rollout_items` from that parser. No currently verified external-input exploit is claimed.
3. Prior reconciled [`append.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/32ab09a141d626df5851965a557e0dce877ac2ec/codex-rs/external-agent-migration/src/sessions/append.rs#L216-L279) ignored `WorkflowState` while extracting model-visible transcript items from **both** untrusted imported source and legitimate existing native history. It correctly ignored trusted destination metadata for transcript identity, but would allow a malformed caller-supplied source containing `WorkflowState` to reach the append suffix. The original source had no explicit source-control-item rejection.
4. [Restored workflow-state logic](https://github.com/Takuetsu/codex-dd-astra/blob/343be968e5c10b5703f95517eb6ed4be955f4f91/codex-rs/history/src/lib.rs#L207-L309) can interpret persisted `WorkflowStateOperation::Set` and `AdaptiveState`. Preventing imported source authority is therefore an appropriate fail-closed invariant independent of whether the current parser admits such records.

## Exact two-file change

| File | Current blob | Narrow change |
| --- | --- | --- |
| [`codex-rs/external-agent-migration/src/sessions/append.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/343be968e5c10b5703f95517eb6ed4be955f4f91/codex-rs/external-agent-migration/src/sessions/append.rs#L216-L279) | `47bc6fb417cfdc1e3415cf5f8f0aa1ea8b44be7b` | `source_model_items` returns `None` when source contains `RolloutItem::WorkflowState`; `history_model_items` still skips legitimate native destination state during transcript comparison. Other branches unchanged. |
| [`codex-rs/external-agent-migration/src/sessions/append_tests.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/343be968e5c10b5703f95517eb6ed4be955f4f91/codex-rs/external-agent-migration/src/sessions/append_tests.rs#L103-L138) | `3c4d1f4a86582aab173eef6c3ff2e286953f28f0` | Regression asserts untrusted source state is rejected at prefix/middle/suffix positions and legitimate native destination metadata still permits an exact source-text suffix. |

**Why reject only source state?** Existing CodexDD threads legitimately persist WorkflowState, including trusted Worker scope and runtime snapshots. Rejecting it in destination history would needlessly break future external-session append to previously touched native history, whereas imported source generation has no legitimate reason to supply privileged metadata. The public API is also hardened against future caller mistakes.

## Local Daniel-CL targeted validation — PASS (operator screenshot, 2026-10-11)

- Tested the isolated `E:\codexdd-043-validation\codex-rs` checkout using the previously provided branch/HEAD-pinned command sequence, not active `E:\codexdd`. The screenshot itself shows the checkout path but not `git rev-parse HEAD`; do **not** treat the console image as a signed source-SHA receipt.
- `cargo fmt --all -- --check` cleared its stop-on-failure guard, as evidenced by subsequent execution of the test command. No separate `FORMAT_EXIT` line is visible in this screenshot.
- `cargo test --locked -p codex-external-agent-migration --lib append::tests -- --nocapture` completed with **3 passed, 0 failed, 0 ignored, 0 measured; 162 filtered out**. Specifically: `external_import_source_rejects_workflow_authority_but_allows_native_history`, `returns_the_missing_suffix_from_its_visible_boundary` and `requires_a_strict_nonempty_model_prefix` each reported `ok`.
- The initial compilation took **10m 39s** on Daniel-CL. This is the narrow source regression gate only: no elevated SSH F2 smoke, full 21-stage candidate release profile or final GitHub CI was run.

[updated 136-path Git-object audit](codexdd-0.4.3-packet-3f-posthardening-source-audit.md) ties the single modified overlap object and new upstream-only regression test to the historical audited source; all semantic approval flags remain false.

## Remaining provenance and release gates

- [updated 136-path Git-object audit](codexdd-0.4.3-packet-3f-posthardening-source-audit.md) is now committed: **135 unchanged overlap Git blobs, one modified overlap**, and one new upstream-only test exception (four total); the original 136/22 inventory membership and original blocked discovery are preserved.
- The new imported-state unit check **passed on Daniel-CL**. Previous green Linux 628-test, 13-generator, hosted Windows F2 and PowerShell receipt runs still refer to older source and do not become refreshed final-source CI.
- The **22 conflict strategies**, **136 semantics**, **six mandatory security-domain** review and true elevated/non-elevated Windows acceptance remain owner gated. Only an explicit source semantic acceptance may precede canonical v2 candidate creation, real 21-stage native release receipt and eventual final GitHub CI/PR/installation decisions.

**Current authority disposition:** Code plus local targeted test gate passed; source-object delta reconciled. No source-owner semantic acceptance, canonical candidate, elevated SSH/F2 or native release-profile receipt.
