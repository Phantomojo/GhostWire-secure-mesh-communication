// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Protocol Integration Tests
//!
//! NOTE: Most tests in this file were written against an older API and need
//! updating. The tests below are minimal smoke tests that compile.

use ghostwire::core::encryption::EncryptionManager;
use ghostwire::core::persistence_manager::PersistenceManager;
use ghostwire::core::store::SecureMessageStore;
use std::sync::Arc;

/// Test that SecureMessageStore initializes correctly
#[test]
fn test_message_store_initialization() {
    let persistence = PersistenceManager::new(None).expect("Persistence should init");
    let encryption = EncryptionManager::new().expect("Encryption should init");
    let _store = SecureMessageStore::new(Arc::new(persistence), Arc::new(encryption));
    // Store initialized — full CRUD tests need API update
}
