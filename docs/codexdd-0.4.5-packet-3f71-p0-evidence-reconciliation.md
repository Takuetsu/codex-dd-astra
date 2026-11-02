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

## Event-queue P0 design-review finding (source verified against 0.4.5 PR #78)

The current fork's `ThreadEventChannel` in `tui/src/app/thread_events.rs` creates `mpsc::unbounded_channel()` even when `ThreadEventStore::new(capacity)` is bounded. `tui/src/app/thread_routing.rs` sends notification, request and history events to the live sender, and the active receiver lives in `App.active_thread_rx` (`tui/src/app.rs`). Both normal UI handling and `drain_active_thread_events_until` consume this receiver. Consequently:

- **Confirmed loss of memory backpressure:** event-count and replay-delta trimming in `ThreadEventStore` do not constrain pending live events. History batch responses and realtime notifications can remain queued while a frame or modal is delayed. This is a present semantic conflict, not merely a test gap.
- **Upstream fallback is not an acceptable blind revert:** upstream `rust-v0.163.0` used a bounded `mpsc::channel(capacity)` with `try_send`, then spawned independent `send().await` tasks when full. Those overflow tasks can overtake later direct sends and reorder a trusted `AdaptiveRuntimeSignal`, approval, or `TurnCompleted` relative to the source stream. Separate overflow tasks may themselves accumulate while the renderer stalls.
- **Naive `sender.send(event).await` is unsafe here:** notification and request enqueue methods execute on the same `App` task that owns the active thread receiver. Waiting for a full bounded channel without pumping that receiver can deadlock the UI, particularly during startup buffering or selection/replay.
- **Inactive threads:** `enqueue_thread_notification` currently sends even when the store is inactive; their snapshot stores have separate bounded retention. Any correction needs to preserve switch/replay and protected modal request ownership. It must not silently discard unconfirmed approvals or trusted terminal signals.
- **Disconnect/voice/shutdown:** the source also has an asynchronous failed-interrupt sender, and current lifecycle uses `ThreadClosed` to fail over to the primary thread or complete a shutdown. FIFO must span these paths.

### Required negative test matrix for the implementation packet

1. Saturate the active queue with more than its limit of normal output; prove bounded memory/event count and uninterrupted frame progress.
2. Enqueue a trusted `AdaptiveRuntimeSignal`, `TurnCompleted`, and an approval under saturation; assert strict source FIFO, identity, exactly-once delivery and no permission bypass.
3. Saturate with a large history batch and sustained realtime/voice audio; assert payload budget (not just event count) while retaining required playback, stop, and completion behavior.
4. Detach, switch, and reattach a side thread while backlog exists; ensure no stale approval is replayed as actionable and foreground input remains intact.
5. Close/shutdown and reinitialize the receiver while saturated; ensure no worker/task retention or silent deadlock.

**Implementation authority:** keep the P0 hold open until the new bounded source path and actual native Windows regressions pass on Daniel-CL. Source documentation alone does not close it.

## Next executable behavior work packet

1. **3F.73: TUI bounded, ordered live-channel repair**. Preserve source-order and delivery of trusted terminals, approvals and turn completions under saturation. An upstream-style bounded channel plus independently spawned send tasks is **not** an acceptable replacement without an ordering regression; merely bounding replay retention is not sufficient.
2. Cheap local `targeted` validation on Daniel-CL first, compiling the affected TUI channel and running true negative saturated-event cases. Do not request broad CI before a local source-bound pass; do not use Windows CI as an iterative debugging loop.
3. **3F.74: Core trust/permission negative matrix**. Pair model tool-choice, phase-gated signal/LVO, pre-tool-hook rewrite, fork inheritance and history reconstitution with boundary-negative tests, not only source text checks.
4. Then perform explicit accountable owner reviews of all thirteen cases. **No `OWNER_ACCEPTED` change may be set merely because this document or one native suite is green.**

**Guardrails:** Production remains 0.4.4; 109/109 source comparisons do not imply 109/109 semantic signoffs. Seven-program final release archive, signing, voice DLL closure, installer and the Daniel-CL 21-stage gate remain separate.
