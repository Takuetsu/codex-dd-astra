# CodexDD 0.3.14 Phase 3F audit

This document is the durable audit record for Phase 3F of the CodexDD 0.3.14 integration of OpenAI Codex `rust-v0.159.2`.

## Phase 3E closure

Phase 3E is complete at source checkpoint `3b6958b70c2657c178c39e65eb3b2fc5807e4c0a`.

The Daniel-CL validation gate completed successfully after that checkpoint:

- `.github/scripts/test_upstream_sync.py`: 13 tests passed.
- `.github/workflows/repo-checks.yml` contains the `Test codexdd upstream sync helpers` repo check.
- `cargo test -p codex-cli --test version_reporting --locked`: 1 test passed.
- Product identity remains `codexdd 0.3.14`.
- Tracked upstream identity remains `rust-v0.159.2`.

The informational upstream-sync diagnostic about GitHub Issues being disabled is not a test failure; the suite completed with `OK`.

## 3F.1 - CodexDD-owned adaptive anchor survival and wiring

**Status: COMPLETE.**

### Scope

3F.1 audits the CodexDD-only adaptive modules explicitly called out by the Phase 3A integration scaffold. It verifies that they survived the upstream replacement, remain wired into the 0.159.2 tree, and were not silently replaced by upstream files.

Target upstream: `ff6aec96948b70d94983af2641a6b67c94faeff5`.

Production/base: `665f5d7c7966f9a6a9570b5b85f9d68c529c1639`.

### Preservation result

All 16 named preservation anchors are present on the 0.3.14 feature branch and absent from the target upstream tree, confirming that the CodexDD-owned surfaces remain explicitly carried by the fork:

- `codex-rs/core/src/tools/handlers/adaptive_signal.rs`
- `codex-rs/core/src/tools/handlers/adaptive_signal_tests.rs`
- `codex-rs/tui/src/adaptive_budget.rs`
- `codex-rs/tui/src/adaptive_classification.rs`
- `codex-rs/tui/src/adaptive_complexity.rs`
- `codex-rs/tui/src/adaptive_controller.rs`
- `codex-rs/tui/src/adaptive_evidence.rs`
- `codex-rs/tui/src/adaptive_policy.rs`
- `codex-rs/tui/src/adaptive_worker.rs`
- `codex-rs/tui/src/chatwidget/adaptive_admission.rs`
- `codex-rs/tui/src/chatwidget/adaptive_effort.rs`
- `codex-rs/tui/src/chatwidget/adaptive_evidence.rs`
- `codex-rs/tui/src/chatwidget/adaptive_runtime_bridge.rs`
- `codex-rs/tui/src/chatwidget/adaptive_signal_transport.rs`
- `codex-rs/tui/src/chatwidget/adaptive_trusted_signal.rs`
- `codex-rs/app-server-protocol/schema/typescript/v2/ThreadAdaptiveWorkflowState.ts`

Fourteen of these anchors are byte-identical to the 0.3.13 production versions.

Two contain narrow 0.159.2 compatibility changes:

1. `adaptive_classification.rs`

   - classifies the new `FlexUnavailable` error as transient infrastructure;
   - classifies the new `TooManyDenials` error as authorization/scope;
   - otherwise preserves the existing CodexDD classification behavior.

2. `adaptive_trusted_signal.rs`
   - carries the new upstream `sandbox_type: None` test-fixture field required by the 0.159.2 structure;
   - the trusted-signal runtime behavior remains otherwise unchanged from production.

### Wiring result

The preserved modules remain connected to runtime surfaces:

- `tui/src/lib.rs` declares the CodexDD adaptive budget, classification, complexity, controller, evidence, policy, and Worker modules.
- `tui/src/chatwidget.rs` declares adaptive admission, effort, evidence, runtime bridge, signal transport, and trusted-signal modules.
- `core/src/tools/handlers/mod.rs` exports `AdaptiveSignalHandler`.
- `core/src/tools/spec_plan.rs` registers `AdaptiveSignalHandler` with `ToolExposure::DirectModelOnly`.
- `core/src/tools/registry.rs` retains the reconnaissance, mechanical-validation, and validation-terminalization execution gates around `report_adaptive_signal`.
- `app-server-protocol/src/protocol/v2/thread.rs` retains `ThreadAdaptiveWorkflowState` and `SetAdaptiveState`.
- The generated TypeScript v2 index exports `ThreadAdaptiveWorkflowState`.
- Trusted-signal consumption still latches `ReadyForValidation`, `RepairRequired`, and `ReadyForOwnerQa`, and persists the owner-QA workflow state.

No 3F.1 repair was required.

## Remaining Phase 3F packets

