# CodexDD 0.4.3 packet 1A.2 — complete upstream change/overlap reconnaissance

**Status:** COMPLETE — read-only inventory and semantic hazard assessment; Gate 1 **NOT ACCEPTED**.  
**Date:** 2026-10-09.  
**Scope:** official Codex `rust-v0.159.2` → `rust-v0.162.0` complete source delta against the then-live CodexDD production baseline, plus intervening release snapshots, Windows/build/generated/security compatibility surfaces. This packet does **not** prepare/transplant upstream, edit runtime, create a candidate or PR, or run/claim Windows validation.  
**Documentation branch:** `dd/codexdd-v0.4.3-phase1-recon`.

## Immutable comparison inputs

| Ref | Commit SHA | Tree SHA |
| --- | --- | --- |
| CodexDD production `dd/astra-policy-v2` | `55139187f31d044d9f413245c38ba2da058cab2e` | `614e1e9cfb1b9749f3a57fa21f48848d7d7d7153` |
| Official tracked `rust-v0.159.2` | `ff6aec96948b70d94983af2641a6b67c94faeff5` | `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361` |
| Official target `rust-v0.162.0` | `c1382380de69521303b416720a52f42d51af6248` | `4899ef4940a8bc7fdf7873aa0d3f438085b8163f` |

**Authoritative full path evidence:** [packet 1A.2 exact tree inventory](codexdd-0.4.3-packet-1a2-tree-inventory.json). This committed JSON file captures every customization path, every upstream-changed path, every exact overlap, the 0.4.2 classifier's overlapping sensitive-category sets and the 12 paths not covered by its patterns. **It is reconnaissance data, not a candidate preparation manifest or validation receipt.**

The calculation independently enumerated nontruncated recursive Git trees and compared each leaf's `mode type object-id` identity, reproducing the algorithm in `.github/scripts/upstream_sync.py`. Root tree leaf counts: tracked **8,703**, fork production **8,778**, target **9,017**. No tree result was truncated.

## Exact current-base path inventory

| Inventory | Count | Added | Removed | Modified |
| --- | ---: | ---: | ---: | ---: |
| Fork customizations vs tracked | **256** | 78 | 3 | 175 |
| Target upstream vs tracked | **2,507** | 410 | 96 | 2,001 |
| Exact paths changed in both | **136** | — | — | — |
| Policy-classified sensitive overlap paths | **124** | — | — | — |
| Policy-unclassified overlap paths | **12** | — | — | — |

The previous discovery manifest counted 255 customization paths at a production snapshot predating the newly committed 0.4.3 design. This current-base calculation counts **256**, while the upstream diff/overlap counts remain **2,507 / 136**. Counts are for the pinned 1A.2 base; all future candidate preparation must recheck the live production SHA and tree. The classifier's categories overlap and must **not** be added together as disjoint counts.

### Subsystem sizing (upstream changed / exact CodexDD overlap)

| Surface | Upstream paths | Overlaps |
| --- | ---: | ---: |
| `codex-rs/tui/` | 882 | **65** |
| `codex-rs/core/` | 520 | **22** |
| `codex-rs/app-server/` | 105 | **6** |
| `codex-rs/app-server-daemon/` | 24 | **3** |
| `codex-rs/app-server-protocol/schema/` | 69 | **10** |
| `sdk/python/src/openai_codex/generated/` | 2 | **2** |
| `codex-rs/windows-sandbox-rs/` | 52 | **0** |
| `codex-rs/windows-sandbox-service/` | 20 | **0** |
| `codex-rs/**/Cargo.toml` and `codex-rs/Cargo.lock` | 44 | **1** |
| `.github/workflows/` | 7 | **1** |

A zero path overlap **is not** a zero integration-risk finding: Windows sandbox/security changes, CI and target dependency or API changes may affect neighboring CodexDD-owned code. Do not skip non-overlap compatibility review.

### Sensitive classification breakdown (not mutually exclusive)

