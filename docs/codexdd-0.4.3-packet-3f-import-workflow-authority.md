# CodexDD 0.4.3 — 3F imported WorkflowState authority boundary

**Status: NARROW SOURCE HARDENING COMMITTED, DANIEL-CL TARGETED TEST PENDING; NOT OWNER SEMANTIC ACCEPTANCE.**

## Scope and exact ancestry

- Production remains `16fdc9826e800ebabab920eadd8e1d976d2f645f` (CodexDD 0.4.2). Official upstream target is `rust-v0.162.0`, peeled `c1382380de69521303b416720a52f42d51af6248`.
- Previous fully inventoried 136-overlap Rust snapshot: `32ab09a141d626df5851965a557e0dce877ac2ec`, full tree `a0bce7166cccb52e9b4fc8d658e0c08616752e62`.
- Latest modified product source: [`343be968e5c10b5703f95517eb6ed4be955f4f91`](https://github.com/Takuetsu/codex-dd-astra/commit/343be968e5c10b5703f95517eb6ed4be955f4f91), full tree `4f58649164ea5ffcfa782b8c9075d68eafe5db65`. The previous object inventory remains **historical** until current-tree re-verification. The new source differs from the previous Rust snapshot in **exactly two files**, verified by GitHub compare; no updater, CLI, daemon, model router, generated SDK, version or release marker edits.
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

## Required targeted local validation (not claimed passed)

- Use the pre-existing isolated `E:\codexdd-043-validation` checkout on Daniel-CL, not active production `E:\codexdd`. Require tracked checkout files clean and exact source `343be968e5c10b5703f95517eb6ed4be955f4f91`; avoid treating the known unrelated untracked `M` file as part of a canonical candidate.
- Cheap source gate: `cargo fmt --all -- --check`, from `codex-rs`, expecting exit code zero and no formatting hunks.
- Targeted executable unit gate: `cargo test --locked -p codex-external-agent-migration --lib external_import_source_rejects_workflow_authority_but_allows_native_history -- --nocapture` with `RUST_MIN_STACK=16777216`. Verify that **one test actually runs**; an exit-zero zero-test filter is not acceptance. Existing prefix/append tests may be selected as a small follow-up after this gate.
- If a local test fails, make a narrow GitHub-side correction and repeat the affected Daniel-CL check. No routine GitHub debugging runs.

## Remaining provenance and release gates

- Update current-source Git-object inventory after passing local checks: original 136 overlap set remains intact, but `append.rs` now has a new blob, and upstream-only test `append_tests.rs` is an additional intentional exception. All untouched 135 overlap identities can be mechanically reverified against the historical 136-row manifest; do not rewrite the original `blocked_transplant_conflict` discovery.
- Previous green Linux 628-test, 13-generator, Windows hosted F2 and PowerShell receipt runs were on older source. Treat this new source test as **pending**, not covered by earlier CI.
- The **22 conflict strategies**, **136 semantics**, **six mandatory security-domain** review and true elevated/non-elevated Windows acceptance remain owner gated. Only an explicit source semantic acceptance may precede canonical v2 candidate creation, real 21-stage native release receipt and eventual final GitHub CI/PR/installation decisions.

**Current authority disposition:** Source change committed; local proof and renewed candidate-bound review not yet established.
