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

## CodexDD 0.4.3 one-command launcher

The repository includes [`scripts/codexdd-start-detached-v2-release.ps1`](../scripts/codexdd-start-detached-v2-release.ps1). This launcher is intentionally **not** merged into the frozen v0.4.3 candidate. Obtain it at an exact documentation-branch Git object and execute a copy **outside the candidate worktree**. Do not amend the candidate to gain access to the launcher.

The launcher:

- Refuses non-Daniel-CL hosts, unexpected candidate SHA, wrong branch, dirty worktree, original blocked-discovery SHA drift, or sidecar drift.
- Creates a **unique per-attempt external directory** under `E:\codexdd-validation-runs` for the candidate, local runner, nonsecret plan, native logs and candidate-bound receipt. Creates a **separate short per-attempt** `E:\ddtmp\<run-id>` scratch directory for both `TEMP` and `TMP` (path budget at most 60 characters). A deep nested log directory is unsuitable for Windows AF_UNIX socket tests.
- Registers and starts an S4U task-account smoke check for Git/Python/Cargo/nextest, the previously required `test_stdio_server.exe`, **one real ConPTY-backed CLI test** (`daemon_exclusion_survives_resume_picker`), **all 15 unfiltered TUI daemon-startup tests**, **and the full 268-test thread-store library stage**. This is a bounded *preflight suite* compared with all 21 release stages; cache misses can still add build time. Saves `cli-pty-smoke.log`, `tui-daemon-smoke.log` and `thread-store-smoke.log`. Any failure **blocks** the long release test without a waiver. The launcher removes the successful probe task.
- Registers and starts one **manual, nonrecurring** eight-hour-bounded `CodexDD-043-<sha>-<id>-release` task, configured to ignore duplicate executions. **No Git ref, source file or candidate manifest is modified.**
- The task executes the frozen candidate's unmodified v2 entrypoint with both approved blocked-discovery checksums. Native stdout/stderr are redirected to logs, the child exit code is propagated, and a full 21-stage PASS must include the exact candidate/source/production-bound JSON receipt copied to the run directory.
- Distinguishes a `CODEXDD_DETACHED_RELEASE_STARTED` **launch acknowledgement** from actual `release-result.json` **PASS**. Never interpret a successful scheduler start, partial profile, missing result file or prior targeted-test PASS as native release completion.

Current CodexDD 0.4.3 release candidate revision r3 is `9349fa6d487ce3f64a07a3c468c7a1395d6d3787` (source `0b11064b373fac94be94f5e3a6e12ee2642781a9`). Revision r2 at `677dd64923afd4a2236075b9da92e5f6ff500485` is archived after stage-15 app-server and stage-20 CLI failures. The MCP fixture test passed 1/1 after compiling `test_stdio_server.exe`, and the corrected Windows daemon-startup integration tests passed **5/5** on Daniel-CL at source SHA `8fd46fbd52fd6bb98b17682bf80ceeea3504a175`. A **fresh full 21-stage native receipt for revision r3 is still required**. Obtain approved hashes and the exact launcher Git SHA from the release handoff; never trust moving branch tips.

**S4U limitation:** The same user name does not guarantee the same interactive profile or network credentials. The first one-shot Scheduled Task smoke check is required; if it fails, do **not** automatically fall back to SSH-owned execution, `SYSTEM`, or an elevated account. Review its result and fix the account context first.

## Git for Windows archive checksum guard (observed October 2026)

When extracting the exact launcher commit into a **separate temporary directory** for Daniel-CL, note that `git archive` on Windows **can apply `core.autocrlf=true` working-tree conversion to archived `.ps1` files**. The Git blob is LF-only but the ZIP entry can be CRLF. As a result, `git hash-object --no-filters <extracted-file>` may differ from the committed Git blob although the text commands appear identical. This was reproduced in a separate Git fixture. The October 12 operator launch printed `LAUNCHER CONTENT MISMATCH` but then proceeded after the interactive `throw`, which is **not** an acceptable verification sequence for future runs. Its subsequent `CODEXDD_DETACHED_RELEASE_STARTED` proves the Scheduled Task launch command completed, **not** a native release PASS.

