# CodexDD 0.4.5 — Packet 3C.6c: tool call routing, attribution and extension consumers

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Twenty-three pinned official 0.163.0 Rust source/test files staged and twelve focused offline static checks authored, not yet run on Daniel-CL. This remains **source-only** integration, not native Rust or security behavior proof.  
**Integration branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Reviewed upstream file ownership

All twenty-three selected `codex-rs/core/src/tools/` files matched tracked official rust-v0.162.0 byte-for-byte in the production CodexDD 0.4.4 fork. Each is now the exact pinned official 0.163.0 blob from `83086188ae0614bd882818ff1aa23f3194962012`; before/after Git blob IDs are pinned by `.github/scripts/test_codexdd_045_3c6c_tool_routing.py`.

The changed consumers cover CodeMode execution, accepted-tool-call and MCP attribution, tool-search and extension tool exposure, dynamic-tools metadata, MCP and multi-agent handlers, parallel dispatch, the tool router, and matching upstream native test fixtures. These move together to reduce signature drift between upstream 0.163.0 callers and previously staged registry/runtime handlers.

The three retired Unix zsh-fork Rust source files remain in the checkout as *orphaned* historical sources and are not re-registered by the upstream 0.163.0 `tools/runtimes/mod.rs`. Deletion is deferred for explicit review; this is a Windows-only fork and no behavior is being silently re-enabled.

## CodexDD guardrails and outstanding validation

CodexDD's previously merged 3C.3 adaptive policy in `tools/registry.rs`, custom `tools/handlers/dynamic.rs`, and fork-enriched `tools/mod.rs` were not overwritten. An extension's `PreToolUse` rewrite must still pass both initial and post-hook adaptive restrictions before execution; the new caller source cannot bypass this by policy. Whether the full end-to-end path actually enforces these restrictions remains subject to native Rust tests.

Predecessor 3C.4 and 3C.5 static checks passed 72/72 in total on Daniel-CL (2026-10-18). Packet 3C.6a/b/c adds 11+11+12 static tests, not yet executed. Remaining fork overlaps include tool-spec plan consumers, thread manager and client, and the full source-wide compilation graph. **Cargo.lock** is still the production 0.4.4 version; **native Rust** runtime checks, schema generation, Windows 21-stage suite, CI and release provenance are pending.

Automatic upstream publication remains hard-disabled; product/upstream version markers remain 0.4.4 and rust-v0.162.0. Do not release, merge, auto-open PRs or distribute artifacts on this checkpoint.
