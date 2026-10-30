# CodexDD 0.4.5 — 3F.60: remaining 18 Core + 38 TUI upstream semantic source audit

**Source reference:** frozen CodexDD 0.4.5 semantic candidate `72cd09d3f52c076d445078758af20489ccdb4e74`. **Official comparison:** `openai/codex@rust-v0.163.0`. This packet adds **no production Rust edits**; all source differences are recorded as exact reversible Git-hunk receipts in seven manifests `codexdd-045-3f60-*-deltas-{a,b,c,d,e}.json`.

## Result

- **18/18 formerly queued Core authority/history paths reconstructed byte-exactly** against official upstream; two nine-file manifests include each current Git blob, official Git blob and every added/removed line.
- **38/38 formerly queued TUI adaptive/daemon paths reconstructed byte-exactly**; five manifests cover app, session routing, UI/Daybreak, tests and startup.
- Combined with 3F.51–3F.59, the formal signoff ledger now has **109/109 source comparisons and evidence records submitted; 0 formally OWNER_ACCEPTED**. It is **not** a claim that all semantics or native test selectors passed.
- The original 109-path owner matrix remains `PENDING_OWNER_REVIEW / NOT_ACCEPTED`, unchanged by this packet. `test_codexdd_045_3f60_all_56_core_tui.py` rejects a changed current blob, missing upstream line, missing owner, missing risk hold or falsely promoted approval.

## Core behavioral review findings

Core fork changes fall into private workflow-state retention/exclusion, child/fork propagation, model-visible histories and tool authority. Most are additive, but they alter sensitive contracts:

- **Required tool choice and terminalization**: `core/src/client.rs` changes upstream `tool_choice: "auto"` into a derived mandatory tool on trusted terminalization turns. Guard against spoofed triggers and off-turn leakage; test both model-request and ordinary-turn negative cases.
- **Tools and permissions**: `core/src/tools/mod.rs` changes direct tool selection for reconnaissance, implementation and mechanical validation; `tools/spec_plan.rs` registers adaptive signals and phase-gated validation tools. Validate source-turn ownership, handler availability, tool policy before/after pre-tool hooks and restricted phase transitions.
- **Child workflow inheritance**: `core/src/thread_manager.rs` propagates/restores private WorkflowState into child/fork histories; prove copied snapshot can preserve preference but cannot inherit pending executor permissions or live signal authority.
- **Evidence IDs and replay**: `tools/handlers/dynamic.rs` preserves tool call identifiers in returned evidence, `codex_thread.rs` flushes private workflow mutations; `session/rollout_reconstruction.rs` and `spawn.rs` keep private data out of model-visible transcript, while `client_tests.rs`, `thread_manager_tests.rs`, `tools/spec_plan_tests.rs`, `unified_exec.rs` and `rollout_reconstruction_tests.rs` include focused regressions.

## TUI behavioral review findings (P0 holds, not waived)

**Upstream behavior is demonstrably missing or changed** in the current fork. Exact-diff evidence is not permission to ignore it:

1. **`tui/src/app.rs`**: upstream protects startup modal input and dispatches rendered selections before composer commands; the fork drops `discard_startup_modal_input` and `handle_rendered_selection_event` early returns. Offline disconnected active-view handling also differs. Requires upstream restoration or a proven equivalent plus native negative tests.
2. **`tui/src/app/event_dispatch.rs`**: fork omits upstream `verify_managed_daemon` on warnings/daemon menu and reshapes realtime stop and API-key model selection handling. Requires native reconnect/voice/API-key acceptance.
3. **`tui/src/app/config_persistence.rs`**: upstream live config tests for session-picker and resume-cwd preferences were reduced to resize-only assertions. Restore native coverage or document/test equivalent behavior.
4. **`tui/src/app/tests.rs`**: upstream `background_resolution_keeps_matching_foreground_user_input_visible` regression was deleted, as were other request/reconnect assertions. Restore test and reject any foreground input loss.
5. **`tui/src/app/thread_routing.rs`**: upstream bounded `mpsc::Receiver` / `try_send` with full-buffer handling became an unbounded channel and plain `send`. This changes backpressure and memory properties; negative saturated-event tests or equivalent envelope limits are required before signoff.
6. **`tui/src/chatwidget/tests.rs`**: upstream `terminal_program_status` test module was unregistered. Restore it or justify and replace its tests.
7. **`tui/src/chatwidget/tests/slash_commands.rs`**: upstream API-key Daybreak gate assertions and Windows image-paste shortcut coverage were deleted. Restore the tests or prove equivalent negative coverage.
8. **`tui/src/app_server_session.rs` and `chatwidget/settings.rs`**: ensure typed workflow mutations are scoped to owning thread/source turn and API-key Daybreak paths do not bypass access programs. Their new logic is high priority.

Other additive TUI changes encompass adaptive model startup, trust signals/terminal ordering, reset of automatic successors on manual input, managed Worker bindings, elevated Windows daemon fallback and progress/status display. They also require native behavioral tests before acceptance.

## Native acceptance contract (not yet green)

Run a **bounded native Windows gate** at exact frozen source, after cheap source checks: Core model tool-choice, phase-gated signal/LVO, child/fork state, history repair/evidence ID; TUI startup, manual input, daemon reconnect, active modal, disconnected view, remote workspace, Daybreak feature gating and lost-upstream-test cases. A green Rust build alone cannot prove missing guard semantics; P0 holds remain open until equivalent behavior is restored or explicitly reviewed with actual negative tests.

No 21-stage release, no schema/Cargo rewrites, no Windows installer and no production 0.4.5 upgrade are authorized.
