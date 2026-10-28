# CodexDD 0.4.5 — Packet 3F.35B combined Windows native regression

**NONRELEASE.** Supersedes 3F.33's PowerShell syntax failure before native test execution ([#40742651126](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40742651126)).

Preconditions:
- [3F.32 startup source proof #40742304689](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40742304689) GREEN — three official upstream WSL clipboard / reconnect cleanup / daemon-join restorations with adaptive startup retained.
- [3F.34 TUI test harness proof #40743349060](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40743349060) GREEN — five Daybreak/API-key feature combinations and expected shutdown WebSocket disconnect handling, without changing live runtime.
- [3F.35A source history inversion #40743590565](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40743590565) GREEN — 3F.20/25/32/34 historical change guards and 109-path owner source manifest run without deep historical Git objects.

Single 60-minute bounded Windows-2022 job compiles the current `codex-tui` test binary once under Rust 1.95.0 with pinned Rusty V8, Cargo --locked, unchanged crypto/deny/dependency security. Each of **eight** selectors runs serially from the same compiled binary:
1. `async_fresh_startup_applies_adaptive_preset_before_attachment`
2. `fresh_startup_thread_drains_buffered_approval_before_draft_handoff`
3. `reconnect_exhaustion_and_unknown_initial_thread_stay_offline`
4. `reconnect_allows_slow_hydration_but_bounds_a_stalled_server`
5. `daemon_startup_falls_back_only_for_implicit_endpoints`
6. `daybreak_command_persists_and_confirms_each_selection`
7. `dynamic_tool_requests_ignore_other_namespaces_and_dispatch_tui_namespace`
8. `older_external_server_starts_without_unsupported_dynamic_tools_or_history`

The last three selectors verify 3F.34's restored Daybreak matrix, previously successful six-case dynamic authorization matrix and an older external session with legacy feature/history negotiation and WebSocket recorder shutdown. A failure is not to be bypassed or interpreted as green. No 90-minute optimized thin-LTO CLI build, no actual release archive, no installer, no version upgrade and no 21-stage suite.

Passing this gate establishes only focused native tests on the source-frozen current candidate; it does **not** prove direct WSL clipboard UI prewarm, daemon restart join on process exit, 109-path semantic owner signoff or shipping readiness. Those are retained as native/manual/final-suite gates. PR #58 stays draft, production CodexDD 0.4.4 unchanged.
