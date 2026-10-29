# Nonrelease 3F.44: checkpoint completed Rust Bazel actions on expected timeout.
[CmdletBinding()]
param(
 [Parameter(Mandatory=$true)][ValidateSet('seed','restore')][string]$Mode,
 [Parameter(Mandatory=$true)][ValidateRange(2,25)][int]$SoftMinutes,
 [Parameter(Mandatory=$true)][long]$MaxCacheBytes,
 [string]$ExpectedPathSha=''
)
$ErrorActionPreference='Stop'
if (-not $env:GITHUB_SHA -or (git rev-parse HEAD).Trim() -ne $env:GITHUB_SHA) { throw 'Unfrozen source' }
if ((Get-Content 'codex-rs/codexdd-version.txt' -Raw).Trim() -ne '0.4.4') { throw 'Product marker promoted' }
if ((Get-Content 'codex-rs/upstream-codex-release.txt' -Raw).Trim() -ne 'rust-v0.162.0') { throw 'Upstream marker promoted' }
if ($env:BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA) { throw 'License override forbidden' }
$paths=@($env:PATH -split ';' | Where-Object { $_ })
$ruby=@($paths | Where-Object { $_ -match '(?i)^[A-Z]:\\hostedtoolcache\\windows\\Ruby\\' })
if ($ruby.Count -ne 1) { throw "Wrong hosted Ruby PATH count: $($ruby.Count)" }
$env:PATH=(@($paths | Where-Object { $_ -notin $ruby }) -join ';')
$hashBytes=[System.Security.Cryptography.SHA256]::HashData([System.Text.Encoding]::UTF8.GetBytes($env:PATH))
$pathSha=[Convert]::ToHexString($hashBytes).ToLowerInvariant()
Write-Host "BAZEL_ACTION_PATH_SHA256=$pathSha"
if ($ExpectedPathSha -and $ExpectedPathSha -ne $pathSha) { throw 'MSVC action environment drift between independent runners' }
if ($env:GITHUB_OUTPUT) { "path_sha=$pathSha" | Out-File $env:GITHUB_OUTPUT -Encoding utf8 -Append }
$bazel=(Get-Command bazel).Source
if (-not $bazel -or -not (Test-Path $bazel)) { throw 'Bazelisk missing' }
$version=(& bazel --version | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or $version -ne 'bazel 9.2.0') { throw "Unexpected Bazel: $version" }
$cache='D:/_bazel-disk'
New-Item -Path $cache -ItemType Directory -Force | Out-Null
$dir=Join-Path $env:RUNNER_TEMP 'codexdd045-3f44-checkpoint'
New-Item -Path $dir -ItemType Directory -Force | Out-Null
$stdout=Join-Path $dir ($Mode+'-stdout.log')
$stderr=Join-Path $dir ($Mode+'-stderr.log')
$combined=Join-Path $dir ($Mode+'-combined.log')
$args=@(
 '--noexperimental_remote_repo_contents_cache','build',
 '--lockfile_mode=error','--config=ci-windows-msvc','--config=release',
 '--platforms=//:windows_x86_64_msvc','--host_platform=//:local_windows_msvc',
 '--//codex-rs:runtime_package_build_mode=release',
 '--disk_cache=D:/_bazel-disk','--jobs=2','--verbose_failures','--announce_rc',
 '//codex-rs/config:config'
)
# Bazel --announce_rc records configuration-expanded flags, NOT arbitrary
# direct command-line options. Validate the actual argument vector for the
# explicit disk cache override instead of expecting it echoed in stderr.
$requiredCacheFlag='--disk_cache=D:/_bazel-disk'
if (@($args | Where-Object { $_ -ceq $requiredCacheFlag }).Count -ne 1) {
 throw 'Missing or duplicate explicit disk-cache argument'
}
$cacheIndex=[Array]::IndexOf([string[]]$args,$requiredCacheFlag)
$ciConfigIndex=[Array]::IndexOf([string[]]$args,'--config=ci-windows-msvc')
if ($cacheIndex -le $ciConfigIndex) {
 throw 'Explicit disk cache must override the ci-windows-msvc config'
}
Write-Host "VERIFIED_EXPLICIT_BAZEL_DISK_CACHE_ARGUMENT=$requiredCacheFlag"
$watch=[System.Diagnostics.Stopwatch]::StartNew()
$proc=Start-Process -FilePath $bazel -ArgumentList $args -PassThru -NoNewWindow -RedirectStandardOutput $stdout -RedirectStandardError $stderr
$timedOut=-not $proc.WaitForExit($SoftMinutes*60000)
if ($timedOut) {
 $ErrorActionPreference='Continue'
 & taskkill.exe /PID $proc.Id /T /F *> $null
 $ErrorActionPreference='Stop'
 Write-Host "EXPECTED_SOFT_DEADLINE: stopped process tree after $SoftMinutes minutes; only completed cache objects are retained"
}
$proc.Refresh()
$rc=if ($timedOut) { -1 } else { $proc.ExitCode }
$watch.Stop()
if (-not (Test-Path $stdout) -or -not (Test-Path $stderr)) { throw 'Missing actual Bazel process logs' }
@((Get-Content $stdout),(Get-Content $stderr)) | Set-Content $combined -Encoding utf8
Get-Content $combined -Tail 30 | ForEach-Object {
 $s=[string]$_
 Write-Host $s.Substring(0,[Math]::Min(650,$s.Length))
}
# Measure cache capacity before log assertions, so a failure never hides the
# reason a partially completed cache could not be checkpointed.
$files=@(Get-ChildItem -LiteralPath $cache -Recurse -File)
$bytes=[long](($files | Measure-Object -Property Length -Sum).Sum)
Write-Host ("CHECKPOINT_DISK_CACHE_INVENTORY=BYTES:{0}; FILES:{1}; MAX:{2}" -f $bytes,$files.Count,$MaxCacheBytes)
if ($bytes -le 0 -or $bytes -gt $MaxCacheBytes -or $files.Count -lt 2) {
 throw "Checkpoint out of bounds: bytes=$bytes files=$($files.Count) max=$MaxCacheBytes"
}
foreach ($opt in @('--compilation_mode=opt','--@rules_rust//rust/settings:lto=thin')) {
 if (-not (Select-String -LiteralPath $combined -SimpleMatch $opt -Quiet)) { throw "Missing optimized flag: $opt" }
}
if (-not $timedOut -and $rc -ne 0) { throw "Actual Rust compiler failure (not timeout): $rc" }
if ($timedOut -and $Mode -ne 'seed') { throw 'RESTORE_TIMEOUT: target did not finish in second bounded runner' }
$complete= -not $timedOut
$summary=''
if ($complete) {
 foreach ($phrase in @('Build completed successfully','Target //codex-rs/config:config up-to-date:')) {
  if (-not (Select-String -LiteralPath $combined -SimpleMatch $phrase -Quiet)) { throw "Missing real Rust build receipt: $phrase" }
 }
 $record=(Select-String -LiteralPath $combined -Pattern 'INFO: [0-9]+ processes:' | Select-Object -Last 1)
 if (-not $record) { throw 'Bazel succeeded but no process summary' }
 $summary=$record.Line
 if ($Mode -eq 'restore' -and $summary -notmatch '(?i)\b[1-9][0-9]* disk cache hits?\b') {
  throw "Independent runner did not reuse compiler actions: $summary"
 }
} else {
 if (-not (Select-String -LiteralPath $stderr -Pattern 'Analyzing:|Analyzed target|Compiling Rust' -Quiet)) {
  throw 'Incomplete seed had no actual Rust compilation progress'
 }
 if (Select-String -LiteralPath $stderr -Pattern '(^|\s)(ERROR:|FAILED:|error\[E[0-9]+\]:)' -Quiet) {
  throw 'Compiler error before timeout: do not checkpoint failed compilation'
 }
}
if (git status --porcelain --untracked-files=no) { throw 'Source or lockfile mutated during compiler run' }
Write-Host ("3F44_{0}_CACHE_BYTES={1}; FILES={2}; ELAPSED_S={3:F1}" -f $Mode.ToUpperInvariant(),$bytes,$files.Count,$watch.Elapsed.TotalSeconds)
Write-Host ("3F44_{0}_BUILD_COMPLETE={1}; SUMMARY={2}" -f $Mode.ToUpperInvariant(),$complete,$summary)
if ($env:GITHUB_OUTPUT) {
 "checkpoint_ready=true" | Out-File $env:GITHUB_OUTPUT -Encoding utf8 -Append
 "build_complete=$($complete.ToString().ToLowerInvariant())" | Out-File $env:GITHUB_OUTPUT -Encoding utf8 -Append
 "cache_bytes=$bytes" | Out-File $env:GITHUB_OUTPUT -Encoding utf8 -Append
}
$receipt=@{mode=$Mode; source_sha=$env:GITHUB_SHA; path_sha=$pathSha; timed_out=$timedOut; build_complete=$complete; cache_bytes=$bytes; cache_files=$files.Count; elapsed_s=$watch.Elapsed.TotalSeconds; action_summary=$summary; release_approved=$false}
$receipt | ConvertTo-Json -Depth 4 | Set-Content (Join-Path $dir ($Mode+'-receipt.json')) -Encoding utf8
if ($complete) {
 Write-Host ("NONRELEASE_REAL_OPTIMIZED_RUST_{0}_TARGET_PASS" -f $Mode.ToUpperInvariant())
} else {
 Write-Host 'NONRELEASE_PARTIAL_ACTION_CHECKPOINT_READY_NOT_TARGET_PASS'
}
