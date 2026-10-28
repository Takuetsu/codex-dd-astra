# CodexDD 0.4.5 — Packet 3C.3: tool registry security and cancellation reconciliation

**Date:** 2026-10-18  
**Status:** INCOMPLETE / NOT A RELEASE CANDIDATE. Registry source merge checkpointed, with **16/16 cheap static source tests PASSED on Daniel-CL in 0.195s** at `aa99738c10db4503e7bcbf26c418340348b98b0b`, confirmed by operator screenshot (2026-10-18). Subsequent commits appended two unrun native Rust fixtures (hook-rewritten input rejection and terminal Blocked lifecycle on initial policy denial) and updated the matching static tripwire; **no new validation receipt exists for those changes**. Native Rust compile, runtime tests, integration reviews, and release receipt remain outstanding.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`

## Three-way inputs and exact Git provenance

| Path                                        | Official tracked 0.162.0                   | Production CodexDD 0.4.4                   | Official pinned 0.163.0                    | Reconciled                                       |
| ------------------------------------------- | ------------------------------------------ | ------------------------------------------ | ------------------------------------------ | ------------------------------------------------ |
| `codex-rs/core/src/tools/registry.rs`       | `e0acae262e8216748c162264d59e98c831302076` | `5b67eb29406d04b021b0fd9a5e259eccdd7da3b2` | `ebde5a7bdc449a11dc9c1fc1d8475c8e4432010f` | `fc75b3bbf91f1881bd32faaaca93356fbb598b5e`       |
| `codex-rs/core/src/tools/registry_tests.rs` | `895d8d4b997d5fbd3d5401855f4088ef3d6cfb3a` | `b9af52dbb0b771e0b9119d5166b6a047c21bd24c` | `895d8d4b997d5fbd3d5401855f4088ef3d6cfb3a` | Existing production blob `b9af52db...` unchanged |

The entire registry starts from official upstream `rust-v0.163.0`, not the 0.4.4 fork. Official 0.163.0 adds `OrCancelToolExt` cancellation propagation through active-turn locks, hooks and memory state, dynamic-tool `finishes_on_cancellation` support, immutable/overridden specs, `ToolSearchInfo`, `FunctionsNamespaceFunctionPrefixes` and per-step tool metadata. None of these upstream source changes was dropped in the targeted merge.

## CodexDD policy preserved with stronger hook boundary

The **entire 393-line block defining 17 original CodexDD adaptive restriction helpers** is copied byte-for-byte from 0.4.4. Their PowerShell command lowering, read-only reconnaissance allowlist, mechanical validation command allowlist, file-edit denial, terminalization signal-only policy and safe Git command checks are unchanged. The original native Rust policy tests (`registry_tests.rs`, fork blob `b9af...`) are preserved byte-for-byte as the file prefix; 3C.4 adds two unrun Rust tests: hook-rewritten input validation and terminal Blocked lifecycle reporting without a tool-start event. The official 0.162.0-to-0.163.0 delta for that test file is empty.

The original fork's pre-dispatch policy rejection is inserted after payload-kind checks and **before PreToolUse hooks**. A second invocation of the same three restriction helpers runs **after** optional `with_updated_hook_input` and immediately **before** `notify_tool_start`. This extra fail-closed boundary prevents a PreToolUse hook from rewriting an initially allowed read-only command into a disallowed mutating command and then executing it. Both rejection branches record failure analytics and terminally report `ToolCallOutcome::Blocked` via upstream `notify_tool_finish_if_unclaimed`, reducing the risk of an unfinished tool lifecycle. Neither policy check allows a new tool type, credential bypass, auto-publisher or production mutation.

This extra post-hook check is a new integration safeguard and **must be verified by native Rust behavior tests** on the source-complete integration branch. A static assertion cannot prove authorization or lifecycle correctness at runtime.

## Validation checkpoint and remaining work

`.github/scripts/test_codexdd_045_3c_registry_security.py` adds **16 read-only source tests**: exact reconciled Git blob, unchanged fork native-test prefix and new 3C.4 hook-rewrite and blocked-lifecycle fixtures, 17 helper functions exactly equal to pinned production bytes, three adaptive triggers, distinct allowlists, terminal-only signal, PowerShell/Git unsafe-command protections, upstream spec/search/cancellation additions, guard placement before and after hooks, terminal blocked lifecycle, unchanged 0.4.4 product/upstream markers and explicit unfinished release status.

**No native Rust validation has passed for this source packet.** Later staged compilation must exercise hook-mutated inputs, cancellation races, forbidden source-write commands, allowed bounded test commands and false-identity/terminal signal rejection. Cargo.lock still reflects the pre-integration 0.4.4 graph. Some upstream data-model/core users, policy modules, app-server events, 109 overlapping paths and 15 original conflict sites remain to reconcile. Generated schemas and production markers must not be advanced yet.

The upstream publication automator remains **source hard-disabled** in its separate feature branch. No CI, merge, version bump, artifact distribution, PR creation or privilege change is authorized by this checkpoint.
