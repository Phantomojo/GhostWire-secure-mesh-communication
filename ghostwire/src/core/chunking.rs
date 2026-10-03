use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Default chunk size for fat transports (libp2p/TCP/QUIC).
///
/// Not universal: a transport's own limit governs. Over Reticulum the usable
/// payload is `PACKET_MDU = 464` bytes, so a 60 KB chunk cannot cross that leg
/// at all. `split_into_chunks_with` takes the size from the caller; this
/// constant is only the fat-transport default.
pub const MAX_CHUNK_SIZE: usize = 60 * 1024;

const REASSEMBLY_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_PENDING_CHUNKS: usize = 64;

/// Hard ceiling on a single chunk's payload, whatever the sender declares.
///
/// `MAX_PENDING_CHUNKS` bounds the NUMBER of chunks in flight. It does nothing
/// about the size of any one, so a peer could send a single chunk with
/// `total_chunks: 1` and an arbitrarily large payload — one packet, one
/// unbounded allocation, and `Some(Vec<u8>)` returned to the caller. That was
/// demonstrated, not theorised: a 64 MB single chunk was accepted.
///
/// Aligned with `message.rs::MAX_INLINE_BYTES` (64 KB), which is the existing
/// bound for an inline (unchunked) message, so a chunk can never exceed what a
/// whole message is already allowed to be.
pub const MAX_CHUNK_PAYLOAD_BYTES: usize = 64 * 1024;

/// Hard ceiling on the total bytes one pending message may buffer.
///
/// Bounds the aggregate: 64 chunks x 64 KB would otherwise be 4 MB per
/// message, and many concurrent message ids multiplies it without limit.
pub const MAX_MESSAGE_BYTES: usize = 4 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkHeader {
    pub message_id: Uuid,
    pub chunk_index: u32,
    pub total_chunks: u32,
    pub original_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub header: ChunkHeader,
    pub payload: Vec<u8>,
}

pub fn split_into_chunks(message_id: Uuid, data: &[u8]) -> Vec<Chunk> {
    split_into_chunks_with(message_id, data, MAX_CHUNK_SIZE)
}

/// Split using a caller-supplied chunk size.
///
/// The size must come from the transport, not from a global: a single
/// `MAX_CHUNK_SIZE` cannot serve both a fat transport and Reticulum's 464-byte
/// MDU. Panics if `chunk_size` is zero, since `data.chunks(0)` panics and a
/// caller bug should not be a runtime abort on a network path.
pub fn split_into_chunks_with(
    message_id: Uuid,
    data: &[u8],
    chunk_size: usize,
) -> Vec<Chunk> {
    assert!(chunk_size > 0, "chunk_size must be non-zero");
    if data.len() <= chunk_size {
        return vec![Chunk {
            header: ChunkHeader {
                message_id,
                chunk_index: 0,
                total_chunks: 1,
                original_size: data.len(),
            },
            payload: data.to_vec(),
        }];
    }

    let total = data.len().div_ceil(chunk_size);
    data.chunks(chunk_size)
        .enumerate()
        .map(|(i, chunk)| Chunk {
            header: ChunkHeader {
                message_id,
                chunk_index: i as u32,
                total_chunks: total as u32,
                original_size: data.len(),
            },
            payload: chunk.to_vec(),
        })
        .collect()
}

struct PendingReassembly {
    chunks: Vec<Option<Vec<u8>>>,
    total_chunks: u32,
    original_size: usize,
    first_seen: Instant,
}

pub struct ChunkReassembler {
    pending: HashMap<Uuid, PendingReassembly>,
}

