//! Bounded Windows-only live HTTP host observation for owner-held preview children.
//!
//! This read-only probe cannot grant an Owner QA gate, restart a service, or
//! authorize a PID by its name. The caller must already own the live Child
//! handle, pin and owner-issued exact-port gate. Native gate evidence is a
//! separate integration step; no status/receipt is synthesized here.

use crate::adaptive_evidence::AdaptiveEvidenceRegistry;
use crate::adaptive_gate_prerequisites::AdaptiveGateLedger;
use crate::adaptive_gate_prerequisites::AdaptiveGateState;
use crate::adaptive_gate_prerequisites::AdaptiveOutstandingGate;
use crate::adaptive_gate_prerequisites::AdaptiveRequiredGate;
use crate::adaptive_worker::AdaptiveWorkerContext;
use crate::adaptive_worker::AdaptiveWorkerPreflight;
use crate::adaptive_worker::preflight_owned_runtime_target;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::path::Path;
use std::path::PathBuf;
use std::process::Child;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use windows_sys::Win32::NetworkManagement::IpHelper::GetExtendedTcpTable;
use windows_sys::Win32::NetworkManagement::IpHelper::MIB_TCPROW_OWNER_PID;
use windows_sys::Win32::NetworkManagement::IpHelper::TCP_TABLE_OWNER_PID_LISTENER;
use windows_sys::Win32::Networking::WinSock::AF_INET;

const MAX_TCP_TABLE_BYTES: usize = 512 * 1024;
const MAX_HTTP_STATUS_BYTES: usize = 512;
const HTTP_DEADLINE: Duration = Duration::from_secs(2);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct AdaptiveOwnedHttpObservation {
    pub(crate) source_sha: String,
    pub(crate) owned_pid: u32,
    pub(crate) approved_port: u16,
    pub(crate) http_status: u16,
    pub(crate) observed_at_ms: u64,
}

/// A selected-port wildcard listener must never masquerade as an exact
/// loopback-owned host. Pure classifier keeps the security decision testable
/// without depending on the Windows TCP table having a particular live shape.
fn require_exact_loopback_listener(
    port: u16,
    listeners: impl IntoIterator<Item = (u16, u32, u32)>,
) -> Result<u32, String> {
    if port == 0 {
        return Err("SETUP_BLOCKED: zero port is never an approved listener".to_string());
    }
    let mut matching = None;
    for (listener_port, local_address, owning_pid) in listeners {
        if listener_port != port {
            continue;
        }
        if local_address == 0 {
            return Err(
                "SETUP_BLOCKED: wildcard IPv4 listener on approved port has no exact loopback owner"
                    .to_string(),
            );
        }
        if local_address != 0x7f000001 {
            continue;
        }
        if owning_pid == 0 {
            return Err("SETUP_BLOCKED: loopback listener has no OS process owner".to_string());
        }
        if matching.replace(owning_pid).is_some() {
            return Err(
                "SETUP_BLOCKED: ambiguous listener owner for approved loopback port".to_string(),
            );
        }
    }
    matching.ok_or_else(|| {
        "SETUP_BLOCKED: no exact loopback TCP listener owns the approved port".to_string()
    })
}

