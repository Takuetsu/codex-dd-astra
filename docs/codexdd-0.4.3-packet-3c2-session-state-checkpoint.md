# CodexDD 0.4.3 — Packet 3C.2: TUI session/runtime source reconciliation

**Status: ALL 19 OWNED SOURCE OVERLAPS RECONCILED; TUI COMPILE AND WINDOWS-NATIVE ACCEPTANCE PENDING.** This packet is a source checkpoint, not a claim that the executable or tests pass.

## Source identity

- Integration branch: `dd/codexdd-v0.4.3-upstream-0.162.0-integration`
- Production base: `4828e3b4232781963594cfaaebdc49c5531de8b1` (unchanged)
- Upstream target: `rust-v0.162.0`, peeled `c1382380de69521303b416720a52f42d51af6248`

## Prior source commits retained (do not repeat)

- `e3bc78e08a`: event contracts, app-server event targets, config persistence.
- `49cbc44469`: managed worktree creation.
- `aacf84a3ef`: session lifecycle and Worker binding.
- `920ac293e4`: initial startup.
- `9fd17f83c0`: event delivery and background/thread event routing.
- `e5865f2772`: trusted trigger and fork thread routing.
- `d666e0793d`: thread session restoration.
- `41f25486fa`: app event dispatch.

## Remaining 3C.2 sources reconciled in this continuation

| File | Source checkpoint | Preserved behavior |
| --- | --- | --- |
| `tui/src/session_state.rs` | `1f760efc` | Upstream Daybreak session flag alongside thread-owned adaptive controller and persisted workflow |
| `tui/src/app_server_session/reasoning_defaults_tests.rs` | `7ab11d28` | Upstream embedded/remote summaries and server-authority fixtures plus CodexDD no-summary-default regressions; signature adjusted for trusted turn trigger and personality compatibility |
| `tui/src/app_server_session/rollout_history.rs` | `b265d522` | New upstream resume permission override and tool validation paths plus startup-only Worker binding and persisted state restoration |
| `tui/src/session_start.rs` | `ddc0e7b6` | Archived resume/retry retains upstream permissions and startup-only Worker binding |
| `tui/src/app/startup.rs` | `3c365b06` | Initial resume uses trusted configuration-bound Worker path; interactive resume remains unbound |
| `tui/src/app_server_session.rs` | `7c86f9f5` | Entire upstream TUI session interface with trusted turn triggers, guarded workflow-state mutation, fresh/existing adaptive restoration, Worker startup context, detached-fork regression, Cyber Access and Daybreak fields |
| `tui/src/app/working_directory.rs` | `cb9fba99` | New upstream trust/config/permission selection behavior plus bound Worker in managed worktree replacement |
| `tui/src/lib.rs` | `56cb2ff6` | Full upstream TUI imports and startup orchestration, CodexDD adaptive modules and CLI family selection |

The `app/startup.rs` source had already been upstream-reconciled; the later commit only corrects the Worker-binding path that matches the newly reconciled resume interface.

## Security and validation boundaries

- Upstream local/remote resume permission overrides and server-owned security semantics must stay intact.
- Startup-only Worker binding does not authorize ordinary interactive resume.
- Trusted app-server signal transport is independent of user-provided final text or direct UI mutations.
- 3B.5's focused Linux smoke passed at `c428e11c750a5857fbe4c6dcf008451ae3402c08`, but predates these changes and **is not TUI validation**.
- Complete 3C.3 TUI source reconciliation, then run a narrow `cargo check -p codex-tui` on the integration branch and inspect all errors. Format and targeted regression tests follow.
- 3C.4 Windows daemon/F2 and interactive TUI validations remain separate; do not alter production, product/upstream versions, Cargo.lock or generated schemas.

**No Daniel-CL action required yet.**
