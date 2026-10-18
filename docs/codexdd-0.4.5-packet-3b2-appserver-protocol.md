# CodexDD 0.4.5 — Packet 3B.2: app-server RPC and workflow-state protocol reconciliation

**Date:** 2026-10-18  
**Status:** INCOMPLETE / NOT A RELEASE CANDIDATE. Four reviewed protocol three-way merges and two exact upstream companion files checkpointed; **14 cheap source tests pending Daniel-CL validation**, and no native Rust compilation or schema regeneration has passed.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Fixed source identities

| File below `codex-rs/app-server-protocol/src/` | Tracked official 0.162.0 | CodexDD 0.4.4 | Pinned official 0.163.0 | Merged Git blob |
| --- | --- | --- | --- | --- |
| `protocol/common.rs` | `3e73b44fffe5a83843727dd500a0baefd0993653` | `82f047ec52f9762e607a8f73db71357ddb3d1ad4` | `b710f18f14e711a748318ee980bcdc4168a7d843` | `6cb1a33ea48ed0de73f420ff6c179d9629fec45a` |
| `protocol/thread_history.rs` | `18b87d9ce485f585a6640f6d9fd49e227b7d351f` | `6941ecc90dc3c364b8029eb420aeb35d413da97f` | `53817bb9e50e87ad8a85f39c6a27037bc87d5081` | `82c980c427501b594389fed439e105c2f696b9a5` |
| `protocol/v2/thread.rs` | `c7f22e2930c39746531798617d2f4595b826078d` | `a18516d0fbae6d74d09eb7586639bd6bc1d0b3a2` | `1746ceb7bcf4ed50ceca643170ffc65dada37f41` | `4180b91565e1a47b38c4f40114716e55b2973d3f` |
| `protocol/v2/turn.rs` | `6b784ada66f5116594ec13224728069c9fca78b0` | `5b646bcb09da65e9752b5f318c9e48aab9b27b36` | `b09a191289349a68e6602a30ec9d887664dab73f` | `04f38d2535ba46da2bc22358589aeb1bd2840499` |

Two disjoint files from official 0.163.0 are copied **byte-for-byte** because the tracked 0.162.0 and production 0.4.4 Git blob for each matched: `export.rs` at `0f90faf920cfcf6565c55548fc4064ba7dcccd96` and `protocol/v2/environment.rs` at `3794780c18b67113374dcf0c9148fdb6e903b6f1`.

## Semantics of the four reviewed merges

- **`common.rs`:** Start from official 0.163.0 and reinsert the exact CodexDD `thread/workflowState/update` request, serialized by `thread_id` rather than a global mutex, plus the `turn/adaptiveRuntimeSignal` notification. Preserve new upstream `thread/readState/update`, `thread/items/read`, `thread/readState/changed` and all existing methods. The workflow mutation is independent from user turn history.
- **`thread_history.rs`:** Preserve the CodexDD `RolloutItem::WorkflowState(_)` history skip alongside official 0.163.0 encrypted and status-bearing response item constructors. The skip prevents this internal workflow state from being added to model-visible history.
- **`v2/thread.rs`:** Reinsert all original CodexDD workflow-state operation, native validation status, thread adaptive state and update request/response types **unchanged** from production. Preserve upstream fork prediction-mode, excluded thread IDs, read state, and bounded read-items request/response types.
- **`v2/turn.rs`:** Reinsert CodexDD's original source-turn-bound adaptive signal envelope and notification types unchanged, while preserving upstream's opt-in `retain` flag for a turn output.

These are schema/source merges only. They **do not** enable a workflow mutation producer, grant new authorization, or change default tool permissions.

## Tests and unresolved integration gates

`.github/scripts/test_codexdd_045_3b_appserver_protocol.py` adds **14 offline source/Git checks** for all six exact blobs, RPC serialization and one-instance notifications, CodexDD workflow skip, upstream encrypted history fields and read-state APIs, unchanged fork type bodies, retained upstream output metadata and untouched release markers.

The upstream code and legacy CodexDD handlers/event converters that consume these types still need semantic integration, targeted native Rust execution and **schema regeneration** from a fully reviewed combined source. Cargo.lock remains the original 0.4.4 graph; 109 original overlap paths, 15 original conflicts and source-bound reconciliation evidence remain open. These counts are the original discovery ledger, not a claim that no individual source files have been staged.

**Next test checkpoint:** execute this 3B.2 source suite (14) together with the 3C.3 registry policy suite (16) in the same clean Daniel-CL validation worktree, then continue app-server handler/event reconciliation. Do not run the long 21-stage Windows release suite, enable the separate auto-publishing writer, create PRs, or advance product/upstream markers until the source is complete and accepted.
