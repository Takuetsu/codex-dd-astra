# CodexDD 0.4.5 3F.84 -- local Windows native integration acceptance, NONRELEASE.
# Run from an isolated, clean Daniel-CL worktree. Never installs or publishes.
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$ExpectedHead,
    [string]$LogDir = 'E:\codexdd\logs\045-3f84',
    [switch]$ResumeAfterStackOverflow,
    [switch]$SnapshotRetest
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
$cargoRoot = Join-Path $repoRoot 'codex-rs'
$results = New-Object 'System.Collections.Generic.List[object]'
$started = [System.Diagnostics.Stopwatch]::StartNew()
# Match the verified Windows native CI test-thread stack configuration.
# This changes test harness threads only; it does not alter production runtime stacks.
$env:RUST_MIN_STACK = '16777216'


function Invoke-CargoStage {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Stage,
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments,
        [bool]$ExpectTests = $false
    )
    $out = Join-Path $LogDir ($Stage + '-stdout.log')
    $err = Join-Path $LogDir ($Stage + '-stderr.log')
    Write-Host ('BEGIN: ' + $Stage)
    # Do not pipe cargo's normal stderr through PowerShell 5.1: NativeCommandError
    # can otherwise stop the test run on benign Compiling/warning messages.
    $p = Start-Process -FilePath $cargo.Source -ArgumentList $Arguments `
        -WorkingDirectory $cargoRoot -NoNewWindow -Wait -PassThru `
        -RedirectStandardOutput $out -RedirectStandardError $err
    if ($p.ExitCode -ne 0) {
        Get-Content -LiteralPath $out -Tail 60 -ErrorAction SilentlyContinue
        Get-Content -LiteralPath $err -Tail 100 -ErrorAction SilentlyContinue
        throw ("FAILED: " + $Stage + " exit=" + $p.ExitCode)
    }
    $passed = 0
    if ($ExpectTests) {
        $text = (Get-Content -LiteralPath $out -Raw) + (Get-Content -LiteralPath $err -Raw)
        $testSummary = [regex]::Match($text, 'test result: ok\.\s+([1-9][0-9]*) passed;\s+0 failed;')
        if (-not $testSummary.Success) {
            throw ("FAILED: " + $Stage + " did not execute nonzero passing tests")
        }
        $passed = [int]$testSummary.Groups[1].Value
    }
    $results.Add([pscustomobject]@{ Stage = $Stage; Status = 'PASS'; Passed = $passed; Stdout = $out; Stderr = $err })
    Write-Host ('PASS: ' + $Stage + ' | selected tests: ' + $passed) -ForegroundColor Green
}

