# CodexDD 0.4.3 — packet 3B.1a: rollout persistence source reconciliation

**Status:** Source changes complete, static invariants PASS. **Native compilation, integration tests and semantic release acceptance PENDING.** This is a deliberately bounded subdivision of 3B.1.

## Owned paths and source decisions

1. **codex-rs/rollout/src/policy.rs** — started with official upstream rust-v0.162.0 persistence implementation (target object 3e5cc07e6ca62ffa13e2a104a5f815a0d4fa40f0). Kept upstream's Cow-based durable item projection, bounded command/MCP tool-result truncation, new into_persisted_rollout_items interface and updated ResponseItem treatment. Retained CodexDD's WorkflowState rollout item as durable, and AdaptiveRuntimeSignal as transient/non-durable. Added a compatibility is_persisted_rollout_item boolean predicate backed by the upstream authoritative projection so existing fork persistence telemetry still compiles by signature.
2. **codex-rs/rollout/src/lib.rs** — retained CodexDD workflow-state/history public exports and appended upstream's persisted_rollout_item and into_persisted_rollout_items reexports; preserved the old is_persisted_rollout_item reexport for existing callers. Did not overwrite CodexDD session recovery types.

## Source evidence

- Production baseline: 4828e3b4232781963594cfaaebdc49c5531de8b1.
- Target rust-v0.162.0: c1382380de69521303b416720a52f42d51af6248.
- WIP scaffold parent: 6f8c7692988181630483e2aa6602e622f6884e35.
- Policy source commit: ad1c4ee4d69ef35dafe9c914be62eaa53519c4a4.
- Public export source commit: df77e9c70b6102f318de8e83a848e0c577188ffb.
- Policy textual conflict from discovery 37960772252 is now represented as a **reviewed source-merge implementation**, **not** marked fully validated or resolved for canonical publication.
- lib.rs is an additional semantic overlap, separately audited here.

## Narrow static validation performed

Confirmed, from the current integration branch after both source commits:
- Upstream rollout crate manifest supplies new codex-utils-output-truncation and codex-utils-string dependencies.
- CodexDD history crate still contains WorkflowState types; the new policy retains WorkflowState and transient AdaptiveRuntimeSignal matches.
- The new upstream item projection and owned-item API exist; previous fork boolean persistence telemetry still has a matching symbol.
- Public reexports contain both new upstream persistence functions and CodexDD WorkflowStateItem plus adaptive snapshots.
- All static invariants PASS. GitHub blob content was re-fetched after each source change and checked for exact expected bytes.

**Not tested:** Cargo/Rust compilation or rustfmt, runtime preservation/truncation with real recorded sessions, memory/CPU cost of using the new projection from telemetry, Windows-native persistence/resume/fork regression. These belong in the subsequent focused 3B.5 and Phase 4 validation. Source compilation may still fail from unrelated unreconciled 0.162.0 overlaps.

## Deferred work and next packet

- **3B.1b:** config_toml.rs and core/src/config/mod.rs remaining two textual conflicts, plus related semantic configuration changes.
- **3B.1c:** rollout recorder/persistence_metrics and any other inherited upstream persistence API caller changes, with focused tests before declaring 3B.1 fully compatible.
- **3B.2–3B.5:** full Worker/session lifecycle, app-server, regenerated protocol and native tests.
- Continue the separated bridge contract and owner-gated merge before final candidate publication. The 0.4.2 clean-only promotion contract remains unchanged; source branch is not promotable.

No production, canonical candidate branch, product version or installation was changed.
