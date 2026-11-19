# CodexDD 0.4.6 — Work packet 3B.1 source preflight: first implementation checkpoint

**Date:** 2026-11-05  
**State:** WIP / SOURCE CHECKPOINT ONLY — Windows Rust formatting, compilation, regression execution and exact-runner binding are NOT YET VERIFIED.  
**Parent:** 3A.3 at `9bbf9a9fffb342b141575a2cdfa291d53ed227ca`.  
**Do not merge, install, run CI, or claim native READY from this checkpoint.**

## Bounded change

A first external `[adaptive_worker]` assignment may now optionally supply `source_root` (absolute path), `source_sha` (40-hex Git commit) and `task_kind` (`source_change`, `operations_preview`, `benchmark`). Both source fields must be supplied together for a pinned assignment; a classified non-source task must be pinned. Unknown task kinds and malformed paths/SHA fail closed. Ordinary existing role/scope-only assignments remain backward-compatible **and still unpinned**. This must not be confused with complete source authority enforcement.

The TUI checks the pinned source before accepting a trusted Implementation `mechanical_validation` transition. It verifies the nearest actual `.git` worktree root against the explicit pin, then reads `git rev-parse --show-toplevel`, `git rev-parse HEAD`, and `git status --porcelain=v1 --untracked-files=all` without changing the worktree. Wrong parent/root, missing Git, mismatched SHA and staged/unstaged/untracked candidate produce a typed `SETUP_BLOCKED` and preserve changes. A non-source benchmark/preview cannot be misrouted into protected source MechanicalValidation; it blocks until the bounded approved operations lane exists. No new privilege or arbitrary tool route is enabled.

The parent/root launch convention `D:\bw58` stays unchanged. For an assignment pinned to a nested checkout, if the native process still points at `D:\bw58`, the correct immediate behavior in this slice is **SETUP_BLOCKED**, not an implicit CWD change or false work_packet PASS.

The same first-assignment target pin is stored in additive optional workflow snapshot fields through TUI, protocol, app-server, and history, restored on resume/fork with strict partial-pin validation, and carried into an automatically spawned independent Validation Worker only when the authorized scope is unchanged. Saved old workflow snapshots without the new fields retain their earlier behavior. Ephemeral new Worker authority remains locked after the initial assignment.

## New focused tests (authored, not executed)

- `adaptive_worker.rs`: pinned first-assignment parsing, invalid/partial authority rejection, nested real Git worktree root mismatch, positive clean nested match, wrong SHA and staged-edit preservation.
- `adaptive_effort.rs`: first binding immutable across later turns.
- `adaptive_trusted_signal.rs`: explicitly pinned benchmark refused BEFORE MechanicalValidation despite accepted complexity.
- `session_state.rs`: bound pin survives serialization/restore; partial pinned snapshot fails closed.
- `history/src/tests.rs` and `app_server_session.rs`: existing snapshot literals updated for additive nullable fields.

## Open 3B.1/3B.2 gates — not falsely complete

1. Run Rust format check, focused compilation and targeted tests on exact source-matched Daniel-CL checkout under `E:\codexdd`. No such check was run in this GitHub-only WIP.
2. Require a pinned candidate in newly migrated source-changing Worker admission where appropriate; unpinned legacy assignments currently follow old behavior. Update startup-config Worker binding as needed without expanding authorization.
3. Align native LVO `turn_environment.cwd()` and receipt HEAD with the immutable TUI-authorized target (the local TUI preflight alone is **not** an end-to-end native runner proof, especially if a remote/alternate environment is selected). Scope this to follow-up **3B.1b / 3B.2** rather than inventing a cross-host file controller.
4. Decide documented handling of staged/uncommitted candidate proof before promotion; existing edits must remain intact and SHA-only proof must not misrepresent a dirty tree.
5. Keep 3B.3–3C for prerequisite/evidence-bound terminals and 3D.1 for approved safe operations and benchmark execution. This packet does not add a service manager, process killing, model escalation or native LVO privilege to Repair/Validation.

**Release boundary:** Production remains `dd/astra-policy-v2` product 0.4.4. Frozen 0.4.5 PRs #83–85 and all prior holds remain untouched. This is not a release or a native validation receipt.
