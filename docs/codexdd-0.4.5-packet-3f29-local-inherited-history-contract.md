# CodexDD 0.4.5 — Packet 3F.29: preserve intentional local/inherited history non-panic recovery

**SOURCE REVIEW ONLY / NONRELEASE.** No Rust implementation or Cargo/Bazel dependencies were changed. The original 0.163 direct-conflict owners `codex-rs/core/src/context_manager/history_tests.rs` (blob `4f8415b...`) and `codex-rs/core/src/context_manager/normalize.rs` (blob `4ca88c6...`) intentionally diverge from the official debug-panic tests.

Official 0.163 has `normalize_adds_missing_output_for_custom_tool_call_panics_in_debug` and `normalize_adds_missing_output_for_local_shell_call_with_id_panics_in_debug` for non-inherited missing outputs. CodexDD permits an interrupted local Worker/session call to recover without a debug-build panic: the local missing custom-tool or local-shell response is synthesized as an `"aborted"` tool output, as is an inherited call whose parent was still running. It is not a missing upstream test to blindly port.

The fork's two `normalize_adds_missing_output_for_...` test functions each explicitly exercise `false; "local"` **and** `true; "inherited"` (four cases) without debug-assertions gating, and check resulting aborted outputs. In the normalizer, only these two missing-output branches log rather than calling `error_or_panic`; orphan-output safeguards are still active.

The source-only Python guard `.github/scripts/test_codexdd_045_3f29_history_semantics.py` pins both reviewed Git objects, preserves the two by two test variants and the non-panic implementation, and checks product/upstream markers stay `0.4.4` / `rust-v0.162.0`. It does not establish full behavioral equivalence with upstream 0.163. **Remaining native gate:** exact-source Windows `codex-core` tests for local and inherited missing custom-tool/shell outputs, interrupted replay and post-hook cancellation on a frozen release candidate. Semantic owner signoff stays `NOT_GRANTED` and PR #58 remains draft.

Packet 3F.28's bounded native optimized Rust cache probe runs independently against its immutable earlier source SHA. Do not launch a full 2,858-action optimized CLI job, merge, publish or install from either source-only result.
