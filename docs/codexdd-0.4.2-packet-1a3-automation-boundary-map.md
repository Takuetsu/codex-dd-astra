# CodexDD 0.4.2 packet 1A.3 — automation boundary map

**Status:** COMPLETE — awaiting Phase 1 acceptance

## Objective

Define which parts of an upstream refresh CodexDD may automate and which actions remain explicit operator promotion gates.

The boundary is designed around the current working model: local/cheap evidence before CI, GitHub-side preparation owned by the assistant/automation, and human control over consequential release actions.

## Boundary map

| Stage | Trigger | Automatic actions allowed | Remote writes | Human gate |
| --- | --- | --- | --- | --- |
| Scheduled discovery | schedule | fetch canonical tags, select newest stable, resolve immutable identity, compute exact tree deltas/overlap, produce summary/artifact | none | no |
| Manual discovery | workflow dispatch / local helper | same as scheduled discovery | none | no |
| Candidate acceptance | explicit operator action | re-resolve manifest identities and stale-base checks | none | **yes** |
| Isolated preparation | accepted candidate | dry-run synthetic transplant; if clean, construct deterministic candidate tree/branch and candidate manifest | candidate branch only | accepted candidate required |
| Conflict path | accepted candidate | collect exact conflicts and reproducible recovery data | no production write; no PR | manual reconciliation required |
| Local validation | prepared branch | run repo-owned LVO profiles on Daniel-CL; capture native receipts | build/test artifacts only | operator initiates/observes gate |
| PR promotion | current local evidence passes | open PR against production with immutable manifest and validation evidence | PR only | **yes** |
| CI | promoted PR | normal required checks | CI artifacts/status only | merge remains gated |
| Merge | green PR | none automatically | production merge | **yes** |
| Install/deploy | merged production | build/install commands only when explicitly requested | local/external runtime change | **yes** |
| Release publication | explicit matching release/tag process | package and publish after tag/version checks | GitHub Release/artifacts | **yes** |

## Scheduled discovery must become read-only

The current six-hour scheduled workflow is too powerful for the modern CodexDD process because it can push a branch, bump product version, and open a PR.

0.4.2 should retain automatic detection but change scheduled execution into a read-only planner.

Scheduled discovery may:

- fetch tags;
- identify the newest eligible stable;
- resolve tag/SHA/tree identity;
- calculate exact path inventories;
- classify sensitive overlaps;
- perform a no-write transplant dry run if practical;
- emit a bounded Actions summary and downloadable manifest artifact.

It must not:

- bump `codexdd-version.txt`;
- update production;
- push an integration branch;
- open a PR;
- close/create issues as a required path;
- run installation or deployment.

## Explicit candidate preparation

Preparation is the first write-capable stage.

It must be explicitly promoted from a specific manifest identity and re-check stale state before writing.

If the synthetic transplant is clean, preparation may create a deterministic candidate branch. The branch should carry:

- the exact target upstream tree plus the preserved CodexDD delta;
- updated tracked-upstream provenance appropriate to the candidate;
- a durable candidate manifest;
- only deterministic preparation repairs required by the contract.

Preparation must not assign the next CodexDD product release number.

If the transplant conflicts, automation must not use broad ours/theirs resolution. The workflow stops with exact unmerged paths and reproduction instructions. Semantic reconciliation becomes a numbered work packet.

## Local validation before PR/CI

A prepared candidate should not automatically open a PR.

The preferred sequence is:

1. prepare isolated candidate branch;
2. synchronize/fetch that branch on Daniel-CL;
3. run the bounded targeted/work-packet validation appropriate to the candidate;
4. perform any required semantic reconciliation packets;
5. run the broader local release gate when the candidate is ready;
6. only then explicitly promote to a PR and CI.

This restores the current CodexDD standard of local evidence before expensive GitHub CI.

## LVO relationship

0.4.2 should reuse the existing Local Validation Orchestrator rather than invent a second validation framework.

The upstream planner/preparer owns identity and source preparation.

LVO owns deterministic local test/build evidence.

A future promotion command/workflow may verify recorded evidence identity, but must not fabricate or infer a PASS from prose.

## Conflict and semantic-review behavior

Textual conflict and semantic risk are separate:

- textual conflict: synthetic transplant cannot apply; stop before candidate source branch publication unless an explicit manual-reconciliation packet is created;
- semantic risk: transplant is clean but overlaps sensitive CodexDD surfaces; branch may be prepared, but promotion remains blocked until those surfaces are audited and locally validated.

No automatic model or script is authorized to blanket-resolve sensitive overlaps.

## Durable evidence

Each candidate should have one machine-readable manifest with at least:

- contract version;
- discovery timestamp;
- production SHA/tree;
- CodexDD product version;
- tracked upstream tag/SHA/tree;
- target tag/SHA/tree;
- ancestry diagnostic;
- customization path count;
- upstream-change path count;
- overlap count;
- sensitive-overlap count/categories;
- transplant dry-run result;
- candidate state;
- deterministic candidate key.

A human-readable summary may be generated from the same data.

## Reusable lesson from PR #13

The unmerged Windows packaging experiment used a strong safety split:

- ordinary PR/manual runs could build and package;
- publication required an explicit matching `codexdd-vX.Y.Z` tag;
- tag/version mismatch failed closed.

0.4.2 should preserve the same pattern: automate reversible preparation, require explicit identity-bound promotion for consequential publication/merge/install steps.

## Implementation packets after Phase 1 acceptance

### 2A.1 — candidate manifest and planner helpers

- refactor upstream helper code around an immutable candidate manifest;
- add exact stable-tag, SHA/tree, delta-count, overlap, stale-base, and state helpers;
- add deterministic unit tests;
- remove product-version selection from discovery/planning.

### 2A.2 — read-only discovery workflow

- split scheduled discovery from write-capable preparation;
- reduce scheduled permissions to read-only;
- emit bounded summary + machine-readable manifest artifact;
- prove no branch/PR/version mutation occurs in discovery mode.

### 2A.3 — explicit preparation contract

- add an explicit, identity-bound preparation entrypoint;
- re-resolve manifest identity immediately before writes;
- dry-run the synthetic transplant;
- create a deterministic candidate branch only on a clean preparation path;
- preserve fail-closed conflict evidence;
- do not open a PR automatically.

### 2B.1 — validation/promotion handoff

- define the prepared-branch Daniel-CL validation commands/evidence;
- bind PR promotion to the exact prepared candidate identity;
- preserve the human PR/CI gate.

### 2B.2 — recovery and regression hardening

- stale-base recovery;
- duplicate/abandoned-candidate handling;
- Issues-disabled behavior;
- interrupted preparation;
- deterministic rerun behavior.

### Phase 3 / 4

After helper/workflow implementation, perform source audit, targeted local validation, full Windows pre-CI validation, CI, merge, install, and soak through the normal CodexDD release process.

## Phase 1 exit condition

Phase 1 is complete when packets 1A.1, 1A.2, and 1A.3 are committed and accepted.

No implementation packet is authorized until that acceptance gate.
