# CodexDD 0.4.5 — Packet 3B.1: protocol data models and adaptive-signal reconciliation

**Date:** 2026-10-18  
**Status:** INCOMPLETE / NOT A RELEASE CANDIDATE. Exact pinned upstream protocol model blobs and one three-way event-protocol merge are checkpointed. **Targeted Python source validation pending; native Rust integration untested.**  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Three-way source verification

The original tracked official `rust-v0.162.0` and CodexDD 0.4.4 production blobs match for each of the following ten paths; therefore none contains fork-local code. Every staged upstream Git blob is byte-exact to official pinned `rust-v0.163.0` commit `83086188ae0614bd882818ff1aa23f3194962012`:

| Path under `codex-rs/protocol/src/` | Official 0.163.0 Git blob |
| --- | --- |
| `approvals.rs` | `09b7115824932453ab6891dbba33d4173e68d684` |
| `items.rs` | `709890809d58bdd81339683e8334764135094d4e` |
| `models.rs` | `b2ce4a10a60553fa195d3eb92768c4759eae0636` |
| `models/executed_tool_calls.rs` | `7b2fe4c2d57d809abe6e794164e1f92914b7f36f` |
| `models/executed_tool_calls_tests.rs` | `7ce715997496aa04b748725690cf41f9de72f0ce` |
| `models/item_metadata_tests.rs` | `ad8d66d73eb12368160208c9c7de51508d1db756` |
| `openai_models.rs` | `45e3d5c0963048f197afcc4e5cdff3770df54a9b` |
| `turn_input.rs` | `7e9977959c37db4187317859f9742e7f8be49874` |
| `conversation_start_params_tests.rs` (new) | `6ccb5e4f1e3c956e6439442aa313c7be16d92e80` |
| `openai_models/incremental_tools.rs` (new) | `f051318136ae1a2f7da7bff02d8d0bc570ab5d05` |

The overlapping `protocol.rs` has tracked blob `841c794d4d46bb5f9da771b88c05fe09c13826bb`, fork production blob `5a19125a22aa3ebccc83a91b20996687cdeb76f9`, official target blob `146209407b78b71c461c854c5d9d1e4d7816b873`, and reviewed merged blob **`9ff46a4e1331e15f35f95b8e17ba41dfc931a5cf`**.

## Exact conflict resolution

The merged `protocol.rs` begins from *all* official 0.163.0 lines, then adds only CodexDD's private adaptive-runtime data transport `AdaptiveRuntimeSignalKind`, `AdaptiveRuntimeSignalEvent` and its `EventMsg::AdaptiveRuntimeSignal` variant at the original event boundary. Seven original event kinds and canonical `snake_case` serde/TS semantics remain unchanged. The transport carries **no caller-supplied thread or turn identity**; runtime correlation stays bound to the owning session and enclosing event. No policy or auto-escalation authorization is added here.

Official upstream additions also remain intact: `ConversationStartParams` redacted Debug implementation to avoid exposing credentials/instructions, `ElicitationAbandonedEvent`, realtime attach/detach messages, voice versions, new model encrypted/content fields, incremental tools, and encrypted-compaction replay-budget estimates. Upstream's new `approvals.rs` supplies the matching elicitation type; `models.rs` supplies the updated `ResponseItem` shape. No source in this packet opens up approval or sandbox permissions: **fork `codex-rs/protocol/src/permissions.rs` remains unchanged** at `140090f5ef1649ed9e4717643dc85773be93da5f`.

## Validation and pending work

`.github/scripts/test_codexdd_045_3b_protocol.py` adds **12 offline source/provenance tests** checking every official Git blob, exact merged entrypoint blob, the original adaptive signal schema, session-owned identity, upstream privacy changes, elicitation, encrypted fields, incremental-module wiring, unchanged fork permissions and original `0.4.4` release markers. These are static tests only, **not native Rust compilation or runtime integration**, and cannot substitute for the full 21-stage Daniel-CL Windows release suite.

**Dependency and artifact gate:** the rest of the upstream 0.163.0 source is incomplete. Later work must integrate app-server protocol/event handlers and runtime consumers, follow on the encrypted model fields, review the remaining security/approval paths, perform **schema regeneration** only from a fully reconciled source, regenerate and review Cargo.lock, then run isolated Rust/WASM/codegen checks and full native release validation. The original 109 overlapping path and 15 conflict ledger must be reconciled, with updated v2 evidence, before any release gate can be closed.

Publishing, draft PR creation, product-version promotion, tag/merge, installer distribution and production writes remain disabled or separately owner-gated.
