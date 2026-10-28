# CodexDD 0.4.5 — 3F.40B Windows core history normalization native validation

**NONRELEASE; compile targeted `codex-core` tests once, no optimized CLI build.** This is the focused native follow-up to [3F.40A source audit #40765637530](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40765637530) GREEN and historical [15-core+25-TUI Windows suite #40583499451](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40583499451) GREEN.

The official Codex 0.163 overlap makes two code differences explicit: upstream debug builds panic on a locally missing CustomToolCall/LocalShellCall output, while CodexDD deliberately synthesizes an `aborted` output for both local and inherited/interrupted calls to permit safe context recovery. However, orphaned outputs with no corresponding call still trigger strict debug panic (fail closed).

The job uses the current exact-source 0.4.5 review branch with unchanged `normalize.rs` blob `4ca88c6a668e9896df06ca81940a0e41e357898d`, unchanged core `history_tests.rs` blob `4f8415b4ce3f3994d4fe8eb87bcbd22e6588848d`, protected secure Cargo/deny/vendor source and 109-row owner matrix. The same Rust 1.95.0 MSVC toolchain, repository-pinned Rusty V8, and `--locked` resolver as the green 3F.39 run are used. Eight cargo test selectors:
1. `normalize_adds_missing_output_for_custom_tool_call`
2. `normalize_adds_missing_output_for_local_shell_call_with_id`
3. `normalize_adds_missing_output_for_tool_search_call`
4. `normalize_removes_orphan_function_call_output_panics_in_debug`
5. `normalize_removes_orphan_custom_tool_call_output_panics_in_debug`
6. `normalize_removes_orphan_client_tool_search_output_panics_in_debug`
7. `normalize_keeps_server_tool_search_output_without_matching_call`
8. `resumed_history_recovers_orphan_custom_tool_call_without_panicking`

Selectors 1 and 2 each run **two local/inherited variants**; the others verify tool search synthesis/retention, three expected debug `should_panic` cases for invalid orphaned results, and resumed custom tool recovery. This gate is bounded to 60 minutes and continues through all selectors to report every failure. A green result would not sign off 109 owners, prove release-profile replay/optimized CLI/PDB, or replace the final 21-stage Daniel-CL test suite. No runtime code, dependency source, installer, release marker or production installation changes. CodexDD 0.4.4 remains installed.

## Verified native result — 2026-10-28

[3F.40B Windows run #40765870072](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40765870072) is **GREEN** on the immutable candidate `509b6b306d86773bb3e4ada4cd62b16f052345ba`. The 8 selected `codex-core --locked --lib` test filters executed **10 native Rust test cases total**: 2 local/inherited CustomToolCall variants, 2 local/inherited LocalShellCall variants, and six singleton tests (tool search synthesis, 3 expected debug orphan panics, server-side orphan search retention and resumed-history recovery). All passed; no skips or exclusions were reported by the focused run. Cargo locked resolver and secured source identity preflights passed. This is focused native evidence, **not 109-owner final signoff** or optimized release readiness.
