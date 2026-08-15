//! core_wiring.rs — The missing integration layer
//!
//! This file contains ALL the exact changes needed to wire every new module
//! from Waves 2-5 into the Core struct and startup sequence.
//!
//! APPLY THIS FIRST. It is the glue between all the new modules and
//! the existing Core. Without this, none of the new code runs.

// ═══════════════════════════════════════════════════════════════════════════
// CHANGE 1: src/core/mod.rs — Add module declarations
// ═══════════════════════════════════════════════════════════════════════════
//
// Add these pub mod declarations after the existing ones:
//
// pub mod encrypted_store;
// pub mod mailbox;
// pub mod recovery_shamir;
// pub mod traffic_armor;
// pub mod typed_crypto;
// pub mod wire_protocol;
// pub mod blockstore;
// pub mod knowledge {
//     pub mod knowledge_ark;
// }
// pub mod social {
//     pub mod social_graph;
//     pub mod ghost_channels;
// }
// pub mod routing {
//     pub mod private_routing;
//     pub mod sphinx;
//     pub mod skademlia;
//     pub mod mixing_delay;
// }
// pub mod transport {
//     pub mod pluggable_transport;
// }
// pub mod network {
//     pub mod presence_obfuscation;
// }
// pub mod channels {
//     pub mod ghost_channels;
// }
//
// Rename/replace:
//   pub mod disaster; → keep but replace with new disaster.rs
//   pub mod reputation; → replace with new reputation.rs (TrustStore)
//   pub mod updater; → replace with updater_fixed.rs

// ═══════════════════════════════════════════════════════════════════════════
// CHANGE 2: src/core/mod.rs — Core struct: add new fields
// ═══════════════════════════════════════════════════════════════════════════
//
// Inside `pub struct Core { ... }` add these fields:
//
//   // Social layer (Wave 5)
//   pub social_graph: Arc<crate::core::social::social_graph::SocialGraph>,
//
//   // Knowledge Ark (Wave 4)
//   pub knowledge_ark: Arc<crate::core::knowledge::knowledge_ark::KnowledgeArk>,
//
//   // Content-addressed file store (Wave 3)
//   pub block_store: Arc<crate::core::blockstore::BlockStore>,
//
//   // Offline delivery (Wave 2)
//   pub mailbox_store: Arc<crate::core::mailbox::MailboxStore>,
//
//   // Trust store / TOFU (Wave 2, replaces reputation.rs stub)
//   pub trust_store: Arc<crate::core::reputation::TrustStore>,
//
//   // Disaster fallback manager (Wave 2 replacement)
//   pub disaster_mgr: Arc<crate::core::disaster::DisasterManager>,
//
//   // Device pairing (Wave 4)
//   pub pairing_manager: Arc<crate::core::identity::device_pairing::PairingManager>,
//
//   // Ghost channels (Wave 6)
//   pub ghost_channels: Arc<crate::core::channels::ghost_channels::GhostChannelRegistry>,
//
//   // Micro-optimizations (Wave 4)
//   pub identity_cache: Arc<crate::core::micro_optimizations::IdentityCache>,
//   pub peer_cache: Arc<crate::core::micro_optimizations::PeerAddressCache>,
//   pub message_dedup: Arc<crate::core::micro_optimizations::MessageDeduplicator>,
//   pub session_warmer: Arc<crate::core::micro_optimizations::SessionWarmer>,

