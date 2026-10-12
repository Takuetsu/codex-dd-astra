# CodexDD 0.4.3 — packet 3B.1c: bounded rollout persistence and resumed reader

**Status:** GitHub source integration complete with static compatibility checks **12/12 PASS**. A known cross-packet compile dependency on `ResumedHistory.history_revision` is **unresolved** (3B.2). No native build, generator run, runtime persistence test, Daniel-CL receipt or release validation is claimed.

## Scope and reason

The 3B.1a compatibility helper `is_persisted_rollout_item` retained the 0.4.2 boolean policy interface. However, the old fork `persistence_metrics.rs` filtered via that boolean and cloned **the raw input**. The new 0.162.0 source projection can return an **owned, truncated item** for oversized command and MCP results. The old filter would bypass the bounded stored representation even when the policy says `kept`.

Packet 3B.1c reconciles exactly two Git semantic overlaps from the 136-path owner matrix. Source target: official `rust-v0.162.0` commit `c1382380de69521303b416720a52f42d51af6248`; fork parent: production `4828e3b4232781963594cfaaebdc49c5531de8b1`; original blocked discovery: [37960772252](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37960772252).

## Source work

1. **`codex-rs/rollout/src/persistence_metrics.rs`** — commit `ab51756f8fe215549e0aec2ec7a843a6018ca925`. Adopted the entire upstream 0.162.0 source, retaining **only** the fork's `RolloutItem::WorkflowState(_) => "codex_dd_workflow_state"` classification. The actual writer path now uses `persisted_rollout_item` and appends `projected.into_owned()`, not the old boolean and original clone. Pre- and post-projection serialized lengths, per-stage metrics, bounded histogram buckets and `bytes_removed` telemetry are retained from upstream, including the ability to account for trimmed command/MCP results.
2. **`codex-rs/rollout/src/recorder.rs`** — commit `ab156e5e8ca92e41b93abc6b7ed2c283235f817e`. Adopted the entire upstream 0.162.0 rollout writer/reader code, adding **only** the fork's `RolloutItem::WorkflowState(_)` ignore arm for latest-turn cwd scanning. Keeps upstream `compression::read_rollout_lines`, state DB pagination error propagation/continuation handling and history revision initialization. Historical legacy-ghost-snapshot handling remains present in the upstream reader.

Neither file is assumed compatible solely because an exact source union was created; all currently unresolved dependencies stay visible.

## Narrow evidence

Twelve checks passed, including a byte-for-byte reconstruction of **both** official target files after removing the single CodexDD-specific addition in each file. This establishes that no other upstream code was silently omitted. Static checks confirmed:

- The writer receives `Cow`-projected item content and persists the **projected owned value**.
- Post-filter measurement measures the **persisted** bytes, rather than incorrectly counting pre-truncation sizes.
- Upstream output-truncation thresholds, bytes-removed metrics and histogram boundaries remain available.
- The staged upstream `persistence_metrics_tests.rs` includes an oversized command-output regression at twice the bounded command-output limit; that test is **not executed** yet.
- The reader uses the new compressed reader helper; it preserves CodexDD's `WorkflowState` handling.

## Unresolved cross-packet compile dependency (must fix in 3B.2)

The 0.162.0 `recorder.rs` initializes `ResumedHistory { history_revision: None, ... }`. The current fork-preserved `codex-rs/history/src/lib.rs` still defines `ResumedHistory` **without** this new field. The official target history struct adds a `#[serde(skip)] pub history_revision: Option<String>` field. **The current WIP source tree must not be represented as compilable until 3B.2 reconciles that type and all history constructors/consumers.**

The same 3B.2 packet must confirm CodexDD workflow-state serialization/resume/fork survives the upstream revision contract.

## Pending tests and gates

- Reconcile `codex-rs/history/src/lib.rs`, state DB, core session consumers, and dependent constructors in **3B.2**.
- Run the relevant `codex-rollout` persistence/metrics and history resume unit tests after codebase compilation becomes possible, including actual oversized command/MCP truncation, exact persisted lengths, durable `WorkflowState` and transient `AdaptiveRuntimeSignal`.
- Review all remaining 3B.1 overlaps and perform the subsystem exit audit; **3B.1 is not yet declared fully validated**.
- Regenerate config/protocol schemas in 3B.4; proceed through mandatory Daniel-CL validation only at the appropriate source-stable gate.
- Keep additive conflict-reconciliation bridge/tooling as a separate validated prerequisite before any canonical candidate publication.

**No changes** were made to `dd/astra-policy-v2`, `codex-rs/codexdd-version.txt`, `codex-rs/upstream-codex-release.txt` or the deterministic candidate branch. Original discovery remains `blocked_transplant_conflict`.
