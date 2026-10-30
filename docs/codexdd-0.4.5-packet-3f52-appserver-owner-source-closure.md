# CodexDD 0.4.5 — 3F.52, all eleven app-server/protocol owner source deltas audited

**NONRELEASE.** Built on [3F.51](codexdd-0.4.5-packet-3f51-schema-protocol-core-tui-semantic-audit.md). This packet modifies no production Rust code or generated schema outputs.

## Scope and evidence

In 3F.51, six of the 11 `app_server_protocol` owner paths were reconstructed byte-for-byte against official `openai/codex@rust-v0.163.0` by reversing every CodexDD-specific edit. 3F.52 adds the **remaining five**: `app-server/src/bespoke_event_handling.rs`, `notification_media.rs`, `request_processors.rs`, `thread_state.rs`, and `protocol/src/protocol.rs`.

The complete **11/11 source-delta audit** is stored in `codexdd-045-3f51-appserver-upstream-delta.json` and `codexdd-045-3f52-appserver-owner-remaining-deltas.json`. `test_codexdd_045_3f52_appserver_owner_source.py` verifies the union equals exactly the 11 app-server owner paths in the immutable 109-path ledger. Every current Git object is pinned; every hunk is verified and reversed; the reconstructed bytes must SHA-1 to the exact official upstream 0.163 Git blob. The test does **not** permit a missing path or promote owner acceptance.

## Actual reviewed differences in these five paths

- **Core protocol event:** 32 added lines establish the private inert `AdaptiveRuntimeSignalKind` and event payload. Core event payload has no caller-provided thread or source-turn identity: app-server attaches those values from the owning thread and enveloping event ID.
- **Event handler:** Nine exact edit hunks introduce source-turn-scoped notification conversion and a turn-summary copy. On a successful terminal completion only, buffered signals replay before the turn-completed notification. Failed/interrupted terminals do not replay. This is the largest source patch, including a reshaped `TurnSummary` destructuring, and requires native ordering and stale-turn checks.
- **Thread state:** One import and a four-line buffered-signal field/comment carry typed summaries by thread.
- **Media notification routing:** One modified arm preserves the adaptive signal unchanged rather than treating it as media-bearing content.
- **Request processor imports:** Three type imports for the thread workflow-state API are the only source delta.

Together with 3F.51, **all eleven app-server/protocol owner paths now have machine-verifiable, complete source-difference provenance**. That is not an assertion that all 11 paths are semantically accepted: exact-source native app-server tests, cross-client authorization limits, FIFO terminal replay, mixed version negotiation, and generated stable/experimental schema/SDK regeneration remain outstanding.

**Release guard:** All 109 paths remain `PENDING_OWNER_REVIEW / NOT_ACCEPTED` and production stays 0.4.4. No full 21-stage validation, signing, runtime installation, or CI release approval is supplied by this source packet.
