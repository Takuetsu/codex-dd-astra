# CodexDD 0.4.5 3F.94 NONRELEASE. Safe no-execution test of native Windows
# detached CREATE_BREAKAWAY_FROM_JOB, under actual standard-user token.
# The child process is suspended and always killed/reaped; no daemon starts.
param([Parameter(Mandatory=$true)][string]$SandboxPath)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
if (-not $SandboxPath -or -not (Test-Path -LiteralPath $SandboxPath -PathType Container)) {
  throw '3F.94 breakaway probe requires an existing isolated sandbox'
}
$path = Join-Path $SandboxPath 'breakaway-result.json'
try {
  Add-Type -TypeDefinition @'
using System;
using System.ComponentModel;
using System.Runtime.InteropServices;
public static class DdBreakawayProbe {
    [StructLayout(LayoutKind.Sequential, CharSet=CharSet.Unicode)]
    struct StartupInfo {
        public int cb;
        public IntPtr lpReserved, lpDesktop, lpTitle;
        public int dwX, dwY, dwXSize, dwYSize, dwXCountChars, dwYCountChars, dwFillAttribute, dwFlags;
        public short wShowWindow, cbReserved2;
        public IntPtr lpReserved2, hStdInput, hStdOutput, hStdError;
    }
    [StructLayout(LayoutKind.Sequential)]
    struct ProcessInfo {
        public IntPtr hProcess, hThread;
        public int dwProcessId, dwThreadId;
    }
    [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
    static extern bool CreateProcessW(string app, string cmd, IntPtr procAttrs, IntPtr threadAttrs, bool inheritHandles, uint flags, IntPtr env, string currentDir, ref StartupInfo si, out ProcessInfo pi);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool TerminateProcess(IntPtr process, uint exitCode);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern uint WaitForSingleObject(IntPtr handle, uint timeout);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool CloseHandle(IntPtr handle);
    [DllImport("kernel32.dll")]
    static extern IntPtr GetCurrentProcess();
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool IsProcessInJob(IntPtr process, IntPtr job, out bool inJob);
    [DllImport("advapi32.dll", SetLastError=true)]
    static extern bool OpenProcessToken(IntPtr process, uint access, out IntPtr token);
    [DllImport("advapi32.dll", SetLastError=true)]
    static extern bool GetTokenInformation(IntPtr token, int kind, out int value, int length, out int returned);
    public static int TokenElevation() {
        IntPtr token;
        if (!OpenProcessToken(GetCurrentProcess(), 8, out token))
            throw new Win32Exception(Marshal.GetLastWin32Error());
        try {
            int value, length;
            if (!GetTokenInformation(token, 20, out value, 4, out length))
                throw new Win32Exception(Marshal.GetLastWin32Error());
            if (length != 4) throw new Exception("Invalid TokenElevation size");
            return value;
        } finally { CloseHandle(token); }
    }
    public static bool InJob() {
        bool inJob;
        if (!IsProcessInJob(GetCurrentProcess(), IntPtr.Zero, out inJob))
            throw new Win32Exception(Marshal.GetLastWin32Error());
        return inJob;
    }
    public static int BreakawayErrorCode(string applicationPath) {
        StartupInfo si = new StartupInfo();
        si.cb = Marshal.SizeOf(typeof(StartupInfo));
        ProcessInfo pi;
        // Same flags as codex-rs/app-server-daemon/src/backend/windows.rs,
        // with a known trusted system executable, CREATED SUSPENDED.
        const uint CREATE_SUSPENDED=0x4;
        const uint DETACHED_PROCESS=0x8;
        const uint CREATE_BREAKAWAY_FROM_JOB=0x01000000;
        if (!CreateProcessW(applicationPath, null, IntPtr.Zero, IntPtr.Zero, false,
             CREATE_SUSPENDED | DETACHED_PROCESS | CREATE_BREAKAWAY_FROM_JOB,
             IntPtr.Zero, null, ref si, out pi))
            return Marshal.GetLastWin32Error();
        try {
            if (!TerminateProcess(pi.hProcess, 1))
                throw new Win32Exception(Marshal.GetLastWin32Error());
            if (WaitForSingleObject(pi.hProcess, 15000) != 0)
                throw new Exception("Could not reap suspended breakaway probe");
            return 0;
        } finally {
            CloseHandle(pi.hThread);
            CloseHandle(pi.hProcess);
        }
    }
}
'@
  $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
  $principal = [Security.Principal.WindowsPrincipal]::new($identity)
  $elevated = [DdBreakawayProbe]::TokenElevation()
  $jobMember = [DdBreakawayProbe]::InJob()
  $cmd = Join-Path $env:WINDIR 'System32\cmd.exe'
  if ($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator) -or $elevated -ne 0) {
    throw '3F.94 scheduled-task process is elevated; cannot safely test daemon'
  }
  if (-not (Test-Path -LiteralPath $cmd -PathType Leaf)) { throw 'System32 cmd.exe missing' }
  $code = [DdBreakawayProbe]::BreakawayErrorCode($cmd)
  $receipt = [pscustomobject]@{
    kind='codexdd045_3f94_scheduled_task_breakaway_preflight_nonrelease'
    identity=$identity.Name
    token_elevated=$elevated
    member_of_job=$jobMember
    breakaway_error_code=$code
    detached_launch_possible=($code -eq 0)
    child_executed=$false
    production_security_changed=$false
    utc=(Get-Date).ToUniversalTime().ToString('o')
  }
  $receipt | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath $path -Encoding UTF8
  if ($code -ne 0) {
    throw "3F.94 scheduled-task detached CREATE_BREAKAWAY_FROM_JOB probe failed with Win32 code $code"
  }
  Write-Host "3F.94 SCHEDULED-TASK NONADMIN BREAKAWAY GREEN (inJob=$jobMember)"
} catch {
  if (-not (Test-Path -LiteralPath $path)) {
    [pscustomobject]@{kind='codexdd045_3f94_scheduled_task_breakaway_preflight_nonrelease';success=$false;reason=$_.Exception.Message} |
      ConvertTo-Json -Depth 3 | Set-Content -LiteralPath $path -Encoding UTF8
  }
  throw
}
