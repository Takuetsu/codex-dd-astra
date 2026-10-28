# CodexDD 0.4.5 — 3F.40A core history recovery source review

**NONRELEASE; no Rust product source edits.** Official upstream Codex `rust-v0.163.0` retains `error_or_panic` for a missing, noninherited CustomToolCall/LocalShellCall output in debug builds. CodexDD intentionally keeps `info!` there and synthesizes an `aborted` output so interrupted local tasks, inherited worktree prefixes and resumed sessions can recover without debug panic. The fork retains **strict debug panics for orphan outputs** (no matching call) and still uses stable synthetic output IDs. Restoring upstream's two missing-output panic statements without a controlled migration would break the interrupted-work recovery contract.

Exact current source blobs:
- `codex-rs/core/src/context_manager/normalize.rs` — `4ca88c6a668e9896df06ca81940a0e41e357898d`
- `codex-rs/core/src/context_manager/history_tests.rs` — `4f8415b4ce3f3994d4fe8eb87bcbd22e6588848d`.

In the fork's `history_tests.rs`, both local and inherited variants are deliberately executed for each missing custom/local-shell output case. Official upstream restricts local variants to non-debug builds, and carries two missing-output `should_panic` debug tests that would directly contradict CodexDD's required recovery. Separately, CodexDD's orphan-output panic tests are retained for debug builds (function/custom/tool-search), providing a fail-closed path for truly orphaned result data.

**Prior Windows core evidence:** [#40583499451](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40583499451) succeeded with core tests including `resumed_history_recovers_orphan_custom_tool_call_without_panicking`. This does not prove both local/inherited history-normalization branches. Packet 3F.40A's source-only audit pins both original conflict source identities, test-case registration, protected orphan panic handling, security manifests and still-draft 109-path owner matrix. A bounded source-frozen Windows core test can then exercise the full matrix without rerunning the optimized release binary.

**Separate completed TUI gate:** [3F.39 Windows #40760413516](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40760413516) **GREEN**, 8/8 native startup/reconnect/daemon/Daybreak/auth/session tests; source-frozen commit `8f3ddf6edd46d8c246c829ecc1a9c79a2f10754e`. Updated corresponding owner rows to retain provenance, not grant signoff. Follow-up WSL clipboard/daemon exit end-to-end tests and full final-suite acceptance remain outstanding.

**Still blocked:** every one of the 109 semantic owners remains `PENDING_OWNER_REVIEW`/`NOT_ACCEPTED`; optimized Windows CLI/PDB/combined archive, true release package, final Daniel-CL 21-stage suite, merge and deployment. Production remains 0.4.4.
