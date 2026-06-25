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
use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Default daemon port
pub const DEFAULT_DAEMON_PORT: u16 = 9097;

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

/// Check if the daemon is currently running
pub fn is_daemon_running() -> bool {
    let pid_path = pid_file_path();
    if !pid_path.exists() {
        return false;
    }

    let pid_str = match fs::read_to_string(&pid_path) {
        Ok(s) => s.trim().to_string(),
        Err(_) => return false,
    };

    let pid: i32 = match pid_str.parse() {
        Ok(p) => p,
        Err(_) => return false,
    };

    // Check if process is actually running
    #[cfg(unix)]
    {
        unsafe {
            let result = libc::kill(pid, 0);
            result == 0
        }
    }
    #[cfg(not(unix))]
    {
        // Fallback: just check PID file exists
        true
    }
}

/// Get the running daemon's PID, if any
pub fn get_daemon_pid() -> Option<i32> {
    if !is_daemon_running() {
        return None;
    }
    let pid_path = pid_file_path();
    let pid_str = fs::read_to_string(&pid_path).ok()?;
    pid_str.trim().parse().ok()
}

/// Extra args to forward to the daemon child process
pub struct DaemonArgs {
    pub tailscale_bootstrap: Vec<String>,
    pub bootstrap: Vec<String>,
    pub mesh_port: u16,
}

impl Default for DaemonArgs {
    fn default() -> Self {
        Self {
            tailscale_bootstrap: Vec::new(),
            bootstrap: Vec::new(),
            mesh_port: 0,
        }
    }
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
    let extra = extra_args.unwrap_or_default();

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
        .arg("--mesh-port")
        .arg(extra.mesh_port.to_string());

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
    fs::write(&pid_file_path(), pid.to_string())?;

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
        let port = DEFAULT_DAEMON_PORT;
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
