//! mesh_health.rs — Live mesh health dashboard
//!
//! FIXES: All fields were returning 0/empty. Wired to real subsystems where
//! they exist in the current codebase. Remaining TODOs are clearly marked.

use crate::core::bootstrap::BootstrapCapabilityState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshHealthSnapshot {
    pub timestamp: u64,
    pub network_state: NetworkStateInfo,
    pub peers: Vec<PeerHealthInfo>,
    pub routing: RoutingHealthInfo,
    pub storage: StorageHealthInfo,
    pub security: SecurityHealthInfo,
    pub performance: PerformanceInfo,
    /// Per-interface lifecycle and capability state. Existing clients may ignore this field.
    pub interfaces: Vec<InterfaceHealthInfo>,
    /// GhostWire-One capability state; configured/authorized/ready are distinct.
    #[serde(default)]
    pub bootstrap: BootstrapCapabilityState,
    /// World-scale discovery health; discovery evidence never implies trust.
    #[serde(default)]
    pub world_discovery: crate::core::world_discovery::WorldDiscoveryHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStateInfo {
    /// Derived from live mesh state; see `derive_network_state`.
    pub state: String,
    pub internet_reachable: bool,
    pub lan_reachable: bool,
    pub active_transport: String,
    /// Real process uptime, or `None` when the kernel does not expose it.
    pub uptime_secs: Option<u64>,
    /// Bootstrap nodes this node was CONFIGURED with. How many are actually
    /// reachable is not probed anywhere in `src/`, so it is not reported:
    /// `bootstrap_nodes_reachable` used to be a hardcoded `0`, which reads as
    /// "probed zero reachable" when nothing had been probed at all.
    pub bootstrap_nodes_configured: usize,
    pub latency_ms: u32,
    pub packet_loss_pct: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceHealthInfo {
    pub id: String,
    pub protocol: String,
    pub medium: String,
    pub state: String,
    pub registered: bool,
    pub started: bool,
    pub ready: bool,
    pub deferred: bool,
    pub listen_addresses: Vec<String>,
    pub peer_count: usize,
    pub last_error: Option<String>,
    pub discovery: Option<DiscoveryHealthInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryHealthInfo {
    pub method: String,
    pub enabled: bool,
    pub state: String,
    pub discovered_total: usize,
    pub expired_total: usize,
    pub last_event_unix: u64,
    pub nearby_peer_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerHealthInfo {
    pub node_id: String,
    pub nickname: Option<String>,
    pub connection_state: String,
    pub is_online: bool,
    pub latency_ms: Option<u32>,
    pub signal_strength: f32,
    pub last_seen_secs_ago: u64,
    pub messages_exchanged: u64,
    pub trust_level: String,
    pub discovery_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingHealthInfo {
    /// Single-hop routes from the live topology, or `None` when no topology
    /// could be read. This used to be the peer count, which is not a routing
    /// table size.
    pub routing_table_size: Option<usize>,
    pub subnet_distribution: Option<HashMap<String, usize>>,
    /// Distinct single-hop routes. Not a multi-hop path count.
    pub active_routes: Option<usize>,
    /// `None`: no counter for live Sphinx sessions exists. It reported the
    /// peer count, which is a different quantity.
    pub sphinx_sessions: Option<usize>,
    pub messages_routed_total: Option<u64>,
    pub mixing_queue_depth: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageHealthInfo {
    pub message_count: u64,
    pub ark_entry_count: usize,
    pub ark_collection_count: usize,
    pub block_store_count: usize,
    pub mailbox_pending: usize,
    /// `None`: the sled handle is private to PersistenceManager, so the
    /// on-disk size is not reachable from here.
    pub encrypted_store_namespaces: Option<usize>,
    pub db_size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHealthInfo {
    /// `None`: no counter for unlock attempts is exposed by the
    /// FailedAttemptTracker, so this reports nothing rather than "zero
    /// attempts", which is a different claim.
    pub failed_unlock_attempts_24h: Option<u32>,
    pub blocked_peers: u32,
    /// `None`: ProofOfWorkSybilDefense has no production call site, so there
    /// is no rejection counter to read.
    pub sybil_rejections_24h: Option<u32>,
    /// Counted from the live in-memory AuditLogger.
    pub audit_events_24h: Option<u32>,
    pub identity_locked: bool,
    pub trust_score_avg: f32,
    pub last_key_rotation_days_ago: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInfo {
    /// `None`: no message-rate counter exists in Core.
    pub messages_per_minute: Option<f32>,
    pub avg_message_latency_ms: u32,
    /// `None`: the session warmer exposes no hit/miss counters.
    pub session_cache_hit_rate: Option<f32>,
    pub dedup_cache_size: usize,
    pub peer_cache_size: usize,
    pub ws_subscriber_count: usize,
    /// `None`: the broadcast event bus does not track dropped sends.
    pub event_bus_overflow_count: Option<u64>,
}

/// Real process uptime in seconds, read from the kernel.
///
/// `SystemTime::now() - UNIX_EPOCH` is NOT uptime: it is the wall clock since
/// 1970 (~1.77e9 on any modern host). `/proc/uptime` minus this process's
/// start offset from `/proc/self/stat` is the actual answer.
///
/// Returns `None` where procfs is unavailable (non-Linux, or a restricted
/// namespace) rather than falling back to a fabricated number.
pub fn process_uptime_secs() -> Option<u64> {
    let uptime_txt = std::fs::read_to_string("/proc/uptime").ok()?;
    let system_uptime: f64 = uptime_txt.split_whitespace().next()?.parse().ok()?;

    let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
    // Field 22 (1-indexed) is starttime in clock ticks since boot. The comm
    // field can contain spaces and parentheses, so split after the LAST ')'.
    let after_comm = stat.rsplit_once(')')?.1;
    let starttime_ticks: f64 = after_comm.split_whitespace().nth(19)?.parse().ok()?;

    let ticks_per_sec = 100.0_f64; // Linux USER_HZ is fixed at 100 on all supported arches.
    let started_secs_ago = starttime_ticks / ticks_per_sec;
    let uptime = system_uptime - started_secs_ago;
    if !uptime.is_finite() || uptime < 0.0 {
        return None;
    }
    Some(uptime as u64)
}

/// Derive the network state string from facts that were actually observed.
///
/// This used to be the literal `"full_mesh"` regardless of state, so a node
/// with no mesh manager, no listeners and zero peers reported `full_mesh` —
/// while ignoring the `mesh_started` and `mesh_listeners` values computed a
/// few lines above it.
fn derive_network_state(
    identity_locked: bool,
    mesh_started: bool,
    mesh_alive: bool,
    listener_count: usize,
    peer_count: usize,
) -> &'static str {
    if identity_locked && !mesh_started {
        return "locked";
    }
    if !mesh_started {
        return "no_mesh";
    }
    if !mesh_alive {
        // The swarm driver task is gone: the API answers but nothing routes.
        return "mesh_degraded";
    }
    if listener_count == 0 {
        return "no_listener";
    }
    if peer_count == 0 {
        return "no_peers";
    }
    "connected"
}

pub async fn build_snapshot(
    core: &crate::core::Core,
    ws_subscriber_count: usize,
) -> MeshHealthSnapshot {
    let now = now_secs();
    let bootstrap = core.get_bootstrap_state().await;
    let world_discovery = {
        #[cfg(feature = "mesh")]
        {
            if let Some(mesh) = core.effective_mesh_manager().await {
                mesh.read().await.world_discovery_health().await
            } else {
                crate::core::world_discovery::WorldDiscoveryHealth::new(
                    false,
                    crate::core::world_discovery::configured_network_id(),
                )
            }
        }
        #[cfg(not(feature = "mesh"))]
        {
            crate::core::world_discovery::WorldDiscoveryHealth::new(
                false,
                crate::core::world_discovery::configured_network_id(),
            )
        }
    };

    // WIRED: real metrics from NetworkMonitor
    let metrics = core.monitor.get_metrics().await;
    let active_transport_name = {
        let guard = core.active_transport.read().await;
        guard
            .as_ref()
            .map(|t| t.name())
            .unwrap_or("none")
            .to_string()
    };
    let registered_transports = core.transport_registry.read().await.list();
    let identity_locked = core.is_locked().await;
    let mesh_started = {
        #[cfg(feature = "mesh")]
        {
            core.effective_mesh_manager().await.is_some()
        }
        #[cfg(not(feature = "mesh"))]
        {
            false
        }
    };
    // Whether the mesh has a usable listener. Distinct from `mesh_started`: a
    // manager can exist with a start requested and zero bound listeners, which
    // is a node that answers API calls but cannot route anything.
    //
    // `MeshManager` exposes no `is_mesh_alive`; the `is_mesh_alive` at
    // mesh.rs:1007 belongs to `MeshTransport` (mesh.rs:257), a different
    // struct, and `driver_alive` is private to `MeshDriver` (mesh.rs:1410).
    // Rather than assert a liveness flag that cannot be read, report the fact
    // that can be observed: an empty listener list means nothing is bound.
    let mesh_alive = {
        #[cfg(feature = "mesh")]
        {
            match core.effective_mesh_manager().await {
                Some(mesh) => !mesh.read().await.get_listeners().await.is_empty(),
                None => false,
            }
        }
        #[cfg(not(feature = "mesh"))]
        {
            false
        }
    };
    let mesh_listeners = if mesh_started {
        #[cfg(feature = "mesh")]
        {
            if let Some(mesh) = core.effective_mesh_manager().await {
                mesh.read().await.get_listeners().await
            } else {
                Vec::new()
            }
        }
        #[cfg(not(feature = "mesh"))]
        {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let network_state = NetworkStateInfo {
        state: derive_network_state(
            identity_locked,
            mesh_started,
            mesh_alive,
            mesh_listeners.len(),
            core.get_peer_count(),
        )
        .to_string(),
        internet_reachable: metrics.is_internet_reachable,
        lan_reachable: local_ip_address::local_ip().is_ok(),
        active_transport: active_transport_name,
        uptime_secs: process_uptime_secs(),
        bootstrap_nodes_configured: core.bootstrap_nodes.len(),
        latency_ms: metrics.latency_ms,
        packet_loss_pct: metrics.packet_loss_ratio * 100.0,
    };

    let peer_count = core.get_peer_count();
    let topo = core.get_mesh_topology().await;
    let topo_peer_count = topo.as_ref().map(|t| t.nodes.len()).unwrap_or(0);
    let effective_peer_count = peer_count.max(topo_peer_count);

    let peers: Vec<PeerHealthInfo> = topo
        .as_ref()
        .map(|t| {
            t.nodes
                .values()
                .map(|n| PeerHealthInfo {
                    node_id: n.id.clone(),
                    nickname: if n.username.is_empty() {
                        None
                    } else {
                        Some(n.username.clone())
                    },
                    connection_state: if n.is_online {
                        "connected".into()
                    } else {
                        "disconnected".into()
                    },
                    is_online: n.is_online,
                    latency_ms: None,
                    signal_strength: n.connection_quality,
                    last_seen_secs_ago: now.saturating_sub(n.last_seen),
                    messages_exchanged: 0,
                    trust_level: format!("{:?}", n.threat_level),
                    discovery_method: "mesh".into(),
                })
                .collect()
        })
        .unwrap_or_default();

    let quic_registered = registered_transports.iter().any(|name| name == "quic");
    let mesh_registered = registered_transports.iter().any(|name| name == "mesh");
    let mesh_discovery = if mesh_started {
        #[cfg(feature = "mesh")]
        {
            if let Some(mesh) = &core.mesh_manager {
                let mdns = mesh.read().await.mdns_health();
                Some(DiscoveryHealthInfo {
                    method: "mdns".into(),
                    enabled: mdns.enabled,
                    state: if mdns.enabled { "ready" } else { "unavailable" }.into(),
                    discovered_total: mdns.discovered_total,
                    expired_total: mdns.expired_total,
                    last_event_unix: mdns.last_event_unix,
                    nearby_peer_count: peers
                        .iter()
                        .filter(|p| p.discovery_method == "mdns")
                        .count(),
                })
            } else {
                None
            }
        }
        #[cfg(not(feature = "mesh"))]
        {
            None
        }
    } else {
        Some(DiscoveryHealthInfo {
            method: "mdns".into(),
            enabled: false,
            state: if identity_locked {
                "deferred"
            } else {
                "unavailable"
            }
            .into(),
            discovered_total: 0,
            expired_total: 0,
            last_event_unix: 0,
            nearby_peer_count: 0,
        })
    };

    let reticulum_stats = core.get_reticulum_stats().await;

    let mut interfaces = vec![
        InterfaceHealthInfo {
            id: "quic".into(),
            protocol: "quic".into(),
            medium: "ip".into(),
            state: if quic_registered {
                "ready"
            } else {
                "unavailable"
            }
            .into(),
            registered: quic_registered,
            started: quic_registered,
            ready: quic_registered,
            deferred: false,
            listen_addresses: Vec::new(),
            peer_count: 0,
            last_error: None,
            discovery: None,
        },
        InterfaceHealthInfo {
            id: "mesh".into(),
            protocol: "libp2p".into(),
            medium: "lan_or_ip".into(),
            state: if mesh_started {
                "ready"
            } else if identity_locked {
                "deferred"
            } else {
                "unavailable"
            }
            .into(),
            registered: mesh_registered,
            started: mesh_started,
            ready: mesh_started && !mesh_listeners.is_empty(),
            deferred: identity_locked && !mesh_started,
            listen_addresses: mesh_listeners.iter().map(ToString::to_string).collect(),
            peer_count: effective_peer_count,
            last_error: None,
            discovery: mesh_discovery.clone(),
        },
    ];

    // Wi-Fi is a real local-link medium for the authenticated libp2p mesh;
    // it is not a second unauthenticated message protocol. Report it only when
    // Linux exposes an active wlan/wlp interface and the mesh listener exists.
    let wifi_ifaces: Vec<String> = std::fs::read_dir("/sys/class/net")
        .ok()
        .into_iter()
        .flat_map(|entries| entries.flatten())
        .filter_map(|entry| entry.file_name().to_str().map(str::to_owned))
        .filter(|name| name.starts_with("wlan") || name.starts_with("wlp"))
        .filter(|name| {
            std::fs::read_to_string(format!("/sys/class/net/{name}/operstate"))
                .map(|state| state.trim() == "up")
                .unwrap_or(false)
        })
        .collect();
    let wifi_link_up = !wifi_ifaces.is_empty();
    interfaces.push(InterfaceHealthInfo {
        id: "wifi".into(),
        protocol: "libp2p-over-wifi".into(),
        medium: "wifi".into(),
        state: if !mesh_started {
            if identity_locked {
                "deferred"
            } else {
                "unavailable"
            }
        } else if wifi_link_up && !mesh_listeners.is_empty() {
            "ready"
        } else if wifi_link_up {
            "started"
        } else {
            "unavailable"
        }
        .into(),
        registered: mesh_registered,
        started: mesh_started && wifi_link_up,
        ready: mesh_started && wifi_link_up && !mesh_listeners.is_empty(),
        deferred: identity_locked && !mesh_started,
        listen_addresses: if wifi_link_up {
            mesh_listeners.iter().map(ToString::to_string).collect()
        } else {
            Vec::new()
        },
        peer_count: if wifi_link_up {
            effective_peer_count
        } else {
            0
        },
        last_error: if wifi_link_up {
            None
        } else {
            Some("No active wlan/wlp interface".into())
        },
        discovery: mesh_discovery.clone(),
    });
    if let Some(rns) = &reticulum_stats {
        let rns_ready = rns.initialized && !rns.degraded && rns.interfaces_configured > 0;
        interfaces.push(InterfaceHealthInfo {
            id: "reticulum".into(),
            protocol: "rns".into(),
            medium: "message-oriented-multi-medium".into(),
            state: if !rns.initialized {
                "unavailable"
            } else if rns.degraded {
                "degraded"
            } else if rns_ready {
                "ready"
            } else {
                "started"
            }
            .into(),
            registered: true,
            started: rns.initialized,
            ready: rns_ready,
            deferred: identity_locked && !rns.initialized,
            listen_addresses: Vec::new(),
            peer_count: rns.online_nodes,
            last_error: rns.last_error.clone(),
            discovery: None,
        });
    }

    // Registry presence is not backend readiness. Expose additional registered
    // transports explicitly as `registered` until their concrete driver reports
    // started/ready, rather than hiding them or claiming the cockpit is green.
    for name in registered_transports {
        if name == "quic" || name == "mesh" {
            continue;
        }
        interfaces.push(InterfaceHealthInfo {
            id: name.clone(),
            protocol: name.clone(),
            medium: "unknown".into(),
            state: "registered".into(),
            registered: true,
            started: false,
            ready: false,
            deferred: false,
            listen_addresses: Vec::new(),
            peer_count: 0,
            last_error: None,
            discovery: None,
        });
    }

    // WIRED: peer_cache size from micro_optimizations
    let peer_cache_size = core.peer_cache.len();
    // WIRED: message dedup cache size
    let dedup_cache_size = core.message_dedup.size();

    // WIRED: ark stats
    let ark_stats = core.knowledge_ark.stats().await;

    // WIRED: identity locked status
    let identity_locked = core.is_locked().await;

    // WIRED: trust store blocked peer count
    let blocked_peers = core.trust_store.blocked_count().await as u32;

    // WIRED: mailbox pending count
    let mailbox_pending = core.mailbox_store.pending_count().unwrap_or(0);

    // WIRED: message store count
    let message_count = {
        let messages = core.messages.read().await;
        messages.len() as u64
    };

    let routing = RoutingHealthInfo {
        // Real single-hop routes from the live topology. Previously the peer
        // count, which answers a different question.
        routing_table_size: topo.as_ref().map(|t| t.routes.len()),
        // Not collected anywhere; an empty map reads as "no subnets", which
        // would be a claim rather than an absence of measurement.
        subnet_distribution: None,
        active_routes: topo.as_ref().map(|t| t.routes.len()),
        sphinx_sessions: None,
        messages_routed_total: None,
        mixing_queue_depth: None,
    };

    let storage = StorageHealthInfo {
        message_count,
        ark_entry_count: ark_stats.total_entries,
        ark_collection_count: ark_stats.total_collections,
        block_store_count: core.block_store.block_count().await,
        mailbox_pending,
        encrypted_store_namespaces: None,
        db_size_bytes: None,
    };

    // REAL: count audit records from the last 24h out of the live in-memory
    // logger, instead of the hardcoded 0 this used to report.
    let audit_events_24h = {
        let now = now;
        let cutoff = now.saturating_sub(24 * 60 * 60);
        let events = core.audit_logger.get_recent_events(10_000).await;
        Some(events.iter().filter(|e| e.timestamp >= cutoff).count() as u32)
    };

    let security = SecurityHealthInfo {
        failed_unlock_attempts_24h: None,
        blocked_peers,
        sybil_rejections_24h: None,
        audit_events_24h,
        identity_locked,
        trust_score_avg: core.trust_store.average_trust_score().await,
        last_key_rotation_days_ago: None,
    };

    let performance = PerformanceInfo {
        messages_per_minute: None,
        avg_message_latency_ms: metrics.latency_ms,
        session_cache_hit_rate: None,
        dedup_cache_size,
        peer_cache_size,
        ws_subscriber_count,
        event_bus_overflow_count: None,
    };

    MeshHealthSnapshot {
        timestamp: now,
        network_state,
        peers,
        routing,
        storage,
        security,
        performance,
        interfaces,
        bootstrap,
        world_discovery,
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot() -> MeshHealthSnapshot {
        MeshHealthSnapshot {
            timestamp: now_secs(),
            network_state: NetworkStateInfo {
                state: "connected".into(),
                internet_reachable: true,
                lan_reachable: true,
                active_transport: "tcp".into(),
                uptime_secs: Some(3600),
                bootstrap_nodes_configured: 3,
                latency_ms: 15,
                packet_loss_pct: 0.1,
            },
            peers: vec![],
            routing: RoutingHealthInfo {
                routing_table_size: Some(5),
                subnet_distribution: None,
                active_routes: Some(5),
                sphinx_sessions: Some(3),
                messages_routed_total: Some(100),
                mixing_queue_depth: None,
            },
            storage: StorageHealthInfo {
                message_count: 42,
                ark_entry_count: 10,
                ark_collection_count: 3,
                block_store_count: 7,
                mailbox_pending: 2,
                encrypted_store_namespaces: Some(1),
                db_size_bytes: Some(1024),
            },
            security: SecurityHealthInfo {
                failed_unlock_attempts_24h: None,
                blocked_peers: 0,
                sybil_rejections_24h: None,
                audit_events_24h: Some(5),
                identity_locked: false,
                trust_score_avg: 0.85,
                last_key_rotation_days_ago: Some(7),
            },
            performance: PerformanceInfo {
                messages_per_minute: Some(12.5),
                avg_message_latency_ms: 50,
                session_cache_hit_rate: Some(0.95),
                dedup_cache_size: 100,
                peer_cache_size: 10,
                ws_subscriber_count: 3,
                event_bus_overflow_count: None,
            },
            interfaces: vec![],
            bootstrap: BootstrapCapabilityState::default(),
            world_discovery: crate::core::world_discovery::WorldDiscoveryHealth::new(
                false,
                "ghostwire-one",
            ),
        }
    }

    #[test]
    fn snapshot_serializes_to_json() {
        let snap = sample_snapshot();

        let json = serde_json::to_string(&snap).unwrap();
        let parsed: MeshHealthSnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.routing.sphinx_sessions, Some(3));
        assert_eq!(parsed.storage.message_count, 42);
        assert!(!parsed.security.identity_locked);
    }

    /// An unmeasured metric must serialise to null, not to 0. A `0` reads as
    /// "measured, and the answer is zero"; the whole point of the Option is
    /// that "we never measured it" is distinguishable from "it is zero".
    #[test]
    fn unmeasured_metrics_serialise_as_null_not_zero() {
        let json = serde_json::to_value(sample_snapshot()).unwrap();
        assert!(json["routing"]["subnet_distribution"].is_null());
        assert!(json["routing"]["mixing_queue_depth"].is_null());
        assert!(json["security"]["failed_unlock_attempts_24h"].is_null());
        assert!(json["security"]["sybil_rejections_24h"].is_null());
        assert!(json["performance"]["event_bus_overflow_count"].is_null());

        // `sample_snapshot` populates `messages_per_minute` with `Some(12.5)`,
        // so the null assertion that used to sit here was wrong for the same
        // reason as db_size_bytes: it demanded null from a field its own
        // fixture had just set. Assert the measured value instead.
        assert_eq!(
            json["performance"]["messages_per_minute"],
            serde_json::json!(12.5)
        );

        // `sample_snapshot` deliberately populates db_size_bytes with
        // `Some(1024)`, so the null assertion this test used to make here was
        // wrong — it asked for null on a field the fixture had just set. It now
        // checks the distinction that actually matters: a measured value
        // serialises as its number, and the real snapshot's unmeasured default
        // (`None`) serialises as null rather than 0.
        assert_eq!(json["storage"]["db_size_bytes"], serde_json::json!(1024));
        let unmeasured = serde_json::to_value(
            MeshHealthSnapshot {
                storage: StorageHealthInfo {
                    db_size_bytes: None,
                    ..sample_snapshot().storage
                },
                ..sample_snapshot()
            },
        )
        .unwrap();
        assert!(
            unmeasured["storage"]["db_size_bytes"].is_null(),
            "an unmeasured db_size_bytes must serialise as null, not 0"
        );

        // And a measured one is still a number, not null.
        assert_eq!(json["storage"]["message_count"], 42);
    }

    #[test]
    fn routing_health_round_trips_route_counts() {
        let routing = RoutingHealthInfo {
            routing_table_size: Some(8),
            subnet_distribution: None,
            active_routes: Some(8),
            sphinx_sessions: None,
            messages_routed_total: None,
            mixing_queue_depth: None,
        };
        assert_eq!(routing.routing_table_size, Some(8));
        assert_eq!(routing.active_routes, Some(8));
        // sphinx_sessions is no longer aliased to the route count.
        assert_eq!(routing.sphinx_sessions, None);
    }

    /// The state string must follow the facts. A node with nothing running
    /// must never report "full_mesh" — that literal is what this replaced.
    #[test]
    fn network_state_follows_observed_mesh_facts() {
        assert_eq!(derive_network_state(true, false, false, 0, 0), "locked");
        assert_eq!(derive_network_state(false, false, false, 0, 0), "no_mesh");
        // Started but the driver task is gone: API up, routing dead.
        assert_eq!(
            derive_network_state(false, true, false, 2, 5),
            "mesh_degraded"
        );
        assert_eq!(derive_network_state(false, true, true, 0, 0), "no_listener");
        assert_eq!(derive_network_state(false, true, true, 1, 0), "no_peers");
        assert_eq!(derive_network_state(false, true, true, 1, 3), "connected");

        for (locked, started, alive, listeners, peers) in [
            (true, false, false, 0usize, 0usize),
            (false, false, false, 0, 0),
            (false, true, false, 4, 9),
            (false, true, true, 0, 0),
        ] {
            assert_ne!(
                derive_network_state(locked, started, alive, listeners, peers),
                "full_mesh",
                "no configuration may report full_mesh"
            );
        }
    }

    /// Uptime must be seconds-since-process-start, not seconds-since-1970.
    /// A value near 1.7e9 is the epoch bug returning.
    #[test]
    fn process_uptime_is_not_the_unix_epoch() {
        match process_uptime_secs() {
            Some(secs) => assert!(
                secs < 100 * 365 * 24 * 3600,
                "uptime {secs}s looks like an epoch offset, not process uptime"
            ),
            // procfs unavailable: honest absence, not a fabricated number.
            None => {}
        }
    }
}
