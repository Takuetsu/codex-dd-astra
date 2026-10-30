# CodexDD 0.4.5 — 3F.51: generated schema, app-server protocol and original core/TUI conflict audit

**NONRELEASE / REVIEW EVIDENCE ONLY.** This packet builds on [3F.50 draft PR #60](https://github.com/Takuetsu/codex-dd-astra/pull/60), source baseline `1f7c938baa5bebfcbb67d87667c88022376b1900`, and official upstream `openai/codex@rust-v0.163.0`. It deliberately changes **no product Rust code, generated schema output, Cargo/deny source, Bazel release rule, or production markers**.

## 1. Generated schema and SDK parity

The source-bound `test_codexdd_045_3f51_generated_protocol_parity.py` checks:

- `ServerNotification.json` is the **86-method public server notification schema**. The Python SDK's `NOTIFICATION_MODELS` must have exactly the same **86** keys. The TypeScript `ServerNotification.ts` and `ServerNotificationEnvelope.ts` preserve the same **88** methods; the **two** deliberately TS-only methods are upstream **internal-only** `rawResponseItem/completed` and `rawResponse/completed`. An additional missing or unexpected method is a hard regression, not an automatically waived mismatch.
- `turn/adaptiveRuntimeSignal` has matching Rust/JSON/TS/Python envelope, source-turn ID, signal kind, evidence refs and nullable diagnostic note. The public signal enumeration remains an **inert transport**—nothing here authorizes an untrusted signal to change model policy.
- `thread/workflowState/update` retains an exactly matched Rust/JSON/TS/Python request, `threadId`-owned dispatch, optional source-turn ID and adaptive state, and the three operation values `setReadyForOwnerQa`, `setAdaptiveState`, `clear`. `setAdaptiveState` without a state is rejected.
- Compressed precomputed **stable and experimental** schema `.json.zst` outputs retain their exact pinned Git objects and nontrivial binary sizes. This is **not** byte-exact regeneration of compressed outputs under the current source SHA; locked native stable/experimental regeneration and SDK tests remain mandatory.

## 2. Exactly reversible current-fork versus official-0.163 diffs

The two machine-readable receipts and `test_codexdd_045_3f51_exact_upstream_semantic_deltas.py` verify the actual current Rust Git blob, reverse **every individual addition/replacement/deletion**, then require a reconstructed Git SHA-1 matching the fetched **official upstream 0.163 Git blob**. A changed line anywhere in these 10 files fails even if its text happens to resemble another hunk.

**Six app-server and protocol paths:**
- `app-server-protocol/src/protocol/common.rs`: exactly eight extra lines for `thread/workflowState/update` and `turn/adaptiveRuntimeSignal` definitions; upstream methods unchanged.
- `app-server-protocol/src/protocol/thread_history.rs`: exactly one extra `RolloutItem::WorkflowState(_)` suppression, keeping private workflow mutations out of model-visible history.
- `app-server-protocol/src/protocol/v2/thread.rs`: one **76-line** fork-only workflow-state operation, snapshot and request/response type block.
- `app-server-protocol/src/protocol/v2/turn.rs`: one **18-line** source-turn-scoped adaptive notification block.
- `app-server/src/message_processor.rs`: five fork-only lines routing workflow-state requests to `turn_processor`.
- `app-server/src/request_processors/turn_processor.rs`: one **61-line** fork-only handler that loads a thread, converts adaptive state, rejects missing `adaptiveState` for `setAdaptiveState`, and persists a private workflow-state item.

**Four original direct core/TUI conflict paths:**
- `core/src/context_manager/history_tests.rs`: exact reconstruction documents **two** local-call debug/release test attribute changes and removal of **two** upstream expected-debug-panic tests for missing custom-tool/local-shell output. The fork instead synthesizes aborted output for interrupted calls. This intentional difference is **not** the same as upstream behavior; the retained **three orphan-output debug panic tests** and subsequent native history reconciliation must remain intact.
- `tui/src/app/tests/session_lifecycle_requests.rs`: 16 exact edit hunks, retaining fork Worker/adaptive snapshots and six-scenario upstream Daybreak/API-key/history negotiation. Notable review gaps: the upstream `local_daemon_registers_approval_gated_mcp_tools_for_both_start_paths` test had an explicit `boxed_future!` wrapper; the fork still has the body but has removed that wrapper, potentially affecting Windows test-stack characteristics. Targeted native execution is required before signing off.
- `tui/src/app/tests/startup_defaults_tests.rs`: official upstream code plus precisely one `/*adaptive_family*/ None` argument.
- `tui/src/app_server_session/workspace_roots_tests.rs`: official upstream code plus precisely one `/*turn_trigger*/ None` argument, preserving the server-owned remote writable root tests.

The byte reconstruction proves **what changed**, not whether the resulting fork behavior satisfies all trust and lifecycle requirements.

## 3. Protocol, event and security-adjacent semantic source checks

`test_codexdd_045_3f51_appserver_event_semantics.py` separately pins:

1. Thread-specific RPC serialization; the three workflow operations' private rollout persistence without adding a user/model message.
2. Adaptive signal provenance from Core's event turn ID, a thread summary copy, successful-turn replay **before** terminal completion, and absence of replay on interrupted turns.
3. Thread-state terminal tracking and reset.
4. TUI typed workflow-state RPC with explicit thread ID, optional source-turn ID and typed state.
5. Core reconnaissance/mechanical/validation policy both **before and after** PreToolUse hooks, with terminal blocked completion, plus press-only manual Escape routing.

These are structural guards, **not** proof against arbitrary external client workflow-state injection, stale mismatched-turn signals, asynchronous reconnect ordering, or native TUI/daemon UI races. Those remain separate behavioral and threat-model gates.

## 4. Semantic status and next execution gates

**No owner accepted:** Original 109-path semantic owner ledger remains `109 PENDING_OWNER_REVIEW / NOT_ACCEPTED`; this packet does not edit source/blob fingerprints, change signatures, waive cargo-deny, or authorize the 21-stage release suite. The 10 full-file reversible deltas and generated cross-SDK checks move source audit readiness forward, but the original 15 direct conflicts still demand an independent final acceptance decision.

After the one-shot cheap source test, prioritize **targeted** native selectors on exact frozen source, recheck generated stable/experimental schema and SDK without drift, and specifically test history interrupted-output synthesis, local daemon MCP test stack safety, WSL/daemon startup/reconnect, adaptive signal replay and authorization boundaries. Only after all 109 owner paths have independent semantic approval should the unchanged 21-stage Daniel-CL release profile be invoked. Production remains `0.4.4 / rust-v0.162.0`, PR #58 and #60 remain draft, and no shipping binaries may be installed/published from this packet.
