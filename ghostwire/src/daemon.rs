//! Daemon management — start, stop, status, PID file handling
//!
//! The GhostWire daemon runs the Core engine (mesh networking, encryption,
//! message routing) as a background process. Clients (TUI, CLI) connect
//! to it via HTTP/WebSocket — no database lock conflicts.
//!
//! Usage:
//!   ghostwire start        — start the daemon in background
//!   ghostwire stop         — stop the running daemon
//!   ghostwire status       — check if daemon is running
//!   ghostwire tui          — launch TUI (auto-starts daemon if needed)

use anyhow::{anyhow, Result};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Default daemon port
pub const DEFAULT_DAEMON_PORT: u16 = 9097;

fn daemon_args_path() -> PathBuf {
    ghostwire_home().join("daemon-args.json")
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SavedDaemonArgs {
    #[serde(default = "default_daemon_port")]
    pub api_port: u16,
    #[serde(default = "default_api_host")]
    pub api_host: String,
    pub tailscale_bootstrap: Vec<String>,
    pub bootstrap: Vec<String>,
    pub mesh_port: u16,
    #[serde(default = "default_quic_port")]
    pub quic_port: u16,
}

pub fn save_daemon_args(args: &DaemonArgs, api_port: u16) -> Result<()> {
    let saved = SavedDaemonArgs {
        api_port,
        api_host: args.api_host.clone(),
        tailscale_bootstrap: args.tailscale_bootstrap.clone(),
        bootstrap: args.bootstrap.clone(),
        mesh_port: args.mesh_port,
        quic_port: args.quic_port,
    };
    let json = serde_json::to_string_pretty(&saved)?;
    fs::write(daemon_args_path(), json)?;
    Ok(())
}

pub fn load_saved_args() -> Option<DaemonArgs> {
    let path = daemon_args_path();
    if !path.exists() {
        return None;
    }
    let json = fs::read_to_string(&path).ok()?;
    let saved: SavedDaemonArgs = serde_json::from_str(&json).ok()?;
    Some(DaemonArgs {
        api_host: saved.api_host,
        tailscale_bootstrap: saved.tailscale_bootstrap,
        bootstrap: saved.bootstrap,
        mesh_port: saved.mesh_port,
        quic_port: saved.quic_port,
    })
}

/// Get the GhostWire home directory
fn ghostwire_home() -> PathBuf {
    if let Ok(home) = std::env::var("GHOSTWIRE_HOME") {
        PathBuf::from(home)
    } else {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        home.join(".ghostwire")
    }
}

/// Path to the daemon PID file
pub fn pid_file_path() -> PathBuf {
    ghostwire_home().join("daemon.pid")
}

/// Path to the daemon log file
pub fn daemon_log_path() -> PathBuf {
    ghostwire_home().join("daemon.log")
}

/// Return true when a process exists for a PID.
fn process_exists(pid: i32) -> bool {
    #[cfg(unix)]
    unsafe {
        libc::kill(pid, 0) == 0
    }
    #[cfg(not(unix))]
    {
        pid > 0
    }
}
/// Get the daemon PID scoped to the current GHOSTWIRE_HOME.
/// Never search the whole machine: another profile or checkout may be active.
pub fn get_daemon_pid() -> Option<i32> {
    let pid_str = fs::read_to_string(pid_file_path()).ok()?;
    let pid = pid_str.trim().parse::<i32>().ok()?;
    process_exists(pid).then_some(pid)
}
/// Check if the daemon for the current GhostWire home is running.
pub fn is_daemon_running() -> bool {
    get_daemon_pid().is_some()
}
/// Extra args to forward to the daemon child process
#[derive(Default)]
pub struct DaemonArgs {
    pub api_host: String,
    pub tailscale_bootstrap: Vec<String>,
    pub bootstrap: Vec<String>,
    pub mesh_port: u16,
    pub quic_port: u16,
}

fn default_daemon_port() -> u16 {
    DEFAULT_DAEMON_PORT
}

fn default_api_host() -> String {
    "127.0.0.1".to_string()
}
fn default_quic_port() -> u16 {
    8081
}

/// Wait until the daemon is serving a successful HTTP health response.
/// A PID only proves that the child exists; this proves the API is usable.
pub fn wait_for_ready(port: u16, timeout: Duration) -> Result<()> {
    let deadline = Instant::now() + timeout;
    let address = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let request = format!(
        "GET /api/v2/ready HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nConnection: close\r\n\r\n",
        port
    );

    while Instant::now() < deadline {
        if let Ok(mut stream) =
            std::net::TcpStream::connect_timeout(&address, Duration::from_millis(250))
        {
            stream
                .set_read_timeout(Some(Duration::from_millis(500)))
                .ok();
            stream
                .set_write_timeout(Some(Duration::from_millis(500)))
                .ok();
            if stream.write_all(request.as_bytes()).is_ok() {
                let mut response = [0u8; 256];
                if let Ok(size) = stream.read(&mut response) {
                    let head = String::from_utf8_lossy(&response[..size]);
                    if head.starts_with("HTTP/") && head.contains(" 200 ") {
                        return Ok(());
                    }
                }
            }
        }
        std::thread::sleep(Duration::from_millis(200));
    }

    Err(anyhow!(
        "Daemon did not become ready on port {} within {} seconds; check {}",
        port,
        timeout.as_secs(),
        daemon_log_path().display()
    ))
}

/// Start the daemon in the background
pub fn start_daemon(port: Option<u16>, extra_args: Option<DaemonArgs>) -> Result<i32> {
    if is_daemon_running() {
        let pid = get_daemon_pid().unwrap_or(-1);
        return Err(anyhow!("Daemon is already running (PID {})", pid));
    }

    // Ensure ghostwire home exists
    let home = ghostwire_home();
    fs::create_dir_all(&home)?;

    // Find the ghostwire binary
    let exe = std::env::current_exe()?;

    let daemon_port = port.unwrap_or(DEFAULT_DAEMON_PORT);
    if std::net::TcpListener::bind(("127.0.0.1", daemon_port)).is_err() {
        return Err(anyhow!(
            "API port {} is already in use. Stop the existing GhostWire daemon or choose another port with --port.",
            daemon_port
        ));
    }
    let extra = extra_args.unwrap_or_default();

    let _ = save_daemon_args(&extra, daemon_port);

    // Spawn the daemon process
    let log_path = daemon_log_path();
    let log_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;

    let stderr = log_file.try_clone()?;

    let mut cmd = Command::new(&exe);
    cmd.arg("--daemon")
        .arg("--port")
        .arg(daemon_port.to_string())
        .arg("--api-host")
        .arg(&extra.api_host)
        .arg("--mesh-port")
        .arg(extra.mesh_port.to_string())
        .arg("--quic-port")
        .arg(extra.quic_port.to_string());

    for ts in &extra.tailscale_bootstrap {
        cmd.arg("--tailscale-bootstrap").arg(ts);
    }

    for b in &extra.bootstrap {
        cmd.arg("--bootstrap").arg(b);
    }

    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(log_file)
        .stderr(stderr)
        .spawn()?;

    // Give it a moment to start up
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Check if it's still running
    match child.try_wait() {
        Ok(Some(status)) => {
            return Err(anyhow!(
                "Daemon exited immediately with status: {:?}\nCheck logs at: {}",
                status,
                log_path.display()
            ));
        }
        Ok(None) => {
            // Still running — good
        }
        Err(e) => {
            return Err(anyhow!("Failed to check daemon status: {}", e));
        }
    }

    let pid = child.id() as i32;

    // Write PID file
    fs::write(pid_file_path(), pid.to_string())?;

    // Detach from child (let it run independently)
    // Don't wait for it — it's a daemon
    std::mem::forget(child);

    Ok(pid)
}

/// Stop the running daemon
pub fn stop_daemon() -> Result<()> {
    let pid = get_daemon_pid().ok_or_else(|| anyhow!("No daemon is running"))?;

    tracing::info!("Stopping daemon (PID {})", pid);

    // Send SIGTERM
    #[cfg(unix)]
    {
        unsafe {
            let result = libc::kill(pid, libc::SIGTERM);
            if result != 0 {
                return Err(anyhow!(
                    "Failed to send SIGTERM to daemon: {}",
                    io::Error::last_os_error()
                ));
            }
        }
    }
    #[cfg(not(unix))]
    {
        return Err(anyhow!("Daemon stop not supported on this platform"));
    }

    // Wait for it to exit (up to 5 seconds)
    for _ in 0..50 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        if !is_daemon_running() {
            // Clean up PID file
            let _ = fs::remove_file(pid_file_path());
            tracing::info!("Daemon stopped successfully");
            return Ok(());
        }
    }

    // Force kill if still running
    tracing::warn!("Daemon didn't stop gracefully, sending SIGKILL");
    #[cfg(unix)]
    {
        unsafe {
            libc::kill(pid, libc::SIGKILL);
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(200));
    let _ = fs::remove_file(pid_file_path());

    Ok(())
}

/// Get daemon status information
pub fn daemon_status() -> String {
    if is_daemon_running() {
        let pid = get_daemon_pid().unwrap_or(-1);
        let port = fs::read_to_string(daemon_args_path())
            .ok()
            .and_then(|json| serde_json::from_str::<SavedDaemonArgs>(&json).ok())
            .map(|saved| saved.api_port)
            .unwrap_or(DEFAULT_DAEMON_PORT);
        format!(
            "🟢 Daemon is running\n   PID:  {}\n   Port: {}\n   API:  http://127.0.0.1:{}",
            pid, port, port
        )
    } else {
        "⚫ Daemon is not running\n   Run 'ghostwire start' to start it".to_string()
    }
}

/// Ensure the daemon is running — start it if not
pub fn ensure_daemon(port: Option<u16>) -> Result<i32> {
    if is_daemon_running() {
        Ok(get_daemon_pid().unwrap_or(-1))
    } else {
        start_daemon(port, None)
    }
}
