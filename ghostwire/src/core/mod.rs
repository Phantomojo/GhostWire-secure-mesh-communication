// Core networking, encryption, metadata sanitizer, and storage modules will be implemented here.

use serde::Serialize;
use zeroize::Zeroize;

pub mod armor;
pub mod binary_hardening;
pub mod blockstore;
pub mod bootstrap;
pub mod channels;
pub mod compression;
pub mod constants;
pub mod crl;
pub mod crypto_profile;
pub mod db;
pub mod dht_signatures;
pub mod disaster;
pub mod discovery_directory;
pub mod double_ratchet;
pub mod dtn;
pub mod encrypted_store;
pub mod encryption;
pub mod identity;
pub mod identity_pairing;
pub mod knowledge;
pub mod mailbox;
pub mod medium_adapter;
pub mod mesh_health;
pub mod message;
pub mod micro_optimizations;
pub mod monitor;
pub mod network;
pub mod obfs4_transport;
pub mod oem_battery_guide;
pub mod passphrase_keyslot;
pub mod peer_health;
pub mod persistence_manager;
pub mod recovery_shamir;
pub mod reliability;
pub mod reputation;
pub mod routing;
pub mod secure_wipe;
pub mod security_v2;
pub mod social;
pub mod traffic_armor;
pub mod transport_ext;
pub mod transport_pluggable;
pub mod typed_crypto;
pub mod updater_fixed;
pub mod vector_clock;
pub mod wire_protocol;
pub mod world_discovery;
pub use security_v2 as security;
pub mod store;
pub mod transport;
pub mod transport_manager;
pub mod transport_selector;
pub mod tutorial;
pub mod username_discovery; // DHT-based username discovery // Background and startup updates

// AI/ML Module
pub mod ai;

#[cfg(feature = "bluetooth")]
pub mod bluetooth;
#[cfg(feature = "briar")]
pub mod briar;
#[cfg(feature = "mesh")]
pub mod mesh;
pub mod qr;
pub mod quic_transport;
#[cfg(feature = "reticulum")]
pub mod reticulum;
pub mod sim_transport;
pub mod stealth_tcp;

#[cfg(feature = "matrix-bridge")]
use matrix_sdk::ruma::{events::room::message::RoomMessageEventContent, RoomId};
#[cfg(feature = "matrix-bridge")]
use matrix_sdk::Client;

pub use encryption::Encryption;
pub use identity::Identity;
pub use message::Message;
// Transport trait removed, using TransportKind enum instead
pub use security::{
    AuditLogger, FailedAttemptTracker, SecurityAuditRecord, SecurityConfig, SecurityManager,
    SecurityStats,
};

#[cfg(feature = "mesh")]
pub use mesh::{MeshManager, MeshNode, MeshStats, MeshTopology};

#[cfg(feature = "reticulum")]
pub use reticulum::{ReticulumManager, ReticulumStats, ReticulumTopology};
#[cfg(feature = "stealth")]
pub use stealth_tcp::{ConnectionStats, StealthTCPProvider};

use anyhow::Result;
use libp2p::Multiaddr;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::OnceLock;
// use std::time::Duration; // redundant
use tokio::sync::RwLock;
// using explicit paths for tracing macros
use tokio::sync::broadcast;
use transport::TransportRegistry;

/// System-wide events for real-time reactivity
#[derive(Debug, Clone, Serialize)]
pub enum CoreEvent {
    NewMessage(Message),
    PeerConnected(String),
    PeerDisconnected(String),
    SystemAlert(String),
    NetworkUpdate { active_peers: u32, latency_ms: u32 },
    SecurityLockdown(bool),
}

// Protocol Adapter Enum for async dispatch
#[derive(Clone)]
pub enum AdapterKind {
    Matrix(MatrixAdapter),
    Meshtastic(MeshtasticAdapter),
    // Add more adapters here as needed
}

impl AdapterKind {
    pub fn name(&self) -> &'static str {
        match self {
            AdapterKind::Matrix(a) => a.name(),
            AdapterKind::Meshtastic(a) => a.name(),
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            AdapterKind::Matrix(a) => a.description(),
            AdapterKind::Meshtastic(a) => a.description(),
        }
    }
    pub async fn send_message(
        &self,
        message: &crate::core::message::Message,
    ) -> anyhow::Result<()> {
        match self {
            AdapterKind::Matrix(a) => a.send_message(message).await,
            AdapterKind::Meshtastic(a) => a.send_message(message).await,
        }
    }
    pub async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>> {
        match self {
            AdapterKind::Matrix(a) => a.receive_message().await,
            AdapterKind::Meshtastic(a) => a.receive_message().await,
        }
    }
}

// Adapter Registry
pub struct AdapterRegistry {
    adapters: HashMap<String, AdapterKind>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }
    pub fn register(&mut self, adapter: AdapterKind) {
        self.adapters.insert(adapter.name().to_string(), adapter);
    }
    pub fn get(&self, name: &str) -> Option<&AdapterKind> {
        self.adapters.get(name)
    }
    pub fn list(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}

// Initial Matrix adapter placeholder definition replaced by real implementation below

// MatrixAdapter implementation moved to AdapterKind enum

#[derive(Clone)]
pub struct MatrixAdapter {
    pub homeserver: String,
    pub user: String,
    pub access_token: String,
    #[cfg(feature = "matrix-bridge")]
    pub client: Option<Arc<Client>>,
}

impl MatrixAdapter {
    pub fn name(&self) -> &'static str {
        "matrix"
    }
    pub fn description(&self) -> &'static str {
        "Matrix protocol bridge adapter"
    }

    pub async fn send_message(
        &self,
        message: &crate::core::message::Message,
    ) -> anyhow::Result<()> {
        #[cfg(feature = "matrix-bridge")]
        {
            if let Some(client) = &self.client {
                let room_id = RoomId::parse(&message.recipient)?;
                let content = RoomMessageEventContent::text_plain(&message.content);
                client
                    .get_room(&room_id)
                    .ok_or_else(|| anyhow::anyhow!("Room not found"))?
                    .send(content)
                    .await?;
                return Ok(());
            }
        }
        println!(
            "[MOCK] Matrix: Would send message to room: {}",
            message.content
        );
        Ok(())
    }

    pub async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>> {
        #[cfg(feature = "matrix-bridge")]
        {
            // Real sync logic would go here
            Ok(None)
        }
        #[cfg(not(feature = "matrix-bridge"))]
        Ok(None)
    }

    /// Initialize real Matrix client
    pub async fn init_client(&mut self) -> anyhow::Result<()> {
        #[cfg(feature = "matrix-bridge")]
        {
            let client = Client::builder()
                .homeserver_url(&self.homeserver)
                .build()
                .await?;
            client
                .matrix_auth()
                .login_username(
                    &self.user,
                    &std::env::var("GHOSTWIRE_MATRIX_PASSWORD")
                        .expect("GHOSTWIRE_MATRIX_PASSWORD env var must be set for Matrix bridge"),
                ) // Use env var
                .send()
                .await?;
            self.client = Some(Arc::new(client));
        }
        Ok(())
    }
}
// Usage:
// 1. Set your Matrix room ID in the code (replace !yourroomid:matrix.org)
// 2. Build with: cargo build --features matrix-bridge
// 3. Call MatrixAdapter::send_test_message("Hello from GhostWire!").await
// 4. Call receive_message() to print new messages

#[derive(Clone)]
pub struct MeshtasticAdapter {
    pub device_path: String,
    pub channel: String,
    pub erasure_armor: Arc<armor::ErasureArmor>,
}

impl MeshtasticAdapter {
    pub fn new(path: String, chan: String) -> Self {
        Self {
            device_path: path,
            channel: chan,
            erasure_armor: Arc::new(armor::ErasureArmor::new(3, 2)), // 3 data, 2 parity (Military grade)
        }
    }
    pub fn name(&self) -> &'static str {
        "meshtastic"
    }
    pub fn description(&self) -> &'static str {
        "Meshtastic protocol bridge adapter"
    }

    pub async fn send_message(
        &self,
        message: &crate::core::message::Message,
    ) -> anyhow::Result<()> {
        // Apply Military-Grade Erasure Coding (Fountain Code simulation)
        let shards = self.erasure_armor.encode(message.content.as_bytes());
        tracing::trace!(
            shards_len = shards.len(),
            device_path = %self.device_path,
            "Meshtastic: Splitting message into shards for extreme resilience"
        );

        #[cfg(feature = "meshtastic")]
        {
            let mut port = serialport::new(&self.device_path, 115_200)
                .timeout(std::time::Duration::from_millis(100))
                .open()?;

            for shard in shards {
                use std::io::Write;
                port.write_all(&shard)?;
            }
        }
        #[cfg(not(feature = "meshtastic"))]
        println!(
            "[MOCK] Meshtastic: Would send {} shards to device {}: {}",
            shards.len(),
            self.device_path,
            message.content
        );
        Ok(())
    }

    pub async fn receive_message(&self) -> anyhow::Result<Option<crate::core::message::Message>> {
        #[cfg(feature = "meshtastic")]
        {
            let mut port = serialport::new(&self.device_path, 115_200).open()?;
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            let n = port.read(serial_buf.as_mut_slice())?;
            if n > 0 {
                // Real implementation would parse Meshtastic protobuf here
                return Ok(None);
            }
        }
        println!(
            "[MOCK] Meshtastic: Would receive message from device {}",
            self.device_path
        );
        Ok(None)
    }
}

/// Core application state and management
#[derive(Clone)]
pub struct Core {
    pub config: crate::config::Config,
    pub identity: Arc<RwLock<Arc<Identity>>>,
    pub encryption: Arc<Encryption>,
    #[cfg(feature = "mesh")]
    pub mesh_manager: Option<Arc<RwLock<MeshManager>>>,
    /// Mesh initialized after a locked profile is unlocked. This slot avoids
    /// rebuilding Core while its persistence handles still own the profile DB.
    #[cfg(feature = "mesh")]
    pub late_mesh_manager: Arc<RwLock<Option<Arc<RwLock<MeshManager>>>>>,
    #[cfg(feature = "reticulum")]
    pub reticulum_manager: Arc<RwLock<Option<Arc<RwLock<ReticulumManager>>>>>,
    pub security_manager: Arc<SecurityManager>,
    pub transport_registry: Arc<RwLock<TransportRegistry>>,
    pub active_transport: Arc<RwLock<Option<crate::core::transport::TransportKind>>>,
    pub adapter_registry: Arc<RwLock<AdapterRegistry>>,
    pub active_adapter: Arc<RwLock<Option<AdapterKind>>>,

