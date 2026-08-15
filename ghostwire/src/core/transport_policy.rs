//! Privacy-policy transport selection with native-carrier semantics.
//!
//! GhostWire is an inter-transport security switchboard, not a universal
//! packet protocol. Every carrier declares its native identity namespace,
//! discovery exposure, fragmentation model, interface eligibility, and relay
//! role. The coordinator may select only a carrier that both meets the message
//! privacy requirement and is actually eligible on the current device.
//!
//! A transport failure or an ineligible local interface must never silently
//! change a message's privacy class, carrier family, or relay policy. Callers
//! must submit a new explicit operation for any lower requirement.

use crate::core::fragmentation::CarrierFragmentation;
use std::fmt;

/// Privacy properties provided by a carrier as observed by the network, not a
/// claim about local encrypted storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum PrivacyClass {
    /// Suitable only for intentionally public dissemination.
    Public,
    /// Payload is confidential, but peer/relationship metadata may be visible.
    ContentConfidential,
    /// The carrier reduces deterministic relationship metadata exposure.
    RelationshipMetadataProtected,
    /// The carrier is designed to reduce sender-recipient linkability.
    SenderRecipientUnlinkability,
}

impl PrivacyClass {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::ContentConfidential => "content_confidential",
            Self::RelationshipMetadataProtected => "relationship_metadata_protected",
            Self::SenderRecipientUnlinkability => "sender_recipient_unlinkability",
        }
    }
}

/// The native network world to which an adapter belongs. This is a factual
/// declaration, not a claim that all instances share the same privacy level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CarrierFamily {
    Libp2p,
    Reticulum,
    WifiDirect,
    BluetoothLowEnergy,
    RNodeOrLoRa,
    StoreAndForward,
    Custom,
}

/// Native identity namespace observable to the carrier/runtime. A GhostWire
/// root identity or GhostWire-side binding key must not be substituted here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum NativeIdentityNamespace {
    Libp2pPeerId,
    ReticulumDestination,
    WifiDirectDeviceOrGroup,
    BluetoothAddressAndGattRole,
    RadioOrSerialInterface,
    OpaqueMailboxHandle,
    CustomExternalIdentity,
}

/// How the native carrier discovers or exposes peers/routes. The coordinator
/// uses this as evidence; it does not infer anonymous routing from encryption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiscoveryMode {
    /// No ambient discovery; endpoint is preconfigured or carried in a private
    /// route setup message.
    ExplicitPrivateRoute,
    /// Local multicast/broadcast or radio advertisements can expose proximity
    /// and membership metadata (for example RNS AutoInterface or BLE scan).
    LocalLinkDiscovery,
    /// A direct overlay connection exposes at least the relationship between
    /// connected peers (for example an ordinary libp2p direct stream).
    OverlayPeerDiscovery,
    /// A public or shared rendezvous/DHT/provider mechanism is used.
    SharedRendezvous,
}

/// Whether this specific interface is allowed to forward other nodes' traffic.
/// "Relay" means forwarding a still-protected GhostWire envelope, never
/// decrypting user plaintext for conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RelayEligibility {
    /// The carrier cannot safely forward GhostWire envelopes yet.
    Unsupported,
    /// The carrier may relay only when the user/device profile has explicitly
    /// opted in; this is the normal phone and constrained-device posture.
    OwnerOptIn,
    /// The carrier is eligible for a supervised relay role after the device has
    /// been configured as such (normally desktop/server infrastructure).
    SupervisedNode,
}

/// Dynamic local facts that are required before a carrier can be selected. A
/// stale `available=true` flag is insufficient when the OS permission, device,
/// owner, or background policy blocks the interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterfaceEligibility {
    pub enabled_by_owner: bool,
    pub device_supported: bool,
    pub operating_system_permission: bool,
    pub requires_background_execution: bool,
    pub background_execution_permitted: bool,
    pub relay_opt_in: bool,
}

impl InterfaceEligibility {
    #[must_use]
    pub const fn usable(self) -> bool {
        self.enabled_by_owner
            && self.device_supported
            && self.operating_system_permission
            && (!self.requires_background_execution || self.background_execution_permitted)
    }
}

/// Carrier facts that must be carried beside a transport capability declaration.
/// This prevents an adapter named merely "RNS", "BLE", or "Wi-Fi" from being
/// selected without acknowledging its native protocol rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CarrierSemantics {
    pub family: CarrierFamily,
    pub native_identity: NativeIdentityNamespace,
    pub discovery: DiscoveryMode,
    pub fragmentation: CarrierFragmentation,
    pub relay: RelayEligibility,
    pub interface: InterfaceEligibility,
}

