[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'codexdd-python-executable.ps1')

# No Python installation is changed. Synthetic ApplicationInfo-style objects
# exercise the WindowsApps alias even if it does not exist on the runner.
$fakePython = Join-Path $env:TEMP ('codexdd-python-fixture-' + [guid]::NewGuid().ToString('N') + '.exe')
$windowsAlias = Join-Path $env:LOCALAPPDATA 'Microsoft\WindowsApps\python.exe'
try {
    [IO.File]::WriteAllBytes($fakePython, [byte[]]@(77, 90))
    $valid = [pscustomobject]@{ Source = $fakePython }
    $alias = [pscustomobject]@{ Source = $windowsAlias }
    $invalid = [pscustomobject]@{ Source = (Join-Path $env:TEMP 'nonexistent-codexdd-python.exe') }

    $resolved = Resolve-CodexDDPythonPath -Candidates @($alias, $invalid, $valid)
    if ($resolved -cne $fakePython) {
        throw 'Did not select the usable Python application from multiple matches.'
    }
    if ((Resolve-CodexDDPythonPath -Candidates @($valid)) -cne $fakePython) {
        throw 'Single valid Python application was not selected.'
    }
    foreach ($invalidSet in @(@($alias), @($invalid))) {
        $rejected = $false
        try {
            $null = Resolve-CodexDDPythonPath -Candidates $invalidSet
        }
        catch {
            $rejected = $true
        }
        if (-not $rejected) {
            throw 'Unusable Python candidates were incorrectly accepted.'
        }
    }

    # Standardize format validation on the exact pinned stable toolchain
    # command that already passed on Daniel-CL. No unstable CLI override.
    foreach ($scriptName in @('codexdd-test-release.ps1', 'codexdd-test-workpacket.ps1')) {
        $scriptText = Get-Content -LiteralPath (Join-Path $PSScriptRoot $scriptName) -Raw
        if (-not $scriptText.Contains('ArgumentList @("fmt", "--all", "--", "--check")') -or
            $scriptText.Contains('imports_granularity=Item')) {
            throw "Unpinned/nonstandard rustfmt command in $scriptName"
        }
    }

    $attr = Get-Content -LiteralPath (Join-Path (Split-Path -Parent $PSScriptRoot) '.gitattributes') -Raw
    if (-not $attr.Contains('docs/upstream-candidates/*.json text eol=lf') -or
        -not $attr.Contains('docs/upstream-candidates/*.txt text eol=lf')) {
        throw 'Candidate metadata LF checkout attributes are missing.'
    }

    Write-Host 'CODEXDD_NATIVE_RELEASE_HARNESS_TEST PASS'
}
finally {
    Remove-Item -LiteralPath $fakePython -Force -ErrorAction SilentlyContinue
}
