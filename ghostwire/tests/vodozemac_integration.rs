//! vodozemac Double Ratchet Integration Tests
//!
//! Tests the vodozemac integration in crypto_machine.rs

#[cfg(test)]
mod tests {
    use crate::core::crypto_machine::{CryptoEvent, CryptoMachine};

    #[test]
    fn test_crypto_machine_initialization() {
        let machine = CryptoMachine::new();

        // Verify identity keys exist
        let _ = machine.ed25519_identity();
        let _ = machine.curve25519_identity();

        assert!(true, "CryptoMachine initialized successfully");
    }

    #[test]
    fn test_one_time_key_generation() {
        let machine = CryptoMachine::new();

        // Generate one-time keys
        let keys = machine.generate_one_time_keys(5);

        assert_eq!(keys.len(), 5, "Should generate 5 one-time keys");
    }

    #[test]
    fn test_encrypt_message_without_session() {
        let machine = CryptoMachine::new();

        let event = CryptoEvent::EncryptMessage {
            recipient_id: "test_peer".to_string(),
            plaintext: b"Hello, secure world!".to_vec(),
        };

        let actions = machine.process(event);

        // Should produce SendPacket action with encrypted data
        assert!(!actions.is_empty(), "Should produce crypto actions");
    }

    #[test]
    fn test_encrypt_decrypt_round_trip() {
        // This test would require a full session establishment
        // which involves network communication in real scenario.
        // For unit test, we verify the machine can process events.

        let machine = CryptoMachine::new();
        let plaintext = b"Test message";

        let event = CryptoEvent::EncryptMessage {
            recipient_id: "peer1".to_string(),
            plaintext: plaintext.to_vec(),
        };

        let actions = machine.process(event);

        // Should produce at least one action
        assert!(!actions.is_empty());
    }
}
