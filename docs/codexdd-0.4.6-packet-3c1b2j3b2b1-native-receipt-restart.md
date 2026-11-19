# CodexDD 0.4.6 — 3C.1b.2j3b2b1: genuine native receipts across an app-server OS restart

**Opened:** 2026-11-13. **Parent:** 3C.1b.2j3b2a accepted at `4bbda6aab623c4fbf29196af19010e300684b222` ([Actions #43468082893](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43468082893), native real-process 1/1 PASS). **Current status: ACCEPTED focused native Windows 3C.1b.2j3b2b1 at `3772a8e9fed0ff1ee71614fad883bd8ebe74b29f` ([Actions #43475890208](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43475890208), SUCCESS 2026-11-13).** The full 3C.1b.2j3b2b / 3C.1 / release remains OPEN.

## One accepted outcome required

In the deterministic real Windows app-server Core+TUI-reducer integration, perform a *real OS-process shutdown/restart between the original Implementation native `work_packet` receipt and the fresh Validation Worker native `independent_review` receipt*. Preserve original Core first-user role/source authority on resume and original authorized source evidence at the TUI handoff, but refuse the old source receipt/terminal signal when replayed against the new Validator. Then execute a genuinely fresh independent profile in a separate Core Worker thread and accept owner READY **only once**, with duplicate native receipt/signal replays unable to change evidence or cause another READY persistence.

The existing accepted 3C.1b.2i nonrestart test remains separately discoverable. A shared test helper varies only the controlled restart/replay sequence; it does not bypass Core-native tool handlers, issue fabricated PASS records, or change security policy.

- **Source owner:** `codex-rs/tui/src/chatwidget/adaptive_live_app_server_tests.rs`. The two fixture PowerShell scripts are committed to an isolated disposable Git root; a local mock model requests the actual built-in Core tools and the test consumes typed JSON-RPC events, not a synthetic successful TUI item.
- **Native Core identity:** shutdown the first `TestAppServer` child completely; initialize a *different* app-server OS child with the same temporary CODEX_HOME and resume the Implementation thread. Require first accepted user-origin Worker role, exact scope, SHA, and canonical source root.
- **TUI gate refusal:** after the new distinct Validator thread is selected, replay the authentic *previous* Implementation `item/completed` and adaptive signal. Neither may change the gate ledger, add a second source PASS, or create READY. The remaining independent review gate stays pending until the newly executed native Validator receipt.
- **Duplicate terminal refusal:** replay the Validator's genuine completed native item and signal after authorized READY and require exactly one `SetReadyForOwnerQa` persistence event for the Validator thread.
- **Windows proof:** dedicated opt-in workflow `[3c1b2j3b2b1-native]` depends on source-contract and Rustfmt, compiles the real `codex-app-server.exe`, enumerates **exactly one** `codex-tui --lib` test, runs it in an isolated Windows environment and does not release/install/modify active game sources.

## Boundaries and next discriminating packet

This b2b1 test uses authentic Core outputs and the production ChatWidget reducer but does **not** yet prove App-owned automatic `StartAutomaticValidation` dispatch, normal `App::handle_app_server_event` buffering, and persisted TUI checkpoint rehydration **in the same combined restarted process run**. Those coupled App/Core lifecycle assertions remain mandatory for 3C.1b.2j3b2b2. Previously accepted independent j2 and j3a App-event tests, j2i native receipt tests, and j3b2a real OS-process checkpoint tests are not falsely counted as one combined integration result.

If the new Windows regression fails, classify whether the failure belongs to fixture/oracle, environment or product; keep native READY assertions strict. Record exact source SHA, failing step, and smallest discriminating repair. Do not turn a failed or skipped job into PASS. Production stays 0.4.4; frozen 0.4.5, installation and BREAKWATER worktrees remain untouched.

## Accepted source-matched Windows evidence

Run [#43475890208](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43475890208) on SHA `3772a8e9fed0ff1ee71614fad883bd8ebe74b29f`: source-contract preflight PASS; Rustfmt PASS; the dedicated `3C.1b.2j3b2b1` native Windows job PASS. The job enumerated **exactly one** test and executed **1 passed, 0 failed; 5917 filtered out; 7.21 seconds**. This test used genuine Core built-in native tool receipts from distinct Worker sessions and an OS-process restart between source and review. It verified persisted first-user Core origin identity and TUI reducer refusal of stale/replayed source evidence and duplicate Validator owner READY events.

**Acceptance boundary:** the TUI reducer was driven by typed live JSON-RPC notifications in the test harness; it did not exercise the entire App event multiplexer, App-owned automatic Worker session creation and durable TUI checkpoint rehydration in a **single** restarted-process sequence. Those remain `3C.1b.2j3b2b2`, not waived by this green run.
