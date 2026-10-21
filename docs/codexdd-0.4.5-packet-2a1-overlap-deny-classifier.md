# CodexDD 0.4.5 — Packet 2A.1: conservative overlap classification

**Date:** 2026-10-17  
**Status:** source and regression tests checkpointed; **Daniel-CL native Python validation PASS, 28 tests in 3.044s on checkpoint 0a93c71497e0fb456b722f908e51f4c62727f227 (owner-provided terminal result)**.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-automation`.  
**Scope:** overlap classification and the independent overlap-only denial gate. No scheduled writer, candidate ref, PR, production change, or 0.163.0 conflict reconciliation.

## Implementation

- The classifier now recognizes the twelve previously unclassified 0.163.0 overlaps: root/nested Bazel BUILD files, CLI/daemon entrypoints/tests, core scenario and unified exec tests, external migration/memory/rollout-trace persistence, and generated Python protocol files.
- Every unfamiliar overlap is recorded explicitly in `unclassified_overlap`. The classifier retains the **exact Git path** (instead of recording slash-normalized aliases), so an unusual path or mode-only/tree rename cannot be silently omitted from overlap identity evidence.
- `auto_preparation_overlap_reasons(inventory)` is independent of the v1 `candidate_state` interface: **any** path intersection returns `overlap_requires_semantic_review`. Unknown classifications and inconsistent inventory are additional blockers. Zero overlap clears *only this gate*; it does not authorize publication or declare cross-file API/security compatibility.
- Existing v1 `CandidateManifest`, JSON contract version, deterministic identity and historical committed artifacts are **unchanged**. The new fallback can correctly cause a fresh v1 discovery with previously missed overlaps to return manual review, but it does not rewrite past artifacts. Never reinterpret the original 0.163.0 conflict manifest as clean.

## Tests / validation boundary

`py -3 .github\scripts\test_upstream_sync.py` (or `python .github/scripts/test_upstream_sync.py` on non-Windows). New unit fixtures explicitly cover the 12 named gaps, unknown-path and literal Windows-backslash identity, deny-by-default known/unknown/inconsistent overlap inventory, exact Git rename and mode-only change detection, and canonical v1 serialization / hash stability. The tests are not claimed PASS until executed against the checkpoint source.

This packet is only groundwork for **2A.2**, the separately auditable auto-eligibility planner (official SHA/tag/tree and production identity re-resolution, protected upstream paths, dependency/security downgrade, transit/race checks). Privileged scheduled auto-publication is still **NO-GO**. The 0.163.0 candidate remains **BLOCKED** and requires owner-reviewed v2 source reconciliation after applicable gates.
