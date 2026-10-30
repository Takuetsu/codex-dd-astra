# CodexDD 0.4.5 3F.56: build ONE missing real optimized Windows sidecar.
# Execute from the EXACT 3F.48 binary-source checkout (not the workflow tooling checkout).
[CmdletBinding()]
param(
  [Parameter(Mandatory=$true)][ValidateSet('service','proxy')][string]$Component,
  [Parameter(Mandatory=$true)][string]$SourceSha,
  [Parameter(Mandatory=$true)][string]$ToolingSha,
  [ValidateRange(10,240)][int]$SoftMinutes = 170,
  [string]$ExpectedPathSha = '7e3457c3218c6933b8b30de65061ccf56c951619862e7d4f38ed9f2be086908e'
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$baseline='57152222a1ef523fe6e2f5079871672423c6cb56'
if ($SourceSha -cne $baseline -or (git rev-parse HEAD).Trim() -cne $baseline) {
  throw 'Seven-binary sidecar source must be the exact old five-program build SHA'
}
if (-not $env:GITHUB_SHA -or $ToolingSha -cne $env:GITHUB_SHA) {
  throw 'Tooling SHA must be the exact triggering one-shot workflow commit'
}
if ((Get-Content 'codex-rs/codexdd-version.txt' -Raw).Trim() -ne '0.4.4' -or
    (Get-Content 'codex-rs/upstream-codex-release.txt' -Raw).Trim() -ne 'rust-v0.162.0') {
  throw 'Production/upstream marker unexpectedly promoted'
}
if (git status --porcelain --untracked-files=no) { throw 'Tracked source dirty before MSVC build' }
$blobs=@{
 'codex-rs/Cargo.lock'='13bc15ff844e93dfaf7777c73bb7313b35bb3b81'
 'codex-rs/Cargo.toml'='cfebd9b784ef9ff027b274aa9f7101c9c2496613'
 'codex-rs/deny.toml'='f282c7c55d05201e308e60804f85754c8522d462'
 'codex-rs/BUILD.bazel'='843e9fe095e0982df2043bb17c7da8e6e1c95025'
 'bazel/rules/runtime_package.bzl'='ba4cdad58f916f04602109c7b4d2512e83da5485'
 'codex-rs/windows-sandbox-service/BUILD.bazel'='63bcdf554bc18d47536daa3d4c5722513492fc6d'
 'codex-rs/responses-api-proxy/BUILD.bazel'='b08e0e6452c0ad6bc5dc3d2aef6d091149f40e66'
}
foreach($path in $blobs.Keys) {
  if ((git rev-parse ('HEAD:'+$path)).Trim() -cne $blobs[$path]) {
    throw "Protected source drift in $path"
  }
}
if ($env:BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA) { throw 'License override forbidden' }
$names=@{
 service=@{target='//codex-rs/windows-sandbox-service:release_files';name='codex-windows-sandbox-service'}
 proxy=@{target='//codex-rs/responses-api-proxy:release_files';name='codex-responses-api-proxy'}
}
$selected=$names[$Component]
$paths=@($env:PATH -split ';' | Where-Object { $_ })
$ruby=@($paths | Where-Object { $_ -match '(?i)^[A-Z]:\\hostedtoolcache\\windows\\Ruby\\' })
if ($ruby.Count -ne 1) { throw "Non-deterministic hosted Ruby PATH count: $($ruby.Count)" }
$env:PATH=(@($paths | Where-Object { $_ -notin $ruby }) -join ';')
$pathBytes=[Text.Encoding]::UTF8.GetBytes($env:PATH)
$actualPathSha=[Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($pathBytes)).ToLowerInvariant()
Write-Host "SEVEN_PROGRAM_ACTION_PATH_SHA256=$actualPathSha"
if ($actualPathSha -cne $ExpectedPathSha) { throw 'Windows compiler action PATH drift; STOP before building' }
$bazel=(Get-Command bazel -ErrorAction Stop).Source
if ((& bazel --version | Out-String).Trim() -ne 'bazel 9.2.0') { throw 'Not Bazel 9.2.0' }
$cache='D:/_bazel-disk'
if (-not (Test-Path -LiteralPath $cache)) { throw 'Missing previously completed core action cache' }
$cacheCount=@(Get-ChildItem -LiteralPath $cache -Force -ErrorAction Stop).Count
if ($cacheCount -lt 1) { throw 'Core cache empty; do not repeat expensive build' }
$runner=Join-Path $env:RUNNER_TEMP 'codexdd045-seven-program'
$logs=Join-Path $runner 'logs'
$dest=Join-Path $runner ('built-'+$Component)
New-Item -ItemType Directory -Force -Path $logs,$dest | Out-Null
$stdout=Join-Path $logs ($Component+'-stdout.log')
$stderr=Join-Path $logs ($Component+'-stderr.log')
$opts=@('--lockfile_mode=error','--config=ci-windows-msvc','--config=release',
 '--platforms=//:windows_x86_64_msvc','--host_platform=//:local_windows_msvc',
 '--//codex-rs:runtime_package_build_mode=release','--disk_cache=D:/_bazel-disk')
$argsBuild=@('--noexperimental_remote_repo_contents_cache','build')+$opts+
 @('--jobs=2','--verbose_failures','--announce_rc',$selected.target)
$clock=[Diagnostics.Stopwatch]::StartNew()
$proc=Start-Process -FilePath $bazel -ArgumentList $argsBuild -PassThru -NoNewWindow -RedirectStandardOutput $stdout -RedirectStandardError $stderr
if (-not $proc.WaitForExit($SoftMinutes*60000)) {
  $ErrorActionPreference='Continue'
  & taskkill.exe /PID $proc.Id /T /F *> $null
  throw "Sidecar $Component exceeded $SoftMinutes minutes; do not continue"
}
$proc.Refresh()
$clock.Stop()
if ($proc.ExitCode -ne 0) {
  Get-Content $stderr -Tail 55 | Write-Host
  throw "Native Bazel release $Component FAILED with exit $($proc.ExitCode)"
}
$combined=Join-Path $logs ($Component+'-combined.log')
@((Get-Content -LiteralPath $stdout),(Get-Content -LiteralPath $stderr)) | Set-Content $combined -Encoding utf8
foreach($need in @('Build completed successfully','--compilation_mode=opt','--@rules_rust//rust/settings:lto=thin')) {
  if (-not (Select-String -LiteralPath $combined -SimpleMatch $need -Quiet)) {
    throw "Missing successful optimized release proof for $Component : $need"
  }
}
Write-Host "NATIVE_SIDECAR_BUILD_COMPLETED=$Component"
$qArgs=@('--noexperimental_remote_repo_contents_cache','cquery')+$opts+
 @('--output=files',$selected.target)
$qFile=Join-Path $logs ($Component+'-cquery.log')
$ErrorActionPreference='Continue'
& bazel @qArgs *> $qFile
$qExit=$LASTEXITCODE
$ErrorActionPreference='Stop'
if ($qExit -ne 0) { throw "Optimized Bazel cquery failed for $Component" }
$lines=@(Get-Content -LiteralPath $qFile | Where-Object { $_ -match '^bazel-out[/\\]' })
$report=@{}
foreach($suffix in @('.exe','.pdb')) {
  $filename=$selected.name+$suffix
  $hits=@($lines | Where-Object { $_ -match ('(?i)[/\\]'+[regex]::Escape($filename)+'$') })
  if ($hits.Count -ne 1) { throw "Expected one optimized $filename output; got $($hits.Count)" }
  $physical=Join-Path (Get-Location).Path ($hits[0] -replace '/','\')
  if (-not (Test-Path -LiteralPath $physical -PathType Leaf)) { throw "Missing physical $filename" }
  $size=(Get-Item -LiteralPath $physical).Length
  if ($size -le 4096) { throw "Truncated $filename" }
  $stream=[IO.File]::OpenRead($physical)
  try {
    $buf=New-Object byte[] 32
    $read=$stream.Read($buf,0,$buf.Length)
    $prefix=[Text.Encoding]::ASCII.GetString($buf,0,$read)
    if ($read -ne 32) { throw "Truncated PE/PDB header: $filename" }
    if ($suffix -eq '.exe' -and -not $prefix.StartsWith('MZ')) { throw "Invalid Windows PE: $filename" }
    if ($suffix -eq '.pdb' -and -not $prefix.StartsWith('Microsoft C/C++ MSF 7.00')) { throw "Invalid MSVC PDB: $filename" }
  } finally { $stream.Dispose() }
  $out=Join-Path $dest $filename
  Copy-Item -LiteralPath $physical -Destination $out
  $hash=(Get-FileHash -LiteralPath $out -Algorithm SHA256).Hash.ToLowerInvariant()
  if ($hash -cne (Get-FileHash -LiteralPath $physical -Algorithm SHA256).Hash.ToLowerInvariant()) {
    throw "Copied optimized output hash mismatch: $filename"
  }
  $report[$filename]=@{bytes=$size;sha256=$hash;bazel_path=$hits[0]}
  Write-Host "REAL_OPTIMIZED_SIDECAR_SHA256=$hash FILE=$filename BYTES=$size"
}
if (git status --porcelain --untracked-files=no) { throw 'Tracked source changed during sidecar compilation' }
$receipt=@{
 contract_version=1
 source_sha=$baseline
 workflow_sha=$ToolingSha
 component=$Component
 result='REAL_OPTIMIZED_MSVC_EXE_AND_PDB_VERIFIED_NONRELEASE'
 target=$selected.target
 action_path_sha256=$actualPathSha
 files=$report
 elapsed_seconds=[math]::Round($clock.Elapsed.TotalSeconds,1)
 release_approved=$false
}
$receipt | ConvertTo-Json -Depth 7 | Set-Content -LiteralPath (Join-Path $dest ($Component+'-integrity.json')) -Encoding utf8
Write-Host "NONRELEASE_SIDECAR_READY=$Component"