    // The Nervous System
    pub persistence: Arc<persistence_manager::PersistenceManager>,
    /// When this Core's profile was created, recorded once at construction.
    /// Previously the API returned `chrono::Utc::now()` per call, so
    /// `created_at` named the time of the READ rather than the creation.
    pub created_at: Arc<RwLock<Option<String>>>,
    pub monitor: Arc<monitor::NetworkMonitor>,
    pub switcher: Arc<RwLock<transport_manager::AutoTransportSwitcher>>,
    pub messages: Arc<RwLock<Vec<Message>>>,
    pub message_store: Arc<store::SecureMessageStore>,
    pub bootstrap_nodes: Vec<Multiaddr>,
    /// GhostWire-One capability state; not authorized or ready by default.
    pub bootstrap_worker: Arc<RwLock<bootstrap::BootstrapWorker>>,
    pub event_bus: broadcast::Sender<CoreEvent>,
    pub handshake_secret: Option<Vec<u8>>,

    /// Cached mesh peer count so sync get_peer_count() can return it without awaiting.
    /// Updated whenever get_mesh_stats() is called (async path); read by get_peer_count().
    pub mesh_peer_count_cache: Arc<AtomicUsize>,
    /// Ensures the actor-backed mesh driver is started at most once per Core.
    #[cfg(feature = "mesh")]
    pub mesh_started: Arc<AtomicBool>,

    /// Security components for rate limiting and audit logging
    pub failed_attempt_tracker: Arc<crate::core::security_v2::FailedAttemptTracker>,
    pub audit_logger: Arc<crate::core::security_v2::AuditLogger>,
    // Wave 2: Offline delivery
    pub mailbox_store: Arc<crate::core::mailbox::MailboxStore>,
    // Wave 2: Trust-on-first-use
    pub trust_store: Arc<crate::core::reputation::TrustStore>,
    // Wave 3: Content-addressed storage
    pub block_store: Arc<crate::core::blockstore::BlockStore>,
    // Wave 4: Knowledge Ark
    pub knowledge_ark: Arc<crate::core::knowledge::knowledge_ark::KnowledgeArk>,
    // Wave 4: Performance caches
    pub identity_cache: Arc<crate::core::micro_optimizations::IdentityCache>,
    pub peer_cache: Arc<crate::core::micro_optimizations::PeerAddressCache>,
    pub message_dedup: Arc<crate::core::micro_optimizations::MessageDeduplicator>,
    // Wave 5: Social graph
    pub social_graph: Arc<crate::core::social::social_graph::SocialGraph>,
    /// Canonical discovery evidence directory; discovery never implies trust.
    pub discovery_directory: Arc<crate::core::discovery_directory::DiscoveryDirectory>,
    // Wave 6: Ghost Channels
    pub ghost_channels: Arc<crate::core::channels::ghost_channels::GhostChannelRegistry>,
    // Security: panic mode and dead man's switch
    pub panic_manager: Arc<crate::core::identity::panic::PanicManager>,
    // DTN: delay-tolerant networking store
    pub dtn_store: Arc<crate::core::dtn::epidemic::DtnStore>,
    // Wave 7: Device pairing
    pub pairing_manager: Arc<crate::core::identity_pairing::device_pairing::PairingManager>,
    // P0: Reliability layer (ACK/retry/TTL)
    pub reliability: Arc<reliability::ReliabilityLayer>,

    // AI/ML Components (Gemma 4 + ONNX)
    pub gemma_client: Arc<OnceLock<Arc<ai::GemmaClient>>>,
    pub anomaly_detector: Option<Arc<ai::IsolationForestModel>>,
    pub onnx_router: Option<Arc<ai::RoutingClassifier>>,
    pub gnn_router: Option<Arc<ai::GnnRouter>>,
    pub feature_extractor: Arc<RwLock<ai::FeatureExtractor>>,
    // Trust/reputation uses the existing TrustStore (reputation.rs) — not a duplicate
}

