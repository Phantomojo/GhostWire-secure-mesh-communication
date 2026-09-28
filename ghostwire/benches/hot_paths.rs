//! Criterion micro-benchmarks for GhostWire Rust-core hot paths.
//!
//! Covers: XChaCha20-Poly1305 crypto round-trip, frame encode/decode,
//! HLC timestamps, message deduplication, and mailbox seal/open.
//!
//! All benchmarks are pure in-memory — no network, no ports, no spawning nodes.
//! Run: `cargo bench --bench hot_paths`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use std::time::Duration;

// ── 1. XChaCha20-Poly1305 encrypt / decrypt ──────────────────────────────────

fn bench_xchacha20_roundtrip(c: &mut Criterion) {
    let key = [0x42u8; 32];

    // 64-byte payload (typical small control message)
    let small = vec![0xABu8; 64];
    // 1 KiB payload
    let medium = vec![0xCDu8; 1024];
    // 64 KiB payload (max frame size)
    let large = vec![0xEFu8; 65_536];

    let mut g = c.benchmark_group("xchacha20_poly1305");

    g.bench_function("encrypt_64B", |b| {
        b.iter(|| {
            ghostwire::core::encryption::EncryptionManager::encrypt_with_xchacha20(
                black_box(&small),
                &key,
            )
            .unwrap()
        })
    });

    g.bench_function("decrypt_64B", |b| {
        let enc =
            ghostwire::core::encryption::EncryptionManager::encrypt_with_xchacha20(&small, &key)
                .unwrap();
        b.iter(|| {
            ghostwire::core::encryption::EncryptionManager::decrypt_with_xchacha20(
                black_box(&enc.ciphertext),
                &enc.nonce,
                &key,
            )
            .unwrap()
        })
    });

    g.bench_function("roundtrip_1K", |b| {
        b.iter(|| {
            let enc = ghostwire::core::encryption::EncryptionManager::encrypt_with_xchacha20(
                black_box(&medium),
                &key,
            )
            .unwrap();
            ghostwire::core::encryption::EncryptionManager::decrypt_with_xchacha20(
                &enc.ciphertext,
                &enc.nonce,
                &key,
            )
            .unwrap()
        })
    });

    g.bench_function("roundtrip_64K", |b| {
        b.iter(|| {
            let enc = ghostwire::core::encryption::EncryptionManager::encrypt_with_xchacha20(
                black_box(&large),
                &key,
            )
            .unwrap();
            ghostwire::core::encryption::EncryptionManager::decrypt_with_xchacha20(
                &enc.ciphertext,
                &enc.nonce,
                &key,
            )
            .unwrap()
        })
    });

    g.finish();
}

// ── 2. Frame encode / decode (bincode + wire format) ─────────────────────────

fn bench_frame_codec(c: &mut Criterion) {
    use ghostwire::core::wire_protocol::{Frame, FrameKind, WireHandshake, WireMessage};

    let mut g = c.benchmark_group("frame_codec");

    // Small frame: Handshake
    let handshake = WireHandshake::new(vec![0xEE; 32], 0b0011);

    g.bench_function("encode_handshake", |b| {
        b.iter(|| {
            let frame = Frame::encode(FrameKind::Handshake, black_box(&handshake)).unwrap();
            frame.to_bytes()
        })
    });

    let handshake_bytes = Frame::encode(FrameKind::Handshake, &handshake)
        .unwrap()
        .to_bytes();

    g.bench_function("decode_handshake", |b| {
        b.iter(|| {
            let (frame, _consumed) = Frame::from_bytes(black_box(&handshake_bytes)).unwrap();
            let _msg: WireHandshake = frame.decode().unwrap();
        })
    });

    // Large frame: Message with 512-byte payload
    let msg = WireMessage {
        from: vec![0xAA; 32],
        to: vec![0xBB; 32],
        payload: vec![0xCC; 512],
        id: vec![0xDD; 16],
        ts: 1_700_000_000u32,
        seq: 42,
    };

    g.bench_function("encode_message_512B", |b| {
        b.iter(|| {
            let frame = Frame::encode(FrameKind::Message, black_box(&msg)).unwrap();
            frame.to_bytes()
        })
    });

    let msg_bytes = Frame::encode(FrameKind::Message, &msg).unwrap().to_bytes();

    g.bench_function("decode_message_512B", |b| {
        b.iter(|| {
            let (frame, _consumed) = Frame::from_bytes(black_box(&msg_bytes)).unwrap();
            let _msg: WireMessage = frame.decode().unwrap();
        })
    });

    // Full roundtrip
    g.bench_function("roundtrip_handshake", |b| {
        b.iter(|| {
            let frame = Frame::encode(FrameKind::Handshake, black_box(&handshake)).unwrap();
            let bytes = frame.to_bytes();
            let (parsed, _consumed) = Frame::from_bytes(&bytes).unwrap();
            let _recovered: WireHandshake = parsed.decode().unwrap();
        })
    });

    g.finish();
}

// ── 3. Hybrid Logical Clock (HLC) send / receive ─────────────────────────────
//
// Uses hlc-rs crate directly (the external dependency, not the dead file
// in src/core/hlc.rs which is not declared in mod.rs).

