# CodexDD 0.4.5 — 3C.9e1 question-answer replay input safety

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Reviewed source-only reconciliation; native Rust compilation, TUI interaction tests and release acceptance pending.

## Three-way source provenance

- File: `codex-rs/tui/src/chatwidget/replay.rs`.
- CodexDD 0.4.4 production parent: `b25550dcb2c6e84a84074b603086c9db6f32a63b` with original fork blob `8b079cdb3863f0d613ec1f9eb702c142c6445133`.
- Official `rust-v0.162.0` file blob: `1907577c5647ef4df7a856a4b88bf77a1fdb2384`.
- Official `rust-v0.163.0` file blob: `1ebff8fcb314d5c8c2a867c18def36b1b2f7ea60`.
- Reviewed integration source blob: `d17ff1c77f63c8c57e0a847c58377999fda78480`.

The CodexDD fork adds only Worker-bound replay duration accounting near the initial reconstructed-turn loop; official 0.163.0 adds one non-overlapping input-safety hunk in the resolved-question reply replay. The reviewed merge changes that reply handling only:

```rust
let displayed_answer_changed =
    self.bottom_pane.question_editor().resolve_answers(&ids);
if displayed_answer_changed && !self.bottom_pane.has_active_view() {
    self.bottom_pane.key_chord_reset_requested = true;
}
```

The prior `refresh_pending_input_preview()`, redraw and `on_committed_user_message` sequencing are preserved. The exact patch is a four-line net insertion. No other source or adaptive policy was replaced.

## Dependency and behavior check

The upstream-staged `bottom_pane/async_questions/state.rs` already returns `bool` from `resolve_answers`; the staged `bottom_pane/mod.rs` exposes `has_active_view()` and `key_chord_reset_requested`. This patch therefore requires no extra source replacement or new UI fields.

The reset only occurs when a reply changes the displayed input target **and** no active modal owns input. It does not grant permission to submit a command, bypass approval prompts or reinterpret any Worker evidence. CodexDD's completed Worker replay-duration accounting remains byte-identical to production.

## Integration inventory and blockers

The 3C.9c protected TUI source guard now pins **eight reviewed source exceptions among 44 original fork paths**, leaving **36 unresolved**. This packet is deliberately independent of the config-panel source merges. The upstream `chatwidget/status_controls.rs` and `app/config_persistence.rs` changes reference `config_panel_tui`, which is absent from the currently protected `chatwidget.rs`; those changes must wait for a separate coordinated config-panel merge.

The new `.github/scripts/test_codexdd_045_3c9e1_question_replay.py` is an offline static source-provenance guard. **No 3C.9e1 test execution is claimed yet.** Prior Daniel-CL evidence: 3C.9c + 3C.9d2–3C.9d4 source checks **50/50 PASS**, before this packet.

Native Rust compile/testing, the security regressions and Windows elevated startup smoke, schema generation, `Cargo.lock`, the full 21-stage Windows release suite and GitHub CI remain outstanding. Release markers remain CodexDD 0.4.4 and `rust-v0.162.0`; upstream auto-publishing stays disabled.
