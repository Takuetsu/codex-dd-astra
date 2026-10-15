#Requires -Version 5.1
[CmdletBinding()]
param(
    [Parameter(Mandatory=$true)][ValidatePattern('^[0-9a-fA-F]{40}$')][string]$ExpectedCandidateSha,
    [string]$RunRoot = 'E:\codexdd-validation-runs'
)

# Read-only, reconnectable monitor for the exact current one-shot task.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$index = Join-Path $RunRoot 'codexdd-044-current.json'
if (!(Test-Path -LiteralPath $index -PathType Leaf)) {
    throw 'No CodexDD 0.4.4 release launch index found; do not start a new run automatically.'
}
$run = Get-Content -LiteralPath $index -Raw | ConvertFrom-Json -ErrorAction Stop
if ($run.candidate_sha -cne $ExpectedCandidateSha.ToLowerInvariant()) {
    throw "Monitor candidate does not match the last recorded task: $($run.candidate_sha)"
}
$task = [string]$run.task_name
$dir = [IO.Path]::GetFullPath([string]$run.run_dir)
$root = [IO.Path]::GetFullPath($RunRoot).TrimEnd('\') + '\'
if (!$dir.StartsWith($root, [StringComparison]::OrdinalIgnoreCase) -or
    !$task.StartsWith("CodexDD-044-$($ExpectedCandidateSha.Substring(0,12))-")) {
    throw 'The launch index contains an unexpected task/path. Refusing to inspect it.'
}
$resultFile = Join-Path $dir 'release-result.json'
$log = Join-Path $dir 'release-stdout.log'
Write-Host "CodexDD 0.4.4 release monitor — $($run.candidate_sha)"
Write-Host "Task: $task"
Write-Host "Run directory: $dir"
Write-Host 'Ctrl+C stops monitoring only. It does not stop or restart the Scheduled Task.'

$notRunning = 0
while (!(Test-Path -LiteralPath $resultFile -PathType Leaf)) {
    $scheduled = Get-ScheduledTask -TaskName $task -ErrorAction SilentlyContinue
    $state = if ($scheduled) { [string]$scheduled.State } else { 'NotFound' }
    Write-Host ("{0} | Task {1}" -f (Get-Date -Format 'HH:mm:ss'), $state)
    if (Test-Path -LiteralPath $log -PathType Leaf) {
        $lastStage = @(Get-Content -LiteralPath $log -Tail 600 |
            Where-Object {
                $_ -match '^CODEXDD_VALIDATION_JSON ' -and
                $_ -match '"event":"stage_(begin|end)"'
            } | Select-Object -Last 1)
        if ($lastStage.Count -gt 0) {
            $obj = ($lastStage[0] -replace '^CODEXDD_VALIDATION_JSON ', '') |
                ConvertFrom-Json -ErrorAction Stop
            Write-Host ("{0}/21 | {1} | {2} {3}" -f $obj.ordinal,
                $obj.stage, $obj.event, $obj.status)
        }
    }
    if ($state -ne 'Running') { $notRunning++ } else { $notRunning=0 }
    if ($notRunning -ge 4) {
        Write-Warning 'Task is not running and no completion receipt was found. Preserve logs.'
        break
    }
    Start-Sleep -Seconds 30
}

if (Test-Path -LiteralPath $resultFile -PathType Leaf) {
    $result = Get-Content -LiteralPath $resultFile -Raw | ConvertFrom-Json -ErrorAction Stop
    Write-Host ("RELEASE RESULT: {0}" -f $result.status.ToUpperInvariant())
    $result | Format-List candidate_sha,status,exit_code,first_failed_stage,detail,receipt_path
    if ($result.status -ceq 'pass' -and $result.exit_code -eq 0 -and
        $result.candidate_sha -ceq $ExpectedCandidateSha.ToLowerInvariant() -and
        !( [string]::IsNullOrWhiteSpace([string]$result.receipt_path) ) -and
        (Test-Path -LiteralPath $result.receipt_path -PathType Leaf)) {
        $receipt = Get-Content -LiteralPath $result.receipt_path -Raw | ConvertFrom-Json -ErrorAction Stop
        if ($receipt.candidate_head_sha -ceq $ExpectedCandidateSha.ToLowerInvariant() -and
            $receipt.status -ceq 'pass' -and $receipt.completed_stages -eq 21 -and
            $receipt.exit_code -eq 0) {
            Write-Host 'CODEXDD_044_RELEASE_21_OF_21_PASS' -ForegroundColor Green
            return
        }
    }
    Write-Warning '21-stage release PASS is not confirmed. Save the task logs; do not retry blindly.'
} else {
    $taskInfo = Get-ScheduledTaskInfo -TaskName $task -ErrorAction SilentlyContinue
    if ($taskInfo) {
        $taskInfo | Format-List LastRunTime,LastTaskResult
    }
    Write-Warning "No release-result.json. Inspect $dir and do not start another release."
}
