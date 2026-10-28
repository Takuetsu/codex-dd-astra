# CodexDD 0.4.5: fail-closed, nonrelease source and optional Windows Bazel graph preflight.
# Run in an isolated worktree on Daniel-CL. Never edits source, installs, or publishes.
[CmdletBinding()]
param(
    [ValidateSet('Source', 'Bazel')]
    [string]$Mode = 'Source'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'codexdd-python-executable.ps1')

$root = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
$started = [System.Diagnostics.Stopwatch]::StartNew()
$head = ''
$exitCode = 1

function Invoke-FocusedCommand {
    [CmdletBinding()]
    param(
        [string]$Stage,
        [string]$FilePath,
        [string[]]$Arguments
    )

    Write-Host ("BEGIN: " + $Stage)
    $saved = $ErrorActionPreference
    try {
        # Native tools legitimately write warnings to stderr on PowerShell 5.1.
        # Treat their exit status, not the stderr stream, as the result.
        $ErrorActionPreference = 'Continue'
        & $FilePath @Arguments 2>&1 | ForEach-Object { Write-Host ([string]$_) }
        $result = $LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $saved
    }
    if ($result -ne 0) {
        throw ("FAILED: " + $Stage + " exit=" + $result)
    }
    Write-Host ("PASS: " + $Stage)
}

Push-Location -LiteralPath $root
try {
    if ($env:OS -ne 'Windows_NT') {
        throw 'This validation is intended for native Windows on Daniel-CL'
    }

    $head = (& git rev-parse HEAD | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or -not $head) {
        throw 'Could not resolve candidate Git HEAD'
    }
    $dirtyBefore = (& git status --porcelain --untracked-files=normal | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $dirtyBefore) {
        throw 'Nonrelease worktree must be clean before validation'
    }
    $expected = @{
        'codex-rs/Cargo.toml' = 'cfebd9b784ef9ff027b274aa9f7101c9c2496613'
        'codex-rs/Cargo.lock' = '13bc15ff844e93dfaf7777c73bb7313b35bb3b81'
        'codex-rs/deny.toml' = 'f282c7c55d05201e308e60804f85754c8522d462'
        'codex-rs/vendor/aws-config-1.13.0' = '5b1db93de9e14ab5c060794d23a567d907b66f21'
    }
    foreach ($path in $expected.Keys) {
        $actual = (& git rev-parse ("HEAD:" + $path) | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $actual -ne $expected[$path]) {
            throw ("Unreviewed protected source: " + $path + " = " + $actual)
        }
    }
    $version = (& git show 'HEAD:codex-rs/codexdd-version.txt' | Out-String).Trim()
    $upstream = (& git show 'HEAD:codex-rs/upstream-codex-release.txt' | Out-String).Trim()
    if ($version -ne '0.4.4' -or $upstream -ne 'rust-v0.162.0') {
        throw 'Unexpected release marker: refusing pre-release validation'
    }
    if ($env:BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA) {
        throw 'External MSVC runtime EULA override is forbidden'
    }
    Write-Host ("CANDIDATE_HEAD: " + $head)
    Write-Host ("MODE: " + $Mode)
    Write-Host 'PASS: pinned secure graph and unchanged production markers'

    $candidates = @(Get-Command python -All -ErrorAction SilentlyContinue)
    $python = Resolve-CodexDDPythonPath -Candidates $candidates
    $pythonVersion = (& $python --version | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $pythonVersion -notmatch '^Python 3\.(1[1-9]|[2-9][0-9])\.') {
        throw ('Python 3.11+ required for tomllib: ' + $pythonVersion)
    }
    $env:PYTHONDONTWRITEBYTECODE = '1'

    $tests = @(
        'test_codexdd_045_3a_manifest_stage.py'
        'test_codexdd_045_3d_lock_acceptance.py'
        'test_codexdd_045_3f1_overlap_snapshot.py'
        'test_codexdd_045_3f2_direct_conflicts.py'
        'test_codexdd_045_3f6_format_provenance.py'
        'test_codexdd_045_3f14_owner_matrix.py'
    )
    foreach ($test in $tests) {
        Invoke-FocusedCommand -Stage $test -FilePath $python -Arguments @(
            '-B', '-m', 'unittest', 'discover', '-s', '.github/scripts',
            '-p', $test, '-q'
        )
    }
    Invoke-FocusedCommand -Stage 'secure-dependency-staging' -FilePath $python -Arguments @(
        '-B', 'scripts/codexdd-045-dependency-security-check.py', '--mode', 'staging'
    )
    Invoke-FocusedCommand -Stage 'inherited-INFO-policy-guard' -FilePath $python -Arguments @(
        '-B', 'scripts/codexdd-045-inherited-risk-guard.py'
    )

    if ($Mode -eq 'Bazel') {
        $bazelCommand = Get-Command bazel -ErrorAction Stop
        $bazelVersion = (& $bazelCommand.Source --version | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $bazelVersion -notmatch 'bazel 9\.2\.0') {
            throw ('Expected Bazel 9.2.0, found ' + $bazelVersion)
        }
        Invoke-FocusedCommand -Stage 'strict Bazel module lock' -FilePath $bazelCommand.Source -Arguments @(
            '--noexperimental_remote_repo_contents_cache', 'mod', 'deps',
            '--lockfile_mode=error'
        )
        $profile = @(
            '--lockfile_mode=error'
            '--config=ci-windows-msvc'
            '--platforms=//:windows_x86_64_msvc'
            '--//codex-rs:runtime_package_build_mode=debug'
            '--output=label'
        )
        Invoke-FocusedCommand -Stage 'Windows MSVC CLI release_files cquery' -FilePath $bazelCommand.Source -Arguments (
            @('--noexperimental_remote_repo_contents_cache', 'cquery') + $profile +
            @('//codex-rs/cli:release_files')
        )
        Invoke-FocusedCommand -Stage 'Windows MSVC combined runtime archive cquery' -FilePath $bazelCommand.Source -Arguments (
            @('--noexperimental_remote_repo_contents_cache', 'cquery') + $profile +
            @('//codex-rs:runtime_package_combined')
        )
        Write-Host 'PASS: Bazel analysis only. NO shipping binaries compiled or packaged.'
    }

    Invoke-FocusedCommand -Stage 'git diff --check' -FilePath 'git' -Arguments @('diff', '--check')
    $dirtyAfter = (& git status --porcelain --untracked-files=normal | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $dirtyAfter) {
        throw ('Validation changed worktree: ' + $dirtyAfter)
    }
    Write-Host 'CODEXDD_045_PREFLIGHT: GREEN (source/graph validation only)'
    $exitCode = 0
}
catch {
    Write-Host ("CODEXDD_045_PREFLIGHT: RED - " + $_.Exception.Message)
    $exitCode = 1
}
finally {
    Pop-Location
    $started.Stop()
    Write-Host ("TOTAL_TIME_WORKED_SECONDS: {0:N1}" -f $started.Elapsed.TotalSeconds)
    Write-Host ("COMMIT: " + $head)
    Write-Host ("DONE_UTC: " + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ'))
}
exit $exitCode
