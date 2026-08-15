//! Deterministic device-capability operating profiles.
//!
//! A GhostWire installation is always a node, but not every node should behave
//! as an unrestricted relay. This module translates explicit device class,
//! owner role, and dynamic resource state into bounded local work limits. It
//! intentionally does **not** alter a message's declared privacy requirement:
//! when a constrained profile cannot support an operation, it returns a typed
//! refusal rather than silently selecting a weaker carrier or relay behavior.

use crate::core::armor::DeviceTier;
use crate::core::transport_policy::{DeliveryKind, DeliveryRequirement, InterfaceEligibility};
use std::fmt;

/// The stable, user-configured physical/operational class of a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeviceClass {
    /// Desktop, laptop on AC, server, or deliberately provisioned home/GCP
    /// node. This does not imply that relay is enabled.
    DesktopOrServer,
    /// Phone, battery-backed tablet, or other platform where background work,
    /// radios, metering, and energy are constrained.
    MobileOrConstrained,
    /// Dedicated low-rate radio or serial gateway. It is never a generic bulk
    /// or flood node merely because it has a persistent power supply.
    LowRateRadioGateway,
}

/// The user-owned node role. A role is explicit; automatic hardware detection
/// must not promote a mobile device to a relay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum NodeRole {
    EndpointOnly,
    RelayOptIn,
    SupervisedRelay,
}

/// Dynamic device state supplied by the platform integration. Missing signals
/// must remain conservative rather than being guessed as unlimited capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceRuntimeState {
    /// Battery percentage. `None` means unknown/not-applicable.
    pub battery_percent: Option<u8>,
    pub external_power: bool,
    pub thermal: ThermalState,
    pub metered_network: bool,
    pub background_execution_permitted: bool,
}

impl DeviceRuntimeState {
    #[must_use]
    pub const fn desktop_nominal() -> Self {
        Self {
            battery_percent: None,
            external_power: true,
            thermal: ThermalState::Nominal,
            metered_network: false,
            background_execution_permitted: true,
        }
    }

    #[must_use]
    pub const fn constrained_nominal() -> Self {
        Self {
            battery_percent: Some(100),
            external_power: false,
            thermal: ThermalState::Nominal,
            metered_network: false,
            background_execution_permitted: false,
        }
    }
}

/// Thermal state is used only to reduce local work. It never loosens privacy,
/// trust, replay, or cryptographic validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ThermalState {
    Nominal,
    Warm,
    Critical,
}

/// Bounded work that a profile permits at this instant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatingLimits {
    pub max_active_peers: usize,
    pub max_pending_operations: usize,
    pub max_relay_queue_bytes: usize,
    pub max_concurrent_streams: usize,
    pub max_concurrent_bulk_sessions: usize,
    pub max_private_bulk_object_bytes: usize,
}

/// Effective policy derived from immutable profile and current device state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectiveOperatingPolicy {
    pub limits: OperatingLimits,
    pub allow_new_relay_work: bool,
    pub allow_new_bulk_work: bool,
    pub allow_background_carriers: bool,
    /// Existing traffic armor tier. This expresses local cover/padding resource
    /// parameters only; it never lowers `DeliveryRequirement::minimum_privacy`.
    pub armor_tier: DeviceTier,
}

/// A user-configured profile plus device class. Runtime state is passed
/// separately so a battery or thermal change has a deterministic, bounded
/// effect without rewriting persistent identity or transport configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatingProfile {
    pub device_class: DeviceClass,
    pub role: NodeRole,
}

impl OperatingProfile {
    #[must_use]
    pub const fn desktop_endpoint() -> Self {
        Self {
            device_class: DeviceClass::DesktopOrServer,
            role: NodeRole::EndpointOnly,
        }
    }

    #[must_use]
    pub const fn desktop_supervised_relay() -> Self {
        Self {
            device_class: DeviceClass::DesktopOrServer,
            role: NodeRole::SupervisedRelay,
        }
    }

    #[must_use]
    pub const fn mobile_endpoint() -> Self {
        Self {
            device_class: DeviceClass::MobileOrConstrained,
            role: NodeRole::EndpointOnly,
        }
    }

