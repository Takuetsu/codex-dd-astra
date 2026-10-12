# CodexDD 0.4.4 — DD-BL-006 cross-repository LVO blocker

**Status:** ACTIVE — packet 1A source reconnaissance and initial root-resolution patch checkpointed; NOT validated, releasable, merged, or installed.

**Source of priority:** `codex-dd — Upcoming Version Backlog` (current post-0.4.3 queue dated 2026-10-11), DD-BL-006. The observed defect affected the Windows CodexDD 0.4.2 Implementation Worker in the TypeScript/Node BREAKWATER worktree `C:\bw55`. The 0.4.3 upstream refresh is now merged separately as PR #52.

## Baseline

- Production base: `dd/astra-policy-v2` @ `d8ad52cc223362a03f4dc5b40ec51695b3c48936` (merged 0.4.3).
- Active fix branch: `dd/codexdd-0.4.4-dd-bl-006`.
- Upstream remains `rust-v0.162.0`. The 0.4.4 product-version bump belongs to the eventual approved release candidate, not to reconnaissance.
- Platform: Windows; authoritative cheap local validation on Daniel-CL over SSH, then final GitHub CI after the local gate. No production installation/merge is implied.

## OBSERVED → REPRO → EXPECTED → OWNING PATH

1. **Observed:** protected Mechanical Validation demands `scripts/codexdd-test-workpacket.ps1` beneath a directory with `justfile` and `codex-rs/Cargo.toml`; a valid Node repository fails before native testing. Follow-on shell commands and retries cannot legitimately create a PASS receipt.
2. **Reproduction target:** the standalone `discover_repository_root` test on a mock Git worktree with `package.json`, and a nested Git repository beneath a parent with a different configured profile.
3. **Expected:** resolve the nearest active Git worktree and *only* its explicit repo-owned contract-v1 profile; if missing, give a clear nonrepairable configuration blocker, no cross-repo execution, no PASS/repair evidence and no model-family escalation. Preserve worktree edits, valid identity-bound receipts, trusted terminals, and mechanical no-write guard.
4. **Owning paths:** `codex-rs/core/src/tools/handlers/codexdd_validation.rs` and tests, mechanical command policy in `codex-rs/core/src/tools/registry.rs`, and TUI admission/continuation paths including `codex-rs/tui/src/chatwidget/adaptive_admission.rs`.

## Packet 1A — implemented, not yet locally validated

- Removed Rust-only root markers from the runner's profile resolution.
- Anchored profile discovery to the nearest `.git` directory or worktree-marker file; never fall back to a different repository's script.
- Kept profile names, script invocation, runtime log handling, strict schema and native SHA/run/evidence handling unchanged.
- Require the selected `scripts/codexdd-test-*.ps1` to be explicit and canonically contained in the active Git repository. A missing profile fails with remediation text rather than being treated as a product/test failure.
- Added regression fixtures for Rust, explicitly configured Node/TypeScript, missing Node profile in a nested repository, no Git root, and a Unix symlink escape.
- GitHub source checkpoints: `fe2a4876239d57a79d4ba3e2aa42c185f367a486`, `3f07a4806e3156dd0ff3968db2e91806e0f1c2e5`. **These have NOT run on Daniel-CL.**

## Follow-up packets (required before claiming DD-BL-006 fixed)

- **1B — mechanical safe-command policy:** diagnose and narrowly allow safe Node/TypeScript read-only inspection and approved project validation commands without permitting mutating arbitrary `npm run` scripts, bypassing the repo's heavy-validation semaphore, or allowing direct execution to impersonate a native LVO PASS.
- **1C — fail-closed workflow handoff:** ensure absent/unregistered profile reports one actionable configuration blocker instead of repeated automatic continuation or capability pressure; ensure source edits and uncommitted work are preserved, no synthetic receipt, no premature `ready_for_validation`.
- **1D — explicit Node repository contract:** define or integrate a fixed, reviewed `work_packet` (plus `targeted` for repairs) contract-v1 script for BREAKWATER or a generic supported registration mechanism. BREAKWATER's existing `package.json` validation scripts do **not** by themselves produce LVO contract-v1 receipts.
- **1E — test and release gates:** inexpensive source-local checks on Daniel-CL first; then Windows behavioral proof including positive Rust, configured Node, unconfigured Node, safe git inspection, staged/uncommitted worktree preservation, deterministic refusal with no escalation, receipt evidence; final CI only after local pass. Owner-governed merge and install remain separate.

**Current outcome:** packet 1A is a source checkpoint, not a working BREAKWATER unblock or release decision. An unconfigured BREAKWATER repo still lacks the explicit contract script; a missing profile must remain a blocker until packet 1D provides a valid contract.
