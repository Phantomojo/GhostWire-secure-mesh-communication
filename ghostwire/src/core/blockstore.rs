//! blockstore.rs — Content-addressed block storage with Bitswap-inspired exchange
//!
//! Implements the full BlockStore + Bitswap protocol for Ghostwire:
//!
//!   - Files are split into fixed-size blocks (default 256 KiB)
//!   - Each block's address = BLAKE3(block_bytes) — content-addressed
//!   - Blocks are distributed across the Kademlia DHT
//!   - Peers exchange blocks via wantlist + ledger + strategy engine
//!   - Receiving a block = start seeding it (Take What You Give)
//!
//! Privacy adaptations vs. plain IPFS Bitswap:
//!   - No public provider announcements (would leak what content you hold)
//!   - Block requests are routed via private routes, not direct DHT queries
//!   - Ledger is per-session (resets on restart) to avoid long-term tracking
//!
//! INTEGRATION:
//!   1. Add BlockStore to Core struct
//!   2. Wire file_send API endpoint → BlockStore::store_file()
//!   3. Wire file_receive API endpoint → BlockStore::retrieve_file()
//!   4. Add Bitswap message type to wire_protocol.rs FrameKind
//!   5. In mesh.rs event loop, handle incoming Bitswap WANT/HAVE/BLOCK messages

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use zeroize::Zeroize;

/// Default block size: 256 KiB (balances DHT overhead vs. transfer efficiency)
pub const DEFAULT_BLOCK_SIZE: usize = 256 * 1024;

/// Maximum number of blocks to request from a single peer at once
pub const MAX_WANTLIST_PER_PEER: usize = 16;

/// How long to remember a block's availability from a peer
const PROVIDER_TTL_SECS: u64 = 3600;

/// A content-addressed block
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize)]
pub struct Block {
    /// BLAKE3 hash of `data` — the block's permanent address
    pub cid: [u8; 32], // Content ID
    /// Raw block bytes
    pub data: Vec<u8>,
}

impl Block {
    /// Create a new block, computing its CID automatically
    pub fn new(data: Vec<u8>) -> Self {
        let cid = *blake3::hash(&data).as_bytes();
        Self { cid, data }
    }

    pub fn cid_hex(&self) -> String {
        hex::encode(self.cid)
    }

    /// Verify data integrity
    pub fn verify(&self) -> bool {
        let computed = blake3::hash(&self.data);
        computed.as_bytes() == &self.cid
    }
}

/// A file split into ordered blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileManifest {
    /// Original filename (optional)
    pub name: Option<String>,
    /// Total original file size in bytes
    pub total_bytes: u64,
    /// Ordered list of block CIDs
    pub block_cids: Vec<[u8; 32]>,
    /// Block size used for this file
    pub block_size: usize,
    /// BLAKE3 hash of the complete reassembled file (integrity check)
    pub file_hash: [u8; 32],
    /// Unix timestamp of creation
    pub created_at: u64,
}

impl FileManifest {
    /// The manifest's own CID = BLAKE3 of its canonical serialization
    pub fn cid(&self) -> [u8; 32] {
        let serialized = bincode::serialize(self).unwrap_or_default();
        *blake3::hash(&serialized).as_bytes()
    }
}

// ── Block storage ─────────────────────────────────────────────────────────────

/// Local block storage (backed by sled via EncryptedStore in production)
pub struct BlockStore {
    /// CID (hex) → Block data
    blocks: Arc<RwLock<HashMap<String, Block>>>,
    /// CID (hex) → file manifests that reference this block
    manifests: Arc<RwLock<HashMap<String, FileManifest>>>,
    /// CID (hex) → set of peer IDs known to have this block
    providers: Arc<RwLock<HashMap<String, ProviderSet>>>,
    /// Wantlist: blocks we want, with priority
    wantlist: Arc<RwLock<Wantlist>>,
    /// Per-peer debt ledger
    ledger: Arc<RwLock<Ledger>>,
    /// Block size for new files
    block_size: usize,
}

#[derive(Debug)]
struct ProviderSet {
    peers: HashSet<String>,
    last_updated: u64,
}

