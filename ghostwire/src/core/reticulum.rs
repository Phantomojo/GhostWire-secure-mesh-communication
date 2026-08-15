//! A narrow, fail-closed bridge to the official Reticulum (RNS) runtime.
//!
//! GhostWire does **not** reimplement Reticulum's packet format, identity
//! model, routing, receipt semantics, or physical interfaces. The previous
//! in-process "Reticulum" code was a local simulation and therefore could not
//! interoperate with RNS nodes. This manager instead launches the separately
//! installed `scripts/reticulum_bridge.py`, which owns one official RNS
//! instance. The bridge speaks a bounded JSON-lines control protocol over a
//! private child-process pipe.
//!
//! The process boundary is intentional. It prevents the RNS identity namespace,
//! interface configuration, and transport routing state from being silently
//! conflated with GhostWire's libp2p identity and policy state. The `RNS` Python
//! package is an optional external runtime, not a vendored Rust dependency.
//!
//! ## Security boundary
//!
//! * RNS configuration and its bridge identity live in a dedicated directory.
//! * The bridge accepts only bounded opaque payloads (currently a single RNS
//!   encrypted-packet MDU); it never receives GhostWire private keys.
//! * A manager is created only after the official RNS bridge reports a valid
//!   destination. Missing Python, RNS, bridge script, or malformed startup
//!   output fails closed.
//! * RNS transport/interface configuration is caller-owned. GhostWire may
//!   select or reject a carrier based on privacy policy, but it must not claim
//!   that an RNS interface has a property that RNS has not established.

use crate::core::operating_profile::{DeviceRuntimeState, OperatingProfile};
use crate::core::rns_hybrid::{
    plan_rns_packet_submission, RnsHybridError, RnsPacketSubmissionPlan,
};
use crate::core::transport_handoff::ProtectedEnvelopeId;
use crate::core::transport_policy::{DeliveryRequirement, InterfaceEligibility};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, SyncSender, TrySendError};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// The manager accepts only an RNS encrypted packet MDU. Larger GhostWire
/// envelopes must wait for an explicitly designed RNS `Link`/`Resource` path;
/// they must never be silently truncated or fragmented into unauthenticated
/// ad-hoc packets.
pub const MAX_RNS_BRIDGE_PAYLOAD_BYTES: usize = 383;
const BRIDGE_STARTUP_TIMEOUT: Duration = Duration::from_secs(10);
const BRIDGE_EVENT_QUEUE_CAPACITY: usize = 128;

/// Stable configuration error returned when the optional external runtime is
/// absent or cannot prove its RNS destination at startup.
pub const RETICULUM_BRIDGE_UNAVAILABLE: &str =
    "RETICULUM_BRIDGE_UNAVAILABLE: the official RNS bridge could not be started";

/// Compatibility alias retained so callers built against earlier hardening
/// commits still fail closed. New callers must use `RETICULUM_BRIDGE_UNAVAILABLE`.
pub const RETICULUM_UNSUPPORTED: &str = RETICULUM_BRIDGE_UNAVAILABLE;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReticulumNode {
    /// RNS destination hash, never a GhostWire/libp2p identity.
    pub id: String,
    pub last_seen: u64,
    pub is_online: bool,
}

/// RNS does not expose a complete network topology to an application. This
/// type remains for API compatibility only; it represents locally observed,
/// explicitly supplied bridge information rather than an inferred global map.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReticulumTopology {
    pub nodes: HashMap<String, ReticulumNode>,
    pub routes: HashMap<String, Vec<String>>,
    pub local_node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReticulumStats {
    /// RNS does not provide a global node count to this bridge.
    pub total_nodes: usize,
    /// RNS does not provide a global online-node count to this bridge.
    pub online_nodes: usize,
    /// Local RNS `ghostwire.bridge` destination hash.
    pub local_node_id: String,
    /// RNS hides its internal next-hop topology by design.
    pub routes_count: usize,
    /// GhostWire does not claim onion routes from an RNS bridge.
    pub onion_routes_count: usize,
    /// Count of receipts that RNS accepted for local submission, not a proof of
    /// application-level receipt or message display.
    pub messages_relayed: u64,
    pub encryption_errors: u64,
}

#[derive(Debug)]
struct BridgeHandles {
    child: Child,
    stdin: ChildStdin,
}

/// Lifecycle manager for one external official-RNS bridge process.
///
/// Incoming bridge events are retained only for the forthcoming unified ingress
/// pipeline. They are intentionally not delivered to GhostWire application
/// state yet, because bypassing CRL, trust, replay, causal-ordering, and message
/// authentication checks would recreate the fragmented-ingress vulnerability.
pub struct ReticulumManager {
    destination: String,
    rns_version: String,
    handles: Mutex<BridgeHandles>,
    events: Mutex<Receiver<Value>>,
    accepted_messages: Mutex<u64>,
}