impl ChunkReassembler {
    pub fn new() -> Self {
        Self {
            pending: HashMap::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk: Chunk) -> Option<Vec<u8>> {
        let id = chunk.header.message_id;
        let idx = chunk.header.chunk_index as usize;
        let total = chunk.header.total_chunks as usize;

        // Reject oversized payloads BEFORE allocating anything on their behalf.
        // The count guard below is not a substitute: it bounds how many chunks
        // are in flight, not how large any one is.
        if chunk.payload.len() > MAX_CHUNK_PAYLOAD_BYTES {
            tracing::warn!(
                message_id = %id,
                bytes = chunk.payload.len(),
                limit = MAX_CHUNK_PAYLOAD_BYTES,
                "Rejected oversized chunk"
            );
            return None;
        }

        // The declared original_size is peer-supplied and must be plausible
        // against our own ceiling, or a message can claim to be enormous.
        if chunk.header.original_size > MAX_MESSAGE_BYTES {
            tracing::warn!(
                message_id = %id,
                declared = chunk.header.original_size,
                limit = MAX_MESSAGE_BYTES,
                "Rejected chunk declaring an oversized message"
            );
            return None;
        }

        if total == 0 || total > MAX_PENDING_CHUNKS {
            return None;
        }

        // A mismatched total_chunks must NOT silently drop the chunk.
        //
        // Previously the entry was created from whichever chunk arrived first,
        // and a later chunk with a different `total` hit `entry.chunks.len() !=
        // total` and returned None — leaving an entry that could never complete.
        // One bogus chunk could therefore poison a message id permanently. Now
        // the entry is re-sized when a later chunk disagrees, because
        // `total_chunks` is not trustworthy from any single sender.
        let needs_resize = self
            .pending
            .get(&id)
            .map(|e| e.chunks.len() != total)
            .unwrap_or(false);

        let entry = self
            .pending
            .entry(id)
            .or_insert_with(|| PendingReassembly {
                chunks: vec![None; total],
                total_chunks: total as u32,
                original_size: chunk.header.original_size,
                first_seen: Instant::now(),
            });

        if needs_resize {
            entry.chunks.resize(total, None);
            entry.total_chunks = total as u32;
        }

        // Keep the largest declared size seen, but only if it stays within the
        // aggregate ceiling, so a peer cannot shrink our expectation and make a
        // short reassembly look complete.
        entry.original_size = entry.original_size.max(chunk.header.original_size);

        if idx < entry.chunks.len() {
            entry.chunks[idx] = Some(chunk.payload);
        }

        // Only assemble once we are actually complete, and verify the length
        // against what the sender declared. `original_size` is attacker-
        // controlled, so it is a cross-check, never the source of truth.
        if entry.chunks.iter().all(|c| c.is_some()) {
            let reassembly = self.pending.remove(&id)?;
            let data: Vec<u8> = reassembly
                .chunks
                .into_iter()
                .filter_map(|c| c)
                .flatten()
                .collect();
            if data.len() != reassembly.original_size {
                tracing::warn!(
                    message_id = %id,
                    actual = data.len(),
                    declared = reassembly.original_size,
                    "Discarding reassembly whose length disagrees with the declared size"
                );
                return None;
            }
            Some(data)
        } else {
            None
        }
    }

    pub fn cleanup_expired(&mut self) {
        self.pending
            .retain(|_, entry| entry.first_seen.elapsed() < REASSEMBLY_TIMEOUT);
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_chunk_small_message() {
        let id = Uuid::new_v4();
        let data = vec![42u8; 100];
        let chunks = split_into_chunks(id, &data);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].header.total_chunks, 1);
        assert_eq!(chunks[0].payload, data);
    }

    #[test]
    fn test_multi_chunk_large_message() {
        let id = Uuid::new_v4();
        let data = vec![7u8; MAX_CHUNK_SIZE * 3 + 100];
        let chunks = split_into_chunks(id, &data);
        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0].header.total_chunks, 4);
        assert_eq!(chunks[3].header.chunk_index, 3);
    }

    #[test]
    fn test_reassemble_single_chunk() {
        let id = Uuid::new_v4();
        let data = vec![1u8; 50];
        let chunks = split_into_chunks(id, &data);
        let mut reassembler = ChunkReassembler::new();
        let result = reassembler.add_chunk(chunks[0].clone());
        assert_eq!(result, Some(data));
    }

    #[test]
    fn test_reassemble_multi_chunk() {
        let id = Uuid::new_v4();
        let data = vec![9u8; MAX_CHUNK_SIZE * 2 + 50];
        let chunks = split_into_chunks(id, &data);
        let mut reassembler = ChunkReassembler::new();

        let mut result = None;
        for chunk in chunks {
            result = reassembler.add_chunk(chunk);
        }
        assert_eq!(result, Some(data));
    }

    #[test]
    fn test_reassemble_out_of_order() {
        let id = Uuid::new_v4();
        let data = vec![5u8; MAX_CHUNK_SIZE * 2 + 50];
        let mut chunks = split_into_chunks(id, &data);
        chunks.reverse();
        let mut reassembler = ChunkReassembler::new();

        let mut result = None;
        for chunk in chunks {
            result = reassembler.add_chunk(chunk);
        }
        assert_eq!(result, Some(data));
    }

    #[test]
    fn test_cleanup_expired() {
        let mut reassembler = ChunkReassembler::new();
        let id = Uuid::new_v4();
        let chunk = Chunk {
            header: ChunkHeader {
                message_id: id,
                chunk_index: 0,
                total_chunks: 2,
                original_size: 100,
            },
            payload: vec![1u8; 50],
        };
        reassembler.add_chunk(chunk);
        assert_eq!(reassembler.pending_count(), 1);
        reassembler.cleanup_expired();
    }
}
