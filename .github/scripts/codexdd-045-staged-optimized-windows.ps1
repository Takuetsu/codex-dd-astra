# NONRELEASE source-frozen release-profile Windows build executor.
[CmdletBinding()]
param(
  [Parameter(Mandatory=$true)][ValidateSet('warm','core','cli','combined')][string]$Stage,
  [Parameter(Mandatory=$true)][ValidateRange(2,170)][int]$SoftMinutes,
  [string]$ExpectedPathSha = '',
  [long]$MaxCacheBytes = 3221225472
)
$ErrorActionPreference='Stop'
$targets=@{
  warm=@('//codex-rs/config:config','//codex-rs/protocol:protocol')
  core=@('//codex-rs/core:core')
  cli=@('//codex-rs/cli:release_files')
  combined=@('//codex-rs:runtime_package_combined')
}
if (-not $env:GITHUB_SHA -or (git rev-parse HEAD).Trim() -ne $env:GITHUB_SHA) { throw 'Unfrozen candidate source' }
if ((Get-Content codex-rs/codexdd-version.txt -Raw).Trim() -ne '0.4.4') { throw 'Version promoted' }
if ((Get-Content codex-rs/upstream-codex-release.txt -Raw).Trim() -ne 'rust-v0.162.0') { throw 'Upstream marker promoted' }
if ($env:BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA) { throw 'External license override forbidden' }
$paths=@($env:PATH -split ';' | Where-Object { $_ })
$ruby=@($paths | Where-Object { $_ -match '(?i)^[A-Z]:\\hostedtoolcache\\windows\\Ruby\\' })
if ($ruby.Count -ne 1) { throw "Hosted Ruby PATH normalization not deterministic: $($ruby.Count)" }
$env:PATH=(@($paths | Where-Object { $_ -notin $ruby }) -join ';')
$bytes=[System.Text.Encoding]::UTF8.GetBytes($env:PATH)
$pathSha=[Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData($bytes)).ToLowerInvariant()
Write-Host "BAZEL_ACTION_PATH_SHA256=$pathSha"
if ($ExpectedPathSha -and $pathSha -ne $ExpectedPathSha) { throw 'MSVC action environment drift across runner jobs' }
if ($env:GITHUB_OUTPUT) { "path_sha=$pathSha" | Out-File -FilePath $env:GITHUB_OUTPUT -Encoding utf8 -Append }
$bazel=(Get-Command bazel).Source
if (-not $bazel -or -not (Test-Path $bazel)) { throw 'Pinned Bazelisk absent' }
$v=(& bazel --version | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or $v -ne 'bazel 9.2.0') { throw "Unexpected Bazel version: $v" }
$cache='D:/_bazel-disk'
New-Item -ItemType Directory -Force -Path $cache | Out-Null
$dir=Join-Path $env:RUNNER_TEMP 'codexdd045-release-staged'
New-Item -ItemType Directory -Force -Path $dir | Out-Null
$stdout=Join-Path $dir ($Stage+'-stdout.log')
$stderr=Join-Path $dir ($Stage+'-stderr.log')
$combined=Join-Path $dir ($Stage+'-combined.log')
$opts=@(
  '--lockfile_mode=error','--config=ci-windows-msvc','--config=release',
  '--platforms=//:windows_x86_64_msvc','--host_platform=//:local_windows_msvc',
  '--//codex-rs:runtime_package_build_mode=release',
  '--disk_cache=D:/_bazel-disk'
)
if ($Stage -in @('cli','combined') -and (Get-PSDrive -Name D).Free -lt 6442450944) {
  throw 'Insufficient D: scratch space for optimized release link'
}
$buildArgs=@('--noexperimental_remote_repo_contents_cache','build')+$opts+@('--jobs=2','--verbose_failures','--announce_rc')+$targets[$Stage]
# --announce_rc only prints configuration/rc flags, not all direct Bazel CLI
# options. Verify explicit disk cache override in actual command arguments.
$requiredCacheArg='--disk_cache=D:/_bazel-disk'
if (@($buildArgs | Where-Object { $_ -ceq $requiredCacheArg }).Count -ne 1) {
  throw 'Missing or duplicate direct disk-cache option in optimized build arguments'
}
$ciOptionIndex=[Array]::IndexOf([string[]]$buildArgs,'--config=ci-windows-msvc')
$cacheOptionIndex=[Array]::IndexOf([string[]]$buildArgs,$requiredCacheArg)
if ($cacheOptionIndex -le $ciOptionIndex) {
  throw 'Explicit Bazel disk-cache argument must appear after ci-windows-msvc config'
}
Write-Host "VERIFIED_EXPLICIT_OPT_RELEASE_BAZEL_DISK_CACHE=$requiredCacheArg"
$clock=[System.Diagnostics.Stopwatch]::StartNew()
$proc=Start-Process -FilePath $bazel -ArgumentList $buildArgs -PassThru -NoNewWindow -RedirectStandardOutput $stdout -RedirectStandardError $stderr
if (-not $proc.WaitForExit($SoftMinutes*60000)) {
  $ErrorActionPreference='Continue'
  & taskkill.exe /PID $proc.Id /T /F *> $null
  $ErrorActionPreference='Stop'
  throw "Native release stage $Stage exceeded $SoftMinutes minutes"
}
$proc.Refresh()
$exitCode=$proc.ExitCode
$clock.Stop()
if (-not (Test-Path $stdout) -or -not (Test-Path $stderr)) { throw "Missing native stage diagnostics: $Stage" }
@((Get-Content $stdout),(Get-Content $stderr)) | Set-Content $combined -Encoding utf8
Get-Content $combined -Tail 55 | ForEach-Object {
  $s=[string]$_
  Write-Host $s.Substring(0,[Math]::Min(600,$s.Length))
}
if ($exitCode -ne 0) { throw "Optimized Bazel build failed for $Stage with exit $exitCode" }
foreach ($required in @('Build completed successfully','--compilation_mode=opt','--@rules_rust//rust/settings:lto=thin')) {
  if (-not (Select-String -LiteralPath $combined -SimpleMatch $required -Quiet)) { throw "Missing required stage receipt: $required" }
}
$summary=(Select-String -LiteralPath $combined -Pattern 'INFO: [0-9]+ processes:' | Select-Object -Last 1).Line
if (-not $summary) { throw "Missing Bazel process summary: $Stage" }
if ($Stage -in @('core','cli') -and $summary -notmatch '(?i)\b[1-9][0-9]* disk cache hits?\b') {
  throw "No real compiler action cache reuse in $Stage : $summary"
}
if (git status --porcelain --untracked-files=no) { throw "Tracked source modified in stage $Stage" }
$stats=Get-ChildItem $cache -Recurse -File | Measure-Object Length -Sum
$cacheBytes=[long]$stats.Sum
if ($cacheBytes -le 0 -or $cacheBytes -gt $MaxCacheBytes) { throw "Disk cache capacity outside approved bound: $cacheBytes" }
if ($env:GITHUB_OUTPUT) { "cache_bytes=$cacheBytes" | Out-File -FilePath $env:GITHUB_OUTPUT -Encoding utf8 -Append }
if ($Stage -in @('cli','combined')) {
  $qFile=Join-Path $dir ($Stage+'-cquery.log')
  $qArgs=@('--noexperimental_remote_repo_contents_cache','cquery')+$opts+@('--output=files',$targets[$Stage][0])
  $ErrorActionPreference='Continue'
  & bazel @qArgs *> $qFile
  $qExit=$LASTEXITCODE
  $ErrorActionPreference='Stop'
  if ($qExit -ne 0 -or -not (Test-Path $qFile)) { throw "Missing optimized $Stage target cquery" }
  $files=@(Get-Content $qFile | Where-Object { $_ -match '^bazel-out[/\\]' })
  if ($Stage -eq 'cli') {
    foreach ($name in @('codex.exe','codex.pdb')) {
      $found=@($files | Where-Object { $_ -match ('(?i)[/\\]'+[regex]::Escape($name)+'$') })
      if ($found.Count -ne 1) { throw "Expected one optimized $name from Bazel cquery, saw $($found.Count)" }
      $full=Join-Path (Get-Location).Path ($found[0] -replace '/','\')
      if (-not (Test-Path -LiteralPath $full) -or (Get-Item $full).Length -le 4096) { throw "Invalid physical optimized $name file" }
      Write-Host ("REAL_OPT_CLI_FILE={0} BYTES={1}" -f $name,(Get-Item $full).Length)
    }
  } else {
    $found=@($files | Where-Object { $_ -match '(?i)[/\\]runtime_package_combined\.tar\.gz$' })
    if ($found.Count -ne 1) { throw "Expected exactly one combined archive target: $($found.Count)" }
    $full=Join-Path (Get-Location).Path ($found[0] -replace '/','\')
    if (-not (Test-Path -LiteralPath $full)) { throw 'Combined archive file missing on disk' }
    $inventory=Join-Path $dir 'combined-sha256-inventory.json'
    python scripts/codexdd_045_verify_windows_archive.py "$full" --json-output "$inventory"
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $inventory)) { throw 'Actual EXE/PDB archive inspection failed' }
  }
}
$receipt=@{ stage=$Stage; source_sha=$env:GITHUB_SHA; path_sha=$pathSha; cache_bytes=$cacheBytes; summary=$summary; elapsed_s=$clock.Elapsed.TotalSeconds; release_approved=$false }
$receipt | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $dir ($Stage+'-receipt.json')) -Encoding utf8
Write-Host "NONRELEASE_OPTIMIZED_STAGE_PASSED=$Stage"
