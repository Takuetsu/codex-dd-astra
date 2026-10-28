# CodexDD 0.4.5 — Packet 3C.1: core history and interrupted tool normalization

**Date:** 2026-10-18  
**Status:** INCOMPLETE / NOT A RELEASE CANDIDATE. Core history conflict source and updated tests staged for later full native Rust testing, with **10/10 inexpensive static guards PASSED on Daniel-CL in 0.166s at `7f4db9fa8e02aadf203e30164829ec463c696252` (operator-provided screenshot, 2026-10-18)**.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Reviewed three-way inputs

| File                                                 | Official 0.162.0                           | CodexDD 0.4.4                              | Official 0.163.0                           | Reconciled Git blob                        |
| ---------------------------------------------------- | ------------------------------------------ | ------------------------------------------ | ------------------------------------------ | ------------------------------------------ |
| `codex-rs/core/src/context_manager/normalize.rs`     | `7fc45ba217a3b7b733ee6225132f7c5eac99ed8c` | `0f60bdc82016c45ff035667778d20aba0ee298f3` | `532cfdfb34a74f774c41a9a1914347a2492997de` | `4ca88c6a668e9896df06ca81940a0e41e357898d` |
| `codex-rs/core/src/context_manager/history_tests.rs` | `c5551b632048455dfda3263d48f498832c52f334` | `c2268ac3c142b8a788afa375f3b832b4c393464a` | `8f22910ed2086c07edbd41f84a1327705f4700f3` | `4f8415b4ce3f3994d4fe8eb87bcbd22e6588848d` |

**`normalize.rs` conflict:** The fork replaced debug panics on missing custom-tool and local-shell call outputs with informative aborted-output recovery for persisted interrupted turns. The official 0.163.0 code adds an `inherited_user_message` metadata guard to avoid panicking for copied in-progress calls in preserved-prefix forks. The integrated source starts from the complete official 0.163.0 file and changes only these two guarded branches: inherited calls **do not log an error**, while non-inherited interrupted calls log an INFO diagnostic and synthesize an `aborted` output **without panicking**. Both still insert deterministic synthetic output next to the call. This preserves the fork's resume tolerance and the upstream inherited-call metadata contract.

**`history_tests.rs` conflict:** The full official 0.163.0 test file is preserved, including new `ResponseItem` encrypted/status fields and encrypted compaction replay-cache coverage. Its two parametrized missing-output tests now run **both local and inherited cases in debug and release**, since CodexDD must not panic for either. The two obsolete debug `#[should_panic]` tests for missing custom-tool/local-shell outputs are removed. Other orphan-call debug panic tests are retained unchanged. The pre-0.163.0 fork already removed the custom-tool panic test but had retained a contradictory local-shell panic test; this packet makes the test contract consistent with 0.4.4 runtime behavior.

## Validation and unresolved semantic dependencies

`.github/scripts/test_codexdd_045_3c_history.py` adds 10 **source-only** checks of exact Git blob provenance, inherited guard count, both abort-output paths, unconditional parametrized tests, retained unrelated panic tests, encrypted replay test and unchanged product markers.

**No native Rust compile, runtime history replay, or Windows tests have passed for 3C.1.** The updated history tests reference `ResponseItem` fields introduced by upstream 0.163.0 that are not yet integrated throughout the fork; compiling this partial branch is premature. All 109 overlapping paths require owner-reviewable reconciliation, including the remaining **15 original conflict paths** (two of which are now staged here but not accepted as release proof). Later targeted core/history tests must exercise local/inherited custom-tool and shell calls, failed outputs, deterministic synthetic IDs and resumed-fork metadata before full 21-stage native Windows validation.

**Next:** stage the other source-level integration packets (3B protocol, 3C tools/guard policy, 3D TUI/startup) and non-overlap upstream dependencies before cargo/lockfile regeneration. Preserve the original blocked discovery artifact and future v2 conflict sidecar evidence; this checkpoint alone cannot authorize candidate publication, release version bump, CI PASS, or merge.
