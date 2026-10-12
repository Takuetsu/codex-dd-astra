# CodexDD 0.4.3 — packet 3B.2b: canonical state, replay and session persistence

**Status:** SOURCE MERGED; narrow exact-upstream/fork-overlay checks passed. No Rust compiler, Windows native test or release receipt was run. This is an isolated work-packet checkpoint, **not** a clean candidate or an approved semantic review.

## Inputs

- Production base: `4828e3b4232781963594cfaaebdc49c5531de8b1` (`0.4.2`, tracked `rust-v0.159.2`).
- Pinned target: official `rust-v0.162.0` commit `c1382380de69521303b416720a52f42d51af6248`.
- Branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`; original `blocked_transplant_conflict` discovery [run 37960772252](https://github.com/Takuetsu/codex-dd-astra/actions/runs/37960772252).
- Parent packet: [3B.2a](codexdd-0.4.3-packet-3b2a-history-fork-revision.md) reconciled the history revision and adaptive child-fork state.

## Exact six-file source union

| File | GitHub commit | Fork-only retained behavior |
| --- | --- | --- |
| `codex-rs/core/src/session/rollout_reconstruction.rs` | `8d5aef17504c` | Four `WorkflowState` exclusions at session/context reconstruction boundaries |
| `codex-rs/state/src/extract.rs` | `ea13f46f6e4e` | Two `WorkflowState` exclusions from ordinary state/metadata extraction |
| `codex-rs/state/src/runtime/threads.rs` | `cc2b411a56ec` | One `WorkflowState` exclusion from thread-derived metadata |
| `codex-rs/thread-store/src/thread_metadata_sync.rs` | `c3399f3181ea` | One `WorkflowState` exclusion from metadata changes |
| `codex-rs/thread-store/src/local/rollout_migration/canonicalizer.rs` | `472d96252cb5` | One `WorkflowState` **preservation** as a durable rollout item in canonicalized storage |
| `codex-rs/core/src/session/session.rs` | `070c0c76e003` | One `WorkflowState` handling arm in ordinary session initialization |

Each file was started from the **full, official upstream 0.162.0 version**, then amended exclusively with the listed fork-owned `WorkflowState` match arms. Four updates completed before a connector batch limit was hit; each path and blob was independently refetched to identify successful commits, and the remaining two files were then updated in separate calls. No successful source write was blindly retried.

For every file, the fork-specific match arms are the **only** intended difference from its corresponding official upstream file. The authoring step independently checked that removing its intended insertion(s) recovered the upstream source byte-for-byte. The first four paths were also independently checked after publication. The last two were verified against the original upstream source in their successful per-file commits.

## Why these distinctions matter

- `WorkflowState` contains CodexDD's durable adaptive Worker snapshot and must survive canonicalized JSONL migration/resume.
- It is not a user conversation item, ordinary thread attribute, runtime-version selector, tool output, or session message; it must be excluded from metadata projections and event reconstruction.
- Upstream 0.162.0's state extraction, thread synchronization, replay and session initialization changes are retained in full rather than reverting to stale 0.159.2 fork files.
- These are semantic source-level placement decisions pending unit/integration tests. Maintaining exhaustive enum-match arms does not itself prove runtime correctness.

## Validation and outstanding gates

- Re-read all six source paths after GitHub writes; exact new blob commits present in isolated branch.
- Verify all affected `RolloutItem` match sites remain exhaustive with the CodexDD variant without changing upstream's other arms.
- **3B.2c** must reconcile remaining core lifecycle/client/tool/session consumers and audit `core/src/session/tests.rs` against 0.162.0 (it currently has a constructor-only compatibility patch, not a complete merge).
- **3B.5** owns native unit-test execution, compiler checks and cross-crate semantics. **3F** owns final all-overlap and Windows F2 audit.
- All 136 original overlap paths must remain explicitly tracked with resolution states; no native-validated state may be assigned until actual tests pass.
- The separate v2 conflict-recovery tooling bridge and its owner-approved production merge remain prerequisites for publication of the deterministic canonical candidate.

**No product-version change, production merge, preparation-state rewrite or machine installation.**
