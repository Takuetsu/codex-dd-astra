# CodexDD 0.4.6 — packet 3B.3d.2: durable prerequisite bridge

**Date:** 2026-11-11. **Parent:** `8290a2526aa5dc118bbaec60458f78f36344935c` (3B.3d.1 native acceptance documented).
**State:** IMPLEMENTATION CHECKPOINT; new exact-source Windows compile/format/regression validation required. No release or production installation.

## Outcome and scope

Wire the previously accepted source-/Worker-bound required-gate checkpoint into the existing TUI → app-server V2 → rollout-history → TUI resume path. The checkpoint stores **only** required source/owned-health/independent Validation gates and bounded setup blockers. It never stores native PASS receipts, success IDs, commands, logs, listener state, independent approval or a READY terminal.

Source owners: `tui/src/chatwidget/adaptive_effort.rs`, `tui/src/session_state.rs`, `tui/src/app_server_session.rs`, `app-server-protocol/src/protocol/v2/thread.rs`, `app-server/src/request_processors/turn_processor.rs`, `history/src/lib.rs`, `rollout/src/lib.rs`. The new regression tests exercise a synthetic checkpoint originating at the TUI snapshot, typed rollout JSON, matching/foreign Worker restoration and legacy no-checkpoint compatibility.

## Boundaries

- The TUI state may hold an optional live requirement ledger; new or unrelated Worker bindings start without inherited gate authority. Existing source and Worker pin data are left unchanged.
- The TUI serializes its ledger to a camelCase v1 checkpoint only with a bound owner thread and Worker role. No registry or successful evidence record is serialized.
- App-server protocol adds optional `gateCheckpoint` as structured JSON. The app-server validates it into the strict `codex-history` checkpoint type before persistence, bounds payload to 8 KiB, and refuses wrong thread/role/scope/source identity or unlocked Worker. Absence is backward compatible.
- The rollout JSON includes a typed optional checkpoint with required gates and blockers only. On restore, the TUI checks owner thread, Worker role, authorized scope, worktree and immutable source SHA against the saved outer assignment. Malformed or foreign checkpoints fail closed; a correctly matched checkpoint restores *pending* gates, not prior tool success.
- An old `ready_for_owner_qa` terminal attached to a restored checkpoint is suppressed, pending new verification. Legacy snapshots without a checkpoint remain backward compatible; legacy READY admission tightening is separately owned by 3C.1.

## Focused tests added (not yet run)

- `codexdd_046_3b3d2_protocol_and_rollout_restart_require_revalidation`: TUI gate payload → typed rollout snapshot → same-owner restored pending gates, empty evidence registry and no terminal authority.
- `codexdd_046_3b3d2_legacy_snapshot_remains_compatible_without_gates`: old snapshot deserializes without new field, remains distinguishable from checkpointed requirements.
- `codexdd_046_3b3d2_rejects_wrong_owner_stale_sha_and_unbound_restore`: refuses different Worker thread, no authenticated thread, or source-SHA mismatch.

## Still open / authorization limits

This source packet does **not** grant owner readiness, independent Validator acceptance, live operational health, or a new permission mechanism. Ledger creation from explicit owner task requirements and the guarded READY terminal belong to 3C.1, while live health evidence belongs to 3D.1. The original 3B.2 private BREAKWATER Node/independent live tests, other 0.4.6 phases, complete release validation, and production install remain open.

Next: run source-contract/Rustfmt and narrow native Windows 3B.3d.2 tests at exact source SHA. Fix any compiler/format errors in the owning files before calling the new persistence bridge accepted. Cheap local on Daniel-CL is the default; GitHub-only native proof needs a separately scoped diagnostic and must not be represented as live-project acceptance.
