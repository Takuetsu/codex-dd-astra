# CodexDD 0.4.5 — 3C.9i native Windows TUI library compile checkpoint

**Status: ONE NATIVE LIBRARY-COMPILE CHECKPOINT PASS; TEST COMPILATION, SECURITY/BEHAVIOR VALIDATION, AND RELEASE ACCEPTANCE PENDING.**

Source target: official `rust-v0.163.0` integrated into the CodexDD 0.4.5 development branch. The shipped product remains CodexDD `0.4.4` on the tracked `rust-v0.162.0` upstream marker.

## Verifiable native checkpoint

- **Workflow:** [CodexDD 0.4.5 Windows compile checkpoint, run 39346683480](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39346683480).
- **Exact passing commit:** `e2383021ba6ef50624af77ec9d7b6f48d931df69` (2026-10-19 UTC/US Central).
- **Command:** `cargo check --keep-going -p codex-tui --lib` on hosted `windows-2022`, pinned Rust `1.95.0`, verified Windows Rusty V8 binding, and isolated runner scratch/target paths.
- **Result:** workflow `completed/success`, including the native Cargo check. Earlier compiler failures were not waived or masked.
- **Important limitation:** Cargo was run without `--locked`, because the development branch's checked-in `Cargo.lock` is still the 0.4.4 resolver graph. This is a _library_ compile, not a test-target compile, native test execution, a locked resolver guarantee, or a runnable installer acceptance.

## Corrections that enabled native library compilation

The 0.163 integration exposed multiple distinct interface mismatches before the green checkpoint:

1. Restore updated Guardian failure-feedback persistence: the 0.163 reviewer constructs `GuardianReviewRecord` and the parent state runtime retains it asynchronously, without broadening opt-in capture.
2. Repair the truncated TUI mention-binding filter and remove a separate duplicated 307-line source tail after the final `impl` brace.
3. Align upstream TUI shared contracts across event types, configured-pet generations, keybinding/selection rendering, user-verification errors, managed-daemon state and thread-list parameters while retaining CodexDD's adaptive Worker/session-specific event routing.
4. Retain buffered app-server approval reply handles through thread switches and restore typed WebRTC error propagation. Security-sensitive source was reconciled by interface rather than overwritten wholesale.

The passing commit establishes compiler compatibility of the selected library crate only. It does **not** attest all fork-specific security or authorization behavior.

## Follow-up validation tracks already isolated

- **TUI test-target compilation:** read-only [workflow 39348265982](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39348265982), initially triggered from commit `7be02ba8ad1e03f19f341ef935b4388fc314378c`. It runs `cargo check --keep-going -p codex-tui --tests` with complete compiler diagnostics; `--tests` does not execute the tests.
- **Python source provenance inventory:** read-only [workflow 39348466846](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39348466846), based on the original staged 0.4.5 source guards. Older packet SHA pins may need to be distinguished from _live_ merged-source invariants; do not fake green results by rewriting immutable historical sources. Historical 3C.9c review inventory is a provenance snapshot, not a substitute for native tests.
- **Lockfile:** independently reconcile the exact Rust resolver result with new `rmcp 3.5.1`/`linkify` and preserve or explicitly re-audit CodexDD security upgrades. In particular, the pinned branch had `rustls 0.23.46`, `quinn-proto 0.11.18`, and `noyalib-serde-yaml 0.0.57`. Official 0.163 `Cargo.lock` cannot be copied verbatim: its workspace packages are version `0.0.0` whereas this fork requires `0.163.0`, and its `rustls` is `0.23.45`. Obtain a resolver-produced, auditable fork lockfile before `--locked` gate.

## Gates still required before CodexDD 0.4.5 is complete

- Clean test-target compilation, followed by focused native TUI/Guardian/approval/adaptive Worker/Windows daemon/CLI and app-server tests.
- Reconcile or explicitly resolve all protected source overlap/conflict records (including the recorded 109-overlap and 15-direct-conflict audit), review the associated security invariants, and regenerate/version-check changed schemas.
- Freeze the exact accepted source and dependency lockfile; run the established SHA-bound Windows 21-stage release suite with the detached monitor and receipt, then CI/promotion review.
- Advance the release markers, merge and distribute a build only after explicit release acceptance. The 0.4.5 automatic upstream update mechanism remains disabled for autonomous publishing.

No local Daniel-CL or Stonks operator action is required to continue the GitHub source and focused test-compile checkpoints. Do not run the expensive release suite yet.
