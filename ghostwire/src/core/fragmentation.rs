//! Carrier-specific fragmentation planning.
//!
//! Fragmentation is **not** transport neutral. A link's MTU, stream semantics,
//! receiver memory budget, loss/reorder profile, airtime cost, and cryptographic
//! framing determine whether a large GhostWire object can cross it safely. This
//! module only makes that decision. It does not split ciphertext itself: doing
//! so before a common authenticated fragment envelope exists would create an
//! unauthenticated reassembly/DoS surface and would falsely imply that all
//! carriers support the same reliability semantics.
//!
//! The caller must either use a native carrier resource/stream mechanism or a
//! private verified block-transfer session after receiving a plan that requires
//! one. A `Rejected` outcome is a normal privacy and safety result, not an
//! invitation to silently try a different carrier or a smaller privacy class.

use std::fmt;

/// Maximum aggregate object size the initial private bulk-transfer design may
/// stage in memory. Larger objects require a separately streamed storage API.
pub const MAX_PRIVATE_BULK_OBJECT_BYTES: usize = 64 * 1024 * 1024;

/// A factual description of a carrier's payload and native transfer behavior.
/// It deliberately does not describe privacy; that belongs to transport policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CarrierFragmentation {
    /// A reliable stream with explicit backpressure. The carrier owns framing;
    /// GhostWire sends bounded logical frames and never adds message fragments.
    Stream { max_logical_frame_bytes: usize },
    /// A packet carrier whose official runtime provides a specific large-object
    /// facility. `max_packet_payload_bytes` is *not* an invitation to split
    /// bytes ourselves; objects beyond it must use the named native facility.
    NativeResource {
        max_packet_payload_bytes: usize,
        max_resource_bytes: usize,
    },
    /// A strict single-packet carrier. Oversize input must fail. This is the
    /// current official-RNS bridge packet mode and the safe baseline for most
    /// low-rate/radio adapters before their native resource protocol is wired.
    SinglePacket { max_payload_bytes: usize },
    /// An intentionally negotiated private bulk-transfer session. Immutable
    /// encrypted blocks must be verified individually and requests must travel
    /// only through the already-private session/path selected by policy.
    PrivateBlockExchange {
        max_block_bytes: usize,
        max_object_bytes: usize,
    },
    /// A carrier has no safe large-object contract yet.
    Unsupported,
}

/// What the adapter must do after policy selects a carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FragmentationPlan {
    /// Submit this object as one carrier payload/frame.
    Direct,
    /// Use the carrier's documented resource/link transfer facility. This plan
    /// does not authorize GhostWire application-level fragmentation.
    NativeResource,
    /// Establish a separately authorized private bulk-transfer session and
    /// exchange integrity-checked blocks. This must not use public DHT provider
    /// records, public availability announcements, or ambient wantlists.
    PrivateBlockExchange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FragmentationError {
    EmptyObject,
    CarrierLimitInvalid,
    SinglePacketLimitExceeded {
        object_bytes: usize,
        max_payload_bytes: usize,
    },
    NativeResourceLimitExceeded {
        object_bytes: usize,
        max_resource_bytes: usize,
    },
    PrivateBulkObjectLimitExceeded {
        object_bytes: usize,
        max_object_bytes: usize,
    },
    PrivateBulkBlockLimitInvalid,
    Unsupported,
}

impl fmt::Display for FragmentationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObject => write!(f, "FRAGMENTATION_EMPTY_OBJECT"),
            Self::CarrierLimitInvalid => write!(f, "FRAGMENTATION_INVALID_CARRIER_LIMIT"),
            Self::SinglePacketLimitExceeded { .. } => {
                write!(f, "FRAGMENTATION_SINGLE_PACKET_LIMIT_EXCEEDED")
            }
            Self::NativeResourceLimitExceeded { .. } => {
                write!(f, "FRAGMENTATION_NATIVE_RESOURCE_LIMIT_EXCEEDED")
            }
            Self::PrivateBulkObjectLimitExceeded { .. } => {
                write!(f, "FRAGMENTATION_PRIVATE_BULK_LIMIT_EXCEEDED")
            }
            Self::PrivateBulkBlockLimitInvalid => {
                write!(f, "FRAGMENTATION_INVALID_PRIVATE_BULK_BLOCK_LIMIT")
            }
            Self::Unsupported => write!(f, "FRAGMENTATION_UNSUPPORTED_CARRIER"),
        }
    }
}

impl std::error::Error for FragmentationError {}

