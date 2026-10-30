# CodexDD 0.4.5 3F.62 focused trust restoration (NONRELEASE)

Baseline: 3F.61 accepted 14 Core and 18 TUI native **test selectors**, not 109 owner approvals. This PR changes Rust source and must pass a new exact-SHA Windows run.

Restored pending native validation:
- app.rs — startup modal input quarantine, rendered selection before normal input, preserve pending modal state on terminal drain timeout
- thread_routing.rs — same fail-closed modal drain after thread attachment
- event_dispatch.rs — managed daemon verification before opening daemon/warnings menu
- config_persistence.rs — upstream live picker and resume-cwd preference reload assertions
- chatwidget/tests.rs — re-enabled two upstream terminal program status regressions
- app/input_ownership_tests.rs — new modal quarantine state-regression test

Still unwaived (13 formal path holds): Core client/thread manager/tools authority [4]; TUI app, event_dispatch, config persistence, app/tests, thread_routing, app-server session, settings, chatwidget/tests and slash_commands [9]. Specific missing negatives: bounded FIFO/backpressure under load, offline/disconnected active-view input, foreground approval vs background request, API-key model-popup / Daybreak feature filtering, Windows Shift+Insert and Ctrl+Shift+V paste, realtime stop and side shutdown, Core unauthorized lifecycle tool exposure. Earlier green tests are evidence, **not** owner signoff.

New workflow 3F.62: source-only + Rustfmt cheap preflight, then 15 actual Windows native TUI test selectors with nonzero test-count assertion and per-case logs. No 21-stage release suite, publication, merge, install, owner approval increments or version promotion. Production 0.4.4 remains unchanged.
