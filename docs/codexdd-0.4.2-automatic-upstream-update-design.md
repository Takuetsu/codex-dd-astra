# CodexDD 0.4.2 automatic upstream-update orchestration

## Release identity

- Release: CodexDD 0.4.2
- Feature branch: `dd/codexdd-v0.4.2-automatic-upstream-update`
- Base production SHA: `f45c5a119296d6e53c6429f9b101b3f52d91047e`
- Starting tracked upstream: OpenAI Codex `0.159.2`
- Target platform: Windows
- Authoritative validation host: Daniel-CL

## Objective

CodexDD 0.4.2 will make upstream refreshes substantially less manual without turning them into unattended production changes.

The intended end state is a CodexDD-owned workflow that can discover a newer eligible upstream, inspect compatibility-relevant surfaces, create a bounded update plan, prepare an isolated candidate branch/change set, and run the appropriate mechanical evidence path. Promotion remains explicit and human-controlled.

## Safety contract

0.4.2 must preserve these invariants:

1. Discovery is read-only until an explicit preparation action is taken.
2. A candidate upstream is identified by both version/tag and immutable commit SHA.
3. Ambiguous or moving upstream identity fails closed.
4. Preparation occurs on an isolated feature/update branch, never directly on production.
5. Unrelated CodexDD feature changes are not bundled into an upstream refresh.
6. Mechanical validation cannot substitute for Windows-authoritative release validation.
7. Merge, install, release publication, deployment, and external-machine changes remain human-controlled.
8. Resume/fork or interrupted execution cannot accidentally promote a partially prepared update.
9. The workflow produces durable evidence sufficient to explain what upstream changed, what CodexDD surfaces were touched, and which validation gates passed or failed.

## Phase 1 — reconnaissance and contract

**Status: COMPLETE — awaiting acceptance before implementation.**

Phase 1 deliverables:

- `docs/codexdd-0.4.2-packet-1a1-current-upstream-sync-reconnaissance.md`
- `docs/codexdd-0.4.2-packet-1a2-candidate-eligibility-policy.md`
- `docs/codexdd-0.4.2-packet-1a3-automation-boundary-map.md`

Key finding: the repository already contains a six-hour scheduled, write-capable upstream sync. 0.4.2 will harden that mechanism by splitting read-only discovery from explicit preparation/promotion, removing automatic product-version allocation, adding immutable candidate manifests and exact tree-overlap evidence, and restoring Daniel-CL local validation before PR/CI promotion.

The current live stable candidate observed during reconnaissance is `rust-v0.160.1` at `d27764b82f7118f674371e6d6e76271d9d606edb`. Exact tree comparison against tracked `rust-v0.159.2` found 243 CodexDD customization paths, 378 upstream-changed paths, and 48 overlap candidates.


### Packet 1A.1 — current upstream-sync surface

Inventory the existing repository mechanisms for:

- tracked upstream version/SHA;
- remotes, tags, or scripts used for upstream discovery;
- branch and PR naming from prior upstream integrations;
- source/version identity;
- generated files and lockfiles;
- CI and LVO validation paths;
- prior update/release automation experiments;
- rollback and failure handling.

Deliverable: a repository-backed reconnaissance report with a complete change-surface map and explicit acceptance criteria.

### Packet 1A.2 — candidate eligibility policy

Define:

- which upstream release/tag families are eligible;
- stable versus prerelease handling;
- version monotonicity rules;
- immutable SHA resolution;
- minimum compatibility checks before preparation;
- behavior when the fork is ahead, behind, diverged, or dirty;
- fail-closed conditions.

### Packet 1A.3 — automation boundary map

Separate the workflow into:

- safe read-only automation;
- safe isolated preparation;
- mechanical validation automation;
- actions that require explicit operator promotion.

No implementation begins until Phase 1 is accepted.

## Implementation plan after Phase 1 acceptance

- **2A.1 — candidate manifest and planner helpers:** immutable identity, exact tree deltas, overlap classification, stale-base checks, deterministic states, tests, and removal of product-version selection from planning.
- **2A.2 — read-only discovery workflow:** scheduled/manual discovery with read-only permissions and machine-readable manifest evidence; no branch, PR, or version mutation.
- **2A.3 — explicit preparation contract:** identity-bound preparation, synthetic transplant dry run, deterministic candidate branch on a clean preparation path, fail-closed conflict evidence, and no automatic PR.
- **2B.1 — validation/promotion handoff:** Daniel-CL/LVO evidence before PR promotion.
- **2B.2 — recovery hardening:** stale-base, duplicate/abandoned candidate, Issues-disabled, interrupted preparation, and deterministic rerun behavior.
- **Phase 3/4:** source audit, Windows validation, CI, merge, install, and soak under the normal CodexDD release gates.

## Validation philosophy

Use the existing LVO and repository-owned validation profiles wherever applicable. Upstream-update-specific validation should be additive and bounded, not a parallel ad hoc test framework.
