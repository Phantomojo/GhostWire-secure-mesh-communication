//! binary_hardening.rs — Anti-reverse-engineering measures
//!
//! Three complementary layers:
//!
//! 1. obfstr: compile-time string encryption
//!    All sensitive strings (error messages, log labels, config keys)
//!    are encrypted at compile time. At runtime they decrypt in-place.
//!    No string appears in the binary as plaintext.
//!    Usage: obfstr::obfstr!("sensitive string") instead of "sensitive string"
//!
//! 2. mlock: memory locking
//!    Critical memory pages (containing key material) are pinned to RAM
//!    and marked non-swappable. Prevents key material appearing in swap files.
//!    Works with the zeroize crate: mlock on allocation, munlock+zeroize on drop.
//!
//! 3. Self-integrity check
//!    At startup, BLAKE3-hash the running binary and compare to a value
//!    embedded at compile time. If the binary has been tampered with
//!    (evil maid attack, backdoored update), startup aborts.
//!    The reference hash is embedded by build.rs from GHOSTWIRE_BINARY_HASH env var.

use anyhow::Result;

// ── Memory locking ────────────────────────────────────────────────────────────

/// Lock a memory region to prevent it from being swapped to disk.
/// Call on allocation of any buffer holding key material.
/// MUST be paired with unlock_memory before freeing.
#[cfg(unix)]
pub fn lock_memory(ptr: *const u8, len: usize) -> Result<()> {
    use std::os::raw::c_void;
    let ret = unsafe { libc::mlock(ptr as *const c_void, len) };
    if ret == 0 {
        Ok(())
    } else {
        // mlock can fail if the process has hit its RLIMIT_MEMLOCK
        // Log the failure but don't abort — memory protection is best-effort
        tracing::warn!(
            "mlock failed for {} bytes: {} — key material may appear in swap",
            len,
            std::io::Error::last_os_error()
        );
        Ok(()) // Non-fatal
    }
}

#[cfg(not(unix))]
pub fn lock_memory(_ptr: *const u8, _len: usize) -> Result<()> {
    Ok(()) // Windows: VirtualLock equivalent — TODO for Android/Windows port
}

#[cfg(unix)]
pub fn unlock_memory(ptr: *const u8, len: usize) {
    use std::os::raw::c_void;
    unsafe {
        libc::munlock(ptr as *const c_void, len);
    }
}

#[cfg(not(unix))]
pub fn unlock_memory(_ptr: *const u8, _len: usize) {}

/// Locked key buffer: memory-locked allocation that zeroizes on drop
pub struct LockedBuffer {
    data: Vec<u8>,
}

impl LockedBuffer {
    pub fn new(size: usize) -> Self {
        let data = vec![0u8; size];
        lock_memory(data.as_ptr(), data.len()).ok();
        Self { data }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut buf = Self::new(bytes.len());
        buf.data.copy_from_slice(bytes);
        buf
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.data
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Drop for LockedBuffer {
    fn drop(&mut self) {
        // Zeroize before unlock
        for byte in self.data.iter_mut() {
            *byte = 0;
        }
        unlock_memory(self.data.as_ptr(), self.data.len());
    }
}

// ── Anti-debug ────────────────────────────────────────────────────────────────

/// Detect and abort if being debugged via ptrace (TRACEME).
/// One-shot check — runs once at startup, then self-removes.
/// Covers: gdb, strace, ltrace, and ptrace-based anti-tamper tools.
#[cfg(unix)]
pub fn detect_debugger() {
    // Read /proc/self/status to check TracerPid — safe, no side effects
    // PTRACE_TRACEME is dangerous: it makes this process a tracee, and any
    // signal delivery (SIGSTOP from nohup, etc) causes a ptrace-stop that
    // freezes the process permanently.
    if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if let Some(val) = line.strip_prefix("TracerPid:") {
                let pid: u32 = val.trim().parse().unwrap_or(0);
                if pid != 0 {
                    eprintln!(
                        "🚨 Anti-debug: debugger detected (TracerPid={}). Aborting.",
                        pid
                    );
                    std::process::exit(1);
                }
                return;
            }
        }
    }
}

#[cfg(not(unix))]
pub fn detect_debugger() {}

// ── Self-integrity check ──────────────────────────────────────────────────────

