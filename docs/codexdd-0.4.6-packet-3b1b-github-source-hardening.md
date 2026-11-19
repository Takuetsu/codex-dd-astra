# CodexDD 0.4.6 — 3B.1b GitHub source hardening checkpoint

**Date:** 2026-11-05  
**Parent:** `ae7ae30d622eaafc77df68d30b1245f5ebbcafb1` (3B.1a WIP)  
**State:** SOURCE CHECKPOINT ONLY / NOT COMPILED / NOT TESTED ON WINDOWS / NOT CI-GREEN.

This is a bounded continuation of 3B.1 using **GitHub source editing only**, per owner instruction. Production `dd/astra-policy-v2` remains 0.4.4. Frozen 0.4.5 and unrelated BREAKWATER/CV source/worktrees remain unchanged.

## Fixed in source

1. **Configured Worker source binding**: launcher `[adaptive_worker]` settings may now supply optional `source_root`, `source_sha` and `task_kind`, matching the original first-message assignment parser. A malformed pin or classified non-source task missing a pin is rejected before the TUI sends `thread/start`, avoiding a half-created Worker. Existing startup configurations without these optional fields remain readable.
2. **Configured authority lock**: startup-bound real roles/scopes are explicitly locked against a later contradictory first-message header. An intentionally unspecified and unscoped startup configuration stays eligible for the first header, avoiding an accidental permission regression.
3. **Durable restoration**: a persisted pinned source now requires a locked, named Worker role and nonempty authorized scope. A saved non-source benchmark/operations task cannot be restored *inside* source-only MechanicalValidation. This prevents malformed or incomplete restored source authority from slipping past the initial binding contract.
4. **Model/effort state reconstruction**: `AdaptiveEffortState::activate_adaptive_family` now carries its `assignment_preflight` field into the rebuilt state. The 3B.1a state reconstruction missed this field; without the fix it would fail to compile and fail to preserve the authorized target across adaptive route changes.
5. **Test fixture formatting**: repaired seven unindented `worker_assignment_locked` source snapshot lines introduced by 3B.1a.

## Targeted regression coverage authored, not executed

- Config TOML backward compatibility and new optional pinned startup fields.
- Startup pin accepts a properly scoped role and rejects a missing SHA, unsupported role/task or empty scope.
- Persisted pins refuse unlocked or inconsistent task/phase snapshots, preserving older unpinned snapshot compatibility.
- Pinned first-message source survives adaptive startup and a model/effort route transition without becoming mutable.
- Previous 3B.1a nested root, wrong SHA, staged changes and benchmark-not-mechanical cases remain in source.

## Important remaining holds

- This GitHub commit does **not** certify successful Rust format/compilation or execution of any regression. No GitHub Actions or Daniel-CL run was initiated; source review/static wiring checks are not native validation.
- Legacy Worker assignments with no explicit pin can still rely on older behavior. Do not call missing target authority fixed for every installed agent.
- TUI pre-admission checking `config.cwd` is not proof that the core native `run_codexdd_validation` handler will use the same authorized target: core currently uses its own `turn_environment.cwd()`/nearest `.git`. An exact target/receipt enforcement path must be established before 3B.1 and 3B.2 can be called release-ready.
- A source-changing Implementation Worker can change candidate HEAD; the start pin and independently validated post-implementation SHA must be reconciled explicitly, not silently mutated or re-used as fresh validator proof.
- No privileged operations, process manager, role bypass, broad workflow framework, 0.4.5 changes, CI promotion, merge, install or version bump are authorized.

**Next packet:** 3B.1c / exact native target and evolving candidate boundary, plus focused executable negative/positive tests before accepting 3B.1. Continue GitHub-side source work only until owner explicitly changes validation instructions.
