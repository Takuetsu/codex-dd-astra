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
    (New-CodexDDValidationStage -Name "rust-format-check" -FilePath "cargo" -ArgumentList @("fmt", "--", "--config", "imports_granularity=Item", "--check") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "workspace-clippy" -FilePath "cargo" -ArgumentList @("clippy", "--tests", "--workspace") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "history-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-history", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "rollout-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-rollout", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "protocol-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-protocol", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "state-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-state", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "thread-store-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-thread-store", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "core-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-core", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "app-server-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-app-server", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "tui-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-tui", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "cli-release-build" -FilePath "cargo" -ArgumentList @("build", "-p", "codex-cli", "--release", "--locked") -WorkingDirectory $codexRsRoot)
)

$exitCode = Invoke-CodexDDValidationProfile -Profile "release" -RepositoryRoot $repositoryRoot -Stages $stages
exit $exitCode
