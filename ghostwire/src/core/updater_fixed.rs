//! updater_fixed.rs — Secure updater replacing the RCE-vulnerable shell script caller
//!
//! PROBLEMS WITH THE CURRENT updater.rs:
//!   1. Downloads and executes update.sh --yes with NO signature verification
//!      → anyone who poisons your DNS can execute arbitrary code as you
//!   2. Uses `git fetch` which requires git to be installed and the repo to be present
//!      → doesn't work for binary distributions
//!   3. No rollback on failed update
//!
//! THIS REPLACEMENT:
//!   - Fetches update manifest from a known URL
//!   - Verifies Ed25519 signature on manifest before acting
//!   - Downloads binary to a temp location, verifies BLAKE3 hash
//!   - Atomic rename (no half-updated state)
//!   - Falls back gracefully if update infra is unreachable
//!
//! ALSO: Fixed store.rs clear() — see bottom of this file.

use anyhow::Result;
use serde::Deserialize;
use tracing::{info, warn};

/// The public key used to sign Ghostwire release manifests.
/// This is baked into the binary at compile time — no TOFU, no trust-on-update.
/// REPLACE WITH YOUR ACTUAL RELEASE SIGNING KEY.
const RELEASE_SIGNING_KEY: &[u8; 32] = &[
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // PLACEHOLDER
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Replace before release
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// Update manifest published at MANIFEST_URL
#[derive(Debug, Deserialize)]
pub struct UpdateManifest {
    pub version: String,
    pub download_url: String,
    /// BLAKE3 hash of the binary (hex)
    pub binary_hash: String,
    /// Ed25519 signature over (version + binary_hash + download_url), hex
    pub signature: String,
    pub release_notes: Option<String>,
    pub min_compatible_version: Option<String>,
}

const MANIFEST_URL: &str = "https://releases.ghostwire.app/latest.json";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check if an update is available.
/// Returns the manifest if there's a newer version, None if up-to-date.
pub async fn check_for_update() -> Option<UpdateManifest> {
    info!("Checking for updates (current: {})", CURRENT_VERSION);

    let manifest = match fetch_manifest().await {
        Ok(m) => m,
        Err(e) => {
            warn!("Update check failed: {}", e);
            return None;
        }
    };

    if !verify_manifest_signature(&manifest) {
        warn!("Update manifest signature INVALID — ignoring update");
        return None;
    }

    if is_newer(&manifest.version, CURRENT_VERSION) {
        info!(
            "Update available: {} → {}",
            CURRENT_VERSION, manifest.version
        );
        Some(manifest)
    } else {
        info!("Up to date ({})", CURRENT_VERSION);
        None
    }
}

/// Apply a verified update manifest.
/// Downloads binary to temp dir, verifies hash, then atomically replaces.
pub async fn apply_update(manifest: &UpdateManifest) -> Result<()> {
    info!("Downloading update {}...", manifest.version);

    // Download to temp file
    let temp_path = std::env::temp_dir().join("ghostwire-update.tmp");

    let response = reqwest::get(&manifest.download_url)
        .await
        .map_err(|e| anyhow::anyhow!("Download failed: {}", e))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| anyhow::anyhow!("Download read failed: {}", e))?;

    // Verify BLAKE3 hash before writing
    let actual_hash = hex::encode(blake3::hash(&bytes).as_bytes());
    if actual_hash != manifest.binary_hash {
        anyhow::bail!(
            "Binary hash mismatch — expected {}, got {}. Download may be corrupt or tampered.",
            manifest.binary_hash,
            actual_hash
        );
    }

    info!("Hash verified. Installing...");

    // Write to temp
    std::fs::write(&temp_path, &bytes).map_err(|e| anyhow::anyhow!("Write temp failed: {}", e))?;

    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&temp_path, std::fs::Permissions::from_mode(0o755))?;
    }

    // Atomic rename to current binary path
    let current_exe =
        std::env::current_exe().map_err(|e| anyhow::anyhow!("Cannot find current exe: {}", e))?;

    // Backup current binary
    let backup_path = current_exe.with_extension("bak");
    if current_exe.exists() {
        std::fs::rename(&current_exe, &backup_path)
            .map_err(|e| anyhow::anyhow!("Backup failed: {}", e))?;
    }

    // Install new binary
    match std::fs::rename(&temp_path, &current_exe) {
        Ok(_) => {
            info!("Update installed. Restart to apply.");
            // Remove backup
            let _ = std::fs::remove_file(&backup_path);
            Ok(())
        }
        Err(e) => {
            // Rollback
            if backup_path.exists() {
                let _ = std::fs::rename(&backup_path, &current_exe);
            }
            anyhow::bail!("Install failed (rolled back): {}", e)
        }
    }
}

fn verify_manifest_signature(manifest: &UpdateManifest) -> bool {
    let sig_bytes = match hex::decode(&manifest.signature) {
        Ok(b) => b,
        Err(_) => return false,
    };

    let payload = format!(
        "{}{}{}",
        manifest.version, manifest.binary_hash, manifest.download_url
    );

    // Verify Ed25519 signature
    use ring::signature::{UnparsedPublicKey, ED25519};
    let pub_key = UnparsedPublicKey::new(&ED25519, RELEASE_SIGNING_KEY);
    pub_key.verify(payload.as_bytes(), &sig_bytes).is_ok()
}

async fn fetch_manifest() -> Result<UpdateManifest> {
    let resp = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        reqwest::get(MANIFEST_URL),
    )
    .await
    .map_err(|_| anyhow::anyhow!("Manifest fetch timed out"))?
    .map_err(|e| anyhow::anyhow!("Manifest fetch failed: {}", e))?;

    let manifest: UpdateManifest = resp
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Manifest parse failed: {}", e))?;
    Ok(manifest)
}

fn is_newer(candidate: &str, current: &str) -> bool {
    // Simple semver comparison: parse "X.Y.Z"
    fn parse(s: &str) -> (u32, u32, u32) {
        let parts: Vec<u32> = s.split('.').filter_map(|p| p.parse().ok()).collect();
        (
            parts.first().copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
            parts.get(2).copied().unwrap_or(0),
        )
    }
    parse(candidate) > parse(current)
}

// ═══════════════════════════════════════════════════════════════════════════
// ALSO FIX: src/core/store.rs clear() method
// ═══════════════════════════════════════════════════════════════════════════
//
// The current clear() is a no-op:
//
//   pub fn clear(&self) -> Result<()> {
//       Ok(()) // ← Does nothing. Wipe command silently lies.
//   }
//
// Replace with:
//
//   pub fn clear(&self) -> Result<()> {
//       // Drop the "messages" sled tree entirely and recreate it empty
//       let tree_name = b"messages";
//       self.storage_stack.persistence.db()?.drop_tree(tree_name)?;
//       tracing::info!("Message store cleared");
//       Ok(())
//   }
//
// And add to PersistenceManager (if not already there):
//
//   pub fn db(&self) -> Result<&sled::Db> {
//       Ok(&self.db)
//   }
//
// Also fix profiles.rs by DELETING profiles_debug.rs entirely:
//   git rm ghostwire/src/controllers/profiles_debug.rs
//   Remove from controllers/mod.rs:
//     - pub mod profiles_debug;
//     - pub use profiles_debug::ProfilesDebugController; (if any)
// ═══════════════════════════════════════════════════════════════════════════

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
}