For **future** retrieval, force raw-blob archive bytes and check before running anything:

```powershell
# Set exact release-specific commit/expected Git blob and already allocated destination first.
git -c core.autocrlf=false -C $checkout archive --format=zip -o $zip $launcherCommit `
    'scripts/codexdd-start-detached-v2-release.ps1'
if ($LASTEXITCODE -ne 0) { throw 'Exact launcher archive failed' }
Expand-Archive -LiteralPath $zip -DestinationPath $destination -ErrorAction Stop
$actual = (git -C $checkout hash-object --no-filters $launcher).Trim()
if ($LASTEXITCODE -ne 0 -or $actual -cne $expectedBlob) {
    throw 'Exact launcher Git blob mismatch: DO NOT RUN'
}
# Launch the verified script only inside the same fail-fast script/function.
```

A failing guard must **terminate the whole launcher orchestration**. Pasting a second, separate PowerShell invocation after an earlier interactive `throw` bypasses the guard. Do not repair the hash by mutating the frozen candidate, disabling integrity checks, or running multiple concurrent release profiles. For the October 12 **already-running** detached candidate, do **not** stop or restart the task merely to improve extraction; await its actual `release-result.json` and preserve all logs/receipts.

## October 12, 2026 Windows S4U ConPTY gate

A detached 0.4.3 release run on Daniel-CL completed the preceding release stages but failed `cli-daemon-startup-tests`: **four of five Windows tests timed out** (`deadline has elapsed`), while the Bedrock onboarding test passed. The failing tests all wait for TUI startup readiness in a synthetic ConPTY terminal; the successful Bedrock test uses a different startup flow. The output proves a real stage-20 failure, **not** a complete native PASS. The exact reason (S4U noninteractive context, TUI startup/model display, or another runtime issue) remains unresolved without an interactive control run.

**Future S4U runs must pass the exact single-test ConPTY probe before launching all 21 stages.** If it fails, do not repeat the whole profile, reclassify failures as success, disable/exclude these tests, or forge a `profile_end` receipt. First compare the same targeted test in an actual elevated SSH/interactive context. If it passes only interactively, the full profile must run in a supported interactive execution context while maintaining the normal SHA-bound receipt contract; schedule detached noninteractive regression checks separately. If it fails in both, investigate the source and captured TUI error before any source correction.

The previous S4U run's `release-result.json` used `exit_code: 2` and `detail: Release terminal or receipt path missing/ambiguous`, but its `first_failed_stage` was `cli-daemon-startup-tests` and the native log showed real timeout failures. The reporting logic was corrected in this PR to prioritize stage failures. **Neither the report correction nor this added smoke test modifies the frozen 0.4.3 candidate.**

## Windows PowerShell 5.1 inline Python quoting failure (2026-10-12)

During the revision-r3 launch, the candidate checkout updated correctly to `9349fa6d487ce3f64a07a3c468c7a1395d6d3787`, but the *separate launcher extraction* failed before task creation. A single-quoted PowerShell argument to `python.exe -c '...'` lost interior Python string quotation marks when passed through the Windows PowerShell 5.1 native process argument marshaller. Python returned `SyntaxError` and `RUNNER EXTRACTION FAILED`. **No new Scheduled Task or release test was started by that failed attempt.** This is a shell-command handoff defect, not a source or Cargo test failure.

For all future launcher retrieval in Windows PowerShell 5.1, **do not use embedded `python -c` string literals** as a raw Git-byte extraction method. Use the earlier `git -c core.autocrlf=false archive` procedure, extract the ZIP outside the candidate checkout, and verify the exact Git blob with `git hash-object --no-filters` **inside the same fail-fast PowerShell script block before execution**. If the blob differs, stop and diagnose; never fall back to an unchecked launcher. This archive/hash method does not change the frozen candidate or source and does not repeat prior successful tests.

## October 12, 2026 S4U temporary SQLite file failure

Revision-r3 candidate `9349fa6d487ce3f64a07a3c468c7a1395d6d3787` was extracted with verified Git blob and launched through an S4U Scheduled Task. Its full release stopped at **stage 8** (`thread-store-tests`, 268 cases) after numerous short test failures. The saved native trace shows SQLx/SQLite `code: 14 unable to open database file` at `C:\Users\ddm55\AppData\Local\Temp\.tmp68KFl\state_5.sqlite`. Two targeted thread-store tests executed successfully (2/2) in the normal elevated SSH account and source/fixture blobs match official upstream 0.162.0. Thus, suspect the **S4U task's inherited temp-file access context**, not 268 independent runtime regressions. This is a working hypothesis, not a proven Windows ACL diagnosis.

**First remediation and evidence:** an October 12 r3 detached attempt used `<run directory>\task-temp` under both `TEMP` and `TMP`; the real S4U CLI ConPTY smoke and **268/268** thread-store smoke passed and the following full 21-stage run progressed past stage 8. The chosen scratch directory was later proven too long for the Windows AF_UNIX socket regression at stage 19. For further attempts, the launcher uses the **short local `E:\ddtmp\<run-id>`** (or the same volume as a nondefault local run root), while preserving a unique external log directory. Each S4U worker checks the run-bound scratch identity, enforces a 60-character socket path budget, performs a create/write/delete canary, and sets both variables **before** launching its children. The full pinned 15-case TUI daemon test stage, 268-case thread-store stage, and one ConPTY CLI test must all PASS in that **exact S4U task context** before any 21-stage run. Any failure preserves its separate log and stops before release launch; none of the tests are waived.

Temporary contents are stored outside the candidate worktree and should be retained until the completed release evidence is reviewed. The bounded short scratch path and the long log/receipt path are deliberately separate. Do not manually change ACLs in the user's home directory, run as SYSTEM, or edit the frozen candidate to work around SQLite errors or socket length limits. Historical release failures remain genuine FAILs, with no native receipt.

## October 12, 2026 r3 stage-19 AF_UNIX path-length failure

The second detached r3 attempt (`CodexDD-043-9349fa6d487c-20261012-165708-da995b3d-release`) completed the corrected preflight and passed **18 of 21** native stages. Stage 19 `tui-daemon-startup-tests` ran **15 tests: 14 PASS, 1 FAIL**; the single failing assertion was `codex-tui daemon_startup_tests::daemon_connection_rejects_unprotected_socket_before_handshake`, returning `path must be shorter than SUN_LEN` at `codex-rs/tui/src/daemon_startup_tests.rs:436:24`. Stages 20–21 were not reached, `release-result.json` reported `exit_code: 2`, and **no native receipt exists**.

The test constructs a temporary Unix-domain socket under `TEMP` with a nested `control/server.sock` suffix. Because the previous scratch path was nested inside a long `E:\codexdd-validation-runs\<40-char SHA>\<run-id>` log directory, that filesystem path exceeded the native Windows socket name limit; changing the production TUI/runtime or weakening the socket-protection assertion is unnecessary and prohibited by the candidate freeze. The new external launcher instead allocates a short dedicated local scratch path, verifies S4U access, and requires the exact **15/15 TUI daemon tests** in S4U preflight before starting the costly release. This is a corrective proposal pending actual Daniel-CL validation, not a claimed native PASS. Preserve the failed task's result and logs separately.

## October 12, 2026 S4U TUI smoke parsing correction

The next short-path r3 S4U preflight (`20261012-172511-84f273df`) ran the exact `cargo nextest run --no-fail-fast -p codex-tui --lib daemon_startup_tests` release-stage selector on Daniel-CL, including the previously failing AF_UNIX socket test. Native Nextest output showed **15 tests run: 15 passed, 5827 skipped** in 0.990 seconds; the task itself returned **native exit 0**. The launcher then mistakenly rejected the passed run because its summary expression required `0 skipped`. The 5,827 are *unselected unrelated TUI library tests* (Nextest's filtered skip count), **not skipped members of the 15 selected cases**. The launcher stopped fail-closed and **did not start another 21-stage release**; this is a launcher parser defect, not evidence of a new source failure.

Launcher commit `1de016c521a47f0e0c65767083a85ef1a1c57fe8` requires native exit 0 and exactly `15 tests run: 15 passed`, accepts the nonnegative filtered skip count, and retains the **unchanged 15-test release-stage selection**. Parser cases were checked against the observed positive output and several fail/partial/native-nonzero negatives. The existing 268/268 SQLite gate, ConPTY gate, immutable candidate/source checks, and full native receipt requirement are unchanged. This correction is **GitHub-side only until a new Daniel-CL attempt supplies fresh evidence**.

## Mandatory live monitor after every 21-stage launch

**Operator handoff contract:** As soon as the launch command prints `CODEXDD_DETACHED_RELEASE_STARTED`, the assistant must provide the operator a **copy/paste-ready PowerShell monitor script**, populated with that launch's exact **Task name** and **Run directory**. This is required **every time a full 21-stage profile starts**, including retries and revised candidates. Do not wait for a separate request, send placeholder task IDs, or schedule periodic notifications. The monitor runs independently of the release, polls about every 30 seconds, reports the most recent begin/end stage when present in the recent log tail, and stops automatically when `release-result.json` appears.

Template for the assistant to specialize (replace both placeholder values in the *user-facing* command):

```powershell
$run = 'REPLACE_WITH_EXACT_RUN_DIRECTORY_FROM_LAUNCHER'
$task = 'REPLACE_WITH_EXACT_TASK_NAME_FROM_LAUNCHER'
$resultFile = Join-Path $run 'release-result.json'
$log = Join-Path $run 'release-stdout.log'

