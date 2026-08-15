//! Policy-bound submission planning for the experimental official-RNS bridge.
//!
//! This is deliberately narrower than an application messaging transport. It
//! accepts only a caller-supplied **already protected opaque envelope** and
//! returns a plan for one RNS encrypted packet. The bridge acknowledgement is
//! a local carrier-submission event; it is never an end-to-end GhostWire
//! receipt. Direct/private message dispatch is intentionally not rerouted here.

use crate::core::fragmentation::{plan as plan_fragmentation, FragmentationError, FragmentationPlan};
use crate::core::operating_profile::{
    DeviceRuntimeState, OperatingProfile, OperatingProfileError,
};
use crate::core::transport_handoff::ProtectedEnvelopeId;
use crate::core::transport_policy::{
    CarrierFamily, CarrierSemantics, DeliveryRequirement, DiscoveryMode, InterfaceEligibility,
    NativeIdentityNamespace, PrivacyClass, RelayEligibility, TransportCapability,
    TransportCoordinator, TransportSelectionError,
};
use std::fmt;

/// The verified RNS bridge currently supports only official RNS encrypted
/// packet-mode payloads. It has no Link/Resource or generic fragmentation path.
pub const RNS_SINGLE_PACKET_PAYLOAD_BYTES: usize = 383;

/// The only documented local lifecycle status after a successful bridge write.
/// It does not claim that RNS found a path, a remote RNS node received a packet,
/// or the GhostWire recipient decrypted/delivered an application envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RnsCarrierSubmissionState {
    AcceptedByLocalBridge,
}

/// A prepared low-level bridge operation. It retains the stable GhostWire
/// envelope ID for correlation, while the RNS destination remains a distinct
/// native-identity value and is never treated as a GhostWire account/peer ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RnsPacketSubmissionPlan {
    pub envelope_id: ProtectedEnvelopeId,
    pub rns_destination: String,
    pub protected_object_bytes: usize,
    pub requirement: DeliveryRequirement,
    pub carrier_submission_state: RnsCarrierSubmissionState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RnsHybridError {
    InvalidDestination,
    Policy(TransportSelectionError),
    Fragmentation(FragmentationError),
    OperatingProfile(OperatingProfileError),
}

impl fmt::Display for RnsHybridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDestination => write!(f, "RNS_HYBRID_INVALID_DESTINATION"),
            Self::Policy(error) => write!(f, "RNS_HYBRID_POLICY_REJECTED: {error}"),
            Self::Fragmentation(error) => write!(f, "RNS_HYBRID_FRAGMENTATION_REJECTED: {error}"),
            Self::OperatingProfile(error) => {
                write!(f, "RNS_HYBRID_PROFILE_REJECTED: {error}")
            }
        }
    }
}

impl std::error::Error for RnsHybridError {}

/// Describe exactly what the current RNS packet bridge can truthfully offer.
/// RNS packet encryption protects content on the RNS leg, but an explicit RNS
/// destination/path still exposes relationship metadata. Therefore this
/// capability is deliberately no stronger than `ContentConfidential`.
#[must_use]
pub fn rns_packet_capability(
    bridge_available: bool,
    interface: InterfaceEligibility,
) -> TransportCapability {
    TransportCapability {
        name: "rns-single-packet-bridge",
        privacy: PrivacyClass::ContentConfidential,
        supports_broadcast: false,
        supports_direct: true,
        supports_store_and_forward: false,
        supports_bulk_ciphertext: false,
        available: bridge_available,
        semantics: CarrierSemantics {
            family: CarrierFamily::Reticulum,
            native_identity: NativeIdentityNamespace::ReticulumDestination,
            // The bridge accepts an explicitly configured/known destination. It
            // does not claim that AutoInterface, announces, or a shared RNS
            // path are private discovery mechanisms for GhostWire.
            discovery: DiscoveryMode::ExplicitPrivateRoute,
            fragmentation: crate::core::fragmentation::CarrierFragmentation::SinglePacket {
                max_payload_bytes: RNS_SINGLE_PACKET_PAYLOAD_BYTES,
            },
            relay: RelayEligibility::OwnerOptIn,
            interface,
        },
    }
}

/// Plan one bounded submission to the external RNS bridge.
///
/// `protected_payload` must have already received GhostWire message-level
/// encryption and authentication. This function intentionally has no `Message`
/// parameter and cannot serialize application plaintext for transport.
pub fn plan_rns_packet_submission(
    profile: OperatingProfile,
    runtime_state: DeviceRuntimeState,
    interface: InterfaceEligibility,
    envelope_id: ProtectedEnvelopeId,
    destination: &str,
    requirement: DeliveryRequirement,
    protected_payload: &[u8],
) -> Result<RnsPacketSubmissionPlan, RnsHybridError> {
    if !valid_destination(destination) {
        return Err(RnsHybridError::InvalidDestination);
    }

    let effective_interface = profile.constrain_interface(runtime_state, interface);
    let capability = rns_packet_capability(true, effective_interface);
    TransportCoordinator::select(std::slice::from_ref(&capability), requirement)
        .map_err(RnsHybridError::Policy)?;

    profile
        .admit_operation(runtime_state, requirement, protected_payload.len())
        .map_err(RnsHybridError::OperatingProfile)?;

    let fragmentation = plan_fragmentation(
        capability.semantics.fragmentation,
        protected_payload.len(),
    )
    .map_err(RnsHybridError::Fragmentation)?;
    debug_assert_eq!(fragmentation, FragmentationPlan::Direct);

    Ok(RnsPacketSubmissionPlan {
        envelope_id,
        rns_destination: destination.to_ascii_lowercase(),
        protected_object_bytes: protected_payload.len(),
        requirement,
        carrier_submission_state: RnsCarrierSubmissionState::AcceptedByLocalBridge,
    })
}

