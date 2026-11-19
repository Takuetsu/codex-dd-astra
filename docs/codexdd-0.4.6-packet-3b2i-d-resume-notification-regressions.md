# CodexDD 0.4.6 - Packet 3B.2i-d, notification and successor restore fixtures

Predecessor green: commit 6d81ff68015634bed34cf7b44893d2c340263ac2, GitHub Actions #42669917310. Development branch: dd/codexdd-v0.4.6-agent-reliability.
State: source-matched regression checkpoint requiring fresh Windows CI, not real-daemon acceptance.

## Changed verification
- One native Core snapshot regression verifies a clean Git descendant promoted by an explicit candidate authorization, roundtrips the resulting exact successor SHA through typed JSON, and restores original Worker role/scope/root and successor SHA after simulated cold restart. A fresh fork without an owned snapshot does not receive the source pin.
- One TUI integration-level regression sends ServerNotification::ThreadSettingsUpdated via the ChatWidget notification handler using a temporary Git repository and three real commits. It checks foreign-thread refusal, same-thread descendant reconciliation, scope mismatch refusal, dirty checkout refusal and preservation of untracked source edits.
- Branch-scoped Windows workflow runs the exact Core and TUI tests in addition to all previous 3B.2i checks.

## Limits and remaining gate
These tests do not exercise native rollout append/flush, a real embedded app-server process or daemon restart. The Core cold-resume unit test explicitly simulates a completed flush; production persistence must still be verified with real process restarts. The TUI notification fixture is not proof that app-server emits it on a real transition.
Next packet: perform actual embedded app-server notification roundtrip and same-thread process cold resume; verify missing/delayed notification fails closed, real isolated Node/Rust profile acceptance and independent validator. No release, merge, install, production change, 0.4.5 thaw or game worktree edit.
