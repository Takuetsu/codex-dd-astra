# CodexDD 0.4.5 — packet 3F.80: integrated bounded queue and workflow authority (NONRELEASE)

**Status:** 3F.81 source-pinned Windows native authorization acceptance **GREEN (5/5)** at `7b07d2cd3d7f841a2542ab794d6ba979d4992589` (nonrelease). All **13 formal Core/TUI owner holds remain open**; aggregate TUI replay memory and pending approval retention, full local release suite, Windows distribution and installer remain unapproved.

## 3F.81 PASS — five executed native Windows authorization regressions

GitHub [nonrelease run 41662986165](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41662986165) completed **success** on source commit `7b07d2cd3d7f841a2542ab794d6ba979d4992589`. The cheap-source job (source identities, Python fixture regressions, Rustfmt) and native Windows job both passed. The app-server integration test binary compiled with exit 0; **all five** focused selectors executed with exit 0 and a single passing test per selector:

1. `workflow_mutation_rejects_foreign_and_unsubscribed_connections`
2. `workflow_mutation_rejects_stale_source_turn_without_blocking_valid_or_cold_resume`
3. `v2_spawned_agent_rejects_direct_workflow_mutations_but_regular_threads_continue`
4. `workflow_state_update_preserves_authorized_top_level_cli_mutation`
5. `websocket_workflow_state_write_rejects_foreign_and_unsubscribed_connections`

Source-bound artifact **13244783790** (`codexdd045-3f81-auth-windows-native-evidence`) contains the compile and per-case logs, plus a nonrelease receipt. The old 3/5 failure remains documented below only as historical triage; its success-only helper was corrected in a test-only commit. No app-server production authorization logic was weakened to obtain the green result.

**Scope limit:** This clears the five-case 3F.81 Windows app-server authorization gate only. It is **not** a release acceptance, a 13-owner semantic signoff, a total replay-store memory guarantee, native verification of all Core/TUI behavior, or the 21-stage release suite on Daniel-CL. The later documentation-only commits do not change the source blobs covered by the green run.

## Exact-source lineage

