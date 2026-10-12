# CodexDD 0.4.3 — Packet 3C.4: TUI and Windows test source reconciliation

**Status: ALL 26 OWNED TEST SOURCE OVERLAPS RECONCILED; test-source CI compilation and native Windows execution PENDING.** This is an isolated source checkpoint, not release acceptance.

- Branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`
- Production baseline: `4828e3b4232781963594cfaaebdc49c5531de8b1`
- Upstream target: `rust-v0.162.0`, peeled `c1382380de69521303b416720a52f42d51af6248`
- Last source checkpoint before test gate: `da9cda3656ba7417abb1849bc6dd538a7a91a82b`
- Read-only test compile workflow published at `d8ccd544db57de62b6be3282289284a7cddb99a7` (`.github/workflows/codexdd-043-tui-source-smoke.yml`)

## Reconciled source inventory (26)

- `codex-rs/app-server-daemon/src/backend/windows_tests.rs`
- `codex-rs/cli/tests/daemon_startup.rs`
- `codex-rs/tui/src/app/owned_transcript_tests.rs`
- `codex-rs/tui/src/app/test_support.rs`
- `codex-rs/tui/src/app/tests.rs`
- `codex-rs/tui/src/app/tests/disconnect_tests.rs`
- `codex-rs/tui/src/app/tests/session_lifecycle_requests.rs`
- `codex-rs/tui/src/app/tests/startup.rs`
- `codex-rs/tui/src/app/tests/startup_defaults_tests.rs`
- `codex-rs/tui/src/app/tests/turn_submission.rs`
- `codex-rs/tui/src/app_server_session/rollout_history_tests.rs`
- `codex-rs/tui/src/app_server_session/workspace_roots_tests.rs`
- `codex-rs/tui/src/chatwidget/tests.rs`
- `codex-rs/tui/src/chatwidget/tests/app_server.rs`
- `codex-rs/tui/src/chatwidget/tests/composer_submission.rs`
- `codex-rs/tui/src/chatwidget/tests/exec_flow.rs`
- `codex-rs/tui/src/chatwidget/tests/history_replay.rs`
- `codex-rs/tui/src/chatwidget/tests/misalignment_policy_tests.rs`
- `codex-rs/tui/src/chatwidget/tests/permissions.rs`
- `codex-rs/tui/src/chatwidget/tests/plan_mode.rs`
- `codex-rs/tui/src/chatwidget/tests/slash_commands.rs`
- `codex-rs/tui/src/chatwidget/tests/status_and_layout.rs`
- `codex-rs/tui/src/chatwidget/tests/worktree_picker_tests.rs`
- `codex-rs/tui/src/daemon_startup_tests.rs`
- `codex-rs/tui/src/dynamic_tools_tests.rs`
- `codex-rs/tui/src/history_cell/tests.rs`

## What remains covered

- Upstream 0.162.0 session, permission, transcript, worktree, Daybreak, MCP, startup, and widget regressions.
- CodexDD adaptive state initialization and explicit opt-out; model/effort restoration on session attachment; fork-state inheritance; fresh Worker-scoped session reset.
- Worker binding authorization and scope preservation across new session and managed worktree routing.
- FIFO event delivery on closed side-thread streams and trusted turn-trigger propagation.
- Adaptive status and LVO evidence, adaptive interrupt/replay, and dynamic tool runtime-native evidence metadata.
- Elevated Windows daemon denial: `backend/windows_tests.rs` keeps the fork's token/daemon guard assertion while retaining all 0.162.0 Windows handle updates.

## Upstream test supersession decision

For `codex-rs/cli/tests/daemon_startup.rs`, the fork previously broadened one restrictive-launcher warning check so an elevated operator could yield either launcher- or elevation-specific embedded fallback. The official upstream 0.162.0 test instead **separates** those paths: `elevated_local_tui_uses_embedded_without_starting_daemon` explicitly covers elevated start, resume, fork, and rejected daemon `start`/`restart`, while `restrictive_launcher_uses_embedded_if_daemon_cannot_start` asserts the non-elevated restrictive-launcher warning. This subsumes the old mixed-message fixture without relaxing the daemon security contract. Native elevated Windows execution remains required.

## Verification boundaries

- Previous [TUI library-only Linux compile smoke #38003376461](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38003376461) passed at `bb1972805b`; it **predates** the 3C.4 test-source changes and does not validate these fixtures.
- A read-only GitHub Actions gate runs `cargo check -p codex-tui --lib --tests`, which checks the reconciled TUI test source without linking/running the complete suite. A successful source compile is not a passing runtime regression.
- Full 3C.4 executable test coverage, native Windows F2/SSH/elevation/daemon acceptance, broader 3D/3E packets, owner-gated provenance/version promotion, and production installation remain pending.
- Do not modify Cargo.lock outside 3E.2, generated bindings manually, or production state. Daniel-CL action is not requested for this source checkpoint.
