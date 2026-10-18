# CodexDD 0.4.5 — Packet 3C.4c: pinned exec-server and sandbox integrity integration

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Official 0.163.0 sandboxing/exec-server source reconciliation is staged but not accepted. Twelve new read-only Python static source tests were added and have not yet been run. No native Rust build/security test, final Windows suite, or release receipt has passed for this packet.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Exact-source provenance and scope

The original `codex-rs/sandboxing/src` and `codex-rs/exec-server/src` trees were byte-identical between pinned official upstream `rust-v0.162.0` at `c1382380de69521303b416720a52f42d51af6248` and production CodexDD 0.4.4 at `b25550dcb2c6e84a84074b603086c9db6f32a63b`. No fork customization was present in these source trees. The integration stage replaces only differing `src/` paths with exact official `rust-v0.163.0` blobs from `83086188ae0614bd882818ff1aa23f3194962012`: **44 exec-server paths** (14 new) and **11 sandboxing paths** (6 new), with no deleted files.

`codex-rs/exec-server/Cargo.toml` was already staged as the official 0.163.0 dependency manifest in 3A.3, and is preserved. No new permission, workflow publisher, repository secret, or product/version-marker modification is introduced. The static test `.github/scripts/test_codexdd_045_3c4c_exec_server_integrity.py` pins all 55 before/after blob identities; original new files are verified absent from the production baseline.

## Sandbox integrity security contract

Upstream introduces `codex_exec_server::run_integrity_checks`, completing the previously missing symbol for the staged 3C.4a async `ExecRequest::from_sandbox_exec_request` path. The new implementation is **telemetry-only**: it observes a bounded inventory of policy-sensitive executor dependencies and reports findings; findings and preparation failures do not change command execution, sandbox creation, approvals, or permissions.

The five-second `tokio::time::timeout` limits the caller's wait for the blocking scan; it does not cancel ongoing blocked work. Windows uses the MXC-specific backend (`SandboxType::WindowsMxc`); legacy Windows sandbox paths are out of scope. Approved `SandboxOverride::EscalatedSandboxWithRestrictions` calls skip preparation and telemetry before policy inspection; ordinary configured write permissions with deny-globs are not treated as approvals. Metrics have bounded, non-path tags and deliberately omit paths, environment variables, and exception text. The pinned upstream `sandbox_integrity/AGENTS.md` documents the observational contract and accepted checker approximations.

These source-level restrictions are not a substitute for native Rust tests, sandbox runtime checks, or a review of Windows elevated-SSH and CodeMode policy interactions. The CodexDD 3C.3 pre- and post-hook adaptive restrictions remain present and unchanged in `core/src/tools/registry.rs`.

## Validation status and next gates

3C.4a: **9/9 static tests passed** at detached Daniel-CL commit `e224462b750698405d4e61587367d3fa9d732ab3`, taking 0.537s. 3C.4b: 12 static checks authored; **not yet run**. 3C.4c: 12 static checks authored; **not yet run**. A source-level GitHub blob/marker review can validate the staged identities but cannot replace either native Rust execution or the cheap detached-worktree tests.

**Cargo.lock** remains the intentionally unreconciled 0.4.4 lockfile. Larger source overlaps in core session/config, upstream approval and tool callers, application server and generated schemas, and the original 109-overlap/15-conflict ledger are still open. Do not run the 21-stage Windows release suite or claim release acceptance until these are source-complete and focused native tests have passed. Keep the separate upstream publisher hard-disabled: no auto-PR, merge, release, or artifact distribution.

**Next:** 3C.5 core session/config/approval source reconciliation, then group the focused 3C.4b/3C.4c offline static tests on Daniel-CL at one exact head. Follow with native Windows sandbox and remote-output behavior checks when compilation is possible.
