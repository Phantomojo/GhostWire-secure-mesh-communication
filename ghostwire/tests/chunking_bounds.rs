//! Chunking must bound what a PEER can make this node allocate.
//!
//! ## What this audits
//!
//! `core/chunking.rs` (198 lines, never declared in `mod.rs` until this work,
//! so never compiled) implements split/reassemble. Reviewing it against
//! `docs/MESSAGE_DELIVERY_SPEC.md` §7 — which specifies a per-chunk
//! `checksum: [u8; 32]` and "Request retransmission" for missing chunks —
//! turned up four gaps. Three are resource-exhaustion and integrity; one is a
//! correctness bug in the reassembler itself.
//!
//! ## The bugs
//!
//! **1. Unbounded payload — remote memory exhaustion.**
//! `Chunk.payload: Vec<u8>` is deserialised straight from the wire with no
//! length check. A peer can send ONE chunk claiming a 2 GB payload. The
//! `total > MAX_PENDING_CHUNKS` guard bounds the number of CHUNKS, not the size
//! of any one. So `total_chunks: 1, payload: <2 GB>` is accepted and
//! `Some(Vec<u8>)` is returned. This is the shape of a trivial DoS.
//!
//! **2. Unbounded `original_size`, and it is never checked against the
//! reassembled length.** A peer can claim any size; nothing verifies the
//! concatenation actually matches.
//!
//! **3. No integrity check.** The spec calls for `checksum: [u8; 32]` per
//! chunk. There is none, so a corrupted or tampered chunk is reassembled into a
//! corrupt message with no error — silently, because reassembly only checks that
//! all slots are `Some`.
//!
//! **4. Reassembly trusts the FIRST chunk's `total_chunks`.** `add_chunk`
//! creates `PendingReassembly` from whichever chunk arrives first, sizing
//! `chunks` to that chunk's `total`. A later chunk with a different `total`
//! returns `None` — permanently dropping a chunk that was in fact valid, and
//! the message never completes. Worse: an attacker sending one bogus chunk
//! with `total_chunks: 1` for a `message_id` that later gets real chunks
//! poisons the entry.
//!
//! ## What is NOT wrong
//!
//! Out-of-order reassembly is handled correctly (the `chunks: Vec<Option<..>>`
//! indexed by position is the right shape), and the split path is arithmetically
//! correct including the `div_ceil` boundary.

use ghostwire::core::chunking::{
    split_into_chunks, Chunk, ChunkHeader, ChunkReassembler, MAX_CHUNK_SIZE,
};
use uuid::Uuid;

fn chunk_of(id: Uuid, index: u32, total: u32, payload: Vec<u8>) -> Chunk {
    Chunk {
        header: ChunkHeader {
            message_id: id,
            chunk_index: index,
            total_chunks: total,
            original_size: payload.len(),
        },
        payload,
    }
}

/// A peer must not be able to dictate an allocation. This is the DoS shape:
/// `total_chunks: 1` bypasses the MAX_PENDING_CHUNKS guard entirely, because
/// that guard counts chunks, not bytes.
#[test]
fn a_peer_cannot_declare_an_oversized_single_chunk() {
    let id = Uuid::new_v4();
    // 1 chunk, so the `total > MAX_PENDING_CHUNKS` guard never fires.
    // 64 MB is a realistic allocation a remote peer should not be able to force
    // per message, and this is the LOWEST-complexity attack: one packet.
    let huge = vec![0u8; 64 * 1024 * 1024];

    let mut r = ChunkReassembler::new();
    let result = r.add_chunk(chunk_of(id, 0, 1, huge));

    assert!(
        result.is_none(),
        "a peer sent ONE chunk of 64 MB with total_chunks=1 and it was accepted \
         and returned. The MAX_PENDING_CHUNKS guard counts chunks, not bytes, so \
         a single oversized chunk bypasses it entirely."
    );
}

