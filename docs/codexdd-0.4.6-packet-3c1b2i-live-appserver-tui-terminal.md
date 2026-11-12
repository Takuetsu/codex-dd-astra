# CodexDD 0.4.6 — 3C.1b.2i: live app-server native receipt → TUI owner terminal

**Prepared:** 2026-11-12. **Parent:** `7c08b07a147a118c036e7c76ff2bdc3ef3181fc1` (3C.1b.2h focused TUI accepted).
**Status:** SOURCE IMPLEMENTATION; isolated native Windows CI **PENDING**. Milestone 3C.1 and release 0.4.6 **OPEN**. No BREAKWATER or installed-binary changes.

## Acceptance gap

3C.1b.2g proved actual app-server Core Implementation + independent Validation receipts and cold-resume identity. 3C.1b.2h proved the TUI can ingest typed fixture notifications to satisfy exact owner gates, refuse forged/foreign/stale proofs and reduce READY terminals. Neither test joined the *real app-server process* and the TUI reducer on the same native validation run. This packet joins them without touching a game source tree or requiring any live AI model or user credentials.

## New integration test

The new Windows-only `codexdd_046_3c1b2i_real_app_server_to_tui_native_ready_handoff`:

1. Creates a private Git repository and **commits** two deterministic repository-owned native PowerShell fixture scripts. Each emits real contract-v1 stage records and returns exit code 0; no subprocess edits source files. Binds an exact HEAD SHA and canonical root.
2. Starts a real `codex-app-server.exe` process using the existing `TestAppServer` JSON-RPC harness and a local mock Responses backend. The mock model controls only deterministic *built-in* tool requests; the Core runs the native scripts. The fixture explicitly limits no-sandbox execution to these disposable CI sessions and does not alter production policy.
3. Starts a real Implementation app-server thread without launcher-bound authority. Its **first user-origin message** supplies the exact role, scope, pinned root, SHA and two owner-required gates: `source_profile(work_packet)` and `independent_validation`. The test reads a **real** `item/completed` UserMessage notification, decodes it as the TUI protocol and feeds the production `ChatWidget::handle_server_notification`. The TUI must create exactly those two pending gates without manual source evidence insertion.
4. On a real later source-bound MechanicalValidation turn, the model invokes the built-in `run_codexdd_validation` with `work_packet`. The Core executes the native script. The test receives the real `item/completed` event through JSON-RPC, verifies native run ID + SHA/profile/status, feeds it into the real TUI reducer and requires *only* the owner source-profile gate to be satisfied. A separate `report_adaptive_signal(ready_for_validation, evidence_ref)` built-in call emits a real `turn/adaptiveRuntimeSignal` event. The real terminal completion must latch `ReadyForValidation`, without converting an Implementation PASS into Validator authority.
5. The production TUI state creates a **fresh** Validation Worker handoff (separate app-server thread, original origin ID and source-pin/gate-ledger inheritance). That thread receives its *own* first user-origin Validation assignment; it never reuses the source session as a Validator. It invokes `run_codexdd_independent_validation` on its own ordinary turn. Only the native independent receipt can complete the remaining gate.
6. A separate real `report_adaptive_signal(ready_for_owner_qa, evidence_ref)` call and terminal notification must latch `ReadyForOwnerQa` on the Validation Worker. It must issue a READY persistence event for the exact Validator thread, cite the actual native receipt and leave the candidate clean. The two native run IDs and app-server thread identities must be different. Any missing native item or signal fails the integration test.

JSON-RPC method/params bytes come from the **live native app-server stream**, not a fabricated ItemCompleted/AdaptiveRuntimeSignal fixture. The test decodes them into the production `ServerNotification` variants, then dispatches through `ChatWidget::handle_server_notification`; it does not directly inject a successful ledger state. The TUI's complexity classification and pre-existing MechanicalValidation phase are intentionally seeded for this *handoff* integration test; the upstream adaptation/reconnaissance transition remains covered by earlier tests.

## Windows validation

Dedicated `[3c1b2i-native]` GitHub Actions job pins branch/SHA, source contracts, Rustfmt, Windows Rust 1.95, DotSlash and verified V8. It explicitly builds the real `codex-app-server` executable, discovers exactly one new `codex-tui --lib` test, and runs it with native Windows scripts. On green: 3C.1b.2i integration acceptance is scoped to this **deterministic mock-model, real-process JSON-RPC-to-TUI reducer** test.

## Explicit limits and follow-up

This is not a real graphical/interactive TUI terminal acceptance, does not prove the TUI's complete app-server transport event multiplexer or automatic app-managed new-thread creation, does not exercise every live restart/replay/source drift sequence, and does not perform a full six-stage independent script or actual owned listener/HTTP health. All production and release gating remains **closed**. Any capability/policy mutation must be separately reviewed.

After this test is green, drive the actual TUI `AppServerSession` transport/automatic session creation and bounded restart/replay negative paths in the next 3C packet; continue 3D owned runtime health, remaining milestone phases, complete Windows distribution and regression acceptance. Only then approve installation and the approximately one-week single-milestone BREAKWATER soak.
