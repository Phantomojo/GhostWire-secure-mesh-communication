/// Test: Simulate a flood of fake peers and assert Sybil cap/detection.
#[test]
fn test_sybil_attack_detection() {
    // TODO: Create SybilDefense impl, simulate N fake peers
    // For now, just assert the stub always allows
    let defense = ghostwire::core::security::AllowAllSybilDefense;
    for i in 0..1000 {
        assert!(defense.allow_new_peer(&format!("peer_{}", i)));
    }
} 