use anyhow::{anyhow, bail, Context, Result};
use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use ring::rand::{SecureRandom, SystemRandom};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use zeroize::Zeroizing;

const MAGIC: &[u8; 4] = b"GWKS";
const VERSION: u8 = 1;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 24;
const KEY_LEN: usize = 32;
const TAG_LEN: usize = 16;
const SLOT_LEN: usize = MAGIC.len() + 1 + SALT_LEN + NONCE_LEN + KEY_LEN + TAG_LEN;
const MIN_PASSPHRASE_LEN: usize = 12;
const ARGON2_MEMORY_KIB: u32 = 65_536;
const ARGON2_TIME_COST: u32 = 3;
const ARGON2_PARALLELISM: u32 = 1;
const AAD_DOMAIN: &[u8] = b"GhostWire.PersistenceKeyslot";

/// A random persistence data-encryption key held in zeroizing memory.
#[derive(Debug)]
pub struct PersistenceKey(Zeroizing<[u8; KEY_LEN]>);

impl Clone for PersistenceKey {
    fn clone(&self) -> Self {
        Self(Zeroizing::new(*self.0))
    }
}

impl PersistenceKey {
    pub(crate) fn random() -> Result<Self> {
        let mut bytes = [0u8; KEY_LEN];
        SystemRandom::new()
            .fill(&mut bytes)
            .map_err(|_| anyhow!("secure random generation failed"))?;
        Ok(Self(Zeroizing::new(bytes)))
    }

    pub(crate) fn from_bytes(bytes: [u8; KEY_LEN]) -> Self {
        Self(Zeroizing::new(bytes))
    }

    pub(crate) fn as_bytes(&self) -> &[u8; KEY_LEN] {
        &self.0
    }
}

pub(crate) fn keyslot_path(base_dir: &Path, profile: Option<&str>) -> PathBuf {
    let suffix = profile.unwrap_or("default");
    base_dir.join(format!("ghost_{suffix}.persistence.keyslot"))
}

pub(crate) fn create_for_key(
    base_dir: &Path,
    profile: Option<&str>,
    passphrase: &str,
    key: &PersistenceKey,
) -> Result<()> {
    let path = keyslot_path(base_dir, profile);
    if path.exists() {
        bail!("persistence keyslot already exists");
    }
    write_slot(&path, base_dir, profile, passphrase, key, false)
}

pub(crate) fn replace_for_key(
    base_dir: &Path,
    profile: Option<&str>,
    passphrase: &str,
    key: &PersistenceKey,
) -> Result<()> {
    let path = keyslot_path(base_dir, profile);
    write_slot(&path, base_dir, profile, passphrase, key, true)
}

pub(crate) fn load(
    base_dir: &Path,
    profile: Option<&str>,
    passphrase: &str,
) -> Result<PersistenceKey> {
    validate_passphrase(passphrase)?;
    let path = keyslot_path(base_dir, profile);
    let encoded =
        fs::read(&path).with_context(|| format!("read persistence keyslot {}", path.display()))?;
    if encoded.len() != SLOT_LEN {
        bail!("invalid persistence keyslot length");
    }
    if &encoded[..MAGIC.len()] != MAGIC {
        bail!("invalid persistence keyslot magic");
    }
    if encoded[MAGIC.len()] != VERSION {
        bail!("unsupported persistence keyslot version");
    }

    let salt_start = MAGIC.len() + 1;
    let salt_end = salt_start + SALT_LEN;
    let nonce_end = salt_end + NONCE_LEN;
    let salt = &encoded[salt_start..salt_end];
    let nonce: &[u8; NONCE_LEN] = encoded[salt_end..nonce_end]
        .try_into()
        .map_err(|_| anyhow!("invalid persistence keyslot nonce"))?;
    let wrapped = &encoded[nonce_end..];
    let wrapping_key = derive_passphrase_key(passphrase, salt)?;
    let cipher = XChaCha20Poly1305::new(Key::from_slice(wrapping_key.as_ref()));
    let plaintext = cipher
        .decrypt(
            XNonce::from_slice(nonce),
            Payload {
                msg: wrapped,
                aad: &aad(profile),
            },
        )
        .map_err(|_| anyhow!("persistence keyslot authentication failed"))?;
    let bytes: [u8; KEY_LEN] = plaintext
        .as_slice()
        .try_into()
        .map_err(|_| anyhow!("invalid wrapped persistence key length"))?;
    Ok(PersistenceKey::from_bytes(bytes))
}

