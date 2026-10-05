[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

. (Join-Path $PSScriptRoot "codexdd-validation-common.ps1")

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$stages = @(
    (New-CodexDDValidationStage -Name "diff-check" -FilePath "git" -ArgumentList @("diff", "--check") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "format-check" -FilePath "just" -ArgumentList @("fmt-check") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "workspace-clippy" -FilePath "just" -ArgumentList @("clippy", "--workspace") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "history-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-history", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "rollout-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-rollout", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "protocol-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-protocol", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "state-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-state", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "thread-store-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-thread-store", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "core-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-core", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "app-server-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-app-server", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "tui-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-tui", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "cli-release-build" -FilePath "cargo" -ArgumentList @("build", "--manifest-path", "codex-rs/Cargo.toml", "-p", "codex-cli", "--release", "--locked") -WorkingDirectory $repositoryRoot)
)

$exitCode = Invoke-CodexDDValidationProfile -Profile "release" -RepositoryRoot $repositoryRoot -Stages $stages
exit $exitCode
