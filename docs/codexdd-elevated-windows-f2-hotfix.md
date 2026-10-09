# CodexDD elevated-Windows F2 startup remediation

**State:** Local elevated SSH fix validated; non-elevated guard validated; Windows CI and live non-elevated shared-daemon startup pending

**Branch:** `dd/codexdd-f2-elevated-ssh-embedded-startup`

**Baseline:** CodexDD 0.4.2, production commit `8336ca046af71cbf032c76d933ca371336f92673`

## Observed issue

An elevated SSH PowerShell on Daniel-CL starts CodexDD 0.4.2 with:

> Running without the shared background server: an elevated Windows terminal requires embedded mode.

CodexDD already falls back to its embedded app server and continues working, but the normal expected fallback produces a persistent F2 startup warning. The operator proved the same terminal works without F2 when using `codexdd --no-daemon`.

This is _not_ authorization to start shared background services with an administrator token or bypass the daemon elevation guard.

## Root cause

In `tui/src/startup_orchestration.rs` the implicit daemon endpoint may be probed and automatic startup attempted before the managed daemon rejects the elevated Windows token. The resulting classified `ElevatedLaunchRestricted` failure then becomes a startup warning even though embedded operation succeeded.

## Bounded correction

- Extract a read-only current-process elevation query from the existing Windows daemon-launch guard. The guard still rejects elevated lifecycle operations and rejects malformed token-query responses.
- Before implicit shared-daemon discovery, automatically select embedded mode for an elevated Windows interactive session with no explicit remote endpoint or daemon-wide agents view. This follows the supported `--no-daemon` path without requiring the operator to add the flag.
- Do not probe, connect to, start, restart, or mutate the implicit shared daemon on this elevated embedded path.
- Treat only a successfully identified elevation fallback as an expected non-warning. If token elevation cannot be verified, fail closed into embedded mode without any implicit daemon discovery or startup, and show an F2 diagnostic. Preserve warnings for restrictive Windows launchers, explicit incompatible options, and daemon feature mismatch.
- Preserve explicit remote selection, daemon-only agents operations, automatic daemon startup in a non-elevated session, and the fail-closed lifecycle guard.
- Tag the selected runtime transport reason as `elevated_windows_embedded` in launch telemetry (or `windows_elevation_unverified_embedded` for a failed elevation probe).

No configuration migration, background-service privilege escalation, upstream Codex integration, or unrelated project changes.

## Windows validation gate

Cheap checks first on Daniel-CL:

1. Exact candidate HEAD + clean worktree + Git diff whitespace check.
2. `cargo fmt --all -- --check`.
3. Targeted Windows unit tests for elevation-probe/guard consistency, failed-probe fail-closed behavior, elevated embedded-policy, and unrelated-warning preservation.

Then run a focused Windows build and a live `ssh -tt` elevated-session smoke:

- `codexdd` **without** `--no-daemon` must launch embedded, with no elevated-session F2 warning.
- `/status` and normal adaptive behavior must remain available.
- Explicit daemon lifecycle operations under elevation must still reject the administrative token.
- A non-elevated interactive startup must remain eligible for normal shared-daemon operation.

## Local validation evidence (2026-10-08)

Candidate SHA: `2caa1b3afa55e23009ffc523fc23aa9abb3c3eba`, against production `8336ca046af71cbf032c76d933ca371336f92673`.

- Daniel-CL: candidate HEAD, clean worktree, formatting/whitespace checks, and debug `codex-cli` build passed. Candidate reports `codexdd 0.4.2+g2caa1b3afa55`.
- Targeted Windows TUI daemon-startup suite: **15 passed, 0 failed**, with `RUST_MIN_STACK=16777216` and `--test-threads=1`. A pre-existing embedded-server integration test overflowed its stack with the default Windows test-thread stack; the same test and full suite passed with 16 MiB. Release validation already sets `RUST_MIN_STACK=16777216`.
- Elevated interactive SSH session: launch the candidate directly with `--adaptive astra`, **without** `--no-daemon`; F2 showed **No warnings**. `/status` showed Adaptive Effort enabled, Astra preference, Luna Low startup, and Full Access.
- Elevated isolated-profile daemon lifecycle: **start, restart, and bootstrap all rejected** as intended. This confirms the elevation guard remained intact.
- Medium-integrity Windows scheduled-task regression: `whoami /groups` reported `S-1-16-8192`, and `elevated_token_probe_and_daemon_guard_agree` reported **1 passed, 0 failed**. The outer PowerShell scheduled-task wrapper remained running without writing its final status marker; the Rust test itself completed successfully.
- **Not yet tested:** an actual managed shared-daemon startup/attachment from a non-elevated session using a _packaged hotfix candidate_. The debug `codex.exe` is not a complete daemon-installable package. This remains a distinct **pre-merge acceptance gate**, not a claimed pass.
- **Not yet tested:** required GitHub Windows CI, final release validation, and production installation.

The candidate is not considered shipped until the installed build and normal release gates pass. The CodexDD product version has **not** been changed in this source-only repair packet.

## One-command live Medium-integrity smoke test

After assembling a complete packaged candidate on a Windows validation host, the
operator runs `scripts/codexdd-f2-daemon-smoke.ps1` from the F2 hotfix branch.
The script discovers the existing package from the
`codexdd-f2-last-smoke-root.txt` marker in the Windows user's temporary
directory (or accepts `-PackageRoot` to select an existing package root).

The script validates the candidate revision and package layout, creates a
uniquely named short-path `CODEX_HOME` with daemon auto-updates disabled, and
runs a temporary **Interactive / Limited** Scheduled Task. The worker must
prove Medium integrity (`S-1-16-8192`), then run the real packaged CLI
daemon lifecycle: `start` must return `started`, a second `start`
must return `alreadyRunning`, `version` must return `running` and
identify an app-server version, and `stop` followed by another `stop`
must return `stopped` / `notRunning`. Managed executable and socket
paths must remain under the temporary `CODEX_HOME`.

Temporary tasks are unregistered; on success the worker verifies daemon
shutdown and the controller deletes the temporary home. Command output,
task results, and a trace are retained in the package's `live-smoke-*`
directory for review. Failures are not treated as passes. The script does
not update the production CLI, install the hotfix, or alter daemon security
settings. A pre-existing interactive user login and UAC are required.

The final live-daemon smoke remains **pending** until the operator provides
a successful `F2 HOTFIX: FINAL ACCEPTANCE PASSED` result.

## Remaining expected limitation

Elevated Windows sessions still cannot use a _locally managed shared background daemon_. This fix makes the already-supported embedded mode automatic and removes a misleading warning; it does not implement safe de-elevation or provide daemon-only functionality under SSH administrator tokens.
