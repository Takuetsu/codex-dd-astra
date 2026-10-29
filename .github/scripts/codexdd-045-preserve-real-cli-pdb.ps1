# Nonrelease artifact salvage: MUST run on the very Windows runner that linked CLI.
# Never accepts a finished Bazel target without two physical authenticated outputs.
[CmdletBinding()]
param()
$ErrorActionPreference='Stop'
if (-not $env:GITHUB_SHA -or (git rev-parse HEAD).Trim() -ne $env:GITHUB_SHA) { throw 'Unfrozen CLI source SHA' }
if ((Get-Content 'codex-rs/codexdd-version.txt' -Raw).Trim() -ne '0.4.4') { throw 'Production marker promoted' }
$dir=Join-Path $env:RUNNER_TEMP 'codexdd045-release-staged'
$log=Join-Path $dir 'cli-combined.log'
$dest=Join-Path $dir 'nonrelease-artifacts'
if (-not (Test-Path -LiteralPath $log)) {
  Write-Warning 'No CLI native build log; no artifact to preserve'
  exit 0
}
if (-not (Select-String -LiteralPath $log -SimpleMatch 'Target //codex-rs/cli:release_files up-to-date:' -Quiet) -or
    -not (Select-String -LiteralPath $log -SimpleMatch 'INFO: Build completed successfully' -Quiet)) {
  Write-Warning 'Optimized CLI compilation incomplete; cannot call it a built binary'
  exit 0
}
# Real Bazel output paths are present in the completed build's authoritative
# target receipt, independently of a later cquery or cache-hit verifier.
$logLines=@(Get-Content -LiteralPath $log)
$items=@{}
foreach($filename in @('codex.exe','codex.pdb')) {
  $needle=[regex]::Escape($filename)
  $candidates=@($logLines | ForEach-Object {
    $text=[string]$_
    if ($text -match ("^\s*(bazel-out[/\\][^\r\n]*[/\\]" + $needle + ")\s*$")) {
      $Matches[1]
    }
  })
  if ($candidates.Count -ne 1) { throw "Expected exactly one actual CLI build path for $filename, got $($candidates.Count)" }
  $physical=Join-Path (Get-Location).Path ($candidates[0] -replace '/','\')
  if (-not (Test-Path -LiteralPath $physical -PathType Leaf)) { throw "Bazel CLI target receipt references missing file: $physical" }
  $size=(Get-Item -LiteralPath $physical).Length
  if ($size -le 4096) { throw "Truncated optimized CLI/PDB file $filename with $size bytes" }
  $stream=[IO.File]::OpenRead($physical)
  try {
    $header=New-Object byte[] 32
    $len=$stream.Read($header,0,$header.Length)
    if ($len -ne 32) { throw "Short PE/PDB header for $filename" }
    $ascii=[Text.Encoding]::ASCII.GetString($header)
    if ($filename.EndsWith('.exe') -and -not $ascii.StartsWith('MZ')) { throw "Not a Windows PE executable: $filename" }
    if ($filename.EndsWith('.pdb') -and -not $ascii.StartsWith('Microsoft C/C++ MSF 7.00')) { throw "Not an MSVC PDB: $filename" }
  } finally { $stream.Dispose() }
  $items[$filename]=@{ original_path=$candidates[0]; bytes=$size; sha256=(Get-FileHash -LiteralPath $physical -Algorithm SHA256).Hash.ToLowerInvariant() }
}
New-Item -Path $dest -ItemType Directory -Force | Out-Null
foreach($name in $items.Keys) {
  $src=Join-Path (Get-Location).Path ($items[$name].original_path -replace '/','\')
  $out=Join-Path $dest $name
  if (-not (Test-Path -LiteralPath $out)) { Copy-Item -LiteralPath $src -Destination $out -Force }
  if ((Get-FileHash -LiteralPath $out -Algorithm SHA256).Hash.ToLowerInvariant() -ne $items[$name].sha256) {
    throw "Copied nonrelease CLI/PDB SHA-256 changed: $name"
  }
}
$receipt=@{ source_sha=$env:GITHUB_SHA; result='REAL_OPTIMIZED_WINDOWS_CLI_AND_PDB_VERIFIED_NONRELEASE'; files=$items; release_approved=$false }
$receipt | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath (Join-Path $dest 'cli-integrity.json') -Encoding utf8
Write-Host "REAL_OPTIMIZED_CLI_AND_PDB_SALVAGED_AND_HASHED=TRUE (NONRELEASE)"
if ($env:GITHUB_OUTPUT) { "cli_artifact_ready=true" | Out-File -FilePath $env:GITHUB_OUTPUT -Append -Encoding utf8 }