if ($env:OS -ne 'Windows_NT') { throw 'Requires native Windows on Daniel-CL' }
if ($ResumeAfterStackOverflow -and $SnapshotRetest) {
    throw 'Select only one retry mode'
}
New-Item -ItemType Directory -Force -Path $LogDir | Out-Null
Push-Location -LiteralPath $repoRoot
try {
    $head = (& git rev-parse HEAD | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $head -cne $ExpectedHead) {
        throw ("Unexpected source SHA: " + $head)
    }
    $dirty = (& git status --porcelain --untracked-files=normal | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $dirty) { throw 'Candidate worktree must be clean' }
    & git diff --check '38800f17744cc600d31b308e17a22ff2462ed013' HEAD
    if ($LASTEXITCODE -ne 0) { throw 'Git diff check failed' }
    $cargo = Get-Command cargo.exe -ErrorAction Stop

    Invoke-CargoStage -Stage '01-rustfmt' -Arguments @('fmt', '--all', '--', '--check')
    Invoke-CargoStage -Stage '02-tui-lib-compile' -Arguments @('test', '--locked', '-p', 'codex-tui', '--lib', '--no-run')

    # SnapshotRetest is a partial gate: it reruns the formerly failing test
    # and the two never-executed cases; it does not claim a fully green 3F.84 suite.
    if ($SnapshotRetest) {
        $previousSource = '3d48af2a0087a3e14b0bb49813233f02ba6f0996'
        $changedRust = @(& git diff --name-only $previousSource HEAD -- codex-rs)
        if ($LASTEXITCODE -ne 0 -or $changedRust.Count -ne 1 -or
            $changedRust[0] -ne 'codex-rs/tui/src/app/tests.rs') {
            throw 'Snapshot retest requires only the audited snapshot test source to change'
        }
    }

    # Run grouped modules, including the 3F.83 four-case replay-pressure suite.
    $groups = @(
        @{ Stage = '03-replay-buffer'; Filter = 'app::thread_event_buffer::tests' },
        @{ Stage = '04-interactive-state'; Filter = 'app::pending_interactive_replay::tests' },
        @{ Stage = '05-live-thread-events'; Filter = 'app::thread_events::tests' }
    )
    if ($SnapshotRetest) {
        Write-Host 'PARTIAL RETEST: skip previous module groups (not counted for new source)'
    }
    elseif ($ResumeAfterStackOverflow) {
        # The previous run reached selector 06, so 03-05 passed on the exact same
        # Rust tree. Do not reuse logs if ANY Rust source changed since that run.
        $previousRunHead = '7a150508a9a25eeb06237ce3cb110f7bb3444317'
        & git diff --exit-code $previousRunHead HEAD -- codex-rs
        if ($LASTEXITCODE -ne 0) {
            throw 'Cannot resume: Rust code or dependency inputs changed since prior run'
        }
        foreach ($group in $groups) {
            $out = Join-Path $LogDir ($group.Stage + '-stdout.log')
            $err = Join-Path $LogDir ($group.Stage + '-stderr.log')
            if (-not (Test-Path -LiteralPath $out) -or
                -not (Test-Path -LiteralPath $err)) {
                throw ('Cannot resume: prior stage evidence missing: ' + $group.Stage)
            }
            $logText = (Get-Content -LiteralPath $out -Raw) +
                (Get-Content -LiteralPath $err -Raw)
            $priorMatch = [regex]::Match(
                $logText,
                'test result: ok\.\s+([1-9][0-9]*) passed;\s+0 failed;'
            )
            if (-not $priorMatch.Success) {
                throw ('Cannot resume: prior stage not proven green: ' + $group.Stage)
            }
            $casesPassed = [int]$priorMatch.Groups[1].Value
            $results.Add([pscustomobject]@{
                Stage = $group.Stage
                Status = 'PASS_REUSED_IDENTICAL_RUST_SOURCE'
                Passed = $casesPassed
                Stdout = $out
                Stderr = $err
            })
            Write-Host ('PASS previously: ' + $group.Stage +
                ' | cases: ' + $casesPassed) -ForegroundColor Green
        }
    }
    else {
        foreach ($group in $groups) {
            Invoke-CargoStage -Stage $group.Stage -Arguments @(
                'test', '--locked', '-p', 'codex-tui', '--lib', $group.Filter
            ) -ExpectTests $true
        }
    }

    # Exact UI-level path validation: every selector must execute one real test.
    $cases = @(
        'enqueue_primary_thread_session_replays_buffered_approval_after_attach'
        'resolved_buffered_approval_does_not_become_actionable_after_drain'
        'enqueue_thread_event_does_not_block_when_channel_full'
        'inactive_thread_overflow_does_not_starve_foreground_or_block_switch'
        'oversized_live_notification_is_rejected_before_replay_is_mutated'
        'replay_thread_snapshot_renders_only_retained_agent_message_deltas'
        'replay_thread_snapshot_restores_draft_and_queued_input'
        'replay_thread_snapshot_does_not_submit_queue_before_replay_catches_up'
        'inactive_thread_approval_bubbles_into_active_view'
        'replay_snapshot_with_pending_request_retains_warnings_without_covering_approval'
        'replayed_file_change_approval_recovers_snapshot_changes'
        'feedback_submission_for_inactive_thread_replays_into_origin_thread'
        'background_resolution_keeps_matching_foreground_user_input_visible'
        'inactive_thread_approval_badge_clears_after_turn_completion_notification'
        'app_server_thread_replacement_clears_previous_transcript_before_replay'
        'snapshot_thread_switch_discards_queued_previous_history'
        'active_history_batch_is_delivered_without_replay_buffering'
    )
    $index = 6
    foreach ($test in $cases) {
        if ($SnapshotRetest -and $index -lt 20) {
            $index += 1
            continue
        }
        $stage = ('{0:D2}-{1}' -f $index, $test)
        Invoke-CargoStage -Stage $stage -Arguments @(
            'test', '--locked', '-p', 'codex-tui', '--lib',
            ('app::tests::' + $test), '--', '--exact'
        ) -ExpectTests $true
        $index += 1
    }
    $total = ($results | Measure-Object -Property Passed -Sum).Sum
    $priorGroupSource = $null
    if ($ResumeAfterStackOverflow) {
        $priorGroupSource = '7a150508a9a25eeb06237ce3cb110f7bb3444317'
    }
    $receiptKind = 'codexdd_045_3f84_windows_native_nonrelease'
    if ($SnapshotRetest) {
        $receiptKind = 'codexdd_045_3f84_snapshot_retest_partial_nonrelease'
    }
    $receipt = [pscustomobject]@{
        kind = $receiptKind
        source_head = $head
        utc = (Get-Date).ToUniversalTime().ToString('o')
        executed_passed = $total
        partial_retest_only = [bool]$SnapshotRetest
        resumed_prior_native_groups = [bool]$ResumeAfterStackOverflow
        prior_native_group_source = $priorGroupSource
        test_thread_stack_bytes = [int]$env:RUST_MIN_STACK
        stages = @($results)
        release_approved = $false
    }
    $receipt | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $LogDir 'receipt.json') -Encoding UTF8
    if ($SnapshotRetest) {
        Write-Host ('3F.84 PARTIAL RETEST GREEN: ' + $total + ' selected native cases') -ForegroundColor Green
        Write-Host 'Full 3F.84 run remains pending for final native evidence'
    }
    else {
        Write-Host ('3F.84 INTEGRATED LOCAL GREEN: ' + $total + ' native tests') -ForegroundColor Green
    }
    Write-Host ('Receipt: ' + (Join-Path $LogDir 'receipt.json'))
}
finally {
    Pop-Location
    $started.Stop()
    Write-Host ("Finished native validation runner in " + $started.Elapsed.ToString())
}
