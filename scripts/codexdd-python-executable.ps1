# Select exactly one usable Python application on Windows PowerShell 5.1.
# Get-Command python -All can include the nonfunctional WindowsApps alias.
function Resolve-CodexDDPythonPath {
    [CmdletBinding()]
    param([object[]]$Candidates)

    foreach ($candidate in @($Candidates)) {
        if ($null -eq $candidate) {
            continue
        }
        $path = [string]$candidate.Source
        if ([string]::IsNullOrWhiteSpace($path)) {
            continue
        }
        if ($path -match '(?i)[\\/]Microsoft[\\/]WindowsApps[\\/]python(?:\.exe)?$') {
            continue
        }
        if (Test-Path -LiteralPath $path -PathType Leaf) {
            return $path
        }
    }

    throw 'No usable Python executable found (excluding WindowsApps alias).'
}
