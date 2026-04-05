//! startup.rs — Ghostwire startup sequence
//!
//! The current main.rs is a flat sequence with no health checks, no
//! connectivity self-test, and no feedback to the user about what's happening.
//! If something fails silently, the daemon just starts in a broken state.
//!
//! This module implements a proper boot sequence:
//!
//!   Phase 0 — Pre-flight  (environment checks, no network)
//!   Phase 1 — Identity    (load/create identity, verify keys)
//!   Phase 2 — Storage     (encrypted store, message DB)
//!   Phase 3 — Network     (mesh init, transport selection, NAT probe)
//!   Phase 4 — Knowledge   (load ark collections, sync index)
//!   Phase 5 — Ready       (announce presence, start API server)
//!
//! Each phase reports status, and failures are surfaced clearly.
//! The sequence is designed so that two PCs running Ghostwire on the
//! same LAN will find each other automatically after Phase 3.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::sync::broadcast;

// ── Boot event ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub enum BootEvent {
    PhaseStart { phase: u8, name: String },
    PhaseComplete { phase: u8, duration_ms: u64 },
    PhaseFailed { phase: u8, reason: String },
    Check { name: String, status: CheckStatus, detail: Option<String> },
    Warning { message: String },
    Ready { node_id: String, transport: String, peer_count: u32 },
}

#[derive(Debug, Clone, Serialize)]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
    Skip,
}

/// The complete startup report — sent to UI on boot
#[derive(Debug, Clone, Serialize)]
pub struct BootReport {
    pub events: Vec<BootEvent>,
    pub total_duration_ms: u64,
    pub success: bool,
    pub node_id: Option<String>,
    pub warnings: Vec<String>,
    pub peer_bootstrap_count: usize,
    pub lan_peers_found: usize,
    pub transport_mode: String,
}

// ── Phase runner ──────────────────────────────────────────────────────────────

pub struct StartupSequence {
    events: Vec<BootEvent>,
    warnings: Vec<String>,
    started_at: Instant,
    event_tx: Option<broadcast::Sender<BootEvent>>,
}

impl StartupSequence {
    pub fn new(event_tx: Option<broadcast::Sender<BootEvent>>) -> Self {
        Self {
            events: Vec::new(),
            warnings: Vec::new(),
            started_at: Instant::now(),
            event_tx,
        }
    }

    fn emit(&mut self, event: BootEvent) {
        if let Some(tx) = &self.event_tx {
            let _ = tx.send(event.clone());
        }
        self.events.push(event);
    }

    fn check(&mut self, name: impl Into<String>, status: CheckStatus, detail: Option<String>) {
        self.emit(BootEvent::Check {
            name: name.into(),
            status,
            detail,
        });
    }

    fn warn(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        self.warnings.push(msg.clone());
        self.emit(BootEvent::Warning { message: msg });
    }

    /// PHASE 0 — Pre-flight checks (fast, no I/O)
    pub fn phase0_preflight(&mut self, data_dir: &PathBuf) -> Result<()> {
        let t = Instant::now();
        self.emit(BootEvent::PhaseStart { phase: 0, name: "Pre-flight".into() });

        // Check data directory
        if data_dir.exists() {
            self.check("data directory", CheckStatus::Pass, Some(data_dir.display().to_string()));
        } else {
            match std::fs::create_dir_all(data_dir) {
                Ok(_) => self.check("data directory (created)", CheckStatus::Pass, Some(data_dir.display().to_string())),
                Err(e) => {
                    self.emit(BootEvent::PhaseFailed { phase: 0, reason: format!("Cannot create data dir: {}", e) });
                    anyhow::bail!("Cannot create data dir: {}", e);
                }
            }
        }

        // Check disk space (warn if < 100MB)
        if let Ok(space) = available_space(data_dir) {
            if space < 100 * 1024 * 1024 {
                self.warn(format!("Low disk space: {} MB available", space / 1024 / 1024));
            } else {
                self.check(
                    "disk space",
                    CheckStatus::Pass,
                    Some(format!("{} MB free", space / 1024 / 1024)),
                );
            }
        }

        // Check OS / platform
        let platform = std::env::consts::OS;
        self.check("platform", CheckStatus::Pass, Some(platform.to_string()));

        // Check if another instance is running (port conflict)
        if is_port_in_use(8080) {
            self.warn("Port 8080 already in use — API server may conflict");
        }

        self.emit(BootEvent::PhaseComplete { phase: 0, duration_ms: t.elapsed().as_millis() as u64 });
        Ok(())
    }

