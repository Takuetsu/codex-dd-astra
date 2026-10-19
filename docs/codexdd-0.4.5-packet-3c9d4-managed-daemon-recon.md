# CodexDD 0.4.5 — 3C.9d4 managed-daemon UI provenance reconnaissance

**Date:** 2026-10-18  
**Status:** RECONNAISSANCE ONLY — NOT A RELEASE CANDIDATE. No implementation in this packet.

## Source inventory and base comparison

- Upstream `rust-v0.163.0` adds a managed-daemon provenance bool across `startup_orchestration.rs` -> `lib.rs::run_ratatui_app` -> `app/startup.rs::App::run` -> `RemoteConnectionStatus.is_managed_daemon`.
- On the integration branch, `startup_orchestration.rs` is the reviewed 3C.9d3 blob `06ea648374cbdcc3ae7276ba146f4dbe45da3118`; its `run_ratatui_app` handoff remains deferred deliberately.
- Protected `tui/src/lib.rs`: integration fork blob `83ad7124e531be410477bd097cde8950bca40ed2`, official `rust-v0.162.0` blob `dc0fbd0547e3655a42f33d7559763c0da6e1b44d`, official `rust-v0.163.0` blob `a3fb5447b06d48d3a2924eb25dae4174243cb844`. Three fork-only hunks preserve adaptive module registration, configuration, and startup family handoff. Fourteen upstream release hunks need broader review.
- Protected `tui/src/app/startup.rs`: integration fork blob `86f1afec7ce715799c9fa62859e53f312918d067`, official `rust-v0.162.0` blob `69a4ff5f3ee2fe6a3ffb8dc993c675ae29dba738`, official `rust-v0.163.0` blob `4c329f378702414c3c17cc31802c778a8dbb2266`. Seven fork-only hunks retain adaptive resume/startup and Worker-bound worktree transitions. Eleven upstream hunks include other UI-state additions and must not be blindly applied.
- `tui/src/cli.rs` already has the internal `managed_daemon: bool` field.
- `tui/src/status/remote_connection.rs` already has `is_managed_daemon: bool`; its baseline value is `false` for all targets.

## Bounded implementation plan

1. Pass the `managed_daemon` bool from the trusted 3C.9d3 startup-orchestration local-daemon selection to `run_ratatui_app`.
2. Extend the protected `lib.rs` function signature, then forward the bool to `App::run` *without* disrupting the adjacent CodexDD adaptive-family argument.
3. Extend `app/startup.rs::App::run` and set `remote_connection.is_managed_daemon` only when `connection.is_local_daemon && managed_daemon`. Never mark explicit remote transport or embedded startup managed.
4. Review the fork-only hunks against official 0.163.0 at all affected signatures. Guard native app tests and prevent accidental fallback/approval-policy widening.
5. Record the exact resulting file blobs and maintain the 3C.9c source-preservation pins. Partial upstream handoff does **not** close unrelated protected lib/app merges or authorize Rust compilation acceptance.
6. Add targeted source/static checks for the complete caller/callee chain. Native Windows tests and runtime daemon provenance still pending.

## Risk boundaries and release gates

Avoid wholesale replacement of `lib.rs` or `app/startup.rs`; 0.163.0 also adds unrelated GovCloud guidance, UI-state and daemon-restart handling that need separate reviewed merges. Never use the new bool to authorize untrusted daemon restart or treat a nonlocal connection as a managed local daemon.

3C.9d3 source-only guards have been authored but not run. Cargo.lock, generated schemas, native Rust tests, the Windows 21-stage suite and CI remain outstanding. Version markers and publishing stay unchanged.
