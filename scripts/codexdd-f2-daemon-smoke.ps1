[CmdletBinding()]
param(
    [switch]$Worker,
    [switch]$Direct,
    [string]$CaseDir,
    [string]$PackageRoot
)

# F2 hotfix acceptance: test a packaged CLI's real Windows daemon lifecycle
# under a Medium-integrity interactive token, never the production CODEX_HOME.
# Only Windows PowerShell 5.1 features are used.
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Write-F2Trace {
    param([string]$Dir, [string]$Message)
    Add-Content -LiteralPath (Join-Path $Dir "trace.txt") -Value $Message -Encoding ASCII
}

function Invoke-F2Daemon {
    param(
        [string]$Exe,
        [string]$Dir,
        [string]$Verb,
        [string]$Label,
        [int]$TimeoutSeconds = 90
    )

    $stdout = Join-Path $Dir "$Label.stdout.txt"
    $stderr = Join-Path $Dir "$Label.stderr.txt"
    # Do not use Start-Process -Wait: managed daemon children can outlive
    # the client, causing PowerShell to wait even after the client exits.
    $process = Start-Process -FilePath $Exe -ArgumentList @("app-server", "daemon", $Verb) -WorkingDirectory $Dir -RedirectStandardOutput $stdout -RedirectStandardError $stderr -WindowStyle Hidden -PassThru
    try {
        if (-not $process.WaitForExit($TimeoutSeconds * 1000)) {
            $process.Kill()
            throw "Daemon '$Verb' timed out after $TimeoutSeconds seconds"
        }

        $process.WaitForExit()
        $errorText = Get-Content -LiteralPath $stderr -Raw -ErrorAction SilentlyContinue
        if ($process.ExitCode -ne 0) {
            throw "Daemon '$Verb' exited $($process.ExitCode): $errorText"
        }

        $text = Get-Content -LiteralPath $stdout -Raw
        if ([string]::IsNullOrWhiteSpace($text)) {
            throw "Daemon '$Verb' returned empty stdout"
        }
        return ($text | ConvertFrom-Json -ErrorAction Stop)
    }
    finally {
        $process.Dispose()
    }
}