    #[must_use]
    pub const fn mobile_relay_opt_in() -> Self {
        Self {
            device_class: DeviceClass::MobileOrConstrained,
            role: NodeRole::RelayOptIn,
        }
    }

    #[must_use]
    pub const fn radio_gateway() -> Self {
        Self {
            device_class: DeviceClass::LowRateRadioGateway,
            role: NodeRole::SupervisedRelay,
        }
    }

    /// Compute an effective policy. This may only reduce work from the base
    /// profile according to resource state; it never creates relay consent or
    /// mutates an operation's privacy/delivery requirement.
    #[must_use]
    pub fn effective(self, state: DeviceRuntimeState) -> EffectiveOperatingPolicy {
        let mut policy = match self.device_class {
            DeviceClass::DesktopOrServer => EffectiveOperatingPolicy {
                limits: OperatingLimits {
                    max_active_peers: 128,
                    max_pending_operations: 4096,
                    max_relay_queue_bytes: 64 * 1024 * 1024,
                    max_concurrent_streams: 32,
                    max_concurrent_bulk_sessions: 8,
                    max_private_bulk_object_bytes: 64 * 1024 * 1024,
                },
                allow_new_relay_work: !matches!(self.role, NodeRole::EndpointOnly),
                allow_new_bulk_work: true,
                allow_background_carriers: true,
                armor_tier: DeviceTier::High,
            },
            DeviceClass::MobileOrConstrained => EffectiveOperatingPolicy {
                limits: OperatingLimits {
                    max_active_peers: 16,
                    max_pending_operations: 256,
                    max_relay_queue_bytes: 4 * 1024 * 1024,
                    max_concurrent_streams: 4,
                    max_concurrent_bulk_sessions: 1,
                    max_private_bulk_object_bytes: 8 * 1024 * 1024,
                },
                allow_new_relay_work: matches!(self.role, NodeRole::RelayOptIn)
                    && state.background_execution_permitted,
                allow_new_bulk_work: !state.metered_network,
                allow_background_carriers: state.background_execution_permitted,
                armor_tier: DeviceTier::Medium,
            },
            DeviceClass::LowRateRadioGateway => EffectiveOperatingPolicy {
                limits: OperatingLimits {
                    max_active_peers: 4,
                    max_pending_operations: 64,
                    max_relay_queue_bytes: 512 * 1024,
                    max_concurrent_streams: 1,
                    max_concurrent_bulk_sessions: 0,
                    max_private_bulk_object_bytes: 0,
                },
                allow_new_relay_work: !matches!(self.role, NodeRole::EndpointOnly),
                allow_new_bulk_work: false,
                allow_background_carriers: true,
                armor_tier: DeviceTier::Low,
            },
        };

        // Critical heat stops new discretionary forwarding/bulk work; already
        // accepted operations require their own graceful lifecycle shutdown.
        if matches!(state.thermal, ThermalState::Critical) {
            policy.allow_new_relay_work = false;
            policy.allow_new_bulk_work = false;
            policy.limits.max_active_peers = policy.limits.max_active_peers.min(4);
            policy.limits.max_pending_operations = policy.limits.max_pending_operations.min(64);
            policy.limits.max_concurrent_streams = policy.limits.max_concurrent_streams.min(1);
        }

        // Battery below 20% on non-external power keeps endpoint messaging but
        // stops new relay/bulk work and sharply bounds retained work.
        if !state.external_power
            && matches!(self.device_class, DeviceClass::MobileOrConstrained)
            && matches!(state.battery_percent, Some(level) if level < 20)
        {
            policy.allow_new_relay_work = false;
            policy.allow_new_bulk_work = false;
            policy.limits.max_active_peers = policy.limits.max_active_peers.min(8);
            policy.limits.max_pending_operations = policy.limits.max_pending_operations.min(64);
            policy.limits.max_relay_queue_bytes =
                policy.limits.max_relay_queue_bytes.min(512 * 1024);
            policy.limits.max_concurrent_streams = policy.limits.max_concurrent_streams.min(1);
            policy.limits.max_concurrent_bulk_sessions = 0;
        }

        policy
    }

