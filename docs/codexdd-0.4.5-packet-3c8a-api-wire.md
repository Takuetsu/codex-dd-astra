# CodexDD 0.4.5 — Packet 3C.8a: API, app-server client and executor wire

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Fifteen exact official Codex 0.163.0 source/test blobs staged with eleven source-only static Python guards authored but not run. There is no **native Rust** compilation or runtime authorization receipt.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`

## Reviewed source ownership and preservation

The four selected crates were byte-identical to official rust-v0.162.0 in CodexDD 0.4.4 prior to this packet: `app-server-client`, `app-server-test-client`, `codex-api` and `exec-server-protocol`. Their 15 changed source/test paths were staged as exact Git blobs from pinned official rust-v0.163.0 at `83086188ae0614bd882818ff1aa23f3194962012`. The manifest `.github/scripts/test_codexdd_045_3c8a_api_wire.py` pins each original (or absent) 0.4.4 path and final blob; it does not dereference unfetched upstream commits in Daniel-CL's offline worktree.

- `app-server-client`: five source/test blobs, including three new server-request reply and regression modules. Requests retain distinct resolve and reject handles across in-process and remote transports.
- `app-server-test-client`: one source blob adding/updating request-response and integration-client support for the pinned protocol.
- `codex-api`: eight source/test blobs spanning request-error bridging, HTTP search, websocket handshakes and SSE response decoding.
- `exec-server-protocol`: one upstream wire protocol file, preserving executor and sandbox request shape compatibility with staged 3C.4a/c runtime changes.

No source change is made to CodexDD's adaptive registry, trusted signals, policy boundaries, Windows-only release tooling, production version markers or upstream auto-publisher.

## Validation and outstanding dependencies

The 3C.4–3C.6 checkpoints are **106/106 static checks passed** on Daniel-CL. 3C.7 added 41 source-only static checks not yet run; 3C.8a adds 11 more not yet run. All source receipts are partial until dependent protocol/app-server implementations and fork-specific paths have been reviewed and exercised.

This packet cannot prove remote websocket/exec-server integration, live tools, approval rules, or resumption. **Cargo.lock**, generated schemas, native Rust tests, the Windows release suite and GitHub CI still require acceptance after the source graph is complete. The separate automatic publisher remains hard-disabled. Do not bump production, merge or publish.
