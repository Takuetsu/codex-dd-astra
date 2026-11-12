[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# A separate read-only review profile. Core alone binds the Validation role,
# exact clean SHA/worktree, task scope and native run ID; this script provides
# independently executed objective tests, never a READY or owner approval.
. (Join-Path $PSScriptRoot "codexdd-validation-common.ps1")

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$codexRsRoot = Join-Path $repositoryRoot "codex-rs"

$env:CODEX_REPO_ROOT = $repositoryRoot
$env:RUST_MIN_STACK = "16777216"
$env:NEXTEST_PROFILE = "local"

$stages = @(
    (New-CodexDDValidationStage -Name "review-diff-check" -FilePath "git" -ArgumentList @("diff", "--check") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "review-rust-format" -FilePath "cargo" -ArgumentList @("fmt", "--all", "--", "--check") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "review-core-adaptive" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-core", "--lib", "adaptive") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "review-tui-adaptive" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-tui", "--lib", "adaptive") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "review-core-receipt-regressions" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-core", "--lib", "codexdd_046_3c1b2b_") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "review-tui-handoff-regressions" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-tui", "--lib", "codexdd_046_3c1b2c_") -WorkingDirectory $codexRsRoot)
)

$exitCode = Invoke-CodexDDValidationProfile -Profile "independent-review" -RepositoryRoot $repositoryRoot -Stages $stages
exit $exitCode
