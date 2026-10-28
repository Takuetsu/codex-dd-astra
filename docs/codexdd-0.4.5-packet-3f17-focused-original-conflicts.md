# CodexDD 0.4.5 — 3F.17: first three original-conflict source reviews

**FOCUSED NONRELEASE EVIDENCE ONLY.** No automatic semantic or production signoff.

Three source conflicts were compared with official rust-v0.163.0 and tied to the exact secured Cargo-graph Windows [40/40 native owner receipt #40583499451](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40583499451):

- `codex-rs/core/src/tools/registry.rs`: reviewed the adaptive read-only gate **both before and after** PreToolUse input rewriting and blocked tool terminal completion. Source-specific tests cover hook rewrite, blocked dispatch, and terminal trusted-signal gating.
- `codex-rs/core/src/context_manager/normalize.rs`: upstream orphan call recovery remains, with deliberate CodexDD aborted-output synthesis for interrupted calls rather than debug panic; secure-graph native resume regression passed.
- `codex-rs/tui/src/chatwidget/interaction.rs`: upstream activation-aware key routing remains; CodexDD manual Escape notification is conditional on a genuine interrupting key press, with relevant TUI native tests passed.

Each corresponding `focused_review` in the [109-path matrix](upstream-candidates/codexdd-045-109-semantic-owner-matrix.json) pins the current Git blob, evidence run, specific Rust test selectors, reviewed invariants, and outstanding coverage gaps. The static matrix test fails closed if these claims drift.

**All 109 paths deliberately remain PENDING_OWNER_REVIEW and NOT_ACCEPTED.** This packet covers bounded behavior, not complete semantic approval, full auth/voice integrations, the final Windows archive, or the 21-stage release suite. Runtime source, Cargo/deny/Bazel dependencies and release markers are unchanged.