- **3F.2 - lifecycle/protocol persistence audit**

  - resume/fork/new-session lifecycle;
  - per-thread adaptive state persistence;
  - interruption/recovery and terminal ordering;
  - app-server notification and workflow-state transport.

- **3F.3 - model/status/version/generated-surface audit**

  - GPT-6 Luna/Sol/Astra routing contract;
  - `/status` adaptive fields;
  - version/provenance identity;
  - generated protocol exports and integration-guide behavior.

- **3F.4 - full implementation closure**
  - reconcile the complete CodexDD customization inventory against the finished 0.159.2 tree;
  - run the targeted local compile/test gate for Phase 3F;
  - fix only 3F defects;
  - close Phase 3F only after the audit and local validation gates are green.

Phase 4 remains separate and owns the broader pre-install validation matrix.

## 3F.2 - lifecycle/protocol persistence audit

**Status: IMPLEMENTED; local validation pending.**

The audit confirmed that the canonical CodexDD workflow-state pipeline survived the 0.159.2 integration:

- TUI adaptive mutations emit `PersistWorkflowState` with `SetAdaptiveState`, `SetReadyForOwnerQa`, or `Clear`.
- The app event dispatcher routes the mutation through `thread/workflowState/update`.
- The app server maps the v2 payload to `codex_history::WorkflowStateItem`.
- `CodexThread::persist_workflow_state` appends and flushes the workflow mutation into the exact thread rollout.
- Resume restores the latest supported workflow-state record and clears ephemeral successor authority.
- Unsupported workflow-state schema versions fail closed.
- Core fork creation copies the latest workflow-state snapshot into the child rollout.
- Trusted adaptive runtime signals are retained in the app-server turn summary and replayed immediately before `turn/completed`, preserving terminal ordering across notification races.
- Manual adaptive interruption persists the paused state and clears pending signal/successor authority.
- Fresh Worker creation resets transient adaptive lifecycle state while preserving the intended worker binding.

### 3F.2 defect found and repaired

The audit found one detached-fork restoration gap.

Core correctly writes the inherited workflow snapshot into a newly forked child rollout, but the TUI mapped `ThreadForkResponse` with `WorkflowStateRestoreMode::FreshThread`. That meant a fork created when its parent was not already attached to the current TUI process could receive a default in-memory adaptive state even though the child rollout already contained the inherited canonical snapshot.

The repair changes fork-response mapping to restore workflow state from the child rollout as an existing persisted thread. In-memory parent inheritance remains as a fallback for cases where no persisted snapshot is available.

Repair commits:

- `74d352646a451b48d7a224d208eee0aa52eb0db7` - restore adaptive state on detached forks.
- `d86dcad06e6814216c03b62859080e80aad170f6` - add a regression test proving a fork restores the persisted adaptive route, complexity class, implementation phase, Worker binding, attempt number, and clears ephemeral successor authority.

3F.2 remains open only for the targeted Daniel-CL validation gate.

## 3F.3 - model/status/version/generated-surface audit

**Status: AUDIT COMPLETE; local validation pending.**

The source audit found no additional repair requirement.

- Adaptive family mapping remains:
  - Luna -> `gpt-6-luna`
  - Sol -> `gpt-6-sol`
  - Astra -> `gpt-6-astra`
- The 0.159.2 model catalog contains all three families and exposes the reasoning levels required by CodexDD. Astra also exposes `xhigh` and `max`.
- The dedicated regression test continues to assert that 0.3.14 does not migrate Sol to `gpt-6.1-sol`.
- `/status` still includes the CodexDD adaptive route, budget mode, complexity, implementation phase/floor, attempt/failure pressure, Worker role/scope/binding, and workflow terminal.
- Product identity is `codexdd 0.3.14`; tracked upstream identity is `rust-v0.159.2`.
- The generated v2 JSON schema, TypeScript bindings, and Python SDK contain `AdaptiveRuntimeSignal`, `ThreadAdaptiveWorkflowState`, and `thread/workflowState/update`.
- The 0.3.7 Worker/Designer integration guide remains compatible with the preserved lifecycle contract.

3F.3 remains open only for its targeted local regression checks.

## Updated remaining Phase 3F work

- **3F.2 local validation**

  - detached-fork adaptive restore regression;
  - lifecycle persistence/terminal-ordering targeted tests.

- **3F.3 local validation**

  - model-family/catalog regression;
  - adaptive `/status` regression.

- **3F.4 - full implementation closure**
  - reconcile the complete CodexDD customization inventory against the finished 0.159.2 tree;
  - run the final targeted Phase 3F compile/test gate;
  - repair any defects found;
  - close Phase 3F only when the audit and local validation gates are green.

Phase 4 remains separate and owns the broader pre-install validation matrix.

## 3F.4 - exact customization-tree reconciliation

**Status: SOURCE AUDIT COMPLETE; local validation pending.**

