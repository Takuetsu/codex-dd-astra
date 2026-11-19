# CodexDD 0.4.6 — 3B.2i-b explicit owner-origin candidate transition (WIP)

**Parent:** `7974a3ac4ab0f8cccbfd6b8190b04c4474c9027e`, read-only 3B.2i-a verifier checkpoint. **Not native-validated.**
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`.
**Status:** nonproduction source checkpoint pending Rustfmt, Windows compile and focused tests; no release or owner QA.

## New behavior
- `[codexdd_candidate]` remains a standalone text-only instruction with explicit previous and candidate SHAs, no additional user items, and a nonempty body.
- Core restricts the proposal to direct user-origin root-session turn input; `StartOrSteer` proposals are routed into the idle-only `StartIfIdle` path. An active turn refuses `NotIdle`; an explicit `Steer` request refuses `SETUP_BLOCKED`. Internal/delegated sessions, non-user triggers and mixed settings changes cannot authorize candidate sources.
- Core verifies the already checkpointed first-user Implementation Worker and a source-change task, original source SHA, canonical same Git worktree, clean status, exact new HEAD and descendant ancestry. A missing native store, mismatched SHA or dirty tree is a typed setup blocker, not a PASS/repair or automatic fallback.
- Core appends the prospective `ThreadSettingsApplied` snapshot with the *same thread ID*, flushes it through the existing shared persistence permit, then replaces **only** the in-memory Worker source SHA. A failed append/flush leaves the prior runtime credential intact. Interruption after flush can recover from the new native settings snapshot on same-thread resume; fork still must not inherit the parent's authority.
- Startup-configured, Validation and Repair Workers cannot switch through this first-user credential path. Role, scope, root and task kind are preserved. No change is inferred from a normal Git commit, model message, tool result, file mutation or source HEAD drift.

## Scoped new regressions
- Four `codexdd_046_3b2i_*` parser/verifier/commit unit tests: strict owner instruction grammar, positive clean descendant, negative unbound/wrong-SHA/dirty cases, and prospective snapshot scope immutability at commit.
- Three `codexdd_046_3b2j_*` Core turn-input tests: standalone exact candidate message, unbound candidate cannot consume the idle turn, and candidate during active work cannot steer or mutate authority.
- Existing first Worker checkpoint, role/tool, cold-resume and full Core/TUI compilation checks are retained by the branch-scoped workflow.

## Outstanding safety/acceptance
The initial first-message Worker turn context still cannot retroactively acquire its first-user pin. The TUI's separate original candidate pin must be reconciled with the Core-approved successor before protected MechanicalValidation succeeds end to end. Real Node and Rust work-packet profiles, native same-thread cold resume, full release profiles, independently bound QA and Daniel-CL local validation are **not** claimed by these source tests. Validation must establish Rustfmt and compiler green on the exact new SHA; any failure is to be repaired at a new checkpoint. No 0.4.5 thaw, default/production merge, release, installation or game worktree modification.
