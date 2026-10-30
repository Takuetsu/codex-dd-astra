# CodexDD 0.4.5 — 3F.54: test-only semantic correction and Daniel-CL native gate

**Status: NONRELEASE; source changes and immutable provenance re-bound, native behavior not yet accepted.** Built on the 3F.53 locked generator candidate. No product runtime, security graph, production version, Bazel release rule, installer, or shipping binaries were changed.

## Why this packet was necessary

The prior 3F.51 audit identified two deliberately non-upstream TUI/Core test behaviors and an untested app-server failure boundary. The Core normalization change is intentional: a missing output for a local/inherited custom tool or LocalShellCall synthesizes an `aborted` output, rather than triggering upstream's debug-only panic. Existing local/inherited normalization tests and three upstream orphan-output debug panic tests remain present; the native gate exercises both policy paths. **We do not re-introduce upstream should-panic cases that contradict CodexDD's accepted recovery semantics.**

Two **test-only** edits were made:

1. `codex-rs/tui/src/app/tests/session_lifecycle_requests.rs`: restored upstream's `boxed_future!(local_daemon_registers_approval_gated_mcp_tools_for_both_start_paths_scenario())` entrypoint, placing the large MCP lifetime future behind an allocation for Windows test-stack safety. The scenario continues to test approval-gated delegation tools, missing permissions fields, server bearer authorization, suspend/reconnect, preserved tool configurations and denial of unapproved tool startup.
2. `codex-rs/app-server/src/bespoke_event_handling.rs`: added `adaptive_runtime_signal_does_not_replay_on_failed_or_interrupted_turn` native regression. It first consumes the live, source-turn-identified signal; then separately verifies **failed** and **interrupted** turn completions send only `TurnCompleted`, with no replay. The existing success-terminal test continues to check adaptive replay is emitted before `TurnCompleted`.

Current source Git objects: TUI `7d2be3352380d43df714399b441a95960b87a59f`; app-server `693eac395673e062d27bbf2cf7b467e9f6af9a5a`.

## Provenance and owner ledger

- Updated both exact reverse-hunk source receipts (`codexdd-045-3f51-core-tui-upstream-delta.json` and `codexdd-045-3f52-appserver-owner-remaining-deltas.json`) to reconstruct official 0.163 bytes **from the new current blobs**.
- Updated `docs/upstream-candidates/codexdd-045-reviewed-semantic-corrections.json` using the repository's existing `before_blob → after_blob` correction protocol. Preserved the original TUI historical prior identity as `previous_after_blob`, did not mutate the archived upstream overlap receipt, and added the test-only app-server correction row.
- Updated the exact 109-row owner matrix source fingerprints and single app-server disposition count, then updated the strict owner-roster test to match. All 109 rows remain `PENDING_OWNER_REVIEW / NOT_ACCEPTED`; no source review or test is silently converted to release approval.
- Existing generated JSON/TypeScript/Python schema and SDK outputs and their 12 generated owner blobs remain untouched.

## Native validation, bounded and exact-source

`scripts/codexdd-045-3f54-targeted-semantic.ps1` is a **Daniel-CL-only** script with default `-Mode Preflight`. Preflight executes the pinned dependency/109-path source audit, all 3F.51/3F.52 source-delta and generated schema parity tests, without compiling Rust. `-Mode Native -ExpectedCommit <full-40-char-candidate-SHA>` additionally runs locked Cargo metadata, `rustfmt --check` on the two intentionally changed Rust test files and the following nine focused native selectors:

- Core custom-tool missing/aborted-output recovery, local-shell missing/aborted-output recovery and orphan-output integrity.
- App-server adaptive signal success/failure/interrupt tests, plus existing failed completion behavior.
- TUI local-daemon MCP delegation and reconnect test (restored boxed wrapper), compatibility with older external server, fresh startup model/default precedence and remote workspace-root propagation.

It stops immediately on the first failure, preserves tracked source identity and prints one machine-readable receipt with stage names, exit status, source SHA and elapsed time. It **does not run the full 21-stage release profile**, change production, or declare release approval. To satisfy the established log preference, execute over SSH **from Stonks** and pipe the entire output into `Tee-Object` on Stonks; no manual copying of Daniel-CL log files should be necessary. A clean dedicated worktree on Daniel-CL is still required.

## Separate generated-schema native gate

The source-frozen, read-only 3F.53 generator workflow on its immutable parent SHA uses the **current secured Cargo.lock** (`13bc15ff844e93dfaf7777c73bb7313b35bb3b81`) rather than the older integration-only `a6cd27...` lock and checks stable and experimental compressed schemas, config schema, Python SDK tests and zero drift without writing accepted outputs. It runs separately and is not proof of 3F.54 native TUI/Core behavior.

**Do not mark any of the 109 owners accepted without exact-source native evidence and appropriate review.** App-server external client workflow-state authorization and stale/foreign source-turn rejection, full daemon/WSL behavior, and final 21-stage release suite remain independent gates. All production markers stay `0.4.4 / rust-v0.162.0`.

## Remaining five original core/TUI direct conflicts — source-delta closure

The source-only 3F.54 follow-up also obtained and pinned **exact per-hunk reverse reconstructions** against official `openai/codex@rust-v0.163.0` for `core/context_manager/normalize.rs`, `core/tools/registry.rs`, `tui/app/startup.rs`, `tui/chatwidget/interaction.rs`, and `tui/lib.rs`. They are recorded in `docs/upstream-candidates/codexdd-045-3f54-core-tui-remaining-conflict-deltas.json`; the companion `test_codexdd_045_3f54_remaining_core_tui_upstream.py` fails if a single hunk changes or any protected upstream source is missing.

Together with the four source paths audited in 3F.51, **all nine original direct core/TUI conflict paths** have exact upstream source-delta provenance. This is **not** final semantic owner acceptance for these nine, nor for the other 56 core/TUI owner overlap paths.

The preliminary full-workspace rustfmt source gate also reported a formatting difference in an **unmodified** `tui/src/app/tests/active_reconnect_tests.rs` file. 3F.54 deliberately does not rewrite that unrelated source and cause a third owner fingerprint/correction. The native targeted runner checks Rust formatting **only for the two files intentionally edited** in this packet; whole-workspace formatting remains a distinct final release gate.
