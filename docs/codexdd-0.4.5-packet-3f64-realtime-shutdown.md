# CodexDD 0.4.5 — 3F.64 realtime-stop and side-thread shutdown restoration (NONRELEASE)

## Stack and release constraints
Stacked after #73 / 3F.63, which follows #72 / 3F.62. The frozen 3F.62 Windows 15-selector acceptance run does not cover 3F.63 or 3F.64 edits. Production remains 0.4.4; no 21-stage release suite, merge, signing, distribution, version promotion, installation, or formal owner acceptance.

## Behavioural gaps identified in 3F.60 upstream semantic receipts
- **Realtime stop transport recovery:** `AppEvent::CodexOp` can return early when a request error is classified as a recoverable transport disconnect. A failed `RealtimeConversationStop` previously reached this early return without recording a realtime failure on the thread's actual voice-owner widget. On the 3F.64 branch, move `record_realtime_failure(AppServerRequest)` ahead of that return, targeting the appropriate parked/visible widget, and remove duplicate accounting in the later nonrecoverable branch.
- **Side conversation shutdown:** `handle_exit_mode` previously shut down the current thread but dropped the original upstream bounded call to `shutdown_side_threads` before exiting. Restore the upstream two-second timeout for side-thread shutdown, preserving user exit responsiveness and logging timeout failures.

## Evidence gates
1. Source-only gate verifies event ordering, one occurrence of realtime-stop failure accounting in the modified dispatch branch, restored bounded side-thread shutdown, locked Cargo graph, version nonpromotion, 109 reviewed paths, 0 owner approvals and 13 outstanding holds.
2. Pin Rust 1.95 formatting to the precise source commit. Do not claim native validation from source inspection.
3. Later native tests must exercise recovered and unrecovered realtime-stop errors, parked-voice ownership, and bounded side-thread cleanup on Windows. These requirements are not waived by the 3F.61 or 3F.62 runs.

The 13 formal high-priority holds remain unapproved. This source-only packet is not a release gate.
