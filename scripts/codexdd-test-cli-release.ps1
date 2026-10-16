[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$codexRsRoot = Join-Path $repositoryRoot "codex-rs"

Push-Location -LiteralPath $codexRsRoot
try {
    # Detached S4U jobs must not download GitHub release assets. The matching
    # Codex-built V8 archive and bindings are reverified in the local cache.
    $artifacts = & (Join-Path $PSScriptRoot "codexdd-prepare-rusty-v8.ps1")
    if ($null -eq $artifacts) {
        throw "Could not resolve verified Rusty V8 release artifacts."
    }
    $env:RUSTY_V8_ARCHIVE = [string]$artifacts.Archive
    $env:RUSTY_V8_SRC_BINDING_PATH = [string]$artifacts.Binding

    Write-Host "Using checksum-verified Codex V8 archive: $($artifacts.Archive)"
    Write-Host "Using checksum-verified Codex V8 binding: $($artifacts.Binding)"
    & cargo build -p codex-cli -p codex-code-mode-host --release --locked
    $buildExit = [int]$LASTEXITCODE
    if ($buildExit -ne 0) {
        Write-Host "CLI release build failed with exit code $buildExit."
    }
    exit $buildExit
}
catch {
    [Console]::Error.WriteLine("CLI release stage preparation error: $($_.Exception.Message)")
    exit 2
}
finally {
    Pop-Location
}
