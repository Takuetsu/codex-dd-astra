[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

. (Join-Path $PSScriptRoot "codexdd-validation-common.ps1")

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$scriptPath = Join-Path $PSScriptRoot "codexdd-test-independent.ps1"
if (-not (Test-Path -LiteralPath $scriptPath -PathType Leaf)) {
    throw "Independent Validator native profile is absent."
}

# Parse the actual repo-owned profile, not a stand-in string.
$tokens = $null
$parseErrors = $null
$null = [System.Management.Automation.Language.Parser]::ParseFile(
    $scriptPath, [ref]$tokens, [ref]$parseErrors
)
if (@($parseErrors).Count -ne 0) {
    throw "Independent Validator script has PowerShell syntax errors: $parseErrors"
}

$source = Get-Content -LiteralPath $scriptPath -Raw
if (-not $source.Contains('Invoke-CodexDDValidationProfile -Profile "independent-review"')) {
    throw "Independent Validator script does not use the fixed independent-review profile."
}
if ($source -match 'Invoke-Expression|Start-Process|Invoke-RestMethod|Invoke-WebRequest') {
    throw "Independent Validator profile contains an unapproved dynamic execution/network command."
}
$expectedStages = @(
    "review-diff-check",
    "review-rust-format",
    "review-core-adaptive",
    "review-tui-adaptive",
    "review-core-receipt-regressions",
    "review-tui-handoff-regressions"
)
$stageMatches = [regex]::Matches($source, 'New-CodexDDValidationStage\s+-Name\s+"([^"]+)"')
$stageNames = @($stageMatches | ForEach-Object { $_.Groups[1].Value })
if ($stageNames.Count -ne $expectedStages.Count) {
    throw "Independent Validator must define exactly $($expectedStages.Count) fixed stages; found $($stageNames.Count)."
}
for ($index = 0; $index -lt $expectedStages.Count; $index++) {
    if ($stageNames[$index] -cne $expectedStages[$index]) {
        throw "Unexpected independent review stage at $index."
    }
}

# Fast contract fixtures exercise the real PowerShell event emitter, not
# mock receipts accepted by Core. All are NONPRODUCTION tests.
$passStages = @(
    (New-CodexDDValidationStage -Name "fixture-pass" -FilePath "cmd.exe" -ArgumentList @("/d", "/c", "exit /b 0") -WorkingDirectory $repositoryRoot)
)
$pass = Invoke-CodexDDValidationProfile -Profile "independent-review" -RepositoryRoot $repositoryRoot -Stages $passStages
if ($pass -ne 0) {
    throw "Independent-review fixture PASS returned $pass instead of 0."
}

$failStages = @(
    (New-CodexDDValidationStage -Name "fixture-fail" -FilePath "cmd.exe" -ArgumentList @("/d", "/c", "exit /b 7") -WorkingDirectory $repositoryRoot)
)
$fail = Invoke-CodexDDValidationProfile -Profile "independent-review" -RepositoryRoot $repositoryRoot -Stages $failStages
if ($fail -ne 1) {
    throw "Independent-review fixture FAIL returned $fail instead of 1."
}

$errorStages = @(
    (New-CodexDDValidationStage -Name "fixture-error" -FilePath "__codexdd_missing_review_executable__" -ArgumentList @() -WorkingDirectory $repositoryRoot)
)
$error = Invoke-CodexDDValidationProfile -Profile "independent-review" -RepositoryRoot $repositoryRoot -Stages $errorStages
if ($error -ne 2) {
    throw "Independent-review fixture ERROR returned $error instead of 2."
}

Write-Host "CODEXDD_046_3C1B2D_INDEPENDENT_PROFILE_CONTRACT PASS"
exit 0
