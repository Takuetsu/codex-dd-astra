# CodexDD 0.4.5 3F.85: targeted Core terminalization regression (NONRELEASE).
# Cheap, single-selector native Windows acceptance. No CI, install, merge or promotion.
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$ExpectedHead,
    [string]$LogDir = 'E:\codexdd\logs\045-3f85'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
if ($env:OS -ne 'Windows_NT') {
    throw 'This runner requires native Windows on Daniel-CL'
}

$repo = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
$cargoRoot = Join-Path $repo 'codex-rs'
New-Item -ItemType Directory -Force -Path $LogDir | Out-Null
$head = (& git -C $repo rev-parse HEAD | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or $head -cne $ExpectedHead) {
    throw ("Unexpected candidate head: " + $head)
}
$dirty = (& git -C $repo status --porcelain --untracked-files=normal | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or $dirty) {
    throw 'Test worktree must be clean'
}
& git -C $repo diff --check '131c4954cfd493eb05bb2694c98275bd8c810a50' HEAD
if ($LASTEXITCODE -ne 0) {
    throw 'Source diff contains invalid whitespace'
}

# No dependency graph changes are permitted for this focused Core test.
foreach ($item in @(
    @{ Path = 'codex-rs/Cargo.toml'; Sha = 'cfebd9b784ef9ff027b274aa9f7101c9c2496613' },
    @{ Path = 'codex-rs/Cargo.lock'; Sha = '13bc15ff844e93dfaf7777c73bb7313b35bb3b81' },
    @{ Path = 'codex-rs/deny.toml'; Sha = 'f282c7c55d05201e308e60804f85754c8522d462' }
)) {
    $actual = (& git -C $repo rev-parse ("HEAD:" + $item.Path) | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $actual -cne $item.Sha) {
        throw ('Protected source graph mismatch: ' + $item.Path)
    }
}

$cargo = (Get-Command cargo.exe -ErrorAction Stop).Source
$env:RUST_MIN_STACK = '16777216'
$stdout = Join-Path $LogDir 'core-terminalization-stdout.log'
$stderr = Join-Path $LogDir 'core-terminalization-stderr.log'
$fmtOut = Join-Path $LogDir 'fmt-stdout.log'
$fmtErr = Join-Path $LogDir 'fmt-stderr.log'

Write-Host 'BEGIN: Rust formatting verification'
$fmt = Start-Process -FilePath $cargo -ArgumentList @('fmt', '--all', '--', '--check') `
    -WorkingDirectory $cargoRoot -NoNewWindow -PassThru -Wait `
    -RedirectStandardOutput $fmtOut -RedirectStandardError $fmtErr
if ($fmt.ExitCode -ne 0) {
    Get-Content -LiteralPath $fmtErr -Tail 60 -ErrorAction SilentlyContinue
    throw 'Rustfmt failed; do not run expensive CI'
}
Write-Host 'PASS: Rustfmt' -ForegroundColor Green

Write-Host 'BEGIN: single Core terminalization native selector'
$test = Start-Process -FilePath $cargo -ArgumentList @(
    'test', '--locked', '-p', 'codex-core', '--lib',
    'validation_terminalization_requests_required_tool_choice',
    '--', '--nocapture'
) -WorkingDirectory $cargoRoot -NoNewWindow -PassThru -Wait `
    -RedirectStandardOutput $stdout -RedirectStandardError $stderr
if ($test.ExitCode -ne 0) {
    Get-Content -LiteralPath $stdout -Tail 50 -ErrorAction SilentlyContinue
    Get-Content -LiteralPath $stderr -Tail 70 -ErrorAction SilentlyContinue
    throw ('Native Core regression failed: ' + $test.ExitCode)
}
$output = (Get-Content -LiteralPath $stdout -Raw) +
    (Get-Content -LiteralPath $stderr -Raw)
if ($output -notmatch 'test result: ok\.\s+1 passed;\s+0 failed;') {
    throw 'Core selector did not execute exactly one passing test'
}
if ($output -notmatch 'validation_terminalization_requests_required_tool_choice\s+\.\.\.\s+ok') {
    throw 'Required Core regression name is not present in passing output'
}

$receipt = [pscustomobject]@{
    kind = 'codexdd_045_3f85_core_terminalization_native_nonrelease'
    head = $head
    utc = (Get-Date).ToUniversalTime().ToString('o')
    selector = 'validation_terminalization_requests_required_tool_choice'
    executed_passed = 1
    test_thread_stack_bytes = [int]$env:RUST_MIN_STACK
    stdout = $stdout
    stderr = $stderr
    release_approved = $false
    formal_owner_signoffs = 0
}
$receiptPath = Join-Path $LogDir 'core-terminalization-receipt.json'
$receipt | ConvertTo-Json -Depth 5 |
    Set-Content -LiteralPath $receiptPath -Encoding UTF8
Write-Host '3F.85 CORE TERMINALIZATION NATIVE GREEN: 1/1' -ForegroundColor Green
Write-Host ('Receipt: ' + $receiptPath)
