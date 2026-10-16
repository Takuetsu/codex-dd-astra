[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# One atomic release stage: build the repository-owned MCP fixture first, then
# run the app-server library tests. Do not count missing fixture setup as a
# product regression or silently skip the affected test.
$codexRsRoot = Join-Path (Split-Path -Parent $PSScriptRoot) "codex-rs"
Push-Location -LiteralPath $codexRsRoot
try {
    $metadataJson = & cargo metadata --locked --no-deps --format-version 1
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo metadata failed with exit code $LASTEXITCODE."
    }
    $metadata = $metadataJson | ConvertFrom-Json
    $targetDir = [string]$metadata.target_directory
    if ([string]::IsNullOrWhiteSpace($targetDir) -or -not [System.IO.Path]::IsPathRooted($targetDir)) {
        throw "Cargo metadata did not return an absolute target directory."
    }

    Write-Host "Preparing app-server MCP test helper (test_stdio_server)."
    & cargo build --locked -p codex-rmcp-client --bin test_stdio_server
    $buildExit = [int]$LASTEXITCODE
    if ($buildExit -ne 0) {
        Write-Host "MCP helper build failed with exit code $buildExit."
        exit $buildExit
    }

    # Cargo may use a shared/custom CARGO_TARGET_DIR, so resolve the helper
    # using Cargo's actual target directory rather than the source worktree.
    $helperPath = Join-Path $targetDir "debug\test_stdio_server.exe"
    if (-not (Test-Path -LiteralPath $helperPath -PathType Leaf)) {
        throw "MCP helper build completed but executable is missing: $helperPath"
    }

    # core_test_support::stdio_server_bin() checks this exact env var.
    $env:CARGO_BIN_EXE_test_stdio_server = $helperPath
    Write-Host "Using MCP fixture: $helperPath"
    & cargo nextest run --no-fail-fast -p codex-app-server --lib
    exit ([int]$LASTEXITCODE)
}
catch {
    [Console]::Error.WriteLine("App-server release stage preparation error: $($_.Exception.Message)")
    exit 2
}
finally {
    Pop-Location
}
