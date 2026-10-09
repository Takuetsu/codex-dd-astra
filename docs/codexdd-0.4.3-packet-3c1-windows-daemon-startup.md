# CodexDD 0.4.3 — Packet 3C.1: Windows daemon and F2 startup compatibility

**Status: FOUR OWNED SOURCE CONFLICTS RECONCILED; STATIC SOURCE CONTRACT REVIEW PASSED 15/15. Native Windows compile/runtime and CI source-validation are NOT YET ACCEPTED.**

## Fixed authority and nonpromotion boundary

- Reconciled branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`.
- Fork production ancestry: `4828e3b4232781963594cfaaebdc49c5531de8b1` (0.4.2, tracking rust-v0.159.2).
- Complete target source: official `rust-v0.162.0` peeled commit `c1382380de69521303b416720a52f42d51af6248`.
- Merges: `1d20a0774ded9dcbcbc4dc8864cff79e4ff5ec5b` (Windows daemon, TUI fallback helpers), `cb6702fcbb4b98a21d593d333e94bd8dc592c8e0` (TUI startup orchestration).
- This is **not** approval to publish the canonical candidate, bump tracked upstream, install, merge production or run a Daniel-CL validation gate.

## Owned source paths

1. `codex-rs/app-server-daemon/src/backend/windows.rs`: began with complete 0.162.0 source, including pointer-based Win32 `HANDLE` migration and upstream's `is_elevated`. Retained the fork's typed `ElevatedLaunchRestricted` explicit lifecycle guard, fail-closed malformed token response length check, and a thin `is_current_process_elevated` alias using the **same upstream probe** for legacy tests/consumers. Retained typed `DetachedLaunchRestricted` error classification. Never weaken elevated-token rejection or duplicate the token query implementation.
2. `codex-rs/app-server-daemon/src/lib.rs`: kept new upstream `background_command` and `diagnostics` modules and upstream `is_elevated` export. Also exports the CodexDD typed elevation guard and compatibility alias, Windows only.
3. `codex-rs/tui/src/daemon_startup.rs`: retained upstream WSL DrvFS/9p CODEX_HOME exclusion and current feature negotiation; restored fork-owned elevated embedded classification, deterministic warning filtering, probe-failure fail-closed reason and typed fallback classification.
4. `codex-rs/tui/src/startup_orchestration.rs`: the target upstream 0.162.0 already added an early elevated-client redirect but displayed an elevated warning in F2 and **aborted** startup on an elevation-probe error. The fork hotfix requires *no F2 warning on confirmed elevated embedded startup*, and requires probe failures to fail **closed into embedded mode with a real F2 diagnostic**. Replaced only that upstream preflight policy with the fork's classified exclusion **before** implicit daemon socket discovery and reuse, while preserving upstream WSL DrvFS guard, new onboarding/startup mechanics, and fork's adaptive Luna startup pin. Kept typed automatic fallback for detached launch restrictions, and distinct telemetry for confirmed elevated vs unverifiable elevation.

## Security and behavior invariants (static source inspection, not execution)

15/15 exact source predicates passed: retained upstream pointer-safe Win32 `HANDLE` and modules, typed elevation errors, strict token length check, delegated upstream `is_elevated` probe, explicit daemon exports, WSL DrvFS exclusion, reason-mapped embedded fallback, warning suppression **only for successfully verified elevation**, diagnostic on failed probe, early daemon discovery exclusion and bounded telemetry. Explicit remote and daemon-wide agent selection remain outside the implicit startup exclusion.

The existing `codex-rs/tui/src/daemon_startup_tests.rs` and `codex-rs/app-server-daemon/src/backend/windows_tests.rs` assert confirmed-elevation no-F2 behavior, failed-probe diagnostics, launcher fallback warnings and explicit daemon guard. They are **3C.4-owned**; preserve/reconcile them there, and use their execution to accept 3C.1 when the native gate is available.

## Validation blockers

- **3B.5 first Linux source compile:** [run #37982039843](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37982039843) identified three upstream 0.162.0 `codexdd_validation.rs` API mismatches in the inherited fork handler. Targeted source fix `227c22bfd1a55a8a4a46c667e8b9a717e3536ef5` updates the StepContext environment resolver and exec handler options. [Read-only retry #37983354658](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37983354658) is a separate pinned-SHA compile receipt; do not count until terminal state checked.
- Native Windows target tests and the live elevated-SSH F2 smoke remain mandatory for the final release. Upstream's new pointer handles and typed daemon errors must be checked on Windows. The non-elevated daemon lifecycle waiver for old hotfix PR #48 does **not** grant a future release waiver.
- The new 3C.1 sources are source-merged pending native validation; remaining 3C.2–3C.4 TUI and tests require reconciliation before full integration.
- `codex-rs/Cargo.lock` drift is 3E.2-owned and must not be silently committed by 3C.1.

**Daniel-CL action: none yet.**
