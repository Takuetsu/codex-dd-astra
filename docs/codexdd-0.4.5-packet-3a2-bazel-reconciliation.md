# CodexDD 0.4.5 — Packet 3A.2: Windows Bazel CI three-way reconciliation

**Date:** 2026-10-18  
**Status:** REVIEWED / NOT APPLIED. A proposed Bazel workflow merge has been analyzed but not committed to the integration branch. No Actions run was dispatched.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Exact three-way evidence

- Tracked upstream `rust-v0.162.0` `.github/workflows/bazel.yml` Git blob: `ede5a7f070322ed101e0f29d55d8c87236926005`.
- Production CodexDD 0.4.4 version: `716fcca85a3cd03f44647ffab051aed34bd9c95f`.
- Official 0.163.0 version: `2f4b09deb1c0af682dd77bcd3370a67f662d959e`.
- Proposed reconciled blob (unreferenced and NOT APPLIED): `d668d8bf1964f83875d89a10dcc0940561433851`.

The base-to-production workflow change adjusts **four** Bazel test job timeouts from 30 minutes to **90 minutes** for the fork's Windows sharded/cross-build critical path. This is a deliberate 0.4.4 policy, not dead fork drift.

Official 0.163.0 introduces exactly two meaningful native MSVC workflow changes: (1) allow `workflow_dispatch` on the native Windows job in addition to post-merge `main`, and (2) add `--config=ci-windows-msvc` to the native Bazel test arguments. The proposed workflow patch would include **both** upstream behaviors while retaining **all four** 90-minute CodexDD test timeouts and the 40-minute standalone native MSVC job budget. No trigger, shard selection, credential permission, caching or alternate platform behavior was knowingly relaxed.

## Acceptance boundaries

- No CI is claimed PASS. The full upstream source, Bazel BUILD files, Cargo workspace/lockfile, generated schemas and 109 overlap-path review remain incomplete.
- Before a meaningful test, confirm Bazel syntax/target selection under the assembled source and ensure the new `ci-windows-msvc` configuration is present and has the intended Windows flags. Do not run a long CI job merely to trial an incomplete source tree.
- Review the four long-shard timeouts and native MSVC dispatch action under source-bound Windows CI acceptance. The separate Daniel-CL 21-stage suite is still required on the final frozen candidate.
- Production `dd/astra-policy-v2` and original updater/receipt workflows are untouched. Candidate publication/PR creation and product marker updates are not authorized by this packet.

**Next:** finish 3A.1 Cargo lockfile/security reconciliation after the source graph is coherent, then move to 3B/3C/3D semantic conflicts and generated artifacts in numbered packets. Historical 0.163.0 15-conflict and 109-overlap ledgers remain immutable.
