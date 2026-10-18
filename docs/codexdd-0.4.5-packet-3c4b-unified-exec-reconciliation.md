# CodexDD 0.4.5 — Packet 3C.4b: unified-exec and shared sandbox API source reconciliation

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Twenty pinned official 0.163.0 source/test blobs are staged, with 12 read-only Python source tests added but not yet executed on Daniel-CL. There is no native Rust compilation, Windows release suite, schema/lockfile approval, or CI release acceptance for this packet.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Exact provenance and ownership

Every one of the **18 `codex-rs/core/src` files** in this packet had a Git blob identical to tracked official `rust-v0.162.0` (`c1382380de69521303b416720a52f42d51af6248`) and production CodexDD 0.4.4 (`b25550dcb2c6e84a84074b603086c9db6f32a63b`). The same is true of the two shared-type companions `codex-rs/protocol/src/shell_environment.rs` and `codex-rs/file-system/src/lib.rs`. Thus none of these 20 paths contained a fork customization that a direct upstream stage could erase. Their committed blobs match the pinned official `rust-v0.163.0` source at `83086188ae0614bd882818ff1aa23f3194962012`.

`.github/scripts/test_codexdd_045_3c4b_unified_exec.py` pins original production SHA and exact 0.163.0 SHA for all 20 paths and checks source-level invariants in twelve offline Python tests. These tests have **not run** on the new head; provenance inspection is not a native behavior test.

## Sources reconciled as one ownership group

- **Unified-exec handlers and runtime:** `tools/handlers/unified_exec.rs`, `handlers/unified_exec/exec_command.rs`, `handlers/unified_exec_tests.rs`, `runtimes/unified_exec.rs`, `runtimes/unified_exec/snapshot_metrics.rs`, `runtimes/mod.rs`, `runtimes/mod_tests.rs` and `runtimes/apply_patch_tests.rs` are exact official 0.163.0 source. Upstream removes the Unix-only zsh-fork shell-mode route and changes the direct-runner constructor/snapshot API. Historical orphaned zsh-fork files remain untouched, with their deletion deferred to a reviewed cleanup.
- **Process lifecycle and remote output:** `unified_exec/mod.rs`, `mod_tests.rs`, `oneshot.rs`, `process.rs`, `process_manager.rs`, `process_manager_tests.rs`, `process_tests.rs`, `async_watcher_tests.rs`, and `shell_snapshot.rs` carry upstream remote-output drain/retention handling and its native tests. `process_manager.rs` awaits `SandboxAttempt::env_for` and `env_for_exec_server`, matching the staged 3C.4a sandbox API. The new output-retention behavior must be compiled and exercised before acceptance.
- **Process environment:** `core/src/exec_env.rs` and `protocol/src/shell_environment.rs` provide canonical `CODEX_TOOL_CALL_ID` injection. The Windows implementation drops existing case-insensitive copies before inserting the current call ID; input containing a NUL is not installed.
- **Wire sandbox metadata:** `file-system/src/lib.rs` serializes and reconstructs `FileSystemSandboxContext.sandbox_override`, retaining the separate policy check that prevents denied-read escalation without a native filesystem sandbox. This does not grant extra permissions.

All pre-existing CodexDD adaptive-signal/native-validation handlers and the custom 3C.3 registry policy restrictions remain unchanged. No edits were made to production or tracked-upstream version markers, workflows, release distribution, or publisher permissions.

## Validations and open blockers

The earlier **3C.4a nine static tests passed 9/9** on the detached Daniel-CL head `e224462b750698405d4e61587367d3fa9d732ab3` in **0.537s** on 2026-10-18, per operator screenshot. That receipt does not cover the new 3C.4b sources.

3C.4b remains source-incomplete: `codex-rs/exec-server/src/lib.rs` and its new upstream sandbox-integrity module must supply `run_integrity_checks`; associated exec-server transport/environment and sandbox consumer APIs remain to reconcile. Session/config source users, remaining fork-vs-upstream overlaps, generated schemas and the original 109-overlap/15-conflict ledger remain open. **Cargo.lock** is still deliberately the 0.4.4 baseline. Native Rust compilation and security/runtime tests have not run or passed for this packet. The 21-stage Windows suite is premature.

**Next packet:** 3C.4c — reconcile the official exec-server sandbox-integrity/transport dependency set and sensitive approval/sandbox consumers. Then run grouped focused static checks, followed by native Rust checks only when the source graph is complete.

Automatic upstream PR publication and artifact release remain source hard-disabled; no PR/merge/version bump/artifact shipment is authorized by this checkpoint.