// ═══════════════════════════════════════════════════════════════════════════
// CHANGE 3: src/core/mod.rs — Core::new() initialization
// ═══════════════════════════════════════════════════════════════════════════
//
// Inside Core::new(), after existing init code, add:
//
//   // Open sled database
//   let db_path = dirs::data_dir()
//       .unwrap_or_else(|| std::path::PathBuf::from("."))
//       .join("ghostwire")
//       .join(profile.as_deref().unwrap_or("default"))
//       .join("db");
//   let db = Arc::new(sled::open(&db_path)?);
//
//   // MailboxStore
//   let mailbox_store = Arc::new(
//       crate::core::mailbox::MailboxStore::open(db.clone())?
//   );
//
//   // TrustStore (replaces NoOpReputationGuard)
//   let trust_store = Arc::new(crate::core::reputation::TrustStore::new());
//
//   // BlockStore
//   let block_store = Arc::new(
//       crate::core::blockstore::BlockStore::new(256 * 1024)
//   );
//
//   // SocialGraph
//   let (social_graph, _social_rx) = crate::core::social::social_graph::SocialGraph::new(
//       identity.id.clone(),
//       identity.get_public_key().to_vec(),
//   );
//   let social_graph = Arc::new(social_graph);
//
//   // KnowledgeArk
//   let knowledge_ark = Arc::new(
//       crate::core::knowledge::knowledge_ark::KnowledgeArk::new(identity.id.clone())
//   );
//
//   // PairingManager
//   let pairing_manager = Arc::new(
//       crate::core::identity::device_pairing::PairingManager::new(
//           identity.id.clone(),
//           identity.get_public_key().to_vec(),
//           encryption.get_x25519_public_key().to_vec(),
//       )
//   );
//
//   // GhostChannelRegistry
//   let ghost_channels = Arc::new(
//       crate::core::channels::ghost_channels::GhostChannelRegistry::new()
//   );
//
//   // Micro-optimizations
//   let identity_cache = Arc::new(crate::core::micro_optimizations::IdentityCache::new());
//   let peer_cache = Arc::new(crate::core::micro_optimizations::PeerAddressCache::new(
//       std::time::Duration::from_secs(300)
//   ));
//   let message_dedup = Arc::new(crate::core::micro_optimizations::MessageDeduplicator::new(
//       std::time::Duration::from_secs(60)
//   ));
//   let session_warmer = Arc::new(crate::core::micro_optimizations::SessionWarmer::new());
//
//   // DisasterManager (from Wave 5 disaster.rs)
//   let (disaster_tx, _disaster_rx) = tokio::sync::mpsc::channel(64);
//   let disaster_mgr = Arc::new(crate::core::disaster::DisasterManager::new(disaster_tx));
//   disaster_mgr.clone().start_monitor();
//
// Then add all new fields to the Core { ... } return value.

// ═══════════════════════════════════════════════════════════════════════════
// CHANGE 4: src/web.rs — AppState: add social_graph + ark fields
// ═══════════════════════════════════════════════════════════════════════════
//
// pub struct AppState {
//     pub core: Arc<RwLock<Arc<Core>>>,
//     pub config: StartupConfig,
//     pub full_config: crate::config::Config,
//     pub failed_attempt_tracker: Arc<crate::core::security_v2::FailedAttemptTracker>,
//     pub audit_logger: Arc<crate::core::security_v2::AuditLogger>,
//     pub jwt_secret: String,
//     pub boot_report: Arc<RwLock<Option<crate::startup::BootReport>>>,
// }
//
// The social_graph, knowledge_ark, etc. are accessed via state.core.read().await.social_stack.social_graph
// No need to duplicate them in AppState.

