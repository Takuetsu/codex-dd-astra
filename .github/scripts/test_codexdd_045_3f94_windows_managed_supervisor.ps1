# 3F.94 NONRELEASE: real Windows Codex CLI managed PID / updater process lifecycle.
# Run only in a GitHub-hosted Windows worker with one-off, isolated CODEX_HOME.
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_OS -ne 'Windows' -or
    $env:GITHUB_REPOSITORY -ne 'Takuetsu/codex-dd-astra' -or
    $env:GITHUB_REF -ne 'refs/heads/dd/codexdd-v0.4.5-3f94-windows-managed-supervisor') {
  throw '3F.94 Windows process acceptance may run only on its isolated GitHub Actions branch'
}
if (-not $env:CARGO_TARGET_DIR -or -not $env:RUNNER_TEMP) {
  throw '3F.94 requires the isolated Cargo target and GitHub runner temp'
}
$exe = Join-Path $env:CARGO_TARGET_DIR 'debug\codex.exe'
if (-not (Test-Path -LiteralPath $exe -PathType Leaf)) {
  throw "3F.94 cannot find the locally compiled Codex executable: $exe"
}

$originalHome = [Environment]::GetEnvironmentVariable('CODEX_HOME')
$originalSqlite = [Environment]::GetEnvironmentVariable('CODEX_SQLITE_HOME')
$blocked = @{}
foreach ($key in @('OPENAI_API_KEY', 'CODEX_API_KEY', 'CODEX_ACCESS_TOKEN',
                   'CODEX_EXEC_SERVER_URL', 'OPENAI_FEDERATION_RULE_ID',
                   'OPENAI_IDENTITY_TOKEN_FILE', 'OPENAI_WORKLOAD_IDENTITY_CONTEXT')) {
  $blocked[$key] = [Environment]::GetEnvironmentVariable($key)
  [Environment]::SetEnvironmentVariable($key, $null)
}