fn write_slot(
    path: &Path,
    base_dir: &Path,
    profile: Option<&str>,
    passphrase: &str,
    key: &PersistenceKey,
    replace_existing: bool,
) -> Result<()> {
    validate_passphrase(passphrase)?;
    fs::create_dir_all(base_dir)?;
    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    let rng = SystemRandom::new();
    rng.fill(&mut salt)
        .map_err(|_| anyhow!("secure salt generation failed"))?;
    rng.fill(&mut nonce)
        .map_err(|_| anyhow!("secure nonce generation failed"))?;

    let wrapping_key = derive_passphrase_key(passphrase, &salt)?;
    let cipher = XChaCha20Poly1305::new(Key::from_slice(wrapping_key.as_ref()));
    let wrapped = cipher
        .encrypt(
            XNonce::from_slice(&nonce),
            Payload {
                msg: key.as_bytes(),
                aad: &aad(profile),
            },
        )
        .map_err(|_| anyhow!("persistence keyslot encryption failed"))?;

    let mut encoded = Vec::with_capacity(SLOT_LEN);
    encoded.extend_from_slice(MAGIC);
    encoded.push(VERSION);
    encoded.extend_from_slice(&salt);
    encoded.extend_from_slice(&nonce);
    encoded.extend_from_slice(&wrapped);

    atomic_write(path, &encoded, replace_existing)
}

fn atomic_write(path: &Path, bytes: &[u8], replace_existing: bool) -> Result<()> {
    let tmp = path.with_extension("keyslot.tmp");
    let _ = fs::remove_file(&tmp);
    let result = (|| -> Result<()> {
        let mut options = OpenOptions::new();
        options.write(true).create_new(true).mode(0o600);
        let mut file = options.open(&tmp)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        if !replace_existing && path.exists() {
            bail!("persistence keyslot already exists");
        }
        fs::rename(&tmp, path)?;
        #[cfg(unix)]
        File::open(
            path.parent()
                .ok_or_else(|| anyhow!("keyslot has no parent directory"))?,
        )?
        .sync_all()?;
        Ok(())
    })();
    let _ = fs::remove_file(&tmp);
    result
}

fn validate_passphrase(passphrase: &str) -> Result<()> {
    if passphrase.as_bytes().len() < MIN_PASSPHRASE_LEN {
        bail!("persistence passphrase must be at least {MIN_PASSPHRASE_LEN} bytes");
    }
    if passphrase.as_bytes().len() > 4096 {
        bail!("persistence passphrase is too long");
    }
    Ok(())
}

fn derive_passphrase_key(passphrase: &str, salt: &[u8]) -> Result<Zeroizing<[u8; KEY_LEN]>> {
    let params = Params::new(
        ARGON2_MEMORY_KIB,
        ARGON2_TIME_COST,
        ARGON2_PARALLELISM,
        Some(KEY_LEN),
    )
    .map_err(|e| anyhow!("invalid Argon2 parameters: {e}"))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut output = Zeroizing::new([0u8; KEY_LEN]);
    argon2
        .hash_password_into(passphrase.as_bytes(), salt, output.as_mut())
        .map_err(|e| anyhow!("Argon2 derivation failed: {e}"))?;
    Ok(output)
}

fn aad(profile: Option<&str>) -> Vec<u8> {
    let profile = profile.unwrap_or("default").as_bytes();
    let mut result = Vec::with_capacity(AAD_DOMAIN.len() + 2 + profile.len());
    result.extend_from_slice(AAD_DOMAIN);
    result.push(VERSION);
    result.extend_from_slice(&(profile.len() as u16).to_be_bytes());
    result.extend_from_slice(profile);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn keyslot_roundtrip_and_wrong_passphrase_rejection() {
        let dir = tempdir().unwrap();
        let key = PersistenceKey::random().unwrap();
        create_for_key(
            dir.path(),
            Some("main"),
            "correct horse battery staple",
            &key,
        )
        .unwrap();
        let loaded = load(dir.path(), Some("main"), "correct horse battery staple").unwrap();
        assert_eq!(loaded.as_bytes(), key.as_bytes());
        assert!(load(dir.path(), Some("main"), "wrong passphrase").is_err());
    }

    #[test]
    fn keyslot_is_profile_bound_and_tamper_evident() {
        let dir = tempdir().unwrap();
        let key = PersistenceKey::random().unwrap();
        create_for_key(
            dir.path(),
            Some("main"),
            "correct horse battery staple",
            &key,
        )
        .unwrap();
        assert!(load(dir.path(), Some("other"), "correct horse battery staple").is_err());
        let path = keyslot_path(dir.path(), Some("main"));
        let mut bytes = fs::read(&path).unwrap();
        bytes[SLOT_LEN - 1] ^= 1;
        fs::write(&path, bytes).unwrap();
        assert!(load(dir.path(), Some("main"), "correct horse battery staple").is_err());
    }

    #[test]
    fn keyslot_rejects_weak_passphrases() {
        let dir = tempdir().unwrap();
        let key = PersistenceKey::random().unwrap();
        assert!(create_for_key(dir.path(), None, "too-short", &key).is_err());
    }
}
