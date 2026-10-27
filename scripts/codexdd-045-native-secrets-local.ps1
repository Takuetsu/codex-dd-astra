# CodexDD 0.4.5: cheap, exact-lock native Windows secrets regression.
# Called from a detached worktree under E:\codexdd. No source edits or release actions.
$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot
$manifestPath = Join-Path $repoRoot "codex-rs\Cargo.toml"
$lockPath = Join-Path $repoRoot "codex-rs\Cargo.lock"
$manifestBlob = "d46347640b6ca6e43e96a12ee14eec894178af4f"
$lockBlob = "f19b0086eac09c6830c887be83db5ae4ae11587e"
$timer = [System.Diagnostics.Stopwatch]::StartNew()
$head = ""
try {
    $head = (& git -C $repoRoot rev-parse HEAD | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or -not $head) { throw "Cannot identify detached test commit" }
    $sourceStatus = (& git -C $repoRoot status --porcelain | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $sourceStatus) { throw "Test worktree has unexpected edits" }
    $actualManifest = (& git -C $repoRoot hash-object $manifestPath | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $actualManifest -ne $manifestBlob) {
        throw "Manifest hash mismatch: $actualManifest"
    }
    $actualLock = (& git -C $repoRoot hash-object $lockPath | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or $actualLock -ne $lockBlob) {
        throw "Cargo.lock hash mismatch: $actualLock"
    }
    Write-Output "CODEXDD_045_SECRETS_NATIVE: start"
    Write-Output "HEAD: $head"
    Write-Output "CARGO_MANIFEST_GIT_BLOB: $actualManifest"
    Write-Output "CARGO_LOCK_GIT_BLOB: $actualLock"
    Write-Output "TEST: codex-secrets --lib --locked, Rust 1.95.0, Windows native"
    & rustup run 1.95.0 cargo test --manifest-path $manifestPath --locked -p codex-secrets --lib
    if ($LASTEXITCODE -ne 0) { throw "Focused codex-secrets native test failed with exit $LASTEXITCODE" }
    Write-Output "CODEXDD_045_SECRETS_NATIVE: GREEN"
    exit 0
} catch {
    Write-Output "CODEXDD_045_SECRETS_NATIVE: RED - $($_.Exception.Message)"
    exit 1
} finally {
    $timer.Stop()
    Write-Output ("ELAPSED_SECONDS: {0:N1}" -f $timer.Elapsed.TotalSeconds)
    Write-Output "COMMIT: $head"
    Write-Output ("FINISHED_UTC: " + [DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ssZ"))
}
