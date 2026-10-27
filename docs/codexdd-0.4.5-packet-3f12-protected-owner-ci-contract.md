# CodexDD 0.4.5 — Packet 3F.12: native semantic owner evidence and CI contract repair

**Status: 40/40 EXACT-CRYPTO-GRAPH WINDOWS OWNER TEST FILTERS PASS. Two separate blocking-ci hygiene repairs staged for validation; full 109-owner review and release acceptance remain OPEN.**

## Exact Windows protected-owner semantic evidence

- [Nonrelease Windows core/TUI run #40583499451](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40583499451) **PASSED** at commit `dd74863e9c3d4efb8d08ddfe9d53f7ed7645f17c`.
- All **15/15** core authority, history, post-hook policy and cancellation filters ran with nonzero match counts. All **25/25** protected TUI adaptive, F2/elevated-startup, daemon, approvals, session, workspace-root and Escape filters ran likewise.
- Source checkout was pinned to exact Cargo.lock Git blob `13bc15ff844e93dfaf7777c73bb7313b35bb3b81`, reviewed Cargo.toml `cfebd9b784ef9ff027b274aa9f7101c9c2496613`, and nine original conflicting protected source identities (with the one reviewed formatter-only startup change). This is a **focused behavior** receipt, not 109/109 semantic signoff.
- The all-target `cargo-deny` and locked Windows crypto native gates separately passed earlier; no AWS live service sign-in or customer ciphertext recovery has been exercised.

## Unrelated mainline blocking-ci diagnostics

- [blocking-ci run #40583505375](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40583505375) failed on **(1)** `cargo fmt` in `codex-rs/voice-host/src/audio_track.rs`, a mechanical layout-only change introduced by RTC 0.21; and **(2)** `.github/scripts/verify_cargo_workspace_manifests.py` disallowing necessary `aws-config` `credentials-login` feature activation and treating the published vendored AWS SDK as a first-party workspace crate.
- **Security-preserving resolution:** restore exact Rustfmt layout in the one voice file without changing expressions or behavior. Keep the vetted AWS vendor files **unchanged** rather than removing optional crypto dependencies or credentials-login. Extend the workspace-manifest verifier only for the immutable, entire audited AWS vendor Git tree `5b1db93de9e14ab5c060794d23a567d907b66f21`, reject any worktree mutation, and admit only the exact first-party `aws-config` `credentials-login` feature. A dedicated five-case unit test asserts that the exceptions stay narrowly pinned and reject altered inputs.
- Existing `scripts/codexdd-045-inherited-risk-guard.py`, exact 74-file AWS inventory, advisory policy, protected Cargo.lock, secure P-256/RNG pins, and real cargo-deny are **not** modified or waived.
- This source change needs CI confirmation; it is not a claim that the broader `blocking-ci` was green.

## Next

Run cheap manifest and formatter checks before reattempting higher-cost CI; inspect any further first-failure log only. Then regenerate protocol/config/Python SDK on the current locked graph, review remaining core/app-server/rollout semantic owners, and validate native Bazel packaging before final frozen-source, detached 21-stage Windows validation. No production merge, marker promotion, installer or automatic release.

## Evidence closeout and remaining CI gate — October 27, 2026

- [Blocking-ci #40598236522](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40598236522) completed **PR-fast native adaptive, original cargo-deny, blob-size policy and codespell SUCCESS** after commit `0888904572991a5a7e231491f8f42b4a157f630a`. The **overall workflow remained RED** because its repo-format check reported 68 Python and four Bazel files, not because the AWS credentials-login feature was removed or crypto tests failed.
- [Format-only job #40603996779](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40603996779) later **PASSED** exact 72-file roster and AST-provenance conditions, writing formatted source at `e50aeb29ffc66490668edbc40921eab1ec3c5966` and retiring the write-capable workflow at `976ff54799b4d7271d34b0274c731bcb7e46eae1`.
- This formatter-only result did not run a full PR `blocking-ci` after the staged formatting. New-format candidate checks, stale historical 0.163 fixture reanchoring and Bazel native release-package acceptance are still independent gates.
