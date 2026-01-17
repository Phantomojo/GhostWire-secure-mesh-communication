use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TutorialStep {
    pub id: String,
    pub title: String,
    pub content: String,
    pub selector: Option<String>,
}

pub fn get_tutorial_steps() -> Vec<TutorialStep> {
    vec![
        TutorialStep {
            id: "welcome".to_string(),
            title: "Welcome to GhostWire".to_string(),
            content: "GhostWire is a secure, decentralized mesh communication platform. This tutorial will guide you through the key features of the interface.".to_string(),
            selector: None,
        },
        TutorialStep {
            id: "hud".to_string(),
            title: "Tactical HUD".to_string(),
            content: "Monitor your system's heartbeat here. You can see real-time CPU and RAM usage, along with your current Mesh security score.".to_string(),
            selector: Some("#tactical-hud".to_string()),
        },
        TutorialStep {
            id: "topology".to_string(),
            title: "Network Topology".to_string(),
            content: "This interactive radar shows your position in the mesh network and all discovered peers. Click on a node to see its details.".to_string(),
            selector: Some("#mesh-topology".to_string()),
        },
        TutorialStep {
            id: "chat".to_string(),
            title: "Secure Messaging".to_string(),
            content: "Send end-to-end encrypted messages to any peer in the network. GhostWire automatically handles key exchange and routing.".to_string(),
            selector: Some("#chat-interface".to_string()),
        },
        TutorialStep {
            id: "security".to_string(),
            title: "Command & Control".to_string(),
            content: "Access advanced security features here, including Lockdown mode, Threat Hunting, and Key Rotation.".to_string(),
            selector: Some("#security-controls".to_string()),
        },
        TutorialStep {
            id: "config".to_string(),
            title: "System Configuration".to_string(),
            content: "Customize your node settings, manage transports (Bluetooth, LoRa, TCP), and view detailed system logs.".to_string(),
            selector: Some("#config-button".to_string()),
        },
        TutorialStep {
            id: "finish".to_string(),
            title: "Ready for Deployment".to_string(),
            content: "You're now ready to use GhostWire. Stay secure, stay decentralized, stay unstoppable.".to_string(),
            selector: None,
        },
    ]
}
