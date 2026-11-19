# CodexDD 0.4.6 — 3C.1b.2j1: App transport routing of native evidence

**Opened:** 2026-11-12. **Parent acceptance:** 3C.1b.2i at `455c1f2bc0432fae6d6f864d66301624b38c6df5`, [Windows Actions #43334938407](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43334938407). **Status:** ACCEPTED — 2026-11-12 focused Windows CI [#43347103729](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43347103729), exact checkpoint `58dd0a3e06f4fbc619f7fc5d85209c36298f2af2`. 3C.1 and release 0.4.6 remain OPEN/nonproduction.

## Outcome and ownership

Prove `App::handle_app_server_event` → `handle_server_notification_event` → per-thread `enqueue_thread_notification` → foreground `ThreadBufferedEvent` → `handle_thread_event_now` preserves selected Worker native tool receipts and **does not** route unowned Validator evidence or READY signals into the active Worker's TUI state.

3C.1b.2i already proved actual Core/app-server JSON-RPC→`ChatWidget` native receipts and gates. It **bypassed the normal App event multiplexer** to test the reducer in isolation; this packet fills only that App-level dispatcher/buffer/ownership gap with *typed fixture notifications*, not a new Core-native producer claim.

Review-only owners: `app/app_server_events.rs`, `app/app_server_thread_ownership.rs`, `app/thread_routing.rs`. Test: `codex-rs/tui/src/app/tests/adaptive_native_event_routing.rs`, registered in `app/tests.rs`. CI: `.github/workflows/codexdd-046-3b1-github-regression.yml`.

## Focused acceptance

The single `codexdd_046_3c1b2j1_app_routes_only_selected_thread_native_evidence` regression uses a real embedded `AppServerSession` and deterministic typed App notifications:
1. Select a primary Worker and active event channel; send an `ItemCompleted` native-tool-shaped source receipt through the normal App entry point.
2. Assert source thread, tool ID, `work_packet` profile and run ID arrive unchanged on its channel, then enter the ChatWidget reducer only when drained. With no owner-authorized gate/source pin, no READY.
3. Send a different-thread independent Validator PASS and `ReadyForOwnerQa` signal through the same App entry point. Both must be refused as unowned, leave the foreground queue empty, not create foreign thread ownership, and not request READY persistence.
4. Tear down the embedded session cleanly.

The `[3c1b2j1-native]` Windows CI job must run exact source/branch preflight, Rustfmt, enumerate exactly one named TUI library test, execute it and leave production unchanged. Any red assertion remains red until classified and repaired at its actual owner path. No test-only manual PASS insertion, relaxed thread ownership, changed security policy, project worktree edits, install, release, or milestone READY.

## Follow-up

**3C.1b.2j2:** App-owned automatic fresh Validator start and first user-origin Worker header, pinned CWD, immutable owner gate ledger and actual thread attachment. **3C.1b.2j3:** bounded App process restart/replay and source-drift refusal. After that: remaining 3C.2, 3B.4, 3D, 3E, 3F and Phase 4 required Windows and real-agent acceptance. Only full 0.4.6 approval authorizes installation; then the planned one-milestone, roughly one-week BREAKWATER soak.

## Native acceptance evidence (run #144)

Source-contract preflight PASS; Linux Rustfmt PASS; dedicated Windows `3C.1b.2j1 Windows App event routing and foreign READY refusal` PASS. The Windows job enumerated exactly one named `codex-tui --lib` test and executed it: **1 passed, 0 failed, 5910 filtered out, 0.17s test runtime**. The test used the normal App event multiplexer and foreground buffer, asserted unchanged source tool ID/profile/native-run ID, then refused foreign independent Validator receipts and READY signals without creating a foreign thread or requesting READY persistence.

Known repair history (not release blockers): #43340044479 stopped at Rustfmt; #43341388909 reached Windows compiler E0659 ambiguous `assert_eq!`; #43345412227 stopped at Rustfmt import ordering. The accepted run at `58dd0a3` passed both checks and executed the Windows test. This remains a deterministic typed-notification App routing test and is not a second genuine Core-produced native stream or a full interactive TUI test. Next packet 3C.1b.2j2 covers automatic source→fresh Validator thread creation, pinned CWD, exact original owner gates and initial user-origin prompt.
