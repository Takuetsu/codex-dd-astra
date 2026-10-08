# CodexDD elevated-Windows F2 startup remediation

**State:** Code candidate — Windows validation pending

**Branch:** `dd/codexdd-f2-elevated-ssh-embedded-startup`

**Baseline:** CodexDD 0.4.2, production commit `8336ca046af71cbf032c76d933ca371336f92673`

## Observed issue

An elevated SSH PowerShell on Daniel-CL starts CodexDD 0.4.2 with:

> Running without the shared background server: an elevated Windows terminal requires embedded mode.

CodexDD already falls back to its embedded app server and continues working, but the normal expected fallback produces a persistent F2 startup warning. The operator proved the same terminal works without F2 when using `codexdd --no-daemon`.

This is *not* authorization to start shared background services with an administrator token or bypass the daemon elevation guard.

## Root cause

In `tui/src/startup_orchestration.rs` the implicit daemon endpoint may be probed and automatic startup attempted before the managed daemon rejects the elevated Windows token. The resulting classified `ElevatedLaunchRestricted` failure then becomes a startup warning even though embedded operation succeeded.

## Bounded correction

- Extract a read-only current-process elevation query from the existing Windows daemon-launch guard. The guard still rejects elevated lifecycle operations.
- Before implicit shared-daemon discovery, automatically select embedded mode for an elevated Windows interactive session with no explicit remote endpoint or daemon-wide agents view. This follows the supported `--no-daemon` path without requiring the operator to add the flag.
- Do not probe, connect to, start, restart, or mutate the implicit shared daemon on this elevated embedded path.
- Treat only the known elevation fallback as an expected non-warning. Preserve warnings for restrictive Windows launchers, explicit incompatible options, and daemon feature mismatch.
- Preserve explicit remote selection, daemon-only agents operations, automatic daemon startup in a non-elevated session, and the fail-closed lifecycle guard.
- Tag the selected runtime transport reason as `elevated_windows_embedded` in launch telemetry.

No configuration migration, background-service privilege escalation, upstream Codex integration, or unrelated project changes.

## Windows validation gate

Cheap checks first on Daniel-CL:

1. Exact candidate HEAD + clean worktree + Git diff whitespace check.
2. `cargo fmt --all -- --check`.
3. Targeted Windows unit tests for the elevation-probe/guard consistency and elevated embedded-policy/other-warning behavior.

Then run a focused Windows build and a live `ssh -tt` elevated-session smoke:

- `codexdd` **without** `--no-daemon` must launch embedded, with no elevated-session F2 warning.
- `/status` and normal adaptive behavior must remain available.
- Explicit daemon lifecycle operations under elevation must still reject the administrative token.
- A non-elevated interactive startup must remain eligible for normal shared-daemon operation.

The candidate is not considered shipped until the installed build and normal release gates pass. The CodexDD product version has **not** been changed in this source-only repair packet.

## Remaining expected limitation

Elevated Windows sessions still cannot use a *locally managed shared background daemon*. This fix makes the already-supported embedded mode automatic and removes a misleading warning; it does not implement safe de-elevation or provide daemon-only functionality under SSH administrator tokens.
