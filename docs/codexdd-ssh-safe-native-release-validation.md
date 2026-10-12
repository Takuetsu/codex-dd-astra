# CodexDD — SSH-safe native Windows release-validation runbook

**Status: STANDARD PROCEDURE FOR FUTURE LONG-RUNNING RELEASE GATES.** Applies to Daniel-CL Windows over SSH. The existing `targeted` and `work_packet` validation profiles remain unchanged. This procedure changes **how** the full native release profile is launched, not what it tests or what proof is accepted.

## Why this exists

PowerShell launched inside an SSH session is not reliably durable across SSH disconnects, laptop sleep or session teardown. A successful SSH command that *starts* a lengthy test does not guarantee the child process survives. **For long-running candidate-bound 21-stage and other broad Windows release gates, launch a manually started Windows Scheduled Task that owns the test process independently of SSH.** Never attempt to "detach" a test that is already running; let it finish as-is or follow the failure recovery contract.

## Non-negotiable requirements

1. **One immutable candidate per run.** Resolve and record repository root, candidate branch, full Git HEAD and clean worktree before starting. Use the exact approved v2 manifest/discovery digests, production/source SHA and official target as required by that release. Refuse any mismatched or moved identity rather than rebasing/resetting active work. Never launch a task against a noncanonical branch.
2. **Only a manual, one-shot scheduler task.** Create a per-SHA named task and call `Start-ScheduledTask` exactly once. Do not create a recurring schedule or auto-retry a failed profile. Configure duplicate-instance policy to **IgnoreNew**; do not run two jobs using the same Cargo build directory.
3. **Run as the appropriate operator account.** Prefer the same nonprivileged local user and toolchain environment as the validated candidate checkout. A Task Scheduler principal with `LogonType S4U` can run without an interactive login if permitted, but **does not have reusable network credentials** and can have a different PATH/profile; all sources/artifacts must already be available locally or a separately authenticated identity must be arranged securely. If that principal cannot run in batch mode, **stop**; do not switch to `SYSTEM`, elevate or store a password in repository files or task arguments simply to get green.
4. **Keep the repository untouched during execution.** Both scheduled job definition and outputs belong outside the working tree (e.g. `E:\codexdd-validation-runs\<candidate-sha>\<run-id>\`). Use a unique directory per attempt; record invocation, start/end timestamps, exit code, all stdout/stderr and the exact `CODEXDD_VALIDATION_JSON` terminal events. Save canonical candidate-bound `CODEXDD_UPSTREAM_VALIDATION_RECEIPT_PATH` if emitted. Protect logs and receipts as development artifacts (may contain paths/diagnostics/secrets).
5. **The job runs the normal trusted entrypoint.** For a v2 reconciled candidate, invoke `scripts/codexdd-validate-upstream-candidate-v2.ps1` from the correct checkout with its three documented parameters (manifest path and the two independently approved SHA-256 digests); the wrapper must not skip native preflight, postflight, profile stages or native receipt checks. The child process exit status must be propagated as the scheduled task's process exit code. Set bounded `CARGO_BUILD_JOBS`, `RUST_MIN_STACK` and tool PATH as needed for that account, never to relax validation.
6. **Readiness checks precede the long run.** Verify the task account can access the local checkout, Git, the real Python executable, Cargo, nextest, Rust toolchain and local output folder using a **cheap short dry-run/smoke**; log the resolved executable paths. If any tool or batch-logon capability is missing, fix the environment once before the heavyweight test. The dry run is not a release PASS.
7. **Only evidence completes the gate.** A task existing, started or marked `Ready` is not evidence. Require a true `profile_end` for `release`, successful native exit status, **all required stages**, unmodified candidate HEAD, and the candidate-SHA/source/provenance-bound receipt. On FAIL, record `failed_stage` and its actual exit code; don't mask errors with a manually printed PASS label.
8. **Preserve separate interactive tests.** Scheduled Tasks are for noninteractive builds/test suites; they **cannot** replace the real elevated-SSH F2/TUI or non-elevated shared-daemon smoke tests that demand the actual operator launch context.

## One-shot Task Scheduler pattern (operator SSH; future release runs)

The assistant prepares and checks the repository-owned runner script and the *exact release-specific inputs* first. The operator should only need a short task-registration/start command and later a status command. A Scheduled Task runs a PowerShell **wrapper script stored outside the worktree**; the wrapper must implement the safety and logging contract above.

```powershell
# Example only. Prepare a signed-off, exact-candidate runner outside the repo.
# Never use the example paths/task name without reviewing the current candidate.
$taskName = 'CodexDD-Release-<candidate-sha12>'
$runner = 'E:\codexdd-validation-runs\<candidate-sha>\<run-id>\run.ps1'
$checkout = 'E:\codexdd-043-candidate-preflight'

if (-not (Test-Path -LiteralPath $runner -PathType Leaf)) {
    throw 'Prepared release runner not found; do not start a task.'
}

$principal = New-ScheduledTaskPrincipal `
    -UserId "$env:USERDOMAIN\$env:USERNAME" `
    -LogonType S4U -RunLevel Limited
$action = New-ScheduledTaskAction `
    -Execute "$env:SystemRoot\System32\WindowsPowerShell\v1.0\powershell.exe" `
    -Argument ('-NoProfile -NonInteractive -ExecutionPolicy Bypass -File "{0}"' -f $runner) `
    -WorkingDirectory $checkout
$settings = New-ScheduledTaskSettingsSet `
    -MultipleInstances IgnoreNew -ExecutionTimeLimit (New-TimeSpan -Hours 6)

# Manual-start only: NO recurring trigger. Do not overwrite an existing task.
if (Get-ScheduledTask -TaskName $taskName -ErrorAction SilentlyContinue) {
    throw 'Task already registered; inspect existing run instead.'
}
Register-ScheduledTask -TaskName $taskName -Action $action `
    -Principal $principal -Settings $settings `
    -Description 'CodexDD immutable native Windows release validation'
Start-ScheduledTask -TaskName $taskName
```

**Prerequisites and limitations:** Registration rights and batch-logon permission are Windows policy dependent. The wrapper must exist and must log/return the actual validator outcome; the above registers/starts it but does **not** magically create a valid release runner. A manual-only task has no scheduler-triggered recurring executions. Do **not** include passwords/tokens in the runner, task parameters or logs. If the candidate uses an account-specific Rust/Python installation, smoke the Scheduled Task identity before a heavy run. Never run a second profile merely because an SSH session disconnected.

## Reconnect, inspect, and clean up

From any later SSH session to Daniel-CL:

```powershell
Get-ScheduledTask -TaskName 'CodexDD-Release-<candidate-sha12>' |
    Select-Object TaskName, State
Get-ScheduledTaskInfo -TaskName 'CodexDD-Release-<candidate-sha12>' |
    Select-Object LastRunTime, LastTaskResult, NumberOfMissedRuns
# Inspect the per-SHA run log and receipt under E:\codexdd-validation-runs.
```

- `Running`: continue waiting; disconnection does not justify rerun.
- `Ready` or `Disabled`: inspect `LastRunTime`, `LastTaskResult`, the wrapper's terminal record and complete log before interpreting it. `LastTaskResult = 0` alone is insufficient for a release PASS.
- Nonzero, missing terminal record or no receipt: classify as **validation failure** or **infrastructure interruption**, preserve evidence, and route the **smallest** repair. Partial stages may inform diagnosis, but are never silently promoted to PASS. Cargo caches may remain reusable.
- After a durable candidate-bound native receipt is archived and inspected, unregister the **specific completed task** (`Unregister-ScheduledTask -TaskName ... -Confirm:$false`) and apply bounded log retention. Do not delete evidence that has not been incorporated in the release handoff.

## Adoption boundary for CodexDD 0.4.3

This runbook is a **documentation-only change** prepared while the v0.4.3 release profile was already running under SSH. It **does not transfer, cancel, restart, detach, approve or alter that running process**. Because the current canonical candidate and native receipt are pinned to production/source commit identities, **do not merge these docs into the live production or frozen source branch mid-validation**; stage this policy separately and merge it only when those identities no longer need to remain unchanged, with the normal release workflow.

The next time a full native Windows release validation is started, use the detached Task Scheduler pattern. Cheap targeted validation remains local-first; broad GitHub CI remains after the applicable native Windows gates.