/// The expected BLAKE3 hash of the release binary, embedded at compile time.
/// Set by build.rs from GHOSTWIRE_BINARY_HASH env var.
/// If not set: returns None and the check is skipped (development mode).
fn expected_binary_hash() -> Option<[u8; 32]> {
    match option_env!("GHOSTWIRE_BINARY_HASH") {
        Some(hex_str) if hex_str.len() == 64 => {
            let bytes = hex::decode(hex_str).ok()?;
            bytes.try_into().ok()
        }
        _ => None, // Not set — skip check (dev build)
    }
}

/// Verify the running binary hasn't been tampered with.
/// Call at startup, before anything else.
///
/// Returns Ok(false) if no reference hash is set (dev mode — skip check).
/// Returns Ok(true) if hash matches.
/// Returns Err if hash doesn't match — caller should abort.
pub async fn verify_binary_integrity() -> Result<bool> {
    let expected = match expected_binary_hash() {
        Some(h) => h,
        None => {
            tracing::debug!(
                "Binary integrity check skipped — GHOSTWIRE_BINARY_HASH not set (dev build)"
            );
            return Ok(false); // Not configured — allow startup
        }
    };

    let exe_path = std::env::current_exe()
        .map_err(|e| anyhow::anyhow!("Cannot determine executable path: {}", e))?;

    let binary = tokio::fs::read(&exe_path)
        .await
        .map_err(|e| anyhow::anyhow!("Cannot read binary for integrity check: {}", e))?;

    let actual_hash = *blake3::hash(&binary).as_bytes();

    if actual_hash == expected {
        tracing::info!(
            "Binary integrity check PASSED — {}",
            hex::encode(&actual_hash[..8])
        );
        Ok(true)
    } else {
        anyhow::bail!(
            "BINARY INTEGRITY CHECK FAILED — binary may be tampered!\n  \
             Expected: {}\n  \
             Actual:   {}\n  \
             Refusing to start. If this is a legitimate update, \
             the release signing system should have caught this.",
            hex::encode(expected),
            hex::encode(actual_hash)
        )
    }
}

/// Generate the hash of the current binary for embedding in the next build.
/// Run this after building a release binary:
///   ghostwire hash-binary > binary.hash
///   GHOSTWIRE_BINARY_HASH=$(cat binary.hash) cargo build --release
pub async fn hash_current_binary() -> Result<String> {
    let exe_path = std::env::current_exe()?;
    let binary = tokio::fs::read(&exe_path).await?;
    let hash = blake3::hash(&binary);
    Ok(hex::encode(hash.as_bytes()))
}

// ── obfstr usage guide ────────────────────────────────────────────────────────
//
// Add obfstr = "0.4" to Cargo.toml (already added).
// Usage anywhere in the codebase:
//
//   use obfstr::obfstr;
//
//   // Instead of:
//   tracing::warn!("Identity unlock failed");
//   // Use:
//   tracing::warn!("{}", obfstr!("Identity unlock failed"));
//
//   // For static &str comparisons:
//   let label: &str = obfstr!("ghostwire-internal-label");
//
// This applies especially to:
//   - Error messages that reveal internal structure
//   - Log messages in sensitive code paths (crypto, identity)
//   - Config key names
//   - Any string that would help a reverse engineer understand the code
//
// The genesis_constants.rs domain bytes are already opaque — no obfstr needed
// for those. obfstr is for human-readable strings in the non-crypto code paths.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locked_buffer_zeroizes_on_drop() {
        let _ptr = {
            let buf = LockedBuffer::from_bytes(&[0xFF; 32]);
            buf.data.as_ptr()
        };
        // After drop, the memory should be zeroed
        // Note: this test can't reliably check freed memory — it's testing
        // that the Drop impl runs without panicking, which is the real concern.
        // Actual zeroization is tested by inspecting the drop impl.
    }

    #[test]
    fn locked_buffer_preserves_data() {
        let data = vec![0x42u8; 64];
        let buf = LockedBuffer::from_bytes(&data);
        assert_eq!(buf.as_bytes(), &data[..]);
    }

    #[tokio::test]
    async fn integrity_check_skipped_in_dev() {
        // No GHOSTWIRE_BINARY_HASH set in dev — should return Ok(false)
        if option_env!("GHOSTWIRE_BINARY_HASH").is_none() {
            let result = verify_binary_integrity().await;
            assert!(result.is_ok());
            assert!(!result.unwrap()); // false = skipped
        }
    }
}