    /// Apply profile limits to a carrier’s existing local eligibility facts.
    /// This can only restrict an interface; it cannot turn a disabled or
    /// non-consensual interface into a relay.
    #[must_use]
    pub fn constrain_interface(
        self,
        state: DeviceRuntimeState,
        interface: InterfaceEligibility,
    ) -> InterfaceEligibility {
        let policy = self.effective(state);
        InterfaceEligibility {
            enabled_by_owner: interface.enabled_by_owner,
            device_supported: interface.device_supported,
            operating_system_permission: interface.operating_system_permission,
            requires_background_execution: interface.requires_background_execution,
            background_execution_permitted: interface.background_execution_permitted
                && policy.allow_background_carriers,
            relay_opt_in: interface.relay_opt_in && policy.allow_new_relay_work,
        }
    }

    /// Check whether the profile may accept new local work for an already
    /// selected, policy-compliant operation. The input requirement is read-only:
    /// a resource refusal cannot lower its privacy class or remove relay intent.
    pub fn admit_operation(
        self,
        state: DeviceRuntimeState,
        requirement: DeliveryRequirement,
        protected_object_bytes: usize,
    ) -> Result<EffectiveOperatingPolicy, OperatingProfileError> {
        let policy = self.effective(state);
        if requirement.requires_relay && !policy.allow_new_relay_work {
            return Err(OperatingProfileError::RelayWorkNotPermitted);
        }
        if matches!(requirement.kind, DeliveryKind::BulkCiphertext) && !policy.allow_new_bulk_work {
            return Err(OperatingProfileError::BulkWorkNotPermitted);
        }
        if protected_object_bytes > policy.limits.max_private_bulk_object_bytes
            && matches!(requirement.kind, DeliveryKind::BulkCiphertext)
        {
            return Err(OperatingProfileError::BulkObjectLimitExceeded {
                limit: policy.limits.max_private_bulk_object_bytes,
            });
        }
        Ok(policy)
    }
}

/// Public-safe resource/role refusal. It contains no peer, route, radio, or
/// device-fingerprint data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum OperatingProfileError {
    RelayWorkNotPermitted,
    BulkWorkNotPermitted,
    BulkObjectLimitExceeded { limit: usize },
}

impl fmt::Display for OperatingProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RelayWorkNotPermitted => write!(f, "PROFILE_RELAY_WORK_NOT_PERMITTED"),
            Self::BulkWorkNotPermitted => write!(f, "PROFILE_BULK_WORK_NOT_PERMITTED"),
            Self::BulkObjectLimitExceeded { limit } => {
                write!(f, "PROFILE_BULK_OBJECT_LIMIT_EXCEEDED:{limit}")
            }
        }
    }
}

