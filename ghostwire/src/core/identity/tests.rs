use super::encryption::{decrypt_identity, encrypt_identity, EncryptedIdentity};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let data = b"test identity data with special chars: !@#$%^&*()";
        let password = "SecureP@ssw0rd!123";

        // Encrypt
        let encrypted = encrypt_identity(&data[..], password).unwrap();

        // Verify encrypted format
        assert_eq!(encrypted.version, 2);
        assert!(!encrypted.encrypted_data.is_empty());
        assert_eq!(encrypted.salt.len(), 32);
        assert_eq!(encrypted.nonce.len(), 12);

        // Decrypt
        let decrypted = decrypt_identity(&encrypted, password).unwrap();

        // Verify data matches
        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_wrong_password_fails() {
        let data = b"secret data";
        let correct_password = "correct123";
        let wrong_password = "wrong456";

        let encrypted = encrypt_identity(&data[..], correct_password).unwrap();
        let result = decrypt_identity(&encrypted, wrong_password);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Decryption failed"));
    }

    #[test]
    fn test_different_salts_different_ciphertext() {
        let data = b"same data";
        let password = "same password";

        let encrypted1 = encrypt_identity(&data[..], password).unwrap();
        let encrypted2 = encrypt_identity(&data[..], password).unwrap();

        // Different salts
        assert_ne!(encrypted1.salt, encrypted2.salt);

        // Different nonces
        assert_ne!(encrypted1.nonce, encrypted2.nonce);

        // Different ciphertext
        assert_ne!(encrypted1.encrypted_data, encrypted2.encrypted_data);

        // Both decrypt correctly
        let decrypted1 = decrypt_identity(&encrypted1, password).unwrap();
        let decrypted2 = decrypt_identity(&encrypted2, password).unwrap();

        assert_eq!(data, decrypted1.as_slice());
        assert_eq!(data, decrypted2.as_slice());
    }

    #[test]
    fn test_empty_data() {
        let data = b"";
        let password = "password";

        let encrypted = encrypt_identity(&data[..], password).unwrap();
        let decrypted = decrypt_identity(&encrypted, password).unwrap();

        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_large_data() {
        // 1MB of data
        let data = vec![42u8; 1_000_000];
        let password = "bigdata";

        let encrypted = encrypt_identity(&data[..], password).unwrap();
        let decrypted = decrypt_identity(&encrypted, password).unwrap();

        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_various_password_lengths() {
        let data = b"test";

        let passwords = vec![
            "a",                                                     // 1 char
            "ab",                                                    // 2 chars
            "short",                                                 // 5 chars
            "medium_length_password",                                // 22 chars
            "very_long_password_with_lots_of_characters_0123456789", // 54 chars
        ];

        for password in passwords {
            let encrypted = encrypt_identity(&data[..], password).unwrap();
            let decrypted = decrypt_identity(&encrypted, password).unwrap();
            assert_eq!(
                data,
                decrypted.as_slice(),
                "Failed for password: {}",
                password
            );
        }
    }

    #[test]
    fn test_unicode_password() {
        let data = b"sensitive data";
        let password = "пароль密码🔐"; // Russian + Chinese + Emoji

        let encrypted = encrypt_identity(&data[..], password).unwrap();
        let decrypted = decrypt_identity(&encrypted, password).unwrap();

        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_corrupted_ciphertext() {
        let data = b"test data";
        let password = "password";

        let mut encrypted = encrypt_identity(&data[..], password).unwrap();

        // Corrupt the ciphertext
        encrypted.encrypted_data[0] ^= 0xFF;

        let result = decrypt_identity(&encrypted, password);
        assert!(result.is_err());
    }

    #[test]
    fn test_corrupted_salt() {
        let data = b"test data";
        let password = "password";

        let mut encrypted = encrypt_identity(&data[..], password).unwrap();

        // Corrupt the salt (should cause wrong key derivation)
        encrypted.salt[0] ^= 0xFF;

        let result = decrypt_identity(&encrypted, password);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization() {
        let data = b"test data";
        let password = "password";

        let encrypted = encrypt_identity(&data[..], password).unwrap();

        // Serialize to JSON
        let json = serde_json::to_string(&encrypted).unwrap();

        // Deserialize
        let deserialized: EncryptedIdentity = serde_json::from_str(&json).unwrap();

        // Decrypt deserialized
        let decrypted = decrypt_identity(&deserialized, password).unwrap();

        assert_eq!(data, decrypted.as_slice());
    }
}