fn valid_destination(destination: &str) -> bool {
    destination.len() == 32 && destination.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::transport_policy::{DeliveryKind, PrivacyClass};

    const DESTINATION: &str = "0123456789abcdef0123456789abcdef";

    fn interface() -> InterfaceEligibility {
        InterfaceEligibility {
            enabled_by_owner: true,
            device_supported: true,
            operating_system_permission: true,
            requires_background_execution: false,
            background_execution_permitted: true,
            relay_opt_in: true,
        }
    }

    fn content_confidential_direct() -> DeliveryRequirement {
        DeliveryRequirement {
            kind: DeliveryKind::DirectMessage,
            minimum_privacy: PrivacyClass::ContentConfidential,
            requires_relay: false,
        }
    }

    #[test]
    fn plans_only_a_bounded_opaque_content_confidential_packet() {
        let payload = [7u8; RNS_SINGLE_PACKET_PAYLOAD_BYTES];
        let plan = plan_rns_packet_submission(
            OperatingProfile::desktop_endpoint(),
            DeviceRuntimeState::desktop_nominal(),
            interface(),
            ProtectedEnvelopeId([42; 32]),
            DESTINATION,
            content_confidential_direct(),
            &payload,
        )
        .unwrap();

        assert_eq!(plan.envelope_id, ProtectedEnvelopeId([42; 32]));
        assert_eq!(plan.rns_destination, DESTINATION);
        assert_eq!(
            plan.carrier_submission_state,
            RnsCarrierSubmissionState::AcceptedByLocalBridge
        );
    }

    #[test]
    fn private_direct_requirement_cannot_be_downgraded_to_rns_packet_mode() {
        let error = plan_rns_packet_submission(
            OperatingProfile::desktop_endpoint(),
            DeviceRuntimeState::desktop_nominal(),
            interface(),
            ProtectedEnvelopeId([1; 32]),
            DESTINATION,
            DeliveryRequirement::privacy_preserving_direct(),
            &[1],
        )
        .unwrap_err();

        assert!(matches!(
            error,
            RnsHybridError::Policy(TransportSelectionError::PrivacyRequirementUnmet { .. })
        ));
    }

    #[test]
    fn oversize_packet_is_rejected_without_generic_fragmentation() {
        let payload = vec![1u8; RNS_SINGLE_PACKET_PAYLOAD_BYTES + 1];
        let error = plan_rns_packet_submission(
            OperatingProfile::desktop_endpoint(),
            DeviceRuntimeState::desktop_nominal(),
            interface(),
            ProtectedEnvelopeId([2; 32]),
            DESTINATION,
            content_confidential_direct(),
            &payload,
        )
        .unwrap_err();

        assert!(matches!(
            error,
            RnsHybridError::Fragmentation(FragmentationError::SinglePacketLimitExceeded { .. })
        ));
    }

    #[test]
    fn mobile_endpoint_cannot_be_promoted_to_relay_by_rns_capability() {
        let relay_requirement = DeliveryRequirement {
            kind: DeliveryKind::DirectMessage,
            minimum_privacy: PrivacyClass::ContentConfidential,
            requires_relay: true,
        };
        let error = plan_rns_packet_submission(
            OperatingProfile::mobile_endpoint(),
            DeviceRuntimeState {
                background_execution_permitted: true,
                ..DeviceRuntimeState::constrained_nominal()
            },
            interface(),
            ProtectedEnvelopeId([3; 32]),
            DESTINATION,
            relay_requirement,
            &[1],
        )
        .unwrap_err();

        assert!(matches!(
            error,
            RnsHybridError::Policy(TransportSelectionError::RelayRequirementUnmet { .. })
        ));
    }

    #[test]
    fn malformed_destination_is_rejected_before_policy_or_bridge_use() {
        let error = plan_rns_packet_submission(
            OperatingProfile::desktop_endpoint(),
            DeviceRuntimeState::desktop_nominal(),
            interface(),
            ProtectedEnvelopeId([4; 32]),
            "not-an-rns-destination",
            content_confidential_direct(),
            &[1],
        )
        .unwrap_err();
        assert_eq!(error, RnsHybridError::InvalidDestination);
    }
}
