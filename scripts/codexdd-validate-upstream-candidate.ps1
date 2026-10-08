[CmdletBinding()]
param(
    [ValidateSet("work-packet", "release")]
    [string]$Profile = "work-packet",

    [string]$ManifestPath = ""
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repositoryRoot = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $repositoryRoot
try {
    $branchName = (& git branch --show-current).Trim()
    if ($LASTEXITCODE -ne 0 -or [string]::IsNullOrWhiteSpace($branchName)) {
        throw "Upstream candidate validation requires a named local branch."
    }

    $headSha = (& git rev-parse HEAD).Trim()
    if ($LASTEXITCODE -ne 0 -or $headSha -notmatch '^[0-9a-f]{40}$') {
        throw "Could not resolve the candidate HEAD SHA."
    }

    $status = @(& git status --porcelain)
    if ($LASTEXITCODE -ne 0) {
        throw "Could not inspect the candidate worktree."
    }
    if ($status.Count -ne 0) {
        throw "Candidate worktree must be clean before validation."
    }

    if ([string]::IsNullOrWhiteSpace($ManifestPath)) {
        $manifestDirectory = Join-Path $repositoryRoot "docs\upstream-candidates"
        if (-not (Test-Path -LiteralPath $manifestDirectory -PathType Container)) {
            throw "Candidate manifest directory was not found: $manifestDirectory"
        }

        $candidateManifests = @(
            Get-ChildItem -LiteralPath $manifestDirectory -Filter "codexdd-upstream-*.json" -File
        )
        $matching = @()
        foreach ($candidate in $candidateManifests) {
            try {
                $candidateJson = Get-Content -LiteralPath $candidate.FullName -Raw |
                    ConvertFrom-Json -ErrorAction Stop
                if (
                    $candidateJson.target_upstream_sha -and
                    $branchName -eq "automation/upstream-candidate-$($candidateJson.target_upstream_tag)-$($candidateJson.target_upstream_sha.Substring(0, 12))"
                ) {
                    $matching += $candidate.FullName
                }
            }
            catch {
                continue
            }
        }

        if ($matching.Count -ne 1) {
            throw "Expected exactly one committed manifest matching branch '$branchName'; found $($matching.Count)."
        }
        $resolvedManifestPath = $matching[0]
    }
    else {
        $resolvedManifestPath = [IO.Path]::GetFullPath(
            (Join-Path $repositoryRoot $ManifestPath)
        )
        if (-not (Test-Path -LiteralPath $resolvedManifestPath -PathType Leaf)) {
            throw "Candidate manifest was not found: $resolvedManifestPath"
        }
    }

    $manifest = Get-Content -LiteralPath $resolvedManifestPath -Raw |
        ConvertFrom-Json -ErrorAction Stop

    if ($manifest.contract_version -ne 1) {
        throw "Unsupported candidate manifest contract version: $($manifest.contract_version)"
    }
    if ($manifest.target_upstream_sha -notmatch '^[0-9a-f]{40}$') {
        throw "Candidate manifest target SHA is invalid."
    }
    if ($manifest.production_sha -notmatch '^[0-9a-f]{40}$') {
        throw "Candidate manifest production SHA is invalid."
    }

    $expectedBranch = "automation/upstream-candidate-$($manifest.target_upstream_tag)-$($manifest.target_upstream_sha.Substring(0, 12))"
    if ($branchName -ne $expectedBranch) {
        throw "Candidate branch mismatch: expected '$expectedBranch', found '$branchName'."
    }

    $trackedRelease = (
        Get-Content -LiteralPath (Join-Path $repositoryRoot "codex-rs\upstream-codex-release.txt") -Raw
    ).Trim()
    if ($trackedRelease -ne $manifest.target_upstream_tag) {
        throw "Candidate provenance mismatch: expected '$($manifest.target_upstream_tag)', found '$trackedRelease'."
    }

    & git merge-base --is-ancestor $manifest.production_sha $headSha
    if ($LASTEXITCODE -ne 0) {
        throw "Candidate HEAD is not descended from manifest production SHA $($manifest.production_sha)."
    }

    # Resolve canonical paths without APIs absent from Windows PowerShell 5.1.
    $canonicalRoot = [IO.Path]::GetFullPath($repositoryRoot)
    $canonicalManifest = [IO.Path]::GetFullPath($resolvedManifestPath)
    $repositoryPrefix = $canonicalRoot.TrimEnd(
        [IO.Path]::DirectorySeparatorChar
    ) + [IO.Path]::DirectorySeparatorChar
    if (-not $canonicalManifest.StartsWith(
        $repositoryPrefix,
        [StringComparison]::OrdinalIgnoreCase
    )) {
        throw "Candidate manifest is outside the repository root."
    }
    $manifestRelativePath = $canonicalManifest.Substring(
        $repositoryPrefix.Length
    ).Replace("\", "/")
    if (-not $manifestRelativePath.StartsWith(
        "docs/upstream-candidates/",
        [StringComparison]::Ordinal
    )) {
        throw "Candidate manifest must be under docs/upstream-candidates."
    }
    $manifestHash = (
        Get-FileHash -LiteralPath $resolvedManifestPath -Algorithm SHA256
    ).Hash.ToLowerInvariant()

    switch ($Profile) {
        "work-packet" {
            $profileScript = Join-Path $PSScriptRoot "codexdd-test-workpacket.ps1"
            $expectedProfileName = "work-packet"
        }
        "release" {
            $profileScript = Join-Path $PSScriptRoot "codexdd-test-release.ps1"
            $expectedProfileName = "release"
        }
    }

    $profileOutput = @(
        & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $profileScript 2>&1
    )
    $profileExitCode = $LASTEXITCODE
    $profileEnd = $null

    foreach ($lineObject in $profileOutput) {
        $line = $lineObject.ToString()
        [Console]::Out.WriteLine($line)
        if ($line.StartsWith("CODEXDD_VALIDATION_JSON ")) {
            $payloadText = $line.Substring("CODEXDD_VALIDATION_JSON ".Length)
            try {
                $payload = $payloadText | ConvertFrom-Json -ErrorAction Stop
                if (
                    $payload.event -eq "profile_end" -and
                    $payload.profile -eq $expectedProfileName
                ) {
                    $profileEnd = $payload
                }
            }
            catch {
                throw "Malformed validation event: $payloadText"
            }
        }
    }

    if ($profileExitCode -ne 0) {
        throw "Validation profile '$expectedProfileName' exited with code $profileExitCode."
    }
    if ($null -eq $profileEnd) {
        throw "Validation profile did not emit a matching profile_end event."
    }
    if ($profileEnd.status -ne "pass" -or $profileEnd.exit_code -ne 0) {
        throw "Validation profile did not produce a native PASS receipt."
    }

    $completedAt = [DateTime]::UtcNow.ToString("o")
    $candidateKey = "$($manifest.target_upstream_tag)@$($manifest.target_upstream_sha)"
    $receipt = [ordered]@{
        contract_version = 1
        receipt_type = "codexdd_upstream_local_validation"
        candidate_key = $candidateKey
        candidate_branch = $branchName
        candidate_head_sha = $headSha
        production_sha = $manifest.production_sha
        target_upstream_tag = $manifest.target_upstream_tag
        target_upstream_sha = $manifest.target_upstream_sha
        manifest_path = $manifestRelativePath
        manifest_sha256 = $manifestHash
        profile = $expectedProfileName
        validation_contract_version = $profileEnd.contract_version
        validation_status = $profileEnd.status
        completed_stages = $profileEnd.completed_stages
        completed_at = $completedAt
    }

    $receiptDirectory = Join-Path $env:TEMP "codexdd\upstream-validation"
    New-Item -ItemType Directory -Force -Path $receiptDirectory | Out-Null
    $receiptPath = Join-Path $receiptDirectory "$headSha-$expectedProfileName.json"
    $receiptJson = $receipt | ConvertTo-Json -Compress -Depth 8
    [IO.File]::WriteAllText($receiptPath, $receiptJson + [Environment]::NewLine)

    [Console]::Out.WriteLine("CODEXDD_UPSTREAM_VALIDATION_RECEIPT $receiptJson")
    [Console]::Out.WriteLine("CODEXDD_UPSTREAM_VALIDATION_RECEIPT_PATH $receiptPath")
    exit 0
}
catch {
    [Console]::Error.WriteLine("CODEXDD_UPSTREAM_VALIDATION_ERROR $($_.Exception.Message)")
    exit 2
}
finally {
    Pop-Location
}
