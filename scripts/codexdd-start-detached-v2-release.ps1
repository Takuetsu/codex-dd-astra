#Requires -Version 5.1
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$CandidateCheckout,
    [Parameter(Mandatory = $true)][ValidatePattern('^[0-9a-fA-F]{40}$')][string]$ExpectedCandidateSha,
    [Parameter(Mandatory = $true)][ValidatePattern('^[0-9a-fA-F]{64}$')][string]$ApprovedDiscoveryManifestSha256,
    [Parameter(Mandatory = $true)][ValidatePattern('^[0-9a-fA-F]{64}$')][string]$ApprovedConflictsSha256,
    [string]$RunRoot = 'E:\codexdd-validation-runs'
)
# Manual one-shot, SSH-independent release launcher. Not a release PASS.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if ($env:OS -ne 'Windows_NT' -or $env:COMPUTERNAME -ine 'Daniel-CL') {
    throw 'Native v2 detached release must run on Daniel-CL Windows.'
}
function Get-UsableApplication([string]$name) {
    foreach ($app in @(Get-Command $name -All -CommandType Application -ErrorAction Stop)) {
        $path = [string]$app.Source
        if ($path -and $path -notmatch '(?i)[\\/]Microsoft[\\/]WindowsApps[\\/]python(?:\.exe)?$' -and (Test-Path -LiteralPath $path -PathType Leaf)) {
            return $path
        }
    }
    throw "No usable application found: $name"
}
$CandidateCheckout = [IO.Path]::GetFullPath($CandidateCheckout)
$git = Get-UsableApplication 'git.exe'
$cargo = Get-UsableApplication 'cargo.exe'
$python = Get-UsableApplication 'python.exe'
$psExe = Join-Path $env:SystemRoot 'System32\WindowsPowerShell\v1.0\powershell.exe'
$head = (& $git -C $CandidateCheckout rev-parse HEAD).Trim()
$branch = (& $git -C $CandidateCheckout branch --show-current).Trim()
$dirty = @(& $git -C $CandidateCheckout status --porcelain)
if ($LASTEXITCODE -ne 0 -or $head -cne $ExpectedCandidateSha.ToLowerInvariant() -or $dirty.Count -ne 0) {
    throw 'Candidate HEAD differs from expected SHA or working tree is dirty.'
}
$manifestPath = Join-Path $CandidateCheckout 'docs\upstream-candidates\codexdd-upstream-rust-v0.162.0.json'
$validator = Join-Path $CandidateCheckout 'scripts\codexdd-validate-upstream-candidate-v2.ps1'
if (!(Test-Path -LiteralPath $manifestPath) -or !(Test-Path -LiteralPath $validator)) { throw 'Missing pinned v2 validation source.' }
$m = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json -ErrorAction Stop
$disc = $ApprovedDiscoveryManifestSha256.ToLowerInvariant()
$conf = $ApprovedConflictsSha256.ToLowerInvariant()
if ($m.contract_version -ne 2 -or $m.state -cne 'manual_conflict_reconciled_review_required' -or
    $branch -cne $m.candidate_branch -or $m.discovery_manifest_sha256 -cne $disc -or
    $m.discovery_conflicts_sha256 -cne $conf) { throw 'V2 manifest/branch/discovery identity mismatch.' }
