//! D02 read-only Windows listener/HTTP observation for an existing Core-owned child.
//!
//! This module never launches, terminates, adopts, signals or restarts a process.
//! It cannot authorize a port: the original Core owner must approve the exact
//! port/command/source before the process-store caller may request observation.
//! Even successful observation is not a trusted QA gate receipt.

use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;
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

/// An ephemeral observation, never a serializable completion receipt or gate.
#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct CoreOwnedLoopbackObservation {
    pub(crate) owner_pid: u32,
    pub(crate) approved_port: u16,
    pub(crate) observed_at_ms: u64,
}

fn exact_ipv4_loopback_owner(
    port: u16,
    rows: impl IntoIterator<Item = (u16, u32, u32)>,
) -> Result<u32, String> {
    if port == 0 {
        return Err("SETUP_BLOCKED: zero port is not an approved listener".to_string());
    }
    let mut matching = None;
    for (row_port, address, pid) in rows {
        if row_port != port {
            continue;
        }
        if address == 0 {
            return Err(
                "SETUP_BLOCKED: same-port wildcard listener is not exact loopback".to_string(),
            );
        }
        if address != 0x7f000001 {
            continue;
        }
        if pid == 0 {
            return Err("SETUP_BLOCKED: loopback listener has no owning PID".to_string());
        }
        if matching.replace(pid).is_some() {
            return Err("SETUP_BLOCKED: ambiguous loopback listener ownership".to_string());
        }
    }
    matching.ok_or_else(|| "SETUP_BLOCKED: exact IPv4 loopback listener is missing".to_string())
}

fn windows_loopback_owner(port: u16) -> Result<u32, String> {
    if port == 0 {
        return Err("SETUP_BLOCKED: zero port is not an approved listener".to_string());
    }
    let mut required_bytes = 0_u32;
    // SAFETY: initial Windows call only queries the bounded required size.
    let status = unsafe {
        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut required_bytes,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_LISTENER,
            0,
        )
    };
    if status != 122 || required_bytes < 4 || required_bytes as usize > MAX_TCP_TABLE_BYTES {
        return Err(format!(
            "SETUP_BLOCKED: Windows TCP owner table unavailable ({status})"
        ));
    }
    let mut data = vec![0_u32; (required_bytes as usize).div_ceil(4)];
    let mut length = (data.len() * 4) as u32;
    // SAFETY: DWORD-aligned owned buffer with supplied capacity; rows are
    // checked against returned bytes before unaligned C structure reads.
    let status = unsafe {
        GetExtendedTcpTable(
            data.as_mut_ptr().cast(),
            &mut length,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_LISTENER,
            0,
        )
    };
    if status != 0 || length < 4 || length as usize > data.len() * 4 {
        return Err(format!(
            "SETUP_BLOCKED: Windows TCP owner table read failed ({status})"
        ));
    }
    let count = data[0] as usize;
    let stride = std::mem::size_of::<MIB_TCPROW_OWNER_PID>();
    if count > ((length as usize) - 4) / stride {
        return Err("SETUP_BLOCKED: invalid TCP owner table row count".to_string());
    }
    let mut rows = Vec::with_capacity(count);
    for index in 0..count {
        // SAFETY: count and stride were bounded by returned byte length.
        let row = unsafe {
            data.as_ptr()
                .cast::<u8>()
                .add(4 + index * stride)
                .cast::<MIB_TCPROW_OWNER_PID>()
                .read_unaligned()
        };
        rows.push((
            u16::from_be(row.dwLocalPort as u16),
            u32::from_be(row.dwLocalAddr),
            row.dwOwningPid,
        ));
    }
    exact_ipv4_loopback_owner(port, rows)
}

fn parse_health_http_200(status: &[u8]) -> Result<(), String> {
    let line = std::str::from_utf8(status)
        .map_err(|_| "SETUP_BLOCKED: non-UTF8 health status".to_string())?;
    let mut tokens = line.split_whitespace();
    if !matches!(tokens.next(), Some("HTTP/1.0" | "HTTP/1.1")) || tokens.next() != Some("200") {
        return Err("SETUP_BLOCKED: owned health route did not return HTTP 200".to_string());
    }
    Ok(())
}

