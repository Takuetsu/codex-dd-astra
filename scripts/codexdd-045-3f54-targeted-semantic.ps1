# CodexDD 0.4.5 3F.54 — targeted, source-pinned native semantic validation.
# No checkout, edits, CI trigger, release build, installer or version promotion.
[CmdletBinding()]
param(
    [ValidateSet('Preflight', 'Native')][string]$Mode = 'Preflight',
    [string]$ExpectedCommit = ''
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'codexdd-python-executable.ps1')
$root = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
$started = [Diagnostics.Stopwatch]::StartNew()
$head = ''
$results = @()
$exitCode = 1

function Invoke-FocusedStage {
    param([string]$Name, [string]$Command, [string[]]$Arguments, [string]$Directory)
    Write-Host ("BEGIN_STAGE: " + $Name)
    Push-Location -LiteralPath $Directory
    $old = $ErrorActionPreference
    $clock = [Diagnostics.Stopwatch]::StartNew()
    try {
        $ErrorActionPreference = 'Continue'
        & $Command @Arguments 2>&1 | ForEach-Object { Write-Host ([string]$_) }
        $status = [int]$LASTEXITCODE
    } finally {
        $ErrorActionPreference = $old
        Pop-Location
        $clock.Stop()
    }
    $script:results += @{
        name = $Name
        result = if ($status -eq 0) { 'PASS' } else { 'FAIL' }
        seconds = [math]::Round($clock.Elapsed.TotalSeconds, 1)
    }
    if ($status -ne 0) { throw ("Stage failed: " + $Name + " (exit " + $status + ")") }
    Write-Host ("PASS_STAGE: " + $Name)
}

Push-Location -LiteralPath $root
try {
    if ($env:OS -ne 'Windows_NT' -or $env:COMPUTERNAME -ine 'Daniel-CL') {
        throw 'Run only on Daniel-CL Windows inside a candidate Git worktree'
    }
    $head = (& git rev-parse HEAD | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $head -cnotmatch '^[0-9a-f]{40}$') {
        throw 'Candidate HEAD must be one exact 40-character SHA'
    }
    if ($Mode -eq 'Native') {
        if ($ExpectedCommit -cnotmatch '^[0-9a-f]{40}$' -or $head -cne $ExpectedCommit) {
            throw 'Native mode requires -ExpectedCommit matching the full current HEAD SHA'
        }
    } elseif ($ExpectedCommit -and $head -cne $ExpectedCommit) {
        throw 'Preflight expected SHA mismatch'
    }
    $dirty = (& git status --porcelain --untracked-files=no | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $dirty) { throw 'Source worktree contains tracked modifications' }
    $env:RUST_MIN_STACK = '16777216'
    $env:NEXTEST_PROFILE = 'local'
    $env:CARGO_INCREMENTAL = '1'
    $env:PYTHONDONTWRITEBYTECODE = '1'
    $python = Resolve-CodexDDPythonPath -Candidates @(Get-Command python -All -ErrorAction SilentlyContinue)
    $version = (& $python --version | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $version -notmatch '^Python 3\.(1[1-9]|[2-9][0-9])\.') {
        throw ("Python 3.11+ required: " + $version)
    }
    Write-Host ('SOURCE_HEAD: ' + $head)
    $rs = Join-Path $root 'codex-rs'
    Invoke-FocusedStage 'locked-owner-source-preflight' 'powershell.exe' @(
        '-NoProfile','-NonInteractive','-ExecutionPolicy','Bypass','-File',
        (Join-Path $PSScriptRoot 'codexdd-045-owner-preflight.ps1'),'-Mode','Source'
    ) $root
    foreach ($name in @(
        'test_codexdd_045_3f51_exact_upstream_semantic_deltas.py',
        'test_codexdd_045_3f51_generated_protocol_parity.py',
        'test_codexdd_045_3f51_appserver_event_semantics.py',
        'test_codexdd_045_3f52_appserver_owner_source.py'
    )) {
        Invoke-FocusedStage $name $python @(
            '-B','-m','unittest','discover','-s','.github/scripts','-p',$name,'-q'
        ) $root
    }
    if ($Mode -eq 'Native') {
        Invoke-FocusedStage 'locked-cargo-metadata' 'cargo' @('metadata','--locked','--no-deps','--format-version','1') $rs
        Invoke-FocusedStage 'cargo-format' 'cargo' @('fmt','--all','--','--check') $rs

        $tests = @(
            @('core-custom-tool-interrupted-output', 'codex-core', 'normalize_adds_missing_output_for_custom_tool_call'),
            @('core-local-shell-interrupted-output', 'codex-core', 'normalize_adds_missing_output_for_local_shell_call_with_id'),
            @('core-orphan-output-integrity', 'codex-core', 'normalize_removes_orphan_'),
            @('app-server-source-turn-signal', 'codex-app-server', 'adaptive_runtime_signal'),
            @('app-server-failed-terminal', 'codex-app-server', 'test_handle_turn_complete_emits_failed_with_error'),
            @('tui-daemon-mcp-stack-and-authorization', 'codex-tui', 'local_daemon_registers_approval_gated_mcp_tools_for_both_start_paths'),
            @('tui-legacy-server-negotiation', 'codex-tui', 'older_external_server_starts_without_unsupported_dynamic_tools_or_history'),
            @('tui-server-owned-startup-defaults', 'codex-tui', 'fresh_startup_uses_server_defaults_with_explicit_and_managed_precedence'),
            @('tui-remote-workspace-roots', 'codex-tui', 'remote_workspace_roots_survive_start_turn_resume_and_fork')
        )
        foreach ($test in $tests) {
            $args = @('nextest','run','--no-fail-fast','-p',$test[1],'--lib',$test[2])
            Invoke-FocusedStage $test[0] 'cargo' $args $rs
        }
    } else {
        Write-Host 'NATIVE_CARGO_TESTS=NOT_RUN_PREFLIGHT_ONLY'
    }
    $after = (& git rev-parse HEAD | Out-String).Trim()
    $dirtyAfter = (& git status --porcelain --untracked-files=no | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $after -cne $head -or $dirtyAfter) {
        throw 'Candidate tracked source changed during validation'
    }
    $exitCode = 0
    Write-Host ('CODEXDD_045_3F54=' + $(if ($Mode -eq 'Native') {'NATIVE_TARGETED_GREEN_NONRELEASE'} else {'PREFLIGHT_GREEN_NONRELEASE'}))
} catch {
    Write-Host ('CODEXDD_045_3F54=RED: ' + $_.Exception.Message)
    $exitCode = 1
} finally {
    Pop-Location
    $started.Stop()
    $receipt = [ordered]@{
        contract_version = 1
        kind = 'codexdd_045_3f54_targeted_semantic_receipt'
        source_sha = $head
        host = $env:COMPUTERNAME
        mode = $Mode
        result = if ($exitCode -eq 0) { 'PASS_NONRELEASE' } else { 'FAIL' }
        release_approved = $false
        owner_paths_accepted = 0
        native_tests_executed = ($Mode -eq 'Native')
        stages = @($results)
        elapsed_seconds = [math]::Round($started.Elapsed.TotalSeconds, 1)
        done_utc = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ')
    }
    Write-Host ('CODEXDD_045_3F54_RECEIPT_JSON ' + ($receipt | ConvertTo-Json -Compress -Depth 6))
    Write-Host ('TOTAL_TIME_WORKED_SECONDS: {0:N1}' -f $started.Elapsed.TotalSeconds)
    Write-Host ('DONE_UTC: ' + $receipt.done_utc)
}
exit $exitCode