/// Decide the only allowed large-object mode for a selected carrier.
///
/// The object length must be the length *after* GhostWire's message-level
/// encryption and authentication preparation, because carrier overhead and
/// actual ciphertext expansion are not inferable from plaintext length.
pub fn plan(
    carrier: CarrierFragmentation,
    protected_object_bytes: usize,
) -> Result<FragmentationPlan, FragmentationError> {
    if protected_object_bytes == 0 {
        return Err(FragmentationError::EmptyObject);
    }

    match carrier {
        CarrierFragmentation::Stream {
            max_logical_frame_bytes,
        } => {
            if max_logical_frame_bytes == 0 {
                return Err(FragmentationError::CarrierLimitInvalid);
            }
            if protected_object_bytes <= max_logical_frame_bytes {
                Ok(FragmentationPlan::Direct)
            } else if protected_object_bytes <= MAX_PRIVATE_BULK_OBJECT_BYTES {
                // Stream adapters may subsequently open the private bulk
                // protocol; a logical-frame limit is not permission to splice
                // arbitrary application fragments into the stream.
                Ok(FragmentationPlan::PrivateBlockExchange)
            } else {
                Err(FragmentationError::PrivateBulkObjectLimitExceeded {
                    object_bytes: protected_object_bytes,
                    max_object_bytes: MAX_PRIVATE_BULK_OBJECT_BYTES,
                })
            }
        }
        CarrierFragmentation::NativeResource {
            max_packet_payload_bytes,
            max_resource_bytes,
        } => {
            if max_packet_payload_bytes == 0
                || max_resource_bytes == 0
                || max_packet_payload_bytes > max_resource_bytes
            {
                return Err(FragmentationError::CarrierLimitInvalid);
            }
            if protected_object_bytes <= max_packet_payload_bytes {
                Ok(FragmentationPlan::Direct)
            } else if protected_object_bytes <= max_resource_bytes {
                Ok(FragmentationPlan::NativeResource)
            } else {
                Err(FragmentationError::NativeResourceLimitExceeded {
                    object_bytes: protected_object_bytes,
                    max_resource_bytes,
                })
            }
        }
        CarrierFragmentation::SinglePacket { max_payload_bytes } => {
            if max_payload_bytes == 0 {
                return Err(FragmentationError::CarrierLimitInvalid);
            }
            if protected_object_bytes <= max_payload_bytes {
                Ok(FragmentationPlan::Direct)
            } else {
                Err(FragmentationError::SinglePacketLimitExceeded {
                    object_bytes: protected_object_bytes,
                    max_payload_bytes,
                })
            }
        }
        CarrierFragmentation::PrivateBlockExchange {
            max_block_bytes,
            max_object_bytes,
        } => {
            if max_block_bytes == 0 || max_object_bytes == 0 || max_block_bytes > max_object_bytes {
                return Err(FragmentationError::PrivateBulkBlockLimitInvalid);
            }
            if protected_object_bytes <= max_block_bytes {
                Ok(FragmentationPlan::Direct)
            } else if protected_object_bytes <= max_object_bytes {
                Ok(FragmentationPlan::PrivateBlockExchange)
            } else {
                Err(FragmentationError::PrivateBulkObjectLimitExceeded {
                    object_bytes: protected_object_bytes,
                    max_object_bytes,
                })
            }
        }
        CarrierFragmentation::Unsupported => Err(FragmentationError::Unsupported),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rns_packet_mode_refuses_oversize_payload_without_splitting() {
        let carrier = CarrierFragmentation::SinglePacket {
            max_payload_bytes: 383,
        };
        assert_eq!(plan(carrier, 383), Ok(FragmentationPlan::Direct));
        assert_eq!(
            plan(carrier, 384),
            Err(FragmentationError::SinglePacketLimitExceeded {
                object_bytes: 384,
                max_payload_bytes: 383,
            })
        );
    }

    #[test]
    fn native_resource_is_the_only_allowed_path_above_packet_mtu() {
        let carrier = CarrierFragmentation::NativeResource {
            max_packet_payload_bytes: 383,
            max_resource_bytes: 8 * 1024 * 1024,
        };
        assert_eq!(plan(carrier, 383), Ok(FragmentationPlan::Direct));
        assert_eq!(plan(carrier, 384), Ok(FragmentationPlan::NativeResource));
    }

    #[test]
    fn stream_falls_back_to_private_block_exchange_not_generic_chunks() {
        let carrier = CarrierFragmentation::Stream {
            max_logical_frame_bytes: 64 * 1024,
        };
        assert_eq!(plan(carrier, 4096), Ok(FragmentationPlan::Direct));
        assert_eq!(
            plan(carrier, 128 * 1024),
            Ok(FragmentationPlan::PrivateBlockExchange)
        );
    }

    #[test]
    fn private_bulk_is_strictly_bounded() {
        let carrier = CarrierFragmentation::PrivateBlockExchange {
            max_block_bytes: 256 * 1024,
            max_object_bytes: 1024 * 1024,
        };
        assert_eq!(plan(carrier, 256 * 1024), Ok(FragmentationPlan::Direct));
        assert_eq!(
            plan(carrier, 256 * 1024 + 1),
            Ok(FragmentationPlan::PrivateBlockExchange)
        );
        assert!(matches!(
            plan(carrier, 1024 * 1024 + 1),
            Err(FragmentationError::PrivateBulkObjectLimitExceeded { .. })
        ));
    }
}
