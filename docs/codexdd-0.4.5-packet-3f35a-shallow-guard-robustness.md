# CodexDD 0.4.5 — Packet 3F.35A: eliminate historical Git object dependence in focused source guards

**Source-only / nonrelease.** The 3F.33 Windows focused job [#40742651126](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40742651126) failed before Rust tests started because a trailing comma after the final PowerShell `$cases` array entry was rejected. Its secure source, dependency and Windows locked resolver checks had passed. A corrected, combined startup and session native job (3F.35B) is planned, not a repeat of the 90-minute optimized CLI test.

On the now-green [3F.34 source restoration #40743349060](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40743349060), three older historical source guards still assumed intermediate Git object blobs would remain available from a shallow checkout. This can fail after unrelated workflow-only commits. Packet 3F.35A changes only these guards:

- 3F.20 startup timeout quarantine: reverse exactly three recorded 3F.32 upstream additions from the current source; then reverse its original timeout fix and verify both historical Git object SHAs.
- 3F.32 startup lifecycle: reverse its three recorded official upstream additions from current `HEAD:startup.rs`, then replay and prove exact byte equality.
- 3F.34 Daybreak/WebSocket TUI fixture: reverse its seven recorded test-only edits from current `HEAD:session_lifecycle_requests.rs`, then replay and prove exact byte equality.

The existing 3F.25 guard already reverses 3F.34 to verify its own eight historical dynamic-tool auth changes; it is executed again here, along with the all-109 owner matrix and pinned Cargo/deny source identities. New source-only Ubuntu CI keeps `fetch-depth: 2` deliberately; no native compiler or Rust source edits, release profile changes, installation or publication.

**Native plan:** on a separate fixed-source GitHub Windows workflow, compile `codex-tui` once and run the five 3F.33 startup/reconnect/daemon selectors plus restored Daybreak API-key and dynamic tool auth selectors (and a session-shutdown smoke). Source-only green is a prerequisite; `semantic_signoff` stays `NOT_GRANTED`, PR #58 draft, production 0.4.4.