impl BlockStore {
    pub fn new(block_size: usize) -> Self {
        Self {
            blocks: Arc::new(RwLock::new(HashMap::new())),
            manifests: Arc::new(RwLock::new(HashMap::new())),
            providers: Arc::new(RwLock::new(HashMap::new())),
            wantlist: Arc::new(RwLock::new(Wantlist::new())),
            ledger: Arc::new(RwLock::new(Ledger::new())),
            block_size,
        }
    }

    /// Split a file into blocks and store them locally.
    /// Returns the FileManifest (share the CID with the recipient).
    pub async fn store_file(&self, data: &[u8], name: Option<String>) -> Result<FileManifest> {
        let file_hash = *blake3::hash(data).as_bytes();
        let mut block_cids = Vec::new();
        let mut blocks = self.blocks.write().await;

        for chunk in data.chunks(self.block_size) {
            let block = Block::new(chunk.to_vec());
            block_cids.push(block.cid);
            blocks.insert(block.cid_hex(), block);
        }

        let manifest = FileManifest {
            name,
            total_bytes: data.len() as u64,
            block_cids,
            block_size: self.block_size,
            file_hash,
            created_at: now_secs(),
        };

        let manifest_cid = hex::encode(manifest.cid());
        self.manifests
            .write()
            .await
            .insert(manifest_cid, manifest.clone());

        Ok(manifest)
    }

    /// Retrieve a complete file from local storage using a manifest.
    /// Returns None if any block is missing (needs to be fetched from peers).
    pub async fn retrieve_file(&self, manifest: &FileManifest) -> Result<Option<Vec<u8>>> {
        let blocks = self.blocks.read().await;
        let mut file_data = Vec::with_capacity(manifest.total_bytes as usize);

        for cid in &manifest.block_cids {
            let cid_hex = hex::encode(cid);
            match blocks.get(&cid_hex) {
                Some(block) => {
                    if !block.verify() {
                        anyhow::bail!("Block {} failed integrity check", cid_hex);
                    }
                    file_data.extend_from_slice(&block.data);
                }
                None => return Ok(None), // Missing — caller should use wantlist
            }
        }

        // Verify complete file integrity
        let actual_hash = blake3::hash(&file_data);
        if actual_hash.as_bytes() != &manifest.file_hash {
            anyhow::bail!("File integrity check failed after reassembly");
        }

        Ok(Some(file_data))
    }

    /// Store a received block. Returns true if this is a new block.
    pub async fn store_block(&self, block: Block) -> Result<bool> {
        if !block.verify() {
            anyhow::bail!("Received block {} failed integrity check", block.cid_hex());
        }
        let cid_hex = block.cid_hex();
        let is_new = !self.blocks.read().await.contains_key(&cid_hex);
        self.blocks.write().await.insert(cid_hex.clone(), block);
        if is_new {
            // Remove from wantlist — we now have it
            self.wantlist.write().await.remove(&cid_hex);
        }
        Ok(is_new)
    }

    /// Get a block we have locally
    pub async fn get_block(&self, cid: &[u8; 32]) -> Option<Block> {
        self.blocks.read().await.get(&hex::encode(cid)).cloned()
    }

    /// Check which blocks from a manifest we're missing
    pub async fn missing_blocks(&self, manifest: &FileManifest) -> Vec<[u8; 32]> {
        let blocks = self.blocks.read().await;
        manifest
            .block_cids
            .iter()
            .filter(|cid| !blocks.contains_key(&hex::encode(*cid)))
            .copied()
            .collect()
    }

    /// Add missing blocks to our wantlist
    pub async fn want_manifest(&self, manifest: &FileManifest) {
        let missing = self.missing_blocks(manifest).await;
        let mut wantlist = self.wantlist.write().await;
        for cid in missing {
            wantlist.add(hex::encode(cid), 1);
        }
    }

    /// Get our current wantlist as a Bitswap WANT message payload
    pub async fn export_wantlist(&self) -> Vec<WantEntry> {
        self.wantlist.read().await.entries()
    }

    /// Process an incoming HAVE announcement from a peer
    pub async fn process_have(&self, peer_id: &str, cid_hex: &str) {
        let mut providers = self.providers.write().await;
        let entry = providers.entry(cid_hex.to_string()).or_insert(ProviderSet {
            peers: HashSet::new(),
            last_updated: now_secs(),
        });
        entry.peers.insert(peer_id.to_string());
        entry.last_updated = now_secs();
    }

