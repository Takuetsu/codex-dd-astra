# CodexDD 0.4.6 — 3B.2i-e actual app-server process candidate / cold-resume acceptance

Baseline: GitHub Actions #42682942240 passed for exact source commit 25584b68d1b04a6c26f5acd4cfcebae499e71f14. Branch: dd/codexdd-v0.4.6-agent-reliability. This packet is a source checkpoint requiring new validation, not a release.

## Native process acceptance
Added a full app-server integration test under tests/suite/v2/codexdd_candidate_native.rs with a real subprocess and a mock model backend, confined to two temporary directories (Codex home and a throwaway Git checkout). The test makes three real commits in that disposable checkout. It starts a source-changing Implementation Worker via a direct first-user header, runs a second direct owner-origin request carrying exact prior/candidate SHAs, verifies the app-server emits a same-thread thread/settings/updated notification with the candidate SHA and unchanged role/scope, and inspects the native rollout's ThreadSettingsApplied record to confirm the checkpoint was actually persisted.

The test then gracefully shuts down that app-server process, starts a brand-new process against the same isolated Codex home and thread ID, resumes the thread, commits a third reviewed descendant and authorizes it with previous_sha equal to the *second* SHA. Success demonstrates that Core reloaded its successor credential from the prior durable thread record rather than the initial first-worker source. It checks the second persisted successor record on disk.

The same branch-scoped Windows workflow now compiles and executes this single integration test after existing Core/TUI regressions; job timeout is 120 minutes to allow linking the app-server integration harness. No production credentials, external model calls, release, installation or user worktrees are used. This is native-process proof if it goes green; a green unit test alone does not confer 3B.2 terminal release acceptance.

## Open after this packet
- Actual local embedded TUI + app-server subprocess reconciliation after a cold UI restart; compare persisted TUI workflow state to the Core-owned source snapshot
- Isolated Windows Node/Rust mechanical validation profiles and independent Validator evidence
- Denial scenarios for wrong-thread replay, invalid owner origin, candidate HEAD drift, changed scope, dirty source, and interrupted native flush at real process level
- Daniel-CL final local validation remains a separate explicit owner gate

0.4.5 remains frozen. No production merge or version bump.

## Native Windows acceptance recorded — 2026-11-10

**PASS — focused real-process acceptance.** Exact-source commit `7583e0012f16c6f06d2633908f3af7a594a71da5`; GitHub Actions [#42847772628](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42847772628), 33m12s; job `3B.2i focused Windows Git-boundary and app-server restart tests` passed. Logs confirm `codexdd_046_3b2n_real_app_server_candidate_and_cold_resume`: **1 executed, 1 passed, 0 failed**, not merely a successful build. It exercises native app-server candidate authorization, same-thread candidate notification and persisted record, a new server process, resume of the same thread, and a second successor SHA authorization in isolated temporary directories with mock model responses.

**Boundary:** This is not a live TUI restart, full independent-validator profile, 21-stage/final Windows distribution acceptance, or production/release approval. Temporary 3B2e diagnostic warnings are removed in the subsequent cleanup commit without changing native authorization behavior; that post-cleanup SHA requires its own source/format check.
