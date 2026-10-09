# CodexDD elevated-Windows F2 startup remediation

**State:** MERGED, INSTALLED AND VERIFIED on Daniel-CL (2026-10-09). Elevated SSH F2: **No warnings**. Separate live non-elevated daemon lifecycle remains **waived/unverified** for PR #48 only.

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
- **Non-elevated daemon attempt:** A complete disposable test package was assembled; Medium token, protected private ACL, and matching snapshot metadata passed. The scheduled-task launcher prevented process breakaway (`Access is denied`, OS error 5). The ordinary desktop lifecycle remains **unverified**, explicitly waived by the operator for this F2 hotfix only.
- **Later completion:** Required GitHub CI passed for PR #48; the merged release binary was built, installed on Daniel-CL with a backup, and verified by production `/status` and F2 screenshots.

The product version remained `0.4.2` throughout the hotfix. Production installation and release-shaped F2 verification are recorded below; the non-elevated daemon waiver does not extend to a future release.

## Direct Medium-integrity desktop smoke test

**Observed limitation (October 2026):** a Medium-integrity task scheduled with
Interactive / Limited permissions passed elevation and private-ACL checks, but
its Windows Job Object rejected the daemon's deliberate
`CREATE_BREAKAWAY_FROM_JOB` preflight (`Access is denied`, error 5). This
restrictive launch context does not demonstrate normal user-desktop daemon
eligibility. Do not disable or bypass daemon detachment checks to pass a test.

Run the existing standalone package candidate from a **normal non-elevated
desktop PowerShell session** on an eligible Windows host (Daniel-CL has
SSH-only access, with no Remote Desktop). A desktop-launched process must still support
daemon breakaway; not every terminal/host does. The `-Direct` switch refuses
High-integrity execution and does not create a scheduled task:

```powershell
git -C E:\codexdd pull --ff-only
powershell.exe -NoProfile -ExecutionPolicy Bypass -File E:\codexdd\scripts\codexdd-f2-daemon-smoke.ps1 -Direct
```

The test discovers its packaged candidate from
`codexdd-f2-last-smoke-root.txt` in the current user's TEMP folder, or
accepts a previously assembled `-PackageRoot`. It requires the validated
`codexdd 0.4.2+g2caa1b3afa55` debug executable and normalizes only the
**disposable test package's** metadata to the exact snapshot SemVer.

The test creates a uniquely named short-path `CODEX_HOME` with daemon
auto-updates disabled and a strict, protected user-only state-directory ACL.
It requires Medium integrity (`S-1-16-8192`), then uses the real packaged
CLI: `start` → `started`, second `start` → `alreadyRunning`, `version` →
`running` with an app-server version, `stop` → `stopped`, and a second
`stop` → `notRunning`. It also confirms the managed executable and control
socket are within the disposable `CODEX_HOME`. On success it deletes that
temporary profile and restores the caller's prior `CODEX_HOME`. Log files
are retained in `live-smoke-*` under the test package root.

**This standalone test does not change production installation, daemon
security, or user credentials.** The direct non-elevated acceptance gate was
**explicitly waived for PR #48** because this environment could not exercise
it. Do not report it as passed or carry that waiver into 0.4.3. The earlier
Medium-integrity eligibility and elevated SSH F2 checks remain valid.

## Production closure (2026-10-09)

- PR #48 was squash-merged to `dd/astra-policy-v2` at `dc10de2b0240ebe51bba009bfa224fc305c9aa6a` after all required CI passed.
- Daniel-CL built the merged `codex-cli` release target successfully, reporting `codexdd 0.4.2+gdc10de2b0240`.
- The installed executable at `E:\Dev\codex-dd-runtime\codexdd\codex.exe` was replaced after creating a timestamped rollback backup; `E:\Dev\codex-dd-bin\codexdd.cmd` was unchanged.
- Production `codexdd` launched via elevated SSH **without** `--no-daemon`. `/status` showed adaptive effort enabled, Astra preference, Luna Low startup and Full Access.
- The operator's production F2 screenshot showed **Warnings → No warnings**. The original F2 defect is resolved.
- One-time non-elevated detached shared-daemon smoke waiver remains documented as an **unverified** path and is not reusable for CodexDD 0.4.3.

**F2 hotfix: complete and deployed.** The next planned release is [CodexDD 0.4.3 upstream 0.162.0](codexdd-0.4.3-upstream-0.162.0-design.md).
## Remaining expected limitation

Elevated Windows sessions still cannot use a _locally managed shared background daemon_. This fix makes the already-supported embedded mode automatic and removes a misleading warning; it does not implement safe de-elevation or provide daemon-only functionality under SSH administrator tokens.