impl CarrierSemantics {
    #[must_use]
    pub const fn relay_eligible(&self) -> bool {
        self.interface.usable()
            && match self.relay {
                RelayEligibility::Unsupported => false,
                RelayEligibility::OwnerOptIn => self.interface.relay_opt_in,
                RelayEligibility::SupervisedNode => self.interface.relay_opt_in,
            }
    }

    #[must_use]
    pub const fn native_identity_matches_family(&self) -> bool {
        matches!(
            (self.family, self.native_identity),
            (CarrierFamily::Libp2p, NativeIdentityNamespace::Libp2pPeerId)
                | (CarrierFamily::Reticulum, NativeIdentityNamespace::ReticulumDestination)
                | (CarrierFamily::WifiDirect, NativeIdentityNamespace::WifiDirectDeviceOrGroup)
                | (
                    CarrierFamily::BluetoothLowEnergy,
                    NativeIdentityNamespace::BluetoothAddressAndGattRole
                )
                | (
                    CarrierFamily::RNodeOrLoRa,
                    NativeIdentityNamespace::RadioOrSerialInterface
                )
                | (
                    CarrierFamily::StoreAndForward,
                    NativeIdentityNamespace::OpaqueMailboxHandle
                )
                | (CarrierFamily::Custom, NativeIdentityNamespace::CustomExternalIdentity)
        )
    }
}

/// Semantics of the operation to be delivered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeliveryKind {
    PublicBroadcast,
    DirectMessage,
    StoreAndForward,
    BulkCiphertext,
}

/// A static/dynamic declaration supplied by each transport adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportCapability {
    pub name: &'static str,
    pub privacy: PrivacyClass,
    pub supports_broadcast: bool,
    pub supports_direct: bool,
    pub supports_store_and_forward: bool,
    pub supports_bulk_ciphertext: bool,
    /// Availability is evaluated by adapter health monitoring. It must be
    /// combined with `semantics.interface` before policy considers a candidate.
    pub available: bool,
    pub semantics: CarrierSemantics,
}

impl TransportCapability {
    #[must_use]
    pub const fn supports(&self, kind: DeliveryKind) -> bool {
        match kind {
            DeliveryKind::PublicBroadcast => self.supports_broadcast,
            DeliveryKind::DirectMessage => self.supports_direct,
            DeliveryKind::StoreAndForward => self.supports_store_and_forward,
            DeliveryKind::BulkCiphertext => self.supports_bulk_ciphertext,
        }
    }

    /// A transport is selectable only if its native declaration is internally
    /// truthful and its actual local interface is permitted to operate.
    #[must_use]
    pub const fn selectable(&self) -> bool {
        self.available
            && self.semantics.interface.usable()
            && self.semantics.native_identity_matches_family()
    }
}

/// Immutable privacy and behavior requirements for a single send operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeliveryRequirement {
    pub kind: DeliveryKind,
    pub minimum_privacy: PrivacyClass,
    /// Cross-carrier handoff requires an explicit eligible relay. It is false
    /// for an endpoint's direct send and true only when the operation actually
    /// asks an intermediate GhostWire node to forward a protected envelope.
    pub requires_relay: bool,
}

impl DeliveryRequirement {
    #[must_use]
    pub const fn public_broadcast() -> Self {
        Self {
            kind: DeliveryKind::PublicBroadcast,
            minimum_privacy: PrivacyClass::Public,
            requires_relay: false,
        }
    }

    /// Current GhostWire direct GossipSub is deliberately *not* declared to
    /// satisfy this requirement. It protects payloads but uses a shared topic,
    /// so it does not protect relationship metadata.
    #[must_use]
    pub const fn privacy_preserving_direct() -> Self {
        Self {
            kind: DeliveryKind::DirectMessage,
            minimum_privacy: PrivacyClass::RelationshipMetadataProtected,
            requires_relay: false,
        }
    }

    /// A private direct message that requires a carrier handoff through an
    /// intermediate GhostWire relay. This does not make the first-hop receipt
    /// an end-to-end delivery receipt.
    #[must_use]
    pub const fn privacy_preserving_relayed_direct() -> Self {
        Self {
            kind: DeliveryKind::DirectMessage,
            minimum_privacy: PrivacyClass::RelationshipMetadataProtected,
            requires_relay: true,
        }
    }
}

