# CodexDD 0.4.6 — 3C.1b.2d: repository-owned independent Validator profile

**Date:** 2026-11-12.
**Parent source:** `acdb9d348a421ddf7365a847e21ffe0da6963806`.
**State:** 3C.1b.2d native Windows profile/contract fixtures accepted on 2026-11-12 at `64cbda642392391f2bf5c27863bfb36429af51b7`. No real Core-authenticated independent PASS or owner READY is asserted.

## Verified prerequisite

3C.1b.2c [GitHub Actions #43249759119](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43249759119) passed source preflight, Rustfmt and the focused native TUI independent-Worker handoff and evidence-matcher tests (18m39s). Those tests do not execute the repo-owned independent script.

## Confirmed runtime integration gap

The already implemented Core-native `run_codexdd_independent_validation` handler resolves `scripts/codexdd-test-independent.ps1` in the **selected exact Git worktree**. At the 3C.1b.2c checkpoint, the CodexDD development repository had targeted/workpacket/release scripts but **no independent script**. Core correctly failed closed with `profile_unavailable`, preventing any real PASS despite green source-unit tests. Installing a binary would not repair the missing repo-owned script.

## Scoped remedy

- Added a fixed `scripts/codexdd-test-independent.ps1`, using existing `codexdd-validation-common.ps1` version-1 native event contract. The script takes **no arbitrary command, profile, repo path, run ID or stage arguments**.
- Explicitly allowed the internal contract profile name `independent-review` in the shared PowerShell runner; Core's model-facing selector remains `independent_review` and only its distinct built-in Validator handler can emit a provenance-bound receipt.
- The six bounded objective source-review stages are: Git diff check, Rustfmt check, Core adaptive regression tests, TUI adaptive regression tests, focused Core native-receipt regressions, and focused TUI independent handoff regressions. They run in the current CodexDD source worktree and return PASS only after every native stage reports exit 0.
- Added Windows PowerShell syntax/stage/profile checks and PASS/FAIL/execution-error fixtures through the **real shared event emitter**. The fixture stages use `cmd.exe` and intentionally do not represent a real owner QA PASS. The separate GitHub job also rechecks the unchanged targeted-profile contract.
- Common/profile output must still be independently parsed by Core; missing stage, malformed JSON, process failure or script absence are non-PASS. Tool registration requires a separate Validation Worker with an exact clean SHA, authorized scope and source root. No evidence is durable or independently trusted merely because the script can be launched manually.

## Limitations and following work

1. This packet proves the configured script exists and adheres to basic native PowerShell contract/exit behavior once Windows CI is green; it does **not** prove the full, expensive six-stage script actually completed successfully through the guarded Core tool in a separate app-server session.
2. Scripts are **project-specific**. If a later authorized BREAKWATER source worktree does not contain an explicitly reviewed native independent profile, Core must remain `SETUP_BLOCKED`, rather than search the parent launcher or run CodexDD's own script in another project.
3. Owner-first-assignment issuance of source/runtime/independent gate requirements is still missing. No model-selected missing gate may be silently omitted. The next packet must bind a complete owner-authorized manifest before protected work, preserve it across fresh Worker handoff and keep operations runtime health pending without a real owned listener/HTTP receipt.
4. Run disposable, source-matched native Core+TUI+app-server end-to-end tests, followed by remaining reliability phases, full Windows acceptance, owner-controlled release/installation, and only then an approximately seven-day single BREAKWATER milestone soak.

**Safety:** No active BREAKWATER/Cycle Vengeance worktrees, deployment, 0.4.5 unfreeze, security weakening, arbitrary script execution, and no released 0.4.6 build.

## Acceptance checkpoint — 2026-11-12

[GitHub Actions #43258603289](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43258603289), exact source `64cbda642392391f2bf5c27863bfb36429af51b7`: **SUCCESS**. Source preflight, Rustfmt, native Windows PowerShell independent profile syntax/fixed-stage checks, shared contract-v1 PASS/FAIL/ERROR classification and existing targeted fixture compatibility passed. The first run [#43257796218](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43257796218) detected an isolated test-harness assignment to PowerShell's reserved `$Error` variable; it was corrected as `$errorExitCode`. The workflow now includes all independent profile script paths in its push filter so script-only changes trigger validation.

**Explicit nonclaims:** These are contract fixtures, not completion of the actual six-stage native profile through Core, not real two-Worker app-server handoff, not owner gate issuance, not 3C.1 completion, not a production install and not a BREAKWATER coding authorization.
