# CodexDD 0.4.5 — Packet 3F.30: two exact official 0.163 TUI fixture additions

**NONRELEASE SOURCE IDENTITY CHECK ONLY.** Two of the 15 original direct-conflict files retain every byte of official upstream 0.163 except one explicit CodexDD argument, as established by a reversible Git blob reconstruction. No Rust/lock/security implementation changes are necessary.

- `codex-rs/tui/src/app/tests/startup_defaults_tests.rs` — CodexDD blob `e60808df0448682d47af1a14b5bd8d5c52206397`, one additional `/*adaptive_family*/ None,` argument in the TUI startup default fixture. Removing exactly that line yields official 0.163 blob `86e0da6fd4e0550050cc1a704186a9a07038278b`.
- `codex-rs/tui/src/app_server_session/workspace_roots_tests.rs` — CodexDD blob `d6447d22a0a1e943b1352b8c945cf7f6cf09734e`, one additional `/*turn_trigger*/ None,` argument in the session request fixture. Removing exactly that line yields official 0.163 blob `525eed5ece8b9c0110b3b296b68fb6c727d6d096`.

The Python source-only guard `.github/scripts/test_codexdd_045_3f30_tui_fixture_additive.py` fetches the committed Git blobs (not Windows-transformed worktree bytes), proves exact two-way inverse reconstruction to the official 0.163 blob SHA, verifies the immutable 15-conflict source receipt and ensures production 0.4.4 / upstream 0.162 markers remain unchanged. The one-shot CI job does not compile Rust or promote the source to release.

**Remaining native acceptance:** targeted startup-default, managed-daemon/adaptive family, workspace-root and session-authority tests on the final source-frozen Windows candidate, plus 109-file review signoff and full 21-stage release testing. Source identity alone is not proof of runtime behavioral correctness. Both rows continue `release_status: NOT_ACCEPTED`, and PR #58 stays draft.

## Verified source-only CI evidence

[3F.30 additive-fixture guard #40739660499](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40739660499) completed **SUCCESS** on the immutable source reviewed in commit `6b22fc5`. Removing each single documented argument reproduced its exact official 0.163 Git blob. This is source provenance, **not runtime or full TUI acceptance**; retain **NOT ACCEPTED** until targeted native Windows and final source-frozen release gates are green.
