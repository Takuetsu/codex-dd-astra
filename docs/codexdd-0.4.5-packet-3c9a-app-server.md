# CodexDD 0.4.5 — Packet 3C.9a: app-server runtime and request processor source reconciliation

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Twenty-nine exact upstream 0.163.0 app-server Rust source/test changes staged. Twelve offline static **source-only** cases authored, not run on Daniel-CL. There is no **native Rust** compilation or runtime acceptance receipt.

## Upstream/fork ownership

Compared tracked official 0.162.0 `c1382380de69521303b416720a52f42d51af6248`, production CodexDD 0.4.4 `b25550dcb2c6e84a84074b603086c9db6f32a63b`, and pinned official rust-v0.163.0 `83086188ae0614bd882818ff1aa23f3194962012`.

All 29 newly staged `codex-rs/app-server/src/` files had byte-identical 0.162.0 and production-0.4.4 blobs or were absent from both. They now match the exact official 0.163.0 Git blobs, preserving the server runtime module graph, configuration and marketplace processor paths, Windows sandbox setup semantics, thread admission and completion, remote-control pairing, and source-level native tests.

Six previously reviewed three-way CodexDD files are explicitly protected by pinned Git SHA pairs and remain unchanged: `bespoke_event_handling.rs`, `message_processor.rs`, `notification_media.rs`, `request_processors.rs`, `request_processors/turn_processor.rs`, and `thread_state.rs`. Their trusted adaptive signal/replay handling must not be displaced by a blind upstream overwrite. Two earlier upstream-only server files already matched official 0.163.0 and remain unchanged.

## Validation and gating

`.github/scripts/test_codexdd_045_3c9a_app_server.py` pins every staged original/target blob and every protected merge. No locally unfetched official commit is dereferenced. The source checks are not native server/client protocol tests.

3C.4–3C.6 static receipts: 106/106 passing on Daniel-CL (2026-10-18). The 3C.7 and 3C.8 static suites have not yet been run. The app-server request handling integration remains dependent on the current app-server-protocol reconciliations, other TUI/client consumers, and generated schemas.

**Cargo.lock**, focused native Rust tests, Windows elevation/SSH security regressions, generated schema checks, the 21-stage Windows release suite, CI and final release provenance are still pending. The automatic upstream publisher remains hard-disabled and version markers remain 0.4.4 / rust-v0.162.0. No release, merge or artifact publication is authorized.
