[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'codexdd-v2-receipt.ps1')

$manifest = [pscustomobject]@{
    contract_version = 2
    state = 'manual_conflict_reconciled_review_required'
    transplant_dry_run = 'conflict'
    candidate_branch = 'automation/upstream-candidate-rust-v0.162.0-222222222222'
    candidate_key = ('rust-v0.162.0@' + ('2' * 40))
    production_sha = '1' * 40
    target_upstream_tag = 'rust-v0.162.0'
    target_upstream_sha = '2' * 40
    sidecar_path = 'docs/upstream-candidates/reconciliation.json'
    sidecar_sha256 = 'b' * 64
    source_snapshot_commit_sha = 'a' * 40
    source_snapshot_tree_sha = 'c' * 40
}
$profileEnd = [pscustomobject]@{
    contract_version = 1
    event = 'profile_end'
    profile = 'release'
    status = 'pass'
    exit_code = 0
    completed_stages = 21
}
$params = @{
    Manifest = $manifest
    ProfileEnd = $profileEnd
    CandidateBranch = $manifest.candidate_branch
    CandidateHeadSha = '9' * 40
    ManifestPath = 'docs/upstream-candidates/codexdd-upstream.json'
    ManifestSha256 = 'f' * 64
    NativeHost = 'Daniel-CL'
}
$receipt = New-CodexDDV2ReleaseReceipt @params
$json = ConvertTo-CodexDDV2CanonicalReceiptJson -Receipt $receipt
$parsed = $json | ConvertFrom-Json
if ($parsed.contract_version -ne 2 -or $parsed.profile -ne 'release') {
    throw 'Native receipt fields were incorrectly serialized.'
}
$tempPath = Join-Path $env:TEMP ('codexdd-v2-receipt-' + [guid]::NewGuid().ToString('N') + '.json')
$tempPython = Join-Path $env:TEMP ('codexdd-v2-canonical-' + [guid]::NewGuid().ToString('N') + '.py')
try {
    [IO.File]::WriteAllText($tempPath, $json, (New-Object Text.UTF8Encoding($false)))
    $py = @'
import json
import pathlib
import sys

raw = pathlib.Path(sys.argv[1]).read_bytes()
obj = json.loads(raw)
canonical = (json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n").encode("utf-8")
if raw != canonical:
    raise SystemExit("PowerShell receipt JSON bytes are not canonical")
if obj["native_platform"] != "windows" or obj["completed_stages"] != 21:
    raise SystemExit("PowerShell receipt values changed")
'@
    [IO.File]::WriteAllText($tempPython, $py, (New-Object Text.UTF8Encoding($false)))
    & python $tempPython $tempPath
    if ($LASTEXITCODE -ne 0) {
        throw "Native receipt canonical byte comparison failed ($LASTEXITCODE)."
    }
}
finally {
    Remove-Item -LiteralPath $tempPath, $tempPython -ErrorAction SilentlyContinue
}

$failedProfile = [pscustomobject]@{
    contract_version = 1
    event = 'profile_end'
    profile = 'work-packet'
    status = 'pass'
    exit_code = 0
    completed_stages = 21
}
$params.ProfileEnd = $failedProfile
$blocked = $false
try {
    $null = New-CodexDDV2ReleaseReceipt @params
}
catch {
    $blocked = $true
}
if (-not $blocked) {
    throw 'Work-packet improperly accepted as v2 release.'
}

$params.ProfileEnd = $profileEnd
$params.NativeHost = 'CI-RUNNER'
$blocked = $false
try {
    $null = New-CodexDDV2ReleaseReceipt @params
}
catch {
    $blocked = $true
}
if (-not $blocked) {
    throw 'Unrelated Windows host improperly accepted.'
}
Write-Host 'CODEXDD_V2_RECEIPT_POWERSHELL_FIXTURE PASS'
