# 3F.94 NONRELEASE: create an ephemeral standard Windows user on an isolated GitHub runner.
# Production daemon's Windows token-elevation policy is never altered.
param([switch]$ProbeOnly)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_OS -ne 'Windows' -or
    $env:GITHUB_REPOSITORY -ne 'Takuetsu/codex-dd-astra' -or
    $env:GITHUB_REF -ne 'refs/heads/dd/codexdd-v0.4.5-3f94-windows-managed-supervisor') {
  throw '3F.94 temporary standard account requires the exact GitHub Windows CI branch'
}
if (-not $env:RUNNER_TEMP -or -not $env:GITHUB_WORKSPACE) { throw 'Missing ephemeral runner paths' }
if (-not (Get-Command New-LocalUser -ErrorAction SilentlyContinue)) { throw 'Cannot provision standard Windows account on runner' }

$bytes = New-Object byte[] 48
$rng = [Security.Cryptography.RandomNumberGenerator]::Create()
try { $rng.GetBytes($bytes) } finally { $rng.Dispose() }
$plainPassword = [Convert]::ToBase64String($bytes) + 'Aa1!'
$securePassword = ConvertTo-SecureString $plainPassword -AsPlainText -Force
$plainPassword = $null
$username = 'cd94_' + [guid]::NewGuid().ToString('N').Substring(0,10)
$account = "$($env:COMPUTERNAME)\$username"
$credential = [Management.Automation.PSCredential]::new($account, $securePassword)
$sandbox = Join-Path $env:RUNNER_TEMP ('codexdd-3f94-user-' + [guid]::NewGuid().ToString('N'))
$userCreated = $false
$child = $null

