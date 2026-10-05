[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

. (Join-Path $PSScriptRoot "codexdd-validation-common.ps1")

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$stages = @(
    (New-CodexDDValidationStage -Name "diff-check" -FilePath "git" -ArgumentList @("diff", "--check") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "format-check" -FilePath "just" -ArgumentList @("fmt-check") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "core-tui-clippy" -FilePath "just" -ArgumentList @("clippy", "-p", "codex-core", "-p", "codex-tui") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "core-adaptive-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-core", "adaptive", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "tui-adaptive-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-tui", "adaptive", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "cli-build" -FilePath "cargo" -ArgumentList @("build", "--manifest-path", "codex-rs/Cargo.toml", "-p", "codex-cli", "--locked") -WorkingDirectory $repositoryRoot)
)

$exitCode = Invoke-CodexDDValidationProfile -Profile "work-packet" -RepositoryRoot $repositoryRoot -Stages $stages
exit $exitCode
