# CodexDD 0.4.5 native release harness. Source-only preflight is default.
[CmdletBinding()]
param(
    [ValidateSet('Preflight', 'Final')][string]$Mode = 'Preflight',
    [string]$ExpectedCommit = '',
    [string]$LogPath = ''
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'codexdd-python-executable.ps1')
$root = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
$watch = [System.Diagnostics.Stopwatch]::StartNew()
$head = ''
$exitCode = 1
function Invoke-CodexDDChecked {
    param([string]$Label, [string]$Command, [string[]]$Arguments)
    Write-Host ('BEGIN: ' + $Label)
    $old = $ErrorActionPreference
    try {
        $ErrorActionPreference = 'Continue'
        & $Command @Arguments 2>&1 | ForEach-Object { Write-Host ([string]$_) }
        $status = $LASTEXITCODE
    } finally { $ErrorActionPreference = $old }
    if ($status -ne 0) { throw ('FAILED: ' + $Label + ', exit=' + $status) }
    Write-Host ('PASS: ' + $Label)
}
Push-Location -LiteralPath $root
try {
    if ($env:OS -ne 'Windows_NT' -or $env:COMPUTERNAME -ine 'Daniel-CL') {
        throw 'Native 0.4.5 validation must run on Daniel-CL Windows'
    }
    $head = (& git rev-parse HEAD | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $head -cnotmatch '^[0-9a-f]{40}$') {
        throw 'Candidate HEAD is not an exact Git SHA'
    }
    if ($ExpectedCommit) {
        if ($ExpectedCommit -cnotmatch '^[0-9a-f]{40}$' -or $head -cne $ExpectedCommit) {
            throw 'Candidate HEAD differs from explicit expected commit'
        }
    } elseif ($Mode -eq 'Final') {
        throw 'Final mode requires -ExpectedCommit (full 40-character SHA)'
    }
    $before = (& git status --porcelain --untracked-files=no | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $before) { throw 'Tracked source dirty before validation' }
    $python = Resolve-CodexDDPythonPath -Candidates @(Get-Command python -All -ErrorAction SilentlyContinue)
    $version = (& $python --version | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $version -notmatch '^Python 3\.(1[1-9]|[2-9][0-9])\.') {
        throw ('Python 3.11+ required: ' + $version)
    }
    $env:PYTHONDONTWRITEBYTECODE = '1'
    Write-Host ('CODEXDD_045_CANDIDATE_HEAD=' + $head)
    Invoke-CodexDDChecked -Label 'Exact-source owner/security preflight' -Command 'powershell.exe' -Arguments @(
        '-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'Bypass',
        '-File', (Join-Path $PSScriptRoot 'codexdd-045-owner-preflight.ps1'), '-Mode', 'Source')
    Invoke-CodexDDChecked -Label 'Seven-program Windows distribution contract' -Command $python -Arguments @(
        '-B', 'scripts/codexdd_045_distribution_gate.py', '--source-only')
    foreach ($name in @('test_codexdd_045_3f50_scoped_owner_review.py', 'test_codexdd_045_3f50_distribution_gate.py')) {
        Invoke-CodexDDChecked -Label $name -Command $python -Arguments @(
            '-B', '-m', 'unittest', 'discover', '-s', '.github/scripts', '-p', $name, '-q')
    }
    if ($Mode -eq 'Preflight') {
        Write-Host 'CODEXDD_045_LOCAL_PREFLIGHT=PASS_NONRELEASE_ONLY'
        Write-Host 'FULL_21_STAGE_RELEASE_PROFILE=NOT_EXECUTED'
        $exitCode = 0
    } else {
        $matrix = Get-Content (Join-Path $root 'docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json') -Raw | ConvertFrom-Json
        $rows = @($matrix.paths)
        if ($rows.Count -ne 109) { throw 'Expected exactly 109 owner-path reviews' }
        $pending = @($rows | Where-Object {
            $_.semantic_status -ne 'OWNER_ACCEPTED' -or $_.release_status -ne 'NOT_ACCEPTED'
        })
        if ($pending.Count -ne 0) {
            throw ("Final profile BLOCKED: {0}/109 upstream semantic owners not individually accepted" -f $pending.Count)
        }
        $target = if ($LogPath) { $LogPath } else {
            Join-Path $env:TEMP ('codexdd-045-final-21-' + $head + '.log')
        }
        $target = [IO.Path]::GetFullPath($target)
        $parent = Split-Path -Parent $target
        if (-not (Test-Path -LiteralPath $parent)) {
            New-Item -ItemType Directory -Path $parent -Force | Out-Null
        }
        if ($target.StartsWith($root.TrimEnd('\') + '\', [StringComparison]::OrdinalIgnoreCase)) {
            throw 'Validation log must be outside candidate Git worktree'
        }
        $writer = [IO.StreamWriter]::new($target, $false, ([Text.UTF8Encoding]::new($false)))
        $events = New-Object 'System.Collections.Generic.List[object]'
        $old = $ErrorActionPreference
        try {
            $ErrorActionPreference = 'Continue'
            & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'codexdd-test-release.ps1') 2>&1 |
                ForEach-Object {
                    $line = [string]$_
                    $writer.WriteLine($line)
                    Write-Host $line
                    if ($line.StartsWith('CODEXDD_VALIDATION_JSON ')) {
                        $evt = $line.Substring('CODEXDD_VALIDATION_JSON '.Length) | ConvertFrom-Json
                        $events.Add($evt)
                    }
                }
            $nativeExit = $LASTEXITCODE
        } finally {
            $ErrorActionPreference = $old
            $writer.Dispose()
        }
        $ends = @($events | Where-Object { $_.event -eq 'profile_end' })
        $begins = @($events | Where-Object { $_.event -eq 'profile_begin' })
        $passed = @($events | Where-Object { $_.event -eq 'stage_end' -and $_.status -eq 'pass' })
        if ($nativeExit -ne 0 -or $begins.Count -ne 1 -or $ends.Count -ne 1 -or
            $begins[0].stage_count -ne 21 -or $ends[0].profile -ne 'release' -or
            $ends[0].status -ne 'pass' -or $ends[0].exit_code -ne 0 -or
            $ends[0].completed_stages -ne 21 -or $passed.Count -ne 21 -or
            @($passed | Select-Object -ExpandProperty stage -Unique).Count -ne 21) {
            throw 'Native 21-stage release profile failed or returned incomplete terminal evidence'
        }
        $after = (& git rev-parse HEAD | Out-String).Trim()
        $dirty = (& git status --porcelain --untracked-files=no | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $after -cne $head -or $dirty) {
            throw 'Candidate source changed during native release validation'
        }
        $receipt = [ordered]@{
            contract_version = 1
            kind = 'codexdd_045_21_stage_native_candidate_receipt'
            candidate_sha = $head
            source_host = $env:COMPUTERNAME
            completed_stages = 21
            result = 'NATIVE_RELEASE_PROFILE_PASS_NOT_PRODUCT_APPROVAL'
            release_approved = $false
            combined_distribution_verified = $false
            log_file = $target
            finished_utc = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ')
        }
        $receiptFile = $target + '.receipt.json'
        [IO.File]::WriteAllText($receiptFile, ($receipt | ConvertTo-Json -Depth 5), ([Text.UTF8Encoding]::new($false)))
        Write-Host ('CODEXDD_045_NATIVE_21_STAGE=PASS_NONRELEASE; receipt=' + $receiptFile)
        $exitCode = 0
    }
} catch {
    Write-Host ('CODEXDD_045_NATIVE_GATE=BLOCKED: ' + $_.Exception.Message)
    $exitCode = 1
} finally {
    Pop-Location
    $watch.Stop()
    Write-Host ('TOTAL_TIME_WORKED_SECONDS: {0:N1}' -f $watch.Elapsed.TotalSeconds)
    Write-Host ('COMMIT: ' + $head)
    Write-Host ('DONE_UTC: ' + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ'))
}
exit $exitCode
