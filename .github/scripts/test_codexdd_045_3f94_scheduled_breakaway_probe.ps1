# CodexDD 0.4.5 3F.94 NONRELEASE: inexpensive Task Scheduler breakaway
# capability probe. This DOES NOT launch the daemon, compile Rust, or install.
# Task Scheduler (service) may create a real non-elevated process outside the
# restrictive GitHub Actions Windows runner job. Prove it before expensive CI.
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_OS -ne 'Windows' -or
    $env:GITHUB_REPOSITORY -ne 'Takuetsu/codex-dd-astra' -or
    $env:GITHUB_REF -ne 'refs/heads/dd/codexdd-v0.4.5-3f94-windows-managed-supervisor') {
  throw '3F.94 Task Scheduler capability probe restricted to its exact ephemeral GitHub Windows branch'
}
if (-not $env:RUNNER_TEMP -or -not $env:GITHUB_WORKSPACE) { throw 'Missing ephemeral Windows runner paths' }
foreach ($cmd in @('New-LocalUser','Remove-LocalUser','Register-ScheduledTask','Start-ScheduledTask','Unregister-ScheduledTask','Get-ScheduledTaskInfo')) {
  if (-not (Get-Command $cmd -ErrorAction SilentlyContinue)) { throw "No $cmd on Windows runner" }
}
$bytes=New-Object byte[] 48
$rng=[Security.Cryptography.RandomNumberGenerator]::Create()
try { $rng.GetBytes($bytes) } finally { $rng.Dispose() }
$passwordPlain=[Convert]::ToBase64String($bytes) + 'aA1!'
$passwordSecure=ConvertTo-SecureString $passwordPlain -AsPlainText -Force
$userName='dd94_' + [guid]::NewGuid().ToString('N').Substring(0,12)
$account="$($env:COMPUTERNAME)\$userName"
$taskName='CodexDd94Breakaway-' + [guid]::NewGuid().ToString('N').Substring(0,12)
$sandbox=Join-Path $env:RUNNER_TEMP ('codexdd-3f94-task-' + [guid]::NewGuid().ToString('N'))
$createdUser=$false
$registered=$false
try {
  $null=New-LocalUser -Name $userName -Password $passwordSecure -PasswordNeverExpires -AccountNeverExpires -Description 'Transient restricted CodexDD 3F.94 CI task'
  $createdUser=$true
  $admins=Get-LocalGroupMember -Group 'Administrators'
  if (@($admins | Where-Object Name -EQ $account).Count -gt 0) { throw 'Probe user is an Administrator' }
  New-Item -ItemType Directory -Path $sandbox -Force | Out-Null
  $testPath=Join-Path $sandbox 'probe-child.ps1'
  Copy-Item -LiteralPath (Join-Path $env:GITHUB_WORKSPACE '.github\scripts\test_codexdd_045_3f94_breakaway_child.ps1') -Destination $testPath -ErrorAction Stop
  $icacls=Join-Path $env:WINDIR 'System32\icacls.exe'
  & $icacls $sandbox /grant "$($account):(OI)(CI)M" /T /C | Out-Null
  if ($LASTEXITCODE -ne 0) { throw 'Cannot grant isolated probe directory to its restricted account' }
  $pwsh=(Get-Command pwsh.exe -ErrorAction Stop).Source
  $arguments="-NoProfile -NonInteractive -File " + ('"' + $testPath + '"') + " -SandboxPath " + ('"' + $sandbox + '"')
  $action=New-ScheduledTaskAction -Execute $pwsh -Argument $arguments -WorkingDirectory $sandbox
  $settings=New-ScheduledTaskSettingsSet -ExecutionTimeLimit (New-TimeSpan -Minutes 3) -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries
  $trigger=New-ScheduledTaskTrigger -Once -At (Get-Date).AddHours(1)
  $null=Register-ScheduledTask -TaskName $taskName -User $account -Password $passwordPlain -Action $action -Settings $settings -Trigger $trigger -RunLevel Limited -Force -ErrorAction Stop
  $registered=$true
  $passwordPlain=$null
  Start-ScheduledTask -TaskName $taskName -ErrorAction Stop
  $path=Join-Path $sandbox 'breakaway-result.json'
  $deadline=(Get-Date).AddSeconds(105)
  while ((Get-Date) -lt $deadline -and -not (Test-Path -LiteralPath $path -PathType Leaf)) {
    Start-Sleep -Milliseconds 500
  }
  if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
    $info=Get-ScheduledTaskInfo -TaskName $taskName -ErrorAction SilentlyContinue
    throw "No child receipt; Task Scheduler last result: $($info.LastTaskResult)"
  }
  $receipt=Get-Content -LiteralPath $path -Raw | ConvertFrom-Json -ErrorAction Stop
  $details=$receipt | ConvertTo-Json -Depth 5
  Write-Host "3F.94 SCHEDULED TASK NONADMIN BREAKAWAY RECEIPT: $details"
  if ($receipt.identity -ine $account -or $receipt.token_elevated -ne 0 -or
      $receipt.child_executed -ne $false -or $receipt.production_security_changed -ne $false -or
      $receipt.breakaway_error_code -ne 0 -or $receipt.detached_launch_possible -ne $true) {
    throw 'Scheduled Task could not prove safe detached background launch under restricted account'
  }
  Write-Host '3F.94 SCHEDULED TASK NONADMIN DETACHED LAUNCH CAPABILITY GREEN' -ForegroundColor Green
} finally {
  $passwordPlain=$null
  if ($registered) {
    try {
      $task=Get-ScheduledTask -TaskName $taskName -ErrorAction SilentlyContinue
      if ($task -and $task.State -eq 'Running') { Stop-ScheduledTask -TaskName $taskName -ErrorAction SilentlyContinue }
      Unregister-ScheduledTask -TaskName $taskName -Confirm:$false -ErrorAction Stop
    } catch {
      Write-Warning 'Could not unregister ephemeral Windows probe task on disposable runner'
    }
  }
  if ($createdUser) {
    try { Remove-LocalUser -Name $userName -ErrorAction Stop }
    catch { Write-Warning 'Could not remove ephemeral standard account on disposable runner' }
  }
}
