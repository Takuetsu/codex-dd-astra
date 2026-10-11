# CodexDD 0.4.3 — post-rustfmt source-hash delta and native cheap-check evidence

**Status: READ-ONLY EVIDENCE INDEX — SOURCE SEMANTICS NOT OWNER ACCEPTED.** This note tracks the changed Rust blobs after the isolated formatting-only source patch without rewriting or reclassifying the historical 136-row review ledger.

## Exact immutable source identity

- Last pre-format runnable source commit: `544a29516353cb1e416b6097c46d8c83b23c7eb9`; prior historical ledger was generated for `2e70b04e1c80b5d9d2ef9ae67a3c9a7e902e63ff` and its tree `9676469c99221ab0be361c06048ed6f2df1b8bd4`. The original ledger remains a historical record, not the current source blob table.
- Post-format validated Rust source commit: `32ab09a141d626df5851965a557e0dce877ac2ec`; tree: `a0bce7166cccb52e9b4fc8d658e0c08616752e62`. The formatting commit was generated from the pinned Rust 1.95.0 formatter and published to the isolated integration branch only.
- One-shot read-only formatter source: [GitHub Actions #38162415522](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38162415522) at `7a7c806416bdea5a56357fc17f5eab8232d67535`; the formatter executed in a disposable Linux checkout with read-only credentials, and its exact patch was applied after validating each hunk against the original blob.
- [Formatting commit `32ab09a141d626df5851965a557e0dce877ac2ec`](https://github.com/Takuetsu/codex-dd-astra/commit/32ab09a141d626df5851965a557e0dce877ac2ec) modifies **15 tracked Rust paths, 23 Rustfmt diff hunks**, and deletes the one-shot workflow. GitHub confirms no other source paths changed in that commit. All 15 are in the original 136-row overlap ledger; **4** are among the original 22 textual conflict paths. These facts are mechanical-format provenance, not a fresh owner semantic attestation.
- Original post-PR-50 production HEAD remains `16fdc9826e800ebabab920eadd8e1d976d2f645f`. No canonical candidate, signed v2 manifest, release-profile receipt, approval, PR, merge or installation was created by this source formatting.

## 15 changed source blobs — delta over historical ledger

The original ledger's `git_blob_ids.reconciled_source` values identify the **pre-format** source. Where necessary, substitute the exact post-format blob SHA below for semantic review of the frozen source. **Do not** blindly replay an old ledger as if its 136 hashes all describe the new tree; unchanged entries require final Git-object re-verification when a candidate is frozen.

| Changed overlap | Owner | Kind | Historical source blob | Post-format source blob |
| --- | --- | --- | --- | --- |
| `codex-rs/config/src/config_toml.rs` | `3B.1` | textual_conflict | `373fc325ea968f735777dbcae919aa5a9b2af88b` | `72f0ad4756d6f4a523bd2ed886be3757343677eb` |
| `codex-rs/core/src/client.rs` | `3B.2` | semantic_overlap | `964cedfe7950b5247cd86f20a89fea60da82e766` | `a08e2386475b0c4e87d0d79aaccc27efeaff641d` |
| `codex-rs/core/tests/suite/agent_websocket.rs` | `3B.5` | semantic_overlap | `079983f449f017fb3b6247bf4b89e3a7e7b1ffcc` | `16b42c6a4709adb420b49c79e529266c8cdcb764` |
| `codex-rs/rollout/src/lib.rs` | `3B.1` | semantic_overlap | `f9f1d7af740a7a3b57059f8278fea5bef77e00ef` | `e6ac1f613b7b26184373a92bc75ef6b7498b7914` |
| `codex-rs/rollout/src/policy.rs` | `3B.1` | textual_conflict | `a5d9581b7f3aa2e7716426f62b0a761692a4698b` | `84818a2e6dbd70ac46f2a96b34488b5e1a1a1155` |
| `codex-rs/tui/src/app/app_server_event_targets.rs` | `3C.2` | semantic_overlap | `d4b51fd6c0c56975ffb20a959bdd32ac4a74f192` | `521afcee6d57e0adfdb6ffdf89aad77f6bc7c11e` |
| `codex-rs/tui/src/app/tests.rs` | `3C.4` | semantic_overlap | `894a67199c0015527111a6e8f3e5bb2d76635e86` | `1a1d42c2cb3586e19aa22ec71f8d264d97ef2909` |
| `codex-rs/tui/src/app/tests/disconnect_tests.rs` | `3C.4` | semantic_overlap | `7a156f55f1cdbb1aff2bd4f9e67d41c65146f398` | `e8506446b855b07a5ae44453c0a730bec627fc5d` |
| `codex-rs/tui/src/app/tests/session_lifecycle_requests.rs` | `3C.4` | semantic_overlap | `20b61c3f238c5240d8401d7eafa93fdc291ff0f1` | `03f32cbd9806398ce05c1e54d08bf8931434a68c` |
| `codex-rs/tui/src/app/tests/turn_submission.rs` | `3C.4` | semantic_overlap | `18f21dee50e072a9fd446113d49324d1f127d23d` | `2a094004581d928204bda120ec5c1cb3d9d5df09` |
| `codex-rs/tui/src/app_server_session.rs` | `3C.2` | textual_conflict | `0fc807cf3329083fa1fcd55ec56fdb98802d0d7b` | `c95f1d080836e934d83296ebab20b01abf1af5be` |
| `codex-rs/tui/src/app_server_session/reasoning_defaults_tests.rs` | `3C.2` | textual_conflict | `8ac2804f130463a4b1d405dbc076e068210a79ed` | `22e9183f76ed8ea5545836f207b0496315f60509` |
| `codex-rs/tui/src/chatwidget/replay.rs` | `3C.3` | semantic_overlap | `2c58e0a2d69ad59147f5136593d960117e4ee3b6` | `8b079cdb3863f0d613ec1f9eb702c142c6445133` |
| `codex-rs/tui/src/chatwidget/tests/composer_submission.rs` | `3C.4` | semantic_overlap | `96e372738867ed81d65d403e747a05f756899cf7` | `80a1b2d3022ea6d989b1b25122dfb786106b3c2f` |
| `codex-rs/tui/src/chatwidget/tests/status_and_layout.rs` | `3C.4` | semantic_overlap | `1053429a21da072a25bc57b7cbc4cbfeba8b6a13` | `0d1f1f28923ca09382c0cb2afc9bcf3b2248dbe9` |

## Daniel-CL native validation receipts — cheap pre-CI tests (2026-10-11)

The operator used a separate `E:\codexdd-043-validation` detached checkout, leaving `E:\codexdd` and the installed production version unchanged.

- Windows LVO validation contract fixture: **PASS**, emitted `CODEXDD_VALIDATION_CONTRACT_TEST PASS`; exercised 0 / 1 / 2 process-outcome classification. This is a process fixture, **not** a full source test or candidate-bound release receipt.
- After the formatter patch was checked out at `32ab09a141d626df5851965a557e0dce877ac2ec`, `cargo fmt --all -- --check` returned **FORMAT_EXIT=0** and **FORMAT_HUNKS=0** on Daniel-CL. No source files were changed by the check.
- Windows v2 canonical release-receipt fixture: **PASS**, emitted `CODEXDD_V2_RECEIPT_POWERSHELL_FIXTURE PASS`. This verifies fixture generation, canonical bytes and expected rejection cases; **not** candidate publication, a 21-stage release execution, or runtime F2 behavior.
- Operator-provided console evidence supports these observations but is not a cryptographically bound native 21-stage candidate receipt. The detached local checkout has an unrelated untracked `M` file observed during diagnostics; do not treat this worktree as a clean canonical release candidate without a fresh clean-worktree gate.

## Remaining gates and test placement

- The preceding [628 targeted runtime regressions](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38155292769), [Windows native daemon/TUI compile + unit regressions](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38158359012), [13 generated output drift gate](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38157430174), and [136-path original source-object SHA audit](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38157286824) passed on previous pinned tree snapshots. Formatting-only changes do not themselves constitute failed runtime tests, but **these are not fresh final-CI runs at the post-format HEAD**.
- Any later source revision requires refreshing relevant blob identities and targeted validation before promotion. The source-review owner must explicitly accept the proposed 22 conflict strategies, 136 overlap dispositions and six security domains, keeping native runtime behavioral checks separate.
- Next testing order: **cheap Daniel-CL relevant source checks**, then final CI only on an accepted/frozen source snapshot, followed by candidate-bound Windows native release-profile and protected promotion gates. Do not authorize or launch GitHub exploratory CI to compensate for missing local evidence.
