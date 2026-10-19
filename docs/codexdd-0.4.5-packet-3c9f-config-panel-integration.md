# CodexDD 0.4.5 — 3C.9f coordinated /config TUI integration checkpoint

**Date:** 2026-10-18
**Status:** **SOURCE IMPLEMENTED; NATIVE BUILD/TESTS PENDING — NOT A RELEASE CANDIDATE**.
**Release target:** official `rust-v0.163.0` pinned for product CodexDD `0.4.5`, not changed.
**Source branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.
**Parent before integration:** `04435f29b56ec4f690973efafcb93c27a682711a`.
**Security baseline:** production `b25550dcb2c6e84a84074b603086c9db6f32a63b` (CodexDD 0.4.4).

## 1. Coordinated source changes

Sixteen related source files are changed in a single atomic checkpoint instead of separate one-file PRs:

- `tui/src/lib.rs`, `app.rs`, and `chatwidget.rs`: register already-staged upstream `config_panel` modules; add notification preview state to `App` and the editable `config_panel_tui` snapshot to `ChatWidget`.
- `chatwidget/constructor.rs`, `app/startup.rs`: initialize new state without replacing protected CodexDD adaptive-family or Windows daemon start parameters.
- `app_event.rs`, `app/event_dispatch.rs`: add `OpenConfigEditor`, `SaveConfigPreference`, `TestConfigNotification`, `SendConfigTestNotification` and route via already-staged upstream `app/config_panel.rs`. The editor transition cancels keyboard chords, quarantines pending input, and fails closed if the terminal cannot discard pending interactive input. Local display/config actions remain reachable when the app-server is offline; server-backed events remain restricted.
- `app.rs`: interrupt pending config transitions on focus loss and non-move mouse input, mirroring upstream keyboard-event cancellation.
- `chatwidget/slash_dispatch.rs`, `chatwidget/input_flow.rs`: support explicit `/config` invocation and block normal queued submission while its editor is active, without changing `/adaptive` or Worker-scope controls.
- `chatwidget/status_controls.rs`, `chatwidget/settings.rs`, `app/config_persistence.rs`: maintain the live panel's theme/status/title and restored session-picker/resume-directory preference state. Preserve existing adaptive successor notifications.
- `app/session_lifecycle.rs`: copy config-panel TUI state on Widget replacement and cancel a pending notification test on thread replacement.
- `app/test_support.rs`, `app/tests.rs`: update Rust constructors to initialize the new notification generation/pending fields (prevent test-only struct-literal compilation failures).
- `chatwidget/tests/history_replay.rs`: extend Daybreak resume coverage so a stored On state cannot bypass explicit API-key access-program eligibility and can still hydrate correctly during ChatGPT sign-in.

The existing staged official `tui/src/config_panel/`, `tui/src/chatwidget/config_panel.rs`, `tui/src/app/config_panel.rs`, `tui/src/chatwidget/model_popup_state.rs`, `tui/src/app/input.rs` and `tui/src/tui.rs` are reused without replacement. No new release permissions or unsafe updater execution path is introduced.

## 2. Protected behavior

- Keep fail-closed elevated Windows SSH/daemon fallback and the 3C.9d2/3C.9d4 daemon provenance/managed connection rules.
- Keep CodexDD GPT-6.1 Sol/Luna adaptive-family model routing, trusted evidence-only Worker transitions, scoped Worker creation, and input/successor cancellation.
- For known API-key users, enable Daybreak only when `Feature::CliDaybreak` and `Feature::ApiKeyCyberAccessPrograms` both permit it. Preserve CodexDD's existing extra `daybreak_turn_eligible` function; no upstream deletion of fork-only guard.
- Leave `Cargo.lock` and security dependency pins unchanged pending the separate exact 0.163.0 dependency reconciliation.

## 3. Provenance and tests

The 3C.9c 44-path protected-source inventory now recognizes **24 reviewed paths**, leaving **20 unresolved for risk-based review**; this is a path inventory, *not* a Rust compile PASS or evidence that remaining paths all require edits.

The 3C.9d4 regression source guard was updated to check its original immutable 3C.9d4 source checkpoint at `1cf92eabd6bc9e7217c70a24c15182a58b5f9d28`, and to validate preserved live caller/callee invariants on HEAD. Historical 3C.9d4 source content was not rewritten. Static source tests still need to run after this checkpoint.

**Next build-oriented gates (order matters):**

1. Review exact 0.163.0 TUI module/constructor/event-shape compatibilities, then execute grouped inexpensive source guards once.
2. Resolve targeted Cargo workspace and lockfile discrepancies, retaining CodexDD's known patched `rustls`, `noyalib-serde-yaml` and `quinn-proto` security versions.
3. Run a **single targeted native Windows** `cargo check -p codex-tui --locked` (from `E:\codexdd\codex-rs`, in a safe worktree if the main checkout is busy), then fix concrete compile errors on GitHub. After compile succeeds, run focused native TUI app/config/security tests; do not run all 21 release stages.
4. Complete remaining TUI/security overlap reconciliation, original 15-path conflict records, generated schema updates and reusable v2 candidate/provenance work.
5. Freeze source identity; obtain release-profile Windows 21-stage PASS, exact-candidate attestation, CI/PR and owner-controlled merge/install.

**Explicitly outstanding:** native Rust compile/runtime, current 0.163.0 dependency lockfile, full 109-overlap security audit, 15 direct conflict resolution attestations, full Windows 21-stage acceptance, CI, final version/provenance update and deployment. Automatic upstream branch/PR publication is still hard-disabled on the separate updater feature branch; production remains unchanged.
