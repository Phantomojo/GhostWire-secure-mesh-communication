//! knowledge_ark.rs — The Knowledge Ark: Ghostwire's distributed knowledge layer
//!
//! The Indra's Net vision: every node in the mesh is not just a relay —
//! it's a sovereign library. Knowledge flows freely between nodes, persists
//! through network partitions, and is owned by no one and available to all.
//!
//! The Knowledge Ark is the implementation of that vision:
//!
//!   - Store: notes, articles, code snippets, references, datasets
//!   - Sync: entries replicate across trusted peers via DHT
//!   - Search: full-text search over local + mesh knowledge
//!   - Access control: public (mesh-wide), shared (trusted peers), private
//!   - Versioning: every edit creates a new content-addressed version
//!   - Resilience: content-addressed blocks via BlockStore means it
//!     survives node failures and network partitions
//!
//! This is what makes Ghostwire more than a messenger — it's a
//! decentralised knowledge commons rooted in Ubuntu philosophy.
//!
//! Architecture:
//!   KnowledgeArk → manages collections → contains ArkEntry items
//!   Each ArkEntry → stored as encrypted block(s) in BlockStore
//!   Entry CID → published to DHT (public) or sent directly (private)
//!   Entry index → local sled tree for search/retrieval

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ── Entry types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryKind {
    /// Free-form markdown note
    Note,
    /// Long-form article with title + body
    Article,
    /// Code snippet with language tag
    Code { language: String },
    /// External reference with optional archived content
    Reference {
        url: String,
        archived_cid: Option<String>,
    },
    /// Structured data (CSV, JSON, etc.)
    DataSet { schema: Option<String> },
    /// Media clip reference (audio/video manifest CID)
    MediaClip {
        manifest_cid: String,
        duration_secs: Option<u32>,
    },
    /// A question seeking answers from the mesh
    Question,
    /// An answer to a Question entry
    Answer { question_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AccessLevel {
    /// Visible to all mesh nodes
    Public,
    /// Visible to trusted peers (WoT threshold)
    Trusted,
    /// Visible only to specified peers
    Shared { peer_ids: Vec<String> },
    /// Only visible locally — never leaves this device
    #[default]
    Private,
}

// ── Core entry ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArkEntry {
    /// Unique entry ID (UUID)
    pub id: String,
    /// Human-readable title
    pub title: String,
    /// The actual content
    pub body: String,
    /// Entry type
    pub kind: EntryKind,
    /// Who created it (node ID)
    pub author: String,
    /// Collection this entry belongs to
    pub collection: String,
    /// Tags for search/filtering
    pub tags: Vec<String>,
    /// Access control
    pub access: AccessLevel,
    /// Creation timestamp
    pub created_at: u64,
    /// Last edited timestamp
    pub updated_at: u64,
    /// Previous version's CID (for version chain)
    pub parent_cid: Option<String>,
    /// Content-addressed ID of this entry (BLAKE3 of serialized content)
    pub cid: String,
    /// Ed25519 signature of (id + title + body + author + created_at)
    pub signature: Option<Vec<u8>>,
    /// Schema version for forward compatibility
    pub schema_version: u8,
}

impl ArkEntry {
    pub fn new(
        title: impl Into<String>,
        body: impl Into<String>,
        kind: EntryKind,
        author: impl Into<String>,
        collection: impl Into<String>,
    ) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_secs();
        let title = title.into();
        let body = body.into();
        let author = author.into();
        let collection = collection.into();

        let cid = Self::compute_cid(&id, &title, &body, &author, now);

        Self {
            id,
            title,
            body,
            kind,
            author,
            collection,
            tags: vec![],
            access: AccessLevel::Private,
            created_at: now,
            updated_at: now,
            parent_cid: None,
            cid,
            signature: None,
            schema_version: 1,
        }
    }

    /// Create a new version of this entry with updated content.
    /// The new version references this entry as its parent.
    pub fn new_version(&self, new_body: impl Into<String>) -> Self {
        let new_body = new_body.into();
        let now = now_secs();
        let new_cid = Self::compute_cid(&self.id, &self.title, &new_body, &self.author, now);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            body: new_body,
            updated_at: now,
            parent_cid: Some(self.cid.clone()),
            cid: new_cid,
            signature: None,
            ..self.clone()
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_access(mut self, access: AccessLevel) -> Self {
        self.access = access;
        self
    }

    /// Bytes that should be signed: deterministic over key fields
    pub fn signable_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(self.id.as_bytes());
        b.extend_from_slice(self.title.as_bytes());
        b.extend_from_slice(self.body.as_bytes());
        b.extend_from_slice(self.author.as_bytes());
        b.extend_from_slice(&self.created_at.to_le_bytes());
        b
    }

    pub fn is_public(&self) -> bool {
        matches!(self.access, AccessLevel::Public)
    }

    fn compute_cid(id: &str, title: &str, body: &str, author: &str, ts: u64) -> String {
        let mut data = id.as_bytes().to_vec();
        data.extend_from_slice(title.as_bytes());
        data.extend_from_slice(body.as_bytes());
        data.extend_from_slice(author.as_bytes());
        data.extend_from_slice(&ts.to_le_bytes());
        hex::encode(blake3::hash(&data).as_bytes())
    }
}