/// Query the Windows owner-PID listener table directly, without shelling out
/// to an untrusted PATH command or trusting a stale host.pid file.
/// A same-port wildcard listener and an ambiguous owner both fail closed.
fn loopback_listener_owner(port: u16) -> Result<u32, String> {
    if port == 0 {
        return Err("SETUP_BLOCKED: zero port is never an approved listener".to_string());
    }
    let mut needed = 0_u32;
    // SAFETY: the first call only asks Windows for the required table size.
    let status = unsafe {
        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut needed,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_LISTENER,
            0,
        )
    };
    // Windows ERROR_INSUFFICIENT_BUFFER = 122. The first call is a size query.
    if status != 122 || needed < 4 || needed as usize > MAX_TCP_TABLE_BYTES {
        return Err(format!(
            "SETUP_BLOCKED: Windows TCP owner table size unavailable (status {status})"
        ));
    }
    // Align the backing allocation to at least DWORD alignment for the table.
    let mut table = vec![0_u32; (needed as usize).div_ceil(4)];
    let mut bytes = (table.len() * 4) as u32;
    // SAFETY: buffer is writable and aligned; Windows may write only bytes
    // up to the supplied capacity. We bounds-check row count before reading.
    let status = unsafe {
        GetExtendedTcpTable(
            table.as_mut_ptr().cast(),
            &mut bytes,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_LISTENER,
            0,
        )
    };
    if status != 0 || bytes < 4 || bytes as usize > table.len() * 4 {
        return Err(format!(
            "SETUP_BLOCKED: Windows TCP owner table query failed (status {status})"
        ));
    }
    let rows = table[0] as usize;
    let row_bytes = std::mem::size_of::<MIB_TCPROW_OWNER_PID>();
    if rows > ((bytes as usize) - 4) / row_bytes {
        return Err("SETUP_BLOCKED: invalid TCP listener owner table length".to_string());
    }
    let mut listeners = Vec::with_capacity(rows);
    for index in 0..rows {
        // SAFETY: the checked count bounds each fixed-size row within the
        // owned table allocation. Unaligned reads also work for the C row ABI.
        let row = unsafe {
            table
                .as_ptr()
                .cast::<u8>()
                .add(4 + index * row_bytes)
                .cast::<MIB_TCPROW_OWNER_PID>()
                .read_unaligned()
        };
        listeners.push((
            u16::from_be(row.dwLocalPort as u16),
            u32::from_be(row.dwLocalAddr),
            row.dwOwningPid,
        ));
    }
    require_exact_loopback_listener(port, listeners)
}

/// Observe a fresh HTTP 200 on /health only after exact original source/role,
/// approved port, live child identity and Windows listener owner all agree.
/// The listener-to-PID correlation is checked again after the response to
/// reduce process/port reuse races; no remote host or arbitrary route is sent.
pub(crate) fn observe_owned_loopback_health(
    worker: &AdaptiveWorkerContext,
    pin: Option<&AdaptiveWorkerPreflight>,
    ledger: Option<&AdaptiveGateLedger>,
    runtime_cwd: &Path,
    port: u16,
    reported_pid: u32,
    child: &mut Child,
) -> Result<AdaptiveOwnedHttpObservation, String> {
    preflight_owned_runtime_target(worker, pin, ledger, runtime_cwd, port, reported_pid, child)?;
    let pin = pin.ok_or_else(|| {
        "SETUP_BLOCKED: owned runtime health requires the exact authorized source pin".to_string()
    })?;
    if loopback_listener_owner(port)? != reported_pid {
        return Err("SETUP_BLOCKED: approved port is served by a foreign process".to_string());
    }
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let mut connection = TcpStream::connect_timeout(&addr, Duration::from_millis(600))
        .map_err(|error| format!("SETUP_BLOCKED: authorized listener did not connect: {error}"))?;
    connection
        .set_write_timeout(Some(HTTP_DEADLINE))
        .map_err(|error| format!("SETUP_BLOCKED: health write timeout unavailable: {error}"))?;
    connection
        .write_all(
            format!("GET /health HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n")
                .as_bytes(),
        )
        .map_err(|error| format!("SETUP_BLOCKED: HTTP health request failed: {error}"))?;
    let deadline = Instant::now() + HTTP_DEADLINE;
    let mut status_line = Vec::new();
    loop {
        if Instant::now() >= deadline || status_line.len() >= MAX_HTTP_STATUS_BYTES {
            return Err(
                "SETUP_BLOCKED: HTTP health status exceeded bounded deadline or size".to_string(),
            );
        }
        connection
            .set_read_timeout(Some(deadline.saturating_duration_since(Instant::now())))
            .map_err(|error| format!("SETUP_BLOCKED: HTTP read timeout unavailable: {error}"))?;
        let mut byte = [0u8; 1];
        connection.read_exact(&mut byte).map_err(|error| {
            format!("SETUP_BLOCKED: no complete fresh HTTP health response: {error}")
        })?;
        status_line.push(byte[0]);
        if status_line.ends_with(b"\r\n") {
            break;
        }
    }
    let line = std::str::from_utf8(&status_line)
        .map_err(|_| "SETUP_BLOCKED: invalid HTTP health status encoding".to_string())?;
    let mut tokens = line.split_whitespace();
    if !matches!(tokens.next(), Some("HTTP/1.0" | "HTTP/1.1")) || tokens.next() != Some("200") {
        return Err(
            "SETUP_BLOCKED: owner listener health endpoint did not return HTTP 200".to_string(),
        );
    }
    drop(connection);
    if loopback_listener_owner(port)? != reported_pid
        || child
            .try_wait()
            .map_err(|error| format!("SETUP_BLOCKED: child status unavailable: {error}"))?
            .is_some()
    {
        return Err(
            "SETUP_BLOCKED: runtime owner changed or exited during health check".to_string(),
        );
    }
    let observed_at_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "SETUP_BLOCKED: observation clock is unavailable".to_string())?
        .as_millis() as u64;
    Ok(AdaptiveOwnedHttpObservation {
        source_sha: pin.source_sha.clone(),
        owned_pid: reported_pid,
        approved_port: port,
        http_status: 200,
        observed_at_ms,
    })
}

