//! GhostWire-native bootstrap capability and supervised policy state.
//!
//! Bootstrap is a capability, not a permanent identity role. This module does
//! not mint trust, authorize application peers, decrypt traffic, or define
//! network membership. It only manages whether a node may offer bounded
//! discovery/DHT entry service under local policy.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_RECORD_TTL_SECS: u64 = 24 * 60 * 60;
const MAX_ADMISSION_PEERS: usize = 4096;

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrapPolicy {
    pub enabled: bool,
    pub network_id: String,
    pub max_inbound_peers: usize,
    pub record_ttl_secs: u64,
    pub require_stable_identity: bool,
    pub require_reachable_listener: bool,
}

impl Default for BootstrapPolicy {
    fn default() -> Self {
        Self {
            enabled: false,
            network_id: "ghostwire-one".to_string(),
            max_inbound_peers: 128,
            record_ttl_secs: 15 * 60,
            require_stable_identity: true,
            require_reachable_listener: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct BootstrapCapabilityState {
    pub implemented: bool,
    pub configured: bool,
    pub eligible: bool,
    pub authorized: bool,
    pub started: bool,
    pub ready: bool,
    pub verified: bool,
    pub degraded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrapEligibilityInput {
    pub stable_identity: bool,
    pub listener_started: bool,
    pub advertised_address_count: usize,
    pub clock_sane: bool,
    pub resource_budget_ok: bool,
    pub network_id_matches: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrapRecord {
    pub network_id: String,
    pub provider_peer_id: String,
    pub advertised_addresses: Vec<String>,
    pub issued_at: u64,
    pub expires_at: u64,
    pub sequence: u64,
    /// Signature bytes are supplied by the identity/control plane. The worker
    /// never treats an unsigned record as verified.
    pub signature: Vec<u8>,
}

impl BootstrapRecord {
    pub fn is_live(&self, now: u64) -> bool {
        self.expires_at > now && self.issued_at <= now
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootstrapError {
    Disabled,
    NotEligible,
    NotAuthorized,
    InvalidNetwork,
    InvalidRecord,
    AdmissionLimit,
}

pub struct BootstrapWorker {
    policy: BootstrapPolicy,
    state: BootstrapCapabilityState,
    admitted_peers: HashMap<String, u64>,
    records: HashMap<String, BootstrapRecord>,
    next_sequence: u64,
}

impl BootstrapWorker {
    pub fn new(policy: BootstrapPolicy) -> Self {
        let configured = policy.enabled;
        Self {
            policy,
            state: BootstrapCapabilityState {
                implemented: true,
                configured,
                ..Default::default()
            },
            admitted_peers: HashMap::new(),
            records: HashMap::new(),
            next_sequence: 1,
        }
    }

    pub fn policy(&self) -> &BootstrapPolicy {
        &self.policy
    }
    /// Enable or disable the capability without authorizing it. Disabling
    /// immediately stops the worker and clears volatile admissions.
    pub fn configure(&mut self, enabled: bool) {
        self.policy.enabled = enabled;
        self.state.configured = enabled;
        if !enabled {
            self.stop();
            self.state.authorized = false;
            self.state.eligible = false;
            self.state.degraded = false;
        }
    }

    pub fn state(&self) -> BootstrapCapabilityState {
        self.state.clone()
    }

    pub fn evaluate_eligibility(&mut self, input: &BootstrapEligibilityInput) -> bool {
        let eligible = (!self.policy.require_stable_identity || input.stable_identity)
            && (!self.policy.require_reachable_listener || input.listener_started)
            && input.advertised_address_count > 0
            && input.clock_sane
            && input.resource_budget_ok
            && input.network_id_matches;
        self.state.eligible = eligible;
        self.state.degraded = !eligible;
        eligible
    }

    /// Authorization is deliberately explicit and local. Eligibility alone
    /// never promotes an ordinary node into a bootstrap provider.
    pub fn authorize(&mut self, authorized: bool) {
        self.state.authorized = authorized;
    }

    pub fn start(&mut self) -> Result<(), BootstrapError> {
        if !self.policy.enabled {
            return Err(BootstrapError::Disabled);
        }
        if !self.state.eligible {
            return Err(BootstrapError::NotEligible);
        }
        if !self.state.authorized {
            return Err(BootstrapError::NotAuthorized);
        }
        self.state.started = true;
        // Starting supervision is not proof that a concrete listener was bound.
        // The transport supervisor must call mark_ready(true) after binding,
        // self-checking, and accepting GhostWire traffic.
        self.state.ready = false;
        self.state.verified = false;
        Ok(())
    }

    /// Record concrete runtime readiness only after the supervised bootstrap
    /// listener has bound and passed its self-check.
    pub fn mark_ready(&mut self, ready: bool) -> Result<(), BootstrapError> {
        if !self.state.started {
            return Err(BootstrapError::NotEligible);
        }
        self.state.ready = ready;
        if !ready {
            self.state.verified = false;
            self.records.clear();
            self.admitted_peers.clear();
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        self.state.started = false;
        self.state.ready = false;
        self.records.clear();
        self.admitted_peers.clear();
    }

    pub fn admit_peer(&mut self, peer_id: String, now: u64) -> Result<(), BootstrapError> {
        if !self.state.started || !self.state.ready {
            return Err(BootstrapError::NotEligible);
        }
        if !self.admitted_peers.contains_key(&peer_id)
            && self.admitted_peers.len() >= self.policy.max_inbound_peers.min(MAX_ADMISSION_PEERS)
        {
            return Err(BootstrapError::AdmissionLimit);
        }
        self.admitted_peers.insert(peer_id, now);
        Ok(())
    }

    /// Accept only a signed, network-scoped, bounded-lifetime record. Signature
    /// verification belongs to the identity layer and must happen before this
    /// method is called; the worker records the verified result only.
    pub fn publish_verified_record(
        &mut self,
        mut record: BootstrapRecord,
    ) -> Result<(), BootstrapError> {
        if !self.state.started || !self.state.ready {
            return Err(BootstrapError::NotEligible);
        }
        if record.network_id != self.policy.network_id
            || record.provider_peer_id.is_empty()
            || record.advertised_addresses.is_empty()
            || record.signature.is_empty()
            || record.expires_at <= record.issued_at
            || record.expires_at - record.issued_at
                > self.policy.record_ttl_secs.min(MAX_RECORD_TTL_SECS)
        {
            return Err(if record.network_id != self.policy.network_id {
                BootstrapError::InvalidNetwork
            } else {
                BootstrapError::InvalidRecord
            });
        }
        record.sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        self.records.insert(record.provider_peer_id.clone(), record);
        self.state.verified = true;
        Ok(())
    }

    pub fn expire(&mut self, now: u64) {
        self.records.retain(|_, record| record.is_live(now));
        self.admitted_peers
            .retain(|_, admitted_at| now.saturating_sub(*admitted_at) < 24 * 60 * 60);
    }

    pub fn live_records(&self, now: u64) -> Vec<BootstrapRecord> {
        self.records
            .values()
            .filter(|record| record.is_live(now))
            .cloned()
            .collect()
    }

    pub fn admitted_peer_count(&self) -> usize {
        self.admitted_peers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eligible_worker() -> BootstrapWorker {
        let mut worker = BootstrapWorker::new(BootstrapPolicy {
            enabled: true,
            ..Default::default()
        });
        assert!(worker.evaluate_eligibility(&BootstrapEligibilityInput {
            stable_identity: true,
            listener_started: true,
            advertised_address_count: 1,
            clock_sane: true,
            resource_budget_ok: true,
            network_id_matches: true,
        }));
        worker.authorize(true);
        worker.start().unwrap();
        worker.mark_ready(true).unwrap();
        worker
    }

    #[test]
    fn ordinary_node_is_not_implicitly_promoted() {
        let mut worker = BootstrapWorker::new(BootstrapPolicy {
            enabled: true,
            ..Default::default()
        });
        assert!(!worker.evaluate_eligibility(&BootstrapEligibilityInput {
            stable_identity: false,
            listener_started: true,
            advertised_address_count: 1,
            clock_sane: true,
            resource_budget_ok: true,
            network_id_matches: true,
        }));
        assert_eq!(worker.start(), Err(BootstrapError::NotEligible));
    }

    #[test]
    fn authorization_is_separate_from_eligibility() {
        let mut worker = BootstrapWorker::new(BootstrapPolicy {
            enabled: true,
            ..Default::default()
        });
        assert!(worker.evaluate_eligibility(&BootstrapEligibilityInput {
            stable_identity: true,
            listener_started: true,
            advertised_address_count: 1,
            clock_sane: true,
            resource_budget_ok: true,
            network_id_matches: true,
        }));
        assert_eq!(worker.start(), Err(BootstrapError::NotAuthorized));
    }

    #[test]
    fn start_is_not_ready_until_runtime_confirmation() {
        let mut worker = BootstrapWorker::new(BootstrapPolicy {
            enabled: true,
            ..Default::default()
        });
        worker.evaluate_eligibility(&BootstrapEligibilityInput {
            stable_identity: true,
            listener_started: true,
            advertised_address_count: 1,
            clock_sane: true,
            resource_budget_ok: true,
            network_id_matches: true,
        });
        worker.authorize(true);
        worker.start().unwrap();
        assert!(!worker.state().ready);
        worker.mark_ready(true).unwrap();
        assert!(worker.state().ready);
    }

    #[test]
    fn records_are_scoped_signed_and_expiring() {
        let mut worker = eligible_worker();
        let now = now_secs();
        let record = BootstrapRecord {
            network_id: "ghostwire-one".to_string(),
            provider_peer_id: "peer-a".to_string(),
            advertised_addresses: vec!["/ip4/127.0.0.1/tcp/4001".to_string()],
            issued_at: now,
            expires_at: now + 60,
            sequence: 0,
            signature: vec![1, 2, 3],
        };
        worker.publish_verified_record(record).unwrap();
        assert_eq!(worker.live_records(now).len(), 1);
        worker.expire(now + 61);
        assert!(worker.live_records(now + 61).is_empty());
    }

    #[test]
    fn admission_is_bounded() {
        let mut worker = BootstrapWorker::new(BootstrapPolicy {
            enabled: true,
            max_inbound_peers: 1,
            ..Default::default()
        });
        worker.evaluate_eligibility(&BootstrapEligibilityInput {
            stable_identity: true,
            listener_started: true,
            advertised_address_count: 1,
            clock_sane: true,
            resource_budget_ok: true,
            network_id_matches: true,
        });
        worker.authorize(true);
        worker.start().unwrap();
        worker.mark_ready(true).unwrap();
        worker.admit_peer("peer-a".into(), now_secs()).unwrap();
        assert_eq!(
            worker.admit_peer("peer-b".into(), now_secs()),
            Err(BootstrapError::AdmissionLimit)
        );
    }
}

pub fn current_timestamp() -> u64 {
    now_secs()
}