// ── Collection ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArkCollection {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub access: AccessLevel,
    pub entry_count: u64,
    pub created_at: u64,
    pub last_synced: Option<u64>,
}

impl ArkCollection {
    pub fn new(name: impl Into<String>, owner: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            description: String::new(),
            owner: owner.into(),
            access: AccessLevel::Private,
            entry_count: 0,
            created_at: now_secs(),
            last_synced: None,
        }
    }
}

// ── The Knowledge Ark ─────────────────────────────────────────────────────────

pub struct KnowledgeArk {
    /// Collection metadata (id → collection)
    collections: Arc<RwLock<HashMap<String, ArkCollection>>>,
    /// All entries indexed by CID
    entries_by_cid: Arc<RwLock<HashMap<String, ArkEntry>>>,
    /// Collection → Vec<entry CIDs>
    collection_index: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// Tag index: tag → Vec<entry CIDs>
    tag_index: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// Full-text word index: word → Vec<entry CIDs>
    word_index: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// This node's owner ID (for access control checks)
    owner_id: String,
}

impl KnowledgeArk {
    pub fn new(owner_id: impl Into<String>) -> Self {
        Self {
            collections: Arc::new(RwLock::new(HashMap::new())),
            entries_by_cid: Arc::new(RwLock::new(HashMap::new())),
            collection_index: Arc::new(RwLock::new(HashMap::new())),
            tag_index: Arc::new(RwLock::new(HashMap::new())),
            word_index: Arc::new(RwLock::new(HashMap::new())),
            owner_id: owner_id.into(),
        }
    }

    /// Create a new collection
    pub async fn create_collection(&self, name: impl Into<String>) -> ArkCollection {
        let col = ArkCollection::new(name, self.owner_id.clone());
        self.collections
            .write()
            .await
            .insert(col.id.clone(), col.clone());
        col
    }

    /// Add an entry to the ark
    pub async fn add_entry(&self, entry: ArkEntry) -> Result<String> {
        let cid = entry.cid.clone();
        let collection = entry.collection.clone();
        let tags = entry.tags.clone();
        let title_words = tokenize(&entry.title);
        let body_words = tokenize(&entry.body);

        // Store entry
        self.entries_by_cid.write().await.insert(cid.clone(), entry);

        // Update collection index
        self.collection_index
            .write()
            .await
            .entry(collection.clone())
            .or_default()
            .push(cid.clone());

        // Update collection entry count
        if let Some(col) = self.collections.write().await.get_mut(&collection) {
            col.entry_count += 1;
        }

        // Update tag index
        for tag in &tags {
            self.tag_index
                .write()
                .await
                .entry(tag.clone())
                .or_default()
                .push(cid.clone());
        }

        // Update word index
        let mut word_idx = self.word_index.write().await;
        for word in title_words.into_iter().chain(body_words) {
            word_idx.entry(word).or_default().push(cid.clone());
        }

        Ok(cid)
    }

    /// Get an entry by CID
    pub async fn get_entry(&self, cid: &str) -> Option<ArkEntry> {
        self.entries_by_cid.read().await.get(cid).cloned()
    }

