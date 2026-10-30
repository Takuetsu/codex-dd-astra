# CodexDD 0.4.5 — 3F.65 native restored-negatives acceptance (NONRELEASE)

This workflow is stacked on #74 and tests the exact reviewed Rust source at `d90a85121337861933d7d3cd0169ef0185aa46a9` with 15 real TUI native selectors, requiring a nonzero passed count and exit 0 per selector. The one-shot branch commit contains only test/workflow/docs files. Windows-2022 Rust 1.95, pinned secure Cargo.lock and V8, two Cargo jobs, and bounded scratch; preflight performs exact source checks and full Rustfmt without compiling. Windows CI uploads individual test logs and a fail-closed receipt even on failure.

Covered: startup modal input, live config reload, foreground terminal status, background request resolution versus foreground input, Windows paste variants, Daybreak flag-disabled behavior, API-key access-model filtering and CLI overrides, side-thread shutdown, stale buffered approvals, managed-daemon permissions, Escape and elevated Windows fallback.

A green narrow test **never grants** the 109 formal owner approvals or waives the 13 high-priority holds. Unbounded event queue/backpressure, offline active-view input, API-key model-popup routing and realtime-stop voice owner negatives remain outstanding. No version bump, 21-stage release suite, signing, publication, merge or install. Production remains CodexDD 0.4.4.