try {
  $null = New-LocalUser -Name $username -Password $securePassword -PasswordNeverExpires -AccountNeverExpires -Description 'Transient nonadmin CodexDD GitHub Actions test'
  $userCreated = $true
  $admins = Get-LocalGroupMember -Group 'Administrators'
  if (@($admins | Where-Object Name -EQ $account).Count -ne 0) { throw 'Transient user was unexpectedly added to Administrators' }

  New-Item -ItemType Directory -Path $sandbox -Force | Out-Null
  foreach ($sub in @('target\debug','native-evidence','tmp')) {
    New-Item -ItemType Directory -Path (Join-Path $sandbox $sub) -Force | Out-Null
  }
  Copy-Item -LiteralPath (Join-Path $env:GITHUB_WORKSPACE '.github\scripts\test_codexdd_045_3f94_windows_managed_supervisor.ps1') -Destination (Join-Path $sandbox 'supervisor-test.ps1') -ErrorAction Stop
  Copy-Item -LiteralPath (Join-Path $env:GITHUB_WORKSPACE '.github\scripts\codexdd_045_3f94_private_directory.ps1') -Destination (Join-Path $sandbox 'codexdd_045_3f94_private_directory.ps1') -ErrorAction Stop
  if (-not $ProbeOnly) {
    if (-not $env:CARGO_TARGET_DIR) { throw 'Missing source-exact compiled Cargo CLI directory' }
    $exe = Join-Path $env:CARGO_TARGET_DIR 'debug\codex.exe'
    if (-not (Test-Path -LiteralPath $exe -PathType Leaf)) { throw "Missing native compiled executable $exe" }
    Copy-Item -LiteralPath $exe -Destination (Join-Path $sandbox 'target\debug\codex.exe') -ErrorAction Stop
  }
  $runnerScript = Join-Path $sandbox 'runner.ps1'
  @'
param([string]$SandboxPath, [switch]$ProbeOnly)
$ErrorActionPreference = 'Stop'
$env:GITHUB_ACTIONS = 'true'
$env:RUNNER_OS = 'Windows'
$env:GITHUB_REPOSITORY = 'Takuetsu/codex-dd-astra'
$env:GITHUB_REF = 'refs/heads/dd/codexdd-v0.4.5-3f94-windows-managed-supervisor'
$env:RUNNER_TEMP = $SandboxPath
$env:CARGO_TARGET_DIR = Join-Path $SandboxPath 'target'
$env:GITHUB_WORKSPACE = $SandboxPath
$env:TEMP = Join-Path $SandboxPath 'tmp'
$env:TMP = $env:TEMP
$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = [Security.Principal.WindowsPrincipal]::new($identity)
if ($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
  throw 'Restricted test identity has administrator privileges'
}
Add-Type -TypeDefinition @"
using System;
using System.ComponentModel;
using System.Runtime.InteropServices;
public static class DdTokenElevation {
  [DllImport("kernel32.dll")] static extern IntPtr GetCurrentProcess();
  [DllImport("advapi32.dll", SetLastError = true)] static extern bool OpenProcessToken(IntPtr process, uint access, out IntPtr token);
  [DllImport("advapi32.dll", SetLastError = true)] static extern bool GetTokenInformation(IntPtr token, int kind, out int elevation, int length, out int returned);
  [DllImport("kernel32.dll")] static extern bool CloseHandle(IntPtr handle);
  public static int Query() {
    IntPtr token;
    if (!OpenProcessToken(GetCurrentProcess(), 8, out token)) throw new Win32Exception(Marshal.GetLastWin32Error());
    try {
      int elevated, returned;
      if (!GetTokenInformation(token, 20, out elevated, 4, out returned)) throw new Win32Exception(Marshal.GetLastWin32Error());
      if (returned != 4) throw new Exception("Unexpected token elevation length");
      return elevated;
    } finally { CloseHandle(token); }
  }
}
"@
if ([DdTokenElevation]::Query() -ne 0) {
  throw 'TokenElevation is nonzero: managed app-server would correctly reject this launcher'
}
Write-Host ('3F.94 RESTRICTED WINDOWS TOKEN GREEN: ' + $identity.Name)
. (Join-Path $SandboxPath 'codexdd_045_3f94_private_directory.ps1')
$fixtureHome = Join-Path $SandboxPath 'secure-socket-probe'
New-Item -ItemType Directory -Path $fixtureHome -Force | Out-Null
New-DDPrivateDaemonDirectory -sandboxHome $fixtureHome
Write-Host '3F.94 STRICT SOCKET ACL PREFLIGHT GREEN'
if ($ProbeOnly) {
  Set-Content -LiteralPath (Join-Path $SandboxPath 'standard-token-attestation.txt') -Value 'NONADMIN_TOKEN_ELEVATION_0' -Encoding UTF8
  Write-Host '3F.94 NONADMIN PREFLIGHT GREEN'
  exit 0
}
& (Join-Path $SandboxPath 'supervisor-test.ps1')
Set-Content -LiteralPath (Join-Path $SandboxPath 'standard-token-attestation.txt') -Value 'NONADMIN_TOKEN_ELEVATION_0_PROCESS_TEST_COMPLETED' -Encoding UTF8
Write-Host '3F.94 STANDARD USER SUPERVISOR TEST SCRIPT COMPLETED'
'@ | Set-Content -LiteralPath $runnerScript -Encoding UTF8

  $icacls = Join-Path $env:WINDIR 'System32\icacls.exe'
  & $icacls $sandbox /grant "$($account):(OI)(CI)M" /T /C | Out-Null
  if ($LASTEXITCODE -ne 0) { throw 'Cannot grant sandbox access to the restricted account' }

  $childOut = Join-Path $sandbox 'child-stdout.log'
  $childErr = Join-Path $sandbox 'child-stderr.log'
  $quotedRunner = '"' + $runnerScript + '"'
  $quotedSandbox = '"' + $sandbox + '"'
  $args = @('-NoProfile','-NonInteractive','-File',$quotedRunner,'-SandboxPath',$quotedSandbox)
  if ($ProbeOnly) { $args += '-ProbeOnly' }
  $pwsh = (Get-Command pwsh.exe -ErrorAction Stop).Source
  $child = Start-Process -FilePath $pwsh -ArgumentList $args -Credential $credential -LoadUserProfile -WorkingDirectory $sandbox -PassThru -RedirectStandardOutput $childOut -RedirectStandardError $childErr
  try {
    $limit = if ($ProbeOnly) { 60000 } else { 900000 }
    if (-not $child.WaitForExit($limit)) {
      $child.Kill()
      $child.WaitForExit()
      throw 'Standard user launcher exceeded the bounded test timeout'
    }
    $code = $child.ExitCode
  } finally { $child.Dispose(); $child = $null }

  if (Test-Path -LiteralPath $childOut) { Get-Content -LiteralPath $childOut -Tail 100 -ErrorAction SilentlyContinue }
  if (Test-Path -LiteralPath $childErr) { Get-Content -LiteralPath $childErr -Tail 100 -ErrorAction SilentlyContinue }
  if ($null -ne $code -and $code -ne 0) { throw "3F.94 standard Windows process exited $code" }
  $attestationPath = Join-Path $sandbox 'standard-token-attestation.txt'
  if (-not (Test-Path -LiteralPath $attestationPath -PathType Leaf)) {
    throw 'Missing standard-user child attestation: process failed or exited before completing its token checks'
  }
  $attestation = (Get-Content -LiteralPath $attestationPath -Raw).Trim()
  $expected = if ($ProbeOnly) { 'NONADMIN_TOKEN_ELEVATION_0' } else { 'NONADMIN_TOKEN_ELEVATION_0_PROCESS_TEST_COMPLETED' }
  if ($attestation -cne $expected) { throw "Unexpected standard-user attestation: $attestation" }

  if ($ProbeOnly) {
    Write-Host '3F.94 REAL NONADMIN WINDOWS ACCOUNT PREFLIGHT GREEN'
  } else {
    $receiptPath = Join-Path $sandbox 'native-evidence\3f94-windows-supervisor-receipt.json'
    if (-not (Test-Path -LiteralPath $receiptPath -PathType Leaf)) { throw 'Missing nonadmin 3F.94 receipt' }
    $receipt = Get-Content -LiteralPath $receiptPath -Raw | ConvertFrom-Json
    if ($receipt.passed -ne 2 -or $receipt.release_approved -ne $false -or $receipt.formal_owner_signoffs -ne 0) {
      throw '3F.94 receipt does not certify 2/2 nonrelease process cases'
    }
    Copy-Item -Path (Join-Path $sandbox 'native-evidence\*') -Destination (Join-Path $env:GITHUB_WORKSPACE 'native-evidence') -Force -ErrorAction Stop
    Copy-Item -LiteralPath $childOut -Destination (Join-Path $env:GITHUB_WORKSPACE 'native-evidence\standard-user-stdout.log') -Force
    Copy-Item -LiteralPath $childErr -Destination (Join-Path $env:GITHUB_WORKSPACE 'native-evidence\standard-user-stderr.log') -Force
    Write-Host '3F.94 NONADMIN WINDOWS DAEMON/UPDATER PROCESS TESTS GREEN: 2/2'
  }
} finally {
  if ($null -ne $child) {
    try { if (-not $child.HasExited) { $child.Kill() } } catch {}
    $child.Dispose()
  }
  if ($userCreated) {
    try { Remove-LocalUser -Name $username -ErrorAction Stop }
    catch { Write-Host '3F.94 temporary standard user cleanup failed on disposable runner' }
  }
}
