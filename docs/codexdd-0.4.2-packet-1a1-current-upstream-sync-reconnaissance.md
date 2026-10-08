# CodexDD 0.4.2 packet 1A.1 — current upstream-sync reconnaissance

**Status:** COMPLETE — awaiting Phase 1 acceptance

**Release:** CodexDD 0.4.2

**Feature branch:** `dd/codexdd-v0.4.2-automatic-upstream-update`

**Production/base SHA:** `f45c5a119296d6e53c6429f9b101b3f52d91047e`

## Purpose

Map the existing upstream synchronization mechanism, its historical failure modes, the current version/provenance surfaces, and the validation/release boundaries before changing synchronization behavior.

This packet is reconnaissance only. It does not modify runtime routing or upstream synchronization behavior.

## Current identity anchors

Production currently records:

- CodexDD product version: `0.4.1` in `codex-rs/codexdd-version.txt`;
- tracked OpenAI Codex release: `rust-v0.159.2` in `codex-rs/upstream-codex-release.txt`;
- tracked upstream commit: `ff6aec96948b70d94983af2641a6b67c94faeff5`;
- tracked upstream tree: `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361`;
- production tree at the 0.4.1 merge: `2087ec992c52281a427091f8eed263d13567cc55`.

The product version is separately asserted by `codex-rs/cli/tests/version_reporting.rs`.

## Existing automatic sync mechanism

The repository already contains a write-capable scheduled synchronization workflow:

- `.github/workflows/upstream-sync.yml`;
- `.github/scripts/upstream_sync.py`;
- `.github/scripts/test_upstream_sync.py`.

The workflow runs on manual dispatch and on a six-hour schedule.

Its current behavior is:

1. check out `dd/astra-policy-v2`;
2. add canonical `https://github.com/openai/codex.git` as `upstream`;
3. fetch all upstream tags;
4. select the numerically highest stable `rust-vX.Y.Z` tag and ignore prereleases;
5. compare it to `codex-rs/upstream-codex-release.txt`;
6. resolve the tracked and target tags to immutable commit SHAs;
7. reject duplicates using a deterministic integration branch and PR marker;
8. build a synthetic one-commit CodexDD customization delta whose parent is the tracked upstream commit and whose tree is current production;
9. cherry-pick that synthetic delta onto the target upstream commit;
10. if the cherry-pick conflicts, abort and publish conflict details to the Actions summary, with GitHub Issue reporting only as best effort;
11. if it is textually clean, automatically increment the CodexDD patch version;
12. update `codexdd-version.txt`, `upstream-codex-release.txt`, the version-reporting test, and local Cargo.lock workspace versions;
13. create and push a deterministic integration branch;
14. immediately open a PR against production, which starts ordinary PR CI.

The workflow does not merge, install, deploy, or publish a CodexDD runtime by itself.

## Historical evolution

The current design was built incrementally:

- PR #14 introduced guarded automatic stable-upstream discovery and owner-gated integration PRs.
- PR #15 made discovery stable-only and excluded alpha/beta/rc tags.
- PR #22 replaced ordinary Git merging with the synthetic-delta transplant so squash-merged CodexDD history does not require the tracked upstream commit to remain a production ancestor.
- PR #23 proved the synthetic-delta path on the 0.155.1 integration.
- PR #29 hardened recovery after the 0.156.1 update required 19 manual overlap resolutions and GitHub Issues were unavailable. Primary conflict evidence now goes to the Actions summary.
- PR #42 integrated 0.159.2 through the newer major-phase/work-packet process instead of trusting a clean automated transplant as sufficient semantic proof.
- PR #13, although not merged, established a useful release-safety precedent: building/packaging may happen on PR/manual execution, but publication requires an explicit matching version tag.

## Current live upstream candidate

As of 2026-10-07, the latest stable official OpenAI Codex release is:

- tag: `rust-v0.160.1`;
- commit: `d27764b82f7118f674371e6d6e76271d9d606edb`;
- tree: `1055b6282cb6b0a594bad187d291f717db9dd428`;
- release commit date: 2026-10-05.

Exact recursive tree comparison, with no truncated trees, gives:

- current CodexDD customization paths versus tracked 0.159.2: **243**;
- upstream paths changed from tracked 0.159.2 to target 0.160.1: **378**;
- paths changed by both CodexDD and upstream: **48**.

