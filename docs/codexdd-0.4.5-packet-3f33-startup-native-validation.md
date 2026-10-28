# CodexDD 0.4.5 — Packet 3F.33: targeted Windows native startup/reconnect/daemon regression

**NONRELEASE NATIVE VALIDATION; NOT SHIPPING APPROVAL.** Packet 3F.32 source validation [#40742304689](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40742304689) succeeded after bounded historical Git blob fetch, verifying exactly three official 0.163 upstream restoration fragments in `startup.rs`, plus the original 3F.20 TimedOut quarantine and the complete 109-path source matrix.

To avoid a repeat of the costly 21-stage suite, run one source-frozen `windows-2022` job with **five** selected native `codex-tui` tests, each using `cargo test --locked -p codex-tui --lib ... -- --test-threads=1` on the same compiled test binary:

1. `async_fresh_startup_applies_adaptive_preset_before_attachment`: adaptive startup preservation.
2. `fresh_startup_thread_drains_buffered_approval_before_draft_handoff`: startup input boundary.
3. `reconnect_exhaustion_and_unknown_initial_thread_stay_offline`: failed reconnect state.
4. `reconnect_allows_slow_hydration_but_bounds_a_stalled_server`: reconnect/hydration lifecycle.
5. `daemon_startup_falls_back_only_for_implicit_endpoints`: managed-daemon startup fallback.

Preflight pins the actual new `startup.rs` blob `e51254ea2f16f2a356670413ae29c9de0d002e9d`, protected Cargo/deny/vendor state and adjacent TUI conflict source; re-runs 3F.32, 3F.20 and 3F.14 source-only guards before Cargo. The workflow uses installed Rust 1.95.0 and preconfigured Rusty V8; compiler/test artifacts are local to the runner and only short logs are uploaded. 60-minute hard job ceiling, no FULL release compile or 21-stage run.

**Limitations:** This is a compilation and focused behavioral smoke of related existing tests, not a dedicated WSL clipboard prewarm or daemon join end-to-end test; those require further targeted/manual validation and full release suite on the final immutable source. These five selectors cannot approve all 109 source owners or optimized CLI/combined EXE/PDB. Product stays 0.4.4 until release signoff.

## Outcome and supersession

The original [3F.33 Windows workflow #40742651126](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40742651126) ended **RED** before invoking any Rust test. Dependency/security, exact startup source, Cargo metadata and Rusty V8 setup all passed. The PowerShell `$cases` array had a trailing comma after its final element, causing `ParserError: Missing expression after ','`. This was a **workflow scripting defect**, not evidence of a startup regression; do not count any of its five cases as passed.

The next validation is the one-shot **3F.35B** combined focused Windows job. It uses the same unchanged startup source and now-restored test-only Daybreak/WebSocket harness, eight selectors in a correctly terminated PowerShell array, and the green 3F.35A source-provenance rechecks. No full optimized CLI or production promotion.
