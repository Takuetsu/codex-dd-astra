# CodexDD 0.4.5 3F.63 — restore lost upstream negative tests (NONRELEASE)

Stacked only after the 3F.62 source-pinned trust patch. The 3F.62 Windows run remains frozen on its own commit and is not evidence for this further source change.

## Restored coverage from the exact official 0.163 source-review hunks
- `tui/src/app/tests.rs`: reinstate `background_resolution_keeps_matching_foreground_user_input_visible`. A background request resolution must not dismiss an unrelated foreground user-input decision.
- `tui/src/chatwidget/tests/slash_commands.rs`: restore Windows Ctrl+V / Ctrl+Shift+V and Shift+Insert image-paste text-fallback and buffered-repeat checks; restore the Daybreak API-key feature-disabled negative scenario.
- `tui/src/app_server_session.rs`: restore multi-case API-key cyber-access model catalog filtering across server feature flags, per-CLI overrides, bootstrap, and post-thread calls.

These are test restorations, not runtime fixes or evidence of native test success. If any restored negative fails, preserve the result and repair the relevant runtime behavior instead of weakening the test or waiving the upstream requirement.

## Risks still blocked
The original 13 formal high-priority path holds stay at 13; 109 owner source reviews remain unapproved. Outstanding work includes bounded FIFO live-event backpressure, offline active-view input protections, API-key model popup compatibility, realtime stop and side-thread shutdown, Daybreak settings, and Core model-mode tool authority. No Windows build, owner signoff, full release gate, publication, install, or 0.4.5 promotion is approved here.

## Gate
A cheap GitHub Actions source gate checks these restored test constructs, locked dependencies, 109/0/13 ledger, and Rust formatting. Native follow-up should run only after the 3F.62 15-case Windows suite is evaluated.
