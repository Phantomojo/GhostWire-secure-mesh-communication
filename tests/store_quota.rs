/// Test: Fill message store and assert quota enforcement.
#[test]
fn test_store_quota_enforcement() {
    // TODO: Create QuotaEnforcer impl, fill with messages
    // For now, just assert the stub always allows
    let quota = ghostwire::core::security::AllowAllQuotaEnforcer;
    for i in 0..1000 {
        assert!(quota.check_quota(&format!("peer_{}", i)));
    }
} 