/// A deliberately ephemeral, single-consumption capability derived from a
/// *real* owner-held Child and a fresh loopback HTTP 200 observation.
///
/// Its private fields, non-Clone type, and absence of Serialize prevent model
/// output or restored checkpoint data from masquerading as native observation.
/// This is NOT a Core-issued health receipt and NEVER satisfies a gate by itself.
#[derive(Debug)]
pub(crate) struct AdaptiveOwnedHealthTicket {
    authorized_scope: String,
    source_root: PathBuf,
    source_sha: String,
    owned_pid: u32,
    approved_port: u16,
    observed_at_ms: u64,
    nonce: uuid::Uuid,
}

const MAX_OWNED_HEALTH_TICKET_AGE_MS: u64 = 5_000;

/// Issue an in-process, non-persistable ticket only after native owner/source,
/// Windows TCP listener-PID correlation, and the HTTP 200 response all pass.
pub(crate) fn issue_owned_health_ticket(
    worker: &AdaptiveWorkerContext,
    pin: Option<&AdaptiveWorkerPreflight>,
    ledger: Option<&AdaptiveGateLedger>,
    runtime_cwd: &Path,
    port: u16,
    reported_pid: u32,
    child: &mut Child,
) -> Result<AdaptiveOwnedHealthTicket, String> {
    let observation =
        observe_owned_loopback_health(worker, pin, ledger, runtime_cwd, port, reported_pid, child)?;
    let pin = pin.ok_or_else(|| {
        "SETUP_BLOCKED: owned health ticket requires the exact authorized source pin".to_string()
    })?;
    let source_root = Path::new(&pin.source_root)
        .canonicalize()
        .map_err(|error| format!("SETUP_BLOCKED: verified source root disappeared: {error}"))?;
    let authorized_scope = worker
        .authorized_scope
        .as_deref()
        .filter(|scope| !scope.trim().is_empty())
        .ok_or_else(|| {
            "SETUP_BLOCKED: owned health ticket requires the original owner scope".to_string()
        })?
        .to_string();
    Ok(AdaptiveOwnedHealthTicket {
        authorized_scope,
        source_root,
        source_sha: observation.source_sha,
        owned_pid: observation.owned_pid,
        approved_port: observation.approved_port,
        observed_at_ms: observation.observed_at_ms,
        nonce: uuid::Uuid::new_v4(),
    })
}

