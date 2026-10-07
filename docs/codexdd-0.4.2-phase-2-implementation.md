# CodexDD 0.4.2 Phase 2 implementation record

**Status:** SOURCE COMPLETE — DANIEL-CL VALIDATION PENDING

**Release:** CodexDD 0.4.2

**Feature branch:** `dd/codexdd-v0.4.2-automatic-upstream-update`

**Base production:** `f45c5a119296d6e53c6429f9b101b3f52d91047e` (CodexDD 0.4.1)

## Scope completed

### 2A.1 — candidate manifest and planner helpers

Implemented in `.github/scripts/upstream_sync.py` with regression coverage in `.github/scripts/test_upstream_sync.py`.

The helper layer now provides:

- immutable candidate keys bound to stable tag + 40-character commit SHA;
- deterministic candidate branch names bound to target tag + SHA prefix;
- exact recursive Git tree-entry inventories including mode/type/object identity;
- exact CodexDD customization, upstream-change, and overlap path sets;
- sensitive-overlap classification across adaptive routing, Worker lifecycle, persistence/resume/fork, LVO, status, generated protocol, provenance, and build/release/CI surfaces;
- deterministic candidate states for discovery, textual conflict, semantic review, and preparation readiness;
- immutable JSON candidate manifests;
- stale production/tracked/target identity checks;
- candidate-bound local validation receipt parsing and validation.

The planner records the current CodexDD product version only. It does not infer or allocate a next CodexDD release number.

### 2A.2 — read-only discovery workflow

`.github/workflows/upstream-sync.yml` is now a read-only discovery workflow.

Scheduled/manual discovery:

- retains the six-hour cadence;
- has only `contents: read` permission;
- fetches canonical OpenAI Codex tags;
- selects only a newer stable `rust-vX.Y.Z` release;
- resolves tracked/target commit and tree identity;
- records ancestry diagnostics without relying on ancestry as authority;
- computes exact tree deltas and sensitive overlap evidence;
- performs an ephemeral synthetic-delta transplant dry run;
- writes a machine-readable candidate manifest and conflict evidence;
- uploads candidate evidence as an Actions artifact;
- writes a bounded Actions summary.

It cannot:

- bump the CodexDD product version;
- push a branch;
- open/close an issue;
- open a PR;
- merge;
- install/deploy/publish.

### 2A.3 — explicit preparation workflow

Added `.github/workflows/upstream-prepare.yml`.

Preparation is manual `workflow_dispatch` and requires the accepted:

- production SHA;
- tracked upstream tag + SHA;
- target upstream tag + SHA.

Before any remote write it verifies all identities, rejects duplicate deterministic candidate branches, reconstructs the squash-safe synthetic CodexDD delta, and performs an isolated transplant.

A clean preparation may:

- update candidate upstream provenance;
- reconcile local Cargo.lock workspace-package versions;
- commit the candidate manifest;
- publish exactly one deterministic candidate branch.

It does not allocate a CodexDD product version and does not open a PR.

Immediately before the push it re-fetches production and upstream tags and fails closed if either moved.

A textual transplant conflict publishes no candidate branch.

### 2B.1 — Daniel-CL validation and PR promotion handoff

Added `scripts/codexdd-validate-upstream-candidate.ps1`.

The Windows helper:

- requires a clean named candidate branch;
- locates the exact committed manifest matching the candidate tag/SHA;
- verifies target provenance and production ancestry;
- runs the repository-owned `work-packet` or `release` validation profile;
- parses the native structured `profile_end` PASS event;
- binds the receipt to the exact candidate HEAD;
- binds the receipt to the committed manifest SHA-256;
- writes the compact receipt outside the source worktree;
- emits `CODEXDD_UPSTREAM_VALIDATION_RECEIPT <json>`.

Added `.github/workflows/upstream-promote.yml`.

Promotion is manual `workflow_dispatch` and requires the native receipt. It re-verifies:

- receipt structure and PASS status;
- candidate branch name and exact HEAD;
- current production still equals the receipt base;
- candidate ancestry;
- committed manifest SHA-256;
- manifest candidate key / target / production identity;
- candidate tracked-upstream provenance;
- duplicate open-PR state.

Only after those checks may it open the normal owner-gated integration PR.

The workflow has `contents: read` + `pull-requests: write`; it cannot merge.

### 2B.2 — recovery hardening and obsolete-path removal

The previous scheduled write-capable mechanism has been removed from the live path.

Removed legacy behavior includes:

- automatic `next_patch_version` allocation;
- automatic version-reporting rewrites during upstream discovery;
- old `automation/upstream-sync-<tag>` integration branch helpers;
- old PR/conflict marker helpers;
- GitHub Issue conflict reporting.

The live workflows have no GitHub Issues dependency.

Deterministic recovery behavior is now:

- discovery reruns are read-only;
- accepted candidate identity deterministically maps to one candidate branch;
- an existing candidate branch fails closed instead of being silently overwritten;
- a candidate appearing during preparation is detected before push;
- production or target-tag movement invalidates preparation/promotion;
- interrupted preparation before push leaves no remote candidate;
- a partially successful push is visible as an existing deterministic branch on rerun;
- PR promotion refuses a moved branch or stale production base.

Repo-check regressions assert the permission boundaries and forbid reintroduction of automatic version allocation, issue writes, automatic PR creation from discovery/preparation, or PR merge commands.

## Product identity

The feature branch now reports CodexDD `0.4.2`.

- `codex-rs/codexdd-version.txt`: `0.4.2`
- `codex-rs/cli/tests/version_reporting.rs`: expects `codexdd 0.4.2+g...`

This explicit release bump is separate from the upstream-candidate planner. Future upstream discovery cannot claim a CodexDD release number.

## Current upstream candidate remains unintegrated

Phase 1 observed OpenAI Codex `rust-v0.160.1` as the current stable candidate.

0.4.2 does **not** integrate 0.160.1. The candidate is used only as design/reconnaissance evidence. Production and the 0.4.2 feature branch continue to track OpenAI Codex `rust-v0.159.2`.

## Validation gates

### Gate 2V.1 — cheap helper / identity validation

Run on Daniel-CL before the broader work-packet profile:

1. branch diff whitespace check;
2. upstream-sync Python regression suite;
3. CodexDD version-reporting regression.

### Gate 2V.2 — work-packet

After 2V.1 passes, run the repository-owned `work-packet` profile.

### Gate 2V.3 — release / CI

Only after the source audit and work-packet are green:

- run the broad Windows release profile;
- promote to PR/CI;
- merge/install/soak through normal human-controlled gates.

No GitHub CI is required before Gate 2V.1.
