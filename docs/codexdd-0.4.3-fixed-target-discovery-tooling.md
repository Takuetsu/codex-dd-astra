# CodexDD 0.4.3 — read-only fixed-target discovery tooling

**Scope:** tooling-only change for the Phase 2 packet 2A.1 upstream discovery blocker. **Not** 0.4.3 source integration, conflict resolution, branch preparation or product version allocation.

**Source:** `dd/astra-policy-v2` at `55139187f31d044d9f413245c38ba2da058cab2e`.  
**Isolated branch:** `dd/codexdd-v0.4.3-upstream-recovery-tooling`.  
**Owner instruction:** 2026-10-09 standing authorization for GitHub-side code/reconnaissance work without recurring permission prompts. Consequential release gates and authoritative Windows validation remain in force.

## What changed

1. `.github/workflows/upstream-sync.yml` now has optional **manual** `target_tag` and `expected_target_sha` inputs. Scheduled discovery retains the existing **latest official stable** selection behavior.
2. `.github/scripts/upstream_sync.py` resolves an explicit stable tag **only if present in canonical official upstream refs**; missing, prerelease and malformed requests fail, never falling back to latest.
3. Manually pinned runs require an exact lowercase 40-character **peeled commit SHA** matching `git rev-parse <tag>^{commit}`. The job also compares the official `ls-remote` tag-object ref SHA to fetched local tag-object ref SHA, for both tracked and selected tags.
4. The workflow continues using an **isolated dry-run synthetic customization transplant** and retaining its real state (`blocked_transplant_conflict` for textual conflicts); no conflict state is replaced with a promotable state.
5. Regression tests cover pinned-versus-latest selection, invalid/off-ref/prerelease tags, real CLI stdin behavior and non-write workflow boundaries. `.github/workflows/repo-checks.yml` already runs `python3 .github/scripts/test_upstream_sync.py` in blocking PR CI; no additional runner job or validation bypass was introduced.

## Dispatch after tooling is merged to the default branch

Use GitHub **Actions → upstream-codex-discovery → Run workflow**; supply exactly:

| Input | Value |
| --- | --- |
| Branch | `dd/astra-policy-v2` |
| `target_tag` | `rust-v0.162.0` |
| `expected_target_sha` | `c1382380de69521303b416720a52f42d51af6248` |

Leaving both optional inputs **empty** preserves the scheduled/latest behavior. Supplying only one input, a malformed tag/SHA, a tag absent from official upstream refs, or the wrong peeled commit must fail closed rather than choose a different target.

The successful job publishes a run summary and artifact `upstream-candidate-rust-v0.162.0-<run_id>` containing a manifest and `transplant-conflicts.txt`. **"Workflow success" only means the read-only discovery completed**, not that transplant or candidate preparation succeeded. If a conflict remains, the manifest **must remain `state=blocked_transplant_conflict`**; no candidate branch, PR or installation is implied.

## Safety, testing and next decision

- No product runtime, adaptive policy, LVO, F2 Windows logic, `codex-rs/codexdd-version.txt` or `codex-rs/upstream-codex-release.txt` changed.
- The job retains `permissions: contents: read`, `persist-credentials: false`, no `git push`, and no `gh pr create`.
- Existing `upstream-prepare.yml` and `upstream-promote.yml` remain **unchanged** and continue failing closed on conflicts, duplicate candidates, stale identities and missing receipts.
- The repository's blocking `repo-checks` plus focused tests are required before tooling PR promotion; no GitHub PR green check replaces an applicable future Windows-native validation.
- **Current limitation:** until the changed workflow reaches production and a new manual pinned run is executed, the 22 historical conflicts are only a forecast; the currently committed 2A.1 `discovered` observation has `transplant_dry_run=not_run`.
- **Next:** after a fresh fixed-target discovery run and exact identity verification, if its result is `conflict`, use the already planned **separate, independently audited conflict-reconciliation contract**; do not attempt a normal `upstream-prepare.yml` dispatch. If clean, evaluate normal safe v1 preparation.
- Re-resolve the production HEAD and regenerate all SHA-bound manifests **after this tooling PR merges**, since even a tooling-only merge moves production.

This file is a tooling change record, **not** a candidate manifest or release approval.