/// A policy result is intentionally a borrowed candidate. The caller owns the
/// actual adapter and is responsible for recording the lifecycle attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportSelection<'a> {
    pub capability: &'a TransportCapability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransportSelectionError {
    NoAvailableCandidate {
        kind: DeliveryKind,
    },
    PrivacyRequirementUnmet {
        kind: DeliveryKind,
        minimum: PrivacyClass,
    },
    RelayRequirementUnmet {
        kind: DeliveryKind,
    },
}

impl fmt::Display for TransportSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoAvailableCandidate { kind } => {
                write!(f, "no available transport supports {kind:?}")
            }
            Self::PrivacyRequirementUnmet { kind, minimum } => write!(
                f,
                "no available transport meets {} privacy for {kind:?}",
                minimum.as_str()
            ),
            Self::RelayRequirementUnmet { kind } => {
                write!(f, "no available transport is eligible to relay {kind:?}")
            }
        }
    }
}

impl std::error::Error for TransportSelectionError {}

/// Selects the first caller-prioritized candidate that is available, locally
/// eligible, semantically truthful, and privacy compliant. Candidate order is
/// explicit product policy; this coordinator does not invent a hidden
/// performance-over-privacy preference.
#[derive(Debug, Default, Clone, Copy)]
pub struct TransportCoordinator;