/// Called only after Core has checked its retained child handle and the
/// immutable original exec owner. Both Windows listener ownership and HTTP
/// availability are rechecked; this is still not independent QA authority.
pub(super) fn observe_read_only_owned_loopback(
    expected_pid: u32,
    approved_port: u16,
) -> Result<CoreOwnedLoopbackObservation, String> {
    if expected_pid == 0 || approved_port == 0 {
        return Err("SETUP_BLOCKED: missing exact owned PID or approved port".to_string());
    }
    if windows_loopback_owner(approved_port)? != expected_pid {
        return Err("SETUP_BLOCKED: approved listener is owned by a foreign process".to_string());
    }
    let addr = SocketAddr::from(([127, 0, 0, 1], approved_port));
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(600))
        .map_err(|error| format!("SETUP_BLOCKED: owned listener connection failed: {error}"))?;
    stream
        .set_write_timeout(Some(HTTP_DEADLINE))
        .map_err(|error| format!("SETUP_BLOCKED: health write deadline unavailable: {error}"))?;
    stream
        .write_all(
            format!(
                "GET /health HTTP/1.1\r\nHost: 127.0.0.1:{approved_port}\r\nConnection: close\r\n\r\n"
            )
            .as_bytes(),
        )
        .map_err(|error| format!("SETUP_BLOCKED: health request failed: {error}"))?;

    let deadline = Instant::now() + HTTP_DEADLINE;
    let mut status = Vec::new();
    loop {
        if Instant::now() >= deadline || status.len() >= MAX_HTTP_STATUS_BYTES {
            return Err("SETUP_BLOCKED: HTTP health status exceeded byte/time bounds".to_string());
        }
        stream
            .set_read_timeout(Some(deadline.saturating_duration_since(Instant::now())))
            .map_err(|error| format!("SETUP_BLOCKED: health read deadline unavailable: {error}"))?;
        let mut one = [0_u8; 1];
        stream
            .read_exact(&mut one)
            .map_err(|error| format!("SETUP_BLOCKED: incomplete fresh health status: {error}"))?;
        status.push(one[0]);
        if status.ends_with(b"\r\n") {
            break;
        }
    }
    parse_health_http_200(&status)?;
    drop(stream);
    if windows_loopback_owner(approved_port)? != expected_pid {
        return Err("SETUP_BLOCKED: listener owner drifted during HTTP probe".to_string());
    }
    let observed_at_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "SETUP_BLOCKED: health observation clock invalid".to_string())?
        .as_millis() as u64;
    Ok(CoreOwnedLoopbackObservation {
        owner_pid: expected_pid,
        approved_port,
        observed_at_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn codexdd_046_3d1d3_owner_filter_blocks_wildcard_duplicate_wrong_interface() {
        let port = 4331;
        let pid = 26884;
        assert_eq!(
            exact_ipv4_loopback_owner(
                port,
                [
                    (port, 0x7f000001, pid),
                    (4332, 0, 19),
                    (port, 0xc0a80102, 20)
                ],
            ),
            Ok(pid)
        );
        assert!(exact_ipv4_loopback_owner(0, [(port, 0x7f000001, pid)]).is_err());
        assert!(exact_ipv4_loopback_owner(port, [(port, 0xc0a80102, pid)]).is_err());
        assert!(exact_ipv4_loopback_owner(port, [(port, 0x7f000001, 0)]).is_err());
        assert!(
            exact_ipv4_loopback_owner(port, [(port, 0x7f000001, pid), (port, 0, 21)])
                .expect_err("same-port wildcard cannot be adopted")
                .contains("wildcard")
        );
        assert!(
            exact_ipv4_loopback_owner(port, [(port, 0x7f000001, pid), (port, 0x7f000001, pid)],)
                .expect_err("duplicate owner rows cannot be considered unique")
                .contains("ambiguous")
        );
    }

    #[test]
    fn codexdd_046_3d1d3_http_parser_requires_exact_200() {
        assert!(parse_health_http_200(b"HTTP/1.1 200 OK\r\n").is_ok());
        assert!(parse_health_http_200(b"HTTP/1.0 200\r\n").is_ok());
        for invalid in [
            b"HTTP/1.1 503 Unavailable\r\n".as_slice(),
            b"HTTP/1.1 201 Created\r\n".as_slice(),
            b"NOTHTTP 200 OK\r\n".as_slice(),
            b"HTTP/1.1 20 OK\r\n".as_slice(),
        ] {
            assert!(parse_health_http_200(invalid).is_err());
        }
    }

    #[test]
    fn codexdd_046_3d1d3_windows_live_local_200_and_503() {
        fn observe(status: &'static str) -> Result<CoreOwnedLoopbackObservation, String> {
            let listener = TcpListener::bind(("127.0.0.1", 0)).expect("disposable listener");
            let port = listener.local_addr().expect("bound address").port();
            listener.set_nonblocking(true).expect("nonblocking fixture");
            // Keep the owning listener open through the observer's second
            // Windows PID check, not just until the HTTP response is sent.
            let (release_tx, release_rx) = mpsc::channel();
            let service = thread::spawn(move || {
                let until = Instant::now() + Duration::from_secs(5);
                while Instant::now() < until {
                    match listener.accept() {
                        Ok((mut stream, _)) => {
                            stream
                                .set_read_timeout(Some(Duration::from_secs(2)))
                                .unwrap();
                            let mut request = [0_u8; 512];
                            let size = stream.read(&mut request).expect("health request");
                            assert!(request[..size].starts_with(b"GET /health HTTP/1.1"));
                            stream
                                .write_all(
                                    format!(
                                        "HTTP/1.1 {status}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                                    )
                                    .as_bytes(),
                                )
                                .expect("health response");
                            release_rx
                                .recv_timeout(Duration::from_secs(5))
                                .expect("release the listener after Core ownership recheck");
                            return;
                        }
                        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                            thread::sleep(Duration::from_millis(15));
                        }
                        Err(error) => panic!("unexpected disposable listener error: {error}"),
                    }
                }
                panic!("bounded local health fixture did not receive request");
            });
            let result = observe_read_only_owned_loopback(std::process::id(), port);
            release_tx.send(()).expect("release disposable listener");
            service.join().expect("owned fixture thread");
            result
        }
        let live = observe("200 OK").expect("Core-local PID and HTTP 200 must match");
        assert_eq!(live.owner_pid, std::process::id());
        assert_ne!(live.approved_port, 0);
        assert!(live.observed_at_ms > 0);
        assert!(
            observe("503 Service Unavailable")
                .expect_err("HTTP 503 must never satisfy native health")
                .contains("HTTP 200")
        );
    }
}