impl std::error::Error for OperatingProfileError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::transport_policy::{DeliveryKind, PrivacyClass};

    const RELAYED_DIRECT: DeliveryRequirement = DeliveryRequirement {
        kind: DeliveryKind::DirectMessage,
        minimum_privacy: PrivacyClass::RelationshipMetadataProtected,
        requires_relay: true,
    };

    const BULK: DeliveryRequirement = DeliveryRequirement {
        kind: DeliveryKind::BulkCiphertext,
        minimum_privacy: PrivacyClass::RelationshipMetadataProtected,
        requires_relay: false,
    };

    fn relay_interface() -> InterfaceEligibility {
        InterfaceEligibility {
            enabled_by_owner: true,
            device_supported: true,
            operating_system_permission: true,
            requires_background_execution: true,
            background_execution_permitted: true,
            relay_opt_in: true,
        }
    }

    #[test]
    fn mobile_profile_is_endpoint_only_by_default() {
        let profile = OperatingProfile::mobile_endpoint();
        let state = DeviceRuntimeState {
            background_execution_permitted: true,
            ..DeviceRuntimeState::constrained_nominal()
        };
        let eligibility = profile.constrain_interface(state, relay_interface());
        assert!(!eligibility.relay_opt_in);
        assert_eq!(
            profile
                .admit_operation(state, RELAYED_DIRECT, 64)
                .unwrap_err(),
            OperatingProfileError::RelayWorkNotPermitted
        );
    }

    #[test]
    fn mobile_relay_requires_platform_background_permission() {
        let profile = OperatingProfile::mobile_relay_opt_in();
        let state = DeviceRuntimeState::constrained_nominal();
        assert!(
            !profile
                .constrain_interface(state, relay_interface())
                .background_execution_permitted
        );
        assert_eq!(
            profile
                .admit_operation(state, RELAYED_DIRECT, 64)
                .unwrap_err(),
            OperatingProfileError::RelayWorkNotPermitted
        );
    }

    #[test]
    fn low_battery_revokes_new_mobile_relay_and_bulk_work_without_mutating_privacy() {
        let profile = OperatingProfile::mobile_relay_opt_in();
        let state = DeviceRuntimeState {
            battery_percent: Some(19),
            background_execution_permitted: true,
            ..DeviceRuntimeState::constrained_nominal()
        };
        let effective = profile.effective(state);
        assert!(!effective.allow_new_relay_work);
        assert!(!effective.allow_new_bulk_work);
        assert_eq!(
            profile
                .admit_operation(state, RELAYED_DIRECT, 64)
                .unwrap_err(),
            OperatingProfileError::RelayWorkNotPermitted
        );
        assert_eq!(
            profile.admit_operation(state, BULK, 64).unwrap_err(),
            OperatingProfileError::BulkWorkNotPermitted
        );
    }

    #[test]
    fn desktop_relay_retains_high_capacity_only_when_explicitly_configured() {
        let endpoint = OperatingProfile::desktop_endpoint();
        let relay = OperatingProfile::desktop_supervised_relay();
        let state = DeviceRuntimeState::desktop_nominal();
        assert_eq!(
            endpoint
                .admit_operation(state, RELAYED_DIRECT, 64)
                .unwrap_err(),
            OperatingProfileError::RelayWorkNotPermitted
        );
        let policy = relay.admit_operation(state, RELAYED_DIRECT, 64).unwrap();
        assert!(policy.allow_new_relay_work);
        assert_eq!(policy.limits.max_active_peers, 128);
    }

    #[test]
    fn critical_thermal_state_reduces_work_without_altering_delivery_requirement() {
        let profile = OperatingProfile::desktop_supervised_relay();
        let state = DeviceRuntimeState {
            thermal: ThermalState::Critical,
            ..DeviceRuntimeState::desktop_nominal()
        };
        let policy = profile.effective(state);
        assert!(!policy.allow_new_relay_work);
        assert_eq!(policy.limits.max_active_peers, 4);
        assert_eq!(
            profile
                .admit_operation(state, RELAYED_DIRECT, 64)
                .unwrap_err(),
            OperatingProfileError::RelayWorkNotPermitted
        );
        assert_eq!(
            RELAYED_DIRECT.minimum_privacy,
            PrivacyClass::RelationshipMetadataProtected
        );
    }

    #[test]
    fn radio_gateway_refuses_generic_bulk_work() {
        let profile = OperatingProfile::radio_gateway();
        let state = DeviceRuntimeState::desktop_nominal();
        assert_eq!(
            profile.admit_operation(state, BULK, 1).unwrap_err(),
            OperatingProfileError::BulkWorkNotPermitted
        );
    }

    #[test]
    fn mobile_bulk_limit_is_enforced_when_work_is_eligible() {
        let profile = OperatingProfile::mobile_endpoint();
        let state = DeviceRuntimeState {
            background_execution_permitted: true,
            ..DeviceRuntimeState::constrained_nominal()
        };
        assert_eq!(
            profile
                .admit_operation(state, BULK, 8 * 1024 * 1024 + 1)
                .unwrap_err(),
            OperatingProfileError::BulkObjectLimitExceeded {
                limit: 8 * 1024 * 1024
            }
        );
    }
}
