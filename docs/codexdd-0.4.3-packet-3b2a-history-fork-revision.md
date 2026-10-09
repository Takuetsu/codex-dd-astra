# CodexDD 0.4.3 — packet 3B.2a: history revision and adaptive child-fork state

**Status:** SOURCE CHECKPOINTED; **no Rust compilation, CI, native release PASS or final semantic acceptance claimed**. The existing v1 original transplant remains blocked. This packet explicitly reconciles the upstream `ResumedHistory.history_revision` extension and CodexDD's durable adaptive Worker state, without changing production.

## Exact identity and boundaries

- Production ancestry: `4828e3b4232781963594cfaaebdc49c5531de8b1`, currently product `0.4.2`, tracked `rust-v0.159.2`.
- Official upstream target: `rust-v0.162.0`, peeled SHA `c1382380de69521303b416720a52f42d51af6248`.
- Original source inventory: 136 overlapping paths, 22 textual conflicts, state `blocked_transplant_conflict`; source branch is isolated and not the deterministic candidate.
- Prior packet 3B.1c exposed missing `ResumedHistory.history_revision` on the staged history struct. This work fixes the type, record constructors and review/test coverage, rather than stubbing out the upstream revision.

## Source and tests checkpointed

| Source file | Commit | Review decision |
| --- | --- | --- |
| `codex-rs/history/src/lib.rs` | `689273818b5762a48360197d30b71d7ffe42f570` | From complete upstream source, restore fork's durable `WorkflowState` type/variant, schema and authoritative latest-valid-record logic. Preserve upstream Guardian history delivery proof and `#[serde(skip)] pub history_revision: Option<String>`. |
| `codex-rs/history/src/compaction_resume_metadata.rs` | `5abb90263ced99c84ece1273350a9ce93ed15749` | Preserve upstream `TurnAttribution` across resume and skip CodexDD `WorkflowState` as irrelevant to multi-agent runtime inference. |
| `codex-rs/history/src/tests.rs` | `14befb381e8fd9ef22f5daf94111b3bc07374791` | Restore the five existing fork workflow record/snapshot regressions, update variant schema cardinality to 13, retain upstream history/attribution constructors and add test that a store-issued revision is *not serialized* into durable history. |
| `codex-rs/core/src/thread_manager.rs` | `ea74947905611e642c7985d7e352a8ae0439c31c` | Start from entire 0.162.0 thread-manager source (including startup admission, root-turn and original stored revision), add the seven small existing fork changes for child workflow-state snapshots, including safe restoration for copied/referenced fork. |
| `codex-rs/core/src/thread_manager_tests.rs` | `8079071bab0b2a5644a5b4272ff3b2c22ae0af27` | Start from full upstream test file (including all three revised-history constructors), preserve exact CodexDD child snapshot test. Owner remains 3B.5 for final test validation. |
| `codex-rs/core/src/session/tests.rs` | `1ec23d0a88695e996f4ea98986e80a56c8cd8b8b` | **Compatibility-only interim edit** to fork-retained test file: explicitly initialize `history_revision: None` in ten resume constructors and `turn_attribution: None` in one compaction expected value. **The rest of this large overlapping file still needs full 0.162.0 reconciliation**, including updated attribution expectations. |

The first five files were constructed as an **exact source union**: deleting the enumerated CodexDD-specific insertions reproduces the entire official upstream 0.162.0 file byte-for-byte. Tests were not executed, and exact-string reconstruction is not equivalent to Rust type or runtime correctness.

## Compatibility and security implications

- `history_revision` is store-issued and `#[serde(skip)]`: it must never become a persisted privilege/authorization marker.
- Forking copies/adapts adaptive Worker workflow state only through the explicit child snapshot path. It must not silently inherit pending runtime signals, successor permits or other ephemeral governance data.
- Upstream session startup membership and root-turn identity must remain intact; child fork source mutation invalidates source snapshot revision (represented as `InitialHistory::Forked`).
- New upstream Guardian delivery and compaction turn attribution must stay compatible with CodexDD state restoration.
- Existing generated history/schema data may need generator regeneration after later source updates; do not hand-edit compressed or binary exports.

## Pending checks and next packet

1. **3B.2b:** merge source-level `WorkflowState` arms in state extraction, store canonicalization and session rollout reconstruction without losing the 0.162.0 source behavior.
2. **3B.2c:** reconcile core session/turn history, thread state and tests beyond the interim constructors. Confirm native tests exercise fork state + upstream revision/Guardian attribution simultaneously.
3. Confirm all `ResumedHistory` and `CompactionResumeMetadata` constructors in other runtime/test directories on the candidate branch are updated. The above file searches cover selected known paths, not a complete repository-wide grep.
4. Narrow Rust and native Windows validation only after the conflicting source tree is sufficiently reconciled and the appropriate work-packet gate is met.

**No production merge, release, canonical candidate publication, version allocation or install is authorized by this checkpoint.**
