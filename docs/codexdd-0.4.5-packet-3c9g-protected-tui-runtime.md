# CodexDD 0.4.5 — 3C.9g coordinated protected TUI runtime reconciliation

**Status: SOURCE-ONLY CHECKPOINT — NATIVE RUST BUILD AND TESTS PENDING; NOT A RELEASE CANDIDATE.**

Scope is a three-file runtime/source batch against official pinned `rust-v0.163.0`. Source target remains separate from the read-only/updater automation branch; product `0.4.4` and tracked `rust-v0.162.0` release markers are intentionally unchanged until full release acceptance.

## Reviewed source outcomes

- `codex-rs/tui/src/chatwidget/interaction.rs`: move to the official 0.163.0 clipboard-paste/activation-key API (`PasteImage { text_fallback }`, `clipboard_paste_target`, `take_key_chord_reset`, `consume_view_transition_key_event`), while restoring the fork's three adaptive manual-Esc interruption paths, pre-input completed-tool flush and legacy `handle_key_event` entrypoint for CodexDD callers and tests. Official base blob `8657de2e8373e4a07fd4e97eb7ca5b1ca6048a54`; fork blob `f8ec1053365881baae512dc9b1575032bec99ac6`; merged blob `e83bc3d1860a8f2f04a14bc136c4b6468785eb67`.
- `codex-rs/tui/src/app/thread_routing.rs`: align the two realtime local-failure call sites with `RealtimeFailureCause::LocalRequest`. App-server request resolution is now via the original `PendingServerRequest.reply` handle and `take_resolution_record` (already staged) rather than submitting a reply through a potentially different active app-server connection. Preserve the CodexDD bound Worker trigger, adaptive turn/validation authorization, side-thread close transition and `daybreak_turn_eligible(enabled)` security check. Original fork blob `46584906c9c228bf20c7a1c1b676b2940b8f707c`; merge blob `629ddda83497d1447819bd84ca8c436d4b23fdcf`.
- `codex-rs/tui/src/app_server_session.rs`: register existing staged upstream `daybreak_api_key_compat` module, initialize the serialized API-key forwarding semaphore, and constrain available models during API-key startup using the current config requirements. Retain all existing CodexDD adaptive workflow state restoration, worker assignment, and original 0.4.4 tests. Original fork blob `c95f1d080836e934d83296ebab20b01abf1af5be`; merged blob `c9ff4fb4294c894490e224590adfc411a7eff2fa`.

The 44-path protected TUI source inventory includes these three reviewed source paths, leaving **15 protected TUI test files for the final native behavior reconciliation**. All other reviewed source paths remain pinned.

## Native-build and dependency gate

These edits are not proof of a working build. Official 0.163.0's app/config/slash/clipboard paths and the fork adaptive Worker and F2 elevated startup remain security-sensitive. The current integration `Cargo.toml` requests `rmcp =3.5.1`, `linkify`, and workspace `version 0.163.0`, but the committed `Cargo.lock` still contains the old `rmcp 3.3.0`, missing `linkify` and `codex-tui 0.162.0`. **`cargo check --locked` must not be labeled green while that mismatch remains.** A separate resolver-backed lockfile merge must preserve `rustls 0.23.46`, `noyalib-serde-yaml 0.0.57`, `quinn-proto 0.11.18`, and recheck security advisories before committing.

Next: short native Windows Cargo metadata/resolve and compile gate on Daniel-CL or an appropriately provisioned Windows runner, repairing only confirmed failures. No full 21-stage suite until the exact source/build state is stable. Source-only regression tests and all outstanding 109-overlap/15-conflict attestations, generated schemas, v2 release receipt, CI, and publishing are still pending. Automated updater branch writes, owner approvals and product-release markers remain unchanged.
