use anyhow::Result;
use serde::Deserialize;
use tracing::{info, warn};

const GITHUB_REPO: &str = "Phantomojo/GhostWire-secure-mesh-communication";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
    body: Option<String>,
}

fn detect_target() -> Option<&'static str> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    match (os, arch) {
        ("linux", "x86_64") => Some("x86_64-linux"),
        ("linux", "aarch64") => Some("aarch64-linux"),
        ("macos", "x86_64") => Some("x86_64-macos"),
        ("macos", "aarch64") => Some("aarch64-macos"),
        ("windows", "x86_64") => Some("x86_64-windows"),
        _ => None,
    }
}

fn asset_name(target: &str) -> String {
    if target.contains("windows") {
        format!("ghostwire-{}.zip", target)
    } else {
        format!("ghostwire-{}.tar.gz", target)
    }
}

fn version_tuple(v: &str) -> (u32, u32, u32) {
    let parts: Vec<u32> = v.split('.').filter_map(|s| s.parse().ok()).collect();
    (
        parts.first().copied().unwrap_or(0),
        parts.get(1).copied().unwrap_or(0),
        parts.get(2).copied().unwrap_or(0),
    )
}

fn is_newer(candidate: &str, current: &str) -> bool {
    version_tuple(candidate) > version_tuple(current)
}

fn strip_v(s: &str) -> &str {
    s.strip_prefix('v').unwrap_or(s)
}

pub fn is_update_available() -> bool {
    match check_for_update_sync() {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            warn!("Update check failed: {}", e);
            false
        }
    }
}

pub fn apply_update() -> Result<()> {
    let manifest = check_for_update_sync()
        .map_err(|e| anyhow::anyhow!("Update check failed: {}", e))?
        .ok_or_else(|| anyhow::anyhow!("No update available"))?;

    let target =
        detect_target().ok_or_else(|| anyhow::anyhow!("Unsupported platform"))?;
    let name = asset_name(target);
    let remote_version = strip_v(&manifest.tag_name);
    let asset = manifest
        .assets
        .iter()
        .find(|a| a.name == name)
        .ok_or_else(|| anyhow::anyhow!("Asset '{}' not found in release v{}", name, remote_version))?;

    info!("Downloading v{}...", remote_version);
    let bytes = reqwest::blocking::get(&asset.browser_download_url)
        .map_err(|e| anyhow::anyhow!("Download failed: {}", e))?
        .bytes()
        .map_err(|e| anyhow::anyhow!("Download read failed: {}", e))?;

    let current_exe = std::env::current_exe()?;
    let tmp_path = current_exe.with_extension("tmp");

    std::fs::write(&tmp_path, &bytes)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(0o755))?;
    }

    let backup = current_exe.with_extension("bak");
    if backup.exists() {
        let _ = std::fs::remove_file(&backup);
    }
    std::fs::rename(&current_exe, &backup)?;
    std::fs::rename(&tmp_path, &current_exe)?;

    let installed_version = strip_v(&manifest.tag_name).to_string();
    info!("Updated to v{}", installed_version);
    println!("✅ Updated to v{}", installed_version);
    println!("   Backup: {}", backup.display());
    Ok(())
}

fn check_for_update_sync() -> Result<Option<GitHubRelease>> {
    let target = detect_target().ok_or_else(|| anyhow::anyhow!("Unsupported platform"))?;
    let _name = asset_name(target);

    let url = format!(
        "https://api.github.com/repos/{}/releases/latest",
        GITHUB_REPO
    );

    let resp = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?
        .get(&url)
        .header("User-Agent", "ghostwire-updater")
        .send()?;

    if !resp.status().is_success() {
        anyhow::bail!("GitHub API returned {}", resp.status());
    }

    let release: GitHubRelease = resp.json()?;
    let remote_version = strip_v(&release.tag_name);

    if is_newer(remote_version, CURRENT_VERSION) {
        info!(
            "Update available: v{} → v{}",
            CURRENT_VERSION, remote_version
        );
        Ok(Some(release))
    } else {
        Ok(None)
    }
}

pub async fn check_for_update() -> Option<(String, String)> {
    match tokio::task::spawn_blocking(check_for_update_sync).await {
        Ok(Ok(Some(release))) => {
            let version = strip_v(&release.tag_name).to_string();
            let notes = release.body.unwrap_or_default();
            Some((version, notes))
        }
        _ => None,
    }
}

pub async fn apply_update_async() -> Result<()> {
    tokio::task::spawn_blocking(apply_update)
        .await
        .map_err(|e| anyhow::anyhow!("Update task failed: {}", e))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_comparison() {
        assert!(is_newer("0.2.0", "0.1.0"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(!is_newer("0.1.0", "0.1.0"));
        assert!(!is_newer("0.1.0", "0.2.0"));
    }

    #[test]
    fn strip_v_prefix() {
        assert_eq!(strip_v("v1.2.3"), "1.2.3");
        assert_eq!(strip_v("1.2.3"), "1.2.3");
    }
}
