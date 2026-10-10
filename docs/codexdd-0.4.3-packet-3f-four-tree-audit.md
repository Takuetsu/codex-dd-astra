# CodexDD 0.4.3 — 3F four-tree source-completeness audit

**Status: ALL SOURCE INVENTORY CATEGORIES RECONCILED WITH EXPLICIT EXCEPTIONS; executable and Windows-native tests remain pending.** This is not a release approval.

## Immutable source identities examined

| Role | Commit | Git tree |
| --- | --- | --- |
| Tracked official upstream 0.159.2 | `ff6aec96948b70d94983af2641a6b67c94faeff5` | `406dfdd5c68f303a3a8d04f32b3965b3b0ca0361` |
| Unmodified CodexDD production | `4828e3b4232781963594cfaaebdc49c5531de8b1` | `f486f0b7ec5a909849246ee91f01af098c08421a` |
| Official target upstream 0.162.0 | `c1382380de69521303b416720a52f42d51af6248` | `4899ef4940a8bc7fdf7873aa0d3f438085b8163f` |
| Isolated integration checkpoint | `0970d29ebcfbd27eb5092e6415a8ef5f2ca077a2` | `ed9b2acde421f2279baf19294be96ae27b28eaeb` |

All four GitHub recursive trees were fetched untruncated and indexed by blob path/SHA; the comparison included added/deleted paths as well as file content modifications. Blob counts old/production/new/integration: 8,703 / 8,779 / 9,017 / 9,123.

## Cross-tree classification

- Production fork delta against official 0.159.2: **257** changed paths.
- Official upstream 0.159.2→0.162.0 delta: **2,507** changed paths.
- Path intersection: **136**, exactly matching the immutable ownership audit and its original classification.
- **2,369 of 2,371 upstream-only changed paths** have identical Git blob hashes in current integration and the official 0.162.0 target.
- **118 of 121 fork-only changed paths** have identical Git blob hashes in current integration and production, retaining fork-only sources without unnecessary rewrites.
- All **136 overlap paths** have explicit source owner/status; **134** differ in bytes from both parent trees, while **two** deliberately match the full upstream target. A source path matching upstream is not independently sufficient proof that the previous fork behavior is safe; these two were examined below.
- **30 integration-only paths** were created after scaffold/reconciliation: 29 work-packet/audit documents plus a temporary read-only test workflow. No unexplained new runtime source path was found among that category.

### Two upstream-only paths requiring intentional changes

1. `codex-rs/tui/src/app/tests/navigation_reconnect_tests.rs`: the upstream test still awaited a bounded channel's `send` future, but the reconciled CodexDD TUI now uses an unbounded FIFO channel. The test was updated to call immediate `send` and preserve the receive assertion; source compile passed in [#38015213286](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38015213286).
2. `codex-rs/tui/src/status/snapshots/codex_tui__status__tests__status_snapshot_remote_server.snap`: a new upstream 0.162.0 remote snapshot must include CodexDD's independent Adaptive Effort section. Its change is documented by 3D.1; source snapshot expectations require executable `insta` verification.

### Three fork-only paths intentionally updated for new upstream APIs

1. `codex-rs/core/src/tools/handlers/codexdd_validation.rs`: adapted the CodexDD-only validation handler to the new upstream source command/exec compatibility contracts; targeted core CI previously passed in [#37992789061](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37992789061).
2. `codex-rs/tui/src/chatwidget/tests/adaptive_effort_tests.rs`: adapted the CodexDD-only thread fixture to official 0.162.0 `ThreadSessionState` (Daybreak field present, old personality field removed).
3. `codex-rs/tui/src/chatwidget/tests/adaptive_pressure_tests.rs`: supplied required `root_turn_id` in three CodexDD-only terminalization fixtures without weakening adaptive test expectations. TUI test-source compilation passed in [#38015213286](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38015213286).

### Two audited overlap paths exactly matching upstream

1. `codex-rs/cli/tests/daemon_startup.rs`: the old fork modified a restrictive-launcher warning assertion to tolerate elevation, while the official 0.162.0 test separately exercises elevated interactive startup (including resume/fork and explicit daemon denial) versus the ordinary restrictive-launcher warning. This intentional upstream supersession is documented in 3C.4. **Windows-native execution still required.**
2. `codex-rs/tui/src/app/tests/permission_selection_tests.rs`: the old fork's only extra source delta used an explicitly scoped `AppEvent::NewSession { worker_binding: None }`. The updated upstream 0.162.0 test already uses that event API and adds server-owned permission selection protections, so the correct union is byte-equal to upstream. This does not waive trusted Worker scope testing.

## Independent CI and release gates

- [#38015213286](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38015213286) verified TUI test-source compilation and CLI source compilation only.
- [#38015689709](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38015689709) verified all generated lockfile external package identity tuples match the official 0.162.0 lock.
- [#38015850664](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38015850664) was started for a read-only, pinned `--locked` executable gate; its final result must be reviewed and is not treated as passing in this report.
- No version/provenance promotion, production merge/install, full Windows binary build or elevated SSH F2/daemon test has occurred. No Daniel-CL action is needed solely to close this static source audit.

This four-tree comparison is a source-completeness proof with explicit exception ownership, not a behavioral regression or release-security proof.
