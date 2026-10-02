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

Run the non-TUI library suites with Cargo, then run the full TUI library suite through the repository's canonical `just test` recipe:

```powershell
$env:RUST_MIN_STACK = "16777216"

cargo test -p codex-protocol --lib --locked
cargo test -p codex-state --lib --locked
cargo test -p codex-thread-store --lib --locked
cargo test -p codex-app-server --lib --locked

Remove-Item Env:RUST_MIN_STACK
cd E:\codexdd
just test -p codex-tui --lib
```

The TUI suite must not use plain `cargo test -p codex-tui --lib` as the Phase 4 full-suite authority. It also must not substitute a hand-written `cargo nextest` invocation for the canonical recipe until the recipe environment is reproduced exactly. The root `justfile` exports `CODEX_REPO_ROOT`, selects `NEXTEST_PROFILE=local`, and applies the repository test-stack setting before launching nextest.

Initial Daniel-CL execution with plain Cargo reached 5601 passing tests but reported 70 snapshot/rendering failures. A follow-up direct nextest run still reported a broad cluster of failures, but that command omitted the root `justfile` environment. Both runs are diagnostic only. The visible snapshot sources were upstream-identical. Do not accept generated `.snap.new` files.

Before another full TUI run, rerun representative failing tests through the exact recipe environment. Only failures reproduced there are Phase 4 product/test-baseline candidates.

The representative analytics snapshot passed under the canonical `just test` environment. The representative dynamic-tool test reproduced and was traced to a CodexDD-specific stale upstream assertion: CodexDD intentionally prepends an `evidence_id` content item to dynamic-tool model outputs, changing the Responses API `output` from a string into a structured content array. The runtime behavior is intentional and required by adaptive evidence references. Commit `8e3a3820fc5da57b3cc0a1a9de5ffd8976b1a914` updates the regression to assert both the evidence metadata and the preserved JSON task response instead of requiring the obsolete upstream string-only shape. This repair requires targeted Daniel-CL validation before the full canonical TUI suite resumes.

Stop at the first non-TUI failure. For the canonical TUI run, allow the full suite to finish so independent failures are visible. This is deliberately more expensive than earlier phase validation and is the primary pre-install whack-a-mole prevention gate.

The canonical full TUI run later reproduced a multi-test failure set even after the dynamic-tool assertion repair. Do not patch those failures blindly. Classify them differentially against the exact target-upstream commit `ff6aec96948b70d94983af2641a6b67c94faeff5` on the same Daniel-CL host and with the same `just test` harness. A failure that reproduces unchanged on the vanilla target is upstream/environment baseline and is not a CodexDD release regression; a feature-only failure remains a Phase 4 blocker. Keep baseline validation in a disposable detached worktree and reuse the main target directory to avoid duplicating build storage.

Daniel-CL differential baseline confirmed at least one canonical TUI failure on the exact vanilla `rust-v0.159.2` commit: `fresh_startup_reads_destination_and_cleared_model_uses_catalog` fails the same inline snapshot because the 0.159.2 test normalizes the wrong cwd value on Windows. Upstream main has since changed this test to normalize the destination path instead, confirming this specific failure is a target-upstream test-baseline defect rather than a CodexDD runtime regression. The next authoritative step is a full vanilla 0.159.2 TUI nextest run on Daniel-CL so the complete baseline failure set can be compared with the CodexDD run.

That full vanilla run completed and reproduced the same broad Windows TUI failure clusters seen on the CodexDD run: owned-transcript/layout, reconnect/replay, realtime-caption replay, startup destination snapshot normalization, history-cell update snapshots, status snapshots, and update-prompt rendering. These overlapping failures are now classified as the 0.159.2 Windows baseline rather than CodexDD regressions.

Two failures visible in the CodexDD final list were not visible in the completed vanilla baseline list and therefore remain differential candidates pending focused rerun:

- `recap_generation_uses_bounded_structured_request_and_inserts_result`
- `slash_copy_picker_copies_status_fields_and_preserves_source_after_copying`

The earlier `oversized_read_preserves_answer_in_next_model_request` differential was already repaired in commit `8e3a3820fc5da57b3cc0a1a9de5ffd8976b1a914`.

Before the next full CodexDD TUI run, preserve the vanilla JUnit failure names and run the two remaining differential candidates individually. Only feature-only failures block 0.3.14; the exact-upstream Windows baseline is recorded separately.

Both remaining differential candidates passed individually under the canonical `just test` environment on Daniel-CL:

- `recap_generation_uses_bounded_structured_request_and_inserts_result` - PASS.
- `slash_copy_picker_copies_status_fields_and_preserves_source_after_copying` - PASS.

The warning about an unused `ToolCallSource` import seen during these runs originates from the exact-upstream 0.159.2 `codex-core` artifact in the shared Cargo target cache; the CodexDD feature branch already removed that upstream-only unused import. It is not a feature-tree warning.

The remaining 4.4 authority is one final full canonical CodexDD TUI run followed by a mechanical JUnit set comparison against the preserved vanilla 0.159.2 failure list. Phase 4.4 passes if the CodexDD failure set is a subset of the recorded vanilla baseline after the already-fixed dynamic-tool differential is excluded; any feature-only failure remains a blocker.

### Differential harness correction

The first full differential comparison is invalidated by cross-worktree Cargo artifact contamination. The disposable vanilla worktree was pointed at the feature tree's `CARGO_TARGET_DIR` to save disk space. A later feature-targeted status test, launched from `E:\codexdd`, produced its Insta `.snap.new` path under `E:\codexdd-upstream-01592\...`, proving that the executable in the shared target directory had been compiled from the vanilla worktree and was reused during feature validation.

Therefore:

- the reported 45-vs-45 JUnit comparison is diagnostic only and must not be used to classify feature-only failures;
- the two apparent feature-only failures from that comparison do not justify source changes;
- future cross-worktree differential runs must use distinct Cargo target directories;
- the preserved vanilla 0.159.2 baseline remains valid for the vanilla run itself;
- CodexDD must be rerun from a fresh feature-only target directory before Phase 4.4 can close.

After the clean feature run, compare its JUnit failure set against the preserved vanilla baseline. Delete the temporary feature-validation target after Phase 4.4 closes to reclaim disk space.

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
