# CodexDD 0.4.3 — packet 3B.1b: Worker configuration vs upstream 0.162.0

**Status:** Source merge and regression-test additions checkpointed. **Static exact-source invariants PASS (10/10)**. Cargo/rustfmt, generated config-schema drift, native Windows profile, and runtime/semantic compatibility remain **UNVERIFIED**. This is a bounded continuation of 3B.1, not a completed release gate.

## Inputs and exact source authority

- Current accepted production ancestry: `4828e3b4232781963594cfaaebdc49c5531de8b1`, CodexDD 0.4.2 tracking `rust-v0.159.2`.
- Official target upstream commit: `c1382380de69521303b416720a52f42d51af6248` for `rust-v0.162.0`.
- Original v1 dry run: [discovery #37960772252](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37960772252); 22 textual conflicts, including the two files below; state remains `blocked_transplant_conflict`.
- Isolated integration branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`. Canonical candidate and production unchanged.

## Owned files and source merge decisions

### 1. `codex-rs/config/src/config_toml.rs`

**GitHub source commit:** `8b7a1ba4011aaced86b63350a74b623a742dce8d`.

Started from the **complete upstream 0.162.0 file**, which retains new `NonZeroUsize` support, `daybreak`, Guardian conversation-history overrides, and multi-channel microphone config. Applied exactly two fork-only additions:

1. CodexDD `AdaptiveWorkerRoleToml` enum (`unspecified`, `implementation`, `validation`, `repair`) plus `AdaptiveWorkerConfigToml` with required role and optional authorized scope.
2. Startup-only `ConfigToml.adaptive_worker: Option<AdaptiveWorkerConfigToml>` field, adjacent to upstream's new daybreak setting. Neither the enum nor the input is silently replaced by an upstream default.

Added two narrowly targeted Rust TOML deserialization regressions in commit `b8ce4a901f4b7d5edb08f5c07c57e74b86a3cad8`: combined `daybreak=true` and `[adaptive_worker] role="implementation"` roundtrip; unknown external Worker role rejected. The tests were **not** executed yet (native/compile gate pending).

### 2. `codex-rs/core/src/config/mod.rs`

**GitHub source commit:** `4a49b8587d33189e4ddf198d7b034b3a11a0f3e4`.

Started from the full upstream 0.162.0 version. Applied exactly the three small fork additions from production: import `AdaptiveWorkerConfigToml`, `Config.adaptive_worker`, and the mapping `adaptive_worker: cfg.adaptive_worker` in the normal `Config` constructor.

Preserved upstream `RuntimeConfigRefresh`/layer deserialization helper, Daybreak configuration, feature default map, Guardian transcript/history settings, Windows MXC constraint and user-visible TUI settings. Inspected `runtime_refresh.rs`: the refresh path clones existing `Config` before selectively updating MCP and feature fields, so this source path does **not** overwrite the existing `adaptive_worker` startup authority. This remains a source review, not proof of all runtime behavior.

## Narrow validation evidence

Ten source-level checks passed immediately after both primary file merges, including:

- Removing precisely the CodexDD role/field additions from the merged TOML file reproduced the **entire** official 0.162.0 target file byte-for-byte.
- Removing precisely the CodexDD import/field/initializer additions from merged core config reproduced the **entire** official 0.162.0 target file byte-for-byte.
- The four pre-existing Worker role variants and `authorized_scope` are present; `adaptive_worker` parses through TOML and is carried into the runtime `Config`.
- Upstream Daybreak, runtime configuration refresh, Guardian and multi-channel microphone symbols survive; no duplicated Worker fields or assignments.
- GitHub re-read all modified files after writes and confirmed expected text and blob changes.

**No cargo build, rustfmt, executable TOML test, native receipt or schema regeneration was run or claimed.**

## Deferred tests, known dependencies and exact next gate

- **3B.4 generated resources:** `codex-rs/core/config.schema.json` still has the fork's previous generated structure and must be regenerated from the fully reconciled 0.162.0 config types; do **not** hand-edit the generated JSON. A stale config schema may not include Daybreak/Guardian additions yet.
- **3B.1c persistence compatibility:** Current fork `rollout/src/persistence_metrics.rs` still uses the old boolean `is_persisted_rollout_item` filter and clones the unmodified original item. Upstream 0.162.0 adds `Cow`-owned transformations that **truncate oversized command/MCP results** before persistence. Keeping the old boolean-only measured filtering can bypass that new upstream storage truncation. Reconcile this caller and its measurement accounting in the next dedicated packet before claiming rollout compatibility.
- **3B.2–3B.5:** review session/resume/Worker lifecycle, new app-server protocol, generated exports, and focused native tests. The current isolated WIP tree remains unready to compile/release until the broader conflicts are resolved.
- **3A bridge tooling prerequisite** and its independently owner-gated merge still stand before any canonical candidate publication. If production moves, re-run exact target discovery and re-anchor identities.

**Packet result:** Both config textual conflicts have a reviewed *source-level* union of fork and upstream behavior; neither is declared release-validated or promotable. No product version, tracked upstream declaration, candidate branch or production state was changed.
