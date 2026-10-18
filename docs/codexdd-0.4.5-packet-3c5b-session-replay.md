# CodexDD 0.4.5 — Packet 3C.5b: core session and trusted event replay reconciliation

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Seventeen exact official upstream session source/test files and five reviewed three-way session merges staged. Thirteen offline static tests **13/13 passed** on Daniel-CL at exact detached commit `c1a582730766302a1295ce6e9051bbaf20e56dbe` in **1.463s**, 2026-10-18 (operator screenshot). No native Rust compile, runtime test, generated schema or release receipt for this packet.  
**Integration branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Source provenance and fork-specific event preservation

Official upstream 0.162.0 baseline `c1382380de69521303b416720a52f42d51af6248`; CodexDD 0.4.4 production `b25550dcb2c6e84a84074b603086c9db6f32a63b`; official pinned 0.163.0 `83086188ae0614bd882818ff1aa23f3194962012`.

The 17 staged exact upstream session files were either unchanged by CodexDD 0.4.4 (15 files), or newly introduced by upstream 0.163.0 (two async-compaction files). The source files include session configuration refresh, event handling, turn input, world state, MCP and relevant upstream tests; old/new Git blob IDs are pinned in `.github/scripts/test_codexdd_045_3c5b_session.py`.

Five existing production files required preserving CodexDD-specific changes while accepting upstream 0.163.0 changes. The original 0.4.4 fork-to-0.162.0 delta was inspected as nine small hunks total and applied at uniquely matching 0.163.0 contexts. Pinned resulting Git blobs:

| File under `codex-rs/core/src/session/` | Reconciled Git blob |
| --- | --- |
| `rollout_reconstruction.rs` | `118907bac6f1f084c96b81e78c2d1b7fc923583d` |
| `rollout_reconstruction_tests.rs` | `6286fb33b5d936da253abe9efdc090e494de0ac4` |
| `session.rs` | `68fb4db5fe8f11638a22c3b249d9f1bc7668cadd` |
| `tests.rs` | `dc7d8179b298113fedcbcb5d5dc0573737a5c2e1` |
| `turn.rs` | `50f524a055a92f0d19be5e6bc2665fe18718e35b` |

`RolloutItem::WorkflowState` remains handled in four reconstruction branches, one session-history case and two session Rust fixtures. The fork's existing native regression that normalizes an orphaned interrupted custom tool call with an `aborted` output remains present. `EventMsg::AdaptiveRuntimeSignal` remains explicitly handled in turn stream rendering. These are exhaustive-match compatibility and replay-safety source requirements, **not evidence that live runtime replay or native Rust tests pass**.

Production adaptive handler and the 3C.3 initial/post-hook restriction helpers remain unchanged; no event spoofing source or permissions are authorized. An adaptive signal must still be attributed to the correct trusted owning turn and terminalized only through the established trusted tool. This packet does not relax those boundaries.

## Validation and remaining gates

Predecessor **3C.4 static validation is 33/33 green** at detached Daniel-CL `cae522cf20d1893d7537e713f234cba098798a59` (2026-10-18). Packet 3C.5a configuration static tests: **15/15 passed in 1.724s**. Packet 3C.5b session static tests: **13/13 passed in 1.463s**. 3C.5c Guardian/approvals static tests subsequently **11/11 passed in 1.213s** at the same detached head, for a **39/39 static** 3C.5 checkpoint.

**Cargo.lock** remains the 0.4.4 graph. Native Rust session/turn/event tests, app-server replay behavior, core config/approval API compatibility, generated schema validation and Windows release suite remain outstanding. Other non-overlapping upstream changes and fork conflict sites must be reconciled before expensive testing. The product and upstream version markers remain 0.4.4/rust-v0.162.0.

**Next:** 3C.5c guardian/approval synchronization, followed by a *single* grouped cheap static 3C.5a/b/c checkpoint; no auto-PR, merge, release, artifact distribution, privilege broadening or publisher enablement.