// ═══════════════════════════════════════════════════════════════════════════
// CHANGE 5: src/web.rs — Router: add all new routes
// ═══════════════════════════════════════════════════════════════════════════
//
// Add to the api_routes Router:
//
// // Identity (fixed)
// .route("/v2/profile/unlock", post(IdentityController::unlock_profile))
// .route("/v2/profile/lock",   post(IdentityController::lock_profile))
// .route("/v2/security/audit", get(IdentityController::get_audit_log))
//
// // Peers (Wave 5)
// .route("/v2/peers",                   get(PeerController::list))
// .route("/v2/peers/nearby",            get(PeerController::nearby))
// .route("/v2/peers/connect",           post(PeerController::request_connect))
// .route("/v2/peers/requests",          get(PeerController::list_requests))
// .route("/v2/peers/requests/respond",  post(PeerController::respond_to_request))
// .route("/v2/peers/:id",               delete(PeerController::disconnect))
// .route("/v2/peers/:id/block",         post(PeerController::block))
// .route("/v2/peers/:id/profile",       get(PeerController::get_profile))
// .route("/v2/peers/ping",              post(PeerController::ping))
//
// // Pairing (Wave 4)
// .route("/v2/pair/generate",     post(PairingController::generate_code))
// .route("/v2/pair/respond",      post(PairingController::respond_to_code))
// .route("/v2/pair/complete",     post(PairingController::complete_pairing))
// .route("/v2/pair/contact-card", get(PairingController::get_contact_card))
// .route("/v2/pair/scan",         post(PairingController::scan_contact_card))
// .route("/v2/pair/rendezvous",   get(PairingController::get_rendezvous_address))
//
// // Knowledge Ark (Wave 4)
// .route("/v2/ark/collections",        get(ArkController::list_collections))
// .route("/v2/ark/collections",        post(ArkController::create_collection))
// .route("/v2/ark/entries",            post(ArkController::create_entry))
// .route("/v2/ark/entries/:cid",       get(ArkController::get_entry))
// .route("/v2/ark/entries/:cid",       delete(ArkController::delete_entry))
// .route("/v2/ark/search",             get(ArkController::search))
// .route("/v2/ark/tags/:tag",          get(ArkController::search_by_tag))
// .route("/v2/ark/stats",              get(ArkController::stats))
//
// // Ghost Channels (Wave 6)
// .route("/v2/channels",               get(ChannelController::list))
// .route("/v2/channels",               post(ChannelController::create))
// .route("/v2/channels/:id/join",      post(ChannelController::join))
// .route("/v2/channels/:id/leave",     post(ChannelController::leave))
// .route("/v2/channels/:id/publish",   post(ChannelController::publish))
// .route("/v2/channels/:id/messages",  get(ChannelController::get_messages))
//
// // Recovery (Wave 2)
// .route("/v2/recovery/setup",         post(RecoveryController::setup_shares))
// .route("/v2/recovery/my-shares",     get(RecoveryController::list_my_shares))
// .route("/v2/recovery/reconstruct",   post(RecoveryController::reconstruct_key))
//
// // Trust (Wave 2)
// .route("/v2/trust/:peer_id",         get(TrustController::get_trust))
// .route("/v2/trust/:peer_id/verify",  post(TrustController::mark_verified))
// .route("/v2/trust/:peer_id/block",   post(TrustController::block_peer))
// .route("/v2/trust",                  get(TrustController::list_peers))
//
// // Network status
// .route("/v2/network/status",         get(NetworkController::status))
// .route("/v2/network/qr-contact",     get(NetworkController::generate_contact_qr))
// .route("/v2/network/scan-qr",        post(NetworkController::scan_contact_qr))
//
// // System
// .route("/v2/system/boot-report",     get(SystemController::boot_report))
// .route("/v2/system/mesh-health",     get(SystemController::mesh_health))

// ═══════════════════════════════════════════════════════════════════════════
// CHANGE 6: src/web.rs — WebSocket handler replacement
// ═══════════════════════════════════════════════════════════════════════════
//
// Delete the existing ws_handler and handle_socket functions.
// Add at top of web.rs:
//   use crate::controllers::ws_handler::{ws_handler, handle_socket};
//
// The route is already registered:
//   .route("/api/ws", get(ws_handler))

// ═══════════════════════════════════════════════════════════════════════════
// CHANGE 7: src/core/mesh.rs — Wire S/Kademlia + Gossipsub hardening
// ═══════════════════════════════════════════════════════════════════════════
//
// Replace:
//   let gossipsub_config = GossipsubConfig::default();
//   let gossipsub = Gossipsub::new(MessageAuthenticity::Signed(local_key.clone()), gossipsub_config)...
//
// With (from skademlia.rs):
//   use crate::core::routing::skademlia::{hardened_gossipsub_config, hardened_peer_score_params, hardened_peer_score_thresholds};
//   let gossipsub = Gossipsub::new_with_subscription_filter_and_transform(
//       MessageAuthenticity::Signed(local_key.clone()),
//       hardened_gossipsub_config(),
//       Some((hardened_peer_score_params(), hardened_peer_score_thresholds())),
//       libp2p::gossipsub::AllowAllSubscriptionFilter,
//       libp2p::gossipsub::IdentityTransform,
//   ).map_err(|e| anyhow::anyhow!(e))?;
//
// Also add connection event handler:
//   SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
//       // Extract IP from endpoint
//       if let libp2p::core::ConnectedPoint::Dialer { address, .. } = endpoint {
//           if let Some(ip) = extract_ip_from_multiaddr(&address) {
//               // Try to add to diverse routing table — rejects if subnet full
//               if let Some(core) = &self.core_ref {
//                   // core.security_stack.trust_store.verify_or_pin(&peer_id.to_string(), &pub_key).await
//               }
//           }
//       }
//       // Pre-warm session
//       // core.session_warmer.mark_warmed(peer_id.to_string());
//   }