/// Consume exactly one native ticket for a second, fresh guarded observation.
/// Source, owner scope, port, PID, source root and a five-second maximum age
/// must match. Neither tickets nor observations survive cold resume; a new
/// trusted Core producer and independently validated QA are required before
/// the OwnedRuntimeHealth ledger can ever become PASS.
pub(crate) fn consume_owned_health_ticket(
    ticket: AdaptiveOwnedHealthTicket,
    worker: &AdaptiveWorkerContext,
    pin: Option<&AdaptiveWorkerPreflight>,
    ledger: Option<&AdaptiveGateLedger>,
    runtime_cwd: &Path,
    port: u16,
    reported_pid: u32,
    child: &mut Child,
) -> Result<AdaptiveOwnedHttpObservation, String> {
    let pin = pin.ok_or_else(|| {
        "SETUP_BLOCKED: owned health ticket cannot be used without its source pin".to_string()
    })?;
    let root = Path::new(&pin.source_root)
        .canonicalize()
        .map_err(|error| format!("SETUP_BLOCKED: ticket source root is unavailable: {error}"))?;
    if ticket.nonce.is_nil()
        || ticket.source_root != root
        || ticket.source_sha != pin.source_sha
        || worker.authorized_scope.as_deref() != Some(ticket.authorized_scope.as_str())
        || reported_pid != ticket.owned_pid
        || port != ticket.approved_port
        || child.id() != ticket.owned_pid
    {
        return Err(
            "SETUP_BLOCKED: owned health ticket source, role, process or port drifted".to_string(),
        );
    }
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "SETUP_BLOCKED: ticket validation clock unavailable".to_string())?
        .as_millis() as u64;
    if now_ms
        .checked_sub(ticket.observed_at_ms)
        .is_none_or(|age| age > MAX_OWNED_HEALTH_TICKET_AGE_MS)
    {
        return Err("SETUP_BLOCKED: stale or future-dated runtime health ticket".to_string());
    }
    let fresh = observe_owned_loopback_health(
        worker,
        Some(pin),
        ledger,
        runtime_cwd,
        port,
        reported_pid,
        child,
    )?;
    if fresh.observed_at_ms < ticket.observed_at_ms {
        return Err("SETUP_BLOCKED: health response predates its original observation".to_string());
    }
    Ok(fresh)
}

/// A one-time, source-bound native observation evaluated at the QA boundary.
/// This is intentionally not an Owner QA PASS or a durable gate receipt:
/// the regular gate ledger still reports OwnedRuntimeHealth as pending until
/// a Core-bound independently revalidated native producer exists.
#[derive(Debug)]
pub(crate) struct AdaptiveOwnedRuntimeQaReview {
    pub(crate) observation: AdaptiveOwnedHttpObservation,
    pub(crate) other_pending_gates: Vec<AdaptiveOutstandingGate>,
}

