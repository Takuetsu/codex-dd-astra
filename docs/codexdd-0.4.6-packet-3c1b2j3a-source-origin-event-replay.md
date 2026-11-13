# CodexDD 0.4.6 — 3C.1b.2j3a: source-origin handoff and replay refusal

**Opened:** 2026-11-13. **Parent:** 3C.1b.2j2 accepted at `628d75c606ba499818344f561178fa214f72db76` ([GitHub Actions #43435363662](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43435363662): source preflight, Rustfmt, and 2/2 native Windows tests green).

**State:** proposed source/test packet, isolated Windows CI PENDING. The full 3C.1b.2j3 cold-process acceptance, 3C.1 and 0.4.6 are NOT accepted.

## Problem / exact boundary

A genuine `ReadyForValidation` signal previously queued the generic `AppEvent::NewSession { worker_binding }`, indistinguishable from manual session creation. A delayed duplicate delivered **after** the source thread lost foreground ownership could create a fresh unpinned Validator at the launcher CWD. A replayed scope-only message also lacked the source thread ID and committed SHA, so the TUI could not verify the original owner's authority at dispatch time.

This packet introduces a separate, private `AppEvent::StartAutomaticValidation { origin_thread_id, source_sha, worker_binding }`. The actual trusted terminal emits this event only when the still-terminalized Implementation Worker has the original owner gate ledger, matching source pin and scope. The normal App event dispatcher requires **both** the displayed source Worker identity and the exact original source SHA/ledger/binding at delivery. The already accepted 3C.1b.2j2 live read-only Git preflight then rechecks the selected worktree and HEAD before starting Core `thread/start`. Manual `NewSession` is unchanged. No model-supplied thread ID or fake receipt can authorize automatic validation.

An Implementation with missing owner source/gates now produces a `SETUP_BLOCKED` explanatory notice instead of silently requesting a new unpinned Validator. The existing Repair-role route cannot inherit independent gate authority from the Implementation, so an automatic Repair-to-Validation event is not issued; it requires an explicit authorized recovery rather than pretending to be independent.

## Focused tests

`codexdd_046_3c1b2j3a_auto_handoff_refuses_duplicate_after_new_validator`: authorized source fixture starts exactly one independent Validator on a committed disposable Git root; preserves source-ledger pending gates and one first user-origin independent assignment; replaying the identical event after Validator navigation must not create a second thread, second prompt or READY.

`codexdd_046_3c1b2j3a_auto_handoff_refuses_stale_origin_sha_and_reloaded_state`: the normal App event path refuses wrong origin ID, forged/mismatching source SHA, missing owner gate ledger, changed Git HEAD, a reconstructed state with no source READY terminal, and an unsupported Repair-role handoff. Assert **zero** fresh `thread/start` calls and an actionable `SETUP_BLOCKED` notification each time.

Only the exact `[3c1b2j3a-native]` commit-scoped Windows CI job can accept this packet. Preflight, Rustfmt, and exactly two native TUI library regressions must pass. No built binary is promoted or installed.

## Explicit remaining work

**3C.1b.2j3b:** actual cold app-server process shutdown/restart with a persisted App-owned source/Validator identity, no replay of stale `READY`/prior PASS receipts, changed-SHA/moved-root refusal at live resume/QA and no duplicate autonomous Validator creation. Then outstanding 3C.2, 3B.4, 3D–3F and Phase 4 full release gates. Until accepted, BREAKWATER/CV development and installed binaries are unchanged.