impl ReticulumManager {
    /// Launch the official RNS bridge and require a valid RNS destination before
    /// considering this adapter available.
    pub async fn new() -> Result<Self> {
        let bridge_path = bridge_path()?;
        let python = env::var("GHOSTWIRE_RNS_PYTHON").unwrap_or_else(|_| "python3".to_string());
        let (config_dir, identity_path) = bridge_paths()?;

        fs::create_dir_all(&config_dir)
            .with_context(|| "could not create dedicated Reticulum configuration directory")?;
        if let Some(parent) = identity_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| "could not create dedicated Reticulum identity directory")?;
        }

        let mut child = Command::new(python)
            .arg(bridge_path)
            .arg("--config-dir")
            .arg(&config_dir)
            .arg("--identity-path")
            .arg(&identity_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| anyhow!(RETICULUM_BRIDGE_UNAVAILABLE))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow!(RETICULUM_BRIDGE_UNAVAILABLE))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!(RETICULUM_BRIDGE_UNAVAILABLE))?;

        let (event_tx, event_rx) = mpsc::sync_channel(BRIDGE_EVENT_QUEUE_CAPACITY);
        spawn_bridge_reader(stdout, event_tx);

        let ready = event_rx
            .recv_timeout(BRIDGE_STARTUP_TIMEOUT)
            .map_err(|_| anyhow!(RETICULUM_BRIDGE_UNAVAILABLE))?;
        let (destination, rns_version) = parse_ready_event(&ready)?;

        debug!(
            rns_destination = %destination,
            rns_version = %rns_version,
            "official Reticulum bridge established"
        );

        Ok(Self {
            destination,
            rns_version,
            handles: Mutex::new(BridgeHandles { child, stdin }),
            events: Mutex::new(event_rx),
            accepted_messages: Mutex::new(0),
        })
    }

    #[must_use]
    pub fn destination(&self) -> &str {
        &self.destination
    }

    #[must_use]
    pub fn rns_version(&self) -> &str {
        &self.rns_version
    }

    #[must_use]
    pub fn get_stats(&self) -> ReticulumStats {
        let accepted_messages = self.accepted_messages.lock().map_or(0, |count| *count);
        ReticulumStats {
            total_nodes: 0,
            online_nodes: 0,
            local_node_id: self.destination.clone(),
            routes_count: 0,
            onion_routes_count: 0,
            messages_relayed: accepted_messages,
            encryption_errors: 0,
        }
    }

    /// Submit a policy-checked, already protected GhostWire envelope to an
    /// explicitly known RNS destination. The successful return value means only
    /// that this local manager accepted the packet for bridge submission; it is
    /// not an RNS path, remote receipt, or application-delivery acknowledgement.
    ///
    /// The caller must keep `requirement` immutable from operation creation.
    /// This method accepts no `Message` and cannot serialize application
    /// plaintext for RNS transport.
    pub fn submit_policy_checked_packet(
        &self,
        profile: OperatingProfile,
        runtime_state: DeviceRuntimeState,
        interface: InterfaceEligibility,
        envelope_id: ProtectedEnvelopeId,
        destination: &str,
        requirement: DeliveryRequirement,
        protected_payload: &[u8],
    ) -> Result<RnsPacketSubmissionPlan> {
        let plan = plan_rns_packet_submission(
            profile,
            runtime_state,
            interface,
            envelope_id,
            destination,
            requirement,
            protected_payload,
        )
        .map_err(|error: RnsHybridError| anyhow!(error.to_string()))?;

        self.submit_opaque_packet(&plan.rns_destination, protected_payload)?;
        Ok(plan)
    }

    /// Raw local bridge write. It is crate-private so application dispatch must
    /// pass the hybrid policy/fragmentation/operating-profile checks above.
    fn submit_opaque_packet(&self, destination: &str, payload: &[u8]) -> Result<()> {
        if destination.len() != 32 || !destination.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(anyhow!("RETICULUM_INVALID_DESTINATION"));
        }
        if payload.is_empty() || payload.len() > MAX_RNS_BRIDGE_PAYLOAD_BYTES {
            return Err(anyhow!("RETICULUM_PAYLOAD_OUT_OF_BOUNDS"));
        }

        let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, payload);
        let command = json!({
            "op": "send",
            "destination": destination,
            "payload_b64": encoded,
        });
        let line = serde_json::to_vec(&command).context("could not encode RNS bridge command")?;

        let mut handles = self
            .handles
            .lock()
            .map_err(|_| anyhow!("RETICULUM_BRIDGE_LOCK_POISONED"))?;
        handles
            .stdin
            .write_all(&line)
            .and_then(|_| handles.stdin.write_all(b"\n"))
            .and_then(|_| handles.stdin.flush())
            .map_err(|_| anyhow!("RETICULUM_BRIDGE_WRITE_FAILED"))?;

        if let Ok(mut count) = self.accepted_messages.lock() {
            *count = count.saturating_add(1);
        }
        Ok(())
    }

    /// Read one bridge event for the forthcoming shared ingress pipeline. It is
    /// crate-private so external callers cannot treat an RNS event as already
    /// authenticated application delivery.
    pub(crate) fn try_next_event(&self) -> Option<Value> {
        self.events.lock().ok()?.try_recv().ok()
    }

    pub async fn stop(&self) -> Result<()> {
        let mut handles = self
            .handles
            .lock()
            .map_err(|_| anyhow!("RETICULUM_BRIDGE_LOCK_POISONED"))?;

        if handles
            .child
            .try_wait()
            .context("could not inspect RNS bridge process")?
            .is_some()
        {
            return Ok(());
        }

        let shutdown = b"{\"op\":\"shutdown\"}\n";
        let _ = handles.stdin.write_all(shutdown);
        let _ = handles.stdin.flush();

        let deadline = Instant::now() + Duration::from_secs(3);
        while Instant::now() < deadline {
            if handles.child.try_wait()?.is_some() {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(25));
        }

        warn!("Reticulum bridge did not stop gracefully; terminating child process");
        handles
            .child
            .kill()
            .context("could not terminate RNS bridge")?;
        let _ = handles.child.wait();
        Ok(())
    }
}

