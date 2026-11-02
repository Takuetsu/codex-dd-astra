# CodexDD 0.4.5 — packet 3F.80: integrated bounded queue and workflow authority (NONRELEASE)

**Status:** Source integration prepared; fresh integrated-head Rustfmt, compiled tests and owner acceptance **NOT YET VERIFIED**. Never treat earlier green runs as validation of this new head.

## Exact-source lineage

- Parent: `7b5485f4bb3f764099d927338dd29a40f30e6364`, the 3F.76 bounded event/payload and thread-switch candidate.
- Parent evidence: [3F.76 Windows native #41493119662](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41493119662) finished SUCCESS; source-pinned 25 existing TUI trust and 11 new queue/payload/switch selectors passed (36/36), with artifact `codexdd045-3f76-payload-native-evidence` (#13165524066).
- Related stacked PR #79 (source head `4697ff07f250dc455d386875907545a07fefd147`) passed [general blocking CI #41493020504](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41493020504): all six jobs green, including Windows adaptive validation and blob-size checks. **This CI run does not validate this integrated head.**
- This branch integrates five app-server files from 3F.78/3F.79 on top of the green 3F.76 TUI parent. Follow-up commits also enforce source-turn freshness when a listener has a known lifecycle ID, with native unit and RPC negatives. No TUI runtime source files changed.

Current integrated app-server source blobs (the guard and test blobs differ from the initial transplant):

- `message_processor.rs`: `70254edbb6f9431f2ed33c254c38d886cea0027c`
- `request_processors/thread_input.rs`: `9aafd51739a2ece039cd4020475436699b8e6554`
- `request_processors/turn_processor.rs`: `ae25bb6106f534e32f113306562d523757e504df`
- `tests/suite/v2/thread_fork_multi_agent_tests.rs`: `eb95243d2b4183982fef0102071991113ade1e28`
- `tests/suite/v2/connection_handling_websocket.rs`: `221dc33a25aeb16aec4fa505bee88546d1979195`

## PR #79 ancestry and controlled merge reconciliation

The current 3F.80 branch and PR #79 head `4697ff07f250dc455d386875907545a07fefd147` diverge at `a887ce0769bdc0c45626766a459923e4ba918bff`. The PR #79-only commit modifies two NONRELEASE CI workflow YAML files (`3f73-bounded-fifo-windows-native.yml` and `3f75-bounded-queue-compile-fix-preflight.yml`); there are **no PR #79-only runtime Rust changes**. The separate 3F.76 Windows acceptance intentionally uses its own exact source pin and 36 selectors. Preserve historical workflow/receipt identities rather than overwriting a 36-case test workflow with the older PR snapshot. Reconcile these workflow-only differences before a final stacked PR merge, only after integrated native validation.

## Security behaviors that must be demonstrated

1. Reject a `thread/workflowState/update` mutation on a multi-agent V2 spawned subagent. Do not change legacy or top-level CLI behavior.
2. Reject the real foreign WebSocket connection even when it owns a different subscribed thread.
3. Accept a genuine subscription's workflow mutation, then reject a stale write after `thread/unsubscribe`.
4. Reject a stale supplied `source_turn_id` when the app-server knows a newer active or terminal turn, while preserving explicit `None` and cold-resumed lifecycle behavior.
5. Preserve ordered, fail-closed TUI delivery and the 3F.76 payload limit, startup staging cap, and thread-switch controls; do not let a successful authorization test substitute for the earlier 36 case suite.
6. Do not infer a *total replay-store memory cap* from the 2 MiB per-event limit. The separate replay snapshot still allows many events; review aggregate retained bytes and voice/history replay separately before formal path acceptance.

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
