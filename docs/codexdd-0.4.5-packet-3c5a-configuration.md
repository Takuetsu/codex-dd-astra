# CodexDD 0.4.5 — Packet 3C.5a: core and config-crate reconciliation

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Twenty-two exact official upstream 0.163.0 source/test files staged plus two reviewed three-way adaptive worker merges. Fifteen offline static checks added but **not yet run** on Daniel-CL. Native Rust compilation/security and release tests remain unrun.  
**Integration branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Provenance: three-way preservation

Official 0.162.0 base: `c1382380de69521303b416720a52f42d51af6248`; CodexDD 0.4.4 production baseline: `b25550dcb2c6e84a84074b603086c9db6f32a63b`; pinned official 0.163.0 target: `83086188ae0614bd882818ff1aa23f3194962012`.

The core module `codex-rs/core/src/config/mod.rs` had **exactly three fork additions** relative to official 0.162.0: importing `AdaptiveWorkerConfigToml`, carrying `Config.adaptive_worker` as optional external authority, and reading `cfg.adaptive_worker` into loaded configuration. The external config-crate file `codex-rs/config/src/config_toml.rs` had **exactly three fork additions**: strict `AdaptiveWorkerRoleToml`/`AdaptiveWorkerConfigToml` declarations, optional `ConfigToml.adaptive_worker`, and two native tests for parsing a valid worker while rejecting an unknown `administrator` role. Both original fork source files were reconstructed byte-for-byte from 0.162.0 plus those additions *before* applying the same additions to 0.163.0. No 0.163.0 upstream source in these two files was deliberately removed.

Reconciled source blobs are pinned by `.github/scripts/test_codexdd_045_3c5a_config.py`: `core/src/config/mod.rs` = `17680eeef69718965df66cc12466985c84bdcd63`; `config/src/config_toml.rs` = `a07f08c8831dd011d4950a52088c7233722620d8`. The same source checks pin **13 core config leaf files** and **9 config-crate leaf files** (22 total), all unchanged by the production fork and staged as exact pinned official 0.163.0 Git blobs.

## Adaptive-security contract

`adaptive_worker` remains *optional launcher-supplied* role/scope metadata. Its only permitted values remain `unspecified`, `implementation`, `validation`, `repair`; unknown roles fail deserialization and the worker struct rejects unknown fields. This metadata does not grant execution privilege, allow source edits before accepted reconnaissance, bypass denied reads, change trusted-signal rules, or authorize publishing. The CodexDD 3C.3 pre/post-hook registry guard and native validation tools were not modified in this packet.

Official configuration additions for runtime refresh, permission profiles, managed restrictions, network configuration, Windows sandbox settings, schema generation and updated config tests are copied from pinned 0.163.0. Native Rust/config deserialization behavior still requires an integrated source graph and runtime tests.

## Validation evidence and remaining gates

The predecessor **3C.4 checkpoint passed 33/33 static tests** on exact detached Daniel-CL commit `cae522cf20d1893d7537e713f234cba098798a59`: 3C.4a 9/9 in 0.506s, 3C.4b 12/12 in 1.334s, 3C.4c 12/12 in 2.607s (operator screenshot dated 2026-10-18). This evidence does not cover the 3C.5a changed sources.

3C.5a has **15 new offline static tests, not yet run**. No native Rust compilation or security/runtime behavior has been validated. Session/turn configuration consumers still have fork/upstream overlaps; approval/guardian modules and remaining app-server consumers must be reconciled. Generated schemas, the final source-wide build and **Cargo.lock** dependency resolution are intentionally pending. Do not run the 21-stage Windows release suite yet. Product/upstream markers remain CodexDD 0.4.4 and rust-v0.162.0.

**Next:** 3C.5b session/turn and guardian/approvals reconciliation, preserving fork-specific changes. Group the next cheap Daniel-CL static checks after a meaningful stage; then native compilation when source-complete. Automatic upstream PR publishing, merging, and artifact distribution remain hard-disabled.
