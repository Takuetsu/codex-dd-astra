# CodexDD 0.4.5 — Packet 3C.9b: protocol permissions, errors and environment tests

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Three exact official 0.163.0 source/test blobs staged, ten offline **source-only** static checks authored but not yet run. Not **native Rust** compilation, a live permission test, or acceptance for publishing.

## Exact source review

`codex-rs/protocol/src/error.rs`, `permissions.rs` and `shell_environment_tests.rs` were identical between official rust-v0.162.0 and production CodexDD 0.4.4. They now match the pinned official 0.163.0 blobs. Old/new identities are in `.github/scripts/test_codexdd_045_3c9b_protocol.py` without requiring the upstream commit in Daniel-CL's local Git object store.

The upstream error conversions, deny-glob and protected agent metadata policy helpers, and `CODEX_TOOL_CALL_ID` environment sanitization tests are staged as a group. The earlier pinned official `shell_environment.rs` and the reviewed CodexDD three-way `protocol.rs` history/workflow extensions remain byte-for-byte unchanged. The new source does not authorize filesystem escalation, bypass tool hooks, or downgrade the policy checks.

## Remaining gates

Prior 3C.4–3C.6 cheap static source tests passed 106/106 on Daniel-CL. The new 3C.7, 3C.8, 3C.9 source-only static tests are still pending. Permission boundary behavior still requires compiled native Rust tests and the Windows sandbox suite.

**Cargo.lock**, generated protocol schemas, app-server/tui integration, native security/approval tests, the full Windows release suite and CI remain gated until source-complete. The upstream auto-publisher remains hard-disabled; product version 0.4.4 and upstream marker rust-v0.162.0 are unchanged.
