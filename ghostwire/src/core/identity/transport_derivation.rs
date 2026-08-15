//! Domain-separated GhostWire transport and interface authentication keys.
//!
//! A GhostWire root identity must not be reused as a libp2p peer key, QUIC key,
//! BLE advertisement identity, radio/interface identity, or external-network
//! identity. Reusing it turns a public identifier from any one carrier into a
//! stable cross-carrier correlation handle.
//!
//! This module derives *GhostWire-side* per-carrier, per-interface, per-epoch
//! Ed25519 authentication keys from an unlocked root identity via HKDF-SHA-256.
//! The caller persists the random [`InterfaceInstanceId`] only in local private
//! configuration. It must never use a MAC address, hardware serial number,
//! interface name, SSID, BSSID, Bluetooth address, IP address, or RNS
//! destination hash as derivation context.
//!
//! External-network identities remain owned by their native runtimes. In
//! particular, the RNS runtime owns its RNS identity; `RnsBridgeBinding` is only
//! a GhostWire authentication key for the local bridge boundary, not an RNS
//! identity seed and not input to RNS cryptography.

use anyhow::{anyhow, Result};
use hkdf::Hkdf;
use libp2p::identity::Keypair;
use ring::rand::{SecureRandom, SystemRandom};
use sha2::Sha256;
use zeroize::Zeroizing;

const DERIVATION_SALT: &[u8] = b"GhostWire/transport-key/salt/v1";
const DERIVATION_LABEL: &[u8] = b"GhostWire/transport-key/v1";
const DERIVED_SEED_BYTES: usize = 32;

/// An explicit security domain for a GhostWire-side transport authentication
/// key. The numeric values are wire-independent and must never be reordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TransportKeyDomain {
    Libp2pMesh = 1,
    DirectQuic = 2,
    BluetoothLowEnergy = 3,
    WifiDirect = 4,
    RNodeOrLoRa = 5,
    StoreAndForward = 6,
    /// Authentication for the **local GhostWire ↔ RNS bridge control boundary**.
    /// This is deliberately not an RNS identity.
    RnsBridgeBinding = 7,
}

/// A locally generated opaque identifier for one configured interface instance.
/// This is not a hardware or network identifier and must be stored only in
/// private configuration alongside the interface configuration that owns it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceInstanceId([u8; 16]);

impl InterfaceInstanceId {
    /// Generate a fresh, unlinkable local interface-instance identifier.
    pub fn random() -> Result<Self> {
        let rng = SystemRandom::new();
        let mut bytes = [0u8; 16];
        rng.fill(&mut bytes)
            .map_err(|_| anyhow!("INTERFACE_INSTANCE_RANDOMNESS_UNAVAILABLE"))?;
        Ok(Self(bytes))
    }

    /// Construct from persistent private configuration. The all-zero value is
    /// rejected because it is a likely accidental default shared by interfaces.
    pub fn from_private_bytes(bytes: [u8; 16]) -> Result<Self> {
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(anyhow!("INTERFACE_INSTANCE_ID_INVALID"));
        }
        Ok(Self(bytes))
    }

    /// Serialize for encrypted/local-only configuration persistence.
    pub fn private_bytes(self) -> [u8; 16] {
        self.0
    }
}

/// Derivation input for a GhostWire-side carrier key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransportKeyScope {
    pub domain: TransportKeyDomain,
    pub interface: InterfaceInstanceId,
    /// Caller-controlled key rotation epoch. This is a logical epoch, not a
    /// timestamp: callers should advance it only through a persistent rotation
    /// policy so a restart does not silently change an advertised key.
    pub rotation_epoch: u64,
}

impl TransportKeyScope {
    pub fn new(
        domain: TransportKeyDomain,
        interface: InterfaceInstanceId,
        rotation_epoch: u64,
    ) -> Self {
        Self {
            domain,
            interface,
            rotation_epoch,
        }
    }
}

