# CodexDD 0.4.5 — Packet 1A.2: complete overlap ledger, conflict triage and semantic-risk assessment

**Date:** 2026-10-17  
**Packet:** Phase 1, 1A.2 — **COMPLETE as read-only reconnaissance**; Phase 1 Gate 1 remains **OPEN** until 1A.3 acceptance.  
**Feature branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-automation`  
**Frozen reconnaissance base:** production `b25550dcb2c6e84a84074b603086c9db6f32a63b` (`0.4.4`); upstream `rust-v0.162.0` peeled commit `c1382380de69521303b416720a52f42d51af6248`; owner-pinned target `rust-v0.163.0` peeled commit `83086188ae0614bd882818ff1aa23f3194962012`.

**Scope:** Inventory and risk-classify every path that is changed by both the fork and the target upstream; inspect the direct conflict set against base/fork/upstream source; identify 0.4.4 regressions, generated/binary files, security semantics, and the automation boundary. This **does not** mean 109/109 full semantic reconciliations are accepted. No production or candidate source edits, code compilation, or Windows tests were undertaken.

## Verified inputs and audit method

1. Read the immutable `manifest.json` and `transplant-conflicts.txt` from GitHub scheduled discovery run [38967580282](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38967580282), artifact [12075916353](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38967580282/artifacts/12075916353). The successful scheduled job reports an intentionally blocked conflict transplant, not a prepared branch.
2. Confirmed GitHub production branch HEAD and the merge of [CodexDD 0.4.4 PR #53](https://github.com/Takuetsu/codex-dd-astra/pull/53). Enumerated PR #53's changed filenames and intersected them with all 109 overlap paths, yielding the seven `H` flags below.
3. Read authoritative [0.163.0 release notes](https://github.com/openai/codex/releases/tag/rust-v0.163.0) (published 2026-10-15) and [0.162.1 fixes](https://github.com/openai/codex/releases/tag/rust-v0.162.1) (published 2026-10-09); the 0.162.1 peeled commit is `092d3acd6bec3e3a14bdc7e7a2810ab628ab759d`.
4. Inspected all **13 text-file conflicts** through GitHub source at original upstream `0.162.0`, current CodexDD `0.4.4`, and new upstream `0.163.0`; the remaining two are binary precomputed schemas and must be regenerated, not hand-merged. Compared additional high-risk overlapping `tools/mod.rs`, `spec_plan_tests.rs`, `thread_routing.rs`, `chatwidget/protocol.rs`, `session_lifecycle.rs`, `daemon_startup.rs`, `app_server_session.rs`, `session/rollout_reconstruction.rs`, `session/session.rs`, `tools/handlers/dynamic.rs`, `app-server/request_processors/turn_processor.rs`, and `core/config/mod.rs` across base/fork/target identities.
5. The upstream compare API returned only its first **300 changed-file entries**, whereas the exact recursive tree-diff manifest reports **1,314 upstream-changed paths**. This audit uses the immutable tree manifest, **not** that truncated API list, for complete counts.

## Exact inventory totals

| Metric                                        | Manifest | Interpretation                                                     |
| --------------------------------------------- | -------: | ------------------------------------------------------------------ |
| CodexDD customization paths                   |      356 | Fork-specific change surface against tracked 0.162.0               |
| Official upstream changed paths               |    1,314 | Exact 0.162.0 → 0.163.0 path identities                            |
| Intersecting paths                            |  **109** | Every path appears in the ledger below                             |
| Classified sensitive paths                    |   **97** | Unique-path union, not additive category totals                    |
| Not classified sensitive                      |   **12** | **Not equivalent to safe**; explicit gap in the current classifier |
| Direct synthetic cherry-pick conflicts        |   **15** | 13 text files + 2 compressed generated schema files                |
| Paths independently changed in 0.4.4 (PR #53) |    **7** | All require explicit protection of recently released behavior      |

The base-to-target upstream ancestry is diverged relative to this fork; the manifest's `blocked_transplant_conflict` state is legitimate and authoritative. The 0.4.5 planning document must not be treated as a fresh candidate manifest once production moves.

## Direct conflict triage (15/15)

| Conflicting file(s)                                                        | Observed two-sided change and required disposition                                                                                                                                                                                                                                                                                                                                 | Owning integration packet |
| -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------- |
| `.github/workflows/bazel.yml`                                              | CodexDD adjusted Windows Bazel test critical-path/timeout policy; upstream changed trigger and Windows MSVC config. Reconcile CI intent deliberately, not `ours`/`theirs`.                                                                                                                                                                                                         | 3A build/CI               |
| `codex-rs/Cargo.lock`                                                      | CodexDD 0.4.4 pinned `noyalib-serde-yaml` replacing `serde_yaml` and updated dependencies; upstream 0.163.0 adds lockfile updates. Regenerate reproducibly from accepted manifests and review dependency/security deltas.                                                                                                                                                          | 3A dependencies           |
| Two `schema/precomputed/app-server-exports-*.json.zst`                     | Compressed generated binary blobs differ on both sides. No text-merge or blanket overwrite; run source-of-truth schema export and compare generated identities.                                                                                                                                                                                                                    | 3B generated              |
| `schema/typescript/ServerNotification.ts`, `ServerNotificationEnvelope.ts` | CodexDD adds `AdaptiveRuntimeSignalNotification` and `turn/adaptiveRuntimeSignal`; upstream expands server notifications. Regenerate from merged Rust protocol definitions and assert both upstream and CodexDD notification coverage.                                                                                                                                             | 3B generated              |
| `core/src/context_manager/history_tests.rs`                                | Fork changed history test coverage while upstream added inherited/compaction scenarios. Preserve both targeted regression families and update test expectations.                                                                                                                                                                                                                   | 3C core/history           |
| `core/src/context_manager/normalize.rs`                                    | CodexDD synthesizes abort-output diagnostics for missing custom tool/local shell calls; upstream adds inherited-user-message filtering. Preserve lossless normalization, abort diagnostics, provenance, and history replay.                                                                                                                                                        | 3C core/history           |
| `core/src/tools/registry.rs`                                               | Highest-risk merge: CodexDD protected read-only reconnaissance, no-edit Mechanical Validation, ValidationTerminalization restrictions, command allowlists, LVO visibility, and 0.4.4 Node scripts; upstream adds model/runtime ToolSpec metadata, tool-search information, and cancellation completion controls. Manually reconcile registration and all deny-by-default branches. | 3C tools/safety           |
| `tui/src/app/startup.rs`                                                   | CodexDD adaptive-family startup and resume-permission overrides; upstream changes startup input quarantine, managed daemon and remote status. Preserve safe elevated Windows F2 fallback, Worker binding, and explicit approval semantics.                                                                                                                                         | 3D TUI/startup            |
| `tui/src/app/tests/session_lifecycle_requests.rs`                          | Fork adds adaptive Worker/binding/session tests; upstream revises disconnected WebSocket handling and new model/access cases. Keep both expectations; no test deletion to make an integration green.                                                                                                                                                                               | 3D TUI/tests              |
| `tui/src/app/tests/startup_defaults_tests.rs`                              | A competing function-argument change: fork passes `adaptive_family`, upstream passes `managed_daemon`. Update call sites and tests for **both** fields.                                                                                                                                                                                                                            | 3D TUI/startup            |
| `tui/src/app_server_session/workspace_roots_tests.rs`                      | Fork adds `turn_trigger`; upstream adds remote workspace-root resume/fork test with bounded stack. Preserve both explicit call parameters, remote roots, and turn-trigger propagation.                                                                                                                                                                                             | 3D TUI/session            |
| `tui/src/chatwidget/interaction.rs`                                        | Fork invokes adaptive pause/interrupt on manual Escape; upstream adds keyboard leader, selection/paste helpers, and transition key consumption. Ensure Code Mode cancellation, Esc, Ctrl+V, Shift+Insert, and Worker terminal logic coexist.                                                                                                                                       | 3D TUI/input              |
| `tui/src/lib.rs`                                                           | Fork registers adaptive modules and startup hooks; upstream registers configuration/keymap/startup features and new imports. Reconcile module exposure, initialization and Windows launch modes.                                                                                                                                                                                   | 3D TUI/integration        |

**Explicit binary clarification:** The 13 text-conflict files above and the two `.json.zst` files total 15. Generated TypeScript files are text, but are still **generator-owned**, not acceptable for manual patchwork as a final outcome.

### Lockfile version evidence (three-way, checked before implementation)

The tracked upstream 0.162.0 lockfile, CodexDD production 0.4.4 lockfile, and official upstream 0.163.0 lockfile were each read directly and compared. The critical version differences are:

| Package / dependency   | Tracked 0.162.0 | CodexDD 0.4.4 | Upstream 0.163.0 | Integration requirement                                                                    |
| ---------------------- | --------------- | ------------- | ---------------- | ------------------------------------------------------------------------------------------ |
| `rustls`               | `0.23.45`       | **`0.23.46`** | `0.23.45`        | **Do not downgrade** the fork's applied security update without a separate security review |
| `hyper`                | `1.8.1`         | `1.8.1`       | **`1.11.1`**     | Carry new upstream HTTP behavior and verify network/proxy security fixes                   |
| `rmcp` / `rmcp-macros` | `3.3.0`         | `3.3.0`       | **`3.5.1`**      | Test MCP tool lifecycle, callback/elicitation and parallel-read behavior                   |
| `quinn-proto`          | `0.11.15`       | `0.11.18`     | `0.11.18`        | Keep current resolved version after reproducible regeneration                              |
| `noyalib-serde-yaml`   | absent          | **`0.0.57`**  | absent           | Preserve CodexDD 0.4.4 dependency replacement; never silently revert to legacy serde YAML  |

The generated lockfile also encodes **CodexDD product-version provenance** separately from upstream package versions. Cargo resolution, security advisory checks, Windows build evidence and the upstream 0.163.0 dependency changes must all be reviewed as one accepted dependency packet rather than taking either lockfile wholesale.

## Cross-release 0.4.4 regression gate

The seven intersecting files actually modified in the merged 0.4.4 PR #53 are:

1. `codex-rs/Cargo.lock` — retained YAML security pin and native build graph
2. `codex-rs/Cargo.toml` — `noyalib-serde-yaml =0.0.57` alias and version/resolution behavior
3. `codex-rs/core/src/tools/mod.rs` — protected controller turns force Direct tool mode
4. `codex-rs/core/src/tools/registry.rs` — bounded Node diagnostics in Mechanical Validation with native receipt requirements
5. `codex-rs/core/src/tools/spec_plan_tests.rs` — direct-mode/LVO tool exposure and gating regression tests
6. `codex-rs/tui/src/app/thread_routing.rs` — Worker/turn trigger and binding propagation
7. `codex-rs/tui/src/chatwidget/protocol.rs` — trusted adaptive signal dispatch and native evidence handoff

In particular, upstream `core/src/tools/mod.rs` removed some prior structure without adding a matching protective force-Direct route. The fork's new 0.4.4 `effective_tool_mode` override cannot disappear. Upstream `registry.rs` introduces `OrCancelToolExt`, new tool-search prefixes, and `model_spec` handling; that cannot bypass the CodexDD protected-turn filtering or convert a generic command into native LVO PASS. All seven require regression ownership even when the textual conflict detector reports no conflict.

## Semantic compatibility contracts that are NOT limited to the 15 conflicts

| Upstream behavior (0.162.1 → 0.163.0)                                                 | CodexDD interaction                                                           | Acceptance signal                                                                                                |
| ------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Follow-up steering, instant interruption, and nested Code Mode cancellation           | Bound Worker, adaptive trusted signal, no-edit validation and terminalization | Interrupt never silently completes an unauthorized validation or bypasses a latched BLOCKED state                |
| Parallel independent read-only tools                                                  | Tool registry, allowed commands, provenance                                   | Only genuinely read-only calls parallelize; protected-turn guard and receipt semantics remain intact             |
| Resume permission/network approval restoration                                        | Startup overrides, adaptive state, managed policy, thread state and rollouts  | Explicit managed policy and user overrides take precedence; resumed Worker retains correct binding and approvals |
| Proxy authorization before DNS, duplicate Host rejection, credential alias bypass fix | Core network and sandbox security                                             | Upstream security fixes retained; no bypass via CodexDD policy or config                                         |
| New `/config` and leader-key `/keymap` behavior                                       | TUI model/status/config persistence, Escape interruption                      | Config and shortcut transitions do not corrupt adaptive state or protected input                                 |
| Managed background server `/daemon` restart                                           | Windows elevated/non-elevated shared-daemon controls and embedded fallback    | Existing elevated Windows daemon prohibition remains enforced even with new restart UI                           |
| Windows paste (Ctrl+V, Shift+Insert), side conversations                              | TUI chat widget, active Worker provenance and pending prompts                 | No lost paste, wrong-thread responses, or manual interrupt misrouting                                            |
| Default Daybreak availability / API-key feature gates                                 | CodexDD GPT-6/6.1 routing and catalog                                         | No implicit routing-tier changes or unsupported model preference selections                                      |
| App-server notification/schema changes and retained output                            | Native `turn/adaptiveRuntimeSignal` and workflow state messages               | Generated Rust/TS/Python protocol contracts include both families and pass generation diff checks                |
| 0.162.1 async question formatting and daemon CLI-feature mismatch fixes               | TUI multiline, CLI and daemon startup                                         | Both preceding patch-release fixes survive the full integration                                                  |

The 0.4.4 native Node LVO implementation lives chiefly in non-overlap `core/src/tools/handlers/codexdd_validation.rs`, but its **calls and visibility gates** are in overlapping core tool files. The absence of an overlap in the handler itself does **not** prove integration safety.

## Risk classifier gap: all 12 presently unclassified overlaps

Current `classify_sensitive_overlaps` returns no semantic category for **all twelve** paths marked `U` below. This is only a classifier implementation limitation. Notable missed surfaces:

- `codex-rs/cli/src/main.rs` and `cli/tests/daemon_startup.rs`: live CLI/daemon entry and regression boundary.
- `sdk/python/src/openai_codex/generated/notification_registry.py` and `v2_all.py`: generated protocol compatibility, not just innocuous source changes.
- `codex-rs/memories/write/src/phase1.rs`, `rollout_input.rs`, `rollout-trace/src/protocol_event.rs`, external migration session tests: persistence, replay, and context metadata.
- `codex-rs/core/tests/suite/scenarios.rs` and `unified_exec.rs`: behavioral tests for tool semantics.
- `codex-rs/BUILD.bazel` and `cli/BUILD.bazel`: release/build graph.

**Design consequence:** The 0.4.5 auto-candidate eligibility policy must default to **manual semantic review for _every_ overlapping path**, unless a newly reviewed category-specific mechanical proof exists. Never accept a false-negative as “safe” merely because `sensitive_overlap_count` is less than `overlap_path_count`. Extend classifier coverage and add tests for these twelve path patterns in an implementation packet **only after Gate 1**.

## Auto-update eligibility decision for 0.163.0

**Result:** `BLOCKED_REQUIRES_SEMANTIC_RECONCILIATION`.

The existing six-hour discovery already works. This release is **not eligible** for unattended preparation or a green draft-PR candidate: 15 direct conflicts, 109 custom/upstream overlap paths, and new behavior in the same tool/Worker/state/security interfaces. CodexDD 0.4.5 should support more automatic evidence planning and potentially auto-preparation of future _provably low-risk_ candidates; 0.163.0 itself needs packetized human-reviewed integration. An automatically opened draft PR must not imply accepted Windows proof, review, or release.

## Full 109-path ledger (immutable manifest identities)

The following entries enumerate **every one of the 109 manifest overlap paths**. Ownership is an engineering triage bucket; it is **not** a proof that only that subsystem is affected.

Flags: **C** = direct synthetic transplant conflict; **H** = overlap also modified by 0.4.4 PR #53; **U** = absent from existing sensitive overlap classification (classifier gap); **G** = generator-owned JSON/TS/compressed/Python schema; **S** = other sensitive overlap, semantic review required. Flags may coexist.

| Overlapping path                                                                           | Review ownership                 | Flags |
| ------------------------------------------------------------------------------------------ | -------------------------------- | ----- |
| `.github/workflows/bazel.yml`                                                              | Build / CI / dependencies        | C     |
| `codex-rs/BUILD.bazel`                                                                     | Build / CI / dependencies        | U     |
| `codex-rs/Cargo.lock`                                                                      | Build / CI / dependencies        | C, H  |
| `codex-rs/Cargo.toml`                                                                      | Build / CI / dependencies        | H     |
| `codex-rs/app-server-protocol/schema/json/ClientRequest.json`                              | Generated protocol               | G     |
| `codex-rs/app-server-protocol/schema/json/ServerNotification.json`                         | Generated protocol               | G     |
| `codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.schemas.json`          | Generated protocol               | G     |
| `codex-rs/app-server-protocol/schema/json/codex_app_server_protocol.v2.schemas.json`       | Generated protocol               | G     |
| `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst` | Generated protocol               | C, G  |
| `codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst`       | Generated protocol               | C, G  |
| `codex-rs/app-server-protocol/schema/typescript/ServerNotification.ts`                     | Generated protocol               | C, G  |
| `codex-rs/app-server-protocol/schema/typescript/ServerNotificationEnvelope.ts`             | Generated protocol               | C, G  |
| `codex-rs/app-server-protocol/schema/typescript/v2/index.ts`                               | Generated protocol               | G     |
| `codex-rs/app-server-protocol/src/protocol/common.rs`                                      | Protocol contracts               | S     |
| `codex-rs/app-server-protocol/src/protocol/thread_history.rs`                              | Protocol contracts               | S     |
| `codex-rs/app-server-protocol/src/protocol/v2/thread.rs`                                   | Protocol contracts               | S     |
| `codex-rs/app-server-protocol/src/protocol/v2/turn.rs`                                     | Protocol contracts               | S     |
| `codex-rs/app-server/src/bespoke_event_handling.rs`                                        | App-server lifecycle             | S     |
| `codex-rs/app-server/src/message_processor.rs`                                             | App-server lifecycle             | S     |
| `codex-rs/app-server/src/notification_media.rs`                                            | App-server lifecycle             | S     |
| `codex-rs/app-server/src/request_processors.rs`                                            | App-server lifecycle             | S     |
| `codex-rs/app-server/src/request_processors/turn_processor.rs`                             | App-server lifecycle             | S     |
| `codex-rs/app-server/src/thread_state.rs`                                                  | App-server lifecycle             | S     |
| `codex-rs/cli/BUILD.bazel`                                                                 | CLI / daemon                     | U     |
| `codex-rs/cli/src/main.rs`                                                                 | CLI / daemon                     | U     |
| `codex-rs/cli/tests/daemon_startup.rs`                                                     | CLI / daemon                     | U     |
| `codex-rs/config/src/config_toml.rs`                                                       | Config / policy                  | S     |
| `codex-rs/core/config.schema.json`                                                         | Config / policy                  | S     |
| `codex-rs/core/src/agent/control/spawn.rs`                                                 | Core runtime / tests             | S     |
| `codex-rs/core/src/client.rs`                                                              | Core runtime / tests             | S     |
| `codex-rs/core/src/client_tests.rs`                                                        | Core runtime / tests             | S     |
| `codex-rs/core/src/codex_thread.rs`                                                        | Core runtime / tests             | S     |
| `codex-rs/core/src/config/mod.rs`                                                          | Config / policy                  | S     |
| `codex-rs/core/src/context_manager/history_tests.rs`                                       | History normalization            | C     |
| `codex-rs/core/src/context_manager/normalize.rs`                                           | History normalization            | C     |
| `codex-rs/core/src/session/rollout_reconstruction.rs`                                      | Session / resume                 | S     |
| `codex-rs/core/src/session/rollout_reconstruction_tests.rs`                                | Session / resume                 | S     |
| `codex-rs/core/src/session/session.rs`                                                     | Session / resume                 | S     |
| `codex-rs/core/src/session/tests.rs`                                                       | Session / resume                 | S     |
| `codex-rs/core/src/session/turn.rs`                                                        | Session / resume                 | S     |
| `codex-rs/core/src/thread_manager.rs`                                                      | Session / resume                 | S     |
| `codex-rs/core/src/thread_manager_tests.rs`                                                | Session / resume                 | S     |
| `codex-rs/core/src/tools/handlers/dynamic.rs`                                              | Tool registration / safety       | S     |
| `codex-rs/core/src/tools/mod.rs`                                                           | Tool registration / safety       | H     |
| `codex-rs/core/src/tools/registry.rs`                                                      | Tool registration / safety       | C, H  |
| `codex-rs/core/src/tools/spec_plan.rs`                                                     | Tool registration / safety       | S     |
| `codex-rs/core/src/tools/spec_plan_tests.rs`                                               | Tool registration / safety       | H     |
| `codex-rs/core/tests/suite/scenarios.rs`                                                   | Core runtime / tests             | U     |
| `codex-rs/core/tests/suite/unified_exec.rs`                                                | Core runtime / tests             | U     |
| `codex-rs/external-agent-migration/src/sessions/append_tests.rs`                           | History / protocol / persistence | U     |
| `codex-rs/history/src/lib.rs`                                                              | History / protocol / persistence | S     |
| `codex-rs/history/src/rollout_payload.rs`                                                  | History / protocol / persistence | S     |
| `codex-rs/history/src/tests.rs`                                                            | History / protocol / persistence | S     |
| `codex-rs/memories/write/src/phase1.rs`                                                    | History / protocol / persistence | U     |
| `codex-rs/memories/write/src/rollout_input.rs`                                             | History / protocol / persistence | U     |
| `codex-rs/protocol/src/protocol.rs`                                                        | History / protocol / persistence | S     |
| `codex-rs/rollout-trace/src/protocol_event.rs`                                             | History / protocol / persistence | U     |
| `codex-rs/rollout/src/policy.rs`                                                           | History / protocol / persistence | S     |
| `codex-rs/rollout/src/recorder.rs`                                                         | History / protocol / persistence | S     |
| `codex-rs/rollout/src/search.rs`                                                           | History / protocol / persistence | S     |
| `codex-rs/state/src/extract.rs`                                                            | History / protocol / persistence | S     |
| `codex-rs/state/src/runtime/threads.rs`                                                    | History / protocol / persistence | S     |
| `codex-rs/thread-store/src/types.rs`                                                       | History / protocol / persistence | S     |
| `codex-rs/tui/src/app.rs`                                                                  | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/app_server_event_targets.rs`                                         | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/config_persistence.rs`                                               | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/event_dispatch.rs`                                                   | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/owned_transcript_tests.rs`                                           | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/session_lifecycle.rs`                                                | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/startup.rs`                                                          | TUI / Windows / Worker           | C     |
| `codex-rs/tui/src/app/test_support.rs`                                                     | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/tests.rs`                                                            | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/tests/disconnect_tests.rs`                                           | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/tests/navigation_reconnect_tests.rs`                                 | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/tests/session_lifecycle_requests.rs`                                 | TUI / Windows / Worker           | C     |
| `codex-rs/tui/src/app/tests/startup.rs`                                                    | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app/tests/startup_defaults_tests.rs`                                     | TUI / Windows / Worker           | C     |
| `codex-rs/tui/src/app/thread_routing.rs`                                                   | TUI / Windows / Worker           | H     |
| `codex-rs/tui/src/app_event.rs`                                                            | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app_server_session.rs`                                                   | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/app_server_session/workspace_roots_tests.rs`                             | TUI / Windows / Worker           | C     |
| `codex-rs/tui/src/chatwidget.rs`                                                           | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/constructor.rs`                                               | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/input_flow.rs`                                                | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/input_submission.rs`                                          | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/interaction.rs`                                               | TUI / Windows / Worker           | C     |
| `codex-rs/tui/src/chatwidget/protocol.rs`                                                  | TUI / Windows / Worker           | H     |
| `codex-rs/tui/src/chatwidget/replay.rs`                                                    | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/session_flow.rs`                                              | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/settings.rs`                                                  | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/slash_dispatch.rs`                                            | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/status_controls.rs`                                           | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/tests.rs`                                                     | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/tests/app_server.rs`                                          | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/tests/composer_submission.rs`                                 | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/tests/history_replay.rs`                                      | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/tests/misalignment_policy_tests.rs`                           | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/tests/slash_commands.rs`                                      | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/chatwidget/tests/status_and_layout.rs`                                   | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/cli.rs`                                                                  | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/daemon_startup.rs`                                                       | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/daemon_startup_tests.rs`                                                 | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/history_cell/tests.rs`                                                   | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/lib.rs`                                                                  | TUI / Windows / Worker           | C     |
| `codex-rs/tui/src/slash_command.rs`                                                        | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/startup_orchestration.rs`                                                | TUI / Windows / Worker           | S     |
| `codex-rs/tui/src/status/tests.rs`                                                         | TUI / Windows / Worker           | S     |
| `sdk/python/src/openai_codex/generated/notification_registry.py`                           | Generated protocol               | U, G  |
| `sdk/python/src/openai_codex/generated/v2_all.py`                                          | Generated protocol               | U, G  |

## Handoff — 1A.3 acceptance design / implementation gates

Packet **1A.3** should decide and checkpoint an owner-verifiable integration and automation contract before permitting source edits:

1. **Eligibility and privileges:** The scheduled discovery job stays read-only. The only privileged auto-preparation path is separate, identity-bound, deterministic, deduplicated, restricted to a provably safe allowlist and no semantic overlaps; 0.163.0 must remain blocked unless explicitly reconciled. Evaluate whether auto-draft PR should instead be a report-only issue when native Windows validation is unavailable.
2. **Source ownership:** Divide the 15 conflicts and the generated/binary sources across separate 3A–3D work packets. Require structural reviews for all 109 overlaps, with the seven 0.4.4 protected overlaps treated as mandatory zero-regression gates.
3. **Reproducibility:** Pin official upstream tag object, peeled commit, tree, production base/tree and candidate manifest hash. Re-resolve before candidate creation or dispatch; no arbitrary artifact-driven token escalation. Regenerate Cargo and Rust/TS/Python schema under reviewed pinned tool versions.
4. **Validation cost control:** Cheap relevant tests on Daniel-CL first, targeted native profile once configured, failing section reruns only during repair, final detached/monitored 21-stage Windows release suite once stable, and GitHub CI afterward. Preserve F2, admin daemon restrictions, adaptive routing/Worker, LVO, resume/fork, protocol generation and network security cases.
5. **Consequence boundary:** Product identity `0.4.5` and tracked `rust-v0.163.0` move only after accepted source integration on the feature/candidate branch; merge, release, installs and Stonks/Doug deployments are explicitly owner-controlled.

**Packet 1A.2 exit:** Audited the exact 109/15/12/7 path sets, triaged each direct conflict, verified relevant triple-version source samples and official upstream release behaviors, and recorded the unresolved semantic decisions for 1A.3. No candidate branch or source release has been prepared. **Next packet:** 1A.3 — formally lock guarded auto-update architecture and subsystem work-packet ownership; Phase 1 Gate 1 must then be accepted before implementation.

## References

- [1A.1 reconnaissance](codexdd-0.4.5-packet-1a1-upstream-0.163.0-reconnaissance.md)
- [Manifest and blocked transplant artifact](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38967580282/artifacts/12075916353)
- [Upstream 0.163.0 release notes](https://github.com/openai/codex/releases/tag/rust-v0.163.0)
- [Upstream 0.162.1 release notes](https://github.com/openai/codex/releases/tag/rust-v0.162.1)
- [Merged 0.4.4 source diff](https://github.com/Takuetsu/codex-dd-astra/pull/53)
- [Major-phase work-packet standard](codexdd-major-phase-work-packet-standard.md)
