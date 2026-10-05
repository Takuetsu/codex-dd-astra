[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

. (Join-Path $PSScriptRoot "codexdd-validation-common.ps1")

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$codexRsRoot = Join-Path $repositoryRoot "codex-rs"

$env:CODEX_REPO_ROOT = $repositoryRoot
$env:RUST_MIN_STACK = "8388608"
$env:NEXTEST_PROFILE = "local"

$stages = @(
    (New-CodexDDValidationStage -Name "diff-check" -FilePath "git" -ArgumentList @("diff", "--check") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "format-check" -FilePath "python" -ArgumentList @("..\scripts\format.py", "--check") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "core-tui-clippy" -FilePath "cargo" -ArgumentList @("clippy", "--tests", "-p", "codex-core", "-p", "codex-tui") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "core-adaptive-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-core", "--lib", "adaptive") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "tui-adaptive-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-tui", "--lib", "adaptive") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "cli-build" -FilePath "cargo" -ArgumentList @("build", "-p", "codex-cli", "--locked") -WorkingDirectory $codexRsRoot)
)

$exitCode = Invoke-CodexDDValidationProfile -Profile "work-packet" -RepositoryRoot $repositoryRoot -Stages $stages
exit $exitCode