`tui_runtime` 65; `core_runtime` 34; `app_server_runtime` 11; `generated_protocol` 10; `worker_lifecycle` 6; `persistence_resume_fork` 17; `status_rendering` 5; `configuration` 3; `build_release_ci` 2; `adaptive_routing` 1. The planner's `local_validation_orchestrator` and `version_provenance` predicates match **zero direct overlap paths**, which is **not** proof their contracts need no review.

**Classifier gap to handle manually:** the 12 "unclassified" overlaps include **three Windows app-server-daemon source files**, two CLI startup files, three core integration-test files, an external-agent migration file, a rollout-trace file, and two generated Python SDK files. In particular, daemon startup/elevation and generated SDK overlap must be treated as **high risk regardless of the classifier label**. Consider tightening classifier scope in a separately accepted work packet if it is necessary for safe 0.4.3 promotion; no planner changes are authorized yet.

## Intervening official stable-release snapshots

Official annotated tags and peeled commits were checked. There is no `rust-v0.161.1` official tag at this inspection, so do not fabricate that release. Edges use exact recursive Git tree identities and count how many existing fork customization paths each upstream interval touched.

| Stable interval | Upstream paths changed in interval | Fork customization paths touched in interval |
| --- | ---: | ---: |
| `0.159.2` → `0.160.0` | 377 | 48 |
| `0.160.0` → `0.160.1` | 2 | 0 |
| `0.160.1` → `0.161.0` | 1,195 | 103 |
| `0.161.0` → `0.162.0` | 1,658 | 111 |

The intervals **must not be summed to infer the net delta** because the same paths can change repeatedly. The authoritative net diff remains 2,507 upstream paths / 136 exact overlaps. Interim 0.160.x and 0.161.x API or behavior changes can be relevant even when their final path outcome is a later 0.162.0 version. The tracked/target release history is reported as **diverged** by the existing discovery planner (merge base `06971ec9aad037d7c32b7466031fbb8b3b407103`). The synthetic-tree-delta transplant contract, not a direct upstream fast-forward or blind Git merge, remains necessary.

## Historical textual conflict evidence — NOT a current-base dry-run result

