# CodexDD 0.4.5 — Packet 3C.8b: feature and plugin integrations

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Fifteen official 0.163.0 source/test Git blobs staged and twelve focused **source-only** static guards added but not yet run. This is not **native Rust** policy validation or approval to publish.

## Provenance and fork ownership

The source changes in `codex-rs/features` and `codex-rs/core-plugins` were compared against official rust-v0.162.0 `c1382380de69521303b416720a52f42d51af6248`, the production CodexDD 0.4.4 fork `b25550dcb2c6e84a84074b603086c9db6f32a63b`, and pinned official rust-v0.163.0 `83086188ae0614bd882818ff1aa23f3194962012`.

All three changed `features/src` and all twelve changed `core-plugins/src` files had identical official-0.162.0 and production-0.4.4 blobs. They are staged as exact official 0.163.0 blobs, with complete before/after SHA pairs in `.github/scripts/test_codexdd_045_3c8b_features_plugins.py`. Any CodexDD plugin-fork additions and unrelated unmodified files are left intact.

## Contracts and validation gates

This packet stages Guardian feature-mode parsing and validation, plugin-source marketplace restrictions, cached feature/recommendation catalogs, scoped layer-stack loading, executor hook routing, and the associated Rust native fixtures. Core plugin/marketplace updates do **not** authorize new providers or change local trust decisions; retained `marketplace_policy` and CodexDD adaptive dispatch remain separate gates.

Earlier 3C.4–3C.6 static validation is 106/106 green on Daniel-CL (2026-10-18). The unrun source-only 3C.7 (41 tests), 3C.8a (11), and 3C.8b (12) checkpoints remain pending for grouped cheap validation. Source provenance is not runtime enforcement proof.

**Cargo.lock**, native Rust compilation/security tests, generated protocol schemas, remaining CLI/TUI/app-server source reconciliation, Windows 21-stage release suite and GitHub CI are outstanding. The publisher stays hard-disabled and production remains 0.4.4 with rust-v0.162.0 marker. No merge, release or automatic artifact publication is approved.
