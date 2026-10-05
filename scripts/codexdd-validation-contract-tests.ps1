[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

. (Join-Path $PSScriptRoot "codexdd-validation-common.ps1")

$repositoryRoot = Split-Path -Parent $PSScriptRoot

$passStages = @(
    (New-CodexDDValidationStage -Name "pass-stage" -FilePath "cmd.exe" -ArgumentList @("/d", "/c", "exit", "0") -WorkingDirectory $repositoryRoot)
)
$passExit = Invoke-CodexDDValidationProfile -Profile "targeted" -RepositoryRoot $repositoryRoot -Stages $passStages
if ($passExit -ne 0) {
    throw "Expected successful contract fixture to exit 0, got $passExit."
}

$failStages = @(
    (New-CodexDDValidationStage -Name "fail-stage" -FilePath "cmd.exe" -ArgumentList @("/d", "/c", "exit", "7") -WorkingDirectory $repositoryRoot)
)
$failExit = Invoke-CodexDDValidationProfile -Profile "targeted" -RepositoryRoot $repositoryRoot -Stages $failStages
if ($failExit -ne 1) {
    throw "Expected validation-failure fixture to exit 1, got $failExit."
}

$errorStages = @(
    (New-CodexDDValidationStage -Name "error-stage" -FilePath "__codexdd_missing_executable__" -ArgumentList @() -WorkingDirectory $repositoryRoot)
)
$errorExit = Invoke-CodexDDValidationProfile -Profile "targeted" -RepositoryRoot $repositoryRoot -Stages $errorStages
if ($errorExit -ne 2) {
    throw "Expected infrastructure-error fixture to exit 2, got $errorExit."
}

[Console]::Out.WriteLine("CODEXDD_VALIDATION_CONTRACT_TEST PASS")
exit 0
