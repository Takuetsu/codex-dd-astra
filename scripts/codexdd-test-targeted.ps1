[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

. (Join-Path $PSScriptRoot "codexdd-validation-common.ps1")

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$stages = @(
    (New-CodexDDValidationStage -Name "diff-check" -FilePath "git" -ArgumentList @("diff", "--check") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "core-adaptive-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-core", "--lib", "adaptive") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "tui-adaptive-tests" -FilePath "just" -ArgumentList @("test", "-p", "codex-tui", "--lib", "adaptive") -WorkingDirectory $repositoryRoot)
)

$exitCode = Invoke-CodexDDValidationProfile -Profile "targeted" -RepositoryRoot $repositoryRoot -Stages $stages
exit $exitCode
