# CodexDD 0.4.5 — 3C.9e2 thread-read-state notification routing

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Reviewed source-only reconciliation. Native Rust compilation and notification routing runtime tests remain outstanding.

## Three-way source provenance

- Path: `codex-rs/tui/src/app/app_server_event_targets.rs`.
- Production CodexDD 0.4.4: commit `b25550dcb2c6e84a84074b603086c9db6f32a63b`; fork file blob `521afcee6d57e0adfdb6ffdf89aad77f6bc7c11e`.
- Official upstream `rust-v0.162.0` blob: `9e5b60bd83e7e9a6241b8baeb90659592c4ecf24`.
- Official upstream `rust-v0.163.0` blob: `13d30c273f472d1c4271216ee3fbeaa5c350894f`.
- Reviewed source blob: `16d6b02da4469a7f1ef2d82916a48d04719124d5`.

The CodexDD fork preserves trusted `ServerNotification::AdaptiveRuntimeSignal` routing and related tests; upstream 0.163.0 adds one global-notification match arm. The reviewed change inserts exactly:

```rust
| ServerNotification::ThreadReadStateChanged(_)
```

between `AccountUpdated` and `GatewayOAuthChanged` in the `None` notification grouping. Existing branch behavior maps this grouping to `ServerNotificationThreadTarget::Global`. Although the notification payload includes a thread ID, this deliberately follows upstream's broadcast routing rather than repurposing the per-thread adaptive-signal dispatcher. All other fork lines remain unchanged.

## Checked dependencies and safety boundaries

`app-server-protocol/src/protocol/common.rs` already declares the `ThreadReadStateChanged` server-notification variant and `protocol/v2/thread.rs` defines `ThreadReadStateChangedNotification` with `thread_id` and `read_state`. No protocol additions are needed for this source merge.

The code adds **no new app-server side effects**; it only classifies the inbound notification as global. Existing trusted adaptive-signal thread targeting, validation authority and approval policy are not modified.

## Integration inventory

The 3C.9c 44-path protected fork guard now has **nine reviewed exceptions** and **35 unresolved** TUI paths. This is 3C.9e2, alongside independent 3C.9e1 question-replay safety.

`.github/scripts/test_codexdd_045_3c9e2_read_state_routing.py` provides exact source-provenance and behavior-shape guards. No execution is claimed for 3C.9e1 or 3C.9e2 yet. Previous Daniel-CL 3C.9c–d4 suite passed 50/50 static source checks, before these patches.

Native Rust compilation, integration and live Windows tests, `Cargo.lock`, schema generation, the full 21-stage Windows release suite and GitHub CI remain outstanding. Version markers remain `0.4.4` and `rust-v0.162.0`; publishing stays disabled.
