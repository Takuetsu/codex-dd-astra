# CodexDD 0.4.4 — DD-BL-006 cross-repository LVO blocker

**Status:** ACTIVE — packets 1A–1D source changes checkpointed on isolated branches; Windows tests and integration proof PENDING. Not releasable, merged, or installed.

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

- **1B — mechanical safe-command policy (IMPLEMENTED; UNVALIDATED):** diagnose and narrowly allow safe Node/TypeScript read-only inspection and approved project validation commands without permitting mutating arbitrary `npm run` scripts, bypassing the repo's heavy-validation semaphore, or allowing direct execution to impersonate a native LVO PASS.
- **1C — fail-closed workflow handoff (IMPLEMENTED; UNVALIDATED):** ensure absent/unregistered profile reports one actionable configuration blocker instead of repeated automatic continuation or capability pressure; ensure source edits and uncommitted work are preserved, no synthetic receipt, no premature `ready_for_validation`.
- **1D — explicit Node repository contract (IMPLEMENTED ON ISOLATED BREAKWATER TOOLING BRANCH; UNVALIDATED):** define or integrate a fixed, reviewed `work_packet` (plus `targeted` for repairs) contract-v1 script for BREAKWATER or a generic supported registration mechanism. BREAKWATER's existing `package.json` validation scripts do **not** by themselves produce LVO contract-v1 receipts.
- **1E — test and release gates:** inexpensive source-local checks on Daniel-CL first; then Windows behavioral proof including positive Rust, configured Node, unconfigured Node, safe git inspection, staged/uncommitted worktree preservation, deterministic refusal with no escalation, receipt evidence; final CI only after local pass. Owner-governed merge and install remain separate.

**Current outcome:** Source changes exist on two isolated feature/tooling branches. The live BREAKWATER worker remains on its original branch and therefore does not yet have the opt-in scripts; no unblock, source merge, receipt PASS or release decision is claimed.

## Source checkpoint — October 12, 2026 (validation not run)

CodexDD integration: `dd/codexdd-0.4.4-dd-bl-006` (product `0.4.4`, upstream retained at `rust-v0.162.0`). Changes:
- Runner now discovers the nearest active Git worktree (`.git` file or directory), rejects path escapes and missing profiles, and emits a structured **non-PASS** result for a missing/invalid repo-owned profile or missing execution log. It preserves native SHA/run-bound evidence, log capture and repair fingerprints only for real test failures.
- Mechanical command admission permits bounded, named BREAKWATER-style npm diagnostics (typecheck/lint/format checks/build/test/focused/maps) plus read-only Git inspection, with arbitrary scripts/flags/unguarded raw commands rejected. Shell diagnostics **do not** count as a native LVO PASS.
- The current-turn, built-in failed `run_codexdd_validation` event without a repairable stage fingerprint deterministically blocks the Implementation Worker's Mechanical Validation phase, suppressing unfinished-turn retries, capability pressure and false `ready_for_validation` terminalization. Actual failed test stages retain their bounded repair path.
- Regression tests added for Node/Rust resolution and repository isolation, safe/unsafe npm commands, deterministic BLOCKED handling, capability-pressure override, and same-turn repairable/other-turn isolation.
- Candidate product identity is `0.4.4`; no CI, binary or installation evidence yet.

BREAKWATER opt-in on **separate untouched-gameplay branch**:
- Repository: `Takuetsu/breakwater`
- Branch: `codex/tooling/codexdd-044-node-lvo-contract`
- Original owner-accepted evidence-tooling base: `fc20cb9c0576668b2853a09032b3d5086e5b423d`
- Native profile entrypoints: `scripts/codexdd-test-targeted.ps1`, `scripts/codexdd-test-workpacket.ps1`, `scripts/codexdd-test-release.ps1`.
- Contract-v1 event emitter: `scripts/codexdd-validation-common.ps1`. Fixed stage names/commands, no external model text; `git diff --check` and staged diff check remain read-only; guarded npm commands retain BREAKWATER's existing focused/heavy semaphore.
- `targeted`: diff/staged-diff/focused. `work_packet`: diff/staged-diff/changed-format/focused/lint/Vitest/build. `release` (explicit only): diff/staged-diff/official full-regression.
- This tooling-only branch is NOT merged into Z-A0.55E, canonical main or any active worker checkout; it does not change gameplay code or waive the independent milestone gate.

### Next required validation gate (do not dispatch broad GitHub CI first)

1. On Daniel-CL, fetch **only** the CodexDD feature SHA into a separate worktree, verify identity, run source formatting and narrow `codex-core` and `codex-tui` regression filters first. Use existing warmed Rust target if safe. No 21-stage release profile during ordinary packet validation.
2. On Windows, parse all four BREAKWATER PowerShell files with the built-in parser (syntax-only); check profile failure/contract-only cases without running heavy npm. Do not alter active Z-A0.55E lineage or its dirty worktree.
3. Only after narrow source checks pass, exercise the configured Node contract in a disposable BREAKWATER checkout; verify PASS, product FAIL, profile-error BLOCKED, no fake native PASS, no cross-repo fallback and preserved staged/uncommitted files.
4. Reinspect source and semantic diff, then run final targeted GitHub CI, owner-governed squash merge and separate install/soak gates. No owner's release decision has been inferred from branch creation.