fn bridge_path() -> Result<PathBuf> {
    let path = env::var_os("GHOSTWIRE_RNS_BRIDGE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scripts/reticulum_bridge.py")
        });
    if !path.is_file() {
        return Err(anyhow!(RETICULUM_BRIDGE_UNAVAILABLE));
    }
    Ok(path)
}

fn bridge_paths() -> Result<(PathBuf, PathBuf)> {
    let base = env::var_os("GHOSTWIRE_RNS_CONFIG_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            dirs::config_dir().map(|directory| directory.join("ghostwire").join("reticulum"))
        })
        .ok_or_else(|| anyhow!(RETICULUM_BRIDGE_UNAVAILABLE))?;
    Ok((base.clone(), base.join("bridge.identity")))
}

fn spawn_bridge_reader(stdout: impl std::io::Read + Send + 'static, event_tx: SyncSender<Value>) {
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            let Ok(line) = line else { break };
            let Ok(event) = serde_json::from_str::<Value>(&line) else {
                warn!("Reticulum bridge violated JSON-lines protocol; dropping record");
                continue;
            };
            match event_tx.try_send(event) {
                Ok(()) => {}
                Err(TrySendError::Full(_)) => {
                    // The local manager is deliberately bounded. Application
                    // ingress is not yet enabled, so dropping an unconsumed
                    // event is safer than allowing an external carrier to grow
                    // unbounded memory.
                    warn!("Reticulum bridge event queue full; dropping event");
                }
                Err(TrySendError::Disconnected(_)) => break,
            }
        }
    });
}

fn parse_ready_event(event: &Value) -> Result<(String, String)> {
    if event.get("event").and_then(Value::as_str) != Some("ready") {
        return Err(anyhow!(RETICULUM_BRIDGE_UNAVAILABLE));
    }
    let destination = event
        .get("destination")
        .and_then(Value::as_str)
        .filter(|value| value.len() == 32 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .ok_or_else(|| anyhow!(RETICULUM_BRIDGE_UNAVAILABLE))?;
    let rns_version = event
        .get("rns_version")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty() && value.len() <= 32)
        .ok_or_else(|| anyhow!(RETICULUM_BRIDGE_UNAVAILABLE))?;

    Ok((destination.to_owned(), rns_version.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_event_requires_a_bounded_rns_destination() {
        let valid = json!({
            "event": "ready",
            "destination": "0123456789abcdef0123456789abcdef",
            "rns_version": "1.4.2",
        });
        assert_eq!(
            parse_ready_event(&valid).unwrap(),
            (
                "0123456789abcdef0123456789abcdef".to_string(),
                "1.4.2".to_string()
            )
        );

        let invalid =
            json!({"event": "ready", "destination": "not-a-destination", "rns_version": "1.4.2"});
        assert!(parse_ready_event(&invalid).is_err());
    }

    #[test]
    fn topology_is_explicitly_empty_until_rns_exposes_observable_data() {
        let topology = ReticulumTopology::default();
        assert!(topology.nodes.is_empty());
        assert!(topology.routes.is_empty());
    }
}