- Parent: `7b5485f4bb3f764099d927338dd29a40f30e6364`, the 3F.76 bounded event/payload and thread-switch candidate.
- Parent evidence: [3F.76 Windows native #41493119662](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41493119662) finished SUCCESS; source-pinned 25 existing TUI trust and 11 new queue/payload/switch selectors passed (36/36), with artifact `codexdd045-3f76-payload-native-evidence` (#13165524066).
- Related stacked PR #79 (source head `4697ff07f250dc455d386875907545a07fefd147`) passed [general blocking CI #41493020504](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41493020504): all six jobs green, including Windows adaptive validation and blob-size checks. **This CI run does not validate this integrated head.**
- This branch integrates five app-server files from 3F.78/3F.79 on top of the green 3F.76 TUI parent. Follow-up commits also enforce source-turn freshness when a listener has a known lifecycle ID, with native unit and RPC negatives. No TUI runtime source files changed.

Current integrated app-server source blobs (the guard and test blobs differ from the initial transplant):

- `message_processor.rs`: `70254edbb6f9431f2ed33c254c38d886cea0027c`
- `request_processors/thread_input.rs`: `9aafd51739a2ece039cd4020475436699b8e6554`
- `request_processors/turn_processor.rs`: `ae25bb6106f534e32f113306562d523757e504df`
- `tests/suite/v2/thread_fork_multi_agent_tests.rs`: `9b7820809c376c2aba83e65e04ed8f58347ee6bb`
- `tests/suite/v2/connection_handling_websocket.rs`: `f5c6fbb91e6e034ed950272143b5ee87423517d2`

## PR #79 ancestry and controlled merge reconciliation

The current 3F.80 branch and PR #79 head `4697ff07f250dc455d386875907545a07fefd147` diverge at `a887ce0769bdc0c45626766a459923e4ba918bff`. The PR #79-only commit modifies two NONRELEASE CI workflow YAML files (`3f73-bounded-fifo-windows-native.yml` and `3f75-bounded-queue-compile-fix-preflight.yml`); there are **no PR #79-only runtime Rust changes**. The separate 3F.76 Windows acceptance intentionally uses its own exact source pin and 36 selectors. Preserve historical workflow/receipt identities rather than overwriting a 36-case test workflow with the older PR snapshot. Reconcile these workflow-only differences before a final stacked PR merge, only after integrated native validation.

## Security behaviors that must be demonstrated

1. Reject a `thread/workflowState/update` mutation on a multi-agent V2 spawned subagent. Do not change legacy or top-level CLI behavior.
2. Reject the real foreign WebSocket connection even when it owns a different subscribed thread.
3. Accept a genuine subscription's workflow mutation, then reject a stale write after `thread/unsubscribe`.
4. Reject a stale supplied `source_turn_id` when the app-server knows a newer active or terminal turn, while preserving explicit `None` and cold-resumed lifecycle behavior.
5. Preserve ordered, fail-closed TUI delivery and the 3F.76 payload limit, startup staging cap, and thread-switch controls; do not let a successful authorization test substitute for the earlier 36 case suite.
6. Do not infer a *total replay-store memory cap* from the 2 MiB per-event limit. The separate replay snapshot still allows many events; review aggregate retained bytes and voice/history replay separately before formal path acceptance.

## 3F.81 Windows native authorization — exact failure and repair

GitHub Actions run [41656502070](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41656502070) on commit `de7aa61c2df4be433970be01478c84f96a432242` passed source pins, Rustfmt, app-server integration binary compilation and **three of five** native authorization selectors, each exactly once. Its fourth test, `workflow_state_update_preserves_authorized_top_level_cli_mutation`, panicked inside the shared `TestAppServer::request` success-only helper when the server correctly returned JSON-RPC error `workflow state update source turn does not match current thread lifecycle` for a deliberate stale mutation. That test incorrectly expected `request()` to yield an ordinary `Err` on an RPC error. The fifth WebSocket selector was **not executed**.

Test-only commit `8abce084733cf0229fb8b9d865b0b45ac2d57f9f` corrects the test to send the expected-invalid request using `send_request`, then read and assert the JSON-RPC error with `read_stream_until_error_message`. The subsequent valid current-turn RPC remains typed and must still succeed. Server authorization production logic, protected dependency graph and the 13 formal high-risk owner holds were not changed.

For the next nonrelease 3F.81 workflow, run all five selectors despite an individual red result and report every failed test in the final receipt. This prevents a single failing selector from concealing an independent WebSocket failure. A green compilation is not native test acceptance; a green 5/5 run is still not formal Core/TUI owner signoff.

## Verified integrated Windows compile — November 3, 2026

Run [41650547740](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41650547740) completed successfully on source head `ee570f802e3943b4efab14ce79300220ebf916c1`. Both `cheap-source` (source pins, formal holds, Python fixture regressions, Rustfmt) and `windows-compile` passed. The exact `cargo test --locked -p codex-app-server --test all --no-run` compiler exit code was **0**, and artifact **13238866457** preserves the compiler log and nonrelease receipt. This clears the previously observed 60-error/40-error test fixture **compilation** blocker.

**Do not treat this as an executed test pass.** The five guarded workflow authorization selectors, including the positive RPC and negative real-WebSocket checks, still require source-bound native execution. This document's earlier failed compiler records are retained as historical evidence. All 13 P0 Core/TUI owner decisions remain pending, and production remains 0.4.4.

## Incremental source QA after the 60-error compiler failure

A lightweight test at `.github/scripts/test_codexdd_045_3f80_local_validation.py` now includes a source-scanning check for the precise preexisting app-server fixture incompatibilities observed in the prior Windows compilation: missing `status`/`encrypted_content` in `ResponseItem::Message`, missing `annotations`/`logprobs` in `ContentItem::OutputText`, and `ThreadReadResponse` destructures missing `read_state`. It scans the app-server V2, suite, and shared test-helper Rust files and fails if those required fields disappear from an explicitly constructed/pattern-matched value.

The scanner helper was exercised on synthetic valid, partial-match and invalid missing-field examples without running Rust. Existing real code was separately inspected through the GitHub API. **These are static source checks, not a green compiled test binary or CI receipt.** The Python check is not a new dependency of the Windows acceptance runner and will not block it on absent Python.

The replay memory audit additionally identifies a distinct interaction hazard: count-based replay eviction invokes `note_evicted_server_request`, which removes pending approval/elicitation/user-input tracking. A new byte-retention budget must preserve or explicitly fail closed on those requests, not silently evict them. See `docs/upstream-candidates/codexdd-045-3f80-replay-aggregate-retention-hold.json` for the source-bound P0 review criteria. The existing 13 unaccepted Core/TUI paths and all native release gates remain unchanged.

## 3F.80 automated recovery and source-only fixture audit

The first implementation of the local diagnostic reporter was found to contain an accidental duplicated script tail and an unterminated regex. Repaired on isolated 3F.80 commit `5bc71fe592e758f45fe956014836274c1541e2ab`, removing the duplicate and restoring a structurally balanced runner. The replacement runs source pins/Rustfmt first, attempts the aggregate app-server `--test all --no-run` compilation once, captures every compiler error/location in a digest, and only then runs the five focused tests. This is a **source-level audit only**; the repaired script has not been executed on Daniel-CL.

A nonrelease structural regression check, `.github/scripts/test_codexdd_045_3f80_local_validation.py`, was added to detect duplicate/truncated future runner edits and protect test ordering and security checks. This check is not equivalent to a PowerShell runtime parser or a Rust build.

I also inspected the **164 Rust source files under `codex-rs/app-server/tests/suite/v2/`** against the known pre-existing test fixture mismatch patterns: `ResponseItem` optional status/encryption fields, `ContentItem` output metadata, and new app-server thread read/list state fields. No remaining matches of those specific missing-field patterns were found after the earlier test-only fixes. The scan did **not** check all Rust struct initializers and cannot determine whether the previously reported 60 compiler errors have been cleared. Do not present it as a native pass.

The underlying first failure log was on Daniel-CL only; no source-equivalent 3F.80 integration compile receipt or fresh general CI exists. Do not request that Stonks retrieve Daniel-CL-only logs for this workflow. Keep CI, release promotion, formal owner signoff, and Windows distribution blocked pending native acceptance.

## Daniel-CL compiled-suite recovery (current blocker)

- Local source-pinned check at `c26a1653de228f2b71949ac486d927ecb3865bfc`: **Rustfmt green** and the first **three app-server library authorization selectors green**.
- The fourth selector, `workflow_state_update_preserves_authorized_top_level_cli_mutation`, could **not execute**, because the aggregated `--test all` integration binary failed to compile with **60 Rust errors** (26 warnings). The fifth real-WebSocket negative was never reached.
- The visible compiler diagnostics include `E0063` (older `ResponseItem::Message` fixtures missing `status` and `encrypted_content`) and `E0027` (older `ThreadReadResponse` patterns missing `read_state`).
- Test-only compatibility commit `ff58a25c74859f9ddd82f844be4ef3997c49f424` applies 18 narrow corrections to eight existing integration-test files. It changes no production source. **This has not cleared the remaining compiler errors and has not passed native compilation.**
- A follow-up validation script update writes a compact `*-errors.txt` diagnostic index whenever any Cargo test compilation fails. The original complete local log remains at `%TEMP%\\codexdd-045-3f80-local-validation\\workflow_state_update_preserves_authorized_top_level_cli_mutation.log`. Retrieve the existing log before another expensive rebuild to identify all remaining error classes.
- Release approvals, broad CI and seven-program packaging stay blocked. No old green run implies acceptance for this changed head.

## Rustfmt remediation after initial Daniel-CL preflight

Initial source-only Rustfmt at `951c4c97a5a722ddb5f6da16c342b4d62c9994` returned exit code 1 with exactly two reported differences: long WebSocket test signature and stale-turn assertion wrapping. These two format-only corrections were committed on the nonrelease branch. The validator now prints `RUSTFMT_DIFF` context and the full log path on failure. **The corrected head still requires a new Daniel-CL source preflight and native receipt**; do not treat this note as a pass.

## Automated Daniel-CL local acceptance (no GitHub CI)

Run only within a clean, isolated `E:\\codexdd` worktree on Daniel-CL, checked out to `dd/codexdd-v0.4.5-3f80-queue-auth-integrated`. The entrypoint verifies the branch, Git head, protected Cargo lock, unchanged production version, 13 unwaived owner holds and Rustfmt **before** attempting any native tests. It neither updates files nor triggers CI:

```powershell
& .\.github\scripts\codexdd-045-3f80-local-validation.ps1 -Mode Native
```

For cheap-only validation without compiling, use `-Mode Source`. Pass `-ExpectedHead <40-character SHA>` to reject a wrong checkout; pass `-LogRoot <writable-folder>` to place full logs and the `3f80-local-receipt.json` in a separate directory, including on a Stonks-accessible share. Both modes report elapsed time, exact commit and completion UTC; failures return a nonzero process exit code and cannot silently report a missing test as green.

## Cheap, nonmutating Windows preflight (Daniel-CL isolated E:\\codexdd worktree)

Do not run GitHub Actions or publish a new PR until this exact source passes the locally available inexpensive gates. Use the verified branch head, not a working-tree approximation.

```powershell
git status --short
git rev-parse HEAD
git rev-parse HEAD:codex-rs/Cargo.lock
Get-Content codex-rs/codexdd-version.txt
Push-Location codex-rs
cargo fmt --all -- --check
Pop-Location
git diff --check
```

The protected secure Cargo.lock blob must remain `13bc15ff844e93dfaf7777c73bb7313b35bb3b81` and the production marker must remain `0.4.4`. Do not autoformat unrelated paths to force the gate green.

## Targeted native cases after cheap preflight is green

Run these on Daniel-CL, capturing the true exit codes and each test's nonzero execution count:

```powershell
Push-Location codex-rs
cargo test --locked -p codex-app-server --lib workflow_mutation_rejects_foreign_and_unsubscribed_connections -- --test-threads=1
cargo test --locked -p codex-app-server --lib workflow_mutation_rejects_stale_source_turn_without_blocking_valid_or_cold_resume -- --test-threads=1
cargo test --locked -p codex-app-server --lib v2_spawned_agent_rejects_direct_workflow_mutations_but_regular_threads_continue -- --test-threads=1
cargo test --locked -p codex-app-server --test all workflow_state_update_preserves_authorized_top_level_cli_mutation -- --test-threads=1
cargo test --locked -p codex-app-server --test all websocket_workflow_state_write_rejects_foreign_and_unsubscribed_connections -- --test-threads=1
Pop-Location
```

Negative tests must fail if an unrelated connection changes a thread, an unsubscribed connection can mutate persisted workflow state, or an earlier source turn overrides a newer completed turn. Only after this is green should a narrowly source-bound Windows CI run or stacked PR be considered.

## Remaining release holds

- Formal 109-path owner ledger: `approved_owner_count=0`; **13** high-risk Core/TUI semantic holds are still listed. Neither source comparison nor green targeted tests auto-signs an owner review.
- Current integrated-head native and broad CI, app-server/daemon/WSL, voice-path and source-equivalent seven-program Windows distribution are not accepted by the historical runs.
- Final Daniel-CL 21-stage release acceptance, signing, voice DLL/installer checks, publication and 0.4.5 installation remain separate and blocked.
- No merge, promotion, signing, publication or installation; production CodexDD stays **0.4.4**.
