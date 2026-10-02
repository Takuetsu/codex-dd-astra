# CodexDD 0.3.14 Phase 4 pre-install validation

Phase 4 is the broad Windows pre-install validation stage for CodexDD 0.3.14 after the completion of Phase 3F.

Target branch: `dd/codexdd-v0.3.14-upstream-0.159.2`

Production/base: `665f5d7c7966f9a6a9570b5b85f9d68c529c1639`

Target upstream: OpenAI Codex `rust-v0.159.2`

Platform authority: Daniel-CL / Windows.

Phase 4 is intentionally broader than the narrow Phase 3 validation gates. Its purpose is to catch integration defects before installation and soak testing, especially defects that only appear when CodexDD adaptive behavior interacts with upstream lifecycle, persistence, app-server, and TUI code.

Validation remains operator-mediated on Daniel-CL. No temporary GitHub Actions workflow is used for iterative validation.

## Packet 4.1 - repository/static and cross-crate compile baseline

**Status: COMPLETE.**

Purpose: establish that the finished feature tree is structurally clean and that the main executable surface compiles together before spending time on broad tests.

Checks:

- clean synchronized feature branch;
- `git diff --check` against the 0.3.13 production base;
- upstream-sync helper suite;
- Cargo workspace-manifest verification;
- TUI/core boundary verification;
- Bazel/Cargo lint-setting verification;
- locked Cargo metadata;
- Rust formatting;
- all-targets compile for CLI, TUI, app-server, and core.

Stop on the first failure and repair it before proceeding.

Initial Daniel-CL execution compiled the full requested all-targets surface successfully and found one warning: an unused `ReasoningEffort` import in `core/tests/suite/scenarios.rs`. The warning was removed in commit `cd0235019a97f93322bbb30d3ea2098dac3eefd2`. The clean compile rerun passed, so packet 4.1 is closed.

## Packet 4.2 - CodexDD adaptive control-plane sweep

**Status: COMPLETE.**

Purpose: run all tests discoverable by the `adaptive` filter across the three main runtime layers.

Checks:

- `codex-core` adaptive tests;
- `codex-app-server` adaptive tests;
- `codex-tui` adaptive tests.

This packet covers signal validation, routing, escalation/de-escalation, complexity floors, Worker authority, evidence pressure, lifecycle state, trusted signal delivery, terminalization, and adaptive UI/state behavior represented by current regression names.

Because the broad TUI `adaptive` filter includes the detached-fork regression, run this packet with the established Windows Rust test stack:

```powershell
$env:RUST_MIN_STACK = "16777216"
```

Remove the environment variable when the packet completes.

Daniel-CL result: PASS across `codex-core`, `codex-app-server`, and `codex-tui`. The TUI adaptive filter completed with 174 passing tests and zero failures. No 4.2 defect was found.

## Packet 4.3 - persistence, resume, fork, and interruption matrix

**Status: COMPLETE.**

Purpose: stress the areas that have produced the most integration defects during 0.3.14 work.

Use the established Windows Rust test stack:

```powershell
$env:RUST_MIN_STACK = "16777216"
```

Checks include:

- `codex-history --lib`;
- `codex-rollout --lib`;
- core rollout reconstruction;
- `child_workflow_snapshot_preserves_adaptive_state`;
- `resume_stopped_thread_from_rollout_spawns_new_thread`;
- `resume_stopped_thread_from_rollout_preserves_thread_source`;
- `rollout_path_resume_and_fork_read_history_through_thread_store`;
- interrupted fork-snapshot boundary handling;
- TUI persisted workflow-state restoration;
- detached-fork adaptive restoration;
- app-server trusted signal replay immediately before terminal completion;
- app-server interrupted-turn terminal handling.

Run these exact commands:

```powershell
$env:RUST_MIN_STACK = "16777216"

cargo test -p codex-history --lib --locked
cargo test -p codex-rollout --lib --locked
cargo test -p codex-core rollout_reconstruction --lib --locked
cargo test -p codex-core child_workflow_snapshot_preserves_adaptive_state --lib --locked
cargo test -p codex-core resume_stopped_thread_from_rollout --lib --locked
cargo test -p codex-core rollout_path_resume_and_fork_read_history_through_thread_store --lib --locked
cargo test -p codex-core interrupted_fork_snapshot --lib --locked
cargo test -p codex-tui session_state::tests --lib --locked
cargo test -p codex-tui detached_fork_restores_persisted_adaptive_workflow_state --locked
cargo test -p codex-app-server adaptive_runtime_signal_binds_event_turn_and_conversation_identity --locked
cargo test -p codex-app-server test_handle_turn_interrupted_emits_interrupted_without_error --locked

Remove-Item Env:RUST_MIN_STACK
```

Stop at the first failure. Remove `RUST_MIN_STACK` when this packet completes.

Daniel-CL result: PASS across the complete 4.3 matrix. History, rollout, reconstruction, workflow-state preservation, stopped-thread resume, rollout-path resume/fork, interrupted fork boundaries, TUI persisted-state restoration, detached-fork adaptive restoration, trusted adaptive signal replay, and interrupted-turn terminal handling all passed with zero failures. No 4.3 defect was found.

## Packet 4.4 - broad support/runtime package suites

Purpose: catch interaction regressions outside test names that explicitly mention CodexDD or adaptive behavior.

Run full library suites for the most relevant changed/runtime packages:

- `codex-protocol`;
- `codex-state`;
- `codex-thread-store`;
- `codex-app-server`;
- `codex-tui`.

Use the established Windows test stack for this packet so the heavier app-server/TUI resume/fork tests do not hit the known harness-only default-stack limit.

Run these exact commands:

```powershell
$env:RUST_MIN_STACK = "16777216"

cargo test -p codex-protocol --lib --locked
cargo test -p codex-state --lib --locked
cargo test -p codex-thread-store --lib --locked
cargo test -p codex-app-server --lib --locked
cargo test -p codex-tui --lib --locked

Remove-Item Env:RUST_MIN_STACK
```

Stop at the first failure. This is deliberately more expensive than earlier phase validation and is the primary pre-install whack-a-mole prevention gate.

## Packet 4.5 - release build and executable smoke

Purpose: prove that the actual release-shaped CLI can be built and started before installation.

Checks:

- locked release build of `codex-cli`;
- `codexdd 0.3.14` version identity from the built executable;
- upstream provenance remains `rust-v0.159.2`;
- basic CLI help/startup smoke that does not modify production installation;
- clean worktree after validation.

No production install occurs in this packet.

## Packet 4.6 - Phase 4 closure

Phase 4 closes only when:

- every packet is green on Daniel-CL;
- any defects discovered by Phase 4 have been repaired and their relevant packet rerun;
- the feature branch is synchronized and clean;
- product/version/upstream identities remain correct;
- no production install has occurred yet.

After Phase 4 closure, the release can proceed to the normal PR/CI stage, followed by install and soak under the existing release workflow.
