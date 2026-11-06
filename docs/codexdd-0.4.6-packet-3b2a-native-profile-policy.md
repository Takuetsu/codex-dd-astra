# CodexDD 0.4.6 — 3B.2a native profile/role setup blocker checkpoint
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`
**Parent:** `42fd68b209b3fd4471c4e26653bc2f15a40d6c5d` (3B.1 Windows CI green, run #42229580396).
**Status:** source packet committed for exact-head GitHub CI; **not a completed 3B.2, integrated native-gate acceptance, or release approval**.

## Scope
- The Core native LVO handler now independently denies Validation/Repair Workers and pinned but unbound roles before any profile script. The Implementation-only runner is not elevated for independent roles.
- A missing, invalid, or cross-repository repo-owned contract-v1 PowerShell profile emits `status=error`, `error_type=profile_unavailable`, and a `SETUP_BLOCKED` diagnostic with observed and expected paths. It is **not** a product stage FAIL, cannot issue PASS, and must not authorize automatic source repair, model escalation, or profile substitution.
- Core and registry regressions cover independent roles, partial pins, allowed bounded Node/Rust mechanical checks, and rejection of deploy/install, mutating format, reserved native profile shell impersonation, and destructive command combinations. Native PowerShell LVO remains guarded, sandboxed and receipt-backed.
- The scoped GitHub Actions regression workflow includes the two new executable Core cases and still verifies the existing 3B.1 worktree/SHA regressions. All execution is nonproduction and branch-exact.

## 3B.2 open work — do not misreport completed
1. First-message `[adaptive_worker]` authority still lives in the TUI; Core receives startup-config pins only. Bridge verified first-message authority to Core **through trusted thread settings**, not model-provided tool arguments or an untrusted prompt. Until bridged, 3B.1's TUI preflight and terminal guard are necessary but do not establish an independent Core pin.
2. `source_sha` currently identifies one immutable validation candidate. A Worker that commits legitimate edits must obtain an explicitly authorized candidate binding before LVO; do not equate initial source authority with a later produced SHA or silently accept arbitrary new HEAD.
3. Native profile availability must be tested on a real correct isolated Node/Rust Windows worktree with the matching repo-owned scripts. A missing contract profile is actionable owner setup, not permission to run arbitrary `.ps1` from another checkout.
4. Gate prerequisites, source/evidence scope and independent Validator acceptance remain 3B.3/3C work. 21 P0/P1 regression matrix is not proven by these two focused tests.

## Validation and safeguards
- Existing nonrelease 3B.1 GitHub run #42229580396 passed on previous source. This patch requires its own exact-head Rustfmt, Windows Core/TUI compile and focused executable tests; results are pending at this checkpoint.
- No 0.4.5 changes, branch merge, production modification, installation, alternate role grant, standalone operations manager or release/version bump.
