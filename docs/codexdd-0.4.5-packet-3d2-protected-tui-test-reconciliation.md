# CodexDD 0.4.5 — Packet 3D.2: upstream protected TUI test reconciliation

**Status: SOURCE AND SNAPSHOT INTEGRATION; NATIVE CHECK/RUNTIME ACCEPTANCE STILL PENDING.**

This packet reconciles eight previously unchanged 0.4.4 TUI protected test sources with the official `rust-v0.163.0` tests, using three-way review against official `rust-v0.162.0`. No runtime implementation, production version marker, release flags, deployment authority or dependency lockfile is changed here.

## Exact source changes

| Path (under `codex-rs/tui/src/`)              | New upstream regression or fixture                                                         | Original CodexDD fork protection retained                                      |
| --------------------------------------------- | ------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------ |
| `app/owned_transcript_tests.rs`               | Footer selection ignores warning click during modal and preserves the draft                | Adaptive state field on thread fixture                                         |
| `app/tests/disconnect_tests.rs`               | Disconnected config panel preserves user input                                             | New-session Worker binding, explicit `None`                                    |
| `app/tests/navigation_reconnect_tests.rs`     | Reconnect presentation follows current app state                                           | Fork's non-async event channel send                                            |
| `app/tests/startup.rs`                        | Unix unattended modal/approval regression and revised test notification                    | Fork's adaptive startup preset fixture and existing Worker tests               |
| `app_server_session/workspace_roots_tests.rs` | Larger native stack for remote roots scenario                                              | `turn_trigger` parameter in shared thread-start API                            |
| `chatwidget/tests/app_server.rs`              | Separate startup config warning presentations for managed, unmanaged and embedded sessions | Adaptive model/effort/thread-state restoration regression                      |
| `chatwidget/tests/composer_submission.rs`     | Plugin mention must not suppress skill mention with same name                              | Existing adaptive interrupt test and nine adaptive thread state fixture fields |
| `history_cell/tests.rs`                       | Legacy zsh migration warning snapshot                                                      | Adaptive state field on existing session fixture                               |

Three exact official 0.163 snapshot fixtures are staged with their tests: `legacy_zsh_migration_notice_snapshot`, `fullscreen_footer_selection`, and `startup_config_warning_panel`. These prevent missing-snapshot false failures. Existing fork-specific snapshots are not overwritten.

The source-provenance guard now pins each of the 44 original fork-protected TUI paths to its original production blob and an explicitly reviewed integration blob. Additional static checks require the eight regression markers and the three snapshots; the full source guard suite must be green before the packet is accepted.

## Remaining gates (no release authority)

1. Re-run `cargo check --locked --keep-going -p codex-tui --tests` and the library check on native Windows for this commit; do not interpret the previous green SHA as approval for changed tests.
2. Execute focused native tests for adaptive routing/Worker contexts/approvals and the newly staged upstream TUI tests; distinguish snapshot/platform regressions from source API errors.
3. Continue separate protected core/app-server security and schema drift audit, then run the full SHA-bound 21-stage Windows suite only after targeted checks pass.
4. Advance upstream/product version markers or perform release promotion only after all release evidence and owner acceptance.

CodexDD production stays **0.4.4**; tracked upstream stays **rust-v0.162.0**. The imported 0.163 source remains an unreleased integration. The focused Windows runtime check started earlier against the prior commit `1f92b78995b3a36abf6f66329a5549c58c71f9d3`; its result does not automatically validate these newly merged tests.
