param(
    [Parameter(Mandatory = $true)]
    [string]$BinaryPath,

    [Parameter(Mandatory = $true)]
    [string]$VersionFile,

    [Parameter(Mandatory = $true)]
    [string]$OutputDirectory,

    [string]$ExpectedTag = ""
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if (-not (Test-Path -LiteralPath $BinaryPath -PathType Leaf)) {
    throw "codexdd binary not found: $BinaryPath"
}

if (-not (Test-Path -LiteralPath $VersionFile -PathType Leaf)) {
    throw "codexdd version file not found: $VersionFile"
}

$version = (Get-Content -LiteralPath $VersionFile -Raw).Trim()
if ([string]::IsNullOrWhiteSpace($version)) {
    throw "codexdd version file is empty: $VersionFile"
}

if (-not [string]::IsNullOrWhiteSpace($ExpectedTag)) {
    $requiredTag = "codexdd-v$version"
    if ($ExpectedTag -ne $requiredTag) {
        throw "release tag '$ExpectedTag' does not match codexdd version '$version'; expected '$requiredTag'"
    }
}

$resolvedBinary = (Resolve-Path -LiteralPath $BinaryPath).Path
$reportedVersion = (& $resolvedBinary --version | Out-String).Trim()
if ($LASTEXITCODE -ne 0) {
    throw "codexdd --version failed with exit code $LASTEXITCODE"
}

$escapedVersion = [regex]::Escape($version)
if ($reportedVersion -notmatch "^codexdd $escapedVersion\+g[0-9a-f]{12}$") {
    throw "unexpected codexdd version output: '$reportedVersion'"
}

$packageName = "codexdd-windows-x86_64-$version"
$packageDirectory = Join-Path $OutputDirectory $packageName
$zipPath = Join-Path $OutputDirectory "$packageName.zip"
$checksumPath = "$zipPath.sha256"

New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
Remove-Item -LiteralPath $packageDirectory -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -LiteralPath $zipPath -Force -ErrorAction SilentlyContinue
Remove-Item -LiteralPath $checksumPath -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $packageDirectory | Out-Null

Copy-Item -LiteralPath $resolvedBinary -Destination (Join-Path $packageDirectory "codex.exe")

$sourceCommit = if ($env:GITHUB_SHA) {
    $env:GITHUB_SHA
} else {
    (git rev-parse HEAD).Trim()
}

@"
codexdd $version
$reportedVersion
Source commit: $sourceCommit
Target: Windows x86_64
"@ | Set-Content -LiteralPath (Join-Path $packageDirectory "VERSION.txt") -Encoding utf8NoBOM

@"
codexdd Windows release package

Contents
--------
codex.exe   The codexdd Windows runtime.
VERSION.txt Product version, stamped build identity, source commit, and target.

Deployment
----------
1. Stop any running codexdd sessions that use the runtime being replaced.
2. Back up the currently deployed codex.exe if rollback is desired.
3. Replace the codex.exe used by your codexdd launcher with this package's codex.exe.
4. Run `codexdd --version` and confirm it reports the expected product version and build commit.

The executable intentionally retains the filename codex.exe because the existing Windows codexdd launchers target that runtime filename.
"@ | Set-Content -LiteralPath (Join-Path $packageDirectory "INSTALL.txt") -Encoding utf8NoBOM

Compress-Archive -Path (Join-Path $packageDirectory "*") -DestinationPath $zipPath -CompressionLevel Optimal

$hash = (Get-FileHash -LiteralPath $zipPath -Algorithm SHA256).Hash.ToLowerInvariant()
$zipName = [IO.Path]::GetFileName($zipPath)
"$hash  $zipName" | Set-Content -LiteralPath $checksumPath -Encoding ascii

if ($env:GITHUB_OUTPUT) {
    "version=$version" | Out-File -FilePath $env:GITHUB_OUTPUT -Encoding utf8 -Append
    "artifact_name=$packageName" | Out-File -FilePath $env:GITHUB_OUTPUT -Encoding utf8 -Append
    "zip_name=$zipName" | Out-File -FilePath $env:GITHUB_OUTPUT -Encoding utf8 -Append
}

Write-Host "Packaged $reportedVersion"
Write-Host "Archive: $zipPath"
Write-Host "SHA256: $hash"
