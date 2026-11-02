# CodexDD 0.4.5 — 3F.71 high-risk behavior evidence reconciliation

**NONRELEASE / SOURCE-ONLY checkpoint (2026-11-02).** Candidate based on draft PR #78 after the verified 3F.70 formatting repair, not a production release. Comparison source is upstream `openai/codex@rust-v0.163.0`. No product Rust changes, no owner acceptance, no permission policy waiver.

## Native evidence already available

- [32-selector Windows Core/TUI native suite #41087104607](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41087104607) — success on 2026-10-30; selectors include 14 Core authority/history and 18 TUI paths.
- [25-selector Windows TUI trust suite #41222090944](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41222090944) — success on 2026-10-31 after repairing one explicitly-feature-off test fixture. See `docs/upstream-candidates/codexdd-045-3f68-native-test-cases.json`.
- The 109-path formal owner ledger remains `PENDING_SIGNOFF`, `approved_owner_count=0`, `pending_native_or_semantic_restoration=13`. Previous reports of missing source tests predate follow-up TUI restorations and must not be repeated as if still absent.

## Reconciling the thirteen P0 source holds

| High-risk path | Source and existing native evidence | Remaining review boundary |
| --- | --- | --- |
| `tui/src/app.rs` | Startup input quarantine and rendered-selection early handling are present; 3F.68 native tests exercise startup modal and disconnected input safety. | Full app lifecycle, alternate TUI/daemon paths and source-bound owner decision. |
| `tui/src/app/event_dispatch.rs` | Managed daemon verification and API-key catalog constraint are present; 3F.68 tests exercise daemon menu and popup guard. | Negative race/reconnect/voice flows beyond those selectors. |
| `tui/src/app/config_persistence.rs` | `refresh_in_memory_config_from_disk_updates_live_widget_preferences` is restored and appears in passing 3F.68 native selectors. | Restore/confirm other config persistence contracts and source-bound owner approval. |
| `tui/src/app/tests.rs` | `background_resolution_keeps_matching_foreground_user_input_visible` is restored and a passing 3F.68 selector; source also retains shutdown regression. | Broader request/reconnect assertions and source-bound owner approval. |
| `tui/src/app/thread_routing.rs` | Live notification FIFO and terminal ordering have explicit source tests. | **P0 unresolved**: its `ThreadEventChannel` remains `mpsc::UnboundedSender`/`unbounded_channel()`; replay store's `capacity` is not a bound on the live channel. Design bounded byte/event backpressure **without dropping approval, signal or completion events**; add saturated-event negative tests on real Windows TUI runtime. |
| `tui/src/chatwidget/tests.rs` | `terminal_program_status` test module is registered at lines 304–305; `session_status_follows_existing_lifecycle_state` is in the passing 3F.68 suite. | Other status/lifecycle permutations and source-bound owner approval. |
| `tui/src/chatwidget/tests/slash_commands.rs` | Daybreak API-key feature and Windows Shift+Insert shortcut checks are present in current source; Daybreak unavailable application test ran in 3F.68. | Explicit native Windows image-paste and feature-disabled negative selectors, plus owner decision. |
| `tui/src/app_server_session.rs` | Contains API-key model filtering and forwarded-feature checks; 3F.68 negative popup test explicitly disabled cyber forwarding. | Thread/source-turn ownership and cross-account/cross-thread negative auth tests. |
| `tui/src/chatwidget/settings.rs` | Feature- and account-constrained settings need end-to-end owner review. | No evidence sufficient to grant access-program bypass approval. |
| `core/src/client.rs` | 3F.61 Windows native selector `validation_terminalization_requests_required_tool_choice` green. | Trusted origin and ordinary-turn/spoofed-trigger negatives required. |
| `core/src/tools/mod.rs` | 3F.61 includes registration/phase-bound tool authority negatives; reviewed source has CodexDD tool policy changes. | Verify tool dispatch after pre-tool hooks and all unauthorized phases. |
| `core/src/tools/spec_plan.rs` | 3F.61 includes mechanical-validation-only visibility and signal tool authority selectors. | Explicit stale/foreign source-turn signal and validation-runner negatives. |
| `core/src/thread_manager.rs` | 3F.61 native selector `child_workflow_snapshot_preserves_adaptive_state` green. | Fork/child permissions and pending control authority must not inherit across thread boundaries. |

## Next executable behavior work packet

1. **3F.72: TUI bounded, ordered live-channel repair**. Preserve source-order and delivery of trusted terminals, approvals and turn completions under saturation. An upstream-style bounded channel plus independently spawned send tasks is **not** an acceptable replacement without an ordering regression; merely bounding replay retention is not sufficient.
2. Cheap local `targeted` validation on Daniel-CL first, compiling the affected TUI channel and running true negative saturated-event cases. Do not request broad CI before a local source-bound pass; do not use Windows CI as an iterative debugging loop.
3. **3F.73: Core trust/permission negative matrix**. Pair model tool-choice, phase-gated signal/LVO, pre-tool-hook rewrite, fork inheritance and history reconstitution with boundary-negative tests, not only source text checks.
4. Then perform explicit accountable owner reviews of all thirteen cases. **No `OWNER_ACCEPTED` change may be set merely because this document or one native suite is green.**

**Guardrails:** Production remains 0.4.4; 109/109 source comparisons do not imply 109/109 semantic signoffs. Seven-program final release archive, signing, voice DLL closure, installer and the Daniel-CL 21-stage gate remain separate.
