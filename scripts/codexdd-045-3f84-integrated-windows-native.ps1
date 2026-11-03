# CodexDD 0.4.5 3F.84 -- local Windows native integration acceptance, NONRELEASE.
# Run from an isolated, clean Daniel-CL worktree. Never installs or publishes.
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$ExpectedHead,
    [string]$LogDir = 'E:\codexdd\logs\045-3f84'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
$cargoRoot = Join-Path $repoRoot 'codex-rs'
$results = New-Object 'System.Collections.Generic.List[object]'
$started = [System.Diagnostics.Stopwatch]::StartNew()

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
        if ($text -notmatch 'test result: ok\.\s+([1-9][0-9]*) passed;\s+0 failed;') {
            throw ("FAILED: " + $Stage + " did not execute nonzero passing tests")
        }
        $passed = [int]$Matches[1]
    }
    $results.Add([pscustomobject]@{ Stage = $Stage; Status = 'PASS'; Passed = $passed; Stdout = $out; Stderr = $err })
    Write-Host ('PASS: ' + $Stage + ' | selected tests: ' + $passed) -ForegroundColor Green
}

if ($env:OS -ne 'Windows_NT') { throw 'Requires native Windows on Daniel-CL' }
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

    # Run grouped modules, including the 3F.83 four-case replay-pressure suite.
    $groups = @(
        @{ Stage = '03-replay-buffer'; Filter = 'app::thread_event_buffer::tests' },
        @{ Stage = '04-interactive-state'; Filter = 'app::pending_interactive_replay::tests' },
        @{ Stage = '05-live-thread-events'; Filter = 'app::thread_events::tests' }
    )
    foreach ($group in $groups) {
        Invoke-CargoStage -Stage $group.Stage -Arguments @(
            'test', '--locked', '-p', 'codex-tui', '--lib', $group.Filter
        ) -ExpectTests $true
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
        $stage = ('{0:D2}-{1}' -f $index, $test)
        Invoke-CargoStage -Stage $stage -Arguments @(
            'test', '--locked', '-p', 'codex-tui', '--lib',
            ('app::tests::' + $test), '--', '--exact'
        ) -ExpectTests $true
        $index += 1
    }
    $total = ($results | Measure-Object -Property Passed -Sum).Sum
    $receipt = [pscustomobject]@{
        kind = 'codexdd_045_3f84_windows_native_nonrelease'
        source_head = $head
        utc = (Get-Date).ToUniversalTime().ToString('o')
        executed_passed = $total
        stages = @($results)
        release_approved = $false
    }
    $receipt | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $LogDir 'receipt.json') -Encoding UTF8
    Write-Host ('3F.84 INTEGRATED LOCAL GREEN: ' + $total + ' native tests') -ForegroundColor Green
    Write-Host ('Receipt: ' + (Join-Path $LogDir 'receipt.json'))
}
finally {
    Pop-Location
    $started.Stop()
    Write-Host ("Finished native validation runner in " + $started.Elapsed.ToString())
}
