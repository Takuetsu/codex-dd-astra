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
