# CodexDD 0.4.5 — Packet 3B.3: trusted adaptive event replay and thread-state lifetime

**Date:** 2026-10-18  
**Status:** INCOMPLETE / NOT A RELEASE CANDIDATE. Three-way app-server runtime source merge checkpointed, with **14 targeted static Python tests pending Daniel-CL validation** and native Rust integration untested.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Three-way source provenance

| Source file under `codex-rs/app-server/src/` | Tracked 0.162.0                            | Production 0.4.4                           | Official 0.163.0                           | Reconciled Git blob                        |
| -------------------------------------------- | ------------------------------------------ | ------------------------------------------ | ------------------------------------------ | ------------------------------------------ |
| `bespoke_event_handling.rs`                  | `f632857401ead872c492b887416db98395e8646f` | `65824b0da966df860bcc901d681017545de0227e` | `5e5430826edf33b22c5cdbdec3f5e5a6078f20aa` | `1c4a913d2373a614a2dfe73c10e6f65c64a727a1` |
| `thread_state.rs`                            | `a19c41b19ff8aea40842771f261f195661b33ec5` | `79bb2c10a6994dfdfcd96d7f458fdc15c5f60ba7` | `6dfe969ab3adfc5d703184ee2ee61a1b6335f282` | `42b434e017c527a8785fbaedf43292a83e1af09d` |
| `notification_media.rs`                      | `4223ea796d2882be95a5153f43212e5356d0fa8d` | `05256bae425f635997229a3b29b26c5f1e8eee19` | `a1572cbaf7b9026f789c96ce9d027e0a7cc4b1be` | `c93967717664145a72390384749747212fd5c63a` |

## Source reconciliation

The event-handling merge starts from the complete official 0.163.0 source. It preserves upstream `RealtimeEvent::SessionUpdated` tracking of `realtime_session_id` and reinserts the exact CodexDD 0.4.4 adaptive signal handler, event match arm and native Rust regression. A native `EventMsg::AdaptiveRuntimeSignal` derives its source-turn ID from the owning event (not from a model-controlled request payload), stores the trusted envelope in the active turn's summary and emits the live notification. On a **successful** turn only, all turn-local signals are replayed to the same outgoing FIFO immediately **before** `TurnCompleted`, preventing a late arrival from being mistaken for a missed terminal. An interrupted or failed turn does not receive the success-only terminal replay. The existing Rust event test checks exact conversation identity and FIFO ordering.

The thread-state merge preserves the CodexDD turn-scoped `adaptive_runtime_signals` buffer and all official 0.163.0 listener generation safeguards, the new `PendingThreadUnloads` map and `remove_unsubscribed_listener` cleanup. The notification media filter preserves both the official `ThreadReadStateChanged` passthrough and CodexDD's inert `AdaptiveRuntimeSignal` passthrough; this is **not** authorization for external thread mutation.

`.github/scripts/test_codexdd_045_3b_adaptive_events.py` adds 14 read-only Git/source tests for exact source identity, owner-bound signal fields, trusted turn summary, success-only replay and ordering, upstream realtime tracking, safe listener-unsubscribe semantics, both media passthrough variants, unchanged 0.4.4 release markers and explicit incomplete status.

## Remaining gates

**No native Rust build, event-stream behavioral run, 21-stage Windows suite or code-generated schema validation has passed for this packet.** These merges must be accepted only alongside the complete upstream 0.163.0 model and app-server graph. Some companion handler imports and new upstream types remain staged in other packets. The original 109 overlapped paths and 15 conflict ledger is not yet source-complete; eventual v2 sidecar review, regenerated Cargo.lock, provenance receipt, Windows and CI tests remain mandatory.

**Next:** Stage separate 3B.4 RPC request routing/workflow-state persistence, preserve official thread read-state and item-read behavior, and validate 3B.3 + 3B.4 at one inexpensive Daniel-CL checkpoint. Do not publish, tag, open automated PRs, or advance CodexDD product version from 0.4.4.
