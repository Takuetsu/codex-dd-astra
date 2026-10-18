# CodexDD 0.4.5 — Packet 3C.9d1: TUI adaptive CLI and slash command

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Two reviewed TUI 0.163.0 source merges and ten **source-only** static tests authored, unrun. No **native Rust** compilation or interactive behavior tested.

## Narrow fork-preserving merges

`codex-rs/tui/src/cli.rs` had exactly one isolated three-line CodexDD 0.4.4 addition: the optional `--adaptive FAMILY` launcher flag. `codex-rs/tui/src/slash_command.rs` had four independent additions registering `SlashCommand::Adaptive`, its help description and relevant matcher paths. Each original 0.4.4 file was reconstructed exactly from official 0.162.0 plus these additions. Each hunk matches a unique anchor in the complete pinned official rust-v0.163.0 source before integration.

The merged and production Git blobs are pinned in `.github/scripts/test_codexdd_045_3c9d1_adaptive_cli.py`. The staged 3C.9c TUI source check now allows **only these two reviewed exceptions** to its original 44 fork-sensitive file inventory; **42** TUI conflicts remain unresolved. No other TUI fork source is overwritten.

These changes preserve visibility and CLI plumbing only. Actual TUI state handling, persisted adaptive status, complexity-floor decisions, trusted validator calls and status rendering remain subject to separate reconciliations and runtime validation. The existing custom chat-widget dispatch is deliberately untouched.

## Release gates

The upstream-0.163.0 source graph is still incomplete. Prior 3C.4–3C.6 cheap static checks passed 106/106; the 3C.7–3C.9 packets are awaiting grouped Daniel-CL static validation. **Cargo.lock**, generated schemas, cross-crate **native Rust** compilation, elevated Windows SSH tests, GitHub CI and release artifacts remain blocked. The automatic publisher remains hard-disabled and production versions are not advanced.
