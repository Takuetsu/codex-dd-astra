# CodexDD 0.4.2 packet 1A.2 — upstream candidate eligibility policy

**Status:** COMPLETE — awaiting Phase 1 acceptance

## Objective

Define when an OpenAI Codex release may become a CodexDD upstream-update candidate and which identity checks must pass before any preparation write occurs.

This policy is intentionally stricter than "a newer tag exists."

## Canonical source

The only automatic upstream source is the official `openai/codex` repository.

Automatic discovery must not accept a tag or commit supplied by an arbitrary remote, fork, PR head, or mutable branch name.

## Release family

Eligible automatic candidates must match exactly:

`rust-v<major>.<minor>.<patch>`

The following are not automatically eligible:

- alpha;
- beta;
- rc;
- nightly/canary;
- arbitrary branches;
- untagged commits.

Prerelease parsing may remain available for diagnostics or historical comparison but must not promote a prerelease candidate.

## Monotonicity

A discovered stable release is a candidate only when its parsed numeric version is strictly greater than the currently tracked stable release.

Equal, older, malformed, or unresolvable releases produce no candidate.

Product-version numbering is separate from upstream-version ordering. Discovery must never infer a CodexDD product version.

## Immutable identity contract

A candidate manifest must record all of these values:

### Production

- production branch;
- production commit SHA;
- production tree SHA;
- CodexDD product version.

### Tracked upstream

- tracked tag from `codex-rs/upstream-codex-release.txt`;
- resolved canonical upstream commit SHA;
- resolved tree SHA.

### Target upstream

- stable target tag;
- resolved canonical upstream commit SHA;
- resolved tree SHA.

### Delta inventory

- exact CodexDD customization path count: tracked upstream tree -> production tree;
- exact upstream-change path count: tracked upstream tree -> target upstream tree;
- exact overlap path count;
- exact overlap path list or a durable artifact containing the list.

Tree enumeration must be complete. A truncated recursive tree result is a fail-closed condition.

## Ancestry policy

Release ancestry is evidence, not authority.

A newer stable release may live on a release line whose commit graph diverges from the previously tracked stable release. Candidate eligibility therefore must not require the target commit to be a descendant of the tracked commit.

Instead:

- record ancestry/merge-base diagnostics when available;
- use exact tag/commit/tree identity;
- build the CodexDD customization delta from exact trees;
- use the synthetic-delta transplant for preparation.

Unexpected ancestry remains review evidence and must be present in the manifest.

## Candidate states

The 0.4.2 planner should classify discovery into explicit states:

- `up_to_date` — no newer stable release;
- `discovered` — a newer stable release has immutable identity and inventory;
- `blocked_identity` — tag/version/SHA/tree identity cannot be proved;
- `blocked_stale_base` — production changed after the manifest snapshot;
- `blocked_duplicate` — the same target already has an active candidate/preparation;
- `blocked_transplant_conflict` — the synthetic delta does not apply cleanly;
- `preparation_ready` — identity is stable and a dry-run transplant is textually clean;
- `manual_semantic_review_required` — sensitive overlap exists even if the transplant is textually clean.

`preparation_ready` is not merge authority.

## Sensitive overlap policy

Overlap count alone must not decide compatibility.

Any overlap with CodexDD-sensitive areas must be flagged for semantic review, including:

- adaptive routing/policy/model catalog;
- Worker/Designer lifecycle and binding;
- trusted adaptive signals;
- resume/fork/persistence;
- app-server workflow state;
- terminal ordering and interruption recovery;
- Local Validation Orchestrator;
- `/status` adaptive rendering;
- generated protocol/schema bindings;
- version/provenance;
- release/build/CI surfaces.

The current 0.160.1 candidate has 48 exact overlap paths and touches multiple sensitive areas, so it is a valid discovery candidate but is not eligible for unattended promotion.

## Stale-plan protection

Immediately before any remote write, preparation must re-resolve:

- current production branch HEAD;
- tracked upstream file contents;
- target tag commit SHA.

If any differs from the manifest, preparation stops and requires a new discovery plan.

This prevents a manifest created against an old production tree or a moved/changed identity from being applied silently.

## Duplicate policy

Candidate identity is keyed by target upstream tag plus immutable target SHA.

A duplicate is detected when an active preparation record already targets the same identity.

A stale or abandoned candidate must not be silently overwritten. Recovery must be explicit.

## Product-version policy

Upstream discovery and preparation must not call `next_patch_version` and must not reserve a CodexDD product release number.

The product version is assigned by the release/work-packet process after the candidate's scope is accepted.

This directly prevents the current behavior where a scheduled update from 0.4.1 could automatically claim 0.4.2.

## Fail-closed conditions

Automatic preparation is forbidden when any of these is true:

- current tracked upstream tag is malformed;
- target tag is malformed or prerelease;
- target version is not strictly newer;
- canonical tag resolution fails;
- recursive tree enumeration is truncated;
- production/tracked/target identity changed since planning;
- duplicate candidate state is ambiguous;
- synthetic transplant reports conflicts;
- a required candidate manifest field is missing;
- a tool/infrastructure failure makes identity or delta inventory uncertain.

## Packet 1A.2 acceptance criteria

- candidate eligibility is based on canonical stable tags plus immutable commit/tree identity;
- product versioning is decoupled from discovery;
- exact tree-delta inventory is mandatory;
- divergent release ancestry is handled explicitly rather than assumed away;
- semantic-overlap review is distinct from textual-conflict detection;
- stale and ambiguous state fail closed;
- no synchronization behavior has been changed.
