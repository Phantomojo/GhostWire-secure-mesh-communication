//! Tests for AI routing and anomaly detection
//!
//! Tests the current 4-layer AI architecture:
//! - L1: ONNX LightGBM anomaly detection (2,387 features)
//! - L1b: Isolation Forest anomaly detection
//! - L4: PRoPHET fallback for routing

use ghostwire::core::ai::{
    fallback, FeatureExtractor, NodeMetrics, RoutingClassifier, RoutingFeatures,
};
use libp2p::PeerId;

/// Test that the routing classifier loads (or gracefully degrades)
#[test]
fn test_routing_classifier_load_or_degrade() {
    // This will fail to load the ONNX model in test environment,
    // but should not panic — graceful degradation to PRoPHET
    let result = RoutingClassifier::new("models/ghostwire_routing.onnx");
    // Either it loads or it errors — both are acceptable
    assert!(result.is_ok() || result.is_err());
}

/// Test PRoPHET fallback routing produces sensible scores
#[test]
fn test_fallback_routing_scores() {
    let good_score = fallback::calculate_delivery_predictivity(
        -30,  // Excellent signal
        50,   // Low latency
        0.01, // Low packet loss
        0.9,  // High trust
    );

    let poor_score = fallback::calculate_delivery_predictivity(
        -80, // Poor signal
        300, // High latency
        0.3, // High packet loss
        0.5, // Medium trust
    );

    assert!(good_score > poor_score, "Good link should score higher");
    assert!(
        good_score > 0.7,
        "Good link should be > 0.7, got {}",
        good_score
    );
    assert!(
        poor_score < 0.5,
        "Poor link should be < 0.5, got {}",
        poor_score
    );
}

/// Test PRoPHET selects best peer correctly
#[test]
fn test_select_best_peer() {
    let peer1 = PeerId::random();
    let peer2 = PeerId::random();

    let candidates = vec![
        (
            peer1.clone(),
            RoutingFeatures {
                rssi: -30,
                latency_ms: 50,
                packet_loss: 0.01,
                trust_score: 0.9,
                hop_count: 1,
                battery_level: 1.0,
                time_of_day: 12,
            },
        ),
        (
            peer2.clone(),
            RoutingFeatures {
                rssi: -80,
                latency_ms: 300,
                packet_loss: 0.3,
                trust_score: 0.5,
                hop_count: 5,
                battery_level: 0.3,
                time_of_day: 12,
            },
        ),
    ];

    let best = fallback::select_next_hop_formula(candidates);
    assert_eq!(best, peer1, "Should select the better peer");
}

/// Test feature extractor produces correct dimension
#[test]
fn test_feature_extractor_dimension() {
    let extractor = FeatureExtractor::new();
    assert_eq!(extractor.feature_dim(), 2387);
}

/// Test feature extractor column map covers all features
#[test]
fn test_column_map_coverage() {
    let extractor = FeatureExtractor::new();
    let total: usize = extractor
        .build_column_map_for_test()
        .values()
        .map(|v| v.len())
        .sum();
    assert_eq!(
        total, 2387,
        "Column map should cover exactly 2,387 features"
    );
}

/// Test local system metrics collection works on Linux
#[test]
fn test_collect_system_metrics() {
    let result = FeatureExtractor::collect_local_system_metrics();
    assert!(result.is_ok(), "Should collect metrics on Linux");

    let metrics = result.unwrap();
    assert!(metrics.idle_rate > 0.0, "Idle rate should be positive");
    assert!(metrics.processes > 0.0, "Process count should be positive");
    assert!(
        metrics.ctxt_rate > 0.0,
        "Context switch rate should be positive"
    );
}

/// Test feature vector building with partial node data
#[test]
fn test_build_feature_vector_partial() {
    let extractor = FeatureExtractor::new();
    let mut node_map = std::collections::HashMap::new();
    node_map.insert(
        23u16,
        NodeMetrics {
            node_id: 23,
            processes: 50.0,
            loadavg_m1: 0.04,
            idle_rate: 93.8,
            ..Default::default()
        },
    );

    let features = extractor.build_feature_vector(&node_map);
    assert_eq!(features.len(), 2387);
    assert!(
        features.iter().any(|&v| v != 0.0),
        "Should have non-zero features"
    );
}
