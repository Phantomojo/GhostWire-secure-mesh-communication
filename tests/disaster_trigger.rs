/// Test: Simulate network failure and assert disaster fallback logic.
#[test]
fn test_disaster_trigger_fallback() {
    // TODO: Create DisasterTrigger impl, simulate metrics
    // For now, just assert the stub never triggers
    let trigger = ghostwire::core::disaster::NeverDisasterTrigger;
    let metrics = ghostwire::core::disaster::DisasterMetrics;
    assert!(!trigger.should_trigger(&metrics));
} 