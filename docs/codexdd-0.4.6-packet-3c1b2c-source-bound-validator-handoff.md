# CodexDD 0.4.6 — 3C.1b.2c: source-bound independent Validation Worker handoff

**Date:** 2026-11-12. **Parent:** `607cd820850a4dff54fae7d80b9d7d8c9c2be6cd` (3C.1b.2b native Windows accepted).
**State:** SOURCE IMPLEMENTATION ONLY; Rustfmt/native Windows, independent fresh-worker real-process acceptance, owner gate issuance and full 3C.1 acceptance **PENDING**.

## 3C.1b.2b prerequisite verified

[Windows GitHub Actions #43235556273](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43235556273): SUCCESS, exact `607cd820`, Core source-preflight + Rustfmt + 3/3 focused Core producer/role/source tests; run duration 21m57s. This does not prove an actual independent task-completion PASS or authorize READY.

## One bounded change

Preserve source provenance as a fresh, separate **Validation** Worker is created from the terminalized source Implementation Worker, and deliver the source pin to the new Core session. The handoff requires the previous Worker to be first-assignment locked, Implementation role, source-change pin, `ReadyForValidation` terminal, exact unchanged scope and a pre-existing owner-specified matching gate ledger. On success, the new Worker receives an *in-memory, revalidatable* clone of original source-gate receipts and an ephemeral source Worker thread ID. It never inherits a successful terminal, persisted PASS or permission escalation. If any prerequisite is absent, no review authority or gate ledger transfers.

Previously the TUI copied a pin only to the local display state while the new Core worker config dropped `source_root`, `source_sha` and `task_kind`. The new session now receives the same immutable pinned first-assignment fields. For an authenticated live handoff, the new Core session also selects the exact canonical source worktree as its current directory; a missing/moved target is a SETUP_BLOCKED refusal, not a fallback to the milestone launcher root. The Core handler still rechecks Validation role, selected canonical worktree, exact clean HEAD/SHA, profile presence, sandbox/approval and native exit before emitting the distinct review receipt.

The TUI native ItemCompleted observer registers a `run_codexdd_independent_validation` PASS only against the fresh Validation thread and previously authorized implementation origin. The strict 3C.1b.2a matcher checks role, scope, task kind, source root/SHA, profile, run ID, completion time, and worker separation. A later conflict or source drift invalidates it. The gate ledger keeps source/independent review references in memory; restart restores requirements pending, never persisted green flags. An accepted READY signal must explicitly cite this independently matched native receipt and pass **every** required owner gate. Generic `exec-*` PASS or a forged success label cannot do so.

## Focused regression contract

Three new `codexdd_046_3c1b2c_` TUI tests cover: (1) exact live transfer preserving pending gates but not terminal or durable review origin, (2) refusing unterminalized/foreign-scope/wrong-source transfer, and (3) source native receipt followed by separate Validation native completion matching the exact gate, with generic success references rejected. Source-preflight + Rustfmt + native Windows compile/test selector and prior three independent matcher tests are required at exact source. No full 21-stage run on this diagnostic commit.

## Important remaining work

**Not yet implemented:** owner-first-assignment creation of required gate ledgers for real tasks and safe dynamic source/operations prerequisites. This packet transfers only already-established requirements. A source Worker with no authorized ledger or pin can still reach the legacy READY_FOR_VALIDATION handoff, but its fresh Validator cannot satisfy READY; it must surface a setup blocker rather than invent gates. Also outstanding: a real separate Core/TUI/app-server/native disposable-worktree execution proving the full handoff and terminal, and 3D.1 owned listener/HTTP health proof where required.

**Safety boundaries:** No BREAKWATER/CV worktree mutation, private project source, production install, role promotion, unprotected shell execution, stale READY replay, frozen 0.4.5 import, or general release authority. Owner direction is full 0.4.6 development/release acceptance **before** a roughly seven-day single-milestone BREAKWATER soak. The soak itself is not an early-release exception.

**Next:** pass native Windows test gate, correct source/format regressions if any, then implement owner-controlled gate issuance and real disposable-host separate Worker integration before calling 3C.1 complete.
