# CodexDD 0.4.6 — 3C.1b.2f: native Core two-Worker validation acceptance

**Date:** 2026-11-12. **Parent:** `ab06352b4a73b867f2e5f9b4266f20f1533db599` (3C.1b.2e accepted).
**State:** Focused native Windows Core two-Worker integration **ACCEPTED** at `bd81688edd5f214e97d97df1a52b1ca38ed462c9` on 2026-11-12 ([Actions #43276903773](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43276903773)). Full Core/TUI/app-server handoff and milestone 3C.1 acceptance remain **OPEN**.

## Purpose

Prior packets proved owner-issued gate construction, strict TUI receipt matching, and the existence/contract of the repository-owned independent review script. None invoked both real Core native handlers in distinct source-pinned Worker sessions against the same committed candidate. This packet tests that missing native process and provenance seam without installing CodexDD or touching BREAKWATER/CV.

## Isolated true native execution

The new `codexdd_046_3c1b2f_real_two_worker_core_receipts_and_refusals` native Windows test:
1. Creates a disposable Git candidate with *only* two tiny test-specific repository-owned PowerShell scripts. Both scripts emit the actual contract-v1 validation events and return actual native process exit codes. They are committed before an exact 40-character SHA is bound. The scripts never modify their candidate.
2. Creates a Core Implementation session with explicit root/SHA/scope and calls the real `CodexDDValidationHandler` (`work_packet`). Verifies process success, source SHA, native stage, and a genuine completed `ItemCompleted` Core tool item.
3. Creates a separately bound Core Validation session for the *same* authorized source and calls the real `CodexDDIndependentValidationHandler` (`independent_review`). Verifies native PASS/exit 0, exact root/SHA, unique run ID, distinct thread identity, independent tool name and correctly bound ItemCompleted arguments.
4. Directly invokes the Implementation-only LVO from the Validation session and verifies a typed non-PASS source identity blocker (no role promotion). After a synthetic **committed** candidate update, reruns independent validation under the *stale* source pin and verifies a typed refusal, while preserving a clean disposable checkout.

The test config explicitly disables interactive permission prompts **only for its isolated disposable Core sessions** so the native test can execute unattended on Windows CI. Production and release policy remain unchanged; this test does not authorize unattended scripts in game projects.

The [3C.1b.2f-native] CI job uses exact SHA/branch source preflight, pinned Windows Rust, DotSlash and V8, requires exactly one new Core integration test, and rechecks the three 3C.1b.2b independent Core role/source unit tests. No release artifact is uploaded.

## Acceptance limits

**Not yet proven by this test:** the actual full six-stage repository-owned independent script; owner first-message binding routed across the live app-server; TUI fresh Validation worker launch and receipt observation from an actual app-server event stream; trusted READY signal admission across the two-process TUI/Core seam; strict deadline/restart/cold resume in a real operator session; operations listener+HTTP health. Treat 3C.1 and release 0.4.6 as **OPEN** regardless of a passing Core test. No BREAKWATER coding until full development/release acceptance, then owner-approved install and the approximately one-week one-milestone soak.

**Next:** after the native Core acceptance run passes, add a disposable app-server/TUI two-thread identity and event pipeline end-to-end test; then expand to real full-script and terminal/READY-gate integration with negative role/source/port evidence cases.

## Acceptance checkpoint — November 12, 2026

[GitHub Actions #43276903773](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43276903773): **SUCCESS**, exact source `bd81688edd5f214e97d97df1a52b1ca38ed462c9`, total 23m01s. Source preflight, Rustfmt, one real two-session Core fixture, and the existing three independent Core provenance/refusal tests passed. Both the native PASS and fail-closed negative paths were executed in isolated Windows CI. The 3C.1b.2f fixture used two separately Core-bound sessions, **not** two app-server/TUI-managed sessions; it does not establish a live app-server notification pipeline or authorize READY.