while (-not (Test-Path -LiteralPath $resultFile)) {
    $scheduledTask = Get-ScheduledTask -TaskName $task -ErrorAction SilentlyContinue
    $state = if ($scheduledTask) { $scheduledTask.State } else { 'NotFound' }
    Write-Host "`n$(Get-Date -Format 'HH:mm:ss') - Task: $state"

    if (Test-Path -LiteralPath $log) {
        $latest = Get-Content -LiteralPath $log -Tail 500 |
            Where-Object {
                $_ -match '^CODEXDD_VALIDATION_JSON ' -and
                $_ -match '"event":"stage_(begin|end)"'
            } | Select-Object -Last 1
        if ($latest) {
            $eventData = ($latest -replace '^CODEXDD_VALIDATION_JSON ', '') |
                ConvertFrom-Json
            Write-Host "$($eventData.ordinal)/21 - $($eventData.stage) - $($eventData.event)"
        }
    }

    if ($state -ne 'Running') {
        Start-Sleep -Seconds 5
        if (-not (Test-Path -LiteralPath $resultFile)) {
            Write-Warning 'Task stopped without a release result. Inspect logs.'
            break
        }
    } else {
        Start-Sleep -Seconds 30
    }
}

if (Test-Path -LiteralPath $resultFile) {
    $result = Get-Content -LiteralPath $resultFile -Raw | ConvertFrom-Json
    Write-Host "`nRELEASE FINISHED: $($result.status.ToUpper())"
    $result | Format-List status,exit_code,first_failed_stage,detail,receipt_path

    if ($result.status -eq 'pass' -and $result.exit_code -eq 0 -and
        $result.receipt_path -and (Test-Path -LiteralPath $result.receipt_path)) {
        Write-Host '21-stage release PASS with receipt found.'
    } else {
        Write-Warning 'Full release PASS with receipt is not confirmed.'
    }
}
```

The monitor is **read-only**; `Ctrl+C` stops polling but **does not interrupt the independently running Scheduled Task**. A successful `CODEXDD_DETACHED_RELEASE_STARTED`, task state `Ready`, partial stage PASS, or monitor exit is **not** a full release PASS. Preserve the immutable release evidence on failure. The full native receipt, candidate/source/production binding, and GitHub promotion gates remain authoritative.

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

This runbook and separate launcher are staged on a **draft docs/tools branch**, outside the frozen v0.4.3 runtime and canonical candidate. They do not alter, approve, or retroactively change a completed historical validation attempt. Because candidate release receipts bind source and production identities, **do not merge these changes into production or the frozen source branch until the v0.4.3 native receipt gate is closed**. Reconcile this branch against the eventual new production commit before merging it under the normal release process.

The next time a full native Windows release validation is started, use the detached Task Scheduler pattern. Cheap targeted validation remains local-first; broad GitHub CI remains after the applicable native Windows gates.
