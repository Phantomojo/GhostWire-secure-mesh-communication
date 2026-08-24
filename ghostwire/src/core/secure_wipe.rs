use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
pub struct WipeReport {
    pub target: String,
    pub stages: Vec<String>,
    pub verified: bool,
    pub residuals: Vec<String>,
    pub physical_media_erasure: bool,
}

fn validate_target(path: &Path) -> Result<()> {
    if !path.is_absolute() {
        bail!("wipe target must be absolute");
    }
    let normalized = path.to_path_buf();
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home"));
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    if normalized == Path::new("/") || normalized == home || normalized == cwd {
        bail!("refusing to wipe a filesystem root, home directory, or current directory");
    }
    let configured = std::env::var_os("GHOSTWIRE_HOME").map(PathBuf::from);
    let is_configured = configured.as_deref() == Some(normalized.as_path());
    let is_default = normalized.file_name().and_then(|n| n.to_str()) == Some(".ghostwire");
    if !is_configured && !is_default {
        bail!("wipe target is not an approved GhostWire data directory");
    }
    Ok(())
}

pub fn wipe_data_dir(path: &Path) -> Result<WipeReport> {
    validate_target(path)?;
    let mut stages = vec!["target_validated".to_string()];
    let mut residuals = Vec::new();

    if !path.exists() {
        stages.push("target_already_absent".to_string());
        return Ok(WipeReport {
            target: path.display().to_string(),
            stages,
            verified: true,
            residuals,
            physical_media_erasure: false,
        });
    }

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let tombstone =
        path.with_file_name(format!(".ghostwire-wipe-{}-{}", std::process::id(), stamp));
    fs::rename(path, &tombstone)
        .with_context(|| format!("failed to stage {} for wipe", path.display()))?;
    stages.push("target_atomically_detached".to_string());

    if let Err(error) = fs::remove_dir_all(&tombstone) {
        let _ = fs::rename(&tombstone, path);
        return Err(error).with_context(|| "failed to remove staged GhostWire data");
    }
    stages.push("staged_tree_removed".to_string());

    if path.exists() {
        residuals.push(path.display().to_string());
    }
    if tombstone.exists() {
        residuals.push(tombstone.display().to_string());
    }
    let verified = residuals.is_empty();
    if verified {
        stages.push("absence_verified".to_string());
    }

    Ok(WipeReport {
        target: path.display().to_string(),
        stages,
        verified,
        residuals,
        // Filesystem unlinking cannot honestly guarantee physical SSD erasure.
        physical_media_erasure: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wipe_stages_removes_and_verifies_ghostwire_directory() {
        let path = std::env::temp_dir()
            .join(format!("ghostwire-wipe-test-{}", std::process::id()))
            .join(".ghostwire");
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(path.join("vault")).unwrap();
        fs::write(path.join("identity.json"), b"secret").unwrap();
        let report = wipe_data_dir(&path).unwrap();
        assert!(report.verified);
        assert!(!path.exists());
        assert!(!report.physical_media_erasure);
        assert!(report
            .stages
            .contains(&"target_atomically_detached".to_string()));
        assert!(report.stages.contains(&"absence_verified".to_string()));
    }

    #[test]
    fn wipe_rejects_unapproved_target() {
        let path = std::env::temp_dir().join("not-ghostwire");
        assert!(wipe_data_dir(&path).is_err());
    }
}
