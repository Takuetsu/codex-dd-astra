[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$ManifestPath,
    [Parameter(Mandatory = $true)][string]$ApprovedDiscoveryManifestSha256,
    [Parameter(Mandatory = $true)][string]$ApprovedConflictsSha256
)

# Isolated native v2 validation: no candidate publishing or PR promotion.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$repositoryRoot = Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot 'codexdd-v2-receipt.ps1')
. (Join-Path $PSScriptRoot 'codexdd-python-executable.ps1')

Push-Location -LiteralPath $repositoryRoot
try {
    if ($env:OS -ne 'Windows_NT' -or $env:COMPUTERNAME -ine 'Daniel-CL') {
        throw 'Native release v2 must run on Daniel-CL Windows.'
    }
    if ($ApprovedDiscoveryManifestSha256 -cnotmatch '^[0-9a-f]{64}$' -or
        $ApprovedConflictsSha256 -cnotmatch '^[0-9a-f]{64}$') {
        throw 'V2 requires independent accepted discovery SHA-256 hashes.'
    }

    $root = [IO.Path]::GetFullPath($repositoryRoot)
    $resolvedManifest = [IO.Path]::GetFullPath($ManifestPath)
    $prefix = $root.TrimEnd([IO.Path]::DirectorySeparatorChar) +
        [IO.Path]::DirectorySeparatorChar
    if (-not $resolvedManifest.StartsWith($prefix, [StringComparison]::OrdinalIgnoreCase)) {
        throw 'V2 candidate manifest escaped the repository root.'
    }
    $manifestRelative = $resolvedManifest.Substring($prefix.Length).Replace('\', '/')
    if ($manifestRelative -cnotmatch '^docs/upstream-candidates/[A-Za-z0-9_.-]+\.json$') {
        throw 'V2 candidate manifest path is unsafe.'
    }
    if (-not (Test-Path -LiteralPath $resolvedManifest -PathType Leaf)) {
        throw 'V2 candidate manifest was not found.'
    }
    $manifest = Get-Content -LiteralPath $resolvedManifest -Raw |
        ConvertFrom-Json -ErrorAction Stop
    if ($manifest.contract_version -ne 2 -or
        $manifest.state -ne 'manual_conflict_reconciled_review_required' -or
        $manifest.transplant_dry_run -ne 'conflict') {
        throw 'V2 native gate rejects v1, unresolved or falsely clean manifests.'
    }

    $candidateBranch = (& git branch --show-current).Trim()
    $candidateSha = (& git rev-parse HEAD).Trim()
    if ($LASTEXITCODE -ne 0 -or $candidateBranch -ne $manifest.candidate_branch) {
        throw 'Local branch is not the canonical v2 candidate.'
    }
    if ($candidateSha -cnotmatch '^[0-9a-f]{40}$') {
        throw 'Candidate HEAD SHA invalid.'
    }
    if (@(& git status --porcelain).Count -ne 0 -or $LASTEXITCODE -ne 0) {
        throw 'Candidate worktree must be clean.'
    }

    $cli = Join-Path $repositoryRoot '.github\scripts\upstream_reconciliation_cli.py'
    $python = Resolve-CodexDDPythonPath -Candidates @(
        Get-Command python -All -CommandType Application -ErrorAction Stop
    )
    $preflightArgs = @(
        $cli, '--repo', $repositoryRoot, '--mode', 'native',
        '--manifest', $manifestRelative,
        '--production-sha', $manifest.production_sha,
        '--source-sha', $manifest.source_snapshot_commit_sha,
        '--source-tree', $manifest.source_snapshot_tree_sha,
        '--target-tag', $manifest.target_upstream_tag,
        '--target-sha', $manifest.target_upstream_sha,
        '--approved-manifest-sha256', $ApprovedDiscoveryManifestSha256,
        '--approved-conflicts-sha256', $ApprovedConflictsSha256
    )
    & $python @preflightArgs
    if ($LASTEXITCODE -ne 0) {
        throw 'Read-only blocked-discovery and Git preflight failed.'
    }

    $releaseScript = Join-Path $PSScriptRoot 'codexdd-test-release.ps1'
    $profileOutput = @(
        & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $releaseScript 2>&1
    )
    $profileExitCode = $LASTEXITCODE
    $endEvents = @()
    foreach ($item in $profileOutput) {
        $line = $item.ToString()
        [Console]::Out.WriteLine($line)
        if ($line.StartsWith('CODEXDD_VALIDATION_JSON ')) {
            $text = $line.Substring('CODEXDD_VALIDATION_JSON '.Length)
            $parsed = $text | ConvertFrom-Json -ErrorAction Stop
            if ($parsed.event -eq 'profile_end') {
                $endEvents += $parsed
            }
        }
    }
    if ($profileExitCode -ne 0 -or $endEvents.Count -ne 1) {
        throw 'Native release failed or emitted ambiguous terminal evidence.'
    }
    $profileEnd = $endEvents[0]
    if ($profileEnd.profile -ne 'release' -or $profileEnd.status -ne 'pass' -or
        $profileEnd.exit_code -ne 0 -or $profileEnd.completed_stages -lt 21) {
        throw 'Native complete release-profile PASS not established.'
    }

    if ((& git rev-parse HEAD).Trim() -ne $candidateSha -or
        @(& git status --porcelain).Count -ne 0) {
        throw 'Candidate changed during native release validation.'
    }
    & $python @preflightArgs
    if ($LASTEXITCODE -ne 0) {
        throw 'Candidate or upstream provenance changed during release validation.'
    }

    $hash = (Get-FileHash -LiteralPath $resolvedManifest -Algorithm SHA256).Hash.ToLowerInvariant()
    $receiptArgs = @{
        Manifest = $manifest
        ProfileEnd = $profileEnd
        CandidateBranch = $candidateBranch
        CandidateHeadSha = $candidateSha
        ManifestPath = $manifestRelative
        ManifestSha256 = $hash
        NativeHost = $env:COMPUTERNAME
    }
    $receipt = New-CodexDDV2ReleaseReceipt @receiptArgs
    $json = ConvertTo-CodexDDV2CanonicalReceiptJson -Receipt $receipt
    $receiptDir = Join-Path $env:TEMP 'codexdd\upstream-validation'
    New-Item -Path $receiptDir -ItemType Directory -Force | Out-Null
    $receiptPath = Join-Path $receiptDir "$candidateSha-release-v2.json"
    [IO.File]::WriteAllText($receiptPath, $json, (New-Object Text.UTF8Encoding($false)))
    [Console]::Out.WriteLine('CODEXDD_UPSTREAM_VALIDATION_RECEIPT ' + $json.TrimEnd([char]10))
    [Console]::Out.WriteLine('CODEXDD_UPSTREAM_VALIDATION_RECEIPT_PATH ' + $receiptPath)
    exit 0
}
catch {
    [Console]::Error.WriteLine('CODEXDD_UPSTREAM_VALIDATION_ERROR ' + $_.Exception.Message)
    exit 2
}
finally {
    Pop-Location
}
