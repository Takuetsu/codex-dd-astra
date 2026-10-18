# CodexDD 0.4.5 — 3C.7b thread/task/client shared consumers

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Eight exact official 0.163.0 Rust source/test files staged, ten static source checks authored but unrun. **source-only** staging; **native Rust** validation still pending.

## Source ownership

Seven paths (`thread_manager/shutdown.rs`, `thread_rollout_truncation_tests.rs`, three `tasks/` files, `client_common.rs` and `client_common_tests.rs`) were byte-identical in original official 0.162.0 and production CodexDD 0.4.4. New `thread_spawn.rs` was absent from both. They are now exact pinned 0.163.0 Git blobs. Original and current Git blob IDs are fixed in `.github/scripts/test_codexdd_045_3c7b_thread_task_client.py`, without dereferencing the upstream commit locally.

This source packet covers multi-agent child preparation/shutdown, review and task-abort lifecycle, request-image handling, and native Rust regressions. The fork-sensitive `client.rs`, `thread_manager.rs`, `codex_thread.rs` and CodexDD adaptive-security sources remain unchanged.

## Validation and release gates

The prior 3C.4/3C.5/3C.6 checkpoint is **106/106 static tests green**. The new 3C.7b ten static checks are unrun. The branch remains partial. **Cargo.lock**, cross-crate API reconciliation, native Rust compile and security tests, Windows 21-stage suite, CI and release provenance remain pending. Production markers remain 0.4.4 / rust-v0.162.0, and automatic publishing is disabled.
