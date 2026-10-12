# CodexDD 0.4.3 — Packet 3B.1d: rollout listing and search late overlap closure

**Status: SOURCE RECONCILED; read-only list/search behavior and Windows runtime validation PENDING.** Identified in the 136-path ownership audit after 3C source work; not waived merely because the prior 3B.1 documentation focused on recorder and persistence.

## Verified source ancestry

- Production baseline `4828e3b4232781963594cfaaebdc49c5531de8b1`; upstream target `rust-v0.162.0` peeled `c1382380de69521303b416720a52f42d51af6248`.
- `codex-rs/rollout/src/list.rs` source commit `ac83d64e4355c17fb7653747e0363641d0e39cd7` starts with complete upstream 0.162.0 listing source, preserving new `list_files.rs` extraction and candidate collection algorithms. Reintroduces precisely the three fork-owned `RolloutItem::WorkflowState` ignore arms: summary-head collection, head item projection, and initial-session metadata discovery. Workflow-state records cannot masquerade as model-visible session summaries.
- `codex-rs/rollout/src/search.rs` source commit `835606d5d968541aa8e85e0bf9daabfc7aa8c43d` begins with complete upstream 0.162.0 streamed search reader; adds only the fork-owned `WorkflowState` exclusion from searchable visible text snippets.

The official 0.159.2 vs production fork diff showed only those respective three and one source additions. The changes above preserve the full official 0.162.0 upstream path contents plus exactly these fork-specific exclusions.

## Gate

Targeted rollout listing, pagination, metadata discovery, and content-search tests must run with recorded `WorkflowState` and regular user-visible messages before release. This checkpoint does not change generated resources, lock, version/provenance, release authorization or installed clients. Review must also verify that upstream-only split-out listing helpers remain present.
