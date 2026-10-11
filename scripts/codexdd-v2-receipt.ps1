# Standalone native v2 receipt serialization. Does not enable release/promotion.
Set-StrictMode -Version Latest

function New-CodexDDV2ReleaseReceipt {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]$Manifest,
        [Parameter(Mandatory = $true)]$ProfileEnd,
        [Parameter(Mandatory = $true)][string]$CandidateBranch,
        [Parameter(Mandatory = $true)][string]$CandidateHeadSha,
        [Parameter(Mandatory = $true)][string]$ManifestPath,
        [Parameter(Mandatory = $true)][string]$ManifestSha256,
        [Parameter(Mandatory = $true)][string]$NativeHost
    )

    if (
        $Manifest.contract_version -ne 2 -or
        $Manifest.state -ne 'manual_conflict_reconciled_review_required' -or
        $Manifest.transplant_dry_run -ne 'conflict'
    ) {
        throw 'Receipt v2 requires a reconciled conflict-state v2 manifest.'
    }
    if ($CandidateBranch -ne $Manifest.candidate_branch) {
        throw 'Candidate branch differs from canonical v2 manifest branch.'
    }
    if ($CandidateHeadSha -cnotmatch '^[0-9a-f]{40}$') {
        throw 'Invalid exact candidate HEAD commit.'
    }
    if ($ManifestPath -cnotmatch '^docs/upstream-candidates/[A-Za-z0-9_.-]+\.json$') {
        throw 'Noncanonical v2 manifest path.'
    }
    if ($ManifestSha256 -cnotmatch '^[0-9a-f]{64}$') {
        throw 'Invalid v2 candidate manifest SHA-256.'
    }
    if ($Manifest.sidecar_path -cnotmatch '^docs/upstream-candidates/[A-Za-z0-9_.-]+\.json$') {
        throw 'Noncanonical v2 sidecar path.'
    }
    if ($Manifest.sidecar_sha256 -cnotmatch '^[0-9a-f]{64}$') {
        throw 'Invalid v2 sidecar SHA-256.'
    }
    if ($NativeHost -ine 'Daniel-CL') {
        throw 'Native v2 release receipt is restricted to Daniel-CL.'
    }
    if (
        $ProfileEnd.contract_version -ne 1 -or
        $ProfileEnd.event -ne 'profile_end' -or
        $ProfileEnd.profile -ne 'release' -or
        $ProfileEnd.status -ne 'pass' -or
        $ProfileEnd.exit_code -ne 0 -or
        $null -eq $ProfileEnd.completed_stages -or
        $ProfileEnd.completed_stages -lt 21
    ) {
        throw 'Complete native release-profile PASS required for v2 receipt.'
    }

    # Keys are lexicographically ordered to match canonical Python JSON.
    # Values are fixed-schema ASCII primitives.
    return [ordered]@{
        candidate_branch = $CandidateBranch
        candidate_head_sha = $CandidateHeadSha
        candidate_key = $Manifest.candidate_key
        completed_at = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffffffZ', [Globalization.CultureInfo]::InvariantCulture)
        completed_stages = [int]$ProfileEnd.completed_stages
        contract_version = 2
        manifest_path = $ManifestPath
        manifest_sha256 = $ManifestSha256
        native_platform = 'windows'
        native_validation_host = 'Daniel-CL'
        production_sha = $Manifest.production_sha
        profile = 'release'
        receipt_type = 'codexdd_upstream_local_validation'
        reconciliation_sidecar_path = $Manifest.sidecar_path
        reconciliation_sidecar_sha256 = $Manifest.sidecar_sha256
        source_snapshot_commit_sha = $Manifest.source_snapshot_commit_sha
        source_snapshot_tree_sha = $Manifest.source_snapshot_tree_sha
        target_upstream_sha = $Manifest.target_upstream_sha
        target_upstream_tag = $Manifest.target_upstream_tag
        validation_contract_version = 1
        validation_status = 'pass'
    }
}

function ConvertTo-CodexDDV2CanonicalReceiptJson {
    [CmdletBinding()]
    param([Parameter(Mandatory = $true)]$Receipt)

    $json = ConvertTo-Json -InputObject $Receipt -Compress -Depth 8
    if ([string]::IsNullOrWhiteSpace($json)) {
        throw 'Native v2 receipt serializer produced empty JSON.'
    }
    return $json + [char]10
}