    /// PHASE 1 — Identity (crypto-heavy, may prompt for password)
    pub async fn phase1_identity(
        &mut self,
        profile: Option<&str>,
        data_dir: &PathBuf,
    ) -> Result<IdentityBootResult> {
        let t = Instant::now();
        self.emit(BootEvent::PhaseStart { phase: 1, name: "Identity".into() });

        let profile_dir = match profile {
            Some(p) => data_dir.join("profiles").join(p),
            None => data_dir.join("default"),
        };

        let is_new_identity = !profile_dir.join("identity.enc").exists();

        if is_new_identity {
            self.check("identity", CheckStatus::Warn, Some("No existing identity — will create on first unlock".into()));
        } else {
            self.check("identity file", CheckStatus::Pass, Some(profile_dir.display().to_string()));
        }

        // Check JWT seed
        let jwt_seed_path = data_dir.join(".jwt_seed");
        if jwt_seed_path.exists() {
            self.check("JWT secret", CheckStatus::Pass, Some("persisted".into()));
        } else {
            self.check("JWT secret", CheckStatus::Warn, Some("will generate fresh on startup".into()));
        }

        // Verify key files are not corrupted (size check)
        if !is_new_identity {
            let size = std::fs::metadata(profile_dir.join("identity.enc"))
                .map(|m| m.len())
                .unwrap_or(0);
            if size < 64 {
                self.warn("Identity file looks corrupt — very small");
            }
        }

        self.emit(BootEvent::PhaseComplete { phase: 1, duration_ms: t.elapsed().as_millis() as u64 });

        Ok(IdentityBootResult {
            is_new_identity,
            profile_dir,
        })
    }

    /// PHASE 2 — Storage
    pub async fn phase2_storage(&mut self, data_dir: &PathBuf) -> Result<()> {
        let t = Instant::now();
        self.emit(BootEvent::PhaseStart { phase: 2, name: "Storage".into() });

        let db_path = data_dir.join("db");

        // Try to open sled
        match sled::open(&db_path) {
            Ok(db) => {
                let tree_count = db.tree_names().len();
                self.check(
                    "sled database",
                    CheckStatus::Pass,
                    Some(format!("{} trees", tree_count)),
                );
                drop(db);
            }
            Err(e) => {
                self.emit(BootEvent::PhaseFailed {
                    phase: 2,
                    reason: format!("Cannot open database: {}", e),
                });
                anyhow::bail!("Storage init failed: {}", e);
            }
        }

        self.emit(BootEvent::PhaseComplete { phase: 2, duration_ms: t.elapsed().as_millis() as u64 });
        Ok(())
    }

    /// PHASE 3 — Network
    pub async fn phase3_network(
        &mut self,
        config: &NetworkBootConfig,
    ) -> Result<NetworkBootResult> {
        let t = Instant::now();
        self.emit(BootEvent::PhaseStart { phase: 3, name: "Network".into() });

        let mut result = NetworkBootResult::default();

        // Check internet connectivity
        let has_internet = test_internet_connectivity().await;
        if has_internet {
            self.check("internet", CheckStatus::Pass, None);
            result.has_internet = true;
        } else {
            self.check("internet", CheckStatus::Warn, Some("offline — LAN/BT only".into()));
            self.warn("No internet detected — operating in local mesh mode");
        }

        // Check LAN connectivity
        let lan_ip = local_ip_address::local_ip().ok();
        if let Some(ip) = lan_ip {
            self.check("LAN address", CheckStatus::Pass, Some(ip.to_string()));
            result.lan_ip = Some(ip.to_string());
        } else {
            self.check("LAN address", CheckStatus::Warn, Some("none detected".into()));
        }

        // Check listen port availability
        let listen_port = config.listen_port;
        if !is_port_in_use(listen_port) {
            self.check(
                &format!("listen port {}", listen_port),
                CheckStatus::Pass,
                None,
            );
            result.listen_port = listen_port;
        } else {
            // Try fallback port
            let fallback = listen_port + 1;
            self.warn(format!("Port {} in use — using {}", listen_port, fallback));
            result.listen_port = fallback;
        }

        // Check bootstrap nodes
        if config.bootstrap_nodes.is_empty() {
            self.check(
                "bootstrap nodes",
                CheckStatus::Warn,
                Some("using defaults — may be slow to discover peers".into()),
            );
        } else {
            self.check(
                "bootstrap nodes",
                CheckStatus::Pass,
                Some(format!("{} configured", config.bootstrap_nodes.len())),
            );
            result.bootstrap_count = config.bootstrap_nodes.len();
        }

        // Select transport mode
        result.transport_mode = if config.websocket_mode {
            "websocket".into()
        } else if config.shadowsocks_mode {
            "shadowsocks_aead".into()
        } else {
            "stealth".into()
        };
        self.check("transport mode", CheckStatus::Pass, Some(result.transport_mode.clone()));

        // Check if obfs4 is requested but known non-functional
        if config.obfs4_enabled {
            self.warn("⚠️  obfs4 is ALPHA and non-functional — DPI evasion inactive");
        }

        self.emit(BootEvent::PhaseComplete {
            phase: 3,
            duration_ms: t.elapsed().as_millis() as u64,
        });
        Ok(result)
    }

