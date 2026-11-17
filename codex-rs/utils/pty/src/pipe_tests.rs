use super::*;
use std::process::Stdio;

#[test]
fn process_fallback_interrupt_terminates_root() -> anyhow::Result<()> {
    let mut child = std::process::Command::new("ping.exe")
        .args(["-n", "60", "127.0.0.1"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let mut terminator = PipeChildTerminator {
        windows: WindowsChildTerminator::Process(child.id()),
    };

    terminator.signal(ProcessSignal::Interrupt)?;

    assert!(!child.wait()?.success());
    Ok(())
}

#[tokio::test]
async fn codexdd_046_3d1d2_real_windows_pipe_child_has_exact_os_pid_and_expires() -> anyhow::Result<()> {
    use std::collections::HashMap;
    use std::time::Duration;

    let args = [
        "-NoProfile",
        "-NonInteractive",
        "-Command",
        "Write-Output $PID; Start-Sleep -Milliseconds 1000",
    ]
    .map(str::to_string);
    let cwd = std::env::current_dir()?;
    let env: HashMap<String, String> = std::env::vars().collect();
    let mut spawned = spawn_process_no_stdin(
        "powershell.exe",
        &args,
        &cwd,
        &env,
        &None,
        &[],
    )
    .await?;
    let os_pid = spawned
        .session
        .live_local_os_pid()
        .expect("real Core-local process handle must hold an OS PID");
    assert_ne!(os_pid, 0);
    assert_ne!(os_pid, std::process::id());

    let mut output = Vec::new();
    let declared_pid = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let chunk = spawned.stdout_rx.recv().await.expect("child stdout closed");
            output.extend_from_slice(&chunk);
            if output.contains(&b'\n') {
                let line = String::from_utf8_lossy(&output);
                break line.lines().next().expect("PID line").trim().parse::<u32>()?;
            }
        }
        Ok::<u32, anyhow::Error>(0)
    })
    .await??;
    assert_eq!(os_pid, declared_pid, "must report the child's own OS PID");

    let exit_code = tokio::time::timeout(Duration::from_secs(15), spawned.exit_rx).await??;
    assert_eq!(exit_code, 0);
    assert_eq!(
        spawned.session.live_local_os_pid(),
        None,
        "exited child cannot serve as a fresh runtime-health witness"
    );
    Ok(())
}

#[tokio::test]
async fn codexdd_046_3d1d2_driver_backed_handle_cannot_claim_local_os_pid() {
    use crate::process::ProcessDriver;
    let (writer_tx, _writer_rx) = tokio::sync::mpsc::channel(1);
    let (out_tx, out_rx) = tokio::sync::broadcast::channel(2);
    let (exit_tx, exit_rx) = tokio::sync::oneshot::channel();
    let spawned = crate::spawn_from_driver(ProcessDriver {
        writer_tx,
        stdout_rx: out_rx,
        stderr_rx: None,
        exit_rx,
        terminator: None,
        writer_handle: None,
        resizer: None,
        tty: false,
    });
    assert_eq!(
        spawned.session.live_local_os_pid(),
        None,
        "synthetic/remote driver sessions must not invent an owned Windows PID"
    );
    let _ = exit_tx.send(0);
    drop(out_tx);
}
