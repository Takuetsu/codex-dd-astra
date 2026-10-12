# CodexDD 0.4.3 — Packet 3D.1: model catalog and status display reconciliation

**Status: ALL FIVE OWNED SOURCE OVERLAPS RECONCILED, SNAPSHOT RUNTIME ACCEPTANCE PENDING.** This checkpoint is not version allocation or release approval.

## Source identity

- Branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`
- Production baseline `4828e3b4232781963594cfaaebdc49c5531de8b1`
- Official upstream `rust-v0.162.0` peeled `c1382380de69521303b416720a52f42d51af6248`

## Merged owned paths

- `codex-rs/tui/src/status/card.rs` — `b910e879712673d1c730383f6ce54a9e6a9057dc`: retained all upstream rendering and usage-link fixes and restored the fork's independent Adaptive Effort section in `/status`; remote reasoning-summary display remains server-owned.
- `codex-rs/tui/src/status/tests.rs` — `f47b11834153052f14e5738688cc088365fb9893`: updated upstream test calls for the adaptive status argument and retained a nontrivial status sample across local/remote/embedded scenarios.
- `codex-rs/tui/src/app/tests/model_catalog.rs` — `a214e7ad9f4aa973aa97cf21ff076ec9af36c5c1`: retained upstream catalog tests and CodexDD family capability assertions, explicitly checking Luna, legacy Sol, **GPT-6.1 Sol (Sol61)** and Astra as distinct authorized routes.
- `codex-rs/tui/src/status/snapshots/codex_tui__status__tests__status_snapshot_local_background_server.snap` — `f585c90786872d3d89e2054ec34d26ab995b9903`.
- `codex-rs/tui/src/status/snapshots/codex_tui__status__tests__status_snapshot_uses_default_reasoning_when_config_empty.snap` — `1c3abc15cfedba897dfe45e3a57c8cd07278ca6f`.

One newly added upstream 0.162.0 snapshot is also updated to cover the adaptive status section: `codex-rs/tui/src/status/snapshots/codex_tui__status__tests__status_snapshot_remote_server.snap` — `52a8d8849e78b853fd8982cabd274ee97dc19f75`. This file was not originally one of the five fork/upstream overlap paths.

## Policy/behavior preserved

No new automatic escalation route, model preference, effort mapping, quota policy, or permission bypass was introduced. The existing CodexDD ladder retains separate `gpt-6-sol` and `gpt-6.1-sol` stages; the verified integration source `codex-rs/models-manager/models.json` includes both model IDs and compatible reasoning effort levels.

The snapshot files use the current **unpromoted** `v0.159.2` package header as a source-development fixture while the integration's release/version owner packet `3E.2` remains pending. After final provenance/version allocation they must be regenerated and reviewed from executable snapshot tests; do not treat these hand-reconciled fixtures as passing screenshot/runtime evidence.

## Validation boundaries

- 3C.4 read-only Linux `cargo check -p codex-tui --lib --tests` run [#38014734160](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38014734160) is pinned to `d8ccd544db57de62b6be3282289284a7cddb99a7` and **does not** compile these later 3D.1 edits.
- Run targeted model-catalog/status snapshot execution on the updated source, check for snapshot drift with normal `insta` assertions, and evaluate Windows-native behavior separately.
- Model/catalog status source is reconciled, not release accepted. 3D.2 CLI source, 3E updater/build/version, 3F audit, and Windows-native Daniel-CL operator gates remain pending. Production and installed binaries stay unchanged.
