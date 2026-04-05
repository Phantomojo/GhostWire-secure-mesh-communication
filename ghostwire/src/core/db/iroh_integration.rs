//! iroh_store.rs — Iroh-based content store (replaces custom BlockStore)
//!
//! WHY IROH OVER CUSTOM BlockStore:
//!   - Production-grade Rust implementation from the n0computer team (ex-IPFS core)
//!   - iroh-bytes: verified, resumable, parallel blob transfer
//!   - iroh-docs: CRDT-based document sync (replaces Knowledge Ark's naive HashMap)
//!   - Well-tested, actively maintained, 2024 stable release
//!   - Designed for exactly Ghostwire's use case: offline-first P2P apps
//!
//! HOW IT FITS:
//!   Custom BlockStore → iroh-bytes  (file/media transfer)
//!   Knowledge Ark     → iroh-docs   (collaborative knowledge entries)
//!   DHT storage       → Keep using libp2p Kademlia (routing)
//!   Message store     → Keep using encrypted sled (private, local)
//!
//! Cargo.toml additions:
//!   iroh = "0.26"
//!   iroh-bytes = "0.26"
//!   iroh-docs = "0.26"
//!   iroh-net = "0.26"
//!
//! NOTE: Iroh is a large dependency. Add it as an optional feature:
//!   [features]
//!   iroh = ["dep:iroh", "dep:iroh-bytes", "dep:iroh-docs"]

use anyhow::Result;
use serde::{Deserialize, Serialize};

// ── Why Iroh specifically ─────────────────────────────────────────────────────
//
// Compared to alternatives:
//
// OrbitDB:
//   + Rich query model, event log, feed, docstore
//   - JavaScript only. Rust FFI is painful. Not suitable.
//
// Gun.js:
//   + Radically decentralised, no server needed
//   - JavaScript only. Same problem.
//
// TiKV / FoundationDB:
//   + Production-grade distributed KV
//   - Designed for server clusters, not P2P. Overkill and wrong model.
//
// Automerge (CRDTs):
//   + Perfect for collaborative editing (Knowledge Ark)
//   + Rust-native via automerge-rs
//   + Offline-first by design
//   - No transport/sync layer — need to wire it yourself
//   VERDICT: Use for Knowledge Ark entries specifically
//
// Iroh:
//   + Rust-native, async, tokio-based
//   + iroh-bytes: content-addressed blob transfer (=our BlockStore)
//   + iroh-docs: CRDT document sync with access control (=Ark collections)
//   + Designed by people who built IPFS, learned from its mistakes
//   + Works offline-first, syncs when peers reconnect
//   + Hash-verified transfers (BLAKE3, same as our crypto stack)
//   + Active 2024 development, stable API
//   VERDICT: Primary choice for file transfer + knowledge sync
//
// Conclusion:
//   iroh-bytes  → file/media transfer (replace BlockStore)
//   iroh-docs   → Knowledge Ark collaborative editing
//   automerge   → fine-grained CRDT for real-time collab (future)
//   sled        → private local storage (messages, identity) — keep

// ── Integration plan ──────────────────────────────────────────────────────────
//
// STEP 1: Add to Cargo.toml (feature-gated):
//
// [dependencies]
// iroh = { version = "0.26", optional = true }
//
// [features]
// iroh-storage = ["dep:iroh"]
//
// STEP 2: Initialize iroh node alongside Core:
//
// ```rust
// use iroh::node::Node;
//
// pub struct GhostwireNode {
//     core: Arc<Core>,
//     iroh: Arc<Node<iroh::blobs::store::fs::Store>>,
// }
//
// impl GhostwireNode {
//     pub async fn new(data_dir: &Path, core: Arc<Core>) -> Result<Self> {
//         let iroh = Node::persistent(data_dir.join("iroh"))
//             .await?
//             .spawn()
//             .await?;
//
//         Ok(Self { core, iroh })
//     }
// }
// ```
//
// STEP 3: File send via iroh-bytes:
//
// ```rust
// // Sender:
// let client = iroh.client();
// let (ticket, size) = client.blobs().add_path(file_path).await?;
// // Share `ticket` to recipient (via message, out-of-band, etc.)
//
// // Recipient:
// client.blobs().download(ticket).await?;
// ```
//
// STEP 4: Knowledge Ark via iroh-docs:
//
// ```rust
// // Create or open a document (= an Ark collection)
// let doc = client.docs().create().await?;
// let doc_id = doc.id(); // Share this to collaborators
//
// // Write an entry:
// doc.set_bytes(author, "entry/abc123", entry_bytes).await?;
//
// // Read entries:
// let entries = doc.get_many(Query::all()).await?;
//
// // Share with a peer:
// let ticket = doc.share(ShareMode::Write).await?;
// // → Peer joins: client.docs().import(ticket).await?
//
// // Entries sync automatically when both peers are online.
// // Works offline — changes merge when they reconnect.
// ```
//
// STEP 5: Wire iroh-docs into KnowledgeArk:
//
// ```rust
// impl KnowledgeArk {
//     // Add an entry that also syncs via iroh-docs:
//     pub async fn add_synced_entry(
//         &self,
//         entry: ArkEntry,
//         iroh_doc: &iroh_docs::Doc,
//     ) -> Result<String> {
//         let cid = self.add_entry(entry.clone()).await?;
//
//         // Also write to iroh doc for cross-peer sync
//         if entry.is_public() {
//             let bytes = bincode::serialize(&entry)?;
//             iroh_doc.set_bytes(
//                 iroh_author,
//                 format!("ark/{}", cid).as_bytes(),
//                 bytes,
//             ).await?;
//         }
//
//         Ok(cid)
//     }
// }
// ```

