[CmdletBinding()]
param(
    # Only the interactive preflight may download. The default is offline verification.
    [switch]$Prepare
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

# Code-mode V8 requires pointer compression and sandbox. The matching
# Windows archive is published by OpenAI Codex, not upstream denoland.
$repositoryRoot = Split-Path -Parent $PSScriptRoot
$codexRsRoot = Join-Path $repositoryRoot "codex-rs"
$workspaceManifest = Get-Content -LiteralPath (Join-Path $codexRsRoot "Cargo.toml") -Raw
$matchesVersion = [regex]::Matches(
    $workspaceManifest,
    '(?m)^[ \t]*v8[ \t]*=[ \t]*"=([0-9]+\.[0-9]+\.[0-9]+)"[ \t]*\r?$'
)
if ($matchesVersion.Count -ne 1) {
    throw "Expected exactly one pinned v8 workspace dependency in codex-rs/Cargo.toml."
}
$version = $matchesVersion[0].Groups[1].Value

$arch = [string]$env:PROCESSOR_ARCHITECTURE
switch ($arch.ToUpperInvariant()) {
    "AMD64" { $target = "x86_64-pc-windows-msvc" }
    "ARM64" { $target = "aarch64-pc-windows-msvc" }
    default { throw "Unsupported Windows release host architecture: $arch" }
}

$profile = "ptrcomp_sandbox_release"
$archiveName = "rusty_v8_{0}_{1}.lib.gz" -f $profile, $target
$bindingName = "src_binding_{0}_{1}.rs" -f $profile, $target
$checksumsName = "rusty_v8_{0}_{1}.sha256" -f $profile, $target

$trustedManifest = Join-Path $repositoryRoot (
    "third_party\v8\rusty_v8_{0}_release_manifests.sha256" -f $version.Replace(".", "_")
)
if (-not (Test-Path -LiteralPath $trustedManifest -PathType Leaf)) {
    throw "The repository lacks the trusted Rusty V8 manifest for version $version."
}
$trustedContent = Get-Content -LiteralPath $trustedManifest -Raw
$trustedPattern = '(?m)^([0-9a-fA-F]{64})[ \t]+' +
    [regex]::Escape($checksumsName) + '[ \t]*\r?$'
$trustedMatches = [regex]::Matches($trustedContent, $trustedPattern)
if ($trustedMatches.Count -ne 1) {
    throw "Trusted manifest must contain exactly one entry for $checksumsName."
}
$trustedDigest = $trustedMatches[0].Groups[1].Value.ToLowerInvariant()

Push-Location -LiteralPath $codexRsRoot
try {
    $metadataJson = & cargo metadata --locked --no-deps --format-version 1
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo metadata failed with exit code $LASTEXITCODE."
    }
    $metadata = ($metadataJson -join [Environment]::NewLine) | ConvertFrom-Json
    $targetDir = [string]$metadata.target_directory
    if (-not [System.IO.Path]::IsPathRooted($targetDir)) {
        throw "Cargo did not resolve an absolute target directory."
    }
}
finally {
    Pop-Location
}

# Cargo's target directory is accessible to the interactive preflight and
# detached S4U task (unlike a per-session TEMP directory).
$cacheDir = Join-Path (Join-Path (Join-Path $targetDir "codexdd-v8-artifacts") "v$version") $target
$baseUrl = "https://github.com/openai/codex/releases/download/rusty-v8-v$version"
if ($Prepare) {
    New-Item -ItemType Directory -Path $cacheDir -Force | Out-Null
}

function Assert-VerifiedFile {
    param(
        [Parameter(Mandatory=$true)][string]$Path,
        [Parameter(Mandatory=$true)][string]$Sha256,
        [Parameter(Mandatory=$true)][string]$Url,
        [Parameter(Mandatory=$true)][bool]$CanDownload
    )

    if (Test-Path -LiteralPath $Path -PathType Leaf) {
        $actual = (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($actual -eq $Sha256.ToLowerInvariant()) {
            Write-Host "Verified $([System.IO.Path]::GetFileName($Path))"
            return
        }
        if (-not $CanDownload) {
            throw "Checksum mismatch for $Path. Run codexdd-prepare-rusty-v8.ps1 -Prepare."
        }
    }
    if (-not $CanDownload) {
        throw "Missing verified V8 artifact $Path. Run codexdd-prepare-rusty-v8.ps1 -Prepare before the detached release task."
    }

    $temporary = "$Path.download-$PID"
    try {
        Write-Host "Downloading $Url"
        Invoke-WebRequest -Uri $Url -OutFile $temporary -UseBasicParsing -MaximumRedirection 10
        $digest = (Get-FileHash -LiteralPath $temporary -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($digest -ne $Sha256.ToLowerInvariant()) {
            throw "Downloaded V8 asset checksum mismatch: $Url"
        }
        Move-Item -LiteralPath $temporary -Destination $Path -Force
        Write-Host "Verified $([System.IO.Path]::GetFileName($Path))"
    }
    finally {
        if (Test-Path -LiteralPath $temporary) {
            Remove-Item -LiteralPath $temporary -Force
        }
    }
}

$checksumsPath = Join-Path $cacheDir $checksumsName
Assert-VerifiedFile -Path $checksumsPath -Sha256 $trustedDigest -Url "$baseUrl/$checksumsName" -CanDownload ([bool]$Prepare)

$checksums = @{}
$checksumLines = @(Get-Content -LiteralPath $checksumsPath | Where-Object { $_.Trim() -ne "" })
if ($checksumLines.Count -ne 2) {
    throw "Expected exactly two checksums in $checksumsPath."
}
foreach ($line in $checksumLines) {
    $match = [regex]::Match($line, '^([a-fA-F0-9]{64})[ \t]+\*?([a-zA-Z0-9_.-]+)[ \t]*$')
    if (-not $match.Success) {
        throw "Malformed V8 release checksum line: $line"
    }
    $filename = $match.Groups[2].Value
    if ($checksums.ContainsKey($filename)) {
        throw "Duplicate V8 release checksum for $filename."
    }
    $checksums[$filename] = $match.Groups[1].Value.ToLowerInvariant()
}
if (-not $checksums.ContainsKey($archiveName) -or -not $checksums.ContainsKey($bindingName)) {
    throw "V8 release checksum file does not match the expected archive and bindings."
}

$archivePath = Join-Path $cacheDir $archiveName
$bindingPath = Join-Path $cacheDir $bindingName
Assert-VerifiedFile -Path $archivePath -Sha256 $checksums[$archiveName] -Url "$baseUrl/$archiveName" -CanDownload ([bool]$Prepare)
Assert-VerifiedFile -Path $bindingPath -Sha256 $checksums[$bindingName] -Url "$baseUrl/$bindingName" -CanDownload ([bool]$Prepare)

# The release stage verifies again and sets its own child-process env vars.
[pscustomobject]@{
    Version = $version
    Target = $target
    Archive = $archivePath
    Binding = $bindingPath
}
