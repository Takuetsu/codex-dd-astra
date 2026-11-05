# NONRELEASE 3F.94 fixture: create the real Windows daemon's private socket root
# using the same owner-only protected SDDL and CreateDirectoryW as codex-rs/uds.
# Never relax or repair permissions on a preexisting untrusted directory.
$ErrorActionPreference = 'Stop'
Add-Type -TypeDefinition @'
using System;
using System.ComponentModel;
using System.Runtime.InteropServices;
public static class CodexDdPrivateDirectory {
    [StructLayout(LayoutKind.Sequential)]
    private struct SecurityAttributes {
        public UInt32 nLength;
        public IntPtr lpSecurityDescriptor;
        public int bInheritHandle;
    }
    [DllImport("advapi32.dll", EntryPoint = "ConvertStringSecurityDescriptorToSecurityDescriptorW", CharSet = CharSet.Unicode, SetLastError = true)]
    private static extern bool ConvertSddl(string sddl, uint revision, out IntPtr descriptor, out uint length);
    [DllImport("kernel32.dll", EntryPoint = "CreateDirectoryW", CharSet = CharSet.Unicode, SetLastError = true)]
    private static extern bool CreateDirectory(string path, ref SecurityAttributes attributes);
    [DllImport("kernel32.dll", EntryPoint = "LocalFree", SetLastError = true)]
    private static extern IntPtr LocalFree(IntPtr descriptor);
    public static void Create(string path, string userSid) {
        if (String.IsNullOrWhiteSpace(path) || String.IsNullOrWhiteSpace(userSid)) throw new ArgumentException("Missing private directory or user SID");
        string sddl = "O:" + userSid + "D:P(A;OICI;FA;;;" + userSid + ")";
        IntPtr descriptor = IntPtr.Zero;
        uint length;
        if (!ConvertSddl(sddl, 1, out descriptor, out length)) throw new Win32Exception(Marshal.GetLastWin32Error());
        try {
            SecurityAttributes attributes = new SecurityAttributes {
                nLength = (UInt32)Marshal.SizeOf(typeof(SecurityAttributes)),
                lpSecurityDescriptor = descriptor,
                bInheritHandle = 0
            };
            if (!CreateDirectory(path, ref attributes)) throw new Win32Exception(Marshal.GetLastWin32Error());
        } finally { LocalFree(descriptor); }
    }
}
'@

function New-DDPrivateDaemonDirectory([string]$sandboxHome) {
  $state = Join-Path $sandboxHome 'app-server-daemon'
  if (-not (Test-Path -LiteralPath $sandboxHome -PathType Container)) {
    throw 'Private daemon state parent must already exist inside an isolated test home'
  }
  $sid = [Security.Principal.WindowsIdentity]::GetCurrent().User
  if (-not (Test-Path -LiteralPath $state)) {
    # SECURITY_ATTRIBUTES set at creation time: no insecure intermediate directory.
    [CodexDdPrivateDirectory]::Create([IO.Path]::GetFullPath($state), $sid.Value)
  }
  # Validate exactly the protected, single inheritable owner-only ACE before use.
  # The actual daemon also independently validates this through its native Win32 code.
  $acl = Get-Acl -LiteralPath $state
  if (-not $acl.AreAccessRulesProtected) { throw '3F.94 private daemon directory inherits permissions' }
  if (-not $acl.GetOwner([Security.Principal.SecurityIdentifier]).Equals($sid)) {
    throw '3F.94 private daemon directory owned by another Windows account'
  }
  $rules = @($acl.GetAccessRules($true, $false, [Security.Principal.SecurityIdentifier]))
  if ($rules.Count -ne 1) { throw "3F.94 expected exactly one owner ACE, observed $($rules.Count)" }
  $rule = $rules[0]
  $flags = [Security.AccessControl.InheritanceFlags]::ContainerInherit -bor [Security.AccessControl.InheritanceFlags]::ObjectInherit
  if (-not $rule.IdentityReference.Equals($sid) -or
      $rule.AccessControlType -ne [Security.AccessControl.AccessControlType]::Allow -or
      $rule.InheritanceFlags -ne $flags -or
      $rule.PropagationFlags -ne [Security.AccessControl.PropagationFlags]::None -or
      $rule.FileSystemRights -ne [Security.AccessControl.FileSystemRights]::FullControl -or
      $rule.IsInherited) {
    throw '3F.94 daemon socket directory does not match strict owner-only SDDL'
  }
  Write-Host '3F.94 WINDOWS PRIVATE DAEMON SOCKET DIRECTORY VERIFIED'
}