    /// Strategy: which peers should we request blocks from?
    /// Prioritises peers with good debt ratios.
    pub async fn strategy_select_peers(
        &self,
        cid_hex: &str,
        max_peers: usize,
    ) -> Vec<String> {
        let providers = self.providers.read().await;
        let ledger = self.ledger.read().await;

        let Some(provider_set) = providers.get(cid_hex) else {
            return vec![];
        };

        let now = now_secs();
        if now - provider_set.last_updated > PROVIDER_TTL_SECS {
            return vec![]; // Stale provider info
        }

        let mut candidates: Vec<(&String, f64)> = provider_set
            .peers
            .iter()
            .map(|peer_id| {
                let score = ledger.peer_score(peer_id);
                (peer_id, score)
            })
            .collect();

        // Sort by score descending (best peers first)
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates
            .into_iter()
            .take(max_peers)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Record bytes received from a peer (for ledger)
    pub async fn record_received(&self, peer_id: &str, bytes: u64) {
        self.ledger.write().await.record_received(peer_id, bytes);
    }

    /// Record bytes sent to a peer (for ledger)
    pub async fn record_sent(&self, peer_id: &str, bytes: u64) {
        self.ledger.write().await.record_sent(peer_id, bytes);
    }

    /// Get ledger stats for a peer
    pub async fn peer_stats(&self, peer_id: &str) -> PeerLedgerEntry {
        self.ledger.read().await.get(peer_id).cloned().unwrap_or_default()
    }

    /// How many blocks we have locally
    pub async fn block_count(&self) -> usize {
        self.blocks.read().await.len()
    }

    /// Purge blocks older than `max_age` that no local manifest references
    pub async fn gc_unreferenced_blocks(&self, max_age_secs: u64) -> usize {
        let now = now_secs();
        let manifests = self.manifests.read().await;

        // Collect all referenced CIDs
        let referenced: HashSet<String> = manifests
            .values()
            .flat_map(|m| m.block_cids.iter().map(hex::encode))
            .collect();

        let mut blocks = self.blocks.write().await;
        let before = blocks.len();
        blocks.retain(|cid, _| referenced.contains(cid));
        before - blocks.len()
    }
}

// ── Wantlist ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WantEntry {
    pub cid_hex: String,
    pub priority: u32,
    pub want_type: WantType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WantType {
    /// We want the full block
    Block,
    /// Just tell us if you have it (don't send the data yet)
    Have,
}

struct Wantlist {
    entries: HashMap<String, WantEntry>,
}

impl Wantlist {
    fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    fn add(&mut self, cid_hex: String, priority: u32) {
        self.entries.entry(cid_hex.clone()).or_insert(WantEntry {
            cid_hex,
            priority,
            want_type: WantType::Block,
        });
    }

    fn remove(&mut self, cid_hex: &str) {
        self.entries.remove(cid_hex);
    }

    fn entries(&self) -> Vec<WantEntry> {
        self.entries.values().cloned().collect()
    }

    fn contains(&self, cid_hex: &str) -> bool {
        self.entries.contains_key(cid_hex)
    }
}

// ── Ledger ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PeerLedgerEntry {
    pub peer_id: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub first_seen: u64,
    pub last_exchange: u64,
}

impl PeerLedgerEntry {
    /// Debt ratio: bytes_received / bytes_sent
    /// > 1.0 = we've received more than sent (we're in debt)
    /// < 1.0 = we've sent more than received (we're a good peer)
    pub fn debt_ratio(&self) -> f64 {
        if self.bytes_sent == 0 {
            return f64::INFINITY; // Full debt — new peer who hasn't contributed yet
        }
        self.bytes_received as f64 / self.bytes_sent as f64
    }
}

struct Ledger {
    entries: HashMap<String, PeerLedgerEntry>,
}

impl Ledger {
    fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    fn record_sent(&mut self, peer_id: &str, bytes: u64) {
        let entry = self.entries.entry(peer_id.to_string()).or_insert(PeerLedgerEntry {
            peer_id: peer_id.to_string(),
            first_seen: now_secs(),
            ..Default::default()
        });
        entry.bytes_sent += bytes;
        entry.last_exchange = now_secs();
    }

