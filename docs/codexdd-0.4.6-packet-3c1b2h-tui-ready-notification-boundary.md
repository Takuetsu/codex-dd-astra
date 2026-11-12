# CodexDD 0.4.6 — Packet 3C.1b.2h: TUI native item completion → owner READY gate

**Date:** 2026-11-12. **Parent:** `ad20adee456fa55a725678e5786dfab9cda2ff03` (3C.1b.2g accepted).
**Status:** SOURCE IMPLEMENTED; targeted Windows TUI native regression **PENDING**. Overall 3C.1 and CodexDD 0.4.6 **OPEN / NONPRODUCTION**.

## Finding and narrow correction

3C.1b.2g established that the **real** app-server forwards the actual native Implementation work_packet and separate Validation independent_review tool completions, preserving source SHA/root/run identity through an app-server restart. The TUI's `register_adaptive_evidence` already enrolled independent Validator receipts in an inherited owner ledger; however, the corresponding Implementation source receipts were **not automatically observed in the owner-required ledger**. An earlier unit fixture invoked `observe_source_pass` manually after `register_adaptive_evidence`, disguising the missing live event-to-gate link. Without that link, native source PASS could not satisfy an owner source-profile gate through the normal notification path, leaving positive READY impossible without a test-only insertion.

This packet wires the received `item/completed` native Implementation LVO PASS to `AdaptiveGateLedger::observe_source_pass`, using the real thread, turn, profile, evidence ID, unique native run ID, exact authorized source root/SHA, completion timestamp and original locked Implementation role. The registry and ledger reject partial, generic, foreign, wrong-role, wrong-SHA, unknown-profile, conflicting or stale completions. Ledger state remains an evidence *reference* and is resolved again on every READY attempt: a later conflict does not retain a cached PASS.

The separate Validation producer remains strictly required. A successful native source LVO, work_packet, targeted profile, generic exec or model narrative cannot substitute for `IndependentValidation` or `OwnedRuntimeHealth`. The TUI's positive owner gate logic is **not loosened**.

## Focused TUI regression proof

Two new `codexdd_046_3c1b2h_` tests drive typed app-server-compatible `ItemCompletedNotification` structures, serialized and decoded like the JSON-RPC payload, through the actual `ChatWidget::handle_server_notification` live dispatcher. They test:

1. Exact source Implementation receipt automatically satisfies the source gate without test-only manual insertion; the trusted `ReadyForValidation` signal creates a terminal and an inherited, separately bound Validation Worker handoff; the fresh Validator receives only the pending independent gate and the original source evidence references.
2. Foreign-thread review is ignored. Wrong-SHA review, a generic receipt, changed active Validator SHA and wrong role fail READY. A conflicting re-delivery using the same native review evidence ID poisons the original result. Only a **new, distinct native review run** can make the independent gate current again. The true trusted `ReadyForOwnerQa` terminal can then be latched and emitted for persistence only on the correct Validator thread with the original owner requirements all satisfied.
3. Historical TUI snapshot replay must not mint live native Validator authorization. Even a legitimate independently bound native review cannot complete an owner ledger when mandatory release-source and owned runtime-health gates remain pending.
4. The original `3C.1b.2c` live-notification unit test no longer calls `observe_source_pass` directly; it asserts the source gate is now observed by the production ingestion path.

The test uses isolated worktree fixtures and realistic notification structs; it **does not** launch another native app-server, execute real native PowerShell, connect the actual TUI process to the real app-server, or prove a full authorized owner QA workflow in a game project. Those acceptance boundaries are still separate.

## Isolated Windows CI

A dedicated `[3c1b2h-native]` push job validates exact branch/SHA and Rustfmt, compiles the focused Windows `codex-tui` library tests with repository-pinned Rust, DotSlash and V8, asserts exactly two new tests are present and reruns the one existing 3C.1b.2c handoff case. No changed game worktrees, installations, published artifacts or production authorizations.

## Follow-on acceptance

After the focused native TUI test passes: (a) prove true end-to-end app-server→TUI live stream against a native disposable candidate, not just two separately tested boundaries; (b) include refusal under real restart/replay/pinned source changes; (c) implement 3D actual owned listener and HTTP health receipt admission where required; (d) complete all remaining 3C/3D/3E/3F/Phase 4 source, Windows distribution and release gates. Only **after full 0.4.6 completion and approved install/rollback** begin the approximately one-week, one-milestone BREAKWATER soak.