/// Derive a GhostWire transport/interface authentication key from an unlocked
/// root keypair. The root private bytes are used only as HKDF input and are
/// held in a zeroizing buffer while the derivation takes place.
pub fn derive_transport_keypair(
    root_identity: &Keypair,
    scope: TransportKeyScope,
) -> Result<Keypair> {
    let root_bytes = Zeroizing::new(
        root_identity
            .to_protobuf_encoding()
            .map_err(|error| anyhow!("TRANSPORT_KEY_ROOT_ENCODING_FAILED: {error}"))?,
    );

    let hkdf = Hkdf::<Sha256>::new(Some(DERIVATION_SALT), root_bytes.as_slice());
    let mut info = [0u8; DERIVATION_LABEL.len() + 1 + 8 + 16];
    info[..DERIVATION_LABEL.len()].copy_from_slice(DERIVATION_LABEL);
    let mut offset = DERIVATION_LABEL.len();
    info[offset] = scope.domain as u8;
    offset += 1;
    info[offset..offset + 8].copy_from_slice(&scope.rotation_epoch.to_be_bytes());
    offset += 8;
    info[offset..offset + 16].copy_from_slice(&scope.interface.0);

    let mut child_seed = Zeroizing::new([0u8; DERIVED_SEED_BYTES]);
    hkdf.expand(&info, child_seed.as_mut())
        .map_err(|_| anyhow!("TRANSPORT_KEY_DERIVATION_FAILED"))?;

    Keypair::ed25519_from_bytes(child_seed.to_vec())
        .map_err(|error| anyhow!("TRANSPORT_KEY_CONSTRUCTION_FAILED: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> Keypair {
        Keypair::ed25519_from_bytes(vec![7u8; 32]).expect("fixed valid test identity seed")
    }

    fn interface(byte: u8) -> InterfaceInstanceId {
        InterfaceInstanceId::from_private_bytes([byte; 16]).expect("non-zero test interface")
    }

    #[test]
    fn same_scope_derives_the_same_transport_public_key() {
        let scope = TransportKeyScope::new(TransportKeyDomain::Libp2pMesh, interface(1), 0);
        let first = derive_transport_keypair(&root(), scope).unwrap().public();
        let second = derive_transport_keypair(&root(), scope).unwrap().public();
        assert_eq!(first, second);
    }

    #[test]
    fn changing_domain_prevents_cross_transport_key_reuse() {
        let libp2p = derive_transport_keypair(
            &root(),
            TransportKeyScope::new(TransportKeyDomain::Libp2pMesh, interface(1), 0),
        )
        .unwrap()
        .public();
        let ble = derive_transport_keypair(
            &root(),
            TransportKeyScope::new(TransportKeyDomain::BluetoothLowEnergy, interface(1), 0),
        )
        .unwrap()
        .public();
        assert_ne!(libp2p, ble);
    }

    #[test]
    fn changing_interface_or_epoch_rotates_the_key() {
        let stable = derive_transport_keypair(
            &root(),
            TransportKeyScope::new(TransportKeyDomain::WifiDirect, interface(1), 0),
        )
        .unwrap()
        .public();
        let other_interface = derive_transport_keypair(
            &root(),
            TransportKeyScope::new(TransportKeyDomain::WifiDirect, interface(2), 0),
        )
        .unwrap()
        .public();
        let next_epoch = derive_transport_keypair(
            &root(),
            TransportKeyScope::new(TransportKeyDomain::WifiDirect, interface(1), 1),
        )
        .unwrap()
        .public();
        assert_ne!(stable, other_interface);
        assert_ne!(stable, next_epoch);
    }

    #[test]
    fn all_zero_interface_identifier_is_rejected() {
        assert!(InterfaceInstanceId::from_private_bytes([0u8; 16]).is_err());
    }

    #[test]
    fn rns_bridge_binding_is_not_the_root_public_key() {
        let root = root();
        let binding = derive_transport_keypair(
            &root,
            TransportKeyScope::new(TransportKeyDomain::RnsBridgeBinding, interface(7), 0),
        )
        .unwrap();
        assert_ne!(root.public(), binding.public());
    }
}
