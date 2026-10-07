# CodexDD 0.4.2 — Automatic upstream-update reconciliation

## Activation state

CodexDD 0.4.2 starts from the merged CodexDD 0.4.1 production SHA:

`f45c5a119296d6e53c6429f9b101b3f52d91047e`

Feature branch:

`dd/codexdd-v0.4.2-upstream-sync-reconciliation`

Product baseline:

- CodexDD: `0.4.1`
- tracked upstream: OpenAI Codex `rust-v0.159.2`
- target platform: Windows
- authoritative local validation host: Daniel-CL

At activation time, the newest stable upstream release is:

- `rust-v0.161.0`
- upstream commit: `979011409de0a60b52f179721948e65531d26144`

The 0.4.2 feature branch does **not** silently absorb `rust-v0.161.0`. Upstream integration remains a distinct generated update candidate after this enhancement is proven.

## Problem statement

CodexDD already has a guarded scheduled upstream synchronizer in:

- `.github/workflows/upstream-sync.yml`
- `.github/scripts/upstream_sync.py`

That automation already:

- discovers newer stable official `rust-vX.Y.Z` releases;
- ignores prereleases;
- reconstructs the CodexDD customization delta from the tracked upstream tree;
- attempts to transplant that delta onto the newest stable upstream release;
- bumps the CodexDD patch version for a clean integration;
- creates a deterministic update branch and PR when the transplant is clean;
- preserves an owner-controlled merge gate;
- never installs or publishes automatically.

The remaining gap is conflict handling.

The live scheduled workflow repeatedly detects `rust-v0.161.0` but fails during the synthetic-delta cherry-pick because upstream and CodexDD both changed overlapping product paths. The current safe behavior correctly refuses blanket ours/theirs resolution, but it then leaves only transient workflow-summary diagnostics. No durable integration branch or PR is published, so the same conflict is rediscovered and retried on every schedule.

## 0.4.2 goal

Make upstream-update conflicts **durable, deduplicated, and agent-actionable** without weakening fail-closed behavior.

When the automatic transplant is clean, preserve the existing path: prepare the next CodexDD patch candidate and open the normal integration PR.

When the transplant conflicts, the updater should automatically preserve all mechanically safe work and publish a bounded reconciliation handoff rather than repeatedly dying at the same point.

## Required safety invariants

0.4.2 must preserve all of these constraints:

1. Never auto-merge `dd/astra-policy-v2`.
2. Never auto-install, publish, or deploy a CodexDD binary.
3. Never use blanket `ours` or `theirs` to resolve product conflicts.
4. Never claim a conflicted candidate is release-ready.
5. Never silently drop a CodexDD customization.
6. Never silently overwrite a changed upstream path with the old CodexDD copy.
7. Preserve exact production SHA, tracked-upstream tag/SHA, and target-upstream tag/SHA provenance.
8. Keep conflict reconciliation isolated on a deterministic automation branch.
9. Require normal local Windows validation and PR CI before merge.
10. Keep merge, install, release publication, deployment, and external-machine changes human-controlled.

## Intended conflict path

For a real conflict:

1. Detect the exact stable target and establish provenance.
2. Attempt the existing synthetic-delta transplant.
3. Capture the exact unresolved-path set.
4. Preserve the automatically merged non-conflicting portion of the CodexDD delta.
5. Restore each unresolved path to the exact target-upstream state rather than choosing an old CodexDD side.
6. Record a machine-readable and human-readable reconciliation manifest containing:
   - production SHA;
   - tracked upstream tag/SHA;
   - target upstream tag/SHA;
   - unresolved paths;
   - deterministic automation branch;
   - validation requirements.
7. Commit the partial integration state to a deterministic reconciliation branch.
8. Publish a draft reconciliation PR when repository policy permits it; otherwise leave the durable branch plus workflow summary as the primary handoff.
9. Deduplicate later schedules against the existing reconciliation branch/PR instead of repeatedly failing.
10. After an agent resolves every manifest path, remove the temporary reconciliation manifest, finalize version/provenance changes, and run the normal validation ladder.

A conflicted reconciliation branch is intentionally **not** merge-ready.

## Validation model

The enhanced updater itself is GitHub-hosted and deterministic. Product acceptance remains Windows-authoritative.

Before any generated upstream candidate can merge:

- helper/unit tests must pass;
- workflow/script structural validation must pass;
- the reconciliation manifest must be empty/removed;
- CodexDD version and upstream provenance must be internally consistent;
- Daniel-CL targeted/work-packet validation must pass as appropriate;
- Daniel-CL release validation must pass;
- normal PR CI must be green;
- owner merge approval remains required.

The Local Validation Orchestrator is the preferred mechanism for routine product validation after the reconciled candidate is capable of exercising it.

## Work packets

### 1A.1 — Existing updater and live-failure reconnaissance

Map the current updater, prove the live failure boundary, identify the exact safety invariants, and document the 0.159.2 → 0.161.0 overlap surface.

No updater behavior changes in this packet.

### 1A.2 — Reconciliation-state contract

Define the branch, manifest, draft-PR, deduplication, and incomplete-candidate semantics. Include delete/modify, add/add, rename, generated schema, binary, and lockfile cases.

### 1B — Helper and manifest implementation

Implement tested helper primitives for deterministic reconciliation metadata and conflict-state preparation.

### 1C — Workflow conflict-path integration

Teach the scheduled workflow to publish the durable reconciliation branch/handoff while preserving the clean-update path.

### 1D — Failure/deduplication hardening

Cover disabled Issues, disabled Actions PR creation, existing reconciliation branches, reruns, newer superseding stable releases, and stale reconciliation candidates.

### 1E — Source audit and release validation preparation

Audit the entire 0.4.2 change surface, verify that `rust-v0.159.2` remains the feature-branch upstream baseline, and prepare the meaningful Daniel-CL validation gate.

## Out of scope

- Automatically resolving semantic source conflicts with a blanket merge policy.
- Automatically merging production.
- Automatically installing or deploying CodexDD.
- Bundling `rust-v0.161.0` into the 0.4.2 feature branch.
- Changing adaptive model routing or the 0.4.1 GPT-6.1 Sol policy.
- Replacing the 0.4.0 Local Validation Orchestrator.

## Release sequence after 0.4.2

After 0.4.2 is merged and proven, re-run the enhanced upstream synchronizer against the then-current stable OpenAI Codex release. That generated upstream-integration candidate becomes its own independently validated release line.
