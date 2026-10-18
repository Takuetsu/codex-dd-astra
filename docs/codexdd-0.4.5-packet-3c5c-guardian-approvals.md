# CodexDD 0.4.5 — Packet 3C.5c: upstream Guardian and approval runtime staging

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Sixteen pinned 0.163.0 Guardian source/test changes plus the upstream `tools/approvals.rs` change were staged; eleven offline source checks added and **not yet run**. No native Rust authorization or Windows runtime test receipt exists for this packet.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Provenance and boundaries

All 17 staged files were byte-identical between official rust-v0.162.0 (`c1382380de69521303b416720a52f42d51af6248`) and the CodexDD 0.4.4 production baseline (`b25550dcb2c6e84a84074b603086c9db6f32a63b`). The integration stages the exact official rust-v0.163.0 blobs from commit `83086188ae0614bd882818ff1aa23f3194962012`; full old/new Git identities are fixed in `.github/scripts/test_codexdd_045_3c5c_guardian.py`.

The upstream approval-action routing removes its historical Unix-specific `Execve` action branch, retaining `ApplyPatch` and standard review/approval decisions. The 16 Guardian files update request identity, feedback, decision/coverage handling, review sessions, the session runtime and their native tests as one source-owned group. Windows-only CodexDD policy remains defined separately in the adaptive registry and native-validation handler. This source staging does not authorize blanket approval, bypass user decisions, suppress sandbox restrictions, broaden privileged SSH behavior or grant runtime elevated execution.

Existing `guardian/permissions.rs`, `guardian/request_budget.rs`, and `guardian/reviewer_config.rs` are byte-identical to official 0.163.0 and remain unchanged. CodexDD's trusted adaptive terminal-signal restriction, 3C.3 pre/post-hook checks, original production version markers and the disabled upstream auto-publisher remain unchanged.

## Validation and open gates

**3C.4 static source tests: 33/33 passed** on Daniel-CL at exact detached commit `cae522cf20d1893d7537e713f234cba098798a59`, with screenshot supplied 2026-10-18. The new 3C.5a/b/c checks total **39 cases (15+13+11)** and have **not run yet**. GitHub source checks cannot demonstrate native Rust compilation, correct approval prompts, permission retention, post-hook authorization or end-to-end turn replay.

**Cargo.lock** remains the baseline 0.4.4 graph. Many cross-crate model and core/tool caller changes are still staged/incomplete; additional fork overlap and app-server consumers, native Rust security tests, Windows 21-stage release validation, GitHub CI, generated schemas and final provenance receipt remain mandatory.

**Next:** group cheap 3C.5a/b/c static validation on one exact detached integration head, then continue the remaining source graph and eventual focused native Rust tests. Do not bump versions, merge, open an automated PR or distribute release artifacts.