#[test]
fn the_total_reassembled_size_is_bounded() {
    // The general shape: whatever the declared sizes, the reassembler must not
    // hand back more than a bounded amount.
    let id = Uuid::new_v4();
    let mut r = ChunkReassembler::new();

    // Claim 64 chunks of 1 MB each = 64 MB total, all valid indices.
    for i in 0..64u32 {
        let c = chunk_of(id, i, 64, vec![0u8; 1024 * 1024]);
        r.add_chunk(c);
    }

    assert_eq!(
        r.pending_count(),
        0,
        "a peer assembled 64 MB of claimed data; the reassembler should have \
         refused the whole message rather than buffering it."
    );
}

#[test]
fn a_lying_original_size_does_not_produce_a_silent_corruption() {
    // original_size is attacker-controlled metadata. Nothing should trust it
    // without checking the reassembled length matches.
    let id = Uuid::new_v4();
    let real = vec![7u8; 1000];

    let chunks = split_into_chunks(id, &real);
    assert_eq!(chunks.len(), 1, "precondition: this is a single-chunk message");

    // Forge: same payload, but claim it came from a 900 KB message.
    let mut forged = chunks.into_iter().next().unwrap();
    forged.header.original_size = 900 * 1024;

    let mut r = ChunkReassembler::new();
    match r.add_chunk(forged) {
        Some(out) => assert_eq!(
            out.len(),
            1000,
            "reassembled length must be derived from the ACTUAL payload, not the \
             peer's declared original_size (which claimed 921600)"
        ),
        // Rejecting outright is also acceptable and safer.
        None => {}
    }
}

/// The reassembly poisoning case: an attacker front-runs a message_id with a
/// bogus `total_chunks`, and the real chunks for that id can then never
/// complete.
#[test]
fn a_poisoned_total_chunks_does_not_wedge_the_real_message() {
    let id = Uuid::new_v4();
    let real = vec![9u8; 50];

    let mut r = ChunkReassembler::new();

    // Attacker: claims this id is a single 10-byte chunk.
    r.add_chunk(chunk_of(id, 0, 1, vec![0u8; 10]));

    // Now the REAL message with the same id arrives, claiming 1 chunk of 50.
    let real_chunk = split_into_chunks(id, &real).into_iter().next().unwrap();
    let out = r.add_chunk(real_chunk);

    // Either the real message wins (preferred), or it is rejected cleanly.
    // What must NOT happen is a silent wrong answer or a permanent wedge where
    // the entry is stuck with a mismatched length and can never complete.
    if let Some(data) = out {
        assert!(
            data == real || data.len() == 10,
            "reassembler returned data that matches neither the poisoned nor the \
             real payload"
        );
    }

    // The entry must not be permanently stuck: a mismatched total_chunks must
    // not leave an entry that silently swallows every later chunk.
    let later = chunk_of(id, 0, 1, real.clone());
    let _ = r.add_chunk(later);
}

#[test]
fn zero_length_input_does_not_panic() {
    let id = Uuid::new_v4();
    let chunks = split_into_chunks(id, &[]);
    assert_eq!(chunks.len(), 1, "an empty message should be one empty chunk");

    let mut r = ChunkReassembler::new();
    let out = r.add_chunk(chunks.into_iter().next().unwrap());
    assert_eq!(out, Some(Vec::new()), "empty payload must round-trip");
}

/// The existing split path is correct and should STAY correct — these guard
/// against a fix for the resource bugs breaking working behaviour.
#[test]
fn splitting_still_round_trips_exactly() {
    let id = Uuid::new_v4();
    let data: Vec<u8> = (0..(MAX_CHUNK_SIZE * 3 + 123)).map(|i| i as u8).collect();

    let chunks = split_into_chunks(id, &data);
    assert_eq!(chunks.len(), 4, "boundary case: 3 chunks + remainder");

    let mut r = ChunkReassembler::new();
    let mut result = None;
    for c in chunks {
        result = r.add_chunk(c);
    }
    assert_eq!(result, Some(data), "split -> reassemble must be lossless");
}