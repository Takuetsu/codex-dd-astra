# CodexDD 0.4.2 packet 1A.1 — Existing updater and live-failure reconnaissance

Status: **COMPLETE**

Feature branch:

`dd/codexdd-v0.4.2-upstream-sync-reconciliation`

Activation production SHA:

`f45c5a119296d6e53c6429f9b101b3f52d91047e`

Tracked upstream:

`rust-v0.159.2` @ `ff6aec96948b70d94983af2641a6b67c94faeff5`

Newest stable upstream observed at activation:

`rust-v0.161.0` @ `979011409de0a60b52f179721948e65531d26144`

## Purpose

Packet 1A.1 maps the updater that already exists, establishes the current failure boundary using live GitHub Actions evidence, and identifies the minimum safe change surface for 0.4.2.

No updater behavior is changed in this packet.

## Existing automation

Authoritative files:

- `.github/workflows/upstream-sync.yml`
- `.github/scripts/upstream_sync.py`
- `.github/scripts/test_upstream_sync.py`
- `codex-rs/upstream-codex-release.txt`
- `codex-rs/codexdd-version.txt`

Current scheduler:

- manual `workflow_dispatch`;
- scheduled approximately every six hours;
- concurrency serialized as `upstream-codex-sync`.

Current clean-path behavior:

1. Fetch official upstream tags.
2. Select the highest stable `rust-vX.Y.Z` tag and reject prereleases.
3. Compare it with the tracked upstream release file.
4. Deduplicate by deterministic update branch / PR marker.
5. Reconstruct the complete CodexDD product tree as a synthetic delta rooted at the tracked upstream commit.
6. Cherry-pick that synthetic delta onto the exact target stable release.
7. Bump the CodexDD patch version.
8. Update tracked-upstream provenance.
9. Reconcile local workspace package versions in `Cargo.lock`.
10. Create a deterministic integration branch and open a PR.
11. Leave normal CI and owner merge approval as required gates.

This design is intentionally independent of upstream ancestry surviving prior CodexDD squash merges.

## Current conflict behavior

When the synthetic delta does not apply cleanly:

- the workflow captures unmerged paths and cherry-pick diagnostics;
- the workflow writes the primary details to the GitHub Actions job summary;
- it attempts secondary GitHub Issue reporting;
- repository Issues are disabled, so Issue creation fails;
- it aborts the cherry-pick;
- it publishes no branch;
- it publishes no PR;
- the workflow exits failed.

This is fail-closed and does not corrupt production, but it produces no durable reconciliation state.

## Live failure evidence

Recent scheduled `upstream-codex-sync` runs have repeatedly failed at the same step:

`Transplant codexdd delta onto exact upstream release`

Example run:

- workflow run: `37628532353`
- run number: `71`
- event: scheduled
- production SHA used by that run: `ee105d4d81a68cd6368700865c5a67e218e3df2d`
- tracked upstream: `rust-v0.159.2`
- target upstream: `rust-v0.161.0`
- target SHA: `979011409de0a60b52f179721948e65531d26144`
- product delta reported by the workflow at that production revision: 225 paths
- outcome: cherry-pick conflict, no integration branch or PR
- secondary Issue reporting: unavailable because Issues are disabled

Multiple earlier scheduled runs show the same failure conclusion rather than a transient infrastructure failure.

## 0.4.1 production overlap reconnaissance

A tree-level comparison using the actual 0.4.1 production SHA shows:

- CodexDD-vs-`0.159.2` changed files: **243**
- `0.161.0`-vs-`0.159.2` changed files: **1408**
- file paths changed by both sides: **107**

The 107 paths are an **overlap surface**, not a claim that all 107 become Git conflicts. They identify the set where automatic carry-forward requires a three-way merge or explicit reconciliation.

The overlap spans consequential surfaces including:

- app-server protocol and generated schemas;
- app-server request/notification/session code;
- CLI and daemon startup;
- core session/thread/config/tool plumbing;
- history/rollout/thread-store behavior;
- large portions of the TUI session/chat/status lifecycle;
- Windows daemon/backend code;
- `Cargo.lock`;
- generated Python SDK protocol output.

This is large enough that repeatedly attempting one monolithic synthetic cherry-pick is expected to continue producing real conflicts as upstream evolves.

## What is already correct and must remain unchanged

The following behavior is not the 0.4.2 problem and should be preserved:

- stable-release selection;
- prerelease rejection;
- tracked-upstream provenance;
- deterministic branch/marker naming;
- duplicate protection;
- synthetic-tree strategy that survives squash history;
- no automatic production merge;
- no automatic binary install/publish/deploy;
- no blanket conflict-resolution policy;
- clean-path version bump and lockfile reconciliation.

## Failure boundary

The automation gap begins **after** Git has proven that some paths cannot be mechanically reconciled.

The updater currently has no durable state between:

`cherry-pick conflict detected`

and

`human/agent manually reconstructs the entire integration`.

0.4.2 should fill only that gap.

## Required 1A.2 contract questions

Packet 1A.2 must specify:

1. What exact tree is published when some files conflict?
2. How are conflict paths represented without committing conflict markers?
3. How are add/add, modify/delete, delete/modify, rename, generated, binary, and lockfile conflicts represented?
4. What makes the branch obviously incomplete and non-mergeable?
5. How does a later agent prove every omitted CodexDD customization has been reconciled?
6. How do scheduled reruns deduplicate against the incomplete candidate?
7. What happens if a newer stable upstream release appears before reconciliation finishes?
8. How does finalization transition the draft/incomplete candidate back onto the normal clean validation path?
9. How is Daniel-CL LVO evidence recorded without making GitHub Actions authoritative for Windows acceptance?
10. What fallback remains when GitHub Actions is not permitted to create PRs?

## 1A.1 acceptance

Packet 1A.1 is accepted when the implementation phase can proceed without changing any of the already-correct clean-path behavior and without weakening fail-closed semantics.

Reconnaissance result: **accepted for 1A.2 design**.
