# CodexDD 0.4.3 — Packet 3C.3b: chatwidget, Worker scope and permission UI

**Status: ALL 13 REMAINING 3C.3 SOURCE OVERLAPS RECONCILED; TUI COMPILATION AND TARGETED TESTS PENDING.** Together with the earlier 3C.3a slash-command work, all 15 owned 3C.3 files are source-merged. This is not runtime or native Windows acceptance.

## Fixed target and branch

- Official `rust-v0.162.0` commit `c1382380de69521303b416720a52f42d51af6248`.
- Production baseline `4828e3b4232781963594cfaaebdc49c5531de8b1`; unchanged.
- Isolated integration branch `dd/codexdd-v0.4.3-upstream-0.162.0-integration`.

## 3C.3a carried forward

- `slash_command.rs` (`e2012371`): CodexDD `/adaptive` coexists with upstream `/daybreak`.
- `chatwidget/slash_dispatch.rs` (`a13f2f93`): retained existing Worker-bound command authorization, Daybreak and MCP confirmation behavior.

## 3C.3b source checkpoints

| Source | Commit | Fork-specific behavior retained on complete upstream file |
| --- | --- | --- |
| `chatwidget.rs` | `9a43bc7f` | Adaptive controller modules and thread-owned adaptive state |
| `chatwidget/constructor.rs` | `722fc8f6` | Initializes fresh adaptive state |
| `chatwidget/input_flow.rs` | `27afc821` | Manual prompt invalidates stale adaptive successor admission |
| `chatwidget/input_submission.rs` | `babc83c1` | Explicit manual submissions invalidate stale successor authorization |
| `chatwidget/misalignment_policy.rs` | `ff4f1a5d` | NewSession passes no implicit Worker grant |
| `chatwidget/interaction.rs` | `6f6e3cb4` | Manual ESC turn interruption terminalizes adaptive lifecycle, preserving upstream modal/keyboard behavior |
| `chatwidget/rate_limits.rs` | `93c72eba` | Adaptive budget assessment only for account usage snapshots; reset on unavailable limits |
| `chatwidget/replay.rs` | `06bcf44a` | Recovered Worker turn-duration evidence from replayed turn metadata |
| `chatwidget/protocol.rs` | `38df16ea` | Trusted runtime-signal notification handling, item evidence capture, fail-closed terminal holding, and replay-safe successor submission |
| `chatwidget/session_flow.rs` | `b8f0b0d9` | Restores thread-owned adaptive state and effective model/effort on attachment |
| `chatwidget/settings.rs` | `25eaf91e` | Reevaluate adaptive successor only after explicit model or effort change |
| `chatwidget/worktree_picker.rs` | `82f387bb` | Carries authorized Worker binding only for new managed worktrees, otherwise `None` |
| `app/tests/permission_selection_tests.rs` | `45e93e62` | Upstream's expanded server-owned settings/permission regression suite supersedes the old fixture; fork-only change was merely adding `worker_binding: None` to the now-removed old NewSession fixture |

## Security and evidence boundary

- The `protocol.rs` `hold_completed_turn_for_adaptive_signal` implementation retains the existing *authorized Worker role + nonblank scope + trusted tool output* conditions. It must not accept final answer prose as proof.
- Live notification processing, evidence registration and successor submission continue to skip replay.
- Worktree UI never turns a raw Worker role into a grant; only an explicitly bound `NewWorkerBinding` is forwarded.
- These are source-review statements, **not tested runtime conclusions**. No TUI compilation, Windows F2/daemon or interactive widget validation is claimed. The only green compatibility evidence remains 3B.5 focused Linux smoke, which predates these source commits.

## Next

Run one focused Linux `cargo check -p codex-tui` against the exact integration HEAD; resolve any upstream interface failures before requesting native Daniel-CL validation. Then reconcile 3C.4 TUI and daemon regression tests, keeping generated schemas and Cargo.lock exclusively under their owner packets.