impl TransportCoordinator {
    pub fn select<'a>(
        candidates: &'a [TransportCapability],
        requirement: DeliveryRequirement,
    ) -> Result<TransportSelection<'a>, TransportSelectionError> {
        let mut supports_kind = false;
        let mut selectable_for_kind = false;
        let mut relay_eligible_for_kind = false;

        for capability in candidates {
            if !capability.supports(requirement.kind) {
                continue;
            }
            supports_kind = true;
            if !capability.selectable() {
                continue;
            }
            selectable_for_kind = true;
            if requirement.requires_relay {
                if !capability.semantics.relay_eligible() {
                    continue;
                }
                relay_eligible_for_kind = true;
            }
            if capability.privacy >= requirement.minimum_privacy {
                return Ok(TransportSelection { capability });
            }
        }

        if requirement.requires_relay && selectable_for_kind && !relay_eligible_for_kind {
            return Err(TransportSelectionError::RelayRequirementUnmet {
                kind: requirement.kind,
            });
        }
        if supports_kind && selectable_for_kind {
            Err(TransportSelectionError::PrivacyRequirementUnmet {
                kind: requirement.kind,
                minimum: requirement.minimum_privacy,
            })
        } else {
            Err(TransportSelectionError::NoAvailableCandidate {
                kind: requirement.kind,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn semantics(
        family: CarrierFamily,
        native_identity: NativeIdentityNamespace,
        relay: RelayEligibility,
        relay_opt_in: bool,
    ) -> CarrierSemantics {
        CarrierSemantics {
            family,
            native_identity,
            discovery: DiscoveryMode::ExplicitPrivateRoute,
            fragmentation: CarrierFragmentation::Stream {
                max_logical_frame_bytes: 64 * 1024,
            },
            relay,
            interface: interface(relay_opt_in),
        }
    }

    fn capability(
        name: &'static str,
        privacy: PrivacyClass,
        available: bool,
        direct: bool,
        broadcast: bool,
    ) -> TransportCapability {
        TransportCapability {
            name,
            privacy,
            supports_broadcast: broadcast,
            supports_direct: direct,
            supports_store_and_forward: false,
            supports_bulk_ciphertext: false,
            available,
            semantics: semantics(
                CarrierFamily::Libp2p,
                NativeIdentityNamespace::Libp2pPeerId,
                RelayEligibility::OwnerOptIn,
                false,
            ),
        }
    }

    #[test]
    fn chooses_the_first_policy_prioritized_compliant_candidate() {
        let candidates = [
            capability(
                "anonymous-mailbox",
                PrivacyClass::SenderRecipientUnlinkability,
                true,
                true,
                false,
            ),
            capability(
                "direct-stream",
                PrivacyClass::RelationshipMetadataProtected,
                true,
                true,
                false,
            ),
        ];

        let selected = TransportCoordinator::select(
            &candidates,
            DeliveryRequirement::privacy_preserving_direct(),
        )
        .unwrap();
        assert_eq!(selected.capability.name, "anonymous-mailbox");
    }

    #[test]
    fn rejects_metadata_visible_direct_carrier_without_downgrade() {
        let candidates = [capability(
            "shared-gossip-topic",
            PrivacyClass::ContentConfidential,
            true,
            true,
            true,
        )];

        assert_eq!(
            TransportCoordinator::select(
                &candidates,
                DeliveryRequirement::privacy_preserving_direct(),
            )
            .unwrap_err(),
            TransportSelectionError::PrivacyRequirementUnmet {
                kind: DeliveryKind::DirectMessage,
                minimum: PrivacyClass::RelationshipMetadataProtected,
            }
        );
    }

    #[test]
    fn ignores_an_unavailable_privacy_candidate() {
        let candidates = [capability(
            "anonymous-mailbox",
            PrivacyClass::SenderRecipientUnlinkability,
            false,
            true,
            false,
        )];

        assert_eq!(
            TransportCoordinator::select(
                &candidates,
                DeliveryRequirement::privacy_preserving_direct(),
            )
            .unwrap_err(),
            TransportSelectionError::NoAvailableCandidate {
                kind: DeliveryKind::DirectMessage,
            }
        );
    }

    #[test]
    fn allows_public_broadcast_on_public_carrier() {
        let candidates = [capability(
            "mesh-gossip",
            PrivacyClass::Public,
            true,
            false,
            true,
        )];

        let selected =
            TransportCoordinator::select(&candidates, DeliveryRequirement::public_broadcast())
                .unwrap();
        assert_eq!(selected.capability.name, "mesh-gossip");
    }

    #[test]
    fn distinguishes_missing_capability_from_privacy_mismatch() {
        let candidates = [capability(
            "broadcast-only",
            PrivacyClass::SenderRecipientUnlinkability,
            true,
            false,
            true,
        )];

        assert_eq!(
            TransportCoordinator::select(
                &candidates,
                DeliveryRequirement::privacy_preserving_direct(),
            )
            .unwrap_err(),
            TransportSelectionError::NoAvailableCandidate {
                kind: DeliveryKind::DirectMessage,
            }
        );
    }

    #[test]
    fn user_disabled_or_permission_denied_interface_is_not_selectable() {
        let mut ble = capability(
            "ble-gatt",
            PrivacyClass::ContentConfidential,
            true,
            true,
            false,
        );
        ble.semantics = semantics(
            CarrierFamily::BluetoothLowEnergy,
            NativeIdentityNamespace::BluetoothAddressAndGattRole,
            RelayEligibility::OwnerOptIn,
            false,
        );
        ble.semantics.interface.enabled_by_owner = false;

        assert_eq!(
            TransportCoordinator::select(
                &[ble],
                DeliveryRequirement {
                    kind: DeliveryKind::DirectMessage,
                    minimum_privacy: PrivacyClass::ContentConfidential,
                    requires_relay: false,
                },
            )
            .unwrap_err(),
            TransportSelectionError::NoAvailableCandidate {
                kind: DeliveryKind::DirectMessage,
            }
        );
    }

    #[test]
    fn relayed_message_requires_explicit_interface_relay_opt_in() {
        let candidate = capability(
            "wifi-direct-group",
            PrivacyClass::RelationshipMetadataProtected,
            true,
            true,
            false,
        );
        assert_eq!(
            TransportCoordinator::select(
                &[candidate],
                DeliveryRequirement::privacy_preserving_relayed_direct(),
            )
            .unwrap_err(),
            TransportSelectionError::RelayRequirementUnmet {
                kind: DeliveryKind::DirectMessage,
            }
        );
    }

    #[test]
    fn false_native_identity_declaration_cannot_be_selected() {
        let mut rns = capability(
            "rns-tcp",
            PrivacyClass::ContentConfidential,
            true,
            true,
            false,
        );
        rns.semantics = semantics(
            CarrierFamily::Reticulum,
            NativeIdentityNamespace::Libp2pPeerId,
            RelayEligibility::Unsupported,
            false,
        );

        assert_eq!(
            TransportCoordinator::select(
                &[rns],
                DeliveryRequirement {
                    kind: DeliveryKind::DirectMessage,
                    minimum_privacy: PrivacyClass::ContentConfidential,
                    requires_relay: false,
                },
            )
            .unwrap_err(),
            TransportSelectionError::NoAvailableCandidate {
                kind: DeliveryKind::DirectMessage,
            }
        );
    }
}
