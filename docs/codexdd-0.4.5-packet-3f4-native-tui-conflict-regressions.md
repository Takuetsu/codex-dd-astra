# CodexDD 0.4.5 — Packet 3F.4: original TUI-conflict native behavior

**Status: EXACT-SOURCE WINDOWS 25/25 FOCUSED TUI REGRESSIONS PASSED; NOT RELEASE ACCEPTANCE.**

## Why this packet exists

The pinned official rust-v0.163.0 cherry-pick replay (GitHub run 39613477624) yielded fifteen original direct conflicts, six in TUI:

- `codex-rs/tui/src/app/startup.rs` (integrated blob `29065c6c0c90c010a53bf065c95278312712a90e`)
- `codex-rs/tui/src/app/tests/session_lifecycle_requests.rs` (integrated blob `994d54b631493b3d0784e836c544c57a275e9985`)
- `codex-rs/tui/src/app/tests/startup_defaults_tests.rs` (integrated blob `e60808df0448682d47af1a14b5bd8d5c52206397`)
- `codex-rs/tui/src/app_server_session/workspace_roots_tests.rs` (integrated blob `d6447d22a0a1e943b1352b8c945cf7f6cf09734e`)
- `codex-rs/tui/src/chatwidget/interaction.rs` (integrated blob `e83bc3d1860a8f2f04a14bc136c4b6468785eb67`)
- `codex-rs/tui/src/lib.rs` (integrated blob `7b0746fe66b55537d3c445bfac194cbdb005b2b2`)

The source-only 3F.2 receipt freezes all fifteen Git object identities; source equivalence does **not** prove that the integrated TUI still implements the correct CodexDD 0.4.4 authority and Windows startup behavior. Core history/registry tests are separately covered by Packet 3F.3; the original source ledger remains `NOT_ACCEPTED`.

## Exact bounded nonrelease Windows test gate

Extend existing `.github/workflows/codexdd-045-windows-focused-native.yml` instead of opening a parallel test runner. In addition to its original 15 adaptive, F2, startup, and approval cases, execute ten independently checked filters:

- `fresh_startup_uses_server_defaults_with_explicit_and_managed_precedence`
- `startup_read_failure_exits_before_thread_creation`
- `fresh_fork_inherits_parent_adaptive_state_when_server_state_is_empty`
- `new_session_worker_binding_resets_dirty_adaptive_state`
- `external_writer_escape_preserves_snapshot_and_explicit_quits`
- `remote_workspace_roots_survive_start_turn_resume_and_fork`
- `embedded_lifecycle_requests_preserve_explicit_workspace_roots`
- `default_daemon_auto_connect_probes_socket_only`
- `app_server_target_for_launch_uses_local_daemon_for_default_socket`
- `startup_remote_current_cwd_without_override_is_rejected`

**Coverage rationale:** server versus CLI startup defaults and failure handling; adaptive state inheritance/reset on fork/new worker; Escape and explicit quit under external-writer ownership; remote versus embedded workspace root propagation; Windows-protected implicit daemon probing; and remote cwd guardrails. Existing 15 TUI checks continue to cover adaptive authority, managed daemon fallback, elevated Windows F2 behavior and buffered approvals. The six TUI conflicted source files are Git-blob-pinned on every run.

**CI contract:** exact nonproduction integration branch and source SHA, Rust 1.95.0, pinned `Cargo.lock`, Windows 2022, repository-managed Rusty V8, read-only token and no persisted checkout credentials. Execute `cargo test --locked -p codex-tui --lib <case> -- --test-threads=1` sequentially, stop at first nonzero status or a missing **nonzero executed test** receipt, upload raw per-filter logs. Never call the full 21-stage release suite as a diagnostic.

## What this packet does not authorize

No Rust source edits, Cargo/deny policy changes, version marker promotion, installer, merge, auto-update or publication. This targeted job cannot substitute for exact candidate-wide native Windows validation, Bazel release-profile packaging or the all-target `cargo deny` remediation decision for four inherited INFO/unmaintained advisories. Log genuine failures and fix only the failing scope before considering another native test. Daniel-CL remains untouched until final validation/install.

## Focused native closeout — October 21, 2026

- [Windows GitHub Actions #39618752181](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39618752181) completed **successfully** at exact integration SHA `5e6db71d7c7a4dd0f9bec01c9da7bc5c68af251c`. All **25/25** individually filtered native TUI tests ran, including the ten original-conflict selectors. Pinned source, Cargo resolver and six conflicting-file identities passed, and logs were uploaded without publishing.
- [Core authority #39615406741](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39615406741) completed **15/15** bounded native filters at SHA `ac7064492a580f52efd724f8b19ab2eaf6d15e49`.
- These receipts are targeted semantic evidence only. They do not authorize release, waive four inherited security advisories, replace Bazel release-profile packaging or claim final 21-stage Windows acceptance.