The 48 overlap candidates include lifecycle, session, config, Worker/runtime, TUI, status, snapshot, Cargo.lock, and CI surfaces. Examples include:

- `codex-rs/core/src/agent/control/spawn.rs`;
- `codex-rs/core/src/session/session.rs`;
- `codex-rs/core/src/session/turn.rs`;
- `codex-rs/core/src/thread_manager.rs`;
- `codex-rs/tui/src/app.rs`;
- `codex-rs/tui/src/app/session_lifecycle.rs`;
- `codex-rs/tui/src/app/thread_routing.rs`;
- `codex-rs/tui/src/chatwidget.rs`;
- `codex-rs/tui/src/chatwidget/session_flow.rs`;
- `codex-rs/tui/src/status/card.rs`;
- `codex-rs/tui/src/status/tests.rs`.

GitHub's commit comparison reports release-history divergence between the 0.159.2 and 0.160.1 release commits. That is not itself a failure: the existing synthetic-tree model is specifically intended to avoid relying on simple ancestry. It does mean candidate identity must be defined by exact tag, commit SHA, and tree SHA rather than ancestry alone.

## Existing validation surfaces

CodexDD 0.4.0 established repository-owned Local Validation Orchestrator profiles:

- `targeted`;
- `work_packet`;
- `release`.

Their Windows contract is documented in `docs/codexdd-0.4.0-validation-profile-contract.md`.

The current upstream-sync workflow does not integrate those local receipts into its promotion flow. It creates a PR immediately after a clean textual transplant, so GitHub CI can run before the preferred Daniel-CL local validation gate.

Repo CI does run `.github/scripts/test_upstream_sync.py` through `.github/workflows/repo-checks.yml`.

## Gaps that 0.4.2 must address

### 1. Discovery and mutation are coupled

The scheduled job has repository write permissions and can progress directly from detecting a new tag to pushing a branch and opening a PR.

0.4.2 should separate read-only discovery from explicit preparation/promotion.

### 2. Product version is chosen automatically

The current workflow calls `next_patch_version(current)`.

From production 0.4.1 it would select 0.4.2 automatically, even though 0.4.2 is already allocated to this enhancement. Product release identity is a planning decision and must not be claimed by upstream discovery.

### 3. Textual transplant success is weaker than semantic compatibility

A clean synthetic cherry-pick proves only that Git can apply the tree delta without textual conflicts. It does not prove that CodexDD lifecycle, adaptive routing, persistence, generated protocol surfaces, status rendering, Worker behavior, or validation authority still work.

The 0.159.2 migration demonstrated that overlap and non-overlap paths both require semantic audit.

### 4. No durable candidate manifest

The workflow emits useful log text but does not produce a first-class manifest containing all immutable identities and exact tree-delta counts needed to reproduce or review a candidate.

### 5. No exact overlap inventory before preparation

A target can be clean under cherry-pick while still touching CodexDD-sensitive paths. Exact customization/upstream/overlap path inventories should be computed before a candidate is considered preparation-ready.

### 6. PR/CI happens before Daniel-CL validation

The current workflow immediately opens a PR after a clean transplant. This contradicts the current operating preference: cheap/local Windows validation first, then CI.

### 7. Conflict reporting still has no repository-native durable channel

GitHub Issues are disabled. The Actions summary is the primary diagnostic channel, but it is run-scoped and not a durable candidate contract.

### 8. Production movement is not a first-class stale-plan check

The workflow snapshots `PRODUCTION_SHA` early in the run. 0.4.2 should re-check that the production base still matches the candidate manifest immediately before any remote preparation write.

### 9. Generated/build surfaces are only partially special-cased

The current preparation explicitly rewrites product version, upstream provenance, version test, and local Cargo.lock package versions. Upstream integrations may also affect generated schemas, Bazel definitions, release workflows, snapshots, and other build surfaces. Those need audit classification rather than assuming the fixed rewrite set is complete.

## Packet 1A.1 acceptance criteria

Packet 1A.1 is complete because:

- the tracked product/upstream identity surfaces are pinned;
- the scheduled sync workflow and helper behavior are mapped end to end;
- the historical safety fixes and their causes are recorded;
- the current live stable candidate is resolved to exact immutable commit/tree identity;
- exact current customization, upstream-change, and overlap counts are recorded;
- the current CI/LVO relationship is identified;
- the defects and missing guarantees that 0.4.2 must address are explicit;
- no synchronization behavior or runtime behavior was changed.
