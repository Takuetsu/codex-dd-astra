# CodexDD 0.4.3 — packet 3B.2e: complete core session test reconciliation

**Status:** COMPLETE SOURCE-LEVEL RECONCILIATION; native Rust tests, CI and semantic release audit **PENDING**.

## Exact basis and original fork delta

- Accepted integration baseline: production `4828e3b4232781963594cfaaebdc49c5531de8b1` tracking upstream `rust-v0.159.2`.
- Official target `rust-v0.162.0`: peeled commit `c1382380de69521303b416720a52f42d51af6248`.
- Source: `codex-rs/core/src/session/tests.rs`.
- A complete line-by-line Myers diff between official tracked 0.159.2 and **production** revealed exactly **two fork-only changed hunks**, both changing `RolloutItem::EventMsg(_) => None` to an exhaustive match that also skips `RolloutItem::WorkflowState(_) => None`. No other fork changes were found in this 13k-line test file. This was confirmed before replacing the temporary 3B.2a constructor patch.

## Actual source merge

- Commit: `4d5e2ff00edfa088e1e02b6aa401a3bb07b27d96`.
- Started from **every byte** of official 0.162.0 `tests.rs` (official Git blob `496114a113a6d8474343c887b9d244dad4b5b1be`).
- Added precisely the two CodexDD `WorkflowState` match arms to its compaction/fork model-history projections, producing blob `b1df117534bae52bd4e6dd1ecdbfc75350042f15`.
- Removing those two exact additions reconstructed the complete official upstream source **byte-for-byte**. Re-fetched the GitHub file and confirmed the published content equals the intended merged source.
- Upstream provides ten `history_revision: None` test constructor initializations, new thread/turn attribution assertions, improved compacted-history behavior, and other full 0.162.0 session tests. None of those upstream tests were overwritten by the old fork's 0.159.2 version.

## Why the interim patch was superseded

The 3B.2a interim edit `1ec23d0a88695e996f4ea98986e80a56c8cd8b8b` inserted `history_revision` in 10 fork-era constructors and one `turn_attribution: None` in a compaction expectation. The complete upstream test file now supersedes that compatibility patch. In particular, upstream 0.162.0 asserts **actual turn attribution** rather than simply assuming it is absent. The final merged source contains these official assertions while preserving the two fork-only enum handling decisions.

## Still required (do not claim verified)

1. Compile and execute native `codex-history`, `codex-rollout`, `codex-core` session/fork/compaction tests once the wider upstream source tree is reconciled.
2. Test CodexDD Worker/Designer role and exact authorized-scope restoration when resuming/forking, including unknown workflow versions fail-closed.
3. Test upstream `history_revision` freshness, nonserialization and thread store propagation; confirm client/tool and multi-agent source integration remains compatible.
4. Run native Windows targeted registry security regressions, new upstream session tests, generator consistency and full release profile later at the accepted gates.
5. The original 22-conflict discovery evidence remains `blocked_transplant_conflict`; no typed v2 recovery bridge was published or used.

**No production merge, deterministic candidate branch, product bump or Daniel-CL installation.** This packet closes the previously **partially reconciled** owner matrix row, with **source review only**.
