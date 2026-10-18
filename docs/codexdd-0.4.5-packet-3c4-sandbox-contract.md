# CodexDD 0.4.5 — Packet 3C.4a: sandbox override propagation source staging

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE**. Six pinned upstream source/test blobs staged, without fork overlap. The nine read-only static checks are added but **not passed or run** at this checkpoint. No native Rust compilation or security runtime tests have passed for this packet.  
**Integration branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Provenance and source ownership

The following six files had identical Git blobs in tracked official `rust-v0.162.0` and production CodexDD 0.4.4 at `b25550dcb2c6e84a84074b603086c9db6f32a63b`. This packet copies exact pinned official `rust-v0.163.0` source bytes from commit `83086188ae0614bd882818ff1aa23f3194962012`; no three-way fork policy merge is needed for these six paths.

| File under `codex-rs/` | Production 0.4.4 blob | Staged official 0.163.0 blob |
| --- | --- | --- |
| `core/src/tools/orchestrator.rs` | `3d7e2b62c5227910980c56fc6be5f426d35e20c1` | `e7f544458bafcb27fb9b86bd8b5828e95cbc6baf` |
| `core/src/tools/sandboxing.rs` | `bf1fd2d79e6adae7bcb7cc1aa7cd36df1221bced` | `d49273beb37fadf1dda778ef5ba8a047438dff69` |
| `core/src/tools/sandboxing_tests.rs` | `95731415baf4906cf0a2834b991264df0cff9ba4` | `a4839cfd33da520cb53b41107a038a9e60f7470c` |
| `core/src/sandboxing/mod.rs` | `69f6ff90447ffd6389bc0d0972c3cf4d6f4bfcea` | `e3ff8053b0f29a68e8bd4e3d10d5d6bdad230598` |
| `protocol/src/sandbox.rs` | `b0304634ad8fb45be9b35b48591ef6c6a4e62099` | `47fd51cbce1785a08cf199bf52effa93dd9315a3` |
| `core/src/tools/runtimes/apply_patch.rs` | `45cc7e2e9b08e163f50054fd43051091f629eb44` | `981958b17a18c06a88a5f618e2d5fc685d539188` |

## Why these six belong together

Official upstream moves `SandboxOverride` out of the tool-local enum into the shared protocol; `SandboxAttempt` now explicitly retains it for initial, retry and network-approval attempts. Tool preparation is asynchronous and calls `codex_exec_server::run_integrity_checks`. The apply-patch exec-server context now carries the override as well. The existing fail-closed denied-read check for `EscalatedSandboxWithRestrictions` on `SandboxType::None` is preserved; the upstream Windows restricted-token regression becomes an async test and also checks inherited override propagation.

This is **source staging**, not proof these paths compile together on the unfinished fork. It is not permission to bypass sandbox enforcement, accept hook-rewritten commands, change approval policies, or automatically publish builds. The 3C.3 registry policy helpers remain intact. The two appended 3C.4 Rust fixtures (unrun) cover policy evaluation on a rewritten payload and terminal `Blocked` lifecycle reporting for an initially denied tool. Neither fixture runs a live hook rewrite end-to-end; lifecycle behavior after an actual hook mutation remains a separate native integration gate.

## Mandatory follow-on source and validation gates

The connected `core/src/tools/runtimes/unified_exec.rs`, `core/src/tools/handlers/unified_exec.rs`, other async `env_for` callers, `exec-server` and file-system sandbox-context types, and upstream tool/schema consumers still require reviewed reconciliation. In particular, official 0.163.0 removes local zsh-fork routing; do not silently transplant that larger removal into fork behavior without its callers/tests. Resolve dependent API ownership before a native Rust compile receipt.

`.github/scripts/test_codexdd_045_3c4_sandbox_contract.py` contains **nine** offline source/provenance checks, including exact Git blobs and preserved native-security test fixtures. They have **not passed** here. On the clean, exact detached integration head on Daniel-CL, run this small packet first; on failure repair the single affected section. Full Cargo.lock renewal, native Rust tests, generated schema validation, Windows release suite, CI acceptance, product/upstream markers and release publication remain blocked until the source graph is complete. The original 109 overlap / 15 conflict ledger still requires closure and evidence.

The separate automatic upstream publication branch remains source hard-disabled. No PR, merge, release, elevation or artifact publication is authorized by this checkpoint.
