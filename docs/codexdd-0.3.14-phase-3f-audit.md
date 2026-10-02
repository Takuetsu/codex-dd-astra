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