/// Evaluate the original owner health requirement while an authorized
/// same-process Child is available. An explicit single-port gate is required;
/// until multi-port process ownership is implemented, never infer a PASS for
/// two services from one listener.
///
/// This issues a fresh private ticket, consumes it for a *second* Windows
/// owner-PID/HTTP 200 inspection, and exposes what other gates remain pending.
/// No durable registry record is written and ready_for_owner_qa remains
/// deliberately unchanged; a later cold resume cannot replay this review.
pub(crate) fn review_live_owned_runtime_at_qa(
    worker: &AdaptiveWorkerContext,
    pin: Option<&AdaptiveWorkerPreflight>,
    ledger: Option<&AdaptiveGateLedger>,
    registry: &AdaptiveEvidenceRegistry,
    runtime_cwd: &Path,
    port: u16,
    reported_pid: u32,
    child: &mut Child,
) -> Result<AdaptiveOwnedRuntimeQaReview, String> {
    let ledger = ledger.ok_or_else(|| {
        "SETUP_BLOCKED: the original owner runtime-health gate is unavailable".to_string()
    })?;
    let outstanding = ledger.outstanding(registry);
    let health = AdaptiveRequiredGate::OwnedRuntimeHealth(vec![port]);
    let runtime_gates = outstanding
        .iter()
        .filter(|entry| matches!(entry.gate, AdaptiveRequiredGate::OwnedRuntimeHealth(_)))
        .collect::<Vec<_>>();
    if runtime_gates.len() != 1
        || runtime_gates[0].gate != health
        || runtime_gates[0].state != AdaptiveGateState::Pending
    {
        return Err(
            "SETUP_BLOCKED: owner runtime-health review requires one unblocked exact approved port"
                .to_string(),
        );
    }
    let ticket = issue_owned_health_ticket(
        worker,
        pin,
        Some(ledger),
        runtime_cwd,
        port,
        reported_pid,
        child,
    )?;
    let observation = consume_owned_health_ticket(
        ticket,
        worker,
        pin,
        Some(ledger),
        runtime_cwd,
        port,
        reported_pid,
        child,
    )?;
    Ok(AdaptiveOwnedRuntimeQaReview {
        observation,
        other_pending_gates: outstanding
            .into_iter()
            .filter(|entry| entry.gate != health)
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptive_worker::AdaptiveWorkerRole;
    use crate::adaptive_worker::parse_adaptive_owner_assignment;
    use std::net::TcpListener;
    use std::process::Command;
    use std::process::Stdio;
    use std::thread;

    // Only launched explicitly in a separate real OS test-binary process.
    // It never runs as a normal member of the native acceptance test filter.
    #[test]
    #[ignore = "explicit owned child fixture"]
    fn native_owned_loopback_test_server() {
        let port: u16 = std::env::var("CODEXDD_046_HEALTH_FIXTURE_PORT")
            .expect("child-approved port")
            .parse()
            .expect("numeric child port");
        let status = std::env::var("CODEXDD_046_HEALTH_FIXTURE_STATUS").expect("child HTTP status");
        let listener = TcpListener::bind(("127.0.0.1", port)).expect("owned loopback listener");
        listener.set_nonblocking(true).expect("nonblocking fixture");
        let expires = Instant::now() + Duration::from_secs(12);
        while Instant::now() < expires {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream
                        .set_read_timeout(Some(Duration::from_secs(2)))
                        .unwrap();
                    let mut input = [0u8; 512];
                    let received = stream.read(&mut input).unwrap_or_default();
                    let requested = String::from_utf8_lossy(&input[..received]);
                    let response_status = if requested.starts_with("GET /health HTTP/1.") {
                        status.as_str()
                    } else {
                        "404 Not Found"
                    };
                    let response = format!(
                        "HTTP/1.0 {response_status}\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK"
                    );
                    stream
                        .write_all(response.as_bytes())
                        .expect("test response");
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(20));
                }
                Err(error) => panic!("test listener acceptance failed: {error}"),
            }
        }
    }

    fn test_owner(
        status: &str,
    ) -> (
        tempfile::TempDir,
        String,
        AdaptiveWorkerContext,
        AdaptiveWorkerPreflight,
        AdaptiveGateLedger,
        u16,
        Child,
    ) {
        let root = tempfile::tempdir().expect("disposable source root");
        let git = |args: &[&str]| -> String {
            let result = Command::new("git")
                .arg("-C")
                .arg(root.path())
                .args(args)
                .output()
                .expect("Git available");
            assert!(result.status.success(), "fixture Git failed");
            String::from_utf8(result.stdout).expect("UTF-8 Git")
        };
        git(&["init", "-q"]);
        git(&[
            "-c",
            "user.name=CodexDD test",
            "-c",
            "user.email=codexdd@example.test",
            "commit",
            "--allow-empty",
            "-q",
            "-m",
            "isolated source",
        ]);
        let sha = git(&["rev-parse", "HEAD"]).trim().to_string();
        let listener = TcpListener::bind("127.0.0.1:0").expect("allocate spare loopback port");
        let port = listener.local_addr().expect("port").port();
        drop(listener);
        let assignment = format!(
            "[adaptive_worker]\nrole = \"implementation\"\nauthorized_scope = \"disposable host preview\"\nsource_root = '{}'\nsource_sha = \"{sha}\"\ntask_kind = \"operations_preview\"\nrequired_gates = [{{ kind = \"owned_runtime_health\", ports = [{port}] }}, {{ kind = \"independent_validation\" }}]\n\nCheck only this disposable service.",
            root.path().display()
        );
        let (owner, pin, gates) = parse_adaptive_owner_assignment(&assignment)
            .expect("valid owner contract")
            .expect("owner admission");
        let mut child = Command::new(std::env::current_exe().expect("self test binary"))
            .arg("--ignored")
            .arg("--exact")
            .arg("adaptive_owned_runtime::tests::native_owned_loopback_test_server")
            .env("CODEXDD_046_HEALTH_FIXTURE_PORT", port.to_string())
            .env("CODEXDD_046_HEALTH_FIXTURE_STATUS", status)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn isolated owned test child");
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if loopback_listener_owner(port).ok() == Some(child.id()) {
                break;
            }
            assert!(Instant::now() < deadline, "fixture listener did not start");
            assert!(
                child.try_wait().expect("test child status").is_none(),
                "fixture child exited"
            );
            thread::sleep(Duration::from_millis(50));
        }
        (
            root,
            sha,
            owner,
            pin.expect("source pin"),
            gates.expect("owner gates"),
            port,
            child,
        )
    }

    #[test]
    fn codexdd_046_3d1b2b2_exact_loopback_listener_refuses_wildcard_and_ambiguity() {
        const PORT: u16 = 4331;
        const LOOPBACK: u32 = 0x7f000001;
        const LAN: u32 = 0xc0a80101;
        const PID: u32 = 24680;
        assert_eq!(
            require_exact_loopback_listener(
                PORT,
                [(PORT, LOOPBACK, PID), (4332, 0, 1234), (PORT, LAN, 4321)],
            ),
            Ok(PID)
        );
        assert!(require_exact_loopback_listener(0, [(PORT, LOOPBACK, PID)]).is_err());
        assert!(require_exact_loopback_listener(PORT, [(PORT, LAN, PID)]).is_err());
        assert!(require_exact_loopback_listener(PORT, [(PORT, LOOPBACK, 0)]).is_err());
        assert!(
            require_exact_loopback_listener(PORT, [(PORT, LOOPBACK, PID), (PORT, 0, 1234)],)
                .expect_err("same-port wildcard must refuse exact ownership")
                .contains("wildcard")
        );
        assert!(
            require_exact_loopback_listener(PORT, [(PORT, LOOPBACK, PID), (PORT, LOOPBACK, PID)],)
                .expect_err("duplicate loopback rows are ambiguous")
                .contains("ambiguous")
        );
    }

    #[test]
    fn codexdd_046_3d1b2b2_review_rechecks_owned_health_without_minting_ready() {
        let (root, sha, owner, pin, gates, port, mut child) = test_owner("200 OK");
        let registry = AdaptiveEvidenceRegistry::default();
        let pid = child.id();
        let reviewed = review_live_owned_runtime_at_qa(
            &owner,
            Some(&pin),
            Some(&gates),
            &registry,
            root.path(),
            port,
            pid,
            &mut child,
        )
        .expect("approved live owner can obtain a non-persisted QA observation");
        assert_eq!(reviewed.observation.source_sha, sha);
        assert_eq!(reviewed.observation.owned_pid, pid);
        assert_eq!(reviewed.observation.approved_port, port);
        assert_eq!(reviewed.observation.http_status, 200);
        assert!(reviewed.observation.observed_at_ms > 0);
        assert_eq!(reviewed.other_pending_gates.len(), 1);
        assert_eq!(
            reviewed.other_pending_gates[0].gate,
            AdaptiveRequiredGate::IndependentValidation
        );
        assert_eq!(
            reviewed.other_pending_gates[0].state,
            AdaptiveGateState::Pending
        );
        // This read-only QA view never persists or produces a trusted Core
        // health receipt. Original ledger continues to block READY.
        assert!(!gates.ready_for_owner_qa(&registry));
        assert!(
            gates
                .outstanding(&registry)
                .iter()
                .any(|gate| { gate.gate == AdaptiveRequiredGate::OwnedRuntimeHealth(vec![port]) })
        );
        child.kill().expect("kill only disposable owned fixture");
        child.wait().expect("reap only disposable fixture");
        assert!(
            review_live_owned_runtime_at_qa(
                &owner,
                Some(&pin),
                Some(&gates),
                &registry,
                root.path(),
                port,
                pid,
                &mut child,
            )
            .is_err()
        );
    }

    #[test]
    fn codexdd_046_3d1b2b2_review_refuses_blocked_and_unapproved_ports() {
        let (root, _sha, owner, pin, mut gates, port, mut child) = test_owner("200 OK");
        let registry = AdaptiveEvidenceRegistry::default();
        let pid = child.id();
        let wrong_port = port.wrapping_add(1);
        assert!(
            review_live_owned_runtime_at_qa(
                &owner,
                Some(&pin),
                Some(&gates),
                &registry,
                root.path(),
                wrong_port,
                pid,
                &mut child,
            )
            .expect_err("foreign port is not an owner-issued single-port gate")
            .contains("one unblocked exact approved port")
        );
        gates
            .mark_blocked(
                &AdaptiveRequiredGate::OwnedRuntimeHealth(vec![port]),
                "owner_lock_hold",
            )
            .expect("owner gate block");
        assert!(
            review_live_owned_runtime_at_qa(
                &owner,
                Some(&pin),
                Some(&gates),
                &registry,
                root.path(),
                port,
                pid,
                &mut child,
            )
            .expect_err("blocked owner gate cannot be cleared by HTTP 200")
            .contains("one unblocked exact approved port")
        );
        assert!(!gates.ready_for_owner_qa(&registry));
        child.kill().expect("only disposable owned fixture");
        child.wait().expect("reap only disposable fixture");
    }

    #[test]
    fn codexdd_046_3d1b2b1_native_ticket_is_single_use_and_still_not_gate_pass() {
        let (root, sha, owner, pin, gates, port, mut child) = test_owner("200 OK");
        let ticket = issue_owned_health_ticket(
            &owner,
            Some(&pin),
            Some(&gates),
            root.path(),
            port,
            child.id(),
            &mut child,
        )
        .expect("original native observation can issue ephemeral ticket");
        assert_ne!(ticket.nonce, uuid::Uuid::nil());
        assert_eq!(ticket.source_sha, sha);
        let fresh = consume_owned_health_ticket(
            ticket,
            &owner,
            Some(&pin),
            Some(&gates),
            root.path(),
            port,
            child.id(),
            &mut child,
        )
        .expect("ticket consumption must execute a fresh OS and HTTP check");
        assert_eq!(fresh.http_status, 200);
        assert_eq!(fresh.owned_pid, child.id());
        assert!(!gates.ready_for_owner_qa(&Default::default()));
        child
            .kill()
            .expect("terminate only disposable owned fixture");
        child.wait().expect("reap only disposable owned fixture");
    }

    #[test]
    fn codexdd_046_3d1b2b1_ticket_refuses_stale_wrong_source_port_and_dead_child() {
        let (root, _sha, owner, pin, gates, port, mut child) = test_owner("200 OK");
        let mut stale = issue_owned_health_ticket(
            &owner,
            Some(&pin),
            Some(&gates),
            root.path(),
            port,
            child.id(),
            &mut child,
        )
        .expect("native ticket");
        stale.observed_at_ms = 0;
        assert!(
            consume_owned_health_ticket(
                stale,
                &owner,
                Some(&pin),
                Some(&gates),
                root.path(),
                port,
                child.id(),
                &mut child,
            )
            .expect_err("stale ticket cannot be replayed")
            .contains("stale")
        );

        let wrong_source = issue_owned_health_ticket(
            &owner,
            Some(&pin),
            Some(&gates),
            root.path(),
            port,
            child.id(),
            &mut child,
        )
        .expect("native ticket");
        let mut drifted = pin.clone();
        drifted.source_sha = "f".repeat(40);
        assert!(
            consume_owned_health_ticket(
                wrong_source,
                &owner,
                Some(&drifted),
                Some(&gates),
                root.path(),
                port,
                child.id(),
                &mut child,
            )
            .expect_err("source drift cannot borrow a health ticket")
            .contains("drifted")
        );

        let wrong_port = issue_owned_health_ticket(
            &owner,
            Some(&pin),
            Some(&gates),
            root.path(),
            port,
            child.id(),
            &mut child,
        )
        .expect("native ticket");
        assert!(
            consume_owned_health_ticket(
                wrong_port,
                &owner,
                Some(&pin),
                Some(&gates),
                root.path(),
                port.wrapping_add(1),
                child.id(),
                &mut child,
            )
            .expect_err("port drift cannot borrow health proof")
            .contains("drifted")
        );

        let dead = issue_owned_health_ticket(
            &owner,
            Some(&pin),
            Some(&gates),
            root.path(),
            port,
            child.id(),
            &mut child,
        )
        .expect("native ticket");
        child.kill().expect("only owned test child");
        child.wait().expect("reap owned test child");
        assert!(
            consume_owned_health_ticket(
                dead,
                &owner,
                Some(&pin),
                Some(&gates),
                root.path(),
                port,
                child.id(),
                &mut child,
            )
            .expect_err("exited process must not inherit old health")
            .contains("has exited")
        );
        assert!(!gates.ready_for_owner_qa(&Default::default()));
    }

    #[test]
    fn codexdd_046_3d1b2_owned_listener_http_200_is_fresh_and_not_a_gate_pass() {
        let (root, sha, owner, pin, gates, port, mut child) = test_owner("200 OK");
        assert_eq!(owner.role, AdaptiveWorkerRole::Implementation);
        let proof = observe_owned_loopback_health(
            &owner,
            Some(&pin),
            Some(&gates),
            root.path(),
            port,
            child.id(),
            &mut child,
        )
        .expect("same-owner live HTTP 200");
        assert_eq!(proof.source_sha, sha);
        assert_eq!(proof.owned_pid, child.id());
        assert_eq!(proof.approved_port, port);
        assert_eq!(proof.http_status, 200);
        assert!(proof.observed_at_ms > 0);
        assert!(!gates.ready_for_owner_qa(&Default::default()));
        child.kill().expect("kill only our disposable fixture");
        child.wait().expect("reap only our fixture");
        assert!(
            observe_owned_loopback_health(
                &owner,
                Some(&pin),
                Some(&gates),
                root.path(),
                port,
                child.id(),
                &mut child
            )
            .is_err()
        );
    }

    #[test]
    fn codexdd_046_3d1b2_non_200_ownership_health_refused_without_gate_credit() {
        let (root, _sha, owner, pin, gates, port, mut child) =
            test_owner("503 Service Unavailable");
        let result = observe_owned_loopback_health(
            &owner,
            Some(&pin),
            Some(&gates),
            root.path(),
            port,
            child.id(),
            &mut child,
        );
        assert!(
            result
                .expect_err("service failure cannot pass")
                .contains("HTTP 200")
        );
        assert!(!gates.ready_for_owner_qa(&Default::default()));
        child.kill().expect("kill only our disposable fixture");
        child.wait().expect("reap only our fixture");
    }
}
