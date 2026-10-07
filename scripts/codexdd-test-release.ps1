[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

. (Join-Path $PSScriptRoot "codexdd-validation-common.ps1")

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$codexRsRoot = Join-Path $repositoryRoot "codex-rs"

$env:CODEX_REPO_ROOT = $repositoryRoot
$env:RUST_MIN_STACK = "16777216"
$env:NEXTEST_PROFILE = "local"

$stages = @(
    (New-CodexDDValidationStage -Name "diff-check" -FilePath "git" -ArgumentList @("diff", "--check") -WorkingDirectory $repositoryRoot),
    (New-CodexDDValidationStage -Name "rust-format-check" -FilePath "cargo" -ArgumentList @("fmt", "--", "--config", "imports_granularity=Item", "--check") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "core-tui-clippy" -FilePath "cargo" -ArgumentList @("clippy", "--tests", "-p", "codex-core", "-p", "codex-tui") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "history-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-history", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "rollout-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-rollout", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "protocol-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-protocol", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "state-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-state", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "thread-store-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-thread-store", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "core-adaptive-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-core", "--lib", "adaptive") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "core-rollout-reconstruction" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-core", "--lib", "rollout_reconstruction") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "core-workflow-snapshot" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-core", "--lib", "child_workflow_snapshot_preserves_adaptive_state") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "core-resume-state" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-core", "--lib", "resume_stopped_thread_from_rollout") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "core-resume-fork-store" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-core", "--lib", "rollout_path_resume_and_fork_read_history_through_thread_store") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "core-interrupted-fork" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-core", "--lib", "interrupted_fork_snapshot") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "app-server-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-app-server", "--lib") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "tui-adaptive-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-tui", "--lib", "adaptive") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "tui-session-state-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-tui", "--lib", "session_state::tests") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "tui-detached-fork-test" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-tui", "--lib", "detached_fork_restores_persisted_adaptive_workflow_state") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "tui-daemon-startup-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-tui", "--lib", "daemon_startup_tests") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "cli-daemon-startup-tests" -FilePath "cargo" -ArgumentList @("nextest", "run", "--no-fail-fast", "-p", "codex-cli", "--test", "daemon_startup") -WorkingDirectory $codexRsRoot),
    (New-CodexDDValidationStage -Name "cli-release-build" -FilePath "cargo" -ArgumentList @("build", "-p", "codex-cli", "--release", "--locked") -WorkingDirectory $codexRsRoot)
)

$exitCode = Invoke-CodexDDValidationProfile -Profile "release" -RepositoryRoot $repositoryRoot -Stages $stages
exit $exitCode
