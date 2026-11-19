# CodexDD 0.4.6 — 3B.2d Core authority hold (prebridge)
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`  
**Based on:** `cf12f3424b1592d187e2d287affc7f769c6dd5b3` (3B.2c validated via Windows GitHub Actions #42284399610).  
**Status:** fail-closed source-policy checkpoint; **not** a completed first-message Core bridge, successful native acceptance, or 3B.2 completion.

## Confirmed defect
`CodexDDValidationHandler` honors a launcher-configured Worker source pin, but a first-message TUI assignment is only present in the TUI state and workflow snapshot. No trusted Core binding is installed. Without a guard, a protected adaptive MechanicalValidation turn can run the script in its selected checkout with `config.adaptive_worker=None`, even if the controller has a separate TUI-only source pin. That violates exact source authority.

## Guard implemented
- For the recognized `CODEXDD_ADAPTIVE_MECHANICAL_VALIDATION_TURN_TRIGGER`, native Core LVO now demands an Implementation Worker with both source root and 40-digit SHA in **Core turn config**; previously null/legacy-unpinned sources return `status=error`, `error_type=source_authority_unavailable`, `SETUP_BLOCKED` with a persisted diagnostic log and no failed-stage/repair fingerprint. No profile is launched.
- A matching source binding still passes the existing canonical Git root/HEAD/clean-tree verification. Validation/Repair roles and wrong/dirty/drifted checkouts remain blocked by the earlier 3B.2a/3B.1 guards.
- Ordinary nonadaptive direct LVO behavior remains unchanged for compatibility; source-change worker validation is deliberately held until the Core bridge is safe.
- Two executable Core regressions lock missing Core binding, legacy unpinned role, wrong role, pinned Implementation preliminary admission, nonrepairable receipt classification, and nonadaptive compatibility.

## Work remaining
- Bridge the first **user-authorized** immutable `[adaptive_worker]` binding to a trusted Core thread setting *before* protected native profile admission. Do not trust model-generated tool args, textual final answers, or ordinary workflow-state replay as authoritative Core config.
- Handle accepted source edits: the initial authorized base SHA is **not automatically** the SHA of a newly committed candidate. Require an explicit, auditable same-worktree candidate transition before LVO; do not silently authorize current HEAD.
- Verify real Node/Rust repo-owned contract-v1 PowerShell profiles in a Windows worktree and keep `profile_unavailable` as setup only.
- Windows CI compile/format/tests for this commit are not yet green; no merge, install, P0/P1 matrix completion or release claim.

Only GitHub branch source is changed. 0.4.5 remains frozen.
