# CodexDD 0.4.6 — Packet 3B.2m: native Windows LVO wrapper receipt integrity

**Scope:** Nonrelease mechanical validation wrapper integrity for real Windows `powershell.exe`; test source only, not production behavior change. **Parent:** `c911190636a26675adf9d161d71b604654c730e4`. **State:** Source checkpoint; native test PENDING.

## Previous verified acceptance

- [#42900729499](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42900729499): at `14380e84`, the actual repo-owned Rust targeted profile completed three native stages, 21/21 Core adaptive and 219/219 TUI adaptive tests (3 nextest leaky observations). The isolated cold fork regression passed with 16 MiB.
- [#42906745243](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42906745243): at `c6890ca0f0d0ef5b9d4d3bb549d444bbd90d0ca8`, six native Windows Core refusal/policy tests executed and passed. This establishes narrow 3B.2l source/role refusal gates, not live independent Validation/Repair.
- Actual `CodexDDValidationHandler::handle_call` delegates to `ExecCommandHandler` using `validation_powershell_command`; writes a log and native exit sidecar outside source; parses contract-v1 and calls `enforce_native_exit_consistency`. Existing code-level mock/parser tests did not execute that exact wrapper on native Windows. A forged `profile_end PASS` coupled with a nonzero native exit must never be accepted.

## Bounded native wrapper test

New Windows-only `codexdd_046_3b2m_native_wrapper_records_real_exit_and_rejects_forged_pass` creates only a disposable temp directory (including an apostrophe-containing path), writes a tiny **synthetic test fixture** `codexdd-test-targeted.ps1`, then executes **the actual CodexDD-generated PowerShell wrapper** in a native Windows process. Its case matrix:

1. Synthetic one-stage contract-v1 PASS + actual process exit 0: wrapper writes full stdout and stderr log, native exit sidecar 0; the validated summary remains PASS.
2. The same synthetic contract-v1 PASS + actual process exit 7: actual wrapper and exit sidecar must retain 7; `enforce_native_exit_consistency` changes the parsed PASS to nonrepairable `status=error`, `exit_code=2`, `error_type=native_exit_mismatch`; no failure/repair fingerprint.

**Acceptance:** A separate commit-message-opt-in Windows 2022 job after cheap source/Rustfmt gates runs `cargo test --locked -p codex-core --lib codexdd_046_3b2m_ -- --test-threads=1 --nocapture`; first enumerate exactly one matching test. Record one actual execution on exact source with native stderr, exit-file and fail-closed evidence. A source or format pass alone does not prove the wrapper. Do not run broad CI, alter the Rust targeted profile stages, invoke a live Worker, or claim an invented production validation receipt.

## Remaining outside this packet

- An actual Core `run_codexdd_validation` ToolInvocation, with a trusted Core-bound Implementation Worker source root/SHA and a repo-owned profile on a disposable project, remains unproven. This fixture calls the real wrapper **only**, not the full handler or role admission.
- Private BREAKWATER Node profile must be proven in its own isolated Windows checkout with proper repository access; do not copy private code, alter the live game branch, or infer PASS from the Rust profile.
- Real separate TUI/app-server restart and independent live Validator report, 3B.3 gate provenance, 3B.4 status/refusal telemetry, and later full Windows/owner release gates remain.