$base = Join-Path $env:RUNNER_TEMP ("codexdd-3f94-" + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $base -Force | Out-Null
$results = [System.Collections.Generic.List[object]]::new()
$owned = [System.Collections.Generic.List[object]]::new()
$serial = 0

function Assert-Condition([bool]$condition, [string]$description) {
  if (-not $condition) { throw "3F.94 assertion failed: $description" }
}
function Set-IsolatedHome([string]$sandboxHome) {
  $env:CODEX_HOME = $sandboxHome
  $env:CODEX_SQLITE_HOME = $sandboxHome
}
function Install-TestBinary([string]$target) {
  New-Item -ItemType Directory -Path (Split-Path $target -Parent) -Force | Out-Null
  try {
    New-Item -ItemType HardLink -Path $target -Target $exe -ErrorAction Stop | Out-Null
  } catch {
    Copy-Item -LiteralPath $exe -Destination $target -ErrorAction Stop
  }
}
function Set-Updater([string]$sandboxHome, [bool]$enabled) {
  $settings = Join-Path $sandboxHome 'app-server-daemon\settings.json'
  New-Item -ItemType Directory -Path (Split-Path $settings -Parent) -Force | Out-Null
  @{
    shutdownGraceSeconds = 0
    updater = @{ autoUpdateEnabled = $enabled; updateIntervalMinutes = 1440 }
  } | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath $settings -Encoding utf8
}
function Invoke-Daemon([string]$sandboxHome, [string]$action) {
  $script:serial++
  Set-IsolatedHome $sandboxHome
  $out = Join-Path $env:GITHUB_WORKSPACE ("native-evidence\3f94-" + $script:serial + '-' + $action + '.out')
  $err = Join-Path $env:GITHUB_WORKSPACE ("native-evidence\3f94-" + $script:serial + '-' + $action + '.err')
  $child = Start-Process -FilePath $exe -ArgumentList @('app-server','daemon',$action) -PassThru -NoNewWindow -RedirectStandardOutput $out -RedirectStandardError $err
  try {
    if (-not $child.WaitForExit(90000)) {
      $child.Kill($true)
      $child.WaitForExit()
      throw "3F.94 daemon $action timed out in isolated home $sandboxHome"
    }
    $status = $child.ExitCode
  } finally {
    $child.Dispose()
  }
  $stdout = if (Test-Path -LiteralPath $out) { Get-Content -LiteralPath $out -Raw } else { '' }
  $stderr = if (Test-Path -LiteralPath $err) { Get-Content -LiteralPath $err -Raw } else { '' }
  Assert-Condition ($status -eq 0) "daemon $action returned $status, stderr: $stderr"
  try {
    $json = $stdout | ConvertFrom-Json -ErrorAction Stop
  } catch {
    throw "3F.94 daemon $action did not return lifecycle JSON: $stdout ; $stderr"
  }
  Write-Host ("3F.94 DAEMON {0}: {1}" -f $action, $json.status)
  return $json
}
function Read-ManagedPid([string]$sandboxHome, [string]$filename) {
  $file = Join-Path $sandboxHome ("app-server-daemon\" + $filename)
  Assert-Condition (Test-Path -LiteralPath $file -PathType Leaf) "missing PID record $file"
  $record = Get-Content -LiteralPath $file -Raw | ConvertFrom-Json
  Assert-Condition ([long]$record.pid -gt 0) "$filename has no PID"
  Assert-Condition (-not [string]::IsNullOrEmpty($record.processStartTime)) "$filename missing process start identity"
  # Windows uses processStartTime/OS process handles; processIdentity is Unix-only.
  if ($filename -notmatch "updater") {
    Assert-Condition ($null -ne $record.executableIdentity) "$filename missing managed executable identity"
  }
  $id = [int]$record.pid
  Assert-Condition ($null -ne (Get-Process -Id $id -ErrorAction SilentlyContinue)) "$filename process $id is not running"
  $script:owned.Add([pscustomobject]@{home=$sandboxHome; pid=$id}) | Out-Null
  return $id
}
function Test-Case([string]$label, [scriptblock]$body) {
  Write-Host "BEGIN 3F.94: $label"
  try {
    & $body
    $script:results.Add([pscustomobject]@{name=$label; passed=$true}) | Out-Null
    Write-Host "PASS 3F.94: $label" -ForegroundColor Green
  } catch {
    $script:results.Add([pscustomobject]@{name=$label; passed=$false; reason="$($_.Exception.Message)"}) | Out-Null
    Write-Host "FAIL 3F.94: $label - $($_.Exception.Message)" -ForegroundColor Red
    throw
  }
}

try {
  Test-Case 'dedicated-managed-daemon-pid-restart-and-stop' {
    $sandboxHome = Join-Path $base 'dedicated'
    $managed = Join-Path $sandboxHome 'packages\app-server-daemon\current\bin\codex.exe'
    Install-TestBinary $managed
    Set-Updater $sandboxHome $false
    $first = Invoke-Daemon $sandboxHome 'start'
    Assert-Condition ($first.status -eq 'started') 'dedicated first start must launch'
    $pid1 = Read-ManagedPid $sandboxHome 'daemon.pid'
    Assert-Condition ($first.backend -eq 'pid') 'dedicated start must own PID backend'
    Assert-Condition ($first.managedCodexPath -eq $managed) 'dedicated start must use isolated package'
    $second = Invoke-Daemon $sandboxHome 'start'
    Assert-Condition ($second.status -eq 'alreadyRunning') 'dedicated duplicate start must not launch'
    Assert-Condition ((Read-ManagedPid $sandboxHome 'daemon.pid') -eq $pid1) 'duplicate start changed daemon PID'
    $version = Invoke-Daemon $sandboxHome 'version'
    Assert-Condition ($version.status -eq 'running' -and $version.backend -eq 'pid') 'dedicated version not healthy'
    Assert-Condition (-not [string]::IsNullOrEmpty($version.appServerVersion)) 'daemon did not answer real control socket'
    $restarted = Invoke-Daemon $sandboxHome 'restart'
    Assert-Condition ($restarted.status -eq 'restarted') 'dedicated restart rejected'
    $pid2 = Read-ManagedPid $sandboxHome 'daemon.pid'
    Assert-Condition ($pid2 -ne $pid1) 'dedicated restart reused daemon PID'
    Assert-Condition (-not (Test-Path (Join-Path $sandboxHome 'app-server-daemon\daemon-updater.pid'))) 'disabled updater launched'
    $stopped = Invoke-Daemon $sandboxHome 'stop'
    Assert-Condition ($stopped.status -eq 'stopped') 'dedicated daemon failed to stop'
    Assert-Condition (-not (Test-Path (Join-Path $sandboxHome 'app-server-daemon\daemon.pid'))) 'stale dedicated PID record after stop'
    Write-Host "3F.94 dedicated process lifecycle GREEN: old=$pid1 successor=$pid2"
  }

  Test-Case 'stable-legacy-updater-survives-restart-and-obeys-disable' {
    $sandboxHome = Join-Path $base 'legacy-stable'
    $release = '0.0.0-x86_64-pc-windows-msvc'
    $standalone = Join-Path $sandboxHome 'packages\standalone'
    $releaseDir = Join-Path $standalone ("releases\" + $release)
    $managed = Join-Path $releaseDir 'bin\codex.exe'
    Install-TestBinary $managed
    New-Item -ItemType Junction -Path (Join-Path $standalone 'current') -Target $releaseDir -ErrorAction Stop | Out-Null
    Set-Content -LiteralPath (Join-Path $standalone 'auto-update-version') -Value $release -Encoding utf8 -NoNewline
    Set-Updater $sandboxHome $true
    Set-Content -LiteralPath (Join-Path $sandboxHome 'app-server-daemon\app-server.stderr.log') -Value '' -Encoding utf8
    $started = Invoke-Daemon $sandboxHome 'start'
    Assert-Condition ($started.status -eq 'started' -and $started.backend -eq 'pid') 'legacy managed daemon did not start'
    $pid1 = Read-ManagedPid $sandboxHome 'app-server.pid'
    $updater1 = Read-ManagedPid $sandboxHome 'app-server-updater.pid'
    Assert-Condition ($pid1 -ne $updater1) 'daemon and updater cannot be the same process'
    $restarted = Invoke-Daemon $sandboxHome 'restart'
    Assert-Condition ($restarted.status -eq 'restarted') 'legacy restart not accepted'
    $pid2 = Read-ManagedPid $sandboxHome 'app-server.pid'
    $updater2 = Read-ManagedPid $sandboxHome 'app-server-updater.pid'
    Assert-Condition ($pid2 -ne $pid1) 'legacy restart did not replace daemon process'
    Assert-Condition ($updater2 -eq $updater1) 'updater unexpectedly replaced on ordinary daemon restart'
    Assert-Condition ((Invoke-Daemon $sandboxHome 'version').status -eq 'running') 'legacy successor did not answer control socket'
    Set-Updater $sandboxHome $false
    Assert-Condition ((Invoke-Daemon $sandboxHome 'restart').status -eq 'restarted') 'restart with updater disabled failed'
    Assert-Condition (-not (Test-Path (Join-Path $sandboxHome 'app-server-daemon\app-server-updater.pid'))) 'updater PID retained despite disabled preference'
    Assert-Condition ($null -eq (Get-Process -Id $updater1 -ErrorAction SilentlyContinue)) 'disabled updater process remains alive'
    Assert-Condition ((Invoke-Daemon $sandboxHome 'stop').status -eq 'stopped') 'legacy daemon stop failed'
    Assert-Condition (-not (Test-Path (Join-Path $sandboxHome 'app-server-daemon\app-server.pid'))) 'legacy PID record survived stop'
    Write-Host "3F.94 updater lifecycle GREEN: old=$pid1 successor=$pid2 updater=$updater1 disabled=true"
  }

  $receipt = [pscustomobject]@{
    kind = 'codexdd045_3f94_real_windows_managed_supervisor_nonrelease'
    tested_head = $env:GITHUB_SHA
    utc = (Get-Date).ToUniversalTime().ToString('o')
    cases = $results.ToArray()
    passed = @($results | Where-Object { $_.passed }).Count
    release_approved = $false
    formal_owner_signoffs = 0
  }
  $receipt | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $env:GITHUB_WORKSPACE 'native-evidence\3f94-windows-supervisor-receipt.json') -Encoding utf8
  Assert-Condition ($results.Count -eq 2 -and @($results | Where-Object { $_.passed }).Count -eq 2) 'expected two passing process cases'
  Write-Host '3F.94 REAL WINDOWS SUPERVISOR / UPDATER TESTS GREEN: 2/2'
} finally {
  foreach ($sandboxHome in @((Join-Path $base 'dedicated'), (Join-Path $base 'legacy-stable'))) {
    if (Test-Path $sandboxHome) {
      try {
        Set-IsolatedHome $sandboxHome
        $null = Invoke-Daemon $sandboxHome 'stop'
      } catch {
        Write-Host "3F.94 cleanup daemon stop: $($_.Exception.Message)"
      }
    }
  }
  # PID reuse guard: force-stop only processes proven to run a binary under our temporary home.
  foreach ($entry in $owned) {
    try {
      $running = Get-Process -Id $entry.pid -ErrorAction SilentlyContinue
      if ($null -ne $running -and $null -ne $running.Path) {
        $ownedPrefix = ([IO.Path]::GetFullPath($entry.home)).TrimEnd('\') + '\'
        $path = [IO.Path]::GetFullPath($running.Path)
        if ($path.StartsWith($ownedPrefix, [StringComparison]::OrdinalIgnoreCase)) {
          Stop-Process -Id $entry.pid -Force -ErrorAction SilentlyContinue
        }
      }
    } catch {
      Write-Host '3F.94 cleanup could not inspect a PID; leaving unknown process untouched'
    }
  }
  $env:CODEX_HOME = $originalHome
  $env:CODEX_SQLITE_HOME = $originalSqlite
  foreach ($key in $blocked.Keys) {
    [Environment]::SetEnvironmentVariable($key, $blocked[$key])
  }
}
