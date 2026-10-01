# CodexDD 0.3.14 upstream integration scaffold

This file is the Phase 3A checkpoint for the CodexDD 0.3.14 integration of OpenAI Codex `rust-v0.159.2`.

## Immutable integration anchors

- CodexDD production/base SHA: `665f5d7c7966f9a6a9570b5b85f9d68c529c1639`
- Existing feature branch: `dd/codexdd-v0.3.14-upstream-0.159.2`
- Previous tracked upstream: `rust-v0.156.1`
- Previous tracked upstream commit: `b412ff32c417f855c2b2d1581b77058eed87c84b`
- Target upstream: `rust-v0.159.2`
- Target upstream commit: `ff6aec96948b70d94983af2641a6b67c94faeff5`
- Target upstream tree: `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361`
- Platform target: Windows only.

Exact Git tree comparison reconfirmed the review counts:

- CodexDD customization delta versus `rust-v0.156.1`: **214 paths**
- Upstream `rust-v0.156.1 -> rust-v0.159.2`: **2233 paths**
- Paths changed by both sides: **107 paths**

The 107 entries below are overlap candidates, not a claim that all 107 produce textual merge conflicts. They are the mandatory manual-review set. No blanket ours/theirs resolution is permitted.

## Integration rules

1. Upstream `rust-v0.159.2` is the structural source of truth.
2. CodexDD-specific behavior must be restored deliberately onto the new upstream structure.
3. Do not resolve overlapping paths by wholesale ours/theirs selection.
4. Preserve the existing CodexDD adaptive lifecycle, Worker authority, trusted-signal, complexity, budget, validation-handoff, terminal-ordering, resume/fork, and interruption-recovery behavior.
5. Preserve the 0.3.14 model-family policy:
   - Luna -> `gpt-6-luna`
   - Sol -> `gpt-6-sol`
   - Astra -> `gpt-6-astra`
   - Do not migrate Sol to GPT-6.1 Sol in this upstream release.
6. Adopt the upstream 0.159.2 `/status` structure/layout, then restore CodexDD route, budget, complexity, Worker, and lifecycle state.
7. Keep upstream synchronization fail-closed. Detailed conflict diagnostics must ultimately be written to `GITHUB_STEP_SUMMARY`; GitHub Issues are not a required diagnostic channel.
8. Version/provenance changes are deferred to Phase 3E. Phase 3A intentionally does not change `codex-rs/codexdd-version.txt`, `codex-rs/upstream-codex-release.txt`, Cargo workspace provenance, or version-reporting expectations.
9. No PR, CI, production install, or Breakwater soak is authorized by this checkpoint.

## Phase ownership of the 107 overlap candidates

The partition below covers all 107 overlap candidates exactly once. It is an implementation ownership map, not an automatic conflict-resolution policy.

### Phase 3B - core/runtime conflicts (59)

**Status: COMPLETE.** Work packets 3B.1, 3B.2, and 3B.3 are closed. See `docs/codexdd-0.3.14-phase-3b-validation.md` for the validation record. Phase 3C must begin as a new numbered work packet.

- `codex-rs/app-server-protocol/schema/json/ClientRequest.json`
- `codex-rs/app-server-protocol/schema/json/ServerNotification.json`
- `codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.schemas.json`
- `codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json`
- `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst`
- `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst`
- `codex-rs/app-server-protocol/schema/typescript/ClientRequest.ts`
- `codex-rs/app-server-protocol/schema/typescript/ServerNotification.ts`
- `codex-rs/app-server-protocol/schema/typescript/ServerNotificationEnvelope.ts`
- `codex-rs/app-server-protocol/schema/typescript/v2/index.ts`
- `codex-rs/app-server-protocol/src/protocol/common.rs`
- `codex-rs/app-server-protocol/src/protocol/thread_history.rs`
- `codex-rs/app-server-protocol/src/protocol/thread_history_projection.rs`
- `codex-rs/app-server-protocol/src/protocol/v2/thread.rs`
- `codex-rs/app-server/src/bespoke_event_handling.rs`
- `codex-rs/app-server/src/message_processor.rs`
- `codex-rs/app-server/src/notification_media.rs`
- `codex-rs/app-server/src/request_processors.rs`
- `codex-rs/app-server/src/request_processors/turn_processor.rs`
- `codex-rs/cli/src/main.rs`
- `codex-rs/config/src/config_toml.rs`
- `codex-rs/core/config.schema.json`
- `codex-rs/core/src/agent/control/spawn.rs`
- `codex-rs/core/src/agent/control_tests.rs`
- `codex-rs/core/src/client.rs`
- `codex-rs/core/src/client_tests.rs`
- `codex-rs/core/src/codex_thread.rs`
- `codex-rs/core/src/config/mod.rs`
- `codex-rs/core/src/context_manager/history_tests.rs`
- `codex-rs/core/src/session/rollout_reconstruction.rs`
- `codex-rs/core/src/session/rollout_reconstruction_tests.rs`
- `codex-rs/core/src/session/session.rs`
- `codex-rs/core/src/session/tests.rs`
- `codex-rs/core/src/session/turn.rs`
- `codex-rs/core/src/thread_manager.rs`
- `codex-rs/core/src/thread_manager_tests.rs`
- `codex-rs/core/src/tools/handlers/mod.rs`
- `codex-rs/core/src/tools/registry.rs`
- `codex-rs/core/src/tools/registry_tests.rs`
- `codex-rs/core/src/tools/spec_plan.rs`
- `codex-rs/core/tests/suite/agent_websocket.rs`
- `codex-rs/core/tests/suite/unified_exec.rs`
- `codex-rs/history/src/lib.rs`
- `codex-rs/history/src/rollout_payload.rs`
- `codex-rs/history/src/tests.rs`
- `codex-rs/protocol/src/protocol.rs`
- `codex-rs/rollout/src/lib.rs`
- `codex-rs/rollout/src/list.rs`
- `codex-rs/rollout/src/metadata.rs`
- `codex-rs/rollout/src/model_context.rs`
- `codex-rs/rollout/src/recorder.rs`
- `codex-rs/rollout/src/search.rs`
- `codex-rs/state/src/extract.rs`
- `codex-rs/state/src/runtime/threads.rs`
- `codex-rs/thread-store/src/local/rollout_migration/rollback_plan.rs`
- `codex-rs/thread-store/src/thread_metadata_sync.rs`
- `codex-rs/thread-store/src/types.rs`
- `sdk/python/src/openai_codex/generated/notification_registry.py`
- `sdk/python/src/openai_codex/generated/v2_all.py`

