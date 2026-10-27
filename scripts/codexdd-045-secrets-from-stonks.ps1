# From Daniel-Stonks, launch ONLY the focused CodexDD 0.4.5 native secrets test.
# Runs on Daniel-CL over SSH, streams its full output to a log on Stonks.
# NONRELEASE: no production switch, integration merge, full suite, or CI.
param(
    [string]$SshHost = 'Daniel-CL',
    [string]$RemoteRepo = 'E:\codexdd',
    [string]$CandidateSha = 'f957370607a31a43fa9ad68a522e8badb25e18e4',
    [string]$LogDirectory = (Join-Path $HOME 'CodexDD-Logs')
)

$ErrorActionPreference = 'Stop'
# This script is the Stonks-side log collector; launching it from an SSH session
# on Daniel-CL otherwise attempts Daniel-CL -> Daniel-CL and fails publickey.
$launcherHost = [Environment]::MachineName
if ($launcherHost -ine 'Daniel-Stonks') {
    throw "Run this launcher on DANIEL-STONKS, not $launcherHost. If inside an SSH session on DANIEL-CL, exit that session first. No native validation was started."
}
if ($CandidateSha -notmatch '^[0-9a-f]{40}$') {
    throw 'CandidateSha must be an exact 40-character commit SHA'
}
New-Item -ItemType Directory -Force -Path $LogDirectory | Out-Null
$log = Join-Path $LogDirectory ("codexdd-045-secrets-" + (Get-Date -Format 'yyyyMMdd-HHmmss') + '.log')
$repoB64 = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($RemoteRepo))

# EncodedCommand keeps both paths and PowerShell quoting intact across SSH.
$remoteScript = @'
$ErrorActionPreference = 'Stop'
$repo = [Text.Encoding]::Unicode.GetString([Convert]::FromBase64String('__REPO_B64__'))
$sha = '__CANDIDATE_SHA__'
$clock = [System.Diagnostics.Stopwatch]::StartNew()
$exitCode = 0
$head = ''
try {
    if (-not (Test-Path -LiteralPath $repo -PathType Container)) {
        throw "Remote repository does not exist: $repo"
    }
    & git -C $repo rev-parse --show-toplevel | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Remote repository is not a Git checkout' }

    # Fetch the exact reviewed candidate; do not update the production checkout.
    & git -C $repo fetch --no-tags origin $sha
    if ($LASTEXITCODE -ne 0) { throw "Cannot fetch candidate $sha" }
    $worktreeParent = Join-Path $repo 'worktrees'
    New-Item -ItemType Directory -Force -Path $worktreeParent | Out-Null
    $worktree = Join-Path $worktreeParent ('codexdd045-secrets-' + $sha.Substring(0, 12))
    if (-not (Test-Path -LiteralPath $worktree)) {
        & git -C $repo worktree add --detach $worktree $sha
        if ($LASTEXITCODE -ne 0) { throw 'Cannot create isolated candidate worktree' }
    }
    $head = (& git -C $worktree rev-parse HEAD | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $head -ne $sha) {
        throw "Worktree identity mismatch: got $head, expected $sha"
    }
    $nativeTest = Join-Path $worktree 'scripts\codexdd-045-native-secrets-local.ps1'
    if (-not (Test-Path -LiteralPath $nativeTest -PathType Leaf)) {
        throw "Native test helper not found: $nativeTest"
    }
    Write-Output "CODEXDD_045_REMOTE: START"
    Write-Output "EXACT_SOURCE: $sha"
    Write-Output "WORKTREE: $worktree"
    & powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File $nativeTest
    if ($LASTEXITCODE -ne 0) { throw "Native secrets check failed (exit $LASTEXITCODE)" }
    Write-Output 'CODEXDD_045_REMOTE: GREEN - focused native secrets test only'
} catch {
    $exitCode = 1
    Write-Output "CODEXDD_045_REMOTE: RED - $($_.Exception.Message)"
} finally {
    $clock.Stop()
    Write-Output ("ELAPSED_SECONDS: {0:N1}" -f $clock.Elapsed.TotalSeconds)
    Write-Output "COMMIT: $head"
    Write-Output ('FINISHED_UTC: ' + [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ'))
}
exit $exitCode
'@
$remoteScript = $remoteScript.Replace('__REPO_B64__', $repoB64).Replace('__CANDIDATE_SHA__', $CandidateSha)
$encoded = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($remoteScript))

Write-Output "Stonks log: $log"
Write-Output "Running focused native secrets validation on $SshHost at $CandidateSha"
# Windows PowerShell 5.1 may promote redirected SSH stderr to error records.
$previousPreference = $ErrorActionPreference
$ErrorActionPreference = 'Continue'
try {
    & ssh -o ServerAliveInterval=30 -o ServerAliveCountMax=8 $SshHost "powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -EncodedCommand $encoded" 2>&1 |
        Tee-Object -FilePath $log
    $sshExit = $LASTEXITCODE
} finally {
    $ErrorActionPreference = $previousPreference
}
Write-Output "LOCAL_LOG: $log"
if ($sshExit -ne 0) { throw "Focused validation failed with SSH exit $sshExit; send the Stonks log" }
Write-Output 'CODEXDD_045_LOCAL: GREEN (native secrets only, NOT release acceptance)'
