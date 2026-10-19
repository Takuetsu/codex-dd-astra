# CodexDD 0.4.5 — 3C.9e3 API-key model refresh on session transition

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Bounded source-only reconciliation; native Rust tests and TUI runtime checks pending.

## Three-way source provenance

- File: `codex-rs/tui/src/chatwidget/session_flow.rs`.
- Production CodexDD 0.4.4 parent: `b25550dcb2c6e84a84074b603086c9db6f32a63b`.
- Original fork blob: `73d7f75b26e5de276be3357a5fda836c24b4090b`.
- Official `rust-v0.162.0` blob: `97c7de5be321add6c92c50345fc19dea8c04dcb0`.
- Official `rust-v0.163.0` blob: `6077a0f9ef11faec6a95b69ddf9d50616c00a6c4`.
- Reviewed merged blob: `17500409f70dfda7fa1c489258fa19ab01bed747`.

Official 0.163.0 adds six lines to the session-change branch to refresh account-scoped model metadata when switching an existing thread or changing cwd under a remote connection **and** the account is in explicit API-key mode. The CodexDD fork independently preserves adaptive effort snapshot restore and protected Worker model/effort handoff. Only the nonoverlapping official conditional was transplanted:

```rust
if (previous_thread_id.is_some()
    || (self.remote_connection.is_some() && self.config.cwd != session.cwd))
    && self.status_account_display == Some(StatusAccountDisplay::ApiKey)
{
    self.refresh_account_scoped_models();
}
```

This remains inside the existing `previous_thread_id != self.thread_id` branch. It neither changes permissions nor broadens trusted adaptive signals.

## Dependency evidence and remaining boundary

`codex-rs/tui/src/chatwidget/model_popup_state.rs` was already staged as official upstream 0.163.0 (blob `15f99f85bbe4d903ebed8560ce733a0ecadb41d4`) and provides `refresh_account_scoped_models`, which strips stale API-key-only program availability and sends a model-fetch request. No new helper or authorization path is introduced by this packet.

Upstream `chatwidget/settings.rs` separately adds Daybreak/API-key eligibility controls and config-panel theme synchronization. That file must be reconciled with the outstanding `chatwidget.rs` / `chatwidget/constructor.rs` / `config_panel` wiring rather than replaced opportunistically. Similarly, new upstream history-replay API-key tests depend on the changed Daybreak gating and are not yet transplanted.

## Integration inventory and validation gates

The existing 3C.9c inventory now tracks **ten reviewed TUI paths among 44 original protected files**; **34 unresolved** remain. Independent packets 3C.9e1 (question replay) and 3C.9e2 (thread read-state routing) are in the same integration branch.

The offline `.github/scripts/test_codexdd_045_3c9e3_model_refresh.py` guards exact source reconstruction, the API-key/cwd conditions, preserved adaptive Worker session binding, the already-staged model-refresh helper and unchanged release control files. No new 3C.9e tests have been run yet. Previous Daniel-CL 3C.9c–3C.9d4 **50/50 static PASS** predates these source changes.

Native Rust compilation and TUI integration tests, `Cargo.lock`, schema generation, the 21-stage Windows release suite and GitHub CI remain outstanding. Product markers remain CodexDD `0.4.4` and upstream `rust-v0.162.0`, with auto-publishing disabled.
