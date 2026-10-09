# CodexDD 0.4.3 — Packet 3C.3a: TUI slash-command routing

**Status: TWO HANDWRITTEN TUI TEXTUAL-CONFLICT FILES SOURCE-MERGED, NATIVE COMPILATION PENDING.** The validated 3B.5 core/app-server CI run is not a TUI test and must not be cited as such.

## Source integration

- Active isolated branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`.
- Official upstream source: `rust-v0.162.0` peeled SHA `c1382380de69521303b416720a52f42d51af6248`; production baseline `4828e3b4232781963594cfaaebdc49c5531de8b1`.
- `codex-rs/tui/src/slash_command.rs` — commit [`e2012371`](https://github.com/Takuetsu/codex-dd-astra/commit/e2012371b445a103d19c8c2b7f87d71013a6b384). The complete upstream 0.162.0 command inventory including Daybreak was retained. CodexDD's /adaptive enum variant, description, command availability and in-progress dispatch were re-added. The additions were reversible to the exact upstream source.
- `codex-rs/tui/src/chatwidget/slash_dispatch.rs` — commit [`a13f2f93`](https://github.com/Takuetsu/codex-dd-astra/commit/a13f2f934f035c8b80de50356e23136f2f5ef8f5). Full upstream 0.162.0 Daybreak and MCP login/confirmation behavior was retained, with *exact* fork-only changes reapplied for `/adaptive` status/inline commands, new-role Worker scoping, and extra `show_session_checkout_picker` binding argument. Reapplying the same edits to the old official 0.159.2 file recreated the production fork file byte-for-byte; no unexplained prior fork delta was dropped.

## Safety requirements

- Roleful Worker creation must continue to refuse a parent without a nonblank `authorized_scope`; passing a raw role is not an authorization grant.
- Keep `/adaptive` separate from upstream `/daybreak`; neither should silently turn into the other.
- Preserve the current side-conversation and busy-state slash-command guards; the dedicated adaptive argument path is deliberately explicit.
- Require a TUI compile/format and interactive/Windows regressions once the related `chatwidget`, picker, and runtime files in 3C.3 are reconciled.

## Validation and next steps

The **3B.5** targeted Linux smoke [#37992789061](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37992789061) passed core/app-server/Code Mode evidence, but predates the slash-dispatcher source merge and does not cover TUI. The temporary read-only 3B.5 workflow has been removed.

Continue **3C.3b** through the remaining chatwidget and permission/UI boundaries before claiming TUI source compilation. Then **3C.4** TUI regression reconciliation. Wider native Windows F2/daemon and release-profile checks stay pending. The production release and installed clients are unchanged. No Daniel-CL action requested.