fn local_resource_budget_ok() -> bool {
    #[cfg(target_os = "linux")]
    {
        let memory_ok = std::fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|contents| {
                contents.lines().find_map(|line| {
                    let mut fields = line.split_whitespace();
                    (fields.next() == Some("MemAvailable:"))
                        .then(|| fields.next()?.parse::<u64>().ok())
                })
            })
            .flatten()
            .map(|available_kib| available_kib >= 32 * 1024)
            .unwrap_or(false);
        let load_ok = std::fs::read_to_string("/proc/loadavg")
            .ok()
            .and_then(|contents| contents.split_whitespace().next()?.parse::<f64>().ok())
            .map(|one_minute_load| one_minute_load < 16.0)
            .unwrap_or(false);
        memory_ok && load_ok
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

impl Core {
    pub async fn new(
        config: crate::config::Config,
        bootstrap_nodes: Vec<Multiaddr>,
        sim_addr: Option<String>,
        profile: Option<String>,
    ) -> Result<Self> {
        let identity = Arc::new(
            Identity::new(profile.clone())
                .map_err(|e| anyhow::anyhow!("Identity creation failed: {}", e))?,
        );
        let encryption = Arc::new(
            Encryption::new().map_err(|e| anyhow::anyhow!("Encryption creation failed: {}", e))?,
        );
        // Link Armor to Encryption for future internal morphing
        // (Uses interior mutability or we could have passed it to new if we changed the signature further)
        // For now, we'll keep it in Core and use it in orchestration.

        let security_config = SecurityConfig::default();
        let security_manager = Arc::new(SecurityManager::new(security_config));
        let mut registry = TransportRegistry::new();
        let mut active_transport_inner = None; // Set this based on user config or default

        if let Some(addr) = sim_addr {
            let sim_transport = Arc::new(RwLock::new(
                crate::core::sim_transport::SimTransport::new(identity.id.clone(), &addr).await?,
            ));
            let sim_kind = crate::core::transport::TransportKind::Simulated(sim_transport);
            registry.register(sim_kind.clone());
            active_transport_inner = Some(sim_kind);
        }

        {
            let quic_port = if config.server.quic_port == 0 {
                config.server.port.wrapping_add(1).max(5001)
            } else {
                config.server.quic_port
            };
            let quic_addr: std::net::SocketAddr = format!("0.0.0.0:{}", quic_port).parse()?;
            let quic =
                match crate::core::quic_transport::QuicTransportProvider::new(quic_addr).await {
                    Ok(q) => Ok(q),
                    Err(_) => {
                        let fallback: std::net::SocketAddr = "0.0.0.0:0".parse()?;
                        crate::core::quic_transport::QuicTransportProvider::new(fallback).await
                    }
                };
            match quic {
                Ok(quic) => {
                    let quic_kind = crate::core::transport::TransportKind::Quic(Arc::new(quic));
                    registry.register(quic_kind.clone());
                    if active_transport_inner.is_none() {
                        active_transport_inner = Some(quic_kind);
                    }
                    tracing::info!("QUIC transport registered on port {}", quic_port);
                }
                Err(e) => {
                    tracing::warn!("QUIC transport init failed (port {}): {}", quic_port, e);
                }
            }
        }

        let registry = Arc::new(RwLock::new(registry));
        let active_transport = Arc::new(RwLock::new(active_transport_inner));

        // Initialize Persistence.
        //
        // A normal node opens LOCKED and stays that way until the unlock API is
        // called, which is correct for a real identity. A dev node instead
        // opens in machine-key mode so it can actually store things, because
        // an automated test has nobody to type a passphrase.
        //
        // `GHOSTWIRE_DEV_PROFILE` is opt-in and loud: it is checked here, it
        // has to be set deliberately, and the startup banner labels the node.
        let dev_profile = std::env::var("GHOSTWIRE_DEV_PROFILE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let persistence = if dev_profile {
            tracing::warn!(
                target: "core",
                "DEV PROFILE MODE: persistence is machine-keyed and NOT passphrase \
                 protected. This node is for testing and must not carry real \
                 secrets."
            );
            let base = match std::env::var("GHOSTWIRE_HOME") {
                Ok(h) => std::path::PathBuf::from(h).join("vault"),
                Err(_) => dirs::home_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
                    .join(".ghostwire")
                    .join("vault"),
            };
            Arc::new(
                persistence_manager::PersistenceManager::new_machine_mode_with_base_dir(
                    base,
                    profile.clone(),
                )?,
            )
        } else {
            Arc::new(persistence_manager::PersistenceManager::new_locked(
                profile.clone(),
            )?)
        };

        // Adapter registry
        let mut adapter_registry = AdapterRegistry::new();

        // GhostWire deliberately has no hard-coded external bootstrap identities.
        // An empty list means standalone mode until explicit GhostWire-native,
        // network-scoped bootstrap records are supplied by local configuration,
        // cached discovery, or the future supervised bootstrap worker.
        let bootstrap_nodes = bootstrap_nodes;
        if bootstrap_nodes.is_empty() {
            tracing::info!(
                "No GhostWire bootstrap providers configured; starting in standalone mode"
            );
        } else {
            tracing::info!(
                providers = bootstrap_nodes.len(),
                "Using explicitly configured GhostWire bootstrap providers"
            );
        }

        // Register Matrix adapter if enabled
        if config.features.matrix_bridge {
            if let Some(matrix_config) = config.get_matrix_config() {
                adapter_registry.register(AdapterKind::Matrix(MatrixAdapter {
                    homeserver: matrix_config.homeserver,
                    user: matrix_config.user,
                    access_token: matrix_config.access_token,
                    #[cfg(feature = "matrix-bridge")]
                    client: None,
                }));
            }
        }
        adapter_registry.register(AdapterKind::Meshtastic(MeshtasticAdapter::new(
            "/dev/ttyUSB0".to_string(),
            "main".to_string(),
        )));
        let adapter_registry = Arc::new(RwLock::new(adapter_registry));
        let active_adapter = Arc::new(RwLock::new(None));

        // Initialize the Nervous System (10/10 DNA)
        let disaster_manager = disaster::DisasterManager::new(
            500, // 500ms latency threshold
            0.2, // 20% packet loss threshold
            1,   // Minimum 1 peer for isolation state
            3,   // Require 3 stable samples before recovering (Anti-Thrashing)
        );
        let switcher = Arc::new(RwLock::new(transport_manager::AutoTransportSwitcher::new(
            disaster_manager,
            disaster::DisasterPolicy::default(),
        )));

        let messages = Arc::new(RwLock::new(Vec::new()));
        let message_store = Arc::new(store::SecureMessageStore::new(
            persistence.clone(),
            encryption.clone(),
        ));
        let (event_bus, _) = broadcast::channel(1024); // Large buffer for high traffic

        // Parse handshake secret from config if available
        let handshake_secret = config
            .network
            .stealth_handshake_secret
            .as_ref()
            .and_then(|s| {
                if let Ok(bytes) = hex::decode(s) {
                    Some(bytes)
                } else {
                    tracing::warn!(
                        "Failed to decode stealth_handshake_secret from config (expected hex)"
                    );
                    None
                }
            });

        let reliability = Arc::new(crate::core::reliability::ReliabilityLayer::with_defaults());
        let discovery_directory =
            Arc::new(crate::core::discovery_directory::DiscoveryDirectory::new());

        #[cfg(feature = "mesh")]
        let mesh_manager_inner = {
            if let Some(peer_id) = identity.public_key().map(|pk| pk.to_peer_id()) {
                let username = identity.get_metadata().nickname;

                let username_discovery = Arc::new(
                    crate::core::username_discovery::UsernameDiscovery::new(peer_id, username),
                );

                Some(Arc::new(RwLock::new(
                    MeshManager::new(
                        identity.clone(),
                        encryption.clone(),
                        messages.clone(),
                        message_store.clone(),
                        bootstrap_nodes.clone(),
                        username_discovery,
                        Some(event_bus.clone()),
                        handshake_secret.clone(),
                        reliability.clone(),
                        discovery_directory.clone(),
                    )
                    .await?,
                )))
            } else {
                tracing::info!("Identity is locked. Deferring MeshManager initialization.");
                None
            }
        };

        #[cfg(not(feature = "mesh"))]
        let mesh_manager_inner = None;

        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = mesh_manager_inner.clone() {
            let mesh_kind = crate::core::transport::TransportKind::Mesh(mesh_manager);
            registry.write().await.register(mesh_kind);
            tracing::info!("libp2p mesh transport registered");
        }

        let monitor = Arc::new(monitor::NetworkMonitor::new(
            #[cfg(feature = "mesh")]
            mesh_manager_inner.clone(),
            std::time::Duration::from_secs(5), // Check every 5 seconds
        ));

        // Clone identity Arc for use in multiple field initializations
        let identity_for_fields = identity.clone();

        // Bootstrap capability is intentionally disabled and unauthorized until an
        // explicit operator control path enables it. This prevents accidental
        // promotion of ordinary nodes and keeps GhostWire-One policy fail-closed.
        let bootstrap_worker = Arc::new(RwLock::new(bootstrap::BootstrapWorker::new(
            bootstrap::BootstrapPolicy::default(),
        )));
        let core = Self {
            config,
            identity: Arc::new(RwLock::new(identity)),
            encryption,
            security_manager,
            transport_registry: registry,
            active_transport,
            adapter_registry,
            active_adapter,
            monitor,
            switcher,
            persistence: persistence.clone(),
            // Stamped once, here, so `created_at` is stable for the life of the
            // profile instead of being recomputed on every API read.
            created_at: Arc::new(RwLock::new(Some(chrono::Utc::now().to_rfc3339()))),
            #[cfg(feature = "mesh")]
            mesh_manager: mesh_manager_inner,
            #[cfg(feature = "mesh")]
            late_mesh_manager: Arc::new(RwLock::new(None)),
            #[cfg(feature = "reticulum")]
            reticulum_manager: Arc::new(RwLock::new(None)),
            messages,
            message_store,
            bootstrap_nodes,
            bootstrap_worker,
            event_bus,
            handshake_secret,
            mesh_peer_count_cache: Arc::new(AtomicUsize::new(0)),
            #[cfg(feature = "mesh")]
            mesh_started: Arc::new(AtomicBool::new(false)),
            failed_attempt_tracker: Arc::new(FailedAttemptTracker::default()),
            audit_logger: Arc::new(AuditLogger::default()),
            // Wave 2-6 subsystems
            mailbox_store: {
                let storage_root = if let Ok(home) = std::env::var("GHOSTWIRE_HOME") {
                    std::path::PathBuf::from(home).join("mailbox")
                } else {
                    dirs::data_dir()
                        .unwrap_or_else(|| std::path::PathBuf::from("."))
                        .join("ghostwire")
                };
                let db_path = storage_root
                    .join(profile.as_deref().unwrap_or("default"))
                    .join("db");
                let db = sled::open(&db_path)
                    .unwrap_or_else(|e| panic!("sled open failed at {}: {}", db_path.display(), e));
                Arc::new(
                    // Fail CLOSED: if the real mailbox won't open, boot must stop
                    // here. The old fallback silently swapped in a temporary
                    // store with a published passphrase, and mail written to it
                    // evaporated on restart.
                    crate::core::mailbox::MailboxStore::open(Arc::new(db), persistence.clone())
                        .expect("mailbox open failed — refusing to boot on a temporary store"),
                )
            },
            trust_store: Arc::new(crate::core::reputation::TrustStore::new()),
            block_store: Arc::new(crate::core::blockstore::BlockStore::new(256 * 1024)),
            knowledge_ark: {
                let node_id = identity_for_fields.id.clone();
                Arc::new(crate::core::knowledge::knowledge_ark::KnowledgeArk::new(
                    node_id,
                ))
            },
            identity_cache: Arc::new(crate::core::micro_optimizations::IdentityCache::new()),
            peer_cache: Arc::new(crate::core::micro_optimizations::PeerAddressCache::new(
                std::time::Duration::from_secs(300),
            )),
            message_dedup: Arc::new(crate::core::micro_optimizations::MessageDeduplicator::new(
                std::time::Duration::from_secs(60),
            )),
            social_graph: {
                let node_id = identity_for_fields.id.clone();
                let pub_key = identity_for_fields.get_public_key().to_vec();
                let (mut sg, _rx) =
                    crate::core::social::social_graph::SocialGraph::new(node_id, pub_key);
                sg.attach_directory(discovery_directory.clone());
                Arc::new(sg)
            },
            discovery_directory,
            ghost_channels: Arc::new(
                crate::core::channels::ghost_channels::GhostChannelRegistry::new(),
            ),
            panic_manager: Arc::new(crate::core::identity::panic::PanicManager::new(
                crate::core::identity::panic::PanicConfig::default(),
            )),
            dtn_store: {
                let node_id_bytes = {
                    let id_str = identity_for_fields.id.clone();
                    let mut arr = [0u8; 32];
                    let bytes = id_str.as_bytes();
                    let len = bytes.len().min(32);
                    arr[..len].copy_from_slice(&bytes[..len]);
                    arr
                };
                Arc::new(crate::core::dtn::epidemic::DtnStore::new(node_id_bytes))
            },
            pairing_manager: {
                let ed_pub = (*identity_for_fields).public_key_bytes();
                let x_pub = vec![]; // X25519 key derived from Ed25519 - wire properly after first test
                Arc::new(
                    crate::core::identity_pairing::device_pairing::PairingManager::new(
                        identity_for_fields.id.clone(),
                        ed_pub,
                        x_pub,
                    ),
                )
            },
            // AI/ML Components - Initialize Gemma client via OnceLock
            gemma_client: Arc::new(OnceLock::new()),
            // Load trained Isolation Forest model — try mesh-domain first, fall back to legacy
            anomaly_detector: match ai::IsolationForestModel::from_file(
                "models/mesh_anomaly_detector.json",
            ) {
                Ok(model) => {
                    tracing::info!(
                        "✅ Mesh anomaly detector loaded: {} trees, {} features (Guifi.net)",
                        model.n_estimators,
                        model.n_features
                    );
                    Some(Arc::new(model))
                }
                Err(_) => match ai::IsolationForestModel::from_file("models/anomaly_detector.json")
                {
                    Ok(model) => {
                        tracing::warn!(
                            "⚠️ Using legacy NSL-KDD anomaly detector ({} features) — retrain on mesh data",
                            model.n_features
                        );
                        Some(Arc::new(model))
                    }
                    Err(e) => {
                        tracing::debug!("No anomaly detector model: {}", e);
                        None
                    }
                },
            },
            // Load ONNX routing model (Layer 1 fast path)
            onnx_router: match ai::RoutingClassifier::new("models/ghostwire_routing.onnx") {
                Ok(router) => {
                    tracing::info!("✅ AI routing system loaded (LightGBM + Fallback chain)");
                    Some(Arc::new(router))
                }
                Err(e) => {
                    tracing::debug!("AI routing not loaded (PRoPHET fallback): {}", e);
                    None
                }
            },
            // Load GNN routing model (Layer 3 — Graph Attention Network)
            gnn_router: match ai::GnnRouter::new(
                "models/gnn_routing/gnn_routing.onnx",
                "models/gnn_routing/node_embeddings.json",
            ) {
                Ok(router) => {
                    tracing::info!("✅ GNN routing system loaded (GAT, Guifi.net trained)");
                    Some(Arc::new(router))
                }
                Err(e) => {
                    tracing::debug!("GNN routing not loaded (PRoPHET fallback): {}", e);
                    None
                }
            },
            // Feature extractor for L1 anomaly detection
            feature_extractor: Arc::new(RwLock::new(ai::FeatureExtractor::new())),
            reliability,
        };

        Ok(core)
    }

    /// Process a DTN encounter with GNN-enhanced routing
    ///
    /// When GNN router is available, builds mesh topology from current peer state
    /// and uses GNN predictions to optimize bundle forwarding.
    /// Falls back to standard PRoPHET when GNN is unavailable or inference fails.
    pub async fn handle_dtn_encounter(
        &self,
        their_node_id: &[u8; 32],
        their_bloom: &dtn::epidemic::SyncBloom,
        their_delivery_scores: &[([u8; 32], f32)],
    ) -> Vec<dtn::bundle::DtnBundle> {
        // If GNN router is available, try GNN-enhanced forwarding
        if let Some(gnn) = &self.gnn_router {
            // Build topology from current mesh peer state
            let peers = self.collect_mesh_peer_features().await;

            if !peers.is_empty() {
                let routing_table = self.collect_routing_table().await;
                let link_metrics = self.collect_link_metrics().await;

                let topology = ai::build_topology_from_mesh(&peers, &routing_table, &link_metrics);

                // Register all peers in the GNN mapper
                for peer in peers.keys() {
                    gnn.register_peer(peer.clone()).await;
                }

                // Build GNN decisions for each bundle destination
                let mut gnn_decisions = std::collections::HashMap::new();
                let bundles = self.dtn_store.bundles.read().await;
                let our_id_str = self.identity.read().await.id.clone();
                let mut our_id_bytes = [0u8; 32];
                let bytes = our_id_str.as_bytes();
                our_id_bytes[..bytes.len().min(32)].copy_from_slice(&bytes[..bytes.len().min(32)]);

                let destinations: Vec<[u8; 32]> = bundles
                    .values()
                    .filter(|b| !b.is_expired() && b.destination != our_id_bytes)
                    .map(|b| b.destination)
                    .collect();
                drop(bundles);

                // For each destination, get GNN next-hop prediction
                for dest in &destinations {
                    // Find src node (us) in topology
                    let mapper = gnn.mapper.read().await;
                    let our_peer = mapper
                        .peers()
                        .iter()
                        .find(|p| {
                            let mut buf = [0u8; 32];
                            let id_bytes = p.to_bytes();
                            let len = id_bytes.len().min(32);
                            buf[..len].copy_from_slice(&id_bytes[..len]);
                            buf == our_id_bytes
                        })
                        .cloned();

                    if let Some(our_peer) = our_peer {
                        if let Some(&src_idx) = mapper.peer_to_index.get(&our_peer) {
                            // Find dest in mapper
                            let dest_peer = mapper
                                .peers()
                                .iter()
                                .find(|p| {
                                    let mut buf = [0u8; 32];
                                    let id_bytes = p.to_bytes();
                                    let len = id_bytes.len().min(32);
                                    buf[..len].copy_from_slice(&id_bytes[..len]);
                                    &buf == dest
                                })
                                .cloned();

                            if let Some(dest_peer) = dest_peer {
                                if let Some(&dst_idx) = mapper.peer_to_index.get(&dest_peer) {
                                    // Get candidates (our neighbors in topology)
                                    if let Some(neighbors) = topology.adjacency.get(&src_idx) {
                                        if !neighbors.is_empty() {
                                            let all_health = if let Some(mesh) =
                                                self.effective_mesh_manager().await
                                            {
                                                let mesh = mesh.read().await;
                                                Some(mesh.peer_health().get_all_health().await)
                                            } else {
                                                None
                                            };
                                            let mut peer_features =
                                                std::collections::HashMap::new();
                                            for &cand in neighbors {
                                                let features = all_health
                                                    .as_ref()
                                                    .and_then(|hl| {
                                                        let peer_id = mapper.get_peer(cand)?;
                                                        let peer_str = peer_id.to_string();
                                                        hl.iter()
                                                            .find(|h| h.peer_id == peer_str)
                                                            .map(|h| ai::RoutingFeatures {
                                                                latency_ms: h.avg_rtt_ms as u64,
                                                                packet_loss: h.packet_loss_ratio()
                                                                    as f32,
                                                                trust_score: if h.is_alive {
                                                                    1.0
                                                                } else {
                                                                    0.0
                                                                },
                                                                ..Default::default()
                                                            })
                                                    })
                                                    .unwrap_or_default();
                                                peer_features.insert(cand, features);
                                            }

                                            match gnn
                                                .select_next_hop(
                                                    &topology,
                                                    src_idx,
                                                    dst_idx,
                                                    neighbors,
                                                    &peer_features,
                                                )
                                                .await
                                            {
                                                Ok(decision) => {
                                                    // Check if encountered peer is the recommended next-hop
                                                    let their_peer = mapper
                                                        .peers()
                                                        .iter()
                                                        .find(|p| {
                                                            let mut buf = [0u8; 32];
                                                            let id_bytes = p.to_bytes();
                                                            let len = id_bytes.len().min(32);
                                                            buf[..len]
                                                                .copy_from_slice(&id_bytes[..len]);
                                                            &buf == their_node_id
                                                        })
                                                        .cloned();

                                                    if let Some(their_peer) = their_peer {
                                                        let their_idx =
                                                            mapper.get_index(&their_peer);
                                                        let should_forward = their_idx
                                                            .map(|idx| {
                                                                decision
                                                                    .scored_candidates
                                                                    .iter()
                                                                    .any(|(p, _)| {
                                                                        mapper.get_index(p)
                                                                            == Some(idx)
                                                                    })
                                                            })
                                                            .unwrap_or(false);
                                                        gnn_decisions.insert(
                                                            *dest,
                                                            (should_forward, decision.confidence),
                                                        );
                                                    }
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Use GNN-enhanced encounter if we got decisions
                if !gnn_decisions.is_empty() {
                    tracing::debug!(
                        "GNN-enhanced DTN encounter: {} decisions",
                        gnn_decisions.len()
                    );
                    return self
                        .dtn_store
                        .on_encounter_with_gnn(
                            their_node_id,
                            their_bloom,
                            their_delivery_scores,
                            &gnn_decisions,
                        )
                        .await;
                }
            }
        }

        // Fallback to standard PRoPHET encounter
        tracing::debug!("Standard PRoPHET DTN encounter (GNN unavailable or no decisions)");
        self.dtn_store
            .on_encounter(their_node_id, their_bloom, their_delivery_scores)
            .await
    }

    async fn collect_mesh_peer_features(
        &self,
    ) -> std::collections::HashMap<libp2p::PeerId, ai::RoutingFeatures> {
        let mesh_manager = match self.effective_mesh_manager().await {
            Some(m) => m,
            None => return std::collections::HashMap::new(),
        };
        let mesh = mesh_manager.read().await;

        let topology = mesh.get_topology().await;
        let all_health = mesh.peer_health().get_all_health().await;

        // peer_health tracks by string peer_id; mesh topology uses the same string keys
        let health_map: std::collections::HashMap<&str, &peer_health::PeerHealth> =
            all_health.iter().map(|h| (h.peer_id.as_str(), h)).collect();

        let trust_avg = self.trust_store.average_trust_score().await;
        let now_hour = {
            use chrono::Timelike;
            chrono::Local::now().hour() as u8
        };

        let mut features = std::collections::HashMap::new();
        for (id, node) in &topology.nodes {
            if !node.is_online {
                continue;
            }

            let peer_health_data = health_map.get(id.as_str());
            let latency = peer_health_data.map(|h| h.avg_rtt_ms as u64).unwrap_or(100);
            let loss = peer_health_data
                .map(|h| h.packet_loss_ratio() as f32)
                .unwrap_or(0.0);

            // connection_quality (0.0-1.0) → rough RSSI estimate
            let rssi = (node.connection_quality * 50.0 - 90.0) as i16;

            features.insert(
                node.peer_id,
                ai::RoutingFeatures {
                    rssi,
                    latency_ms: latency,
                    packet_loss: loss,
                    trust_score: trust_avg,
                    hop_count: 1,
                    battery_level: 1.0,
                    time_of_day: now_hour,
                },
            );
        }

        features
    }

    /// Collect routing table from mesh state
    async fn collect_routing_table(
        &self,
    ) -> std::collections::HashMap<
        libp2p::PeerId,
        std::collections::HashMap<libp2p::PeerId, libp2p::PeerId>,
    > {
        std::collections::HashMap::new()
    }

    /// Collect link metrics from mesh state
    async fn collect_link_metrics(
        &self,
    ) -> std::collections::HashMap<(libp2p::PeerId, libp2p::PeerId), f32> {
        std::collections::HashMap::new()
    }

    /// Get our node ID as bytes (for DTN operations)
    pub fn our_node_id(&self) -> [u8; 32] {
        // This matches how dtn_store was initialized in new()
        let id_str = self.identity.blocking_read().id.clone();
        let mut arr = [0u8; 32];
        let bytes = id_str.as_bytes();
        let len = bytes.len().min(32);
        arr[..len].copy_from_slice(&bytes[..len]);
        arr
    }

    /// Starts the autonomic nervous system heartbeat
    pub async fn start_nervous_system(self: Arc<Self>, mesh_bind_ip: String, mesh_port: u16) {
        tracing::info!("🧠 Activating GhostWire Nervous System (Autonomic Response enabled)");

        // Start the network monitor
        self.monitor.clone().start().await;

        // Start L1 periodic anomaly detection loop
        let core_anomaly = self.clone();
        tokio::spawn(async move {
            // Wait for system to stabilize
            tokio::time::sleep(std::time::Duration::from_secs(15)).await;

            // Load scaler if available
            let scaler_path = "models/guifisants_scaler.json";
            if let Ok(()) = core_anomaly
                .feature_extractor
                .write()
                .await
                .load_scaler(scaler_path)
                .await
            {
                tracing::info!("✅ L1 feature scaler loaded from {}", scaler_path);
            } else {
                tracing::warn!(
                    "⚠️ L1 feature scaler not loaded (using raw features): {}",
                    scaler_path
                );
            }

            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;

                // Collect local system metrics
                match ai::FeatureExtractor::collect_local_system_metrics() {
                    Ok(local_metrics) => {
                        // Build feature vector — for now, only local node metrics
                        // Future: collect from all mesh peers
                        let mut node_map = std::collections::HashMap::new();

                        // Map local node to node_id 23 (the first Guifisants node)
                        // In production, this would map actual mesh node IDs
                        node_map.insert(23u16, local_metrics);

                        let features = core_anomaly
                            .feature_extractor
                            .read()
                            .await
                            .build_feature_vector(&node_map);

                        // Run anomaly detection
                        if let Some(prob) = core_anomaly.check_mesh_anomaly(&features) {
                            if prob > 0.283 {
                                // Optimal threshold from training
                                tracing::warn!(
                                    "🚨 MESH ANOMALY DETECTED — probability: {:.4} (threshold: 0.283)",
                                    prob
                                );
                            } else {
                                tracing::debug!(
                                    "L1 mesh check — anomaly probability: {:.4} (normal)",
                                    prob
                                );
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("⚠️ Failed to collect system metrics: {}", e);
                    }
                }
            }
        });

        // Start AI health check loop
        let core_ai = self.clone();
        tokio::spawn(async move {
            // Try to initialize Gemma client after a short delay
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            if let Err(e) = core_ai.initialize_gemma_client().await {
                tracing::warn!("⚠️ Gemma 4 AI unavailable (will retry): {}", e);
            }
        });

        // Start periodic mesh topology analysis with Gemma 4 AI
        let core_mesh_ai = self.clone();
        tokio::spawn(async move {
            // Wait for initial AI initialization
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;

            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            loop {
                interval.tick().await;

                // Check if Gemma client is initialized
                if let Some(gemma_client) = core_mesh_ai.get_gemma_client() {
                    // Get mesh stats if available
                    #[cfg(feature = "mesh")]
                    if let Some(stats) = core_mesh_ai.get_mesh_stats().await {
                        // Build telemetry from available mesh stats
                        let nodes = vec![ai::NodeTelemetry {
                            node_id: stats.local_node_id.clone(),
                            rssi: -50.0, // Default — RSSI not yet tracked in mesh stats
                            latency_ms: stats.connection_stats.avg_connection_time as f32,
                            packet_loss: if stats.connection_stats.total_connections > 0 {
                                stats.connection_stats.failed_connections as f32
                                    / stats.connection_stats.total_connections as f32
                            } else {
                                0.0
                            },
                            battery_level: 1.0, // Unknown for now
                            hop_count: stats.routes_count as u32,
                        }];

                        match gemma_client
                            .analyze_mesh(nodes, "Optimize relay selection")
                            .await
                        {
                            Ok(analysis) => {
                                tracing::info!(
                                    "🤖 AI Mesh Analysis — Health: {:.1}%, Relays: {}, Anomalies: {}",
                                    analysis.network_health * 100.0,
                                    analysis.optimal_relays.len(),
                                    analysis.anomalies.len()
                                );
                                for recommendation in &analysis.recommendations {
                                    tracing::info!("  → {}", recommendation);
                                }
                            }
                            Err(e) => {
                                tracing::warn!("⚠️ AI mesh analysis failed: {}", e);
                            }
                        }
                    }
                }
            }
        });

        // Start the autonomic switcher loop
        let core = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
            loop {
                interval.tick().await;
                let metrics = core.monitor.get_metrics().await;
                let eligible: Vec<crate::core::disaster::TransportType> = {
                    let registry = core.transport_registry.read().await;
                    registry.eligible_selector_transports()
                };

                // An empty live registry is a real degraded state, not permission
                // to select a compiled-but-unregistered fallback transport.
                if eligible.is_empty() {
                    tracing::warn!(
                        "Autonomic transport selection paused: no eligible registered transport"
                    );
                    continue;
                }

                let mut switcher = core.switcher.write().await;
                let previous_transport = switcher.current_transport();
                if let Some(new_transport) = switcher.update_and_evaluate(&metrics, &eligible) {
                    // A selector decision is not a successful switch until the
                    // registry can apply the corresponding live transport.
                    let registry = core.transport_registry.read().await;
                    if let Some(transport) = registry.get(new_transport.name()) {
                        let mut active = core.active_transport.write().await;
                        *active = Some(transport.clone());
                        tracing::info!(
                            "Autonomic system switched transport to {:?}",
                            new_transport
                        );
                    } else {
                        switcher.restore_transport(previous_transport);
                        tracing::warn!(
                            attempted = new_transport.name(),
                            restored = previous_transport.name(),
                            "Transport selection rejected: no registered transport is available"
                        );
                    }
                }
            }
        });

        // Cover traffic heartbeat — send cover frames to prevent idle detection
        let core_chaff = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                if core_chaff.active_adapter.read().await.is_some() {
                    tracing::debug!("📡 Sending Military-Grade Chaff Traffic (DPI Evasion)");
                    let dummy = crate::core::message::Message::new(
                        core_chaff.get_identity_id().await,
                        "CHAFF_TARGET".to_string(),
                        "DUMMY_TRAFFIC_DATA_NOOP".to_string(),
                    );
                    let _ = core_chaff.send_adapter_message(&dummy).await;
                }
            }
        });

        // Auto-Start Mesh Network (Zero-Conf)
        let core_mesh = self.clone();
        tokio::spawn(async move {
            // Bind to the specified IP, with a random port (0) for P2P
            let listen_addr = format!("/ip4/{}/tcp/{}", mesh_bind_ip, mesh_port);
            match core_mesh.start_mesh(&listen_addr).await {
                Ok(_) => {
                    tracing::info!("✅ Mesh Network Auto-Started on {}", listen_addr);
                    // Give the swarm a moment to bind, then print addresses
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    let listeners = core_mesh.get_mesh_listeners().await;
                    if !listeners.is_empty() {
                        println!("\n--- GhostWire Mesh Node Started ---");
                        if let Some(peer_id) = core_mesh
                            .identity
                            .read()
                            .await
                            .public_key()
                            .map(|pk| pk.to_peer_id())
                        {
                            println!("Peer ID: {}", peer_id);
                        } else {
                            println!("Peer ID: <Locked>");
                        }
                        println!("Listening on:");
                        for addr in listeners {
                            println!("  {}", addr);
                        }
                        println!("------------------------------------\n");
                    }
                }
                Err(e) => tracing::warn!("⚠️ Failed to auto-start mesh: {}", e),
            }
        });
    }

    /// Select an active transport by name
    pub async fn set_active_transport(&self, name: &str) -> Result<()> {
        let registry = self.transport_registry.read().await;
        if let Some(transport) = registry.get(name) {
            let mut active = self.active_transport.write().await;
            *active = Some(transport.clone());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Transport not found: {}", name))
        }
    }

    /// Send a message through the selected transport
    pub async fn send_message(&self, message: &Message) -> Result<()> {
        // Enforce Lockdown
        if self.security_manager.is_lockdown().await {
            tracing::warn!(
                "BLOCKED: Attempted to send message during LOCKDOWN: {}",
                message.id
            );
            return Err(anyhow::anyhow!(
                "SYSTEM LOCKDOWN ACTIVE - OUTGOING TRAFFIC BLOCKED"
            ));
        }

        self.reliability.on_message_sent(message.id);

        // Persist the message before transmitting it.
        //
        // This used to log a persistence failure and carry on, which produced a
        // third variant of the false-success pattern in this codebase: the API
        // answered `{"status":"accepted"}` with HTTP 200 while nothing was
        // written, and the only evidence was an ERROR line in a log file. A
        // caller told "accepted" has no way to know the message is not
        // recoverable, so the failure is now returned.
        if message.recipient == "broadcast" {
            // Broadcast has no recipient key, so plaintext is correct here
            // rather than a fallback — a broadcast is public by definition.
            self.message_store
                .save_message_plain(message)
                .map_err(|e| anyhow::anyhow!("broadcast NOT persisted: {e}"))?;
        } else {
            self.message_store.save_message(message).map_err(|e| {
                anyhow::anyhow!(
                    "message NOT persisted (encrypted storage refused it: {e}). \
                     Nothing was transmitted."
                )
            })?;
        }

        // Add to in-memory history
        {
            let mut messages = self.messages.write().await;
            messages.push(message.clone());
        }

        // Record message for security stats
        self.security_manager.record_message(&message.sender).await;

        // Broadcast to Event Bus
        let _ = self.event_bus.send(CoreEvent::NewMessage(message.clone()));

        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            let mut mgr = mesh_manager.write().await;
            return mgr.send_message(message).await;
        }

        if let Some(transport) = self.active_transport.read().await.as_ref() {
            return transport.send_message(message).await;
        } else {
            tracing::trace!(
                "Message processed locally (no active transport): {}",
                message.id
            );
            Ok(())
        }
    }

    pub async fn send_direct_message(&self, recipient: String, message: &Message) -> Result<()> {
        // Enforce Lockdown
        if self.security_manager.is_lockdown().await {
            tracing::warn!(
                "BLOCKED: Attempted to send direct message during LOCKDOWN: {}",
                message.id
            );
            return Err(anyhow::anyhow!(
                "SYSTEM LOCKDOWN ACTIVE - OUTGOING TRAFFIC BLOCKED"
            ));
        }

        self.reliability.on_message_sent(message.id);

        // Persist message to encrypted storage.
        //
        // This USED TO fall back to `save_message_plain` when the encrypted write
        // failed, on the theory that "the recipient key is unknown". That is a
        // silent downgrade of the confidentiality guarantee: a direct message
        // that failed to encrypt was then written to disk in the clear, and the
        // only trace was a `tracing::debug!` line, invisible at default log
        // level. For a network whose premise is anonymity, degrading to
        // plaintext storage on a recoverable error is the wrong default — the
        // message is still transmitted end-to-end encrypted, so the send itself
        // is fine; only the local copy is lost, and losing it is far better
        // than writing it in the clear.
        //
        // Broadcast keeps its plaintext write, deliberately and in its own
        // branch above: a broadcast is public by definition.
        self.message_store.save_message(message).map_err(|e| {
            anyhow::anyhow!(
                "message NOT persisted (encrypted storage refused it: {e}). \
                 The message was still transmitted, but no local copy exists."
            )
        })?;

        // Add to in-memory history
        {
            let mut messages = self.messages.write().await;
            messages.push(message.clone());
        }

        // Record message for security stats
        self.security_manager.record_message(&message.sender).await;

        // We do NOT broadcast direct messages to the event bus indiscriminately
        // But we might want to emit a "MessageSent" event for UI updates

        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            let mut mgr = mesh_manager.write().await;
            if let Ok(peer_id) = recipient.parse::<libp2p::PeerId>() {
                return mgr.send_direct_message(peer_id, message.clone()).await;
            } else {
                return Err(anyhow::anyhow!("Invalid P2P Recipient ID: {}", recipient));
            }
        }

        Err(anyhow::anyhow!("Mesh network not available"))
    }

    /// Initialize a default transport for basic functionality
    pub async fn init_default_transport(&self) -> Result<()> {
        let provider = Arc::new(crate::core::stealth_tcp::StealthTCPProvider::new(
            None,
            self.security_manager.clone(),
            None,
            true, // Enable Stealth Mode by default
        ));

        let mut active = self.active_transport.write().await;
        *active = Some(crate::core::transport::TransportKind::StealthTCP(provider));
        tracing::info!("Initialized StealthTCP transport (Default) 🥷");

        Ok(())
    }

    /// Select an active protocol adapter by name
    pub async fn set_active_adapter(&self, name: &str) -> Result<()> {
        let registry = self.adapter_registry.read().await;
        if let Some(adapter) = registry.get(name) {
            let mut active = self.active_adapter.write().await;
            *active = Some(adapter.clone());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Adapter not found: {}", name))
        }
    }
    /// Send a message through the selected protocol adapter
    pub async fn send_adapter_message(&self, message: &Message) -> Result<()> {
        if let Some(adapter) = self.active_adapter.read().await.as_ref() {
            adapter.send_message(message).await
        } else {
            Err(anyhow::anyhow!("No active adapter selected"))
        }
    }

    /// Get messages from both in-memory buffer and encrypted store
    /// This ensures incoming mesh messages are visible via the API even if
    /// they haven't been persisted to the encrypted store yet.
    pub async fn get_messages(
        &self,
        peer_id: &str,
        limit: usize,
    ) -> Vec<crate::core::message::Message> {
        use crate::core::message::Message;

        // Start with in-memory messages (includes incoming mesh messages)
        let mut messages: Vec<Message> = {
            let mem = self.messages.read().await;
            mem.iter()
                .filter(|m| m.recipient == peer_id || m.sender == peer_id || peer_id == "all")
                .cloned()
                .collect()
        };

        // Also load from encrypted store
        if let Ok(store_messages) = self.message_store.load() {
            for m in store_messages {
                if m.recipient == peer_id || m.sender == peer_id || peer_id == "all" {
                    // Avoid duplicates — check if already in memory list
                    if !messages.iter().any(|existing| existing.id == m.id) {
                        messages.push(m);
                    }
                }
            }
        }

        // Sort by timestamp (newest first) and limit
        messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        messages.truncate(limit);
        messages
    }

    /// Get the identity ID
    pub async fn get_identity_id(&self) -> String {
        self.identity.read().await.id.clone()
    }

    /// Check if a display name profile has been set
    pub async fn has_profile(&self) -> bool {
        let metadata = self.get_metadata().await;
        !metadata.nickname.is_empty()
    }

    /// Save a display name to the identity profile
    pub async fn save_profile(&self, display_name: &str) -> Result<()> {
        self.update_metadata(Some(display_name.to_string()), None, None, Some(true))
            .await;
        if !self.persistence.is_unlocked() {
            tracing::warn!(
                "Profile metadata changed in memory; unlock persistence before saving it to disk"
            );
        }
        tracing::info!("Profile saved: {}", display_name);
        Ok(())
    }

    /// Get the DID
    pub async fn get_did(&self) -> String {
        self.identity.read().await.did.clone()
    }

    /// Get the metadata
    pub async fn get_metadata(&self) -> crate::core::identity::IdentityMetadata {
        self.identity.read().await.get_metadata()
    }

    /// Update metadata
    pub async fn update_metadata(
        &self,
        nickname: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
        setup_complete: Option<bool>,
    ) {
        self.identity
            .read()
            .await
            .update_metadata(nickname, bio, avatar_url, setup_complete);
    }

    /// Get the public key
    pub fn get_public_key(&self) -> Vec<u8> {
        self.encryption.export_public_key()
    }

    pub async fn is_locked(&self) -> bool {
        self.identity.read().await.is_locked()
    }

    /// Whether the persistence layer can currently read and write private
    /// records. This is the lock that actually blocks message writes; the
    /// identity lock reported by `is_locked` is a different subsystem and can
    /// be open while this one is shut.
    pub async fn persistence_is_unlocked(&self) -> bool {
        self.persistence.is_unlocked()
    }

    /// The profile's real creation time, or `None` when it was never persisted.
    /// Returning the current time here would be a fabrication, so this does
    /// not guess: it reports what is actually recorded.
    pub async fn get_created_at(&self) -> Option<String> {
        self.created_at.read().await.clone()
    }

    pub async fn lock(&self) -> Result<()> {
        self.encryption
            .lock_private_state()
            .map_err(|error| anyhow::anyhow!("encryption lock failed: {}", error))?;
        self.persistence.lock()?;
        self.identity
            .read()
            .await
            .lock()
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    pub async fn unlock(&self, password: &str) -> Result<()> {
        self.identity
            .read()
            .await
            .unlock(password)
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        let persistence_result = if self.persistence.is_user_mode()? {
            self.persistence.unlock_with_passphrase(password)
        } else {
            self.persistence
                .migrate_legacy_with_verified_passphrase(password)
        };
        if let Err(error) = persistence_result
            .and_then(|_| self.mailbox_store.migrate_legacy_records())
            .and_then(|_| {
                self.encryption
                    .migrate_to_user_keyed(&self.persistence)
                    .map_err(|error| {
                        anyhow::anyhow!("encryption user-key migration failed: {}", error)
                    })
            })
        {
            let _ = self.encryption.lock_private_state();
            let _ = self.persistence.lock();
            let _ = self.identity.read().await.lock();
            return Err(anyhow::anyhow!(
                "persistence, mailbox, or encryption migration failed; profile was re-locked: {}",
                error
            ));
        }
        Ok(())
    }

    /// Explicitly recover a legacy machine-sidecar identity and migrate its
    /// matching persistence store. Normal startup never calls this method.
    pub async fn recover_legacy_identity(&self) -> Result<()> {
        let password = self
            .identity
            .read()
            .await
            .unlock_from_legacy_keyfile()
            .map_err(|e| anyhow::anyhow!("legacy identity recovery failed: {}", e))?;
        let persistence_result = if self.persistence.is_user_mode()? {
            self.persistence.unlock_with_passphrase(password.as_str())
        } else {
            self.persistence
                .migrate_legacy_with_verified_passphrase(password.as_str())
        };
        if let Err(error) = persistence_result
            .and_then(|_| self.mailbox_store.migrate_legacy_records())
            .and_then(|_| {
                self.encryption
                    .migrate_to_user_keyed(&self.persistence)
                    .map_err(|error| {
                        anyhow::anyhow!("encryption user-key migration failed: {}", error)
                    })
            })
        {
            let _ = self.encryption.lock_private_state();
            let _ = self.persistence.lock();
            let _ = self.identity.read().await.lock();
            return Err(anyhow::anyhow!(
                "legacy identity, mailbox, or encryption migration failed; profile was re-locked: {}",
                error
            ));
        }
        Ok(())
    }

    pub async fn save_to_disk(&self, password: &str) -> Result<()> {
        if self.persistence.is_unlocked() {
            self.persistence.change_passphrase(password)?;
        } else if self.persistence.is_user_mode()? {
            self.persistence.unlock_with_passphrase(password)?;
        } else if !self.persistence.initialize_user_mode_if_empty(password)? {
            self.persistence
                .migrate_legacy_with_verified_passphrase(password)?;
        }
        self.mailbox_store.migrate_legacy_records()?;
        self.encryption
            .migrate_to_user_keyed(&self.persistence)
            .map_err(|error| anyhow::anyhow!("encryption user-key migration failed: {}", error))?;
        self.identity
            .read()
            .await
            .save_to_disk(password)
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    /// Recover account from DHT
    pub async fn recover_account(&self, username: &str, recovery_password: &str) -> Result<()> {
        use sha2::Digest;
        let mut shares = Vec::new();
        let threshold = 3;

        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            let mgr = mesh_manager.read().await;
            for i in 0..5 {
                let dht_key_preimage = format!("{}:recovery:{}", username, i);
                let dht_key = hex::encode(sha2::Sha256::digest(dht_key_preimage.as_bytes()));

                if let Ok(data) = mgr
                    .get_dht_record(hex::decode(dht_key).unwrap_or_default())
                    .await
                {
                    if let Ok(share) = serde_json::from_slice::<
                        crate::core::identity::EncryptedRecoveryShare,
                    >(&data)
                    {
                        shares.push(share);
                    }
                }

                if shares.len() >= threshold {
                    break;
                }
            }
        }

        if shares.len() < threshold {
            return Err(anyhow::anyhow!(
                "Finding recovery shares failed ({} found, threshold {})",
                shares.len(),
                threshold
            ));
        }

        // Recover secret
        let secret = crate::core::identity::RecoveryManager::recover_identity_secret(
            shares,
            recovery_password,
            threshold,
        )?;

        // Create matching identity
        let new_identity = Arc::new(Identity::from_seed(&secret)?);

        // Atomic swap
        {
            let mut id_lock = self.identity.write().await;
            *id_lock = new_identity;
        }

        tracing::info!("Account recovered successfully for {}", username);
        Ok(())
    }

    /// Get the key ID
    pub fn get_key_id(&self) -> String {
        self.encryption.get_key_id()
    }

    /// Set PQC Armor mode
    pub async fn set_pqc_mode(&self, enabled: bool) {
        // Implementation logic for toggling Kyber requirement
        // For now we'll just log it as the actual encryption module will check this
        tracing::info!("Post-Quantum Armor mode set to: {}", enabled);
    }

    /// Set Regional Shard
    pub async fn set_dht_shard(&self, shard: &str) -> anyhow::Result<()> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            let mut mgr = mesh_manager.write().await;
            mgr.set_shard(shard).await?;
            tracing::info!("DHT Regional Shard updated to: {}", shard);
            return Ok(());
        }
        Err(anyhow::anyhow!("Mesh networking not available"))
    }

    /// Get network topology as JSON (nodes, routes, local_node_id from mesh when available).
    pub async fn get_network_topology(&self) -> Result<String> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            let topology = mesh_manager.read().await.get_topology().await;
            return serde_json::to_string_pretty(&topology)
                .map_err(|e| anyhow::anyhow!("Topology serialization failed: {}", e));
        }
        #[cfg(not(feature = "mesh"))]
        let _ = self;
        Ok(serde_json::json!({
            "nodes": {},
            "routes": {},
            "local_node_id": "",
            "message": "Mesh not enabled or not initialized"
        })
        .to_string())
    }

    #[cfg(feature = "mesh")]
    pub async fn effective_mesh_manager(&self) -> Option<Arc<RwLock<MeshManager>>> {
        if let Some(mesh_manager) = &self.mesh_manager {
            return Some(mesh_manager.clone());
        }
        self.late_mesh_manager.read().await.clone()
    }

    /// Initialize mesh network
    pub async fn init_mesh(&mut self) -> Result<()> {
        #[cfg(feature = "mesh")]
        {
            if self.mesh_manager.is_none() {
                let peer_id = self
                    .identity
                    .read()
                    .await
                    .public_key()
                    .map(|pk| pk.to_peer_id())
                    .ok_or_else(|| anyhow::anyhow!("Cannot init mesh: Identity is locked"))?;
                let username = self.identity.read().await.get_metadata().nickname;

                let username_discovery = Arc::new(
                    crate::core::username_discovery::UsernameDiscovery::new(peer_id, username),
                );

                let mesh_manager = MeshManager::new(
                    self.identity.read().await.clone(),
                    self.encryption.clone(),
                    self.messages.clone(),
                    self.message_store.clone(),
                    self.bootstrap_nodes.clone(),
                    username_discovery,
                    Some(self.event_bus.clone()),
                    self.handshake_secret.clone(),
                    self.reliability.clone(),
                    self.discovery_directory.clone(),
                )
                .await?;
                let mut mesh_manager = mesh_manager;
                mesh_manager.set_peer_count_cache(self.mesh_peer_count_cache.clone());
                mesh_manager.set_dtn_store(self.dtn_store.clone());
                mesh_manager.set_mailbox_store(self.mailbox_store.clone());
                self.mesh_manager = Some(Arc::new(RwLock::new(mesh_manager)));
                tracing::info!("Mesh network initialized");
            }
        }
        #[cfg(not(feature = "mesh"))]
        {
            tracing::warn!("Mesh feature not enabled");
        }
        Ok(())
    }

    /// Initialize the mesh in-place after a profile unlock.
    ///
    /// A locked profile cannot create MeshManager during Core construction.
    /// Rebuilding Core after unlock is unsafe because the old Core still owns
    /// the profile database lock. Instead, create MeshManager once and attach
    /// it through the interior-mutable late slot.
    pub async fn initialize_mesh_after_unlock(&self) -> Result<()> {
        #[cfg(feature = "mesh")]
        {
            if self.mesh_manager.is_some() {
                return Ok(());
            }
            let mut slot = self.late_mesh_manager.write().await;
            if slot.is_some() {
                return Ok(());
            }

            let identity = self.identity.read().await.clone();
            let peer_id = identity
                .public_key()
                .map(|pk| pk.to_peer_id())
                .ok_or_else(|| anyhow::anyhow!("Cannot init mesh: Identity is locked"))?;
            let username_discovery =
                Arc::new(crate::core::username_discovery::UsernameDiscovery::new(
                    peer_id,
                    identity.get_metadata().nickname,
                ));
            let mut mesh_manager = MeshManager::new(
                identity,
                self.encryption.clone(),
                self.messages.clone(),
                self.message_store.clone(),
                self.bootstrap_nodes.clone(),
                username_discovery,
                Some(self.event_bus.clone()),
                self.handshake_secret.clone(),
                self.reliability.clone(),
                self.discovery_directory.clone(),
            )
            .await?;
            mesh_manager.set_peer_count_cache(self.mesh_peer_count_cache.clone());
            mesh_manager.set_dtn_store(self.dtn_store.clone());
            mesh_manager.set_mailbox_store(self.mailbox_store.clone());

            let handle = Arc::new(RwLock::new(mesh_manager));
            self.transport_registry
                .write()
                .await
                .register(crate::core::transport::TransportKind::Mesh(handle.clone()));
            *slot = Some(handle);
            tracing::info!("Mesh network initialized after profile unlock");
        }
        #[cfg(not(feature = "mesh"))]
        {
            tracing::warn!("Mesh feature not enabled");
        }
        Ok(())
    }

    /// Refresh and return the local GhostWire-One capability state.
    ///
    /// This method deliberately does not authorize or start the worker. It only
    /// evaluates observable prerequisites and expires stale records.
    pub async fn get_bootstrap_state(&self) -> bootstrap::BootstrapCapabilityState {
        let stable_identity = !self.is_locked().await;
        let mesh_listener_count = self.get_mesh_listeners().await.len();
        let reticulum_listener_started = self
            .get_reticulum_stats()
            .await
            .map(|stats| stats.initialized && stats.interfaces_configured > 0)
            .unwrap_or(false);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let mut worker = self.bootstrap_worker.write().await;
        let input = bootstrap::BootstrapEligibilityInput {
            stable_identity,
            listener_started: mesh_listener_count > 0 || reticulum_listener_started,
            advertised_address_count: mesh_listener_count + usize::from(reticulum_listener_started),
            clock_sane: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .is_ok(),
            resource_budget_ok: local_resource_budget_ok(),
            network_id_matches: worker.policy().network_id == "ghostwire-one",
        };
        worker.evaluate_eligibility(&input);
        worker.expire(now);
        worker.state()
    }

    /// Enable or disable GhostWire-One bootstrap capability. This never grants
    /// authorization and never starts the worker.
    pub async fn configure_bootstrap(&self, enabled: bool) {
        if !enabled {
            if let Some(mesh_manager) = self.effective_mesh_manager().await {
                let _ = mesh_manager
                    .read()
                    .await
                    .set_world_discovery_provider(false)
                    .await;
            }
        }
        self.bootstrap_worker.write().await.configure(enabled);
    }

    /// Explicit local control hook for an authenticated operator/API path.
    /// Authorization is never inferred from eligibility.
    pub async fn authorize_bootstrap(&self, authorized: bool) {
        if !authorized {
            if let Some(mesh_manager) = self.effective_mesh_manager().await {
                let _ = mesh_manager
                    .read()
                    .await
                    .set_world_discovery_provider(false)
                    .await;
            }
        }
        self.bootstrap_worker.write().await.authorize(authorized);
    }

    /// Start the GhostWire-One bootstrap capability after explicit authorization
    /// and current eligibility have been established.
    pub async fn start_bootstrap(&self) -> anyhow::Result<()> {
        let _ = self.get_bootstrap_state().await;
        self.bootstrap_worker
            .write()
            .await
            .start()
            .map_err(|e| anyhow::anyhow!("bootstrap start rejected: {:?}", e))?;

        // Starting supervision is not itself proof that a concrete listener is
        // available. Confirm the runtime listener now and expose that truth to
        // the API/UI; the worker remains started-but-not-ready if no listener
        // is bound yet, rather than claiming bootstrap capability prematurely.
        let mesh_listener_ready = !self.get_mesh_listeners().await.is_empty();
        let reticulum_listener_ready = self
            .get_reticulum_stats()
            .await
            .map(|stats| stats.initialized && stats.interfaces_configured > 0)
            .unwrap_or(false);
        let runtime_ready = mesh_listener_ready || reticulum_listener_ready;
        self.bootstrap_worker
            .write()
            .await
            .mark_ready(runtime_ready)
            .map_err(|e| anyhow::anyhow!("bootstrap readiness confirmation failed: {:?}", e))?;
        if runtime_ready {
            if let Some(mesh_manager) = self.effective_mesh_manager().await {
                let manager = mesh_manager.read().await;
                manager.set_world_discovery_provider(true).await?;
                // Publication is best-effort until a real externally routable
                // listener exists; health remains truthful if it is withheld.
                let _ = manager.publish_world_discovery().await;
            }
        }
        Ok(())
    }
    /// Stop local bootstrap service and clear its ephemeral admission state.
    pub async fn stop_bootstrap(&self) {
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            let _ = mesh_manager
                .read()
                .await
                .set_world_discovery_provider(false)
                .await;
        }
        self.bootstrap_worker.write().await.stop();
    }
    pub async fn world_discovery_health(
        &self,
    ) -> crate::core::world_discovery::WorldDiscoveryHealth {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            return mesh_manager.read().await.world_discovery_health().await;
        }
        crate::core::world_discovery::WorldDiscoveryHealth::new(
            false,
            crate::core::world_discovery::configured_network_id(),
        )
    }

    pub async fn publish_world_discovery(&self) -> anyhow::Result<()> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            return mesh_manager.read().await.publish_world_discovery().await;
        }
        Err(anyhow::anyhow!("mesh discovery is unavailable"))
    }

    pub async fn refresh_world_discovery_providers(&self) -> anyhow::Result<()> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            return mesh_manager
                .read()
                .await
                .refresh_world_discovery_providers()
                .await;
        }
        Err(anyhow::anyhow!("mesh discovery is unavailable"))
    }

    /// Add bootstrap nodes dynamically
    pub async fn add_bootstrap_nodes(&self, nodes: Vec<Multiaddr>) {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            let mut mgr = mesh_manager.write().await;
            tracing::info!("Added {} bootstrap nodes dynamically", nodes.len());
            mgr.add_bootstrap_nodes(nodes);
        }
    }

    /// Enable DHT-based account recovery
    pub async fn enable_recovery(&self, recovery_password: &str) -> Result<Vec<String>> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            let shares = crate::core::identity::RecoveryManager::create_recovery_shares(
                &(*self.identity.read().await),
                recovery_password,
                3, // Threshold
                5, // Total shares
            )?;

            let mgr = mesh_manager.write().await;
            let mut stored_indices = Vec::new();

            for share in shares {
                // index_hash is hex string of the DHT key
                let key = hex::decode(&share.index_hash)?;
                // We serialize the share struct to bytes for storage
                let value = serde_json::to_vec(&share)?;
                mgr.put_dht_record(key, value).await?;
                stored_indices.push(share.index_hash);
            }

            tracing::info!("Recovery enabled. Shares stored at: {:?}", stored_indices);
            Ok(stored_indices)
        } else {
            Err(anyhow::anyhow!(
                "Mesh network not active. Please start the mesh first."
            ))
        }
        #[cfg(not(feature = "mesh"))]
        Err(anyhow::anyhow!("Mesh feature not enabled"))
    }

    /// Initialize Reticulum network
    pub async fn init_reticulum(&self) -> Result<()> {
        #[cfg(feature = "reticulum")]
        {
            let mut slot = self.reticulum_manager.write().await;
            if slot.is_none() {
                let identity = self.identity.read().await.clone();
                let node_id = identity.id.clone();
                let interfaces_ini = self.config.network.reticulum_interfaces.clone();
                let reticulum_manager =
                    ReticulumManager::new(identity, &interfaces_ini, &node_id).await?;
                let reticulum_transport = reticulum_manager.transport_arc();
                let reticulum_handle = Arc::new(RwLock::new(reticulum_manager));
                self.transport_registry.write().await.register(
                    crate::core::transport::TransportKind::Reticulum(reticulum_transport),
                );
                *slot = Some(reticulum_handle);
                tracing::info!("Reticulum network initialized and registered");
            }
        }
        #[cfg(not(feature = "reticulum"))]
        {
            tracing::warn!("Reticulum feature not enabled");
        }
        Ok(())
    }

    /// Stop the Reticulum medium and remove its runtime registration.
    pub async fn stop_reticulum(&self) -> Result<()> {
        #[cfg(feature = "reticulum")]
        {
            if let Some(reticulum_manager) = self.reticulum_manager.write().await.take() {
                let mut manager = reticulum_manager.write().await;
                manager.stop().await?;
            }
            self.transport_registry
                .write()
                .await
                .unregister("reticulum");
            tracing::info!("Reticulum network stopped and unregistered");
        }
        #[cfg(not(feature = "reticulum"))]
        {
            tracing::warn!("Reticulum feature not enabled");
        }
        Ok(())
    }

    /// Get mesh network statistics.
    /// Also updates the peer count cache so get_peer_count() returns a recent value.
    pub async fn get_mesh_stats(&self) -> Option<MeshStats> {
        #[cfg(feature = "mesh")]
        {
            if let Some(mesh_manager) = self.effective_mesh_manager().await {
                let manager = mesh_manager.read().await;
                let stats = manager.get_stats().await;
                self.mesh_peer_count_cache
                    .store(stats.total_nodes, Ordering::Relaxed);
                Some(stats)
            } else {
                None
            }
        }
        #[cfg(not(feature = "mesh"))]
        {
            None
        }
    }

    /// Get Reticulum network statistics
    pub async fn get_reticulum_stats(&self) -> Option<ReticulumStats> {
        #[cfg(feature = "reticulum")]
        {
            let slot = self.reticulum_manager.read().await;
            if let Some(reticulum_manager) = slot.as_ref() {
                let manager = reticulum_manager.read().await;
                Some(manager.get_stats().await)
            } else {
                None
            }
        }
        #[cfg(not(feature = "reticulum"))]
        {
            None
        }
    }

    /// Get security statistics
    pub async fn get_security_stats(&self) -> SecurityStats {
        self.security_manager.get_security_stats().await
    }

    pub async fn connect_meshtastic(&self, address: &str) -> anyhow::Result<()> {
        let mut registry = self.adapter_registry.write().await;
        registry.register(AdapterKind::Meshtastic(MeshtasticAdapter::new(
            address.to_string(),
            "main".to_string(),
        )));
        Ok(())
    }

    pub async fn connect_reticulum(&self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "reticulum")]
        {
            let slot = self.reticulum_manager.read().await;
            let reticulum_manager = slot
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Reticulum is not initialized"))?;
            let mut manager = reticulum_manager.write().await;
            manager.connect(address).await?;
            return Ok(());
        }
        #[cfg(not(feature = "reticulum"))]
        {
            let _ = address;
            Err(anyhow::anyhow!("Reticulum feature not enabled").into())
        }
    }

    pub async fn get_reticulum_topology(&self) -> Result<Option<ReticulumTopology>> {
        #[cfg(feature = "reticulum")]
        {
            let slot = self.reticulum_manager.read().await;
            if let Some(reticulum_manager) = slot.as_ref() {
                let manager = reticulum_manager.read().await;
                Ok(Some(manager.get_topology().await))
            } else {
                Ok(None)
            }
        }
        #[cfg(not(feature = "reticulum"))]
        {
            Ok(None)
        }
    }

    pub async fn start_mesh(&self, address: &str) -> anyhow::Result<()> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            if self.mesh_started.swap(true, Ordering::AcqRel) {
                return Ok(());
            }
            let addr = address.parse()?;
            let mesh_manager = mesh_manager.clone();
            let mesh_started = self.mesh_started.clone();
            tokio::spawn(async move {
                let mut mgr = mesh_manager.write().await;
                if let Err(e) = mgr.start(addr).await {
                    mesh_started.store(false, Ordering::Release);
                    tracing::error!("Mesh loop failed: {}", e);
                }
            });
            return Ok(());
        }
        Err(anyhow::anyhow!("Mesh networking not available"))
    }
    pub async fn get_mesh_topology(&self) -> Option<MeshTopology> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            return Some(mesh_manager.read().await.get_topology().await);
        }
        None
    }

    pub async fn get_mesh_listeners(&self) -> Vec<Multiaddr> {
        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = self.effective_mesh_manager().await {
            return mesh_manager.read().await.get_listeners().await;
        }
        Vec::new()
    }

    /// Peer count: from mesh cache when mesh is enabled, else from encryption layer.
    /// The mesh cache is updated whenever get_mesh_stats() is called (e.g. by HUD or CLI).
    pub fn get_peer_count(&self) -> usize {
        #[cfg(feature = "mesh")]
        if self.mesh_manager.is_some()
            || self
                .late_mesh_manager
                .try_read()
                .map(|slot| slot.is_some())
                .unwrap_or(false)
        {
            return self.mesh_peer_count_cache.load(Ordering::Relaxed);
        }
        self.encryption.get_peer_count()
    }

    pub async fn shutdown(&self) -> Result<()> {
        tracing::info!("Core initiating shutdown sequence...");

        // 1. Stop Monitor
        self.monitor.stop();

        // 2. Stop local BootstrapWorker
        self.stop_bootstrap().await;
        // 3. Stop Mesh
        #[cfg(feature = "mesh")]
        if let Some(mesh_mgr) = &self.mesh_manager {
            let mgr = mesh_mgr.read().await;
            mgr.stop().await?;
        }

        // 4. Stop Reticulum
        #[cfg(feature = "reticulum")]
        if let Some(ret_mgr) = self.reticulum_manager.write().await.take() {
            let mut mgr = ret_mgr.write().await;
            mgr.stop().await?;
        }

        // 5. Flush Persistence
        self.persistence.flush()?;

        tracing::info!("Core shutdown complete.");
        Ok(())
    }

    /// Initialize Gemma 4 AI client if Ollama is available
    async fn initialize_gemma_client(&self) -> Result<(), String> {
        // Try different model variants based on available hardware
        let model = if self.config.features.gemma_4b {
            "gemma4:e4b"
        } else if self.config.features.gemma_26b {
            "gemma4:26b"
        } else {
            "gemma4:e2b" // Default: lightweight edge model
        };

        let client = ai::GemmaClient::new("http://localhost:11434", model);

        if client.health_check().await {
            tracing::info!("✅ Gemma 4 AI initialized with model: {}", model);
            // Store in OnceLock — this is thread-safe and can only succeed once
            self.gemma_client
                .set(Arc::new(client))
                .map_err(|_| "Gemma client already initialized".to_string())?;
            Ok(())
        } else {
            Err("Ollama server not reachable at localhost:11434".to_string())
        }
    }

    /// Get the Gemma client if initialized
    pub fn get_gemma_client(&self) -> Option<Arc<ai::GemmaClient>> {
        self.gemma_client.get().cloned()
    }

    /// Check mesh state for anomalies using the ONNX LightGBM model (Layer 1)
    ///
    /// Takes a 2,387-dimensional feature vector from mesh monitoring.
    /// Returns anomaly probability in [0.0, 1.0] — higher means more likely anomaly.
    /// Falls back to None if model unavailable.
    pub fn check_mesh_anomaly(&self, features: &[f32]) -> Option<f32> {
        self.onnx_router.as_ref()?.check_anomaly(features)
    }

    /// Score a link for anomalies using the Isolation Forest model
    ///
    /// Returns anomaly result with score, classification, and probability.
    /// Returns None if model unavailable.
    pub fn score_anomaly(&self, features: &[f32]) -> Option<ai::AnomalyResult> {
        self.anomaly_detector.as_ref().map(|m| m.score(features))
    }
}

