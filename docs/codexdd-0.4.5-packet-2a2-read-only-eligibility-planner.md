# CodexDD 0.4.5 — Packet 2A.2: independent read-only eligibility planner

**Date:** 2026-10-17
**Status:** source and regression suite checkpointed; targeted Daniel-CL Python validation **pending**.
**Scope:** pure policy decisions only. NO scheduled writer, automatic PR, candidate publication, production changes, release-version edits, or Windows validation substitutions.

## Implementation and authority boundary

- New source: `.github/scripts/upstream_auto_eligibility.py` with `CanonicalSnapshot`, `EligibilityDecision`, `assess_eligibility`, and deterministic, canonical JSON serialization.
- Independent recheck inputs include the canonical repository identity `openai/codex`, official tag name and object OID (recorded versus observed), annotated/lightweight tag type, peeled commit/tree, production SHA/tree, tracked-upstream SHA/tree, and complete Git tree-entry inventories.
- Validation compares the stored v1 discovery manifest hash, full exact overlap inventory, classifier categories, state and current Git identities. No user-provided or obsolete v1 `preparation_ready` state can authorize preparation.
- Status precedence is fail closed: malformed or moved tag/manifest => `blocked_identity`; changed production/tracked source => `blocked_stale_base`; missing evidence => `blocked_incomplete`; original or rechecked conflict => `blocked_transplant_conflict`; existing branch/PR/competing run => `blocked_duplicate`; any overlap, protected path, unapproved path, nonregular tree mode, or unsafe dependencies => `manual_semantic_review_required`.
- The **shipped low-risk safe-path allowlist is intentionally empty**. A test-only controlled allowlist proves the positive decision works, but an owner-reviewed path-specific policy must be committed before such a decision can occur in production. Every delta to Cargo dependency versions is review required, with additional hard stops for rollback of `rustls 0.23.46`, `quinn-proto 0.11.18`, or `noyalib-serde-yaml 0.0.57`, and resurrection of legacy `serde_yaml`.
- `auto_preparation_ready` is only a read-only planner classification. It does not provide permission to write, merge, release, install or bypass Daniel-CL receipt gates. Existing v1/v2 manifest or receipt bytes are unmodified.

## Explicit limitations / future packets

This pure planner does not read Git objects itself: its snapshot MUST later be derived by a pinned, read-only trusted resolver that validates tag objects, verifies every `ls-tree -rz` record against the claimed commit/tree OID, rechecks the security lockfile from Git objects, and guards read/prepare races. **Do not wire this directly to a writer.** The next packet 2A.3 owns this report-only read-only adapter and its provenance-bound report; 2B owns a separately accepted least-privilege preparer. Existing Actions token policy may forbid PR creation; do not circumvent it.

## Targeted local test

In the isolated validation worktree at the exact packet HEAD, run:

```powershell
py -3 .github\scripts\test_upstream_auto_eligibility.py
```

No broad Rust build, 21-stage suite, GitHub CI or installation is needed for this source packet. The outcome remains pending until Daniel-CL runs these tests. The official `rust-v0.163.0` update still has 109 overlaps and 15 direct conflicts, so it cannot be auto-prepared. Manual owner-reviewed v2 reconciliation is a separate later phase.
