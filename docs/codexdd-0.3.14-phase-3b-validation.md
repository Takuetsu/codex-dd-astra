# CodexDD 0.3.14 Phase 3B validation

Phase 3B core/runtime integration for the CodexDD 0.3.14 upgrade to OpenAI Codex `rust-v0.159.2` is complete.

## Anchors

- CodexDD production/base SHA: `665f5d7c7966f9a6a9570b5b85f9d68c529c1639`
- Target upstream: `rust-v0.159.2`
- Target upstream SHA: `ff6aec96948b70d94983af2641a6b67c94faeff5`
- Feature branch: `dd/codexdd-v0.3.14-upstream-0.159.2`
- Phase 3B ownership set: 59 overlap paths from the Phase 3A integration scaffold
- Target platform: Windows

## 3B.3 ownership and hygiene

The complete 59-path Phase 3B ownership set was audited.

Acceptance results:

- all 59 ownership paths were present;
- no unresolved merge markers remained;
- no overlap path remained an exact 0.3.13 copy where upstream 0.159.2 changed it;
- Rust formatting passed;
- Phase-3B-scoped `git diff --check` passed;
- validation-generated `codex-rs/Cargo.lock` refreshes were restored and intentionally deferred to Phase 3E provenance/build work.

## Compile validation

Local validation on Daniel-CL:

- core/runtime package surface compiled successfully;
- app-server protocol, app-server, and config compiled successfully with `--all-targets`;
- CLI compilation is intentionally deferred to Phase 3C because `codex-cli` directly depends on `codex-tui`, and the unfinished 3C TUI surface currently fails on the newly restored CodexDD adaptive notification variants.

This CLI deferral is a phase-boundary dependency, not a Phase 3B failure.

## Generated app-server export validation

The two 3B.2 precomputed export bundles were regenerated from the merged 0.159.2 schema sources and validated locally:

- stable export consistency: PASS
- experimental export consistency: PASS

Artifacts:

- `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst`
- `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst`

## Adaptive signal regression

The CodexDD app-server adaptive signal regression passed after the test was isolated from unnecessary full ThreadManager startup:

- live adaptive signal delivery: PASS
- source turn identity binding: PASS
- conversation identity binding: PASS
- trusted signal replay immediately before terminal completion: PASS

The production behavior is unchanged; only the regression-test setup was narrowed to the actual behavior under test.

## Core/runtime support-crate tests

The following library suites passed locally on Daniel-CL:

- `codex-history --lib`
- `codex-rollout --lib`
- `codex-protocol --lib`
- `codex-state --lib`
- `codex-thread-store --lib`

The thread-store suite completed with 261 passing tests and zero failures.

## Core reconstruction and resume tests

The rollout reconstruction module completed with:

- 36 passed
- 0 failed

Additional targeted tests passed:

- `thread_manager::tests::child_workflow_snapshot_preserves_adaptive_state`
- `thread_manager::tests::resume_stopped_thread_from_rollout_spawns_new_thread`
- `thread_manager::tests::resume_stopped_thread_from_rollout_preserves_thread_source`
- `thread_manager::tests::rollout_path_resume_and_fork_read_history_through_thread_store`

## Windows Rust test-stack note

The stopped-thread resume tests above overflow the default Rust test-thread stack on Windows with `STATUS_STACK_OVERFLOW`.

The same upstream-aligned tests pass unchanged when run with:

```powershell
$env:RUST_MIN_STACK = "16777216"
```

This is treated as Windows test-harness stack sensitivity, not a CodexDD runtime defect:

- the failing resume test matches upstream 0.159.2 structure;
- increasing only the test-thread stack makes the test pass;
- no production code change is required or authorized for this condition.

For heavy core resume/fork tests on Windows, local validation should set `RUST_MIN_STACK=16777216` and remove it afterward.

## Defects found and repaired during 3B validation

Robust local validation caught and repaired several integration defects before PR/CI:

- missing `RolloutItem::WorkflowState` handling in resume metadata;
- missing `RolloutItem::WorkflowState` handling in rollout reconstruction;
- stale Phase 3B import and formatting drift;
- outdated app-server adaptive regression call signature after upstream API changes;
- invalid whole-`ServerNotification` equality assertion after upstream enum changes;
- oversized adaptive regression fixture that overflowed the Windows test stack;
- three CodexDD client-test call sites missing upstream 0.159.2's new `include_internal` argument.

## Phase 3B exit

Phase 3B exit conditions are satisfied for its owned core/runtime surface.

Phase 3C begins as a new numbered work packet and owns the TUI/adaptive-lifecycle integration required before the CLI/TUI surface can compile end-to-end.

Do not treat this checkpoint as PR/CI authorization, production installation authorization, or soak authorization.
