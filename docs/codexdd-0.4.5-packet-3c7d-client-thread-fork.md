# CodexDD 0.4.5 — Packet 3C.7d: client and thread fork-sensitive reconciliation

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Five fork-sensitive 0.4.4 files have three-way source merges into pinned official 0.163.0. Ten offline static checks authored and not yet run. This is **source-only**, not **native Rust** compilation or behavior proof.

## Reviewed 0.162.0 → 0.4.4 fork deltas

The original 0.4.4 fork changes were independently reconstructed byte-for-byte on official 0.162.0 (`c1382380de69521303b416720a52f42d51af6248`) and then overlaid at unique contexts on complete official 0.163.0 (`83086188ae0614bd882818ff1aa23f3194962012`).

- `codex_thread.rs` — one 11-line addition: persist and flush one workflow-state event on its owning thread
- `thread_manager.rs` — seven small hunks: restore and append workflow state to child/fork snapshot
- `thread_manager_tests.rs` — one 42-line native adaptive child snapshot regression
- `client.rs` — three hunks: force required tool choice on trusted validation terminalization until the signal has been called
- `client_tests.rs` — one 63-line native terminalization tool-choice test

All five original production and reconciled Git blob IDs are pinned in `.github/scripts/test_codexdd_045_3c7d_client_thread_fork.py`. The 3C.7c direct copy explicitly excluded these paths; its source guard was subsequently updated to expect these reviewed merges. Upstream behavior outside the original fork patches was retained, but native compilation and runtime are unverified.

## Validation and remaining gates

3C.4 33/33, 3C.5 39/39 and 3C.6 34/34 static source tests passed on Daniel-CL (106/106 total). The new 3C.7a/b/c/d offline static checks total 10+10+11+10 and have not run. The original fork's complexity-floor, trusted signal and policy constraints must pass native Rust testing before release.

**Cargo.lock** is still baseline 0.4.4. Cross-crate and module-level source integration, schema generation, native Rust security and client tests, Windows 21-stage release suite, GitHub CI and release provenance remain outstanding. Product/upstream version markers remain 0.4.4 and rust-v0.162.0; automatic publishing remains disabled. Do not merge, publish or auto-release on this checkpoint.
