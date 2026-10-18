# CodexDD 0.4.5 — Packet 2B.3: legacy contracts, privilege and provenance audit

**Date:** 2026-10-18
**Status:** **PASS: 15/15 targeted Daniel-CL Python tests in 0.050 seconds** on checkpoint `db3716b0e747055798cd54a12dc19d0058aeec25`, confirmed by operator screenshot 2026-10-18. Windows LF/CRLF marker portability regression resolved. No runtime/workflow changes.
**Scope:** no runtime code, no policy gates opened, no GitHub Actions permissions changed and no new scheduled workflow.

## Boundaries audited

- The four original 0.4.3 v1-blocked and v2-reconciliation evidence files retain their exact historical Git blob SHA-1 identities: `ff4376a08c0023dee754a8e77f3dc950f498be2c`, `434ec9a6abf1417a2adea38d537f3151497c024e`, `a49bc9b33ada2ac2f82a12f19a4ab6e369f6736a`, `8d2c0645147660bf7514899057ac47177cba7608`. **These are historical 0.4.3 records, not the current 0.163.0 discovery evidence.**
- v1 candidate validation receipt parsing still represents a typed discovery/candidate claim and cannot bypass a release gate. v2 remains a separate strict contract with native Windows `release` profile, **at least 21 actual stages**, exact candidate/source SHA/tree and manifest/sidecar hashes, canonical JSON, required Daniel-CL host claims and explicit owner acceptance. Synthetic fixtures test parser rejection; they do not authenticate the host or replace genuine source-bound receipts.
- The production branch and expected 0.4.4 markers remain `dd/astra-policy-v2`, `0.4.4`, and `rust-v0.162.0` until the separately owner-reviewed 0.163.0 integration changes them.
- The only six-hour scheduled upstream workflow stays `contents: read` and report-only, with no automated candidate writer or draft-PR adapter execution. Old v1/v2 `workflow_dispatch` preparation (contents:write) is separate from promotion (pull-requests:write). Both v2 owner/Windows gates and absent-ref lease remain part of the historical workflows. No new source path changes those old flows.
- Both new writes are still **closed by source**: `AUTO_PUBLISH_ENABLED = False` and `DRAFT_PR_WRITES_ENABLED = False`; the reviewed path allowlist is empty. Real GitHub Actions PR-creation restrictions remain in place.

## Regression evidence

New `.github/scripts/test_upstream_045_contract_audit.py` contains 15 tests: legacy evidence byte SHA, v1/v2 positive schema fixtures and cross-contract denial, incomplete/forged native PASS rejection, v2 hash and candidate mismatch, original blocked-state integrity, pre-integration product identity, new closed gates, scheduled-vs-explicit workflow privilege isolation, existing v2 lease/owner controls, and native PowerShell receipt-identity guardrails.

After updating Daniel-CL's existing clean detached validation worktree to this commit, run **both** targeted scripts once:

```powershell
py -3 "$wt\.github\scripts\test_upstream_auto_pr.py"
if ($LASTEXITCODE -ne 0) { throw '2B.2 failed' }
py -3 "$wt\.github\scripts\test_upstream_045_contract_audit.py"
if ($LASTEXITCODE -ne 0) { throw '2B.3 failed' }
```

The target is **18 new 2B.2 tests plus 15 new 2B.3 tests**. Previously reported 28+17+18+16 tests already passed on Daniel-CL; do not rerun the long 21-stage Windows release suite or fire GitHub CI until accepted source integration needs a final release receipt. If a test fails, fix GitHub source, rerun only the affected cheap section.

## Final Phase 2 decision

2A.1–2B.3 can be accepted as **guarded updater infrastructure**, not automatic merge/installation or permission to prepare the conflict-heavy 0.163.0 release. Independently owner-accepted admission policy and review of any privileged scheduled writer remain OPEN; GitHub Actions permission denial may require a separately approved app/owner credential, never an unreviewed workaround.

**Next work after local test acceptance:** 0.4.5 Phase 3 source integration must preserve the original 0.163.0 blocked discovery, resolve the 15 conflicts and review the full 109 overlaps with separate exact v2 source/sidecar evidence and native release validation. Do not mark the release ready early.
