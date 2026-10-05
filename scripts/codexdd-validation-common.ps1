Set-StrictMode -Version Latest

$script:CodexDDValidationContractVersion = 1
$script:CodexDDValidationPrefix = "CODEXDD_VALIDATION_JSON "

function Write-CodexDDValidationEvent {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Event,

        [Parameter(Mandatory = $true)]
        [System.Collections.IDictionary]$Fields
    )

    $payload = [ordered]@{
        contract_version = $script:CodexDDValidationContractVersion
        event = $Event
    }

    foreach ($key in $Fields.Keys) {
        $payload[$key] = $Fields[$key]
    }

    $json = $payload | ConvertTo-Json -Compress -Depth 8
    [Console]::Out.WriteLine($script:CodexDDValidationPrefix + $json)
}

function New-CodexDDValidationStage {
    param(
        [Parameter(Mandatory = $true)]
        [ValidatePattern('^[a-z0-9][a-z0-9-]*$')]
        [string]$Name,

        [Parameter(Mandatory = $true)]
        [string]$FilePath,

        [string[]]$ArgumentList = @(),

        [Parameter(Mandatory = $true)]
        [string]$WorkingDirectory
    )

    [pscustomobject]@{
        Name = $Name
        FilePath = $FilePath
        ArgumentList = @($ArgumentList)
        WorkingDirectory = $WorkingDirectory
    }
}

function Test-CodexDDValidationRepositoryRoot {
    param(
        [Parameter(Mandatory = $true)]
        [string]$RepositoryRoot
    )

    if (-not (Test-Path -LiteralPath (Join-Path $RepositoryRoot "justfile") -PathType Leaf)) {
        return $false
    }

    if (-not (Test-Path -LiteralPath (Join-Path $RepositoryRoot "codex-rs\Cargo.toml") -PathType Leaf)) {
        return $false
    }

    return $true
}

function Write-CodexDDNativeOutput {
    param(
        [Parameter(ValueFromPipeline = $true)]
        $InputObject
    )

    process {
        if ($null -eq $InputObject) {
            return
        }

        [Console]::Out.WriteLine($InputObject.ToString())
    }
}

