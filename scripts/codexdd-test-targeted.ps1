[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

. (Join-Path $PSScriptRoot "codexdd-validation-common.ps1")

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$stages = @(
    (New-CodexDDValidationStage -Name "diff-check" -FilePath "git" -ArgumentList @("diff", "--check") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "core-adaptive-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-core", "adaptive", "--lib") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "tui-adaptive-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-tui", "adaptive", "--lib") -WorkingDirectory $repositoryRoot)
)

$exitCode = Invoke-CodexDDValidationProfile -Profile "targeted" -RepositoryRoot $repositoryRoot -Stages $stages
exit $exitCode
