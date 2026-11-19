# CodexDD 0.4.6 — 3B.2f Core first-user authority intake
**Source parent:** 791628cab08d3e105f963a2311562d26cd551046 (3B.2e Windows run #42300237548 green).
**Status:** source checkpoint awaiting exact-head GitHub CI; not 3B.2 completed.

## Implementation
Core now prevalidates the first direct user-origin UserInput's leading text with the shared Worker parser. A malformed header is refused before accepting the submission; the first accepted header or plain message latches an immutable decision. Rejected/queued input never consumes authority, and later user/model continuations or synthetic turn triggers cannot rebind it. The same Core latch handles start-or-steer, start-if-idle and explicit steering. Launcher-config authority wins and prior user history locks the first-turn slot on cold resume.

## Release hold
The new Core latch is NOT yet copied into TurnContext.config or durably stored in a Core-authenticated thread settings snapshot. A first-message-only Worker still receives the existing 3B.2d SETUP_BLOCKED from the protected native runner. Cold resume without a trusted saved Core credential remains locked and nonprivileged, never replaying user prose into authority.

Next: trusted thread snapshot/persistence and candidate-SHA transition with a read-only exact-worktree verifier; independent Windows Node/Rust profile proof. No 0.4.5, release, install or merge changes.