The successful read-only discovery run [#37936059803](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37936059803) published artifact `upstream-candidate-rust-v0.162.0-37936059803` (ID `11617648968`). That artifact has `state=blocked_transplant_conflict`, a `conflict` dry-run result, **22** unmerged paths and `production_sha=dc10de2b0240ebe51bba009bfa224fc305c9aa6a`. The live production base is now `55139187...`, so the old artifact **fails the stale-manifest check and must not be used for candidate preparation.** A candidate-specific clean or conflicted result must be regenerated against the production SHA current at Gate 2. Until then, the 22 entries below are a **historical conflict forecast**, not a fresh exact-base dry-run assertion.

Historical conflict paths (full file list, 22):
```text
codex-rs/app-server-daemon/src/backend/windows.rs
codex-rs/app-server-daemon/src/lib.rs
codex-rs/app-server-protocol/schema/precomputed/app-server-exports-experimental.json.zst
codex-rs/app-server-protocol/schema/precomputed/app-server-exports-stable.json.zst
codex-rs/app-server-protocol/schema/typescript/ClientRequest.ts
codex-rs/app-server-protocol/schema/typescript/ServerNotification.ts
codex-rs/app-server-protocol/schema/typescript/ServerNotificationEnvelope.ts
codex-rs/config/src/config_toml.rs
codex-rs/core/src/config/mod.rs
codex-rs/rollout/src/policy.rs
codex-rs/tui/src/app/tests/permission_selection_tests.rs
codex-rs/tui/src/app/thread_routing.rs
codex-rs/tui/src/app/thread_session_state.rs
codex-rs/tui/src/app/working_directory.rs
codex-rs/tui/src/app_server_session.rs
codex-rs/tui/src/app_server_session/reasoning_defaults_tests.rs
codex-rs/tui/src/app_server_session/rollout_history.rs
codex-rs/tui/src/chatwidget/slash_dispatch.rs
codex-rs/tui/src/daemon_startup.rs
codex-rs/tui/src/session_start.rs
codex-rs/tui/src/slash_command.rs
codex-rs/tui/src/startup_orchestration.rs
```

## Semantic risk findings from actual baseline, fork and target source reads

### A. P0 — Windows daemon privilege and elevated SSH F2 contract

Upstream 0.162.0 introduces `app-server-daemon::is_elevated()` and modifies `tui/src/startup_orchestration.rs` to mark elevated Windows launch as `no_daemon` **with a user-facing elevated warning**. The fork's PR #48 implementation instead uses `is_current_process_elevated()`, rejects elevated explicit daemon start/restart/bootstrap, treats a successful elevated implicit check as **expected embedded mode without an F2 warning**, and fails closed **with** a diagnostic warning when token elevation cannot be verified. Those are not interchangeable implementations.

- Conflicting sources: `app-server-daemon/src/backend/windows.rs`, `app-server-daemon/src/lib.rs`, `tui/src/daemon_startup.rs`, `tui/src/startup_orchestration.rs`.
- Target upstream also changes the Windows daemon backend's job/process handling. Do not bypass Windows ACL/job/process-breakaway restrictions or silently weaken the detached-launch guard.
- Retain direct regression cases in `tui/src/daemon_startup_tests.rs` and `cli/tests/daemon_startup.rs`: elevated embedded **no warning**; elevation-probe failure **diagnostic warning**; genuine launch failures **visible**; explicit elevation rejected; daemon safety and non-elevated desktop gap independently reported.
- The PR #48 one-time waiver of detached shared-daemon validation **does not apply to 0.4.3**. Release Gate 4 must obtain new evidence or explicit fresh owner decision.

### B. P0 — Core history, rollout retention, adaptive Worker persistence

In `rollout/src/policy.rs`, upstream introduces `persisted_rollout_item`/`into_persisted_rollout_items`, reorganizes event filtering, and adds persisted command/MCP result truncation. The fork adds `RolloutItem::WorkflowState` and `EventMsg::AdaptiveRuntimeSignal` retention. A naïve upstream replacement could silently drop recovery-critical adaptive state or signals.

Upstream `core/src/agent/control/spawn.rs` changes history revisions, subagent analytics and thread residency. The fork still requires `WorkflowState` inclusion in context/rollout selection. Test persisted records through interrupt/resume/fork, detached subagents, worker successor admission and mechanical-validation boundaries. Inspect `core/src/session/*`, `core/src/thread_manager*`, `history/`, `state/`, `thread-store/`, `rollout/` even if a given file is textually nonconflicting.

### C. P0 — Adaptive configuration and TUI workflow routing

`core/src/config/mod.rs` upstream adds runtime refresh, additional model/guardian/daybreak features and cloud binding. CodexDD adds `adaptive_worker` settings that must survive config-layering/reloads. `tui/src/app/thread_routing.rs` upstream changes permission profile resolution and session configuration while CodexDD hooks bind Worker roles, protected turn triggers, terminalization, and session state. `tui/src/app_server_session.rs` upstream adds provider selection, worktree config/attachments; fork adds adaptive workflow-state RPC and Worker context.

Explicitly retain protected reconnaissance-before-implementation and native LVO validation receipts, target Worker/Designer binding, trusted signals, `/status`, 6.1 Sol model tier and persisted adaptive effort. No automatic routing ladder change solely because upstream ships a different catalog.

### D. P1 — App-server/protocol/generated artifact contract

Upstream `app-server/src/thread_state.rs` adds a goal-resume mutex; CodexDD adds `adaptive_runtime_signals`. Upstream protocol v2 thread definitions add `ThreadGoalMutationOrigin`; CodexDD adds `ThreadAdaptiveWorkflowState` and validation receipt/status fields. Merge definitions **and** their consumers, not one side.

The target has **69** changed app-server protocol schema paths and **2** changed generated Python SDK files. **10** schema paths and **2** Python SDK generated paths overlap fork changes. The historical dry run conflicts on two binary `.json.zst` precomputed app-server exports and three generated TypeScript declarations. Use source-of-truth reconciliation then the proper generator/test pipeline: do **not** hand-edit compressed export artifacts or accept stale schemas.

### E. P1 — Rust/CI/toolchain/Windows signing and dependencies

- `codex-rs/rust-toolchain.toml` is **unchanged** at Rust **1.95.0**; edition **2024** and resolver `2` also remain unchanged.
- Official upstream `codex-rs/Cargo.toml` workspace version changes **0.159.2 → 0.162.0**; CodexDD's separately maintained product identity remains **0.4.2** during reconnaissance. Version rewriting occurs only in its approved release-integration packet, never during discovery.
- The full upstream Cargo dependency/lock delta is not optional: upstream adds `agent-message-board-client`, new Windows sandbox test-support workspace package, updates `windows-sys` to **0.61.2**, and pins a newer `rmcp` dependency. Resolve Cargo.lock and package/build consistency *after* source merger.
- `.bazelversion` increases **9.0.0 → 9.2.0**. `.github/workflows/bazel.yml` changes Windows target-shard selection, a direct path overlap. The upstream `rust-ci.yml` and argument-comment-lint toolchain shift to nightly **2026-08-20**; these are **not** authorization to replace repo-specific Windows-authoritative LVO/release test profiles.
- Official upstream Windows release workflow now adds signed `scripts/install/install.ps1` with signature/timestamp verification. Avoid overwriting CodexDD's Windows installer provenance, SSH deployment safety or protected manual approval gates.
- Target includes extensive non-overlap changes in Windows sandbox (52 paths) and Windows sandbox service (20 paths). Perform relevant Windows-target compatibility/security regression tests; non-overlap does not mean no privilege interaction.

### F. P1 — TUI and status/model behavior

Upstream changes **882** TUI paths (65 direct overlaps) across app/session routing, transcript, interactive input and status, while the fork has custom protected work phases, F2 warning behavior, paste/alternate-screen expectations and adaptive status rendering. `tui/src/status/card.rs` target adds a usage URL constant while CodexDD adds adaptive status fields; straightforward textual merges can still change status ordering or snapshots. `tui/src/app/tests/model_catalog.rs` target changes startup test wiring while the fork asserts Luna/Sol/Sol-6.1/Astra catalog/effort support. Preserve its tests and explicit capability validation rather than altering model policy to match new UI defaults.

## Safeguards / validation evidence and limitations

- **Completed now:** official stable target and tracked tag/peeled commit/tree verification (1A.1); *current-production-base* exact nontruncated Git tree inventory (this packet); interim stable snapshot comparison; source-guided risk classification; transparent historical 22-path conflict forecast; checked-in exhaustive JSON evidence.
- **Not completed / not claimed:** live current-base synthetic cherry-pick dry run; updated current-base candidate manifest; line-by-line merge of any overlap; resolved textual/semantic conflicts; regenerated schemas; Windows build/tests; LVO profile PASS; native validation receipt; CI/PR; owner acceptance; release/install.
- The repo's 0.4.2 discovery flow selects newest stable automatically, while 0.4.3 is owner-pinned specifically to `rust-v0.162.0`. The eventual candidate must be fixed to that exact SHA even if newer tags appear.
- No change to product `0.4.2`, upstream provenance `rust-v0.159.2`, source implementation or deployed runtime was made here.

## Packet 1A.2 exit status / handoff

**1A.2 complete** with frozen commit/tree evidence and a comprehensive conflict/risk inventory. No implementation approval is implied.

**Next: packet 1A.3 (still read-only).** Specify deterministic preparation candidate versus development/integration branch ownership, exact source reconciliation owners/3A–3F subpackets, version allocation, generated-artifact order, stale/duplicate handling, manual semantic acceptance evidence, Windows SSH-only testing strategy and release authorization/rollback gates. After 1A.3 is durably checkpointed, present **Phase 1 Gate 1** for an explicit owner acceptance decision. No upstream preparation or source change until then.
