use anyhow::Result;
use std::process::Command;
use tracing::{error, info, warn};

/// Checks if there are any updates available in the git repository.
/// Returns true if the local main branch is behind origin/main.
pub fn is_update_available() -> bool {
    info!("🔍 Checking for repository updates...");

    // Fetch the latest information from origin
    let fetch_status = Command::new("git").arg("fetch").arg("origin").output();

    match fetch_status {
        Ok(output) if output.status.success() => {
            // Check if local branch is behind remote
            let diff_status = Command::new("git")
                .arg("rev-list")
                .arg("HEAD..origin/main")
                .arg("--count")
                .output();

            match diff_status {
                Ok(diff_output) if diff_output.status.success() => {
                    let count_str = String::from_utf8_lossy(&diff_output.stdout)
                        .trim()
                        .to_string();
                    let count: u32 = count_str.parse().unwrap_or(0);

                    if count > 0 {
                        info!("✨ {} new update(s) found on origin/main", count);
                        return true;
                    } else {
                        info!("✅ Repository is up to date");
                        return false;
                    }
                }
                _ => {
                    warn!("⚠️ Failed to check git diff, assuming no update.");
                    false
                }
            }
        }
        _ => {
            warn!("⚠️ Failed to fetch from git origin. Check internet connection.");
            false
        }
    }
}

/// Triggers a non-interactive update using update.sh and exits.
pub fn apply_update() -> Result<()> {
    info!("🚀 Applying GhostWire update...");

    // We assume the update.sh is in the root of the repository
    // Since the binary is usually run from the repository root (or via ghostwire-manager.sh)
    let status = Command::new("bash")
        .arg("./update.sh")
        .arg("--yes")
        .status();

    match status {
        Ok(s) if s.success() => {
            info!("✅ Update successful! Please restart GhostWire.");
            // In a real scenario, we might want to use execve to replace the current process
            // but for a simple bash script that rebuilds, exiting is safer.
            std::process::exit(0);
        }
        _ => {
            error!("❌ Update failed. Check logs for details.");
            anyhow::bail!("Update script failed")
        }
    }
}
