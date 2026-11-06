# CodexDD 0.4.6 — Packet 3B.1c: native runner source-binding checkpoint

**Date:** 2026-11-06  
**Parent:** `6d85cea231366d064e79de570e61ea28178066cc` (3B.1b Windows CI green)  
**Status:** IMPLEMENTED IN GITHUB SOURCE; **not yet validated by 3B.1c CI at this checkpoint**.  
**Boundary:** No production merge, 0.4.5 thaw, installation, privilege relaxation, source fallback, worktree reset, or host service operation.

## Why this packet is necessary

3B.1a/b guarded an Implementation Worker's `mechanical_validation` transition in the TUI by comparing the pinned root/SHA against the TUI config cwd. The actual protected Core `run_codexdd_validation` handler still resolved its runner location from `turn_environment.cwd()` and the nearest Git worktree. Without independently checking the native execution target, a mismatched backend cwd or another repository could run its own script and emit a pass receipt.

## Code changes

1. **Embedded launcher authority transfer:** `config_request_overrides_from_config` serializes a launcher-pinned `[adaptive_worker]` binding into the embedded app-server thread config. It does not add local host paths to a remote thread's config overrides, and unpinned legacy Workers retain their previous override behavior.
2. **Independent Core binding:** After the actual native tool execution cwd and Git HEAD have been resolved, Core checks the bound path and exact SHA. It canonicalizes selected/expected roots, reads the Git-reported top-level, compares live HEAD to the expected SHA and captured metadata, and rejects staged, unstaged, or untracked candidate changes. The pin never selects an alternate repository or runs scripts in it.
3. **Typed non-PASS refusal:** A root mismatch, invalid/incomplete binding, wrong SHA, dirty candidate, or explicitly non-source task yields a completed native ERROR receipt with `source_identity_mismatch`. Repository-owned profile absence remains `profile_unavailable`. There is no fabricated positive receipt.
4. **Post-script recheck:** Before retaining a successful native LVO completion, Core rechecks the pinned target. If a script changes HEAD or dirties the candidate, the would-be PASS becomes `source_identity_changed` ERROR. Script process failures and invalid native output retain existing non-PASS precedence.
5. **Focused source-matched GitHub regressions:** Core tests create a temporary real Git worktree and exercise clean nested target, wrong root, mismatched SHA, incomplete binding, non-source task, staged-change preservation, and a changed HEAD. TUI tests ensure that only an embedded launcher-pinned Worker gets the native startup-config bridge. The branch-scoped Windows CI now compiles Core tests and requires two specific Core test cases to execute.

## Explicit limitations and next work

- **First-message-only TUI source pins are not yet passed to Core.** This packet closes the startup-configured launcher path. A separate trusted TUI↔Core runtime authority bridge is necessary before claiming all Worker source bindings are enforced end-to-end (3B.2).
- **The source SHA is an exact candidate snapshot.** Implementation that creates a newer commit must checkpoint and intentionally bind a new candidate before this exact-SHA validator can accept it. 3B.2 must resolve the authorized initial SHA versus produced candidate SHA transition without treating arbitrary changes as trusted.
- The current Core validation profile is repo-owned PowerShell contract-v1. If the correct project does not provide it, the outcome is a typed setup/configuration blocker, not a source test FAIL, CLI upgrade, or privilege fallback.
- Validation must still pass rustfmt, Windows native `cargo check` for TUI/Core, and executable focused Core+TUI tests. The passing 3B.1b run **#42135076996** does not validate this new source.
- No performance/accuracy claims, full Windows distribution, runtime soak, overall 0.4.6 release acceptance, or production readiness follows from this limited source checkpoint.

**Go/no-go:** Keep 3B.1 open until new Windows CI is green. Do not move onto broader 3B.2 admission/evidence claims based on unexecuted source assertions.