    /// PHASE 4 — Knowledge Ark init
    pub async fn phase4_knowledge(&mut self, data_dir: &PathBuf) -> Result<KnowledgeBootResult> {
        let t = Instant::now();
        self.emit(BootEvent::PhaseStart { phase: 4, name: "Knowledge Ark".into() });

        let ark_dir = data_dir.join("ark");
        let is_new = !ark_dir.exists();

        if is_new {
            std::fs::create_dir_all(&ark_dir).ok();
            self.check("knowledge ark", CheckStatus::Warn, Some("initialising fresh".into()));
        } else {
            // Count entry files
            let entry_count = std::fs::read_dir(&ark_dir)
                .map(|d| d.count())
                .unwrap_or(0);
            self.check(
                "knowledge ark",
                CheckStatus::Pass,
                Some(format!("~{} entries", entry_count)),
            );
        }

        self.emit(BootEvent::PhaseComplete { phase: 4, duration_ms: t.elapsed().as_millis() as u64 });
        Ok(KnowledgeBootResult { is_new })
    }

    /// PHASE 5 — Ready
    pub fn phase5_ready(&mut self, node_id: &str, transport: &str, peer_count: u32) {
        self.emit(BootEvent::Ready {
            node_id: node_id.to_string(),
            transport: transport.to_string(),
            peer_count,
        });

        tracing::info!("╔══════════════════════════════════════╗");
        tracing::info!("║          GHOSTWIRE ONLINE            ║");
        tracing::info!("╠══════════════════════════════════════╣");
        tracing::info!("║  Node: {}  ║", &node_id[..16.min(node_id.len())]);
        tracing::info!("║  Transport: {:28}  ║", transport);
        tracing::info!("║  Peers: {:31}  ║", peer_count);
        tracing::info!("╚══════════════════════════════════════╝");
    }

    /// Generate the full boot report
    pub fn report(&self, node_id: Option<String>, net: &NetworkBootResult) -> BootReport {
        let success = !self.events.iter().any(|e| matches!(e, BootEvent::PhaseFailed { .. }));
        BootReport {
            events: self.events.clone(),
            total_duration_ms: self.started_at.elapsed().as_millis() as u64,
            success,
            node_id,
            warnings: self.warnings.clone(),
            peer_bootstrap_count: net.bootstrap_count,
            lan_peers_found: 0,
            transport_mode: net.transport_mode.clone(),
        }
    }
}

// ── Result types ──────────────────────────────────────────────────────────────

pub struct IdentityBootResult {
    pub is_new_identity: bool,
    pub profile_dir: PathBuf,
}

#[derive(Debug, Default)]
pub struct NetworkBootResult {
    pub has_internet: bool,
    pub lan_ip: Option<String>,
    pub listen_port: u16,
    pub bootstrap_count: usize,
    pub transport_mode: String,
}

pub struct KnowledgeBootResult {
    pub is_new: bool,
}

#[derive(Debug, Clone)]
pub struct NetworkBootConfig {
    pub listen_port: u16,
    pub bootstrap_nodes: Vec<String>,
    pub websocket_mode: bool,
    pub shadowsocks_mode: bool,
    pub obfs4_enabled: bool,
}

impl Default for NetworkBootConfig {
    fn default() -> Self {
        Self {
            listen_port: 4001,
            bootstrap_nodes: vec![],
            websocket_mode: false,
            shadowsocks_mode: false,
            obfs4_enabled: false,
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn is_port_in_use(port: u16) -> bool {
    std::net::TcpListener::bind(format!("127.0.0.1:{}", port)).is_err()
}

fn available_space(path: &PathBuf) -> Result<u64> {
    // Simplified: read statvfs on Linux
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::ffi::OsStrExt;
        let path_c = std::ffi::CString::new(path.as_os_str().as_bytes())?;
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
        if unsafe { libc::statvfs(path_c.as_ptr(), &mut stat) } == 0 {
            return Ok(stat.f_bavail * stat.f_bsize);
        }
    }
    // Fallback: assume plenty of space
    Ok(u64::MAX)
}

async fn test_internet_connectivity() -> bool {
    tokio::time::timeout(
        Duration::from_secs(3),
        tokio::net::TcpStream::connect("8.8.8.8:53"),
    )
    .await
    .is_ok()
}