### Phase 3C - TUI/adaptive lifecycle conflicts (39)

- `codex-rs/tui/src/app.rs`
- `codex-rs/tui/src/app/app_server_event_targets.rs`
- `codex-rs/tui/src/app/background_requests.rs`
- `codex-rs/tui/src/app/config_persistence.rs`
- `codex-rs/tui/src/app/event_dispatch.rs`
- `codex-rs/tui/src/app/owned_transcript_tests.rs`
- `codex-rs/tui/src/app/session_lifecycle.rs`
- `codex-rs/tui/src/app/startup.rs`
- `codex-rs/tui/src/app/test_support.rs`
- `codex-rs/tui/src/app/tests.rs`
- `codex-rs/tui/src/app/tests/disconnect_tests.rs`
- `codex-rs/tui/src/app/tests/rate_limits.rs`
- `codex-rs/tui/src/app/tests/session_lifecycle_requests.rs`
- `codex-rs/tui/src/app/tests/startup_defaults_tests.rs`
- `codex-rs/tui/src/app/thread_routing.rs`
- `codex-rs/tui/src/app_event.rs`
- `codex-rs/tui/src/app_server_session.rs`
- `codex-rs/tui/src/chatwidget.rs`
- `codex-rs/tui/src/chatwidget/completion.rs`
- `codex-rs/tui/src/chatwidget/constructor.rs`
- `codex-rs/tui/src/chatwidget/interaction.rs`
- `codex-rs/tui/src/chatwidget/protocol.rs`
- `codex-rs/tui/src/chatwidget/rate_limits.rs`
- `codex-rs/tui/src/chatwidget/session_flow.rs`
- `codex-rs/tui/src/chatwidget/slash_dispatch.rs`
- `codex-rs/tui/src/chatwidget/tests.rs`
- `codex-rs/tui/src/chatwidget/tests/app_server.rs`
- `codex-rs/tui/src/chatwidget/tests/completion_styling_tests.rs`
- `codex-rs/tui/src/chatwidget/tests/composer_submission.rs`
- `codex-rs/tui/src/chatwidget/tests/exec_flow.rs`
- `codex-rs/tui/src/chatwidget/tests/misalignment_policy_tests.rs`
- `codex-rs/tui/src/chatwidget/tests/plan_mode.rs`
- `codex-rs/tui/src/chatwidget/tests/slash_commands.rs`
- `codex-rs/tui/src/history_cell/separators.rs`
- `codex-rs/tui/src/history_cell/separators_tests.rs`
- `codex-rs/tui/src/history_cell/tests.rs`
- `codex-rs/tui/src/lib.rs`
- `codex-rs/tui/src/slash_command.rs`
- `codex-rs/tui/src/startup_orchestration.rs`

### Phase 3D - /status + model/catalog integration (4 direct overlap paths)

- `codex-rs/tui/src/chatwidget/tests/status_and_layout.rs`
- `codex-rs/tui/src/status/card.rs`
- `codex-rs/tui/src/status/snapshots/codex_tui__status__tests__status_snapshot_uses_default_reasoning_when_config_empty.snap`
- `codex-rs/tui/src/status/tests.rs`

Phase 3D must also audit CodexDD-owned model/catalog and adaptive routing paths that do not appear in the overlap set. A non-overlap does not mean the behavior can be ignored when upstream call sites or data structures moved.

### Phase 3E - version/provenance + upstream-sync/build repair (5 overlap paths)

- `.github/workflows/repo-checks.yml`
- `.github/workflows/rust-release-prepare.yml`
- `.github/workflows/rust-release.yml`
- `codex-rs/Cargo.lock`
- `defs.bzl`

Phase 3E also owns the CodexDD-only upstream-sync and provenance anchors, including:

- `.github/scripts/upstream_sync.py`
- `.github/scripts/test_upstream_sync.py`
- `codex-rs/codexdd-version.txt`
- `codex-rs/upstream-codex-release.txt`
- `codex-rs/cli/tests/version_reporting.rs`

### Phase 3F - full GitHub implementation audit

Phase 3F must verify the complete 214-path CodexDD delta against the finished 0.159.2 tree, including CodexDD-only adaptive modules that are not overlap candidates. In particular, the audit must prove that the adaptive modules, trusted signal tooling, lifecycle persistence, version identity, generated protocol surfaces, and integration-guide behavior were not silently dropped.

## CodexDD-owned anchors that require preservation checks

Important CodexDD-only paths include, but are not limited to:

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

## Phase 3A exit condition

Phase 3A is complete when this scaffold is committed to the existing 0.3.14 feature branch and the branch provenance/counts above are verified. No source conflict is considered resolved by this commit.

The next authorized implementation slice is **Phase 3B only**.
