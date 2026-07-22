pub mod all_controllers; // Backward compatibility re-export
pub mod ark;
pub mod auth;
pub mod channel;
pub mod health;
pub mod identity;
pub mod messaging;
pub mod network;
pub mod pairing;
pub mod peer;
pub mod peer_controller;
pub mod profiles;
pub mod recovery;
pub mod system;
pub mod trust;
pub mod ws_handler;

// Re-export individual controllers (split from all_controllers.rs)
pub use ark::ArkController;
pub use channel::ChannelController;
pub use network::NetworkController;
pub use pairing::PairingController;
pub use recovery::RecoveryController;
pub use trust::TrustController;

// Existing controllers
pub use auth::AuthController;
pub use health::HealthController;
pub use identity::IdentityController;
pub use messaging::MessagingController;
pub use peer::PeerController as PeerControllerV1;
pub use peer_controller::PeerController;
pub use profiles::ProfilesController;
pub use system::SystemController;

pub use ws_handler::{handle_socket, ws_handler};
