# CodexDD 0.4.5 3F.62 fixed-source native TUI trust acceptance (NONRELEASE).
[CmdletBinding()]
param(
  [Parameter(Mandatory=$true)][string]$ExpectedHead,
  [string]$LogRoot = '',
  [ValidateSet('Tui')][string]$Group = 'Tui'
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$clock = [Diagnostics.Stopwatch]::StartNew()
$root = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path
$baselineSha = 'ccec79f9dca3035562fe72c55961eea8f73a414d'
$lock = '13bc15ff844e93dfaf7777c73bb7313b35bb3b81'
$stages = @()
$exitCode = 1
if (-not $LogRoot) { $LogRoot = Join-Path $env:TEMP 'codexdd045-3f62-acceptance' }
New-Item -ItemType Directory -Force -Path $LogRoot | Out-Null
Push-Location -LiteralPath $root
try {
  if ($env:OS -ne 'Windows_NT') { throw 'Native tests require Windows' }
  $head = (git rev-parse HEAD | Out-String).Trim()
  if ($LASTEXITCODE -ne 0 -or $ExpectedHead -cnotmatch '^[0-9a-f]{40}$' -or $head -cne $ExpectedHead) {
    throw 'Exact one-shot source SHA mismatch'
  }
  if ((git status --porcelain --untracked-files=no | Out-String).Trim()) { throw 'Tracked files are dirty' }
  if ((git rev-parse 'HEAD:codex-rs/Cargo.lock' | Out-String).Trim() -cne $lock) { throw 'Secure lock source changed' }
  $spec = Get-Content (Join-Path $root 'docs\upstream-candidates\codexdd-045-3f62-native-test-cases.json') -Raw | ConvertFrom-Json
  if ($spec.baseline_reviewed_head -cne $baselineSha -or $spec.release_approved -or $spec.expected_core -ne 0 -or $spec.expected_tui -ne 15) {
    throw 'Native selector manifest changed'
  }
  $formal = Get-Content (Join-Path $root 'docs\upstream-candidates\codexdd-045-3f55-formal-owner-approvals.json') -Raw | ConvertFrom-Json
  if ($formal.approved_owner_count -ne 0 -or $formal.ready_for_formal_review_count -ne 109) { throw 'Formal review identity changed' }
  $env:CARGO_INCREMENTAL = '0'
  $env:CARGO_BUILD_JOBS = '2'
  $env:CARGO_PROFILE_DEV_DEBUG = '0'
  $env:CARGO_PROFILE_TEST_DEBUG = '0'
  $env:RUST_MIN_STACK = '16777216'
  Push-Location (Join-Path $root 'codex-rs')
  try {
    $formatLog = Join-Path $LogRoot 'rustfmt.log'
    & cmd.exe /d /c ('cargo fmt --all -- --check > "'+$formatLog+'" 2>&1')
    if ($LASTEXITCODE -ne 0) { throw 'Full workspace rustfmt failed' }
    Write-Host 'PASS: FULL_WORKSPACE_RUSTFMT'
    $metadataLog = Join-Path $LogRoot 'cargo-metadata.log'
    & cmd.exe /d /c ('cargo metadata --locked --no-deps --format-version 1 > "'+$metadataLog+'" 2>&1')
    if ($LASTEXITCODE -ne 0) { throw 'Locked Cargo metadata failed' }
    Write-Host 'PASS: CARGO_LOCKED_METADATA'
    $areas = if ($Group -eq 'Both') { @('core','tui') } else { @($Group.ToLowerInvariant()) }
    foreach ($area in $areas) {
      $package = if ($area -eq 'core') {'codex-core'} else {'codex-tui'}
      $cases = if ($area -eq 'core') { $spec.cases.core } else { $spec.cases.tui }
      $index = 0
      foreach ($case in $cases) {
        $index++
        $selector = [string]$case.selector
        if ($selector -cnotmatch '^[a-z][a-z0-9_]+$') { throw 'Invalid selector' }
        $log = Join-Path $LogRoot ('{0}-{1:D2}-{2}.log' -f $area,$index,$selector)
        Write-Host ('BEGIN_NATIVE: '+$area+' '+$index+' '+$selector)
        $step = [Diagnostics.Stopwatch]::StartNew()
        & cmd.exe /d /c ('cargo test --locked -p '+$package+' --lib '+$selector+' -- --test-threads=1 > "'+$log+'" 2>&1')
        $status = [int]$LASTEXITCODE
        $step.Stop()
        if (-not (Test-Path -LiteralPath $log -PathType Leaf)) { throw "No native log: $selector" }
        $tail = (Get-Content -LiteralPath $log -Tail 50) -join [Environment]::NewLine
        Write-Host $tail
        $match = [regex]::Match($tail,'(?m)^test result: ok\.\s+([1-9][0-9]*) passed;\s+0 failed;')
        $pass = ($status -eq 0 -and $match.Success)
        $stages += @{
          owner=$area;selector=$selector;rationale=$case.rationale
          result=$(if($pass){'PASS'}else{'FAIL_OR_ZERO_CASES'})
          native_exit_code=$status
          actual_passed_tests=$(if($match.Success){[int]$match.Groups[1].Value}else{0})
          elapsed_seconds=[math]::Round($step.Elapsed.TotalSeconds,1)
          log_file=[IO.Path]::GetFileName($log)
        }
        if (-not $pass) { throw "Native test $selector failed or ran zero test cases; see log $log" }
        Write-Host ('PASS_NATIVE: '+$area+' '+$index+' '+$selector)
      }
      $required = if ($area -eq 'core') {$spec.expected_core} else {$spec.expected_tui}
      if ($index -ne $required) { throw "Missing native $area selectors" }
    }
  } finally { Pop-Location }
  if ((git rev-parse HEAD | Out-String).Trim() -cne $head -or (git status --porcelain --untracked-files=no | Out-String).Trim()) {
    throw 'Source changed during native test run'
  }
  $exitCode = 0
  Write-Host ('CODEXDD_045_3F62=FOCUSED_TUI_NATIVE_GREEN_NONRELEASE '+$Group)
} catch {
  Write-Host ('CODEXDD_045_3F62=RED: '+$_.Exception.Message)
} finally {
  Pop-Location
  $clock.Stop()
  $receipt = [ordered]@{
    contract_version=1;kind='codexdd_045_3f62_native_owner_gate'
    baseline_reviewed_head=$baselineSha;tested_head=$ExpectedHead
    host=$env:COMPUTERNAME;group=$Group
    result=$(if($exitCode -eq 0){'PASS_NONRELEASE'}else{'FAIL'})
    approved_owner_count=0;release_approved=$false;unresolved_high_risk_path_count=13
    stages=@($stages);elapsed_seconds=[math]::Round($clock.Elapsed.TotalSeconds,1)
    done_utc=[DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ')
  }
  $receipt | ConvertTo-Json -Depth 6 | Set-Content (Join-Path $LogRoot ('native-'+$Group+'-receipt.json')) -Encoding utf8
  Write-Host ('CODEXDD_045_3F62_RECEIPT_JSON '+($receipt | ConvertTo-Json -Compress -Depth 6))
  Write-Host ('TOTAL_TIME_WORKED_SECONDS: '+$receipt.elapsed_seconds)
  Write-Host ('DONE_UTC: '+$receipt.done_utc)
}
exit $exitCode