function Invoke-F2Worker {
    param([string]$Dir)

    $resultFile = Join-Path $Dir "result.txt"
    $startAttempted = $false
    $passed = $false
    $stoppedCleanly = $false
    $problem = ""
    $isolatedHome = $null
    $exe = $null

    try {
        $config = Get-Content -LiteralPath (Join-Path $Dir "config.json") -Raw | ConvertFrom-Json
        $isolatedHome = [string]$config.home
        $exe = [string]$config.exe

        if (-not (Test-Path -LiteralPath $exe -PathType Leaf)) {
            throw "Packaged candidate executable missing"
        }
        if ((Split-Path -Leaf $isolatedHome) -notmatch '^f2h-[0-9a-f]{8}$') {
            throw "Unexpected temporary CODEX_HOME"
        }

        $token = whoami /groups | Out-String
        $token | Set-Content -LiteralPath (Join-Path $Dir "integrity.txt")
        if ($token -notmatch 'S-1-16-8192' -or $token -match 'S-1-16-12288') {
            throw "Expected Medium-integrity token (S-1-16-8192)"
        }
        Write-F2Trace $Dir "PASS: Medium integrity"

        # An inherited TEMP ACL is too broad for a shared daemon state dir.
        # Create the protected user-only directory AT CREATION TIME from the
        # Medium-integrity worker, matching codex-uds/windows_security.rs.
        if (Test-Path -LiteralPath $isolatedHome) {
            throw "Isolated CODEX_HOME exists; refusing ACL repair"
        }
        $currentSid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value
        $stateAcl = New-Object System.Security.AccessControl.DirectorySecurity
        $stateAcl.SetSecurityDescriptorSddlForm("O:${currentSid}D:P(A;OICI;FA;;;${currentSid})")
        [void][System.IO.Directory]::CreateDirectory($isolatedHome)
        $stateDirectory = Join-Path $isolatedHome "app-server-daemon"
        [void][System.IO.Directory]::CreateDirectory($stateDirectory, $stateAcl)
        $settingsPath = Join-Path $stateDirectory "settings.json"
        '{"updater":{"autoUpdateEnabled":false}}' | Set-Content -LiteralPath $settingsPath -Encoding ASCII
        Write-F2Trace $Dir "PASS: State directory created with user-only protected ACL"
        $settings = Get-Content -LiteralPath $settingsPath -Raw | ConvertFrom-Json
        if ($settings.updater.autoUpdateEnabled -ne $false) {
            throw "Isolated daemon auto-updates are not disabled"
        }
        Write-F2Trace $Dir "PASS: Isolated CODEX_HOME; auto-updates disabled"

        $env:CODEX_HOME = $isolatedHome

        $startAttempted = $true
        $first = Invoke-F2Daemon -Exe $exe -Dir $Dir -Verb "start" -Label "1-start"
        Write-F2Trace $Dir "START: $($first.status)"
        if ($first.status -ne "started") {
            throw "Expected freshly started daemon"
        }

        $again = Invoke-F2Daemon -Exe $exe -Dir $Dir -Verb "start" -Label "2-reconnect"
        Write-F2Trace $Dir "RECONNECT: $($again.status)"
        if ($again.status -ne "alreadyRunning") {
            throw "Expected alreadyRunning on second client"
        }

        $info = Invoke-F2Daemon -Exe $exe -Dir $Dir -Verb "version" -Label "3-version"
        Write-F2Trace $Dir "VERIFY: $($info.status)"
        Write-F2Trace $Dir "APP SERVER: $($info.appServerVersion)"
        if ($info.status -ne "running" -or [string]::IsNullOrWhiteSpace([string]$info.appServerVersion)) {
            throw "Daemon is not responsive"
        }

        $prefix = [System.IO.Path]::GetFullPath($isolatedHome).TrimEnd('\') + '\'
        if (-not ([string]$info.socketPath).StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "Daemon socket is outside isolated CODEX_HOME"
        }
        if (-not ([string]$info.managedCodexPath).StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "Managed daemon path is outside isolated CODEX_HOME"
        }
        Write-F2Trace $Dir "PASS: Socket and managed executable isolated"
        $passed = $true
    }
    catch {
        $problem = $_.Exception.Message
        Write-F2Trace $Dir "ERROR: $problem"
    }
    finally {
        if ($startAttempted) {
            try {
                # Never switch to the user's production profile for cleanup.
                $env:CODEX_HOME = $isolatedHome
                $stop = Invoke-F2Daemon -Exe $exe -Dir $Dir -Verb "stop" -Label "4-stop"
                Write-F2Trace $Dir "STOP: $($stop.status)"
                if ($stop.status -notin @("stopped", "notRunning")) {
                    throw "Unexpected stop status: $($stop.status)"
                }
                $after = Invoke-F2Daemon -Exe $exe -Dir $Dir -Verb "stop" -Label "5-confirm-stopped"
                Write-F2Trace $Dir "FINAL: $($after.status)"
                if ($after.status -ne "notRunning") {
                    throw "Daemon still appears to be running after stop"
                }
                $stoppedCleanly = $true
            }
            catch {
                $problem += " Cleanup failure: $($_.Exception.Message)"
                Write-F2Trace $Dir "CLEANUP ERROR: $($_.Exception.Message)"
            }
        }
        else {
            $stoppedCleanly = $true
        }

        if ($passed -and $stoppedCleanly) {
            Set-Content -LiteralPath $resultFile -Value "PASS: MEDIUM DAEMON START RECONNECT VERIFY STOP" -Encoding ASCII
        }
        else {
            if ([string]::IsNullOrWhiteSpace($problem)) {
                $problem = "Lifecycle assertions or cleanup failed"
            }
            Set-Content -LiteralPath $resultFile -Value "FAIL: $problem" -Encoding ASCII
        }
    }
}

if ($Worker) {
    if ([string]::IsNullOrWhiteSpace($CaseDir) -or -not (Test-Path -LiteralPath $CaseDir -PathType Container)) {
        throw "Worker requires an existing -CaseDir"
    }
    Invoke-F2Worker -Dir $CaseDir
    $resultText = (Get-Content -LiteralPath (Join-Path $CaseDir "result.txt") -Raw).Trim()
    if ($resultText -notlike "PASS:*") {
        exit 1
    }
    exit 0
}

if ($env:OS -ne "Windows_NT") {
    throw "Windows-only smoke test"
}

# A Task Scheduler Job Object can forbid daemon process detachment.
# Run this acceptance check directly from a normal Medium desktop process.
if (-not $Direct) {
    throw "Use -Direct from non-elevated Daniel-CL desktop PowerShell; scheduled task execution is not a valid detached-daemon test."
}
$desktopGroups = whoami /groups | Out-String
if ($desktopGroups -notmatch 'S-1-16-8192' -or $desktopGroups -match 'S-1-16-12288') {
    throw "Expected Medium-integrity Windows desktop PowerShell, not elevated SSH."
}

$repository = Split-Path -Parent $PSScriptRoot
$activeBranch = (& git -C $repository branch --show-current).Trim()
if ($LASTEXITCODE -ne 0 -or $activeBranch -ne "dd/codexdd-f2-elevated-ssh-embedded-startup") {
    throw "Run this smoke test from the F2 hotfix branch"
}
if (& git -C $repository status --porcelain) {
    throw "Git worktree must be clean"
}

$uac = (Get-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System").EnableLUA
if ($uac -ne 1) {
    throw "UAC must be enabled; refusing to relax token security"
}

if ([string]::IsNullOrWhiteSpace($PackageRoot)) {
    $marker = Join-Path $env:TEMP "codexdd-f2-last-smoke-root.txt"
    if (-not (Test-Path -LiteralPath $marker -PathType Leaf)) {
        throw "Package marker missing; supply -PackageRoot or build the standalone package"
    }
    $PackageRoot = (Get-Content -LiteralPath $marker -Raw).Trim()
}

$package = Join-Path $PackageRoot "package"
$exe = Join-Path $package "bin\codex.exe"
foreach ($item in @(
    "codex-package.json",
    "bin\codex.exe",
    "bin\codex-code-mode-host.exe",
    "codex-path\rg.exe",
    "codex-resources\codex-command-runner.exe",
    "codex-resources\codex-windows-sandbox-setup.exe"
)) {
    if (-not (Test-Path -LiteralPath (Join-Path $package $item) -PathType Leaf)) {
        throw "Incomplete standalone package: $item"
    }
}

$version = & $exe --version
$expectedSnapshotVersion = "0.4.2+g2caa1b3afa55"
if ($LASTEXITCODE -ne 0 -or $version -ne "codexdd $expectedSnapshotVersion") {
    throw "Packaged executable is not the validated F2 hotfix candidate: $version"
}

# This debug executable identifies a snapshot, not a stable release.
# A stable "0.4.2" package manifest is deliberately rejected by the daemon
# installer, because it does not match the executable's --version output.
# Normalize ONLY this disposable smoke-test package, never production.
$normalizedRoot = [System.IO.Path]::GetFullPath($PackageRoot).TrimEnd('\')
$normalizedTemp = [System.IO.Path]::GetFullPath($env:TEMP).TrimEnd('\') + '\'
if (-not $normalizedRoot.StartsWith($normalizedTemp, [System.StringComparison]::OrdinalIgnoreCase) -or
    (Split-Path -Leaf $normalizedRoot) -notmatch '^codexdd-f2-final-[0-9a-f]{32}$') {
    throw "Smoke package root is not a disposable CodexDD F2 test directory"
}
$manifestPath = Join-Path $package "codex-package.json"
$manifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
if ($manifest.version -eq "0.4.2") {
    $manifest.version = $expectedSnapshotVersion
    $manifestJson = $manifest | ConvertTo-Json -Depth 10
    $utf8NoBom = [System.Text.UTF8Encoding]::new($false)
    [System.IO.File]::WriteAllText($manifestPath, $manifestJson + "`n", $utf8NoBom)
    Write-Host "Prepared disposable smoke package snapshot $expectedSnapshotVersion"
}
elseif ($manifest.version -ne $expectedSnapshotVersion) {
    throw "Unexpected smoke package version: $($manifest.version)"
}
$manifestCheck = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
if ($manifestCheck.version -ne $expectedSnapshotVersion) {
    throw "Snapshot package metadata does not match the executable"
}

$id = [guid]::NewGuid().ToString("N").Substring(0, 8)
$isolatedHome = Join-Path $env:TEMP "f2h-$id"
$socket = Join-Path $isolatedHome "app-server-control\app-server-control.sock"
if ($socket.Length -ge 100 -or (Test-Path -LiteralPath $isolatedHome)) {
    throw "Unsafe or overlong isolated CODEX_HOME path"
}

$runDir = Join-Path $PackageRoot "live-smoke-$id"
New-Item -ItemType Directory -Path $runDir -ErrorAction Stop | Out-Null
# The Medium worker owns protected ACL creation; never create it elevated.
@{ home = $isolatedHome; exe = $exe } | ConvertTo-Json -Compress | Set-Content -LiteralPath (Join-Path $runDir "config.json") -Encoding ASCII

Write-Host "F2 HOTFIX: Testing direct shared-daemon lifecycle under Medium integrity"
Write-Host "Isolated test log: $runDir"

# Keep the user desktop CODEX_HOME intact after the test.
$originalCodexHome = [Environment]::GetEnvironmentVariable("CODEX_HOME", "Process")
try {
    Invoke-F2Worker -Dir $runDir
    $resultFile = Join-Path $runDir "result.txt"
    if (-not (Test-Path -LiteralPath $resultFile -PathType Leaf)) {
        throw "Smoke worker produced no result; logs: $runDir"
    }
    if (Test-Path -LiteralPath (Join-Path $runDir "trace.txt")) {
        Get-Content -LiteralPath (Join-Path $runDir "trace.txt")
    }
    $outcome = (Get-Content -LiteralPath $resultFile -Raw).Trim()
    Write-Host $outcome
    if ($outcome -notlike "PASS:*") {
        foreach ($log in (Get-ChildItem -LiteralPath $runDir -Filter "*.stderr.txt" -File -ErrorAction SilentlyContinue)) {
            Write-Host "=== $($log.Name) ==="
            Get-Content -LiteralPath $log.FullName -Tail 15
        }
        throw "Live daemon smoke test failed; logs: $runDir"
    }
    # The worker verified clean shutdown and notRunning before this cleanup.
    Remove-Item -LiteralPath $isolatedHome -Recurse -Force -ErrorAction Stop
    Write-Host "F2 HOTFIX: FINAL ACCEPTANCE PASSED"
    Write-Host "Production installation unchanged"
}
finally {
    if ($null -eq $originalCodexHome) {
        Remove-Item Env:CODEX_HOME -ErrorAction SilentlyContinue
    }
    else {
        $env:CODEX_HOME = $originalCodexHome
    }
}
