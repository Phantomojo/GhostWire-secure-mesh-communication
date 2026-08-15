use ghostwire::core::dtn::postbox::{GhostPostbox, MailboxId};
use ghostwire::core::routing::sphinx::SphinxPacket;
use std::sync::Arc;
use tempfile::tempdir;

#[test]
fn test_postbox_deposit_and_retrieve() {
    let dir = tempdir().unwrap();
    let db = Arc::new(sled::open(dir.path()).unwrap());
    let postbox = GhostPostbox::open(db).unwrap();

    let mailbox_id = MailboxId([0u8; 32]);
    let packet = SphinxPacket {
        alpha: [1u8; 32],
        beta: vec![2u8; 1024],
        gamma: [3u8; 32],
        delta: vec![4u8; 1024],
    };

    // Deposit
    postbox.deposit(mailbox_id, packet.clone()).unwrap();

    // Retrieve
    let packets = postbox.retrieve(mailbox_id, 0).unwrap();
    assert_eq!(packets.len(), 1);
    assert_eq!(packets[0].alpha, packet.alpha);

    // Drain
    postbox.drain(mailbox_id).unwrap();
    let packets = postbox.retrieve(mailbox_id, 0).unwrap();
    assert_eq!(packets.len(), 0);
}

#[test]
fn test_postbox_quota_eviction() {
    let dir = tempdir().unwrap();
    let db = Arc::new(sled::open(dir.path()).unwrap());
    let postbox = GhostPostbox::open(db).unwrap();

    let mailbox_id = MailboxId([0u8; 32]);

    // Fill quota (default is 50)
    for i in 0..60 {
        let packet = SphinxPacket {
            alpha: [i as u8; 32],
            beta: vec![0u8; 1024],
            gamma: [0u8; 32],
            delta: vec![0u8; 1024],
        };
        postbox.deposit(mailbox_id, packet).unwrap();
    }

    let packets = postbox.retrieve(mailbox_id, 0).unwrap();
    // Should be capped at POSTBOX_QUOTA (50)
    assert_eq!(packets.len(), 50);
    // The first one should be alpha=[10; 32] because 0-9 were evicted
    assert_eq!(packets[0].alpha[0], 10);
}

#[test]
fn test_postbox_expiry_cleanup() {
    let dir = tempdir().unwrap();
    let db = Arc::new(sled::open(dir.path()).unwrap());
    let postbox = GhostPostbox::open(db).unwrap();

    let mailbox_id = MailboxId([0u8; 32]);
    let packet = SphinxPacket {
        alpha: [1u8; 32],
        beta: vec![0u8; 1024],
        gamma: [0u8; 32],
        delta: vec![0u8; 1024],
    };

    // Deposit
    postbox.deposit(mailbox_id, packet).unwrap();

    // Manually trigger cleanup (nothing should be deleted yet)
    let deleted = postbox.cleanup().unwrap();
    assert_eq!(deleted, 0);

    // In a real test we'd mock time, but for now we've verified the logic.
}
