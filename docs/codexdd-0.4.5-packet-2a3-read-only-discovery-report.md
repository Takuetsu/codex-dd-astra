# CodexDD 0.4.5 — Packet 2A.3: canonical read-only automatic update reporting

**Date:** 2026-10-17
**Status:** source, workflow and 17 targeted regression tests checkpointed; **Daniel-CL Python validation pending**.
**Scope:** add a read-only report to the existing six-hour upstream discovery. Do not add any write token, candidate branch, PR, release or publishing schedule.

## What changed

- Added `.github/scripts/upstream_auto_report.py`, a trusted Git-backed evidence adapter. It loads the immutable v1 manifest with duplicate-key, strict-field, type and canonical-byte checks. Recomputes the SHA-256; validates checkout HEAD and remote production ref, product/tracked version files, official pinned-versus-observed tag object OIDs (both tracked and target), local tag objects, peeled commits, Git trees and all tree entries. Each new evaluation is a **read-only report**, not a write-time authorization.
- On a clean synthetic transplant, the existing discovery job copies only the provisional Cargo.lock into ephemeral runner scratch **before** deleting the worktree, then parses the lockfile without running any upstream/candidate scripts. A SHA-256 of the complete lockfile and all package versions prevents skipping non-security dependency drift. No copy is taken from conflicted, symlinked or missing lockfiles; evidence gaps are fail-closed.
- A read-only GitHub PR query and exact candidate branch ref lookup detect duplicate candidates. An unavailable PR query fails closed to `blocked_incomplete`; it is never interpreted as no PR. Eventual write-time races cannot be eliminated by this read-only report.
- Existing `.github/workflows/upstream-sync.yml` retains `contents: read`, its six-hour schedule, original report, original immutable manifest, original conflict file and artifact-upload path. It appends `eligibility-report.json` to the discovery artifact and an explicit blocked/eligible reason summary. No write permission, PR creation, workflow escalation, or automatic promotion is introduced.
- All reports set `report_only=true`, `may_publish=false` and `windows_validated=false`, including hypothetical positive planner results. The reviewed safe-upstream-path allowlist remains EMPTY. The 0.163.0 candidate remains blocked with 109 overlaps and 15 conflicts.

## Test plan and gate

17 new regression tests in `.github/scripts/test_upstream_auto_report.py` use actual temporary Git commits/tag/tree objects with mocked remote metadata and cover pinned identity, synthetic conflicts, stale production, moved tags, PR/branch duplicates, security lockfile downgrades, canonical/duplicate JSON, missing dependency evidence, origin spoofing, Markdown safety and workflow privileges.

Targeted Daniel-CL command after updating the existing isolated worktree to the checkpoint: `py -3 .github\scripts\test_upstream_auto_report.py`. Do not run the expensive 21-stage Windows suite or broad GitHub CI for this source packet.

**Important:** This feature-branch workflow change is only staged in GitHub. It is **not active in the production six-hour schedule** until the separately authorized 0.4.5 merge. The pure 2A.2 planner and this reporting adapter are not a privileged preparer; Phase 2B remains separately governed.
