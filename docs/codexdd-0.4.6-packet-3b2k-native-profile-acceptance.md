# CodexDD 0.4.6 — Packet 3B.2k: actual repo-owned Windows validation profiles

**Scope:** Native profile availability and contract acceptance in disposable test checkouts; no project source mutation, role relaxation, 0.4.5 thaw, production install, release, or branch merge.
**Starting checkpoint:** `d4b1f261a794a21032e9cb80f07b033b2c3a144b` on `dd/codexdd-v0.4.6-agent-reliability`.
**State:** Audit complete; native script execution **PENDING** on a new exact commit.

## Evidence and boundaries

- [#42890342163](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42890342163) passed both new native LVO Git-timeout and authorized nested source/SHA regressions at `80a2098543df51a45e424a3f13342d61573aa64f`. 3B.2j is closed for these exact focused tests.
- The separate preflight identity call `codex_git_utils::collect_git_info` is already **bounded**: `info.rs` uses 5-second `GIT_COMMAND_TIMEOUT`; `git_process.rs` uses `tokio::time::timeout`, `kill_on_drop` and Windows JobObject subprocess-tree ownership. Existing `git_process_tests.rs` includes descendant-cleanup regressions. **This is a source audit, not a new Windows execution of those Git-utils tests.** Do not duplicate this code or claim broad runtime proof.
- The real CodexDD Rust repo has `scripts/codexdd-test-targeted.ps1`, `codexdd-test-workpacket.ps1`, and `codexdd-validation-common.ps1`. Its targeted profile has three actual stages: clean Git diff, `cargo nextest` Core adaptive regression tests, `cargo nextest` TUI adaptive regressions. It emits contract-v1 profile/stage events and records executable exit codes.
- Read-only live source review of **private** `Takuetsu/breakwater` `main` confirms real **Node/TypeScript** repo-owned `codexdd-test-targeted.ps1` and `codexdd-test-workpacket.ps1` with contract-v1 native stages (targeted 4; work-packet 7), package scripts for focused validation, lint, build and profile tests, and Node >=20.19.0. **Availability/source contract only; no BW Node profile execution yet.** No private source, code, credential or private paths may be copied into CodexDD's repository or its public Actions artifacts.

## Bounded native Rust profile smoke (one outcome)

On an explicit opt-in source-matched `windows-2022` job, use the repository-pinned Rust/V8 and `cargo-nextest` setup. Run the unmodified actual `scripts/codexdd-test-targeted.ps1` in the disposable **CodexDD repository checkout**. Verify actual `CODEXDD_VALIDATION_JSON` `profile_begin`, three corresponding `stage_begin` / `stage_end` pairs and a matching final `profile_end` (status `pass`, exit 0, `completed_stages=3`); compare native script exit and declared stage exit codes. Preserve log as job output only, not an artificial release or user-facing LVO PASS.

If the environment lacks a required tool, the profile must emit an observable ERROR and the job must be red; do not substitute fake PASS or silently edit the targeted profile. Stop repeated identical runs after two attempts without new evidence. This **proves the actual Rust profile command contract on a native isolated Windows checkout**, *not* the live Core tool invocation/role/Worker handoff, **not** Node profile completion, and **not** a full work-packet or 21-stage release gate.

## Remaining hard requirements

1. Execute BREAKWATER's actual repo-owned Node profile against a correct isolated Windows checkout with owner-controlled repository access and current source credential. Do not run against live game worktrees; isolated native receipt must be checked before Node acceptance.
2. Verify real Core `run_codexdd_validation` tool invocation and correct first-user Implementation Worker admission/persisted source snapshot using the exact scoped candidate. Independent Validation/Repair roles remain denied the Implementation-only runner.
3. Complete actual separate TUI/app-server restart smoke, Phase 3B.3 durable gate evidence, 3B.4 progress/refusal taxonomy and 3C–4 release/owner gates.

**Never equate a green source/Rustfmt or synthetic fixture run with actual Rust/Node profile acceptance.**

## First native Windows profile run — observed red and bounded repair (2026-11-10)

**Fail (real test, not infrastructure):** [GitHub Actions #42893504401](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42893504401), exact source `cbfb01787733ad056684171a677d12ea08d9bc03`. Source-contract PASS, Rustfmt PASS, profile starts and emits contract-v1 events. Stage 1 `diff-check` PASS, stage 2 `core-adaptive-tests` **21/21 PASS**, stage 3 `tui-adaptive-tests` **218/219 PASS; 1 ABORT**. The failing `app_server_session::tests::detached_fork_restores_persisted_adaptive_workflow_state` aborted on Windows with `0xc00000fd` stack overflow **on two attempts**, while other tests continued. Nextest reported two non-fatal leaky cases. The native contract correctly ended `profile_end status=fail exit_code=1 completed_stages=2 failed_stage=tui-adaptive-tests` and script exit 1; the workflow correctly stayed red. Do not claim Rust profile PASS or blame format/compilation.

**Confirmed configuration mismatch:** This run supplied `RUST_MIN_STACK=16777216` at the isolated Windows job level, but `scripts/codexdd-test-targeted.ps1` reset it to `8388608`; `scripts/codexdd-test-workpacket.ps1` also reset it to 8 MiB. The existing release profile already used 16 MiB. This mismatch plausibly caused or exposed the single stack overflow; **16 MiB has not yet been proven sufficient**, and the test may have a separate recursion defect.

**Narrow repair:** Align only the targeted and work-packet profile `RUST_MIN_STACK` values with existing release/Windows runner 16 MiB. Retain every test, the repo-owned profile stages, independent execution, contract event rules, and fail-closed behavior; do not filter out the test, change stack guards in product code, or synthesize PASS. Before rerunning all three real stages, separately execute the exact `detached_fork_restores_persisted_adaptive_workflow_state` native Windows nextest filter with 16 MiB to isolate the regression. If the same overflow remains, STOP the full profile and classify/debug the test; do not repeat an identical 33-minute failed profile. A green isolated test alone is insufficient to close 3B.2k; the complete actual profile must pass on exact source.

## Second native Windows profile run — acceptance (2026-11-10)

**PASS: actual repository-owned Rust `targeted` profile.** [GitHub Actions #42900729499](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42900729499), exact source `14380e844f8ff31b00e937ca8457f20a27bb4c76`, native Windows job `130126443241`. The source-contract preflight and Rustfmt checks passed. Windows profile setup, pinned dependencies, and isolated source checkout passed.

- All three repository-owned Rust profiles (`targeted`, `work-packet`, `release`) now specify `RUST_MIN_STACK=16777216`; the two targeted/work-packet changes align them with the already-established release setting. This run **executed only targeted**.
- Before the real profile, the previously aborting `app_server_session::tests::detached_fork_restores_persisted_adaptive_workflow_state` executed independently on native Windows, **1/1 passed**, without an exception or manual skip.
- The actual unmodified `scripts/codexdd-test-targeted.ps1` emitted one contract-v1 `profile_begin` and one matching `profile_end` (`profile=targeted`, `status=pass`, `exit_code=0`, `completed_stages=3`). The workflow parsed the native stdout and compared the three stage_begin/stage_end pairs, ordinals and native exit codes to the process exit.
- `diff-check`: PASS, native exit 0.
- `core-adaptive-tests`: **21/21 passed**, native exit 0.
- `tui-adaptive-tests`: **219/219 passed**, native exit 0; nextest noted **3 leaky** tests. Those are non-fatal for this profile but remain a separate resource-lifecycle observation requiring evaluation during broader acceptance.
- The actual profile summary was **PASS, exit 0**; no synthetic PASS, disabled test, modified stage list, release, installed binary or project worktree mutation.

**Scoped disposition:** **3B.2k Rust targeted native-profile acceptance is COMPLETE** for exact source `14380e844f8ff31b00e937ca8457f20a27bb4c76`. This establishes a working repository-owned *Rust* profile and real test coverage in disposable GitHub Windows CI. It does **not** establish execution of the private BREAKWATER *Node* profile, an actual Implementation Worker calling the live Core `run_codexdd_validation` tool, independent Validation/Repair role enforcement across a live handoff, a real separate-TUI/app-server restart, full `work_packet`, or release acceptance. Phase 3B.2 remains open and the installed production CodexDD stays 0.4.4. The earlier failed source `cbfb0178` remains a documented red result and is superseded only for the observed stack defect under the tested 16 MiB configuration.
