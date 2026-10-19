# CodexDD 0.4.5 — Packet 3C.9f: coordinated config-panel / adaptive TUI reconnaissance

**Date:** 2026-10-18  
**Status:** **RECONNAISSANCE ONLY — INCOMPLETE / NOT A RELEASE CANDIDATE.** No protected config-panel implementation in this packet. Do not claim native Rust or Windows runtime acceptance.

## Current state

The 3C.9c official 0.163.0 staging already includes:

- `tui/src/config_panel/mod.rs` (upstream blob `ebc5799be2b87909ea321c0b9c0718a362b6a7bc`) and its `input.rs`, `render.rs`, `toggle.rs` dependencies.
- `tui/src/chatwidget/config_panel.rs` (upstream blob `ea8cd8c7a5e1e33a097669343995e2735558d118`), implementing `open_config_panel` and editor interactions.
- `tui/src/app/config_panel.rs` (upstream blob `f1fb01ff12deb14fd99f402812839721fd1ef88b`), implementing preference persistence and notification previews.
- `tui/src/chatwidget/model_popup_state.rs` (upstream blob `15f99f85bbe4d903ebed8560ce733a0ecadb41d4`), providing account-scoped model refresh. Independent 3C.9e3 source already calls this helper.

The *protected* CodexDD `tui/src/lib.rs`, `app.rs`, and `chatwidget.rs` still do **not** register their respective `config_panel` modules. Protected `chatwidget.rs` lacks the upstream `config_panel_tui: codex_config::types::Tui` field; `chatwidget/constructor.rs` has not initialized it. These gaps prevent blindly porting individual `/config` call sites.

## Coordinated source/call-site inventory

1. Module and state registration: `tui/src/lib.rs` (`mod config_panel;`), `tui/src/app.rs` (`mod config_panel;`), `tui/src/chatwidget.rs` (`mod config_panel;` and the `config_panel_tui` field), and `chatwidget/constructor.rs` (clone and assign initial TUI config). Preserve all preexisting CodexDD adaptive modules and initialization.
2. Event routing and persistence: `tui/src/app_event.rs` (official `OpenConfigEditor`, `SaveConfigPreference` variants), `tui/src/app/event_dispatch.rs` (corresponding guarded handlers), and existing staged `tui/src/app/config_panel.rs`. Maintain modal input quarantine and never send unverified file/config mutations.
3. Widget command and UI state: protected `chatwidget/slash_dispatch.rs`, `chatwidget/input_flow.rs`, `chatwidget/status_controls.rs`, and `chatwidget/settings.rs`. Port `SlashCommand::Config` handling only once `open_config_panel` and its module compile. Preserve `/adaptive`, roleful Worker binding, trusted signal handling and the existing status line.
4. Persistent/replacement state: protected `app/config_persistence.rs` copies live `session_picker_view` and `resume_cwd` preferences; protected `app/session_lifecycle.rs` must carry the `config_panel_tui` setting through ChatWidget replacement. These interact with adaptive Worker session transfer.
5. Account/Daybreak feature gating: `chatwidget/settings.rs` official 0.163.0 adds API-key program feature gating; corresponding `chatwidget/tests/history_replay.rs` extends API-key and sign-in hydration fixtures. These should be reviewed as a distinct behavioral subpacket and not lost during config-panel reconstruction.
6. Audit `chatwidget/interaction.rs`, `app/test_support.rs`, event dispatch tests and the new config-panel test files for additional registration/field/use-site dependencies. The event variants must never be staged without their handlers.

## Protected source blob references at reconnaissance time

- `tui/src/lib.rs`: `a40147cca1333114d82e0335521279254f5c1f21` (already reviewed by 3C.9d4).
- `tui/src/app.rs`: `b76ee1107fabcc98625fc663c7967e0dc6e906a5`.
- `tui/src/chatwidget.rs`: `e1a042122e0df76b990d4b2f6668de666fe3beab`.
- `tui/src/app_event.rs`: `44cd4adbcc90db4f55500d9f10b3bd8e3220364c`.
- `tui/src/app/event_dispatch.rs`: `86f62f5f80ea9691d96c62ba1151cd87f0c57c1e`.
- `tui/src/app/config_persistence.rs`: `a4a84eb699475a4042a3e7b038d2cc323e079c48`.
- `tui/src/chatwidget/status_controls.rs`: `b815e819fb0742d339d074a2dce2f98577c7c5a4`.
- `tui/src/chatwidget/slash_dispatch.rs`: `63d8b4b41cc237167a6c32ace5eb60cb193e4c46`.

## Implementation and acceptance gate

Before editing, review exact 0.162.0 fork-only hunks against official 0.163.0 in all linked protected paths and accept a cohesive source/caller-callee map. Prefer a bounded coordinated packet rather than piecemeal feature activation: unregistered upstream module files already exist on branch but individual `/config` wiring commits can become non-compilable in isolation.

Regression requirements: `/adaptive` and Worker-scope authority are preserved; explicit user input alone may invoke config editors, with modal input quarantined; read-only source guards do not substitute for native Rust interaction tests; config save and session replacement obey current permissions and trust. Do not run the 21-stage Windows suite while outstanding protected path reconciliations and native compile failures remain.

## Current validation and release controls

The 3C.9c–3C.9d4 Daniel-CL source checks passed **50/50**. Subsequent 3C.9e1–3C.9e3 have **33 authored source-only tests**, unrun, and ten reviewed protected source paths out of 44 original conflicts (**34 unresolved**). Publisher remains disabled. Cargo.lock, generated schemas, native Rust, Windows integration, GitHub CI and version-marker advancement remain pending. Product markers stay `0.4.4` / `rust-v0.162.0`.