// ═══════════════════════════════════════════════════════════════════════════
// CHANGE 8: src/core/mesh.rs — Message send path: wrap with Sphinx
// ═══════════════════════════════════════════════════════════════════════════
//
// In handle_command(BroadcastMessage { message, reply }):
//
// BEFORE:
//   let data = serde_json::to_vec(&message)?;
//   self.swarm.behaviour_mut().gossipsub.publish(topic, data)?;
//
// AFTER:
//   // Use wire_protocol MessagePack instead of JSON
//   use crate::core::wire_protocol::{Frame, FrameKind};
//   let frame = Frame::encode(FrameKind::Message, &message)?;
//   let data = frame.to_bytes();
//
//   // Deduplication check
//   if self.storage_stack.message_dedup.is_duplicate(&message.id) {
//       return; // Already seen
//   }
//
//   self.swarm.behaviour_mut().gossipsub.publish(topic, data)?;
//
// In handle_command(SendMessage { recipient, message, reply }):
// (for direct Sphinx-routed private messages)
//   // Look up route from private routing layer
//   // wrap_onion / create_sphinx_packet
//   // apply_mixing_delay
//   // send via request_response

// ═══════════════════════════════════════════════════════════════════════════
// CHANGE 9: src/controllers/mod.rs — export new controllers
// ═══════════════════════════════════════════════════════════════════════════
//
// Add:
// pub mod ark;
// pub mod channel;
// pub mod network;
// pub mod pairing;
// pub mod recovery;
// pub mod trust;
// pub mod ws_handler;
//
// pub use ark::ArkController;
// pub use channel::ChannelController;
// pub use network::NetworkController;
// pub use pairing::PairingController;
// pub use recovery::RecoveryController;
// pub use trust::TrustController;
// pub use ws_handler::{ws_handler, handle_socket};

// ═══════════════════════════════════════════════════════════════════════════
// CHANGE 10: src/main.rs — Use new startup sequence
// ═══════════════════════════════════════════════════════════════════════════
//
// Add at top: use ghostwire::startup::StartupSequence;
//
// In main(), before Core::new():
//   let data_dir = dirs::data_dir()
//       .unwrap_or_default().join("ghostwire");
//   let mut boot = StartupSequence::new(None);
//   boot.phase0_preflight(&data_dir)?;
//   boot.phase1_identity(args.profile.as_deref(), &data_dir).await?;
//   boot.phase2_storage(&data_dir).await?;
//   let net_result = boot.phase3_network(&net_config).await?;
//   boot.phase4_knowledge(&data_dir).await?;
//
// After Core::new():
//   boot.phase5_ready(&core_arc.get_identity_id().await, &net_result.transport_mode, 0);
//   let boot_report = boot.report(Some(core_arc.get_identity_id().await), &net_result);
//   // Store in AppState

// ═══════════════════════════════════════════════════════════════════════════
// FILES TO DELETE from the original codebase
// ═══════════════════════════════════════════════════════════════════════════
//
// git rm ghostwire/src/controllers/profiles_debug.rs
// git rm ghostwire/src/core/reputation.rs       ← replace with Wave 2 version
// git rm ghostwire/src/core/armor.rs            ← DONE: TrafficArmor+MorphingConfig removed, DeviceTier+ErasureArmor remain
// git rm ghostwire/src/core/updater.rs          ← replace with updater_fixed.rs
//
// In controllers/mod.rs remove: pub mod profiles_debug;
// In core/mod.rs remove: pub mod reputation; (re-add pointing to new file)
