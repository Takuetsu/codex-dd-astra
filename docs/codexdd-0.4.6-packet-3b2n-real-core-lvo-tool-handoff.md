# CodexDD 0.4.6 — Packet 3B.2n: actual Core LVO handler admission and native receipt

**State:** source-only integration fixture awaiting exact-source Rustfmt and native Windows acceptance.
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`.
**Parent:** `ac5e5fa7cde1cf56b8b845c50b768b0a367c9af6`.

## Evidence and scope

- 3B.2m's real PowerShell **wrapper** and native exit-sidecar cases passed on `4edbd5491e1661a7cc746d55ccb0b83c8f4d5f47`, [GitHub Actions #42922829343](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42922829343). That did **not** invoke `CodexDDValidationHandler` or validate an authorized Core turn configuration.
- 3B.2k's actual repo-owned Rust targeted profile passed 3/3 native stages (21 Core + 219 TUI focused tests) at `14380e84`; 3B.2l's six independent-role refusal-policy regressions passed at `c6890ca0`. The present packet does not repeat those expensive suites.
- The new Windows-only async fixture `codexdd_046_3b2t_real_core_lvo_handler_runs_trusted_pinned_fixture` creates a **disposable** new Git repository and one tiny contract-v1 test-only `scripts/codexdd-test-targeted.ps1`, commits it, builds an isolated Core Session/Turn with a **trusted config-bound Implementation Worker** at that exact worktree and SHA, marks the turn as protected MechanicalValidation, and invokes the **real `CodexDDValidationHandler`** with only `{"profile":"targeted"}`.
- Unlike 3B.2m, this must exercise **real handler admission**, Core-bound root/SHA comparison, in-process delegation via the actual `ExecCommandHandler`, native Windows PowerShell execution, the persisted out-of-tree log, the recorded child exit sidecar, post-exec no-dirty/no-SHA-drift check, and a completed tool receipt.
- It asserts `status=pass`, `exit_code=0`, expected `head_sha`, and the declared single native fixture stage in the structured tool output. Any generic tool error, handler execution refusal, timeout, bad source, missing receipt, or fake PASS causes a **red test**. The script and test repository are **synthetic test scaffolding**, not a real CodexDD/BREAKWATER validation profile and not evidence of a live model Worker.
- No `codexdd_validation.rs` production handler logic, Role privileges, source authorization, project scripts, or worktrees were changed to write this test.

## Validation contract

1. Run source/CI-contract and exact `cargo fmt -- --config imports_granularity=Item --check` against the **same** candidate SHA.
2. Opt-in a Windows 2022 job with repository-pinned Rust 1.95, V8, DotSlash, isolated Cargo temp target. Enumerate and execute **only** `tools::handlers::codexdd_validation::tests::codexdd_046_3b2t_real_core_lvo_handler_runs_trusted_pinned_fixture` using `--exact`. Never match the shared old 3B.2n or 3B.2m prefixes.
3. Record exact candidate SHA, job link, one test actually executed, process exit and control receipt. On failure, classify missing preflight permission/Exec runtime vs actual product bug; repair only the demonstrated issue. Stop after two noninformative retries.
4. Do not reuse a PASS-shaped synthetic script as proof that a real Node work_packet was run. Preserve private BREAKWATER profile boundary and live game worktrees.

## Still required afterward

A **real** independent Validator session and final owner receipt, actual private BREAKWATER Node profile in a safe isolated Windows repository checkout, actual separate running TUI/app-server cold restart, 3B.3 gate provenance, 3B.4 progress/refusal classifications, and final 3E/4 real-agent/21-stage/source-integrity acceptance. No release, merge, installation, 0.4.5 thaw or production 0.4.4 changes.

## Focused Windows handoff dispatch — 2026-11-10

The first 3B.2n native opt-in run [#42926791546](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42926791546) was **red at Rustfmt only**; source-contract PASS and the native Windows job SKIPPED. The Rustfmt log showed one indentation difference for the chained `tokio::time::timeout(...).await` block; no compiler error, test assertion, or runtime failure was observed.

The source-only repair `f7f01a984a1c4b570d56f59a55de4cd8001f6791` passed both source-contract and exact Rustfmt in [#42927535739](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42927535739); the native test was intentionally skipped on that unmarked push. **The next opt-in push runs only the 3B.2n native Core handler fixture after the same cheap gates.** No changes to product handler, Worker privileges, repo-owned actual profiles or live game worktrees. Do not claim 3B.2n acceptance until the full Windows test is enumerated, executed and green on the resulting exact commit.

## First native Core handler attempt and CI-only approval triage — 2026-11-10

**Observed RED:** [GitHub Actions #42927844687](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42927844687), source `7e814473042dfd1ec120db8840c25542b39e9838`. Source-contract and Rustfmt green; native Core build green; exact selector enumerated **1 test, 0 benchmarks**. Test `codexdd_046_3b2t_real_core_lvo_handler_runs_trusted_pinned_fixture` reached `handler.handle(invocation)` and **timed out after 90 seconds**, without returning a receipt. This is an incomplete integration test, not a PASS, compile failure, or a known profile-stage assertion failure.

**Hypothesis, not yet proven:** The test helper creates a Core session using the standard default `AskForApproval` policy, but has no frontend/user actor consuming `ExecApprovalRequest` and responding to it. Core's normal restricted-command policy can request approval; a legitimate approval request would be left pending in this fixture. This source audit does not prove that this exact test emitted such an event. A genuine product hang in the delegate or sandbox/executor remains possible.

**Narrow source-only diagnostic change:** Test fixture `b906943006a588e7e627fd514902dd2a8cc2ff78` sets `config.permissions.approval_policy = Constrained::allow_any(AskForApproval::Never)` **only for this disposable CI session**. This suppresses interactive approval questions that no CI frontend could answer; it does *not* disable the existing sandbox profile, change production approval defaults, grant a model new tools or alter the source-root/SHA/Implementation Worker gates. The test's outer bound is now 45 seconds. If it still stalls, it reports observed event categories (`exec_approval_request`, `exec_begin`, `exec_end`, or `other`) rather than asserting a false PASS or blindly increasing deadlines.

**Acceptance:** Exact source contract + Rustfmt, then one opt-in native Windows execution. If the handler reports an ordinary setup blocker, unauthorized source or timeout, the packet stays OPEN. Do not claim the approval hypothesis confirmed until runtime evidence supports it; the entire Core handler must execute the safe native fixture, persist the receipt and keep the source worktree clean before marking 3B.2n complete.

## Native denial diagnosis — 2026-11-11

**Observed RED with typed denial (not a timeout):** [GitHub Actions #42954490135](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42954490135), exact source `3e2a91d6c533756762af6e36490010836d24f623`, Windows job `130306695778`. Source-contract and Rustfmt PASS; the one exact Core test compiled and executed, then failed after 0.64s. The diagnostic receipt was `status=error`, `error_type=missing_native_log`, `exit_code=2`. Its message confirms the *delegated `exec_command` was rejected as "blocked by policy"*, so the native PowerShell script did not start and could not create the required log. The full handler **failed closed** and did not issue PASS.

**Root-cause distinction:** The earlier `OnRequest` configuration waited for a UI/frontend approval that did not exist in the direct test harness; changing only to `AskForApproval::Never` caused a command that still required authorization to be rejected by policy. This is expected security behavior, **not** evidence that the LVO source authorization has been bypassed. Windows' no-sandbox-backend restrictions and/or dangerous-command heuristics can require user approval even for a controlled fixture. Do not weaken a production policy or remove any role/source checks merely to make this synthetic fixture green.

**Narrow candidate, not yet accepted:** In source-only commit `59b341e5b11534bba22453b47c25d8e51f140e0c`, the single disposable test session sets the explicit `PermissionProfile::Disabled` (unrestricted) alongside `AskForApproval::Never` *only for its synthetic temporary repository*. The working branch does not change `codexdd_validation.rs`, any production sandbox/approval default, tool registration, or any game repository. No remote data or installed binaries are touched. This proves the desired *positive* Core path under explicitly authorized test execution if its Windows test passes; it **does not** prove the actual project profile can run under ordinary restricted Windows policy. 3B.2l negative-role tests and 3B.2m native-exit integrity remain distinct evidence.

**Next focused gate:** [GitHub Actions #43029759768](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43029759768) must pass the exact source/Rustfmt preflights, enumerate exactly one test and execute it to completion with a real handler PASS, expected root/SHA, out-of-tree receipt and clean source tree before 3B.2n's positive fixture may be closed. If the policy still rejects, stop broad/repeated Windows CI and diagnose the specific safety rule. An unrestricted **test-only** success must never be represented as safe automatic privilege escalation for active agents.
