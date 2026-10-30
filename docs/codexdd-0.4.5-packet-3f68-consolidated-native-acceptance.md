# CodexDD 0.4.5 — 3F.68 consolidated native acceptance (NONRELEASE)

**Exact reviewed runtime source:** `dcc428e7078a51f4663a6aa1fe5099891def755e` at draft PR #77, stacked on #72–#76. This commit adds only test workflow, selector manifest, runner and this document. Production stays CodexDD 0.4.4 with pinned secure Cargo.lock.

A cheap Linux preflight first checks 25 unique declared Rust test selectors, all fixed-source startup, daemon, Daybreak, offline-view and API-key popup safety invariants, untouched 109-path / 0 owner approvals / 13 high-risk holds, and complete Rust 1.95 formatting. Only after green, Windows-2022 compiles with pinned Rusty V8 and two Cargo jobs and runs each selector separately, requiring nonzero real tests, 0 failures and an always-uploaded JSON receipt and per-case logs.

The suite combines the 15-selector 3F.62 and 3F.65 suites, adds both offline-view negatives and the new API-key raw model/account-switch boundary. Original 3F.62 Windows evidence passed 9 tests and exposed stale daemon menu snapshots; original 3F.65 evidence passed 6 and exposed the stale enabled-state API-key Daybreak help snapshot. Both were corrected from observed native output without waiving negative behaviors. They remain historical red receipts.

**Not release authority:** unbounded live-channel backpressure, Core lifecycle tool boundaries and 13 formal high-priority paths still require native/semantic owner approval. No 21-stage Windows optimized release suite, seven-program distribution, merge, version promotion, signing, publication or installation is authorized.

## 3F.68 native failure triage (October 30)

The third Windows attempt compiled and passed selectors 1–9, then stopped at selector 10 because the daemon confirmation snapshot recorded a development CLI version 0.0.0 while the Windows build correctly rendered 0.163.0. The test now verifies the actual install-context version, then normalizes only that dynamic field for the unchanged snapshot. The Windows runner now compiles the test binary once, executes all 25 selectors even if a prior selector fails, and emits an aggregate failing receipt rather than stopping after one snapshot. The one-shot native source pin is `e7fd2b0f007add318a0d6023205ac88930fe7d8f`. These changes do not waive native behavior or any owner review holds.

## 3F.68 Rustfmt gate correction

[Run #41116734652](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41116734652) passed 25-selector integrity but failed at a single Rustfmt change in the daemon CLI dynamic version normalization. Windows compilation was skipped. This candidate applies the exact formatter diff, pins reviewed runtime to `961abe24f4081fe31d5d39a8c819ca5e15db465c`, and preserves the complete 25-selector native failure-aggregation contract.