fn bench_hlc(c: &mut Criterion) {
    let mut g = c.benchmark_group("hlc");

    g.bench_function("send", |b| {
        let clock = hlc_rs::HybridLogicalClock::new(1);
        b.iter(|| {
            black_box(clock.send());
        })
    });

    g.bench_function("receive", |b| {
        let clock = hlc_rs::HybridLogicalClock::new(1);
        let remote = hlc_rs::HlcTimestamp {
            physical_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
                + 1000,
            logical_counter: 5,
            node_id: 2,
        };
        b.iter(|| {
            black_box(clock.receive(&remote));
        })
    });

    g.bench_function("send_receive_cycle", |b| {
        let clock_a = hlc_rs::HybridLogicalClock::new(1);
        let clock_b = hlc_rs::HybridLogicalClock::new(2);
        b.iter(|| {
            let ts_a = clock_a.send();
            let _ts_b = clock_b.receive(&ts_a);
            let ts_b_reply = clock_b.send();
            black_box(clock_a.receive(&ts_b_reply));
        })
    });

    g.finish();
}

// ── 4. Message deduplication (DashMap-based, O(1) lookup) ─────────────────────

fn bench_dedup(c: &mut Criterion) {
    use ghostwire::core::micro_optimizations::MessageDeduplicator;

    let mut g = c.benchmark_group("message_dedup");

    g.bench_function("is_duplicate_new", |b| {
        let dedup = MessageDeduplicator::new(Duration::from_secs(60));
        let mut counter = 0u64;
        b.iter(|| {
            counter += 1;
            let id = format!("msg_{counter:016x}");
            black_box(dedup.is_duplicate(&id));
        })
    });

    g.bench_function("is_duplicate_seen", |b| {
        let dedup = MessageDeduplicator::new(Duration::from_secs(60));
        // Pre-populate
        for i in 0..10_000 {
            dedup.is_duplicate(&format!("pre_{i:08x}"));
        }
        b.iter(|| {
            black_box(dedup.is_duplicate(black_box("pre_00000001")));
        })
    });

    g.finish();
}

// ── 5. Mailbox store: seal + open roundtrip (ECDH + ChaCha20Poly1305) ────────

fn bench_mailbox(c: &mut Criterion) {
    use ghostwire::core::mailbox::MailboxStore;
    use ghostwire::core::persistence_manager::PersistenceManager;
    use rand_core::OsRng;
    use x25519_dalek::{PublicKey, StaticSecret};

    let mut g = c.benchmark_group("mailbox");

    // Set up store with temp dir + sled temporary DB
    let home = tempfile::tempdir().unwrap();
    let persistence = Arc::new(
        PersistenceManager::new_with_base_dir_and_passphrase(
            home.path().join("vault"),
            Some("bench".to_string()),
            "bench-passphrase",
        )
        .unwrap(),
    );
    let db = Arc::new(sled::Config::new().temporary(true).open().unwrap());
    let store = MailboxStore::open(db, persistence).unwrap();

    // Pre-generate recipient key pair
    let recipient_secret = StaticSecret::random_from_rng(OsRng);
    let recipient_pub: PublicKey = (&recipient_secret).into();

    // Plaintext sizes
    let small_plaintext = vec![0x42u8; 64];
    let medium_plaintext = vec![0x42u8; 1024];

    g.bench_function("seal_64B", |b| {
        b.iter(|| {
            store
                .seal_for_peer(
                    "bench-peer",
                    recipient_pub.as_bytes(),
                    black_box(&small_plaintext),
                )
                .unwrap()
        })
    });

    g.bench_function("seal_1K", |b| {
        b.iter(|| {
            store
                .seal_for_peer(
                    "bench-peer",
                    recipient_pub.as_bytes(),
                    black_box(&medium_plaintext),
                )
                .unwrap()
        })
    });

    // For open_envelope, we need a pre-sealed envelope
    let envelope = store
        .seal_for_peer("bench-peer", recipient_pub.as_bytes(), &small_plaintext)
        .unwrap();

    g.bench_function("open_envelope_64B", |b| {
        b.iter(|| {
            store
                .open_envelope(black_box(&envelope), &recipient_secret)
                .unwrap()
        })
    });

    // Full seal + open roundtrip
    g.bench_function("roundtrip_64B", |b| {
        b.iter(|| {
            let env = store
                .seal_for_peer(
                    "bench-peer",
                    recipient_pub.as_bytes(),
                    black_box(&small_plaintext),
                )
                .unwrap();
            store.open_envelope(&env, &recipient_secret).unwrap()
        })
    });

    g.finish();
}

/// ML-KEM-768 post-quantum key establishment.
///
/// The Bible previously quoted an 890 microsecond on-device encapsulation figure
/// that had no benchmark behind it anywhere in the repo. This measures the real
/// cost of the three operations that make up the hybrid handshake, so the claim
/// can cite a reproducible number instead.
fn bench_mlkem768(c: &mut Criterion) {
    use pqcrypto_mlkem::mlkem768;

    let mut g = c.benchmark_group("mlkem768");

    g.bench_function("keypair", |b| {
        b.iter(|| black_box(mlkem768::keypair()));
    });

    let (pk, sk) = mlkem768::keypair();

    g.bench_function("encapsulate", |b| {
        b.iter(|| black_box(mlkem768::encapsulate(&pk)));
    });

    let (shared, ct) = mlkem768::encapsulate(&pk);

    g.bench_function("decapsulate", |b| {
        b.iter(|| black_box(mlkem768::decapsulate(&ct, &sk)));
    });

    // Sanity: encapsulating and decapsulating the same ciphertext must recover the
    // same shared secret, or the timings above are meaningless.
    let recovered = mlkem768::decapsulate(&ct, &sk);
    assert_eq!(shared, recovered, "PQ round-trip sanity check failed");

    g.finish();
}

criterion_group!(
    benches,
    bench_xchacha20_roundtrip,
    bench_frame_codec,
    bench_hlc,
    bench_dedup,
    bench_mailbox,
    bench_mlkem768,
);
criterion_main!(benches);
