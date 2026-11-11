# CodexDD 0.4.6 — packet 3B.3d.1: non-authoritative gate checkpoint

**Date:** 2026-11-11. **Parent:** `b65a32fa5c055a5266ceb952c46e2153eb9e0a2f` (3B.3c.1 accepted native evidence documented).
**State:** SOURCE CHECKPOINT; native Windows validation and app-server/rollout integration not yet completed. Production 0.4.4 unchanged, 0.4.5 frozen.

## Outcome / authority / scope

Add a compact, typed, serializable checkpoint to the 3B.3c prerequisite ledger, plus restart and rejection tests. Its purpose is to allow the later persisted handoff to retain **which requirements are pending or blocked**, without silently inheriting successful tool evidence or reviewer authority. The owner-authorized worktree, exact SHA, scope, receiving Worker role and receiving Worker thread must all match on restore.

Owned source: `codex-rs/tui/src/adaptive_gate_prerequisites.rs` and `codex-rs/tui/src/adaptive_evidence_tests.rs`. This packet does **not** change active TUI terminal admission, perform a real service/preview, execute private BREAKWATER/CV tasks, add privileges, or merge/install/publish any runtime. Scope excludes the app-server/rollout protocol bridge (next 3B.3d.2), independent Validator evidence production (3C.1) and owned healthy listener proof (3D.1).

## Checkpoint v1 invariants

- Stores schema version, exact owner Worker thread ID and role, authorized scope, canonical source root and 40-character source SHA, a bounded explicit required-gate list, and bounded typed blocker codes.
- **Never serializes `source_refs`, evidence IDs, native run IDs, command text, raw logs, completion success, listener-health success or a READY terminal.** A restored ledger has no revalidated source proof, even if the old in-memory ledger was green.
- Rejects missing/duplicate Independent Validation, invalid or duplicate profiles/ports, unbounded requirements, unsupported checkpoint schema, cross-worker thread/role, scope change, previous source SHA, foreign/inaccessible root, and duplicate/invalid blockers. All unsupported or untrusted inputs fail closed rather than generate PASS.
- Exact Worker rehydration is not permission to adopt another Worker's evidence; separate independent handoff must be explicitly authorized and verified later.
- Current 3B.3c model has no production use yet. This codec is a **foundation only**; runtime protection remains pending until ledger is integrated with snapshots and terminal checks.

## Synthetic regression tests added, not yet accepted

- `codexdd_046_3b3d_checkpoint_roundtrip_drops_all_success_authority`: serializes/deserializes required gates, excludes evidence/run identifiers, and changes prior source PASS to pending on restore.
- `codexdd_046_3b3d_rejects_cross_worker_prior_sha_and_wrong_worktree`: mismatched Worker thread/role, scope, worktree or SHA rejected.
- `codexdd_046_3b3d_restores_only_valid_pending_blockers_and_fails_closed`: preserves setup blockers, rejects invalid version, missing independent gate, duplicate blocker and unexpected JSON authority.

## Proof and handoff

The previous [3B.3c.1 Windows gate #43070130453](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43070130453) is green at `9b159f54e7b5582f72b95db8e3434322dc73593b`: native 1 Core + 6 3B.3b + 4 3B.3c tests. It is not proof for this new changed source.

Next: verify exact source checkpoint; check Rustfmt/source contract, then run the three new focused Windows tests and relevant earlier controls. Repair any compile or formatter problem narrowly. After that begin **3B.3d.2** app-server/rollout persistence and backward compatibility, then **3C.1** terminal admission with actual independent proof. Do not mistake source tests for full gate or release acceptance.
