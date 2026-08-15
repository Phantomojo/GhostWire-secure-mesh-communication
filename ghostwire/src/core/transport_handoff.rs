//! Secure inter-carrier protected-envelope handoff planning.
//!
//! GhostWire is a switchboard: it accepts a **previously validated, still
//! protected** GhostWire envelope on one carrier and emits that same envelope on
//! another carrier. It converts carrier framing and per-leg behavior, never user
//! plaintext. This module plans the next leg; it intentionally has no socket,
//! RNS, BLE, radio, or storage handle.
//!
//! Before a caller constructs a [`HandoffRequest`], shared ingress must already
//! have verified the envelope's authentication, CRL/trust state, expiry, replay
//! status, and causal-ordering eligibility. The current Core has not yet wired
//! all ingress paths through that single pipeline, so this planner must not be
//! used to claim universal runtime handoff support.

use crate::core::fragmentation::{
    plan as plan_fragmentation, FragmentationError, FragmentationPlan,
};
use crate::core::transport_policy::{
    DeliveryRequirement, TransportCapability, TransportCoordinator, TransportSelectionError,
};
use std::fmt;

/// Stable end-to-end GhostWire envelope identifier. Carrier-level receipt IDs
/// must remain per-leg and must not replace this identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProtectedEnvelopeId(pub [u8; 32]);

/// A validated protected envelope awaiting a cross-carrier handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandoffRequest {
    pub envelope_id: ProtectedEnvelopeId,
    /// Length after GhostWire message-level encryption and authentication. It
    /// deliberately excludes user plaintext and carrier framing overhead.
    pub protected_object_bytes: usize,
    /// Immutable privacy/delivery policy declared by the originating operation.
    /// This must require an explicit relay for a carrier-changing handoff.
    pub requirement: DeliveryRequirement,
}

/// Selected next leg and its only permissible object-transfer outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandoffPlan<'a> {
    pub envelope_id: ProtectedEnvelopeId,
    pub source_carrier: &'a TransportCapability,
    pub next_carrier: &'a TransportCapability,
    pub protected_object_bytes: usize,
    pub fragmentation: FragmentationPlan,
}

/// A planner failure must be exposed as an explicit operation failure or a
/// safe encrypted store-and-forward decision. It must not trigger an implicit
/// carrier retry with a weaker privacy requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HandoffError {
    SourceCarrierIneligible,
    CarrierDidNotChange,
    RelayWasNotRequired,
    NoEligibleDestination(TransportSelectionError),
    Fragmentation(FragmentationError),
}

impl fmt::Display for HandoffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceCarrierIneligible => write!(f, "HANDOFF_SOURCE_CARRIER_INELIGIBLE"),
            Self::CarrierDidNotChange => write!(f, "HANDOFF_CARRIER_DID_NOT_CHANGE"),
            Self::RelayWasNotRequired => write!(f, "HANDOFF_RELAY_REQUIREMENT_MISSING"),
            Self::NoEligibleDestination(error) => {
                write!(f, "HANDOFF_DESTINATION_REJECTED: {error}")
            }
            Self::Fragmentation(error) => write!(f, "HANDOFF_FRAGMENTATION_REJECTED: {error}"),
        }
    }
}

impl std::error::Error for HandoffError {}

