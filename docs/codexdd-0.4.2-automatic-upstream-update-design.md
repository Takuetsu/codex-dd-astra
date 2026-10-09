# CodexDD 0.4.2 automatic upstream-update orchestration

> **Historical release design:** CodexDD 0.4.2 was completed and merged via PR #47 at `8336ca046af71cbf032c76d933ca371336f92673`. The phase statuses below reflect the original work checkpoints rather than the current production state. The **next queued release is 0.4.3**, integrating OpenAI Codex `rust-v0.162.0` under `docs/codexdd-0.4.3-upstream-0.162.0-design.md`.

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

**Status: COMPLETE — accepted.**

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

## Phase 2 — implementation

**Status: SOURCE COMPLETE — DANIEL-CL VALIDATION PENDING.**

Completed packets:

- **2A.1 — candidate manifest and planner helpers:** immutable identity, exact Git tree deltas, overlap classification, stale-base checks, deterministic states, receipt validation, and regression coverage.
- **2A.2 — read-only discovery workflow:** six-hour/manual discovery now has read-only repository permission and produces manifest/dry-run evidence only.
- **2A.3 — explicit preparation contract:** manual identity-bound preparation can publish only an isolated deterministic candidate branch after stale-state rechecks; no product-version allocation and no automatic PR.
- **2B.1 — validation/promotion handoff:** Windows candidate validation emits a native HEAD/manifest-bound receipt; PR promotion requires that receipt and re-verifies all identities.
- **2B.2 — recovery hardening:** legacy auto-version/issue/auto-PR helpers are removed; duplicate, moved-base, interrupted, and rerun behavior fail closed.

Durable implementation record:

- `docs/codexdd-0.4.2-phase-2-implementation.md`

The source identity is now `0.4.2`. The tracked OpenAI Codex upstream remains `rust-v0.159.2`; the observed `rust-v0.160.1` candidate is not integrated by this release.

## Phase 3 / 4

Next gates are:

1. cheap Daniel-CL helper/identity validation;
2. repository-owned work-packet profile;
3. pre-validation source audit;
4. broad Windows release profile;
5. PR/CI;
6. explicit merge/install/soak.

## Validation philosophy

Use the existing LVO and repository-owned validation profiles wherever applicable. Upstream-update-specific validation should be additive and bounded, not a parallel ad hoc test framework.

## Completed release / successor handoff (2026-10-09)

CodexDD 0.4.2 merged and shipped the safe upstream-update orchestration contract. Production subsequently received the elevated Windows SSH F2 hotfix in PR #48 (`dc10de2b0240ebe51bba009bfa224fc305c9aa6a`), which was installed and verified on Daniel-CL. The tracked Codex upstream remains `rust-v0.159.2`.

**Next item: CodexDD 0.4.3 — integrate the owner-selected stable upstream `rust-v0.162.0`.**

- [0.4.3 integration design](codexdd-0.4.3-upstream-0.162.0-design.md)
- [Current roadmap](codexdd-roadmap.md)
- Activation phrase: `activate 0.4.3` in a new chat.
- Start with read-only Phase 1 / packet 1A.1, accepting reconnaissance before any source modification or candidate preparation.
- Reuse 0.4.2 read-only discovery, explicit preparation, native validation receipt, and manual promotion gates. No unattended merge/install or automatic CodexDD product version bump.

This paragraph supersedes the older "next gates" wording in the historical 0.4.2 phase record.