// ── Automerge for real-time collaboration ─────────────────────────────────────
//
// Cargo.toml:
//   automerge = "0.5"
//
// When to use automerge vs iroh-docs:
//   - iroh-docs: asynchronous sync, eventually consistent (best for Ark entries)
//   - automerge: real-time collab like Google Docs (best for future co-editing)
//
// Example: collaborative note editing
//
// ```rust
// use automerge::{AutoCommit, Automerge, ReadDoc, transaction::Transactable};
//
// // Both users start with the same doc state
// let mut doc = AutoCommit::new();
//
// // User A edits:
// doc.put(automerge::ROOT, "content", "Ubuntu - I am because we are")?;
// let change_a = doc.save();
//
// // User B edits independently (offline):
// let mut doc_b = AutoCommit::load(&initial_bytes)?;
// doc_b.put(automerge::ROOT, "content", "I am because we are - Ubuntu")?;
// let change_b = doc_b.save();
//
// // Merge (no conflict — last-write-wins for text fields):
// doc.merge(&mut AutoCommit::load(&change_b)?)?;
// // Result: deterministically merged, no data lost
// ```

// ── Ghost Channels (anonymous broadcast) ─────────────────────────────────────
//
// Architecture decision: use existing Gossipsub with Sphinx routing
//
// A Ghost Channel is:
//   - A Gossipsub topic with a random 32-byte topic_id
//   - All messages Sphinx-routed before publishing (sender unlinkable)
//   - Anti-spam: publishers must solve a PoW puzzle (blake3 preimage, 16 bits)
//
// Implementation (add to mesh.rs):
//
// ```rust
// pub struct GhostChannel {
//     pub topic_id: [u8; 32],
//     pub topic_str: String, // gossipsub IdentTopic string = hex(topic_id)
//     pub name: Option<String>,
//     pub description: Option<String>,
// }
//
// impl GhostChannel {
//     pub fn new() -> Self {
//         let mut topic_id = [0u8; 32];
//         SystemRandom::new().fill(&mut topic_id).unwrap();
//         let topic_str = hex::encode(topic_id);
//         Self { topic_id, topic_str, name: None, description: None }
//     }
//
//     /// Compute PoW puzzle solution before publishing
//     /// Returns (message_bytes, nonce) where blake3(msg || nonce) has N leading zeros
//     pub fn solve_pow(message: &[u8], difficulty_bits: u32) -> (Vec<u8>, u64) {
//         let mut nonce = 0u64;
//         loop {
//             let mut data = message.to_vec();
//             data.extend_from_slice(&nonce.to_le_bytes());
//             let hash = blake3::hash(&data);
//             if meets_difficulty(hash.as_bytes(), difficulty_bits) {
//                 return (data, nonce);
//             }
//             nonce += 1;
//         }
//     }
// }
// ```
//
// In mesh.rs publish path:
//   1. Solve PoW
//   2. Wrap in Sphinx packet (sender unlinkable)
//   3. Publish to gossipsub topic
//
// No owner, no membership, no identity. Anyone with the topic_id can join.
// Share topic_id via QR code or direct message.

// ── Iroh cargo.toml snippet ───────────────────────────────────────────────────
pub const CARGO_ADDITIONS: &str = r#"
# Wave 5: Iroh content store (optional feature)
iroh = { version = "0.26", optional = true, features = ["mem"] }

# Wave 5: CRDT collaborative editing
automerge = { version = "0.5", optional = true }

[features]
# Add to existing features:
iroh-storage = ["dep:iroh"]
collab = ["dep:automerge"]
"#;