/// Plan the next carrier leg for a validated protected envelope.
///
/// The caller must supply candidates in explicit product-policy preference
/// order. The coordinator then rejects unavailable, locally ineligible,
/// semantically contradictory, privacy-inadequate, or non-relay-enabled
/// candidates. The selected carrier's native fragmentation contract chooses
/// direct, native-resource, or private-block-exchange handling.
pub fn plan_handoff<'a>(
    source: &'a TransportCapability,
    destinations: &'a [TransportCapability],
    request: HandoffRequest,
) -> Result<HandoffPlan<'a>, HandoffError> {
    if !source.selectable() {
        return Err(HandoffError::SourceCarrierIneligible);
    }
    if !request.requirement.requires_relay {
        return Err(HandoffError::RelayWasNotRequired);
    }

    let selection = TransportCoordinator::select(destinations, request.requirement)
        .map_err(HandoffError::NoEligibleDestination)?;
    let next_carrier = selection.capability;

    // A normal same-carrier send is not an inter-carrier handoff. Treating it
    // as one would blur receipt/lifecycle semantics and hide a missing route.
    if source.name == next_carrier.name {
        return Err(HandoffError::CarrierDidNotChange);
    }

    let fragmentation = plan_fragmentation(
        next_carrier.semantics.fragmentation,
        request.protected_object_bytes,
    )
    .map_err(HandoffError::Fragmentation)?;

    Ok(HandoffPlan {
        envelope_id: request.envelope_id,
        source_carrier: source,
        next_carrier,
        protected_object_bytes: request.protected_object_bytes,
        fragmentation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::fragmentation::CarrierFragmentation;
    use crate::core::transport_policy::{
        CarrierFamily, CarrierSemantics, DeliveryKind, DiscoveryMode, InterfaceEligibility,
        NativeIdentityNamespace, PrivacyClass, RelayEligibility,
    };

    fn interface(relay_opt_in: bool) -> InterfaceEligibility {
        InterfaceEligibility {
            enabled_by_owner: true,
            device_supported: true,
            operating_system_permission: true,
            requires_background_execution: false,
            background_execution_permitted: true,
            relay_opt_in,
        }
    }

    fn capability(
        name: &'static str,
        family: CarrierFamily,
        native_identity: NativeIdentityNamespace,
        privacy: PrivacyClass,
        fragmentation: CarrierFragmentation,
        relay_opt_in: bool,
    ) -> TransportCapability {
        TransportCapability {
            name,
            privacy,
            supports_broadcast: false,
            supports_direct: true,
            supports_store_and_forward: false,
            supports_bulk_ciphertext: true,
            available: true,
            semantics: CarrierSemantics {
                family,
                native_identity,
                discovery: DiscoveryMode::ExplicitPrivateRoute,
                fragmentation,
                relay: RelayEligibility::OwnerOptIn,
                interface: interface(relay_opt_in),
            },
        }
    }

    fn request(bytes: usize) -> HandoffRequest {
        HandoffRequest {
            envelope_id: ProtectedEnvelopeId([42u8; 32]),
            protected_object_bytes: bytes,
            requirement: DeliveryRequirement::privacy_preserving_relayed_direct(),
        }
    }

    fn source() -> TransportCapability {
        capability(
            "wifi-direct-source",
            CarrierFamily::WifiDirect,
            NativeIdentityNamespace::WifiDirectDeviceOrGroup,
            PrivacyClass::ContentConfidential,
            CarrierFragmentation::Stream {
                max_logical_frame_bytes: 64 * 1024,
            },
            true,
        )
    }

    #[test]
    fn handoff_preserves_end_to_end_envelope_id_and_uses_rns_packet_limit() {
        let source = source();
        let rns = capability(
            "rns-tcp-destination",
            CarrierFamily::Reticulum,
            NativeIdentityNamespace::ReticulumDestination,
            PrivacyClass::RelationshipMetadataProtected,
            CarrierFragmentation::SinglePacket {
                max_payload_bytes: 383,
            },
            true,
        );

        let destinations = [rns];
        let plan = plan_handoff(&source, &destinations, request(383)).unwrap();
        assert_eq!(plan.envelope_id, ProtectedEnvelopeId([42u8; 32]));
        assert_eq!(plan.source_carrier.name, "wifi-direct-source");
        assert_eq!(plan.next_carrier.name, "rns-tcp-destination");
        assert_eq!(plan.fragmentation, FragmentationPlan::Direct);
    }

    #[test]
    fn handoff_refuses_rns_packet_oversize_without_generic_fragmentation() {
        let source = source();
        let rns = capability(
            "rns-tcp-destination",
            CarrierFamily::Reticulum,
            NativeIdentityNamespace::ReticulumDestination,
            PrivacyClass::RelationshipMetadataProtected,
            CarrierFragmentation::SinglePacket {
                max_payload_bytes: 383,
            },
            true,
        );

        assert!(matches!(
            plan_handoff(&source, &[rns], request(384)),
            Err(HandoffError::Fragmentation(
                FragmentationError::SinglePacketLimitExceeded { .. }
            ))
        ));
    }

    #[test]
    fn handoff_rejects_metadata_visible_destination_instead_of_downgrading() {
        let source = source();
        let gossip = capability(
            "shared-gossip",
            CarrierFamily::Libp2p,
            NativeIdentityNamespace::Libp2pPeerId,
            PrivacyClass::ContentConfidential,
            CarrierFragmentation::Stream {
                max_logical_frame_bytes: 64 * 1024,
            },
            true,
        );

        assert!(matches!(
            plan_handoff(&source, &[gossip], request(64)),
            Err(HandoffError::NoEligibleDestination(
                TransportSelectionError::PrivacyRequirementUnmet { .. }
            ))
        ));
    }

    #[test]
    fn handoff_requires_destination_relay_opt_in() {
        let source = source();
        let no_relay_ble = capability(
            "ble-destination",
            CarrierFamily::BluetoothLowEnergy,
            NativeIdentityNamespace::BluetoothAddressAndGattRole,
            PrivacyClass::RelationshipMetadataProtected,
            CarrierFragmentation::SinglePacket {
                max_payload_bytes: 185,
            },
            false,
        );

        assert!(matches!(
            plan_handoff(&source, &[no_relay_ble], request(64)),
            Err(HandoffError::NoEligibleDestination(
                TransportSelectionError::RelayRequirementUnmet { .. }
            ))
        ));
    }

    #[test]
    fn oversized_stream_uses_private_block_exchange_not_an_implicit_chunker() {
        let source = source();
        let quic = capability(
            "quic-destination",
            CarrierFamily::Libp2p,
            NativeIdentityNamespace::Libp2pPeerId,
            PrivacyClass::RelationshipMetadataProtected,
            CarrierFragmentation::Stream {
                max_logical_frame_bytes: 64 * 1024,
            },
            true,
        );

        let destinations = [quic];
        let plan = plan_handoff(&source, &destinations, request(128 * 1024)).unwrap();
        assert_eq!(plan.fragmentation, FragmentationPlan::PrivateBlockExchange);
    }

    #[test]
    fn same_carrier_is_not_misreported_as_a_handoff() {
        let mut source = source();
        source.privacy = PrivacyClass::RelationshipMetadataProtected;
        assert_eq!(
            plan_handoff(&source, &[source.clone()], request(64)).unwrap_err(),
            HandoffError::CarrierDidNotChange
        );
    }

    #[test]
    fn direct_send_requirement_cannot_be_reused_for_relay_handoff() {
        let source = source();
        let rns = capability(
            "rns-tcp-destination",
            CarrierFamily::Reticulum,
            NativeIdentityNamespace::ReticulumDestination,
            PrivacyClass::RelationshipMetadataProtected,
            CarrierFragmentation::SinglePacket {
                max_payload_bytes: 383,
            },
            true,
        );
        let mut not_a_handoff = request(64);
        not_a_handoff.requirement = DeliveryRequirement {
            kind: DeliveryKind::DirectMessage,
            minimum_privacy: PrivacyClass::RelationshipMetadataProtected,
            requires_relay: false,
        };

        assert_eq!(
            plan_handoff(&source, &[rns], not_a_handoff).unwrap_err(),
            HandoffError::RelayWasNotRequired
        );
    }
}
