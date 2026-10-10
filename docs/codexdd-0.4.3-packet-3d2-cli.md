# CodexDD 0.4.3 — Packet 3D.2: CLI adaptive-flag reconciliation

**Status: SINGLE OWNED CLI SOURCE OVERLAP RECONCILED; EXECUTABLE TESTS AND WINDOWS VALIDATION PENDING.**

- Source branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`
- Production: `4828e3b4232781963594cfaaebdc49c5531de8b1`; upstream target: `c1382380de69521303b416720a52f42d51af6248` (`rust-v0.162.0`).
- Reconciled `codex-rs/cli/src/main.rs` at [`faf0ac943f`](https://github.com/Takuetsu/codex-dd-astra/commit/faf0ac943f949c0da2a703e83a784d1262a3b0ea): began from entire official upstream 0.162.0 file and reapplied exactly the four-line CodexDD `adaptive` flag extraction/preservation in `merge_interactive_cli_flags`.
- Upstream 0.162.0 sandbox/uninstall subcommands, remote-control no-daemon forwarding, and exec argument tests remain intact.
- An explicit subcommand `--adaptive` value must continue to override root/default interactive adaptive preferences when constructing resume/fork launch arguments, while no subcommand override preserves existing state.

No routing-policy or model-family change was introduced. A source-content merge does not demonstrate argument/PTY/runtime acceptance. Native Windows CLI, F2, elevated daemon denial, and broader 3D.2 validation remain pending. Cargo.lock, version/provenance and production are not touched. No operator action is required for this source checkpoint.
