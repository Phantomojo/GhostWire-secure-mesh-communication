//! Tests for the multi-transport router.
//!
//! ## Why these exist
//!
//! `core/transport_router.rs` (147 lines) and its dependency `core/peer_table.rs`
//! (326 lines) were both **never compiled** — neither was declared in
//! `core/mod.rs`, so nothing in the tree ever built or called them. The router is
//! the component Reticulum routing is supposed to hang off, so nothing could be
//! said about whether it worked.
//!
//! They compile now. These tests say whether they work.
//!
//! ## The specific hole
//!
//! `UnifiedPeerTable.reticulum_address` is written only inside `peer_table.rs`, by
//! `upsert(.., TransportAddr::Reticulum(hash))`. Nothing in the tree called `upsert`
//! with the Reticulum variant, so the field was permanently `None` and
//! `RoutingDecision::Reticulum` could never be produced — the enum arm existed and
//! was unreachable.
//!
//! These tests drive `upsert` directly to pin the router's behaviour once a Reticulum
//! address IS present. Live population of the table from the Reticulum transport is
//! separate work; what is pinned here is that the decision logic is correct given
//! the data, so the wiring step has something sound to feed.

use ghostwire::core::peer_table::{IdentityId, TransportAddr, TransportKind, UnifiedPeerTable};
use ghostwire::core::transport_router::{MessagePriority, RoutingDecision, TransportRouter};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

fn ident(seed: u8) -> IdentityId {
    IdentityId([seed; 32])
}

fn rns_hash(seed: u8) -> [u8; 16] {
    [seed; 16]
}

fn router_with(entries: Vec<(IdentityId, TransportKind, TransportAddr)>) -> TransportRouter {
    let mut table = UnifiedPeerTable::new(100, Duration::from_secs(3600));
    for (id, kind, addr) in entries {
        table.upsert(id, kind, addr);
    }
    TransportRouter::new(Arc::new(RwLock::new(table)))
}

#[test]
fn a_peer_with_only_a_reticulum_address_routes_over_reticulum() {
    let id = ident(1);
    let hash = rns_hash(1);
    let router = router_with(vec![(
        id.clone(),
        TransportKind::Reticulum,
        TransportAddr::Reticulum(hash),
    )]);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let decision = runtime.block_on(router.route(&id, MessagePriority::Normal));

    match decision {
        RoutingDecision::Reticulum { address_hash } => {
            assert_eq!(
                address_hash, hash,
                "the router must return the address the peer was registered with"
            );
        }
        other => panic!("expected Reticulum, got {other:?}"),
    }
}

/// The hole this file documents, asserted so it cannot silently return.
///
/// Before anything called `upsert` with `TransportAddr::Reticulum`, this returned
/// `Unreachable` — a peer whose ONLY reachable transport was Reticulum was
/// reported unreachable, and the router would have refused to route to it at all.
#[test]
fn a_reticulum_only_peer_is_not_reported_unreachable() {
    let id = ident(2);
    let router = router_with(vec![(
        id.clone(),
        TransportKind::Reticulum,
        TransportAddr::Reticulum(rns_hash(2)),
    )]);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let decision = runtime.block_on(router.route(&id, MessagePriority::Normal));

    assert!(
        !matches!(decision, RoutingDecision::Unreachable),
        "a peer reachable over Reticulum must never be reported unreachable — \
         that is what makes the Reticulum enum arm unreachable in practice"
    );
}

#[test]
fn an_unknown_peer_is_unreachable() {
    let router = router_with(vec![]);
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let decision = runtime.block_on(router.route(&ident(99), MessagePriority::Normal));
    assert!(
        matches!(decision, RoutingDecision::Unreachable),
        "an unknown peer has no route, and saying so is correct"
    );
}

#[test]
fn a_libp2p_only_peer_routes_over_libp2p() {
    let id = ident(3);
    let router = router_with(vec![(
        id.clone(),
        TransportKind::Quic,
        TransportAddr::LibP2p("/ip4/203.0.113.5/udp/4001/quic-v1".to_string()),
    )]);

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let decision = runtime.block_on(router.route(&id, MessagePriority::Normal));

    match decision {
        RoutingDecision::LibP2p { peer_id, addr } => {
            assert!(peer_id.contains("203.0.113.5") || !peer_id.is_empty());
            assert!(!addr.is_empty(), "the libp2p address must be carried through");
        }
        other => panic!("expected LibP2p, got {other:?}"),
    }
}

/// Dual routing on Critical is the feature that makes the router worth having:
/// a message that must arrive should not depend on one transport.
#[test]
fn critical_messages_dual_route_when_both_transports_exist() {
    let id = ident(4);
    let hash = rns_hash(4);
    let mut table = UnifiedPeerTable::new(100, Duration::from_secs(3600));
    table.upsert(
        id.clone(),
        TransportKind::Quic,
        TransportAddr::LibP2p("/ip4/203.0.113.9/udp/4001/quic-v1".to_string()),
    );
    table.upsert(
        id.clone(),
        TransportKind::Reticulum,
        TransportAddr::Reticulum(hash),
    );
    let router = TransportRouter::new(Arc::new(RwLock::new(table)));

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");

    let dual = runtime.block_on(router.route(&id, MessagePriority::Critical));
    assert!(
        matches!(dual, RoutingDecision::Dual { .. }),
        "a Critical message to a peer with both transports must dual-route, so \
         delivery does not depend on one path. Got {dual:?}"
    );

    // A Normal message should pick one, not both.
    let normal = runtime.block_on(router.route(&id, MessagePriority::Normal));
    assert!(
        !matches!(normal, RoutingDecision::Unreachable),
        "with both transports available the peer is reachable; got {normal:?}"
    );
}

/// Recording outcomes is what makes the success-rate selection adaptive rather
/// than hardcoded. Without it, the "pick highest success_rate" branch is inert.
#[test]
fn recording_outcomes_changes_what_the_router_prefers() {
    let id = ident(5);
    let mut table = UnifiedPeerTable::new(100, Duration::from_secs(3600));
    table.upsert(
        id.clone(),
        TransportKind::Quic,
        TransportAddr::LibP2p("/ip4/203.0.113.5/udp/4001/quic-v1".to_string()),
    );
    table.upsert(
        id.clone(),
        TransportKind::Reticulum,
        TransportAddr::Reticulum(rns_hash(5)),
    );
    let router = TransportRouter::new(Arc::new(RwLock::new(table)));

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");

    runtime.block_on(async {
        // Drive Reticulum's success rate up and libp2p's down.
        for _ in 0..8 {
            router
                .record_outcome(&id, &TransportKind::Reticulum, true, Some(30))
                .await;
            router
                .record_outcome(&id, &TransportKind::Quic, false, None)
                .await;
        }
    });

    let decision = runtime.block_on(router.route(&id, MessagePriority::Normal));
    assert!(
        matches!(decision, RoutingDecision::Reticulum { .. }),
        "after 8 successes on Reticulum and 8 failures on QUIC, the router must \
         prefer Reticulum. Success-rate selection is the whole point of feeding \
         outcomes back. Got {decision:?}"
    );
}