function Invoke-CodexDDValidationProfile {
    param(
        [Parameter(Mandatory = $true)]
        [ValidateSet("targeted", "work-packet", "release")]
        [string]$Profile,

        [Parameter(Mandatory = $true)]
        [string]$RepositoryRoot,

        [Parameter(Mandatory = $true)]
        [object[]]$Stages
    )

    $profileStopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    Write-CodexDDValidationEvent -Event "profile_begin" -Fields ([ordered]@{
        profile = $Profile
        stage_count = $Stages.Count
    })

    if (-not (Test-CodexDDValidationRepositoryRoot -RepositoryRoot $RepositoryRoot)) {
        $profileStopwatch.Stop()
        Write-CodexDDValidationEvent -Event "profile_end" -Fields ([ordered]@{
            profile = $Profile
            status = "error"
            exit_code = 2
            duration_ms = $profileStopwatch.ElapsedMilliseconds
            error_type = "invalid_repository_root"
            message = "Expected repository markers justfile and codex-rs/Cargo.toml were not found."
        })
        return 2
    }

    if ($Stages.Count -eq 0) {
        $profileStopwatch.Stop()
        Write-CodexDDValidationEvent -Event "profile_end" -Fields ([ordered]@{
            profile = $Profile
            status = "error"
            exit_code = 2
            duration_ms = $profileStopwatch.ElapsedMilliseconds
            error_type = "empty_profile"
            message = "Validation profiles must contain at least one stage."
        })
        return 2
    }

    $stageNames = @{}
    foreach ($stage in $Stages) {
        if ($null -eq $stage -or [string]::IsNullOrWhiteSpace([string]$stage.Name)) {
            $profileStopwatch.Stop()
            Write-CodexDDValidationEvent -Event "profile_end" -Fields ([ordered]@{
                profile = $Profile
                status = "error"
                exit_code = 2
                duration_ms = $profileStopwatch.ElapsedMilliseconds
                error_type = "invalid_stage"
                message = "Every validation stage must have a nonblank name."
            })
            return 2
        }

        if ($stageNames.ContainsKey($stage.Name)) {
            $profileStopwatch.Stop()
            Write-CodexDDValidationEvent -Event "profile_end" -Fields ([ordered]@{
                profile = $Profile
                status = "error"
                exit_code = 2
                duration_ms = $profileStopwatch.ElapsedMilliseconds
                error_type = "duplicate_stage"
                message = "Validation stage names must be unique."
                stage = $stage.Name
            })
            return 2
        }

        $stageNames[$stage.Name] = $true
    }

    $completedStages = 0
    for ($index = 0; $index -lt $Stages.Count; $index++) {
        $stage = $Stages[$index]
        $ordinal = $index + 1
        $stageStopwatch = [System.Diagnostics.Stopwatch]::StartNew()

        Write-CodexDDValidationEvent -Event "stage_begin" -Fields ([ordered]@{
            profile = $Profile
            stage = $stage.Name
            ordinal = $ordinal
            stage_count = $Stages.Count
            executable = $stage.FilePath
            arguments = @($stage.ArgumentList)
        })

        $nativeExitCode = $null
        try {
            Push-Location -LiteralPath $stage.WorkingDirectory
            try {
                $arguments = @($stage.ArgumentList)
                & $stage.FilePath @arguments 2>&1 | Write-CodexDDNativeOutput
                $nativeExitCode = $LASTEXITCODE
                if ($null -eq $nativeExitCode) {
                    $nativeExitCode = 0
                }
            }
            finally {
                Pop-Location
            }
        }
        catch {
            $stageStopwatch.Stop()
            $profileStopwatch.Stop()
            Write-CodexDDValidationEvent -Event "stage_end" -Fields ([ordered]@{
                profile = $Profile
                stage = $stage.Name
                ordinal = $ordinal
                status = "error"
                native_exit_code = $null
                duration_ms = $stageStopwatch.ElapsedMilliseconds
                error_type = "execution_error"
                message = $_.Exception.Message
            })
            Write-CodexDDValidationEvent -Event "profile_end" -Fields ([ordered]@{
                profile = $Profile
                status = "error"
                exit_code = 2
                duration_ms = $profileStopwatch.ElapsedMilliseconds
                completed_stages = $completedStages
                failed_stage = $stage.Name
            })
            return 2
        }

        $stageStopwatch.Stop()
        if ($nativeExitCode -ne 0) {
            $profileStopwatch.Stop()
            Write-CodexDDValidationEvent -Event "stage_end" -Fields ([ordered]@{
                profile = $Profile
                stage = $stage.Name
                ordinal = $ordinal
                status = "fail"
                native_exit_code = $nativeExitCode
                duration_ms = $stageStopwatch.ElapsedMilliseconds
            })
            Write-CodexDDValidationEvent -Event "profile_end" -Fields ([ordered]@{
                profile = $Profile
                status = "fail"
                exit_code = 1
                duration_ms = $profileStopwatch.ElapsedMilliseconds
                completed_stages = $completedStages
                failed_stage = $stage.Name
            })
            return 1
        }

        $completedStages++
        Write-CodexDDValidationEvent -Event "stage_end" -Fields ([ordered]@{
            profile = $Profile
            stage = $stage.Name
            ordinal = $ordinal
            status = "pass"
            native_exit_code = 0
            duration_ms = $stageStopwatch.ElapsedMilliseconds
        })
    }

    $profileStopwatch.Stop()
    Write-CodexDDValidationEvent -Event "profile_end" -Fields ([ordered]@{
        profile = $Profile
        status = "pass"
        exit_code = 0
        duration_ms = $profileStopwatch.ElapsedMilliseconds
        completed_stages = $completedStages
    })
    return 0
}