$manifestHash = (Get-FileHash -LiteralPath $manifestPath -Algorithm SHA256).Hash.ToLowerInvariant()
$sidecar = Join-Path $CandidateCheckout ($m.sidecar_path.Replace('/', '\'))
$origManifest = Join-Path $CandidateCheckout 'docs\upstream-candidates\original-blocked-v1-manifest.json'
$origConflicts = Join-Path $CandidateCheckout 'docs\upstream-candidates\original-blocked-conflicts.txt'
if ((Get-FileHash -LiteralPath $sidecar -Algorithm SHA256).Hash.ToLowerInvariant() -cne $m.sidecar_sha256 -or
    (Get-FileHash -LiteralPath $origManifest -Algorithm SHA256).Hash.ToLowerInvariant() -cne $disc -or
    (Get-FileHash -LiteralPath $origConflicts -Algorithm SHA256).Hash.ToLowerInvariant() -cne $conf) {
    throw 'Candidate sidecar or original discovery bytes differ from approved digests.'
}
$active = @(Get-ScheduledTask -TaskName 'CodexDD-043-*-release' -ErrorAction SilentlyContinue | Where-Object { $_.State -eq 'Running' })
if ($active.Count -ne 0) { throw 'Another scheduled CodexDD 0.4.3 release is running.' }
$id = [DateTime]::UtcNow.ToString('yyyyMMdd-HHmmss') + '-' + [guid]::NewGuid().ToString('N').Substring(0,8)
$runDir = Join-Path $RunRoot (Join-Path $head $id)
New-Item -ItemType Directory -Path $runDir -Force | Out-Null
$runDir = [IO.Path]::GetFullPath($runDir)
# A nested <runDir>\task-temp can exceed Windows AF_UNIX SUN_LEN.
# Use a distinct short local directory for tempfile-backed test sockets.
$volumeRoot = [IO.Path]::GetPathRoot($runDir)
if ($volumeRoot -cnotmatch '^[A-Za-z]:\\$') {
    throw 'Detached validation requires a local drive for short S4U scratch.'
}
$scratchPath = Join-Path $volumeRoot (Join-Path 'ddtmp' $id)
if ($scratchPath.Length -gt 60 -or (Test-Path -LiteralPath $scratchPath)) {
    throw 'Unique short S4U scratch path unavailable or exceeds socket length budget.'
}
# Allocate outside the candidate checkout; the S4U child must separately
# prove its own create/write/delete access before running any tests.
New-Item -ItemType Directory -Path $scratchPath -ErrorAction Stop | Out-Null
$planPath = Join-Path $runDir 'plan.json'
$runnerPath = Join-Path $runDir 'runner.ps1'
$user = [Security.Principal.WindowsIdentity]::GetCurrent().Name
$pathPrefix = (@($git,$cargo,$python) | ForEach-Object { Split-Path -Parent $_ } | Select-Object -Unique) -join ';'
$plan = [ordered]@{
    Checkout=$CandidateCheckout; Sha=$head; Branch=$branch; SourceSha=[string]$m.source_snapshot_commit_sha
    ProductionSha=[string]$m.production_sha; Manifest=$manifestPath; ManifestSha256=$manifestHash
    DiscoverySha256=$disc; ConflictsSha256=$conf; Git=$git; Cargo=$cargo; Python=$python
    PowerShell=$psExe; PathPrefix=$pathPrefix; CargoTargetDir=[string]([Environment]::GetEnvironmentVariable('CARGO_TARGET_DIR'))
    User=$user; RunDir=$runDir; ShortTempDir=$scratchPath
}
$utf8 = New-Object Text.UTF8Encoding($false)
[IO.File]::WriteAllText($planPath, (ConvertTo-Json -InputObject $plan -Depth 8), $utf8)
# A separate local script owns the child process; nothing runs under the SSH session.
$runner = @'
#Requires -Version 5.1
[CmdletBinding()]
param([Parameter(Mandatory=$true)][string]$PlanPath,
      [Parameter(Mandatory=$true)][ValidateSet('Smoke','Release')][string]$Mode)
Set-StrictMode -Version Latest
$ErrorActionPreference='Stop'
$exitCode=2
$status='fail'
$detail='Runner did not finish'
$failedStage=$null
$receiptCopy=$null
$started=[DateTime]::UtcNow.ToString('o')
$cfg=$null
try {
    $cfg=Get-Content -LiteralPath $PlanPath -Raw | ConvertFrom-Json -ErrorAction Stop
    $env:PATH=[string]$cfg.PathPrefix+';'+$env:PATH
    $env:CARGO_BUILD_JOBS='2'
    $env:RUST_MIN_STACK='16777216'
    $env:NEXTEST_PROFILE='local'
    if (![string]::IsNullOrWhiteSpace([string]$cfg.CargoTargetDir)) {
        $env:CARGO_TARGET_DIR=[string]$cfg.CargoTargetDir
    }
    if ($env:OS -ne 'Windows_NT' -or $env:COMPUTERNAME -ine 'Daniel-CL' -or
        [Security.Principal.WindowsIdentity]::GetCurrent().Name -cne $cfg.User) { throw 'Scheduled task account or host mismatch.' }
    $head=(& $cfg.Git -C $cfg.Checkout rev-parse HEAD).Trim()
    $branch=(& $cfg.Git -C $cfg.Checkout branch --show-current).Trim()
    $dirty=@(& $cfg.Git -C $cfg.Checkout status --porcelain)
    if ($LASTEXITCODE -ne 0 -or $head -cne $cfg.Sha -or $branch -cne $cfg.Branch -or $dirty.Count -ne 0) {
        throw 'Candidate changed or is dirty under Scheduled Task identity.'
    }
    $m=Get-Content -LiteralPath $cfg.Manifest -Raw | ConvertFrom-Json -ErrorAction Stop
    if ((Get-FileHash -LiteralPath $cfg.Manifest -Algorithm SHA256).Hash.ToLowerInvariant() -cne $cfg.ManifestSha256 -or
        $m.source_snapshot_commit_sha -cne $cfg.SourceSha -or $m.production_sha -cne $cfg.ProductionSha -or
        $m.discovery_manifest_sha256 -cne $cfg.DiscoverySha256 -or $m.discovery_conflicts_sha256 -cne $cfg.ConflictsSha256) {
        throw 'Candidate provenance moved under Scheduled Task account.'
    }
    & $cfg.Python --version | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Python not usable in task account.' }
    & $cfg.Cargo --version | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Cargo not usable in task account.' }
    & $cfg.Cargo nextest --version | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Cargo nextest not usable in task account.' }
    Push-Location -LiteralPath (Join-Path $cfg.Checkout 'codex-rs')
    try {
        $meta=(& $cfg.Cargo metadata --no-deps --format-version 1) | ConvertFrom-Json -ErrorAction Stop
        if ($LASTEXITCODE -ne 0) { throw 'Cargo metadata failed.' }
        $helper=Join-Path $meta.target_directory 'debug\test_stdio_server.exe'
        if (!(Test-Path -LiteralPath $helper -PathType Leaf)) { throw "MCP test helper missing in task context: $helper" }
    } finally { Pop-Location }
    # S4U's inherited profile TEMP caused SQLite permission failures, while
    # <runDir>\task-temp exceeded Windows AF_UNIX SUN_LEN for a TUI test.
    # Bind BOTH smoke and release to one short, per-attempt local temp path.
    $scratch=[IO.Path]::GetFullPath([string]$cfg.ShortTempDir)
    $expectedScratch=Join-Path ([IO.Path]::GetPathRoot([string]$cfg.RunDir)) (Join-Path 'ddtmp' (Split-Path -Leaf ([string]$cfg.RunDir)))
    if ($scratch -cne $expectedScratch -or $scratch.Length -gt 60) {
        throw 'Short S4U scratch identity or socket length budget mismatch.'
    }
    [IO.Directory]::CreateDirectory($scratch) | Out-Null
    $canary=Join-Path $scratch 'write-probe.tmp'
    [IO.File]::WriteAllText($canary, 'CodexDD SQLite scratch path preflight')
    [IO.File]::Delete($canary)
    $env:TEMP=$scratch
    $env:TMP=$scratch
    if ($Mode -eq 'Smoke') {
        # This native gate also runs ConPTY-backed CLI integration tests.
        # Checking tool versions and the MCP helper alone cannot prove an S4U
        # task can drive an interactive Windows terminal. Probe exactly one
        # existing startup test BEFORE spending ~40 minutes on the release.
        $ptyLog=Join-Path $cfg.RunDir 'cli-pty-smoke.log'
        $savedPreference=$ErrorActionPreference
        Push-Location -LiteralPath (Join-Path $cfg.Checkout 'codex-rs')
        try {
            try {
                # PowerShell 5.1 maps native stderr into ErrorRecord objects.
                $ErrorActionPreference='Continue'
                & $cfg.Cargo nextest run --no-fail-fast -p codex-cli --test daemon_startup -E 'test(daemon_exclusion_survives_resume_picker)' --no-capture *> $ptyLog
                $ptyExit=$LASTEXITCODE
            } finally {
                $ErrorActionPreference=$savedPreference
            }
        } finally {
            Pop-Location
        }
        $ptyResult=Get-Content -LiteralPath $ptyLog -Raw -ErrorAction Stop
        if ($ptyExit -ne 0 -or $ptyResult -notmatch '1 passed') {
            throw "CLI ConPTY smoke failed in S4U task (exit $ptyExit); review $ptyLog. Do not launch noninteractive full release."
        }
        # Stage 19 failed on a too-long AF_UNIX socket path. Exercise the
        # exact 15-test release-stage selection under S4U in the short TEMP.
        $tuiLog=Join-Path $cfg.RunDir 'tui-daemon-smoke.log'
        $savedPreference=$ErrorActionPreference
        Push-Location -LiteralPath (Join-Path $cfg.Checkout 'codex-rs')
        try {
            try {
                $ErrorActionPreference='Continue'
                & $cfg.Cargo nextest run --no-fail-fast -p codex-tui --lib daemon_startup_tests *> $tuiLog
                $tuiExit=$LASTEXITCODE
            } finally {
                $ErrorActionPreference=$savedPreference
            }
        } finally {
            Pop-Location
        }
        $tuiResult=Get-Content -LiteralPath $tuiLog -Raw -ErrorAction Stop
        if ($tuiExit -ne 0 -or $tuiResult -notmatch '15 tests run:\s+15 passed,\s+0 skipped') {
            throw "TUI daemon S4U socket smoke failed (exit $tuiExit); review $tuiLog. Do not launch the 21-stage release."
        }
        # Oct-12 r3 failed at stage 8 because a batch-logon thread-store
        # test could not open state_5.sqlite in a user-profile temp directory.
        # A complete stage-8 S4U test, with the exact same target + options
        # as the release profile, is a bounded (< 21 stages) environment gate.
        # Compilation may be needed despite a warm target cache. The launcher
        # allows up to 15 minutes for the probe to finish, without treating
        # a probe timeout or launch acknowledgement as release success.
        $storeLog=Join-Path $cfg.RunDir 'thread-store-smoke.log'
        $savedPreference=$ErrorActionPreference
        Push-Location -LiteralPath (Join-Path $cfg.Checkout 'codex-rs')
        try {
            try {
                $ErrorActionPreference='Continue'
                & $cfg.Cargo nextest run --no-fail-fast -p codex-thread-store --lib *> $storeLog
                $storeExit=$LASTEXITCODE
            } finally {
                $ErrorActionPreference=$savedPreference
            }
        } finally {
            Pop-Location
        }
        $storeResult=Get-Content -LiteralPath $storeLog -Raw -ErrorAction Stop
        if ($storeExit -ne 0 -or $storeResult -notmatch '268 tests run:\s+268 passed,\s+0 skipped') {
            throw "Thread-store S4U SQLite smoke failed (exit $storeExit); review $storeLog. Do not launch the 21-stage release."
        }
        $status='pass'
        $exitCode=0
        $detail='Scheduled account, CLI ConPTY, 15/15 TUI socket and 268/268 thread-store SQLite smokes PASS; no full release stages run'
    } else {
        $validator=Join-Path $cfg.Checkout 'scripts\codexdd-validate-upstream-candidate-v2.ps1'
        $releaseArguments='-NoProfile -NonInteractive -ExecutionPolicy Bypass -File "'+$validator+'" -ManifestPath "'+
            $cfg.Manifest+'" -ApprovedDiscoveryManifestSha256 '+$cfg.DiscoverySha256+
            ' -ApprovedConflictsSha256 '+$cfg.ConflictsSha256
        $stdout=Join-Path $cfg.RunDir 'release-stdout.log'
        $stderr=Join-Path $cfg.RunDir 'release-stderr.log'
        $child=Start-Process -FilePath $cfg.PowerShell -ArgumentList $releaseArguments -PassThru -RedirectStandardOutput $stdout -RedirectStandardError $stderr
        $child.WaitForExit()
        $exitCode=[int]$child.ExitCode
        $lines=@(Get-Content -LiteralPath $stdout)
        $failure=@($lines | Where-Object { $_.StartsWith('CODEXDD_VALIDATION_JSON ') -and
            $_.Contains('"event":"stage_end"') -and
            ($_.Contains('"status":"fail"') -or $_.Contains('"status":"error"')) } | Select-Object -Last 1)
        if ($failure.Count -eq 1) {
            $event=$failure[0].Substring('CODEXDD_VALIDATION_JSON '.Length) | ConvertFrom-Json -ErrorAction Stop
            $failedStage=[string]$event.stage
            # A stage_end FAIL is authoritative even if an intermediate Windows
            # PowerShell process unexpectedly reports exit code 0. Never bury
            # a real failing test behind the missing-receipt fallback.
            if ($exitCode -eq 0) { $exitCode=2 }
            $detail="Native release stage '$failedStage' reported $($event.status); see release-stdout.log and release-stderr.log"
        } elseif ($exitCode -ne 0) {
            $detail="Native release validator failed with exit $exitCode"
        } else {
            $end=@($lines | Where-Object { $_.StartsWith('CODEXDD_VALIDATION_JSON ') -and $_.Contains('"event":"profile_end"') })
            $paths=@($lines | Where-Object { $_.StartsWith('CODEXDD_UPSTREAM_VALIDATION_RECEIPT_PATH ') })
            if ($end.Count -ne 1 -or $paths.Count -ne 1) { throw 'Release terminal or receipt path missing/ambiguous.' }
            $profileEvent=$end[0].Substring('CODEXDD_VALIDATION_JSON '.Length) | ConvertFrom-Json -ErrorAction Stop
            if ($profileEvent.profile -cne 'release' -or $profileEvent.status -cne 'pass' -or
                $profileEvent.exit_code -ne 0 -or $profileEvent.completed_stages -lt 21) { throw 'Incomplete release profile.' }
            $path=$paths[0].Substring('CODEXDD_UPSTREAM_VALIDATION_RECEIPT_PATH '.Length).Trim()
            $receipt=Get-Content -LiteralPath $path -Raw | ConvertFrom-Json -ErrorAction Stop
            if ($receipt.candidate_head_sha -cne $cfg.Sha -or $receipt.source_snapshot_commit_sha -cne $cfg.SourceSha -or
                $receipt.production_sha -cne $cfg.ProductionSha -or $receipt.validation_status -cne 'pass' -or
                $receipt.completed_stages -lt 21) { throw 'Candidate-bound native receipt invalid.' }
            $receiptCopy=Join-Path $cfg.RunDir 'native-release-receipt.json'
            Copy-Item -LiteralPath $path -Destination $receiptCopy -ErrorAction Stop
            $status='pass'
            $detail='Full 21-stage native candidate receipt verified'
        }
    }
}
catch {
    $status='fail'
    if ($exitCode -eq 0) { $exitCode=2 }
    $detail=$_.Exception.Message
}
finally {
    if ($null -ne $cfg) {
        $result=[ordered]@{
            candidate_sha=[string]$cfg.Sha; mode=$Mode; status=$status; exit_code=$exitCode
            first_failed_stage=$failedStage; detail=$detail; receipt_path=$receiptCopy
            started_at_utc=$started; finished_at_utc=[DateTime]::UtcNow.ToString('o')
        }
        $out=Join-Path $cfg.RunDir ($Mode.ToLowerInvariant()+'-result.json')
        $tmp=$out+'.tmp'
        [IO.File]::WriteAllText($tmp, (ConvertTo-Json -InputObject $result -Depth 8), (New-Object Text.UTF8Encoding($false)))
        Move-Item -LiteralPath $tmp -Destination $out -Force
    }
}
exit $exitCode
'@
[IO.File]::WriteAllText($runnerPath, $runner, $utf8)
$principal=New-ScheduledTaskPrincipal -UserId $user -LogonType S4U -RunLevel Limited
$settings=New-ScheduledTaskSettingsSet -MultipleInstances IgnoreNew -ExecutionTimeLimit (New-TimeSpan -Hours 8)
$taskPrefix='CodexDD-043-'+$head.Substring(0,12)+'-'+$id
$probeName=$taskPrefix+'-probe'
$releaseName=$taskPrefix+'-release'
function New-TaskAction([string]$mode) {
    $taskArguments='-NoProfile -NonInteractive -ExecutionPolicy Bypass -File "'+$runnerPath+'" -PlanPath "'+$planPath+'" -Mode '+$mode
    return (New-ScheduledTaskAction -Execute $psExe -Argument $taskArguments -WorkingDirectory $CandidateCheckout)
}
foreach ($task in @($probeName,$releaseName)) {
    if (Get-ScheduledTask -TaskName $task -ErrorAction SilentlyContinue) { throw "Task already exists: $task" }
}
Register-ScheduledTask -TaskName $probeName -Action (New-TaskAction 'Smoke') -Principal $principal -Settings $settings -Description 'One-shot CodexDD v2 S4U CLI/TUI/SQLite preflight' | Out-Null
Start-ScheduledTask -TaskName $probeName
$probeFile=Join-Path $runDir 'smoke-result.json'
$timeout=[DateTime]::UtcNow.AddMinutes(15)
while (!(Test-Path -LiteralPath $probeFile -PathType Leaf) -and [DateTime]::UtcNow -lt $timeout) {
    Start-Sleep -Seconds 1
}
if (!(Test-Path -LiteralPath $probeFile -PathType Leaf)) {
    $state=Get-ScheduledTaskInfo -TaskName $probeName
    throw "Scheduled Task smoke timed out. LastTaskResult=$($state.LastTaskResult); inspect $runDir"
}
$smoke=Get-Content -LiteralPath $probeFile -Raw | ConvertFrom-Json -ErrorAction Stop
if ($smoke.status -cne 'pass' -or $smoke.exit_code -ne 0) {
    $storeLog=Join-Path $runDir 'thread-store-smoke.log'
    $tuiLog=Join-Path $runDir 'tui-daemon-smoke.log'
    $ptyLog=Join-Path $runDir 'cli-pty-smoke.log'
    $failureLog=if (Test-Path -LiteralPath $storeLog -PathType Leaf) { $storeLog }
        elseif (Test-Path -LiteralPath $tuiLog -PathType Leaf) { $tuiLog }
        elseif (Test-Path -LiteralPath $ptyLog -PathType Leaf) { $ptyLog } else { $null }
    if ($null -ne $failureLog) {
        Write-Host ('CODEXDD_S4U_SMOKE_LOG: '+$failureLog)
        Get-Content -LiteralPath $failureLog -Tail 45
    }
    throw "Scheduled Task smoke failed: $($smoke.detail); inspect $runDir"
}
Unregister-ScheduledTask -TaskName $probeName -Confirm:$false
Register-ScheduledTask -TaskName $releaseName -Action (New-TaskAction 'Release') -Principal $principal -Settings $settings -Description ('One-shot CodexDD v2 native 21-stage release for '+$head) | Out-Null
Start-ScheduledTask -TaskName $releaseName
Write-Host 'CODEXDD_DETACHED_RELEASE_STARTED (not a validation PASS)'
Write-Host ('Task name: '+$releaseName)
Write-Host ('Candidate: '+$head)
Write-Host ('Run directory: '+$runDir)
Write-Host ('Short S4U TEMP/TMP: '+$scratchPath)
Write-Host ('Status: Get-ScheduledTask -TaskName "'+$releaseName+'"')
Write-Host ('Result: '+(Join-Path $runDir 'release-result.json'))
Write-Host ('Log: '+(Join-Path $runDir 'release-stdout.log'))
