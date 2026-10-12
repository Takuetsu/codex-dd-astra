# CodexDD 0.4.3 — packet 3B.3a: adaptive workflow protocol source reconciliation

**Status:** FOUR PROTOCOL SOURCE OVERLAPS RECONCILED. GitHub blob/byte checks complete; Rust compile, schema generator, app-server tests and native Windows verification **not yet run**.

## Pinned source and input evidence

- Production CodexDD 0.4.2 branch SHA `4828e3b4232781963594cfaaebdc49c5531de8b1`; prior tracked upstream `rust-v0.159.2`.
- Selected official `rust-v0.162.0` commit `c1382380de69521303b416720a52f42d51af6248`.
- Original [fixed target read-only conflict report](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37960772252) remains state `blocked_transplant_conflict`. The deterministic canonical candidate remains unpublished.

## Source changes

| Path | Commit | CodexDD behavior retained |
| --- | --- | --- |
| `codex-rs/app-server-protocol/src/protocol/thread_history.rs` | `fd60007ebbcbd788fcd386ed56ac92003aad143a` | Durable internal `RolloutItem::WorkflowState` is not projected as a user-facing history message |
| `codex-rs/app-server-protocol/src/protocol/thread_history_projection.rs` | `964ab66d139126293171aecd909ba0b6d48242b5` | Same internal workflow record does not produce a thread history change notification |
| `codex-rs/app-server-protocol/src/protocol/v2/thread.rs` | `21f2dbf1918c522464411c8e9b297b62ec86d6c3` | Existing `ThreadWorkflowStateOperation`, `ThreadAdaptiveValidationStatus`, `ThreadAdaptiveWorkflowState`, `ThreadWorkflowStateUpdateParams`, and `ThreadWorkflowStateUpdateResponse` DTOs |
| `codex-rs/app-server-protocol/src/protocol/common.rs` | `51f19010ba8343977145075b6c6b640cc4652cb2` | `thread/workflowState/update` RPC plus experimental `turn/adaptiveRuntimeSignal` notification registered with upstream's complete request/notification protocol |

Each merge started from the full official upstream 0.162.0 source file. The fork-specific additions are exactly reversible to the official source, without discarding any intervening 0.160.x/0.161.x source work. GitHub source contents were re-fetched following the changes. This is **source-level** proof, not a generator artifact or native build receipt.

## Invariants and dependencies

- Internal adaptive snapshots are **durable in rollout storage** but not model-visible thread messages; routine history projections skip those entries.
- The workflow-state RPC must remain bound to its thread ID and server-side authority; its public representation does not authorize arbitrary remote mutation without server validation.
- The adaptive runtime notification remains experimental; do not silently make it a stable or unprotected protocol route.
- Remaining `v2/turn.rs` and app-server request processors must be reconciled before the packet can pass semantic review.
- App-server JSON/TS/Python and compressed precomputed schemas are owned by **3B.4**, and must be regenerated from the reconciled source types, **never** modified as hand-written outputs.
- Rust source/test checks, F2 privilege behavior, terminalization trust and native Windows release profile are still explicit gates.

**No source tests executed, production merge, candidate ref, version change, CI promotion or installation.** This is an auditable nonpromotable integration checkpoint.
