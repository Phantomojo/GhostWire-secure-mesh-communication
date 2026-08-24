use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoverySource {
    Mdns,
    Dht,
    Transport,
    Manual,
    Social,
    GossipSub,
    Rendezvous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    Unverified,
    Candidate,
    Verified,
    Authorized,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReachabilityState {
    Unknown,
    Seen,
    Reachable,
    Unreachable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryEntry {
    pub node_id: String,
    pub did: Option<String>,
    pub multiaddrs: BTreeSet<String>,
    pub sources: BTreeSet<DiscoverySource>,
    pub first_seen: u64,
    pub last_seen: u64,
    pub trust: TrustState,
    pub reachability: ReachabilityState,
    pub online: bool,
}

#[derive(Clone, Default)]
pub struct DiscoveryDirectory {
    entries: Arc<RwLock<HashMap<String, DirectoryEntry>>>,
}

impl DiscoveryDirectory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Discovery is evidence only. It cannot authorize or verify a peer.
    pub async fn observe(
        &self,
        node_id: String,
        did: Option<String>,
        multiaddrs: impl IntoIterator<Item = String>,
        source: DiscoverySource,
        now: u64,
    ) {
        let mut entries = self.entries.write().await;
        let entry = entries
            .entry(node_id.clone())
            .or_insert_with(|| DirectoryEntry {
                node_id,
                did: None,
                multiaddrs: BTreeSet::new(),
                sources: BTreeSet::new(),
                first_seen: now,
                last_seen: now,
                trust: TrustState::Unverified,
                reachability: ReachabilityState::Unknown,
                online: false,
            });
        if did.is_some() {
            entry.did = did;
        }
        entry.multiaddrs.extend(multiaddrs);
        entry.sources.insert(source);
        entry.last_seen = now;
        entry.reachability = ReachabilityState::Seen;
        entry.online = true;
    }

    /// Trust transitions are explicit and never implied by discovery evidence.
    pub async fn set_trust(&self, node_id: &str, trust: TrustState) -> bool {
        let mut entries = self.entries.write().await;
        if let Some(entry) = entries.get_mut(node_id) {
            entry.trust = trust;
            true
        } else {
            false
        }
    }

    pub async fn mark_reachability(&self, node_id: &str, state: ReachabilityState) -> bool {
        let mut entries = self.entries.write().await;
        if let Some(entry) = entries.get_mut(node_id) {
            entry.reachability = state;
            entry.online = matches!(
                state,
                ReachabilityState::Reachable | ReachabilityState::Seen
            );
            true
        } else {
            false
        }
    }

    pub async fn snapshot(&self) -> Vec<DirectoryEntry> {
        let entries = self.entries.read().await;
        let mut snapshot: Vec<_> = entries.values().cloned().collect();
        snapshot.sort_by(|a, b| a.node_id.cmp(&b.node_id));
        snapshot
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn discovery_does_not_authorize_peer() {
        let directory = DiscoveryDirectory::new();
        directory
            .observe(
                "peer-a".into(),
                Some("did:ghost:a".into()),
                ["/ip4/192.0.2.1/udp/4001/quic-v1".into()],
                DiscoverySource::Mdns,
                10,
            )
            .await;
        let entry = directory.snapshot().await.pop().unwrap();
        assert_eq!(entry.trust, TrustState::Unverified);
        assert!(entry.sources.contains(&DiscoverySource::Mdns));
        assert!(directory.set_trust("peer-a", TrustState::Authorized).await);
        assert_eq!(directory.snapshot().await[0].trust, TrustState::Authorized);
    }

    #[tokio::test]
    async fn observations_merge_sources_and_addresses_deterministically() {
        let directory = DiscoveryDirectory::new();
        directory
            .observe(
                "peer-b".into(),
                None,
                ["addr-b".into()],
                DiscoverySource::Dht,
                20,
            )
            .await;
        directory
            .observe(
                "peer-b".into(),
                None,
                ["addr-a".into()],
                DiscoverySource::Mdns,
                30,
            )
            .await;
        let entry = directory.snapshot().await.pop().unwrap();
        assert_eq!(entry.first_seen, 20);
        assert_eq!(entry.last_seen, 30);
        assert_eq!(
            entry.multiaddrs.iter().cloned().collect::<Vec<_>>(),
            vec!["addr-a", "addr-b"]
        );
        assert_eq!(entry.sources.len(), 2);
    }
}
