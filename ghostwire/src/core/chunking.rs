use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

pub const MAX_CHUNK_SIZE: usize = 60 * 1024;
const REASSEMBLY_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_PENDING_CHUNKS: usize = 64;

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
    if data.len() <= MAX_CHUNK_SIZE {
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

    let total = data.len().div_ceil(MAX_CHUNK_SIZE);
    data.chunks(MAX_CHUNK_SIZE)
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

        if total > MAX_PENDING_CHUNKS {
            return None;
        }

        let entry = self.pending.entry(id).or_insert_with(|| PendingReassembly {
            chunks: vec![None; total],
            total_chunks: total as u32,
            original_size: chunk.header.original_size,
            first_seen: Instant::now(),
        });

        if entry.chunks.len() != total {
            return None;
        }

        if idx < total {
            entry.chunks[idx] = Some(chunk.payload);
        }

        if entry.chunks.iter().all(|c| c.is_some()) {
            let reassembly = self.pending.remove(&id)?;
            let data: Vec<u8> = reassembly
                .chunks
                .into_iter()
                .filter_map(|c| c)
                .flatten()
                .collect();
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