// Ponytail: zeroize handshake_secret before memory is freed.
// Core's Drop runs before field drops, so all Arc fields are cleaned up normally.
impl Drop for Core {
    fn drop(&mut self) {
        if let Some(ref mut secret) = self.handshake_secret {
            secret.zeroize();
        }
    }
}

// Documentation: MatrixAdapter and MeshtasticAdapter now have config fields and stubbed logic. Replace stubs with real SDK/API calls for production bridging.
// Documentation: The core now supports modular, pluggable transports via TransportRegistry. Use feature flags to include/exclude transports at build time. Select active transport at runtime for message routing.
// Documentation: The core now supports modular, pluggable protocol adapters via AdapterRegistry. Register new adapters as plugins. Select active adapter at runtime for protocol bridging.

// Feature-guarded type stubs for when features are disabled
#[cfg(not(feature = "mesh"))]
pub struct MeshStats {
    pub peer_count: usize,
    pub message_count: usize,
    pub uptime_seconds: u64,
}

#[cfg(not(feature = "mesh"))]
pub struct MeshNode {
    pub id: String,
    pub name: String,
    pub is_online: bool,
}

#[cfg(not(feature = "mesh"))]
pub struct MeshTopology {
    pub nodes: Vec<MeshNode>,
    pub connections: Vec<(String, String)>,
}