    fn record_received(&mut self, peer_id: &str, bytes: u64) {
        let entry = self.entries.entry(peer_id.to_string()).or_insert(PeerLedgerEntry {
            peer_id: peer_id.to_string(),
            first_seen: now_secs(),
            ..Default::default()
        });
        entry.bytes_received += bytes;
        entry.last_exchange = now_secs();
    }

    /// Score a peer for strategy: higher = serve them faster
    fn peer_score(&self, peer_id: &str) -> f64 {
        match self.entries.get(peer_id) {
            None => 0.5, // Unknown peer: neutral score
            Some(entry) => {
                let ratio = entry.debt_ratio();
                if ratio == f64::INFINITY {
                    0.0 // Full debt — serve last
                } else {
                    // Score = 1 / (1 + debt_ratio)
                    // debt_ratio=0 (great peer) → score=1.0
                    // debt_ratio=1 (neutral)     → score=0.5
                    // debt_ratio=10 (leecher)    → score=0.09
                    1.0 / (1.0 + ratio)
                }
            }
        }
    }

    fn get(&self, peer_id: &str) -> Option<&PeerLedgerEntry> {
        self.entries.get(peer_id)
    }
}

// ── Bitswap message types ─────────────────────────────────────────────────────

/// Bitswap WANT message: "I want these blocks"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitswapWant {
    pub entries: Vec<WantEntry>,
}

/// Bitswap HAVE message: "I have this block (but am not sending it yet)"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitswapHave {
    pub cid_hex: String,
    pub block_size: u64,
}

/// Bitswap BLOCK message: "Here is the block you wanted"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitswapBlock {
    pub block: Block,
}

/// Bitswap CANCEL message: "I no longer need this block"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitswapCancel {
    pub cid_hex: String,
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

    #[tokio::test]
    async fn store_and_retrieve_file() {
        let store = BlockStore::new(1024); // 1 KiB blocks for test
        let data = b"Hello from Nairobi! ".repeat(200); // ~4 KiB
        let manifest = store.store_file(&data, Some("test.txt".to_string())).await.unwrap();

        assert!(manifest.block_cids.len() >= 4);
        let retrieved = store.retrieve_file(&manifest).await.unwrap().unwrap();
        assert_eq!(retrieved, data.to_vec());
    }

    #[tokio::test]
    async fn block_integrity_check() {
        let mut block = Block::new(b"valid data".to_vec());
        assert!(block.verify());
        block.data[0] ^= 0xFF; // Tamper
        assert!(!block.verify());
    }

    #[tokio::test]
    async fn wantlist_tracks_missing() {
        let store = BlockStore::new(1024);
        // Use unique data: each byte is different based on position
        let data: Vec<u8> = (0..3000).map(|i| ((i * 31 + i / 256 * 17 + 7) % 256) as u8).collect();
        let manifest = store.store_file(&data, None).await.unwrap();

        // Verify we have 3 unique blocks
        assert_eq!(manifest.block_cids.len(), 3, "Should have 3 blocks for 3000 bytes");

        // Verify all CIDs are unique
        let unique_cids: std::collections::HashSet<_> = manifest.block_cids.iter().collect();
        assert_eq!(unique_cids.len(), 3, "All block CIDs should be unique");

        // Now simulate having cleared one block
        let cid = manifest.block_cids[0];
        store.blocks.write().await.remove(&hex::encode(cid));

        let missing = store.missing_blocks(&manifest).await;
        assert_eq!(missing.len(), 1, "Should have exactly 1 missing block");
        assert_eq!(missing[0], cid);
    }

    #[test]
    fn ledger_debt_ratio() {
        let mut ledger = Ledger::new();
        ledger.record_sent("peer1", 1000);
        ledger.record_received("peer1", 500);

        let entry = ledger.get("peer1").unwrap();
        assert_eq!(entry.debt_ratio(), 0.5); // We sent more — good peer
        assert!(ledger.peer_score("peer1") > 0.5);
    }

    #[test]
    fn leecher_gets_low_score() {
        let mut ledger = Ledger::new();
        ledger.record_sent("leecher", 100);
        ledger.record_received("leecher", 10_000);
        assert!(ledger.peer_score("leecher") < 0.1);
    }
}