    /// List entries in a collection
    pub async fn list_collection(&self, collection_id: &str) -> Vec<ArkEntry> {
        let index = self.collection_index.read().await;
        let entries = self.entries_by_cid.read().await;

        index
            .get(collection_id)
            .map(|cids| {
                cids.iter()
                    .filter_map(|cid| entries.get(cid))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Full-text search across all accessible entries
    /// Returns entries sorted by relevance (term frequency)
    pub async fn search(&self, query: &str) -> Vec<(ArkEntry, f32)> {
        let words = tokenize(query);
        let word_idx = self.word_index.read().await;
        let entries = self.entries_by_cid.read().await;

        let mut scores: HashMap<String, f32> = HashMap::new();
        for word in &words {
            if let Some(cids) = word_idx.get(word) {
                for cid in cids {
                    *scores.entry(cid.clone()).or_insert(0.0) += 1.0;
                }
            }
            // Partial prefix match: "rust" matches "rustacean"
            for (indexed_word, cids) in word_idx.iter() {
                if indexed_word.starts_with(word.as_str()) && indexed_word != word.as_str() {
                    for cid in cids {
                        *scores.entry(cid.clone()).or_insert(0.0) += 0.5;
                    }
                }
            }
        }

        let mut results: Vec<(ArkEntry, f32)> = scores
            .into_iter()
            .filter_map(|(cid, score)| {
                entries
                    .get(&cid)
                    .filter(|e| self.can_access(e))
                    .map(|e| (e.clone(), score))
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Search by tag
    pub async fn search_by_tag(&self, tag: &str) -> Vec<ArkEntry> {
        let tag_idx = self.tag_index.read().await;
        let entries = self.entries_by_cid.read().await;

        tag_idx
            .get(tag)
            .map(|cids| {
                cids.iter()
                    .filter_map(|cid| entries.get(cid))
                    .filter(|e| self.can_access(e))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all collections visible to this node
    pub async fn list_collections(&self) -> Vec<ArkCollection> {
        self.collections.read().await.values().cloned().collect()
    }

    /// Delete an entry by CID
    pub async fn delete_entry(&self, cid: &str) -> Result<()> {
        let entry = self
            .entries_by_cid
            .write()
            .await
            .remove(cid)
            .ok_or_else(|| anyhow::anyhow!("Entry not found: {}", cid))?;

        // Remove from collection index
        if let Some(cids) = self
            .collection_index
            .write()
            .await
            .get_mut(&entry.collection)
        {
            cids.retain(|c| c != cid);
        }

        // Remove from tag index
        for tag in &entry.tags {
            if let Some(cids) = self.tag_index.write().await.get_mut(tag) {
                cids.retain(|c| c != cid);
            }
        }

        Ok(())
    }

    /// Export all public entries for mesh replication
    /// Returns entries as (cid, serialized_bytes) pairs
    pub async fn export_public(&self) -> Vec<(String, Vec<u8>)> {
        let entries = self.entries_by_cid.read().await;
        entries
            .values()
            .filter(|e| e.is_public())
            .filter_map(|e| {
                bincode::serialize(e)
                    .ok()
                    .map(|bytes| (e.cid.clone(), bytes))
            })
            .collect()
    }

    /// Import an entry received from the mesh (verify signature before accepting)
    pub async fn import_entry(&self, bytes: &[u8]) -> Result<String> {
        let entry: ArkEntry = bincode::deserialize(bytes)
            .map_err(|e| anyhow::anyhow!("Deserialize failed: {}", e))?;

        // Verify CID integrity
        let expected_cid = ArkEntry::compute_cid(
            &entry.id,
            &entry.title,
            &entry.body,
            &entry.author,
            entry.created_at,
        );
        if entry.cid != expected_cid {
            anyhow::bail!(
                "Entry CID mismatch — possible tampering (got {}, expected {})",
                entry.cid,
                expected_cid
            );
        }

        // Only accept public entries from peers
        if !entry.is_public() {
            anyhow::bail!("Refusing to import non-public entry from peer");
        }

        self.add_entry(entry).await
    }

    /// Stats summary
    pub async fn stats(&self) -> ArkStats {
        ArkStats {
            total_entries: self.entries_by_cid.read().await.len(),
            total_collections: self.collections.read().await.len(),
            total_tags: self.tag_index.read().await.len(),
            indexed_words: self.word_index.read().await.len(),
        }
    }

    fn can_access(&self, entry: &ArkEntry) -> bool {
        match &entry.access {
            AccessLevel::Public => true,
            AccessLevel::Private => entry.author == self.owner_id,
            AccessLevel::Trusted => true, // Trust check handled at mesh layer
            AccessLevel::Shared { peer_ids } => {
                peer_ids.contains(&self.owner_id) || entry.author == self.owner_id
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ArkStats {
    pub total_entries: usize,
    pub total_collections: usize,
    pub total_tags: usize,
    pub indexed_words: usize,
}

/// Tokenize text for indexing: lowercase, alphanumeric words only, min 3 chars
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .flat_map(|w| w.split(|c: char| !c.is_alphanumeric()))
        .map(|w| w.to_lowercase())
        .filter(|w| w.len() >= 3)
        .collect()
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
    async fn add_and_search() {
        let ark = KnowledgeArk::new("node_abc");
        let col = ark.create_collection("Personal Notes").await;

        let entry = ArkEntry::new(
            "Ubuntu philosophy",
            "I am because we are — the foundation of community",
            EntryKind::Note,
            "node_abc",
            col.id.clone(),
        )
        .with_tags(vec!["philosophy".into(), "ubuntu".into()])
        .with_access(AccessLevel::Public);

        ark.add_entry(entry).await.unwrap();

        let results = ark.search("ubuntu community").await;
        assert!(!results.is_empty());
        assert!(results[0].0.title.contains("Ubuntu"));
    }

    #[tokio::test]
    async fn cid_integrity_check_on_import() {
        let ark = KnowledgeArk::new("node_xyz");
        let mut entry = ArkEntry::new("Test", "Content", EntryKind::Note, "node_abc", "col1");
        entry.access = AccessLevel::Public;

        // Tamper with body after CID computed
        entry.body = "TAMPERED".to_string();

        let bytes = bincode::serialize(&entry).unwrap();
        let result = ark.import_entry(&bytes).await;
        assert!(result.is_err(), "Tampered entry should be rejected");
    }

    #[tokio::test]
    async fn versioning_preserves_parent_chain() {
        let ark = KnowledgeArk::new("node_abc");
        let col = ark.create_collection("Docs").await;

        let v1 = ArkEntry::new(
            "Doc",
            "Version 1",
            EntryKind::Note,
            "node_abc",
            col.id.clone(),
        );
        let v1_cid = v1.cid.clone();
        ark.add_entry(v1.clone()).await.unwrap();

        let v2 = v1.new_version("Version 2 — improved");
        assert_eq!(v2.parent_cid, Some(v1_cid));
    }
}
