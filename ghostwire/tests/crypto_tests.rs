//! Unit tests for cryptographic operations in GhostWire
//!
//! These tests verify the core cryptographic functionality including:
//! - Key generation and derivation
//! - AES-256-GCM encryption/decryption
//! - Ed25519 signatures
//! - X25519 key exchange
//! - Session key management

use ghostwire::core::encryption::{EncryptionManager, KeyType, MessageType};

/// Test that encryption manager initializes correctly
#[test]
fn test_encryption_manager_initialization() {
    let manager = EncryptionManager::new();
    assert!(
        manager.is_ok(),
        "EncryptionManager should initialize successfully"
    );

    let mgr = manager.unwrap();
    assert!(
        !mgr.get_public_key().is_empty(),
        "Public key should not be empty"
    );
    assert!(!mgr.get_key_id().is_empty(), "Key ID should not be empty");
}

/// Test that two different managers generate different key pairs
#[test]
fn test_unique_key_generation() {
    let mgr1 = EncryptionManager::new().unwrap();
    let mgr2 = EncryptionManager::new().unwrap();

    assert_ne!(
        mgr1.get_public_key(),
        mgr2.get_public_key(),
        "Different managers should have different public keys"
    );

    assert_ne!(
        mgr1.get_key_id(),
        mgr2.get_key_id(),
        "Different managers should have different key IDs"
    );
}

/// Test hybrid key format (Ed25519 + X25519)
#[test]
fn test_hybrid_key_format() {
    let mgr = EncryptionManager::new().unwrap();
    let pub_key = mgr.get_public_key();

    // Hybrid public key should be 64 bytes: 32 for Ed25519 + 32 for X25519
    assert_eq!(pub_key.len(), 64, "Hybrid public key should be 64 bytes");
}

/// Test message signing and verification
#[test]
fn test_message_signing() {
    let mgr = EncryptionManager::new().unwrap();
    let message = b"Test message for signing";

    let signature = mgr.sign_message(message);
    assert!(signature.is_ok(), "Signing should succeed");

    let sig = signature.unwrap();
    assert!(!sig.is_empty(), "Signature should not be empty");

    // Verify our own signature
    let verify_result = mgr.verify_signature(message, &sig, mgr.get_public_key());
    assert!(
        verify_result.is_ok(),
        "Signature verification should succeed"
    );
}

/// Test that signature verification fails for tampered messages
#[test]
fn test_signature_tamper_detection() {
    let mgr = EncryptionManager::new().unwrap();
    let message = b"Original message";
    let tampered = b"Tampered message";

    let signature = mgr.sign_message(message).unwrap();

    // Verification should fail for tampered message
    let result = mgr.verify_signature(tampered, &signature, mgr.get_public_key());
    assert!(result.is_err(), "Tampered message should fail verification");
}

/// Test symmetric CLI encryption/decryption
#[test]
fn test_cli_encryption_roundtrip() {
    let mgr = EncryptionManager::new().unwrap();
    let plaintext = b"Secret CLI data that needs protection";

    let encrypted = mgr.encrypt(plaintext);
    assert!(!encrypted.is_empty(), "Encrypted data should not be empty");
    assert_ne!(
        encrypted.as_slice(),
        plaintext,
        "Encrypted should differ from plaintext"
    );

    let decrypted = mgr.decrypt(&encrypted);
    assert_eq!(
        decrypted.as_slice(),
        plaintext,
        "Decrypted should match original"
    );
}

/// Test encryption with empty data
#[test]
fn test_empty_data_encryption() {
    let mgr = EncryptionManager::new().unwrap();
    let plaintext = b"";

    let encrypted = mgr.encrypt(plaintext);
    let decrypted = mgr.decrypt(&encrypted);

    assert_eq!(
        decrypted.as_slice(),
        plaintext,
        "Empty data should round-trip correctly"
    );
}

/// Test encryption with large data
#[test]
fn test_large_data_encryption() {
    let mgr = EncryptionManager::new().unwrap();
    let plaintext: Vec<u8> = (0u8..=255).cycle().take(100_000).collect();

    let encrypted = mgr.encrypt(&plaintext);
    let decrypted = mgr.decrypt(&encrypted);

    assert_eq!(
        decrypted, plaintext,
        "Large data should round-trip correctly"
    );
}

/// Test peer key management
#[test]
fn test_peer_key_management() {
    let mgr1 = EncryptionManager::new().unwrap();
    let mgr2 = EncryptionManager::new().unwrap();

    assert_eq!(mgr1.get_peer_count(), 0, "Initial peer count should be 0");

    mgr1.add_peer_key(
        mgr2.get_key_id(),
        mgr2.get_public_key().to_vec(),
        KeyType::Hybrid,
    );

    assert_eq!(
        mgr1.get_peer_count(),
        1,
        "Peer count should be 1 after adding"
    );
}

/// Test key rotation detection
#[test]
fn test_key_rotation_detection() {
    let mgr = EncryptionManager::new().unwrap();

    // New key should not need rotation immediately
    assert!(
        !mgr.should_rotate_keys(),
        "Fresh keys should not need rotation"
    );
}

/// Test cross-peer encryption and decryption
#[test]
fn test_cross_peer_encryption() {
    let alice = EncryptionManager::new().unwrap();
    let bob = EncryptionManager::new().unwrap();

    // Exchange keys
    alice.add_peer_key(
        bob.get_key_id(),
        bob.get_public_key().to_vec(),
        KeyType::Hybrid,
    );
    bob.add_peer_key(
        alice.get_key_id(),
        alice.get_public_key().to_vec(),
        KeyType::Hybrid,
    );

    let message = "Hello, Bob! This is a secure message from Alice.";

    // Alice encrypts for Bob
    let encrypted = alice
        .encrypt_message(message, &bob.get_key_id(), MessageType::Text)
        .expect("Encryption should succeed");

    assert!(!encrypted.ciphertext.is_empty());
    assert!(!encrypted.nonce.is_empty());
    assert!(!encrypted.signature.is_empty());

    // Bob decrypts
    let decrypted = bob
        .decrypt_message(&encrypted)
        .expect("Decryption should succeed");

    assert_eq!(decrypted, message);
}

/// Test message types are preserved
#[test]
fn test_message_type_preservation() {
    let alice = EncryptionManager::new().unwrap();
    let bob = EncryptionManager::new().unwrap();

    alice.add_peer_key(
        bob.get_key_id(),
        bob.get_public_key().to_vec(),
        KeyType::Hybrid,
    );
    bob.add_peer_key(
        alice.get_key_id(),
        alice.get_public_key().to_vec(),
        KeyType::Hybrid,
    );

    for msg_type in [
        MessageType::Text,
        MessageType::Binary,
        MessageType::Control,
        MessageType::Heartbeat,
    ] {
        let encrypted = alice
            .encrypt_message("test", &bob.get_key_id(), msg_type.clone())
            .unwrap();

        assert_eq!(
            encrypted.message_type, msg_type,
            "Message type should be preserved"
        );
    }
}

/// Test public key export
#[test]
fn test_public_key_export() {
    let mgr = EncryptionManager::new().unwrap();
    let exported = mgr.export_public_key();

    assert_eq!(exported, mgr.get_public_key(), "Exported key should match");
    assert_eq!(exported.len(), 64, "Exported key should be 64 bytes");
}
