# CodexDD 0.4.6 — Packet 3B.2j: bounded Git inspection before native LVO profiles

**State:** NONRELEASE source checkpoint, **not** formatting- or native-validated. **Parent:** `b41969be810ea5d22556c3fd4a3272785c5f51fb` on `dd/codexdd-v0.4.6-agent-reliability`.

## Outcome / classified evidence

Exact-source Windows [#42878341161](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42878341161) was GREEN at `b5c99d3d4af3828f3c590ea3122e9d4a0b4cdde3` with four individually executed/green Core Git, real app-server cold resume, embedded TUI pin and TUI Git-child regressions. **That result does not cover LVO's separate Git subprocess path.**

Read-only inspection of `codex-rs/core/src/tools/handlers/codexdd_validation.rs` found `validate_native_authorized_target` still performed three unbounded `Command::output` Git invocations (root, HEAD, full untracked status), independently of both corrected source-auth and TUI preflights. **Classification: A (source-design boundedness gap), not evidence of an observed LVO hang.** With a large/blocked Windows Git worktree, protected MechanicalValidation could wait indefinitely before a typed control receipt. No arbitrary source switching or approval bypass is authorized.

## Narrow repair

- In the LVO handler only, add `run_bounded_lvo_git_command` / `run_lvo_git_step`: launch read-only Git with `--no-optional-locks`, no interactive credentials, null stdin/stderr and scratch-file stdout. Bound metadata commands to 5s each, status to 12s, max 1 MiB stdout; poll the direct child, kill/wait on timeout; keep stage-specific, fail-closed `SETUP_BLOCKED` errors. Do not claim descendant-process-tree termination.
- Use this path for native `root`, `head` and `status` checks before an Implementation Worker may invoke a repo-owned PowerShell LVO profile. Preserve exact authorized source/SHA, clean-worktree and role gating. No profile PASS on incomplete preflight.
- Add cross-platform `codexdd_046_3b2s_native_lvo_git_child_timeout_is_setup_blocked_and_reaped`: an isolated five-second synthetic direct child with a 150ms deadline must fail `SETUP_BLOCKED` with termination reported.
- No change to `collect_git_info` or its timing guarantee, the PowerShell profile execution logic, Node/Rust project scripts, LVO permission policy or generated artifacts. The separate `collect_git_info` source identity inspection is not proven bounded by this packet and must be audited before complete LVO availability acceptance.

## Focused validation and exclusions

1. Verify the source branch HEAD, exact `cargo fmt -- --config imports_granularity=Item --check`, and source-contract review. If a formatting error occurs, correct only that source and recheck; no auto broad Windows CI.
2. After the required approved local-first/GitHub-only decision, run `cargo test --locked -p codex-core --lib codexdd_046_3b2s_ -- --test-threads=1 --nocapture` on native Windows and the existing `native_authorized_target_checks_real_nested_git_worktree_and_candidate_sha` positive/negative source checks at the same exact SHA. **Unrun tests are PENDING.** Do not turn source-only success into native PASS.
3. Still needed for Phase 3B.2: actual **repo-owned** Node and Rust contract-v1 LVO profiles in safe isolated real Windows worktrees, verified completion receipts with correct source and role; independently bound Validation/Repair; real separate-program app-server/TUI cold startup. A synthetic PASS script must not be represented as an official project work_packet receipt.
4. Later phases retain 3B.3/3B.4 and full 21-stage Daniel-CL, daemon/F2/security/distribution and explicit owner release acceptance. Never touch live BREAKWATER/Cycle Vengeance worktrees or production `0.4.4`, and do not thaw frozen `0.4.5`.

**Checkpoint policy:** This packet is an in-progress source-only repair with exact SHA after commit. No native acceptance, product release, installation, release version bump or merge authorized.

## Focused native Windows acceptance — 2026-11-10

**PASS on exact source `80a2098543df51a45e424a3f13342d61573aa64f`**, [GitHub Actions #42890342163](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42890342163), Windows native job `130090252983`. Source-contract and Rustfmt both passed. The native job enumerated and executed one test for each selector:

- `codexdd_046_3b2s_native_lvo_git_child_timeout_is_setup_blocked_and_reaped`: **1 passed, 0 failed**, actual native Windows process timeout and direct-child cleanup, 0.24s test time.
- `native_authorized_target_checks_real_nested_git_worktree_and_candidate_sha`: **1 passed, 0 failed**, actual nested worktree/SHA positive and refusal regression, 0.74s test time.

This is **2/2 executed tests**, not compilation-only success. Other focused Core/TUI and broad Windows jobs were **skipped by design**; previous green 3B.2i-h evidence remains scoped to its earlier tested SHA. The reported 3B.2j source/format revisions are superseded by this accepted exact source.

**Disposition: 3B.2j focused repair COMPLETE.** The native LVO profile command runner, actual Node/Rust contract-v1 profile completion, independent Validator and actual two-program cold-start integration remain separate. The initial `codex_git_utils::collect_git_info` source-identity call precedes the newly bounded helper and was explicitly excluded from this packet; verify its own process budgets before declaring all native LVO Git inspection bounded. No release, merge, install, 0.4.5 thaw, user worktree mutation or model authority expansion.
