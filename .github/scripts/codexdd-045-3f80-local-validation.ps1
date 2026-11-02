# CodexDD 0.4.5 3F.80: local, nonrelease, source-pinned Windows acceptance.
# Runs source preflight before optional native tests. Never checks out,
# autoformats, commits, merges, installs, signs, or publishes.
[CmdletBinding()]
param(
    [ValidateSet('Source', 'Native')]
    [string]$Mode = 'Source',
    [string]$ExpectedHead = '',
    [string]$LogRoot = ''
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$root = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..\..'))
$started = [System.Diagnostics.Stopwatch]::StartNew()
$head = ''
$lockBlob = ''
$sourceGatePassed = $false
$nativeCases = @()
$exitCode = 1
$failure = ''
$expectedBranch = 'dd/codexdd-v0.4.5-3f80-queue-auth-integrated'
$expectedLock = '13bc15ff844e93dfaf7777c73bb7313b35bb3b81'

if (-not $LogRoot) {
    $LogRoot = Join-Path $env:TEMP 'codexdd-045-3f80-local-validation'
}
$LogRoot = [System.IO.Path]::GetFullPath($LogRoot)

function Invoke-CargoStage {
    [CmdletBinding()]
    param(
        [string]$Stage,
        [string[]]$Arguments,
        [bool]$SingleTest = $false
    )
    $log = Join-Path $LogRoot ($Stage + '.log')
    Write-Host ('BEGIN: ' + $Stage)
    Push-Location (Join-Path $root 'codex-rs')
    $savedPreference = $ErrorActionPreference
    try {
        # Rust tools can write ordinary warnings to stderr on Windows PowerShell.
        $ErrorActionPreference = 'Continue'
        & cargo @Arguments 2>&1 | Out-File -FilePath $log -Encoding utf8
        $code = [int]$LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $savedPreference
        Pop-Location
    }
    $text = Get-Content -LiteralPath $log -Raw
    $onePassed = [regex]::IsMatch(
        $text,
        '(?m)^test result: ok\.\s+1 passed;\s+0 failed;'
    )
    $passed = ($code -eq 0 -and (-not $SingleTest -or $onePassed))
    if ($SingleTest) {
        $script:nativeCases += [ordered]@{
            selector = $Stage
            exit_code = $code
            exactly_one_test_passed = $onePassed
            passed = $passed
            log = $log
        }
    }
    if (-not $passed) {
        if ($Stage -eq 'source-rustfmt') {
            # Rustfmt on stable can emit hundreds of known nightly-only
            # imports_granularity warnings. Show actual diffs, not that noise.
            $diffs = @(Select-String -LiteralPath $log -Pattern '^Diff in ' -Context 0,12)
            if ($diffs.Count -gt 0) {
                foreach ($diff in ($diffs | Select-Object -First 20)) {
                    Write-Host ('RUSTFMT_DIFF: ' + $diff.Line)
                    foreach ($line in $diff.Context.PostContext) {
                        Write-Host $line
                    }
                }
            }
            else {
                Get-Content -LiteralPath $log |
                    Where-Object { $_ -notmatch 'imports_granularity = Item' } |
                    Select-Object -Last 60 |
                    ForEach-Object { Write-Host $_ }
            }
        }
        else {
            # Retain a compact error/location index to avoid losing the first
            # compiler diagnostics when a large integration binary fails.
            $lines = @(Get-Content -LiteralPath $log)
            $diagnostics = New-Object 'System.Collections.Generic.List[string]'
            for ($i = 0; $i -lt $lines.Count; $i++) {
                if ($lines[$i] -notmatch '^error(\[[A-Z][0-9]+\])?:') {
                    continue
                }
                $diagnostics.Add([string]$lines[$i])
                for ($j = $i + 1; $j -lt [Math]::Min($lines.Count, $i + 9); $j++) {
                    if ($lines[$j] -match '^\s*-->\s+.+:\d+(?::\d+)?\s*
        throw ('FAILED: ' + $Stage + ' exit=' + $code + ' one_passed=' + $onePassed)
    }
    Write-Host ('PASS: ' + $Stage)
}

try {
    if ($env:OS -ne 'Windows_NT' -or $env:COMPUTERNAME -ine 'DANIEL-CL') {
        throw 'Run only on the canonical Windows host Daniel-CL'
    }
    if ($root -ine 'E:\codexdd' -and -not $root.StartsWith('E:\codexdd\', [System.StringComparison]::OrdinalIgnoreCase)) {
        throw 'Expected an isolated worktree inside E:\codexdd'
    }
    if ($LogRoot.StartsWith(
            ($root.TrimEnd('\') + '\'),
            [System.StringComparison]::OrdinalIgnoreCase
        )
    ) {
        throw 'LogRoot must be outside the Git worktree'
    }
    New-Item -ItemType Directory -Path $LogRoot -Force | Out-Null

    Push-Location $root
    try {
        $head = (& git rev-parse HEAD | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $head -notmatch '^[0-9a-f]{40}$') {
            throw 'Cannot establish exact source commit'
        }
        if ($ExpectedHead -and $ExpectedHead -cne $head) {
            throw ('Wrong source commit: expected ' + $ExpectedHead + ', got ' + $head)
        }
        $actualBranch = (& git branch --show-current | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $actualBranch -cne $expectedBranch) {
            throw ('Wrong candidate branch: ' + $actualBranch)
        }
        $dirty = (& git status --porcelain --untracked-files=normal | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $dirty) {
            throw ('Candidate worktree is dirty: ' + $dirty)
        }
        $lockBlob = (& git rev-parse 'HEAD:codex-rs/Cargo.lock' | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $lockBlob -cne $expectedLock) {
            throw ('Unreviewed Cargo.lock identity: ' + $lockBlob)
        }
        $securitySources = @{
            'codex-rs/Cargo.toml' = 'cfebd9b784ef9ff027b274aa9f7101c9c2496613'
            'codex-rs/deny.toml' = 'f282c7c55d05201e308e60804f85754c8522d462'
        }
        foreach ($protectedPath in $securitySources.Keys) {
            $actual = (& git rev-parse ('HEAD:' + $protectedPath) | Out-String).Trim()
            if ($LASTEXITCODE -ne 0 -or $actual -cne $securitySources[$protectedPath]) {
                throw ('Unreviewed protected security file: ' + $protectedPath)
            }
        }
        $upstream = (& git show 'HEAD:codex-rs/upstream-codex-release.txt' | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $upstream -cne 'rust-v0.162.0') {
            throw ('Unexpected upstream release marker: ' + $upstream)
        }
        $version = (& git show 'HEAD:codex-rs/codexdd-version.txt' | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $version -cne '0.4.4') {
            throw 'Production version changed; refusing nonrelease acceptance'
        }

        $ledger = Get-Content 'docs/upstream-candidates/codexdd-045-3f55-formal-owner-approvals.json' -Raw | ConvertFrom-Json
        if ($ledger.approved_owner_count -ne 0 -or
            @($ledger.pending_native_or_semantic_restoration).Count -ne 13) {
            throw 'High-risk owner ledger was unexpectedly waived or changed'
        }
        $source = Get-Content 'codex-rs/app-server/src/request_processors/turn_processor.rs' -Raw
        if (-not $source.Contains('workflow_mutation_matches_known_turn(') -or
            -not $source.Contains('subscribed_connection_ids(thread_id)')) {
            throw 'Workflow mutation source-turn or connection guard is absent'
        }

        Invoke-CargoStage -Stage 'source-rustfmt' -Arguments @('fmt', '--all', '--', '--check')
        & git diff --check
        if ($LASTEXITCODE -ne 0) {
            throw 'git diff --check failed'
        }
        $dirty = (& git status --porcelain --untracked-files=normal | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $dirty) {
            throw 'Source preflight dirtied the worktree'
        }
        $sourceGatePassed = $true
        Write-Host ('SOURCE_GATE=PASS_NONRELEASE HEAD=' + $head)

        if ($Mode -eq 'Native') {
            $selectors = @(
                @{ selector = 'workflow_mutation_rejects_foreign_and_unsubscribed_connections'; binary = 'lib' },
                @{ selector = 'workflow_mutation_rejects_stale_source_turn_without_blocking_valid_or_cold_resume'; binary = 'lib' },
                @{ selector = 'v2_spawned_agent_rejects_direct_workflow_mutations_but_regular_threads_continue'; binary = 'lib' },
                @{ selector = 'workflow_state_update_preserves_authorized_top_level_cli_mutation'; binary = 'all' },
                @{ selector = 'websocket_workflow_state_write_rejects_foreign_and_unsubscribed_connections'; binary = 'all' }
            )
            $env:CARGO_TERM_COLOR = 'never'
            foreach ($test in $selectors) {
                $cargoArguments = @('test', '--locked', '-p', 'codex-app-server')
                if ($test.binary -eq 'lib') {
                    $cargoArguments += '--lib'
                }
                else {
                    $cargoArguments += @('--test', 'all')
                }
                $cargoArguments += @($test.selector, '--', '--test-threads=1')
                Invoke-CargoStage -Stage $test.selector -Arguments $cargoArguments -SingleTest $true
            }
            if (@($nativeCases).Count -ne 5) {
                throw 'Missing focused native test cases'
            }
            $dirty = (& git status --porcelain --untracked-files=normal | Out-String).Trim()
            if ($LASTEXITCODE -ne 0 -or $dirty) {
                throw 'Native test validation dirtied the worktree'
            }
            Write-Host 'CODEXDD_045_3F80_NATIVE=PASS_5_OF_5_NONRELEASE'
        }
    }
    finally {
        Pop-Location
    }
    $exitCode = 0
}
catch {
    $failure = $_.Exception.Message
    Write-Host ('CODEXDD_045_3F80=RED ' + $failure)
}
finally {
    $started.Stop()
    $done = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ')
    $receipt = [ordered]@{
        contract_version = 1
        kind = 'CODEXDD_045_3F80_LOCAL_NONRELEASE'
        tested_head = $head
        machine = $env:COMPUTERNAME
        mode = $Mode
        cargo_lock_blob = $lockBlob
        source_gate_passed = $sourceGatePassed
        native_cases = @($nativeCases)
        result = $(if ($exitCode -eq 0) { 'PASS_NONRELEASE' } else { 'FAIL' })
        error = $failure
        formal_owner_approvals = $(if ($sourceGatePassed) { 0 } else { $null })
        release_approved = $false
        elapsed_seconds = [Math]::Round($started.Elapsed.TotalSeconds, 1)
        done_utc = $done
    }
    if (Test-Path -LiteralPath $LogRoot) {
        $receipt | ConvertTo-Json -Depth 5 |
            Set-Content -LiteralPath (Join-Path $LogRoot '3f80-local-receipt.json') -Encoding utf8
        Write-Host ('RECEIPT: ' + (Join-Path $LogRoot '3f80-local-receipt.json'))
    }
    Write-Host ('TOTAL_TIME_WORKED_SECONDS: ' + $receipt.elapsed_seconds)
    Write-Host ('COMMIT: ' + $head)
    Write-Host ('DONE_UTC: ' + $done)
}
exit $exitCode
) {
                        $diagnostics.Add(([string]$lines[$j]).Trim())
                        break
                    }
                    if ($lines[$j] -match '^error(\[[A-Z][0-9]+\])?:') {
                        break
                    }
                }
            }
            if ($diagnostics.Count -gt 0) {
                $digest = Join-Path $LogRoot ($Stage + '-errors.txt')
                $diagnostics | Set-Content -LiteralPath $digest -Encoding utf8
                Write-Host ('COMPILER_ERROR_DIGEST: ' + $digest)
                foreach ($entry in ($diagnostics | Select-Object -First 120)) {
                    Write-Host $entry
                }
            }
            else {
                Write-Host ((Get-Content -LiteralPath $log -Tail 60) -join [Environment]::NewLine)
            }
        }
        Write-Host ('FULL_STAGE_LOG: ' + $log)
        throw ('FAILED: ' + $Stage + ' exit=' + $code + ' one_passed=' + $onePassed)
    }
    Write-Host ('PASS: ' + $Stage)
}

try {
    if ($env:OS -ne 'Windows_NT' -or $env:COMPUTERNAME -ine 'DANIEL-CL') {
        throw 'Run only on the canonical Windows host Daniel-CL'
    }
    if ($root -ine 'E:\codexdd' -and -not $root.StartsWith('E:\codexdd\', [System.StringComparison]::OrdinalIgnoreCase)) {
        throw 'Expected an isolated worktree inside E:\codexdd'
    }
    if ($LogRoot.StartsWith(
            ($root.TrimEnd('\') + '\'),
            [System.StringComparison]::OrdinalIgnoreCase
        )
    ) {
        throw 'LogRoot must be outside the Git worktree'
    }
    New-Item -ItemType Directory -Path $LogRoot -Force | Out-Null

    Push-Location $root
    try {
        $head = (& git rev-parse HEAD | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $head -notmatch '^[0-9a-f]{40}$') {
            throw 'Cannot establish exact source commit'
        }
        if ($ExpectedHead -and $ExpectedHead -cne $head) {
            throw ('Wrong source commit: expected ' + $ExpectedHead + ', got ' + $head)
        }
        $actualBranch = (& git branch --show-current | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $actualBranch -cne $expectedBranch) {
            throw ('Wrong candidate branch: ' + $actualBranch)
        }
        $dirty = (& git status --porcelain --untracked-files=normal | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $dirty) {
            throw ('Candidate worktree is dirty: ' + $dirty)
        }
        $lockBlob = (& git rev-parse 'HEAD:codex-rs/Cargo.lock' | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $lockBlob -cne $expectedLock) {
            throw ('Unreviewed Cargo.lock identity: ' + $lockBlob)
        }
        $securitySources = @{
            'codex-rs/Cargo.toml' = 'cfebd9b784ef9ff027b274aa9f7101c9c2496613'
            'codex-rs/deny.toml' = 'f282c7c55d05201e308e60804f85754c8522d462'
        }
        foreach ($protectedPath in $securitySources.Keys) {
            $actual = (& git rev-parse ('HEAD:' + $protectedPath) | Out-String).Trim()
            if ($LASTEXITCODE -ne 0 -or $actual -cne $securitySources[$protectedPath]) {
                throw ('Unreviewed protected security file: ' + $protectedPath)
            }
        }
        $upstream = (& git show 'HEAD:codex-rs/upstream-codex-release.txt' | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $upstream -cne 'rust-v0.162.0') {
            throw ('Unexpected upstream release marker: ' + $upstream)
        }
        $version = (& git show 'HEAD:codex-rs/codexdd-version.txt' | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $version -cne '0.4.4') {
            throw 'Production version changed; refusing nonrelease acceptance'
        }

        $ledger = Get-Content 'docs/upstream-candidates/codexdd-045-3f55-formal-owner-approvals.json' -Raw | ConvertFrom-Json
        if ($ledger.approved_owner_count -ne 0 -or
            @($ledger.pending_native_or_semantic_restoration).Count -ne 13) {
            throw 'High-risk owner ledger was unexpectedly waived or changed'
        }
        $source = Get-Content 'codex-rs/app-server/src/request_processors/turn_processor.rs' -Raw
        if (-not $source.Contains('workflow_mutation_matches_known_turn(') -or
            -not $source.Contains('subscribed_connection_ids(thread_id)')) {
            throw 'Workflow mutation source-turn or connection guard is absent'
        }

        Invoke-CargoStage -Stage 'source-rustfmt' -Arguments @('fmt', '--all', '--', '--check')
        & git diff --check
        if ($LASTEXITCODE -ne 0) {
            throw 'git diff --check failed'
        }
        $dirty = (& git status --porcelain --untracked-files=normal | Out-String).Trim()
        if ($LASTEXITCODE -ne 0 -or $dirty) {
            throw 'Source preflight dirtied the worktree'
        }
        $sourceGatePassed = $true
        Write-Host ('SOURCE_GATE=PASS_NONRELEASE HEAD=' + $head)

        if ($Mode -eq 'Native') {
            $selectors = @(
                @{ selector = 'workflow_mutation_rejects_foreign_and_unsubscribed_connections'; binary = 'lib' },
                @{ selector = 'workflow_mutation_rejects_stale_source_turn_without_blocking_valid_or_cold_resume'; binary = 'lib' },
                @{ selector = 'v2_spawned_agent_rejects_direct_workflow_mutations_but_regular_threads_continue'; binary = 'lib' },
                @{ selector = 'workflow_state_update_preserves_authorized_top_level_cli_mutation'; binary = 'all' },
                @{ selector = 'websocket_workflow_state_write_rejects_foreign_and_unsubscribed_connections'; binary = 'all' }
            )
            $env:CARGO_TERM_COLOR = 'never'
            foreach ($test in $selectors) {
                $cargoArguments = @('test', '--locked', '-p', 'codex-app-server')
                if ($test.binary -eq 'lib') {
                    $cargoArguments += '--lib'
                }
                else {
                    $cargoArguments += @('--test', 'all')
                }
                $cargoArguments += @($test.selector, '--', '--test-threads=1')
                Invoke-CargoStage -Stage $test.selector -Arguments $cargoArguments -SingleTest $true
            }
            if (@($nativeCases).Count -ne 5) {
                throw 'Missing focused native test cases'
            }
            $dirty = (& git status --porcelain --untracked-files=normal | Out-String).Trim()
            if ($LASTEXITCODE -ne 0 -or $dirty) {
                throw 'Native test validation dirtied the worktree'
            }
            Write-Host 'CODEXDD_045_3F80_NATIVE=PASS_5_OF_5_NONRELEASE'
        }
    }
    finally {
        Pop-Location
    }
    $exitCode = 0
}
catch {
    $failure = $_.Exception.Message
    Write-Host ('CODEXDD_045_3F80=RED ' + $failure)
}
finally {
    $started.Stop()
    $done = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ')
    $receipt = [ordered]@{
        contract_version = 1
        kind = 'CODEXDD_045_3F80_LOCAL_NONRELEASE'
        tested_head = $head
        machine = $env:COMPUTERNAME
        mode = $Mode
        cargo_lock_blob = $lockBlob
        source_gate_passed = $sourceGatePassed
        native_cases = @($nativeCases)
        result = $(if ($exitCode -eq 0) { 'PASS_NONRELEASE' } else { 'FAIL' })
        error = $failure
        formal_owner_approvals = $(if ($sourceGatePassed) { 0 } else { $null })
        release_approved = $false
        elapsed_seconds = [Math]::Round($started.Elapsed.TotalSeconds, 1)
        done_utc = $done
    }
    if (Test-Path -LiteralPath $LogRoot) {
        $receipt | ConvertTo-Json -Depth 5 |
            Set-Content -LiteralPath (Join-Path $LogRoot '3f80-local-receipt.json') -Encoding utf8
        Write-Host ('RECEIPT: ' + (Join-Path $LogRoot '3f80-local-receipt.json'))
    }
    Write-Host ('TOTAL_TIME_WORKED_SECONDS: ' + $receipt.elapsed_seconds)
    Write-Host ('COMMIT: ' + $head)
    Write-Host ('DONE_UTC: ' + $done)
}
exit $exitCode
