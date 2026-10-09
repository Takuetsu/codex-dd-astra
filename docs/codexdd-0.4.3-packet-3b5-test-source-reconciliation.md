# CodexDD 0.4.3 — Packet 3B.5: core regression-test source reconciliation

**Status: ALL EIGHT OWNED TEST SOURCE OVERLAPS RECONCILED. Native Rust execution and wider integration semantic acceptance PENDING.** This is an isolated 0.162.0 source-development checkpoint, not release acceptance.

## Source identity and scope

- Live production baseline at start of 0.4.3: `4828e3b4232781963594cfaaebdc49c5531de8b1` tracking `rust-v0.159.2`.
- Official upstream target `rust-v0.162.0`: peeled commit `c1382380de69521303b416720a52f42d51af6248`. Each merge began with the **entire target file**, then retained only the previously audited fork-specific source difference.
- Branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`. Original discovery `blocked_transplant_conflict`, canonical candidate unpublished.

## Reconciled paths

| Path | Commit/evidence | Fork-specific behavior retained |
| --- | --- | --- |
| `core/src/client_tests.rs` | `23463bdabd05935f80b83b4de6ca90b526655ecb` | The 63-line Validation terminalization model-request tool-choice test |
| `core/src/tools/spec_plan_tests.rs` | `23463bdabd05935f80b83b4de6ca90b526655ecb` | Mechanical-validation runner visibility and runtime-bound adaptive signal tool exposure tests |
| `core/src/session/rollout_reconstruction_tests.rs` | `23463bdabd05935f80b83b4de6ca90b526655ecb` | Interrupted orphan custom-tool call recovery during resumed history, without panic |
| `core/src/context_manager/history_tests.rs` | `800328ae8d3b5e66eabea02f42ee086b64760ff2` | Former fork's two debug-test guard removals and removal of the contradictory custom-tool should-panic test |
| `core/tests/suite/scenarios.rs` | `7a85d82036991b0535e1bacd18a9d10fc5c515dc` | Remove the unused old `ReasoningEffort` import while retaining all new upstream scenarios |
| `core/tests/suite/unified_exec.rs` | `cfd6ed337ee72b4a929803cd098b3f4126536798` | Runtime-native exec evidence-ID propagation into subsequent model request and adaptive signal |
| `core/tests/suite/agent_websocket.rs` | `4ded033753d44cf8e22b499d1ff1fdcb3633e9e7` | Code-mode nested exec result/evidence-ID over WebSocket; retain all upstream test cases |
| `core/src/thread_manager_tests.rs` | Existing 3B.2 source merge | Already reconciled, kept source-merged pending native validation; no unnecessary rewrite in 3B.5 |

The three 3B.5a additions were individually checked against complete 0.159.2→production source deltas: removing only the retained CodexDD regression-test blocks reconstructs the exact official 0.159.2 input, and removing those same blocks from the merged output recreates 0.162.0 byte-for-byte. The remaining changes retained their full upstream 0.162.0 source and preserved fork-only tests/expected debug semantics. Files were committed without overwriting unrelated new upstream tests.

**Important distinction:** these are source-content checks. No debug/release Rust assertion, Cargo test, Windows signal/provenance runtime test, or final security check is claimed solely from byte alignment.

## Current focused compile/test gate

Read-only isolated [GitHub Actions core compatibility smoke #37982039843](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37982039843) is the packet's first narrow core/app-server compile and regression suite. Treat its run outcome as authoritative only after it reaches a terminal success/failure state. The job covers core and app-server `cargo check`, library-level adaptive and resumed-history regressions, the app-server trusted signal completion replay, and two newly reconciled core integration scenarios.

Cargo may refresh `codex-rs/Cargo.lock` during compilation; its exact reconciliation is independently owned by **3E.2**. This read-only test workflow is not allowed to push or merge and cannot generate a release receipt.

## Deferred Gate 3 and Windows validation

- Any failed core/app-server compile or test blocks 3B.5 acceptance; fix sources and rerun the focused check. Tests skipped due external network policy are not treated as runtime proof.
- Run the broader session/fork/history/worker and owner-authorization regression profiles after all 3C/3D/3E source work. Verify runtime signal origin is trusted and no client can spoof terminalization.
- Native Windows release validation on Daniel-CL remains an explicit operator gate, not replaced by these Linux-targeted Rust checks.
- No product version/provenance change, production merge, v2 conflict-recovery publication, canonical candidate, Daniel-CL install or owner-gated release change occurs in 3B.5.

**No Daniel-CL action needed yet.**