#[cfg(not(feature = "mesh"))]
pub struct MeshManager {
    pub identity: Arc<Identity>,
}

#[cfg(not(feature = "mesh"))]
impl MeshManager {
    pub async fn new(_identity: Arc<Identity>) -> Result<Self> {
        Ok(Self {
            identity: _identity,
        })
    }

    /// Peer count is provided by Core via mesh_peer_count_cache when mesh is active.
    pub fn get_peer_count(&self) -> usize {
        0
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(not(feature = "reticulum"))]
#[derive(Serialize)]
pub struct ReticulumStats {
    pub total_nodes: usize,
    pub online_nodes: usize,
    pub routes_count: usize,
}

#[cfg(not(feature = "reticulum"))]
pub struct ReticulumTopology {
    pub nodes: Vec<String>,
    pub connections: Vec<(String, String)>,
}

#[cfg(not(feature = "reticulum"))]
pub struct ReticulumManager {
    pub identity: Arc<Identity>,
}

#[cfg(not(feature = "reticulum"))]
impl ReticulumManager {
    pub async fn new(_identity: Arc<Identity>) -> Result<Self> {
        Ok(Self {
            identity: Arc::new(
                Identity::new(None)
                    .map_err(|e| anyhow::anyhow!("Failed to create identity: {}", e))?,
            ),
        })
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod core_test;
