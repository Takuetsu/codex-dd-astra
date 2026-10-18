# CodexDD 0.4.5 — 3C.7c pinned upstream core root source integration

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** 48 exact official 0.163.0 root-level Rust source/test blobs staged, 11 static tests authored but unrun. **source-only** staging; **native Rust** compilation/security validation outstanding.

## Ownership and source changes

Official 0.162.0 commit `c1382380de69521303b416720a52f42d51af6248` and production CodexDD 0.4.4 `b25550dcb2c6e84a84074b603086c9db6f32a63b` had identical Git blobs on the 48 staged `codex-rs/core/src/*.rs` paths, or both lacked the newly introduced files. Every staged blob is exact official 0.163.0 from `83086188ae0614bd882818ff1aa23f3194962012`. Before/after identities are pinned in `.github/scripts/test_codexdd_045_3c7c_core_root.py`, which requires no local upstream commit.

The staged sources include `lib.rs`, CodeMode host, retained-output and remote compaction, hook lifecycle, execution policy, MCP tool attribution, shell snapshots, native event mapping, Windows sandbox and associated Rust tests.

**Five fork-sensitive files deliberately excluded:** `client.rs`, `client_tests.rs`, `codex_thread.rs`, `thread_manager.rs` and `thread_manager_tests.rs`. These are still exactly the production 0.4.4 fork blobs and **have not yet been merged** with upstream 0.163.0. The Python source tests explicitly guard against overwriting them. Other cross-crate and module-level integration remains open.

## Validation and release gates

3C.4 33/33, 3C.5 39/39, 3C.6 34/34 static tests passed on Daniel-CL (106/106 total). New 3C.7a/b/c static checks (10+10+11) have not run. **Cargo.lock**, schema generation, native Rust compilation and runtime security tests, Windows 21-stage suite, CI and final release provenance remain pending. Product version stays 0.4.4 and tracked upstream marker stays rust-v0.162.0. Automatic publishing remains disabled.

Next: reconcile pending fork-sensitive client/thread files, then run the grouped cheap 3C.7 source tests before attempting native builds.
