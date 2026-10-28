# CodexDD 0.4.5 — Packet 3B.4: workflow persistence and upstream thread request routing

**Date:** 2026-10-18  
**Status:** INCOMPLETE / NOT A RELEASE CANDIDATE. Three reviewed app-server RPC/controller source merges plus two byte-exact official upstream companion files checkpointed, with **16 cheap static tests awaiting Daniel-CL validation**. Native Rust compilation and schema/codegen testing still pending.  
**Integration branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Exact source identity

| File under `codex-rs/app-server/src/`  | Tracked official 0.162.0                   | CodexDD 0.4.4                              | Official 0.163.0                           | Merged Git blob                            |
| -------------------------------------- | ------------------------------------------ | ------------------------------------------ | ------------------------------------------ | ------------------------------------------ |
| `message_processor.rs`                 | `6b705c55565ebe3d58c117b38268aea37d05bcd7` | `c4011790aa019b25978cc86fad98d59c4206d88c` | `c9858cd9d513c69a917f6505410ee89b16bdd955` | `d9d1b8403c7bd9bb963f4d3a3b8d24ddf06a393a` |
| `request_processors.rs`                | `3d3a1df9886edbafcb7f9bb787f4924f68e2bdeb` | `da10d7c0781e81034b872292a280cf31b5a5b9c8` | `a40486e0d512af66aeef1c1307696b2740053500` | `5c4633ab48ea77692e28193ea6e128ec58067a37` |
| `request_processors/turn_processor.rs` | `67946c8ebcb0b5f84b9fde7a7f77ad87e2ac38fa` | `8b0825017d7a05793184b98134474a0f03c7c5de` | `1b9a253becc97a7a2c1b0de9e92def7c192691c2` | `1186f0a181704d8ea45243a850f971f1057cb080` |

Two upstream-only companions are staged byte-for-byte from pinned official `rust-v0.163.0` commit `83086188ae0614bd882818ff1aa23f3194962012`, after verifying there were no fork modifications against `rust-v0.162.0`: `request_processors/thread_read_state.rs` at `a64c31b55e7075cf13d8ac8e29e11f4c188096f3` and `message_processor_workspace_routing_tests.rs` at `db478a99602807fd436b121d4c6c46610e365832`.

## Reviewed behavior

The full official 0.163.0 `message_processor.rs` is retained (including the new `ThreadReadStateUpdate` and `ThreadItemsRead` routes, pending-unload map, workspace routing, account/plugin dispatch changes), with the original CodexDD **`ClientRequest::ThreadWorkflowStateUpdate` route** inserted before turn-steer handling. This route forwards its typed payload to the thread's existing workflow-state persistence handler rather than injecting a fake user turn.

The official `request_processors.rs` import/module graph is retained, adding only the fork's three workflow-state types. The official `turn_processor.rs` is retained with its `TurnInput::AnnotatedResponseItem` behavior for explicitly `retain`ed tool outputs; the fork's original **61-line `thread_workflow_state_update` method** is inserted byte-for-byte. It loads the exact requested thread, handles `SetReadyForOwnerQa`, `SetAdaptiveState` and `Clear`, rejects missing `adaptive_state`, preserves native validation/failure fingerprints, worker assignment and authorized scope fields, and persists a `WorkflowStateItem` outside model-visible history. This packet creates **no new permission to set those fields** and does not change the existing access boundary; that requires runtime review.

The new upstream thread-read-state handler and workspace-routing Rust regression are included unchanged. Some their companion core state and thread handler sources remain unintegrated: **do not attempt full Rust release validation on this partial branch**.

## Regression and release gates

`.github/scripts/test_codexdd_045_3b_rpc_routing.py` adds **16 read-only provenance/contract tests** for the five Git blobs, once-only workflow route, upstream read-state and item-read behavior, immutable workflow-state receipt mapping, explicit invalid state rejection, output-retention metadata, workspace routing and original product/version markers.

No native Rust tests, code generation, 21-stage Windows suite or CI release testing have passed for this packet. The branch still has old Cargo.lock and CodexDD 0.4.4 product/tracked-upstream markers. The original **109 overlaps / 15 conflicts** ledger still needs source-complete review, native receipts and owner acceptance. Automatic publisher and draft PR writers remain closed.

**Next cheap checkpoint:** validate 3B.3 (14 tests) + 3B.4 (16 tests) together on the clean detached Daniel-CL worktree. If either test fails, rerun only its affected section after a GitHub source fix.
