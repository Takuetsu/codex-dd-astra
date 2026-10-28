# CodexDD 0.4.5 — Packet 2B.1: isolated guarded preparer

**Date:** 2026-10-17
**Status:** **Daniel-CL targeted validation PASS: 16/16 tests in 7.710 seconds** at source checkpoint `e68f5c3ba6c44f3f87888181430e78e335fba924` (operator terminal evidence). The guarded publisher remains hard-disabled.
**Feature branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-automation`.

## Scope / consequence boundary

`.github/scripts/upstream_auto_prepare.py` adds the _uninvoked_, hard-disabled isolated candidate construction and publication contract. **`AUTO_PUBLISH_ENABLED = False`** is a literal source guard; neither the six-hour discovery workflow nor a new privileged schedule imports this module. The separate reviewed safe upstream path allowlist is **empty**, making real automatic preparation ineligible even if the hard gate were changed accidentally.

No GitHub Actions permission changes, scheduled writers, candidate branch, PR, merge, product-version bump, installation or upstream 0.163.0 conflict reconciliation are part of this packet. The original 0.163.0 manifest and fifteen real conflicts stay blocked for owner-reviewed v2 integration. Prior 0.4.2/0.4.3 manifests, receipts and v1 explicit manual preparation are unchanged.

## Source behavior

1. **Preflight:** use `upstream_auto_report.evaluate`, which independently rechecks official pinned-versus-observed tags and peeled/tree objects, exact production HEAD/ref, Git path inventory, Cargo lockfile and candidate branch/PR absence. A copied artifact or a stored positive status cannot authorize the helper. Only the exact `auto_preparation_ready` classification is a preparer input.
2. **Prepare locally:** use an isolated temporary `GIT_INDEX_FILE` and Git `read-tree`, exact `update-index --cacheinfo`, `hash-object`, `write-tree` and `commit-tree` plumbing. Apply each independently re-derived, zero-overlap upstream tree change onto the trusted CodexDD production tree as a 100644 blob; reject deletion, invalid mode/type or unsafe path. Never execute upstream scripts or checkout and run candidate hooks/filters/merges.
3. **Provenance:** copy the immutable canonical v1 discovery-manifest bytes exactly to `docs/upstream-candidates/codexdd-upstream-<tag>-<sha12>.json`, update only the source provenance marker `codex-rs/upstream-codex-release.txt`, and retain the approved CodexDD product version and security Cargo.lock. Verify the full staged leaf inventory against a separately predicted tree and its manifest blob.
4. **Double-check:** independently repeat the entire official tag, production, Git tree, dependency and branch/PR checks after computing the provisional candidate tree. Fail on movement or duplication. Create a deterministic, **unreferenced local** commit with the accepted production SHA as its sole parent, provenance-bound to the original immutable manifest SHA-256 and recorded discovery time. Do not create a branch or contact a write API.
5. **Future publish entrypoint:** a function includes the necessary exact commit/tree/manifest/parent checks, final eligibility recheck and `git push --force-with-lease=<candidate-ref>:` (remote ref must be absent). It is intentionally unreachable while `AUTO_PUBLISH_ENABLED` is false. Disabling the guard, adding privileged workflows, and defining a nonempty reviewed safe path allowlist require separate owner acceptance.

## Validation

`.github/scripts/test_upstream_auto_prepare.py` adds 16 actual-Git, no-network fixtures (reusing 2A.3 provenance samples) for closed gates, exact base/target overlay, deterministic commits, unchanged v1 manifest bytes, zero local refs, production-moved races, candidate branch races, unavailable PR queries, malformed manifests, conflicts, unsafe paths, Windows reserved names/case-fold collisions, mode transitions, interrupted temporary index operation and read-only workflow boundaries.

After fetching the exact checkpoint to the existing worktree inside `E:\codexdd`, run:

```powershell
py -3 "$wt\.github\scripts\test_upstream_auto_prepare.py"
```

Run only this cheap Python suite until it passes. Do not launch the 21-stage Windows release suite, GitHub build CI, real candidate creation or any privileged workflow. Before 2B.2, the separate owner-reviewed auto-publish admission policy and GitHub permission review remain explicit gates.