The final source audit used exact recursive Git tree/blob comparison rather than the GitHub compare-file cap.

Tree anchors:

- previous tracked upstream tree: `0a0b0bf9acf2f45e9239e7a52ef79f7702aa5799`
- 0.3.13 production tree: `03fd53c7249d371aa795d92022f1a72150addcf0`
- target 0.159.2 upstream tree: `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361`
- audited 0.3.14 feature tree before the local gate: `9be7141bb7e273a3f06caff79aad9030c490c4bf`

The exact counts reconfirm the Phase 3A inventory:

- 214 CodexDD customization paths versus the previous tracked upstream.
- 2233 upstream paths changed from 0.156.1 to 0.159.2.
- 209 of the 214 prior customization paths still differ from target upstream in the finished feature tree.
- 5 prior customization paths intentionally no longer differ from target upstream.
- 7 additional 0.3.14 feature-only paths are present for integration repairs, regressions, snapshots, and durable audit documentation.

### Five intentional non-differences

The five prior customization paths that now match target upstream are accounted for; none is an unexplained silent drop.

1. `.github/workflows/rust-release-prepare.yml`
2. `.github/workflows/rust-release.yml`
3. `defs.bzl`

These three are Phase 3E build/release surfaces intentionally realigned to the 0.159.2 upstream structure. CodexDD-specific provenance is carried by the separate product/upstream identity files and repo-check logic instead of preserving obsolete release-file drift.

4. `codex-rs/core/src/agent/control_tests.rs`

The old CodexDD delta was an exhaustive-match carryover adding `RolloutItem::WorkflowState(_)` to a helper that no longer exists in 0.159.2. The upstream 0.159.2 test structure replaced that helper, so there is no equivalent exhaustive match requiring a CodexDD arm.

5. `codex-rs/rollout/src/model_context.rs`

The old CodexDD delta added `WorkflowState` to an exhaustive scan match. Upstream 0.159.2 rewrote `ModelContextScan` so it only special-cases compaction boundaries and otherwise carries rollout items generically. `WorkflowState` therefore flows through without a dedicated arm; retaining the old match edit would reintroduce obsolete 0.156.1 structure.

These two code paths are the two obsolete 0.156.1-only exhaustive-match carryovers called out by the Phase 3B integration checkpoint.

### Seven deliberate feature-only paths

The feature tree contains seven paths that were not part of the prior 214-path customization set:

- `codex-rs/history/src/compaction_resume_metadata.rs` - 0.159.2 resume-metadata integration with explicit `WorkflowState` handling.
- `codex-rs/tui/src/app/tests/model_catalog.rs` - 0.3.14 regression that locks Luna/Sol/Astra model-family and reasoning support.
- `codex-rs/tui/src/status/snapshots/codex_tui__status__tests__status_snapshot_local_background_server.snap` - restored CodexDD adaptive status surface on the 0.159.2 snapshot.
- `docs/codexdd-0.3.14-phase-3b-validation.md`
- `docs/codexdd-0.3.14-phase-3f-audit.md`
- `docs/codexdd-0.3.14-upstream-integration-scaffold.md`
- `docs/codexdd-major-phase-work-packet-standard.md`

No CodexDD-only path is silently missing from the finished tree.

### Windows fork-test stack handling

The first detached-fork regression run on Daniel-CL ended with `STATUS_STACK_OVERFLOW` under the default Rust test-thread stack.

That result is not yet classified as a runtime defect. The project-wide validation standard and the Phase 3B record already establish that heavy Windows resume/fork tests can require:

```powershell
$env:RUST_MIN_STACK = "16777216"
```

The unchanged detached-fork test must be rerun with that environment variable before the 3F.2 repair is accepted or rejected. A speculative production stack workaround was briefly evaluated and then removed so the source tree again contains only the detached-fork behavior repair plus its regression.

## Phase 3F closure

**Status: COMPLETE.**

All four targeted Daniel-CL gates passed against the completed 0.3.14 feature tree:

1. `detached_fork_restores_persisted_adaptive_workflow_state` - PASS with the established Windows `RUST_MIN_STACK=16777216` test-stack setting;
2. `adaptive_runtime_signal_binds_event_turn_and_conversation_identity` - PASS;
3. `codexdd_adaptive_families_match_0159_catalog_and_reasoning_support` - PASS;
4. `status_output_includes_codexdd_adaptive_route_and_worker_state` - PASS.

The detached-fork default-stack failure was therefore confirmed as the already-documented Windows Rust test-harness stack sensitivity rather than a production runtime defect.

Phase 3F is closed. The full 214-path customization inventory is accounted for, the detached-fork persistence defect found during the audit is repaired, and the targeted lifecycle/model/status gates are green.

The next release stage is **Phase 4 - robust pre-install validation**.
