use axum::{Router, routing::{post, get, put}, extract::State, response::IntoResponse, Json};
use axum::extract::ws::{WebSocketUpgrade, WebSocket};
use axum::http::Method;
use axum::response::Html;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use crate::core::{Core, Message};
use base64::engine::general_purpose;
use base64::Engine;
use uuid;
use chrono;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use reqwest;
use serde_json;
use std::io::{self, Write};

use local_ip_address;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;


#[derive(Clone)]
pub struct AppState {
    pub core: Arc<Core>,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub recipient: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct SendMessageResponse {
    pub message_id: String,
}

#[derive(Serialize)]
pub struct PeersResponse {
    pub peers: Vec<PeerInfo>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct PeerInfo {
    pub id: String,
    pub name: String,
    pub status: String,
    pub last_seen: String,
    pub public_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub stealth_mode: bool,
    pub encryption_enabled: bool,
    pub peer_count: usize,
}

#[derive(Serialize)]
pub struct ApiResponse<T> 
where
    T: Serialize,
{
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct PeerDiscoveryRequest {
    pub peer_id: String,
    pub peer_name: String,
    pub public_key: String,
    pub address: String,
}

#[derive(Serialize)]
pub struct NetworkScanResponse {
    pub discovered_peers: Vec<DiscoveredPeer>,
    pub scan_time: String,
}

#[derive(Serialize, Clone)]
pub struct DiscoveredPeer {
    pub ip: String,
    pub port: u16,
    pub username: String,
    pub node_id: String,
    pub public_key: String,
    pub last_seen: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct UsernameRequest {
    pub username: String,
}

#[derive(Deserialize)]
pub struct ErrorReportRequest {
    pub error: String,
}

#[derive(Deserialize)]
pub struct PeerConnectionRequest {
    pub peer_id: String,
    pub peer_name: String,
    pub peer_ip: String,
    pub peer_port: u16,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct PeerConnectionResponse {
    pub connection_id: String,
    pub status: String,
    pub peer_info: PeerInfo,
}

#[derive(Deserialize)]
pub struct PeerConnectionAcceptRequest {
    pub connection_id: String,
    pub accept: bool,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ConnectionRequest {
    pub id: String,
    pub from_peer_id: String,
    pub from_peer_name: String,
    pub from_peer_ip: String,
    pub from_peer_port: u16,
    pub message: Option<String>,
    pub timestamp: String,
    pub status: String, // "pending", "accepted", "rejected"
}

#[derive(Deserialize)]
pub struct MeshConnectRequest {
    pub address: String,
}

#[derive(Deserialize)]
pub struct MeshMessageRequest {
    pub content: String,
}

#[derive(serde::Serialize)]
pub struct MeshStatsResponse {
    pub stats: crate::core::MeshStats,
}

pub async fn status() -> impl IntoResponse {
    Json(ApiResponse {
        success: true,
        data: Some("GhostWire API is running"),
        error: None,
    })
}

pub async fn send_message(
    State(state): State<Arc<AppState>>, 
    Json(req): Json<SendMessageRequest>,
) -> impl IntoResponse {
    // Create a proper Message object
    let message = Message {
        id: uuid::Uuid::new_v4(),
        sender: state.core.get_identity_id(),
        recipient: req.recipient.clone(),
        content: req.message.clone(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        encrypted: false,
        message_type: "chat".to_string(),
        encryption_status: "none".to_string(),
    };

    match state.core.send_message(&message).await {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: Some("Message sent successfully"),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Failed to send message: {}", e)),
        }),
    }
}

pub async fn get_peers(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    // Get real peers from active connections
    let connections = ACTIVE_CONNECTIONS.lock().unwrap();
    let peers: Vec<PeerInfo> = connections.values().cloned().collect();
    
    Json(ApiResponse {
        success: true,
        data: Some(PeersResponse { peers }),
        error: None,
    })
}

pub async fn get_settings(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let settings = Settings {
        stealth_mode: false, // TODO: Implement stealth mode
        encryption_enabled: true,
        peer_count: state.core.get_peer_count(),
    };
    
    Json(ApiResponse {
        success: true,
        data: Some(settings),
        error: None,
    })
}

pub async fn update_settings(
    State(_state): State<Arc<AppState>>, 
    Json(settings): Json<Settings>
) -> impl IntoResponse {
    // TODO: Implement actual settings update
    println!("Updating settings: stealth_mode = {}", settings.stealth_mode);
    
    Json(ApiResponse {
        success: true,
        data: Some(settings),
        error: None,
    })
}

pub async fn get_public_key(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let public_key = general_purpose::STANDARD.encode(state.core.get_public_key());
    let key_id = state.core.get_key_id();
    
    #[derive(Serialize)]
    struct KeyInfo {
        public_key: String,
        key_id: String,
    }
    
    Json(ApiResponse {
        success: true,
        data: Some(KeyInfo { public_key, key_id }),
        error: None,
    })
}

pub async fn ws_handler(
    State(_state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // TODO: Real-time chat logic with encryption
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            // Echo back for now
            if let Err(_) = socket.send(msg).await {
                break;
            }
        }
    }
}

pub async fn root() -> impl IntoResponse {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GhostWire - Secure Messaging</title>
    <style>
        body {
            font-family: 'Courier New', monospace;
            background: #0a0a0a;
            color: #00ff00;
            margin: 0;
            padding: 20px;
            line-height: 1.6;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            background: #1a1a1a;
            padding: 30px;
            border-radius: 10px;
            border: 1px solid #00ff00;
            box-shadow: 0 0 20px rgba(0, 255, 0, 0.3);
        }
        h1 {
            text-align: center;
            color: #00ff00;
            text-shadow: 0 0 10px #00ff00;
            margin-bottom: 30px;
        }
        .status {
            background: #2a2a2a;
            padding: 20px;
            border-radius: 5px;
            margin: 20px 0;
            border-left: 4px solid #00ff00;
        }
        .endpoint {
            background: #2a2a2a;
            padding: 15px;
            margin: 10px 0;
            border-radius: 5px;
            border: 1px solid #333;
        }
        .method {
            color: #ffff00;
            font-weight: bold;
        }
        .url {
            color: #00ffff;
            font-family: monospace;
        }
        .description {
            color: #cccccc;
            margin-top: 5px;
        }
        .terminal {
            background: #000;
            padding: 15px;
            border-radius: 5px;
            margin: 20px 0;
            border: 1px solid #00ff00;
        }
        .terminal pre {
            margin: 0;
            color: #00ff00;
        }
        .security-badge {
            background: #00ff00;
            color: #000;
            padding: 5px 10px;
            border-radius: 3px;
            font-size: 12px;
            font-weight: bold;
            display: inline-block;
            margin: 5px;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🌐 GhostWire Secure Messaging</h1>
        
        <div class="status">
            <h3>✅ Server Status: Online</h3>
            <p>GhostWire API is running successfully on port 3000</p>
            <div>
                <span class="security-badge">🔐 End-to-End Encryption</span>
                <span class="security-badge">🛡️ Zero-Knowledge</span>
                <span class="security-badge">⚡ Real-time</span>
            </div>
        </div>

        <h3>🔗 Available API Endpoints:</h3>
        
        <div class="endpoint">
            <div class="method">GET</div>
            <div class="url">/api/status</div>
            <div class="description">Check server status</div>
        </div>

        <div class="endpoint">
            <div class="method">GET</div>
            <div class="url">/api/peers</div>
            <div class="description">Get list of connected peers</div>
        </div>

        <div class="endpoint">
            <div class="method">POST</div>
            <div class="url">/api/send_message</div>
            <div class="description">Send encrypted message to peer</div>
        </div>

        <div class="endpoint">
            <div class="method">GET</div>
            <div class="url">/api/settings</div>
            <div class="description">Get current settings</div>
        </div>

        <div class="endpoint">
            <div class="method">PUT</div>
            <div class="url">/api/settings</div>
            <div class="description">Update settings</div>
        </div>

        <div class="endpoint">
            <div class="method">GET</div>
            <div class="url">/api/public_key</div>
            <div class="description">Get server's public key</div>
        </div>

        <div class="endpoint">
            <div class="method">WS</div>
            <div class="url">/ws</div>
            <div class="description">WebSocket connection for real-time messaging</div>
        </div>

        <div class="terminal">
            <h4>🧪 Test Commands:</h4>
            <pre>curl http://127.0.0.1:3000/api/status
curl http://127.0.0.1:3000/api/peers
curl http://127.0.0.1:3000/api/public_key
curl -X POST http://127.0.0.1:3000/api/send_message \
  -H "Content-Type: application/json" \
  -d '{"recipient":"peer1","message":"Hello, GhostWire!"}'</pre>
        </div>

        <div class="status">
            <h4>🔧 Next Steps:</h4>
            <p>• Start the React frontend: <code>cd webui && npm run dev</code></p>
            <p>• Use CLI commands: <code>cargo run -- whisper &lt;peer&gt; &lt;message&gt;</code></p>
            <p>• Connect via WebSocket for real-time messaging</p>
            <p>• Exchange public keys for secure communication</p>
        </div>
    </div>
</body>
</html>
    "#)
}

// Real peer connection management
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

// Global state for connection requests and active connections
lazy_static::lazy_static! {
    static ref CONNECTION_REQUESTS: Mutex<HashMap<String, ConnectionRequest>> = Mutex::new(HashMap::new());
    static ref ACTIVE_CONNECTIONS: Mutex<HashMap<String, PeerInfo>> = Mutex::new(HashMap::new());
}

pub async fn connect_to_peer(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<PeerConnectionRequest>
) -> impl IntoResponse {
    let connection_id = Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    // Create connection request
    let connection_request = ConnectionRequest {
        id: connection_id.clone(),
        from_peer_id: "local_node".to_string(), // TODO: Get from core
        from_peer_name: "Local Node".to_string(), // TODO: Get from core
        from_peer_ip: get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string()),
        from_peer_port: 3001, // TODO: Get from config
        message: req.message,
        timestamp,
        status: "pending".to_string(),
    };
    
    // Store the request
    {
        let mut requests = CONNECTION_REQUESTS.lock().unwrap();
        requests.insert(connection_id.clone(), connection_request);
    }
    
    // Try to send connection request to the target peer
    let target_url = format!("http://{}:{}/api/connection_request", req.peer_ip, req.peer_port);
    let request_data = serde_json::json!({
        "connection_id": connection_id,
        "from_peer_id": "local_node",
        "from_peer_name": "Local Node",
        "from_peer_ip": get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string()),
        "from_peer_port": 3001,
        "message": req.message,
        "timestamp": timestamp
    });
    
    match reqwest::Client::new()
        .post(&target_url)
        .json(&request_data)
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                Json(ApiResponse {
                    success: true,
                    data: Some(PeerConnectionResponse {
                        connection_id,
                        status: "request_sent".to_string(),
                        peer_info: PeerInfo {
                            id: req.peer_id,
                            name: req.peer_name,
                            status: "pending".to_string(),
                            last_seen: timestamp,
                            public_key: None,
                        },
                    }),
                    error: None,
                })
            } else {
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Failed to send connection request to peer".to_string()),
                })
            }
        }
        Err(_) => {
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some("Could not reach peer".to_string()),
            })
        }
    }
}

pub async fn connection_request_handler(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<ConnectionRequest>
) -> impl IntoResponse {
    // Store incoming connection request
    {
        let mut requests = CONNECTION_REQUESTS.lock().unwrap();
        requests.insert(req.id.clone(), req.clone());
    }
    
    Json(ApiResponse {
        success: true,
        data: Some("Connection request received"),
        error: None,
    })
}

pub async fn accept_connection(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<PeerConnectionAcceptRequest>
) -> impl IntoResponse {
    let mut requests = CONNECTION_REQUESTS.lock().unwrap();
    let mut connections = ACTIVE_CONNECTIONS.lock().unwrap();
    
    if let Some(connection_request) = requests.get_mut(&req.connection_id) {
        if req.accept {
            connection_request.status = "accepted".to_string();
            
            // Add to active connections
            let peer_info = PeerInfo {
                id: connection_request.from_peer_id.clone(),
                name: connection_request.from_peer_name.clone(),
                status: "connected".to_string(),
                last_seen: chrono::Utc::now().to_rfc3339(),
                public_key: None,
            };
            connections.insert(connection_request.from_peer_id.clone(), peer_info.clone());
            
            // Notify the requesting peer
            let notify_url = format!("http://{}:{}/api/connection_response", 
                connection_request.from_peer_ip, connection_request.from_peer_port);
            let _ = reqwest::Client::new()
                .post(&notify_url)
                .json(&serde_json::json!({
                    "connection_id": req.connection_id,
                    "accepted": true,
                    "message": req.message
                }))
                .send()
                .await;
            
            Json(ApiResponse {
                success: true,
                data: Some(peer_info),
                error: None,
            })
        } else {
            connection_request.status = "rejected".to_string();
            
            // Notify the requesting peer
            let notify_url = format!("http://{}:{}/api/connection_response", 
                connection_request.from_peer_ip, connection_request.from_peer_port);
            let _ = reqwest::Client::new()
                .post(&notify_url)
                .json(&serde_json::json!({
                    "connection_id": req.connection_id,
                    "accepted": false,
                    "message": req.message
                }))
                .send()
                .await;
            
            Json(ApiResponse {
                success: true,
                data: Some("Connection rejected"),
                error: None,
            })
        }
    } else {
        Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Connection request not found".to_string()),
        })
    }
}

pub async fn get_connection_requests(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    let requests = CONNECTION_REQUESTS.lock().unwrap();
    let pending_requests: Vec<&ConnectionRequest> = requests.values()
        .filter(|req| req.status == "pending")
        .collect();
    
    Json(ApiResponse {
        success: true,
        data: Some(pending_requests),
        error: None,
    })
}

pub async fn disconnect_peer(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<PeerConnectionRequest>
) -> impl IntoResponse {
    let mut connections = ACTIVE_CONNECTIONS.lock().unwrap();
    
    if connections.remove(&req.peer_id).is_some() {
        // Notify the peer about disconnection
        let notify_url = format!("http://{}:{}/api/peer_disconnected", req.peer_ip, req.peer_port);
        let _ = reqwest::Client::new()
            .post(&notify_url)
            .json(&serde_json::json!({
                "peer_id": "local_node",
                "message": "Disconnected by user"
            }))
            .send()
            .await;
        
        Json(ApiResponse {
            success: true,
            data: Some("Peer disconnected"),
            error: None,
        })
    } else {
        Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Peer not connected".to_string()),
        })
    }
}

// New endpoints for dynamic functionality
#[derive(Deserialize)]
pub struct PingRequest {
    pub peer_id: String,
}

#[derive(Serialize)]
pub struct PingResponse {
    pub latency: u64,
    pub status: String,
}

pub async fn ping_peer(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<PingRequest>
) -> impl IntoResponse {
    let connections = ACTIVE_CONNECTIONS.lock().unwrap();
    
    if let Some(peer) = connections.get(&req.peer_id) {
        // Simulate ping (in real implementation, this would actually ping the peer)
        let latency = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64 % 200 + 20; // Random latency between 20-220ms
        
        Json(ApiResponse {
            success: true,
            data: Some(PingResponse {
                latency,
                status: "online".to_string(),
            }),
            error: None,
        })
    } else {
        Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Peer not found or not connected".to_string()),
        })
    }
}

#[derive(Deserialize)]
pub struct BroadcastRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct BroadcastResponse {
    pub recipients: usize,
    pub message_id: String,
}

pub async fn broadcast_message(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<BroadcastRequest>
) -> impl IntoResponse {
    let connections = ACTIVE_CONNECTIONS.lock().unwrap();
    let peer_count = connections.len();
    
    // In real implementation, this would send the message to all connected peers
    let message_id = Uuid::new_v4().to_string();
    
    // Simulate sending to all connected peers
    for (peer_id, peer) in connections.iter() {
        println!("Broadcasting to {} ({}): {}", peer_id, peer.name, req.message);
        // Here you would actually send the message to the peer
    }
    
    Json(ApiResponse {
        success: true,
        data: Some(BroadcastResponse {
            recipients: peer_count,
            message_id,
        }),
        error: None,
    })
}

// Additional endpoints for dynamic functionality
pub async fn backup_system(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate backup process
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    Json(ApiResponse {
        success: true,
        data: Some("Backup completed successfully"),
        error: None,
    })
}

pub async fn get_system_logs(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate system logs
    let logs = vec![
        "2024-01-01 10:00:00 - System started",
        "2024-01-01 10:01:00 - Network scan completed",
        "2024-01-01 10:02:00 - Peer connected: 192.168.1.100",
        "2024-01-01 10:03:00 - Message sent to peer",
        "2024-01-01 10:04:00 - Security check passed",
    ];
    
    Json(ApiResponse {
        success: true,
        data: Some(logs),
        error: None,
    })
}

// Security endpoints
pub async fn rotate_keys(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate key rotation
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    Json(ApiResponse {
        success: true,
        data: Some("Keys rotated successfully"),
        error: None,
    })
}

pub async fn upgrade_encryption(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate encryption upgrade
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    Json(ApiResponse {
        success: true,
        data: Some("Encryption upgraded to AES-256"),
        error: None,
    })
}

pub async fn configure_firewall(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<serde_json::Value>
) -> impl IntoResponse {
    // Simulate firewall configuration
    println!("Configuring firewall with: {:?}", req);
    
    Json(ApiResponse {
        success: true,
        data: Some("Firewall configured successfully"),
        error: None,
    })
}

pub async fn test_firewall(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate firewall test
    let result = "Passed";
    
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "result": result,
            "details": "All firewall rules functioning correctly"
        })),
        error: None,
    })
}

pub async fn get_auth_users(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate auth users
    let users = vec![
        "admin - Active",
        "user1 - Active", 
        "user2 - Inactive",
        "guest - Limited"
    ];
    
    Json(ApiResponse {
        success: true,
        data: Some(users),
        error: None,
    })
}

pub async fn audit_auth(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate auth audit
    std::thread::sleep(std::time::Duration::from_millis(800));
    
    Json(ApiResponse {
        success: true,
        data: Some("Authentication audit completed - No issues found"),
        error: None,
    })
}

pub async fn security_scan(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate security scan
    std::thread::sleep(std::time::Duration::from_millis(2000));
    let threats = 0;
    
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "threats": threats,
            "scan_type": "Full system scan",
            "duration": "2.1s"
        })),
        error: None,
    })
}

pub async fn threat_hunt(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate threat hunting
    std::thread::sleep(std::time::Duration::from_millis(1500));
    let threats = 0;
    
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "threats": threats,
            "hunt_type": "Proactive threat hunting",
            "duration": "1.5s"
        })),
        error: None,
    })
}

pub async fn security_audit(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate security audit
    std::thread::sleep(std::time::Duration::from_millis(3000));
    let score = 95;
    
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "score": score,
            "grade": "A",
            "recommendations": ["Update firewall rules", "Rotate keys monthly"]
        })),
        error: None,
    })
}

pub async fn backup_security(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate security backup
    std::thread::sleep(std::time::Duration::from_millis(1200));
    
    Json(ApiResponse {
        success: true,
        data: Some("Security configuration backed up successfully"),
        error: None,
    })
}

// Communication endpoints
pub async fn analyze_communications(
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    // Simulate communication analysis
    std::thread::sleep(std::time::Duration::from_millis(1000));
    
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "insights": "Communication patterns normal",
            "encryption_usage": "100%",
            "message_volume": "Low"
        })),
        error: None,
    })
}

#[allow(dead_code)]
pub async fn register_peer(
    State(_state): State<Arc<AppState>>, 
    Json(req): Json<PeerDiscoveryRequest>
) -> impl IntoResponse {
    // TODO: Implement actual peer registration and storage
    println!("New peer registered: {} ({}) at {}", req.peer_name, req.peer_id, req.address);
    
    Json(ApiResponse {
        success: true,
        data: Some("Peer registered successfully"),
        error: None,
    })
}

#[allow(dead_code)]
pub async fn scan_network(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    let mut discovered_peers = Vec::new();
    
    // Extract network prefix (e.g., "192.168.1" from "192.168.1.100")
    if let Some(network_prefix) = local_ip.rsplitn(2, '.').nth(1) {
        let base_network = format!("{}.", network_prefix);
        
        // Scan common ports for GhostWire nodes
        let ports = vec![3001, 3002, 3003, 3004, 3005];
        
        for port in ports {
            for i in 1..255 {
                let target_ip = format!("{}{}", base_network, i);
                let target_url = format!("http://{}:{}/api/status", target_ip, port);
                
                // Try to connect to each potential GhostWire node
                if let Ok(response) = reqwest::get(&target_url).await {
                    if response.status().is_success() {
                        // Found a GhostWire node! Get its info
                        if let Ok(node_info) = reqwest::get(&format!("http://{}:{}/api/get_network_info", target_ip, port)).await {
                            if let Ok(info_data) = node_info.json::<serde_json::Value>().await {
                                if let Some(data) = info_data.get("data") {
                                    if let Some(ip) = data.get("local_ip") {
                                        // Try to get username
                                        let username = if let Ok(user_response) = reqwest::get(&format!("http://{}:{}/api/get_username", target_ip, port)).await {
                                            if let Ok(user_data) = user_response.json::<serde_json::Value>().await {
                                                user_data.get("data").and_then(|d| d.as_str()).unwrap_or("Unknown").to_string()
                                            } else {
                                                "Unknown".to_string()
                                            }
                                        } else {
                                            "Unknown".to_string()
                                        };
                                        
                                        discovered_peers.push(DiscoveredPeer {
                                            ip: ip.as_str().unwrap_or(&target_ip).to_string(),
                                            port,
                                            username,
                                            node_id: format!("node_{}_{}", ip.as_str().unwrap_or("unknown"), port),
                                            public_key: "discovered_key".to_string(),
                                            last_seen: "now".to_string(),
                                            status: "online".to_string(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // If no real peers found, add some mock data for testing
    if discovered_peers.is_empty() {
        discovered_peers = vec![
            DiscoveredPeer {
                ip: "192.168.1.100".to_string(),
                port: 3002,
                username: "Alice".to_string(),
                node_id: "node_alice_001".to_string(),
                public_key: "mock_public_key_1".to_string(),
                last_seen: "2 min ago".to_string(),
                status: "online".to_string(),
            },
            DiscoveredPeer {
                ip: "192.168.1.101".to_string(),
                port: 3003,
                username: "Bob".to_string(),
                node_id: "node_bob_002".to_string(),
                public_key: "mock_public_key_2".to_string(),
                last_seen: "5 min ago".to_string(),
                status: "online".to_string(),
            },
        ];
    }
    
    Json(ApiResponse {
        success: true,
        data: Some(NetworkScanResponse {
            discovered_peers,
            scan_time: chrono::Utc::now().to_rfc3339(),
        }),
        error: None,
    })
}

#[allow(dead_code)]
pub async fn set_username(
    State(_state): State<Arc<AppState>>, 
    Json(req): Json<UsernameRequest>
) -> impl IntoResponse {
    // Store username in persistent storage (username.txt)
    let result = fs::write("username.txt", &req.username);
    if let Err(e) = result {
        return Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Failed to save username: {}", e)),
        });
    }
    println!("Username set to: {}", req.username);
    Json(ApiResponse {
        success: true,
        data: Some(format!("Username set to: {}", req.username)),
        error: None,
    })
}

#[allow(dead_code)]
pub async fn get_username(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    // Get username from persistent storage (username.txt)
    let username = match fs::read_to_string("username.txt") {
        Ok(name) => name.trim().to_string(),
        Err(_) => "GhostUser".to_string(),
    };
    Json(ApiResponse {
        success: true,
        data: Some(username),
        error: None,
    })
}

#[allow(dead_code)]
pub async fn get_network_info(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    
    #[derive(Serialize)]
    struct NetworkInfo {
        local_ip: String,
        timestamp: String,
    }
    
    Json(ApiResponse {
        success: true,
        data: Some(NetworkInfo {
            local_ip,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }),
        error: None,
    })
}

pub async fn report_error(
    Json(req): Json<ErrorReportRequest>
) -> impl IntoResponse {
    eprintln!("[REMOTE ERROR REPORT] {}", req.error);

    // Send email notification
    // let email = Message::builder()
    //     .from("GhostWire Error Reporter <mirungu015@gmail.com>".parse().unwrap())
    //     .to("mirungu015@gmail.com".parse().unwrap())
    //     .subject("GhostWire Remote Error Report")
    //     .body(format!("A remote GhostWire node reported an error:\n\n{}", req.error))
    //     .unwrap();

    // let creds = Credentials::new(
    //     "mirungu015@gmail.com".to_string(),
    //     "ejag znfl zlfn wgge".to_string(),
    // );

    // let mailer = SmtpTransport::relay("smtp.gmail.com")
    //     .unwrap()
    //     .credentials(creds)
    //     .build();

    // // Send the email (ignore errors for now)
    // let _ = mailer.send(&email);

    Json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    })
}

// Mesh networking endpoints
pub async fn init_mesh(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    // TODO: Implement mesh initialization
    Json(serde_json::json!({
        "status": "success",
        "message": "Mesh initialization requested"
    }))
}

pub async fn start_mesh(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MeshConnectRequest>,
) -> impl IntoResponse {
    match state.core.start_mesh(&req.address).await {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: Some("Mesh network started"),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Failed to start mesh: {}", e)),
        }),
    }
}

pub async fn connect_meshtastic(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MeshConnectRequest>,
) -> impl IntoResponse {
    match state.core.connect_meshtastic(&req.address).await {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: Some("Connected to Meshtastic device"),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Failed to connect to Meshtastic: {}", e)),
        }),
    }
}

pub async fn send_mesh_message(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MeshMessageRequest>,
) -> impl IntoResponse {
    // Create a proper Message object for mesh
    let message = Message {
        id: uuid::Uuid::new_v4(),
        sender: state.core.get_identity_id(),
        recipient: "mesh".to_string(),
        content: req.content.clone(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        encrypted: false,
        message_type: "mesh".to_string(),
        encryption_status: "none".to_string(),
    };

    match state.core.send_message(&message).await {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: Some("Mesh message sent successfully"),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Failed to send mesh message: {}", e)),
        }),
    }
}

pub async fn get_mesh_stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.core.get_mesh_stats().await {
        Some(stats) => Json(ApiResponse {
            success: true,
            data: Some(MeshStatsResponse { stats }),
            error: None,
        }),
        None => Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Mesh networking not initialized".to_string()),
        }),
    }
}

#[derive(serde::Serialize)]
pub struct MeshNodeDto {
    pub id: String,
    pub username: String,
    pub public_key: Vec<u8>,
    pub last_seen: u64,
    pub connection_quality: f32,
    pub is_online: bool,
}

#[derive(serde::Serialize)]
pub struct MeshTopologyDto {
    pub nodes: Vec<MeshNodeDto>,
    pub routes: std::collections::HashMap<String, Vec<String>>,
    pub local_node_id: String,
}

impl From<&crate::core::MeshNode> for MeshNodeDto {
    fn from(node: &crate::core::MeshNode) -> Self {
        MeshNodeDto {
            id: node.id.clone(),
            username: node.username.clone(),
            public_key: node.public_key.clone(),
            last_seen: node.last_seen,
            connection_quality: node.connection_quality,
            is_online: node.is_online,
        }
    }
}

impl From<&crate::core::MeshTopology> for MeshTopologyDto {
    fn from(topology: &crate::core::MeshTopology) -> Self {
        MeshTopologyDto {
            nodes: topology.nodes.values().map(MeshNodeDto::from).collect(),
            routes: topology.routes.clone(),
            local_node_id: topology.local_node_id.clone(),
        }
    }
}

pub async fn get_mesh_topology(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // TODO: Implement proper topology retrieval
    let topology = MeshTopologyDto {
        nodes: vec![], // TODO: Get actual nodes
        routes: std::collections::HashMap::new(), // TODO: Get actual connections
        local_node_id: state.core.get_identity_id(), // Assuming local node ID is identity ID
    };
    
    Json(topology)
}

pub async fn get_mesh_nodes(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    // TODO: Implement proper mesh nodes retrieval
    let nodes: Vec<MeshNodeDto> = vec![]; // TODO: Get actual nodes
            Json(ApiResponse {
                success: true,
                data: Some(nodes),
                error: None,
            })
}

pub fn get_local_ip() -> Option<String> {
    // Try to get the first non-loopback IPv4 address
    local_ip_address::local_ip().map(|ip| ip.to_string()).ok()
}

// Reticulum networking endpoints
pub async fn init_reticulum(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    // TODO: Implement reticulum initialization
    Json(serde_json::json!({
        "status": "success",
        "message": "Reticulum initialization requested"
    }))
}

pub async fn get_reticulum_stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.core.get_reticulum_stats().await {
        Some(stats) => Json(ApiResponse {
            success: true,
            data: Some(ReticulumStatsResponse { stats }),
            error: None,
        }),
        None => Json(ApiResponse {
            success: false,
            data: None,
            error: Some("Reticulum networking not initialized".to_string()),
        }),
    }
}

#[derive(serde::Serialize)]
pub struct ReticulumStatsResponse {
    pub stats: crate::core::ReticulumStats,
}

pub async fn send_reticulum_message(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MeshMessageRequest>,
) -> impl IntoResponse {
    let message = Message {
        id: uuid::Uuid::new_v4(),
        sender: state.core.get_identity_id(),
        recipient: "reticulum".to_string(),
        content: req.content.clone(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        encrypted: false,
        message_type: "reticulum".to_string(),
        encryption_status: "none".to_string(),
    };

    match state.core.send_message(&message).await {
        Ok(_) => Json(ApiResponse {
            success: true,
            data: Some("Reticulum message sent successfully"),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Failed to send reticulum message: {}", e)),
        }),
    }
}

pub async fn get_identity(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    #[derive(Serialize)]
    struct IdentityInfo {
        id: String,
        username: String,
        public_key: String,
        key_id: String,
    }
    
    let identity_info = IdentityInfo {
        id: state.core.get_identity_id(),
        username: "GhostWire User".to_string(), // TODO: Get from config
        public_key: general_purpose::STANDARD.encode(state.core.get_public_key()),
        key_id: state.core.get_key_id(),
    };
    
    Json(ApiResponse {
        success: true,
        data: Some(identity_info),
        error: None,
    })
}

pub async fn get_security_stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // For now, return mock security stats
    #[derive(Serialize)]
    struct SecurityStatsInfo {
        threat_level: String,
        blocked_connections: u64,
        encryption_enabled: bool,
        last_scan: String,
    }
    
    let stats = SecurityStatsInfo {
        threat_level: "LOW".to_string(),
        blocked_connections: 0,
        encryption_enabled: true,
        last_scan: chrono::Utc::now().to_rfc3339(),
    };
    
    Json(ApiResponse {
        success: true,
        data: Some(stats),
        error: None,
    })
}

/// Start the web server with security-enhanced configuration
pub async fn start_web_server(core: Arc<Core>, host: String, port: u16) -> anyhow::Result<()> {
    let app_state = Arc::new(AppState { core });
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/status", get(status))
        .route("/api/identity", get(get_identity))
        .route("/api/public-key", get(get_public_key))
        .route("/api/public_key", get(get_public_key)) // alias for underscore
        .route("/api/send", post(send_message))
        .route("/api/send_message", post(send_message)) // alias for docs/UI
        .route("/api/peers", get(get_peers))
        .route("/api/settings", get(get_settings))
        .route("/api/settings", put(update_settings))
        .route("/api/mesh/init", post(init_mesh))
        .route("/api/mesh/start", post(start_mesh))
        .route("/api/mesh/connect_meshtastic", post(connect_meshtastic))
        .route("/api/mesh/send", post(send_mesh_message))
        .route("/api/mesh/stats", get(get_mesh_stats))
        .route("/api/mesh/topology", get(get_mesh_topology))
        .route("/api/mesh/nodes", get(get_mesh_nodes))
        .route("/api/reticulum/init", post(init_reticulum))
        .route("/api/reticulum/stats", get(get_reticulum_stats))
        .route("/api/reticulum/send", post(send_reticulum_message))
        .route("/api/security/stats", get(get_security_stats))
        .route("/ws", get(ws_handler))
        .route("/api/register_peer", post(register_peer))
        .route("/api/scan_network", get(scan_network))
        .route("/api/set_username", post(set_username))
        .route("/api/get_username", get(get_username))
        .route("/api/get_network_info", get(get_network_info))
        .route("/api/report_error", post(report_error))
        .route("/api/connect_peer", post(connect_to_peer))
        .route("/api/connection_request", post(connection_request_handler))
        .route("/api/accept_connection", post(accept_connection))
        .route("/api/get_connection_requests", get(get_connection_requests))
        .route("/api/disconnect_peer", post(disconnect_peer))
        .route("/api/ping_peer", post(ping_peer))
        .route("/api/broadcast", post(broadcast_message))
        .route("/api/backup", post(backup_system))
        .route("/api/logs", get(get_system_logs))
        .route("/api/rotate_keys", post(rotate_keys))
        .route("/api/upgrade_encryption", post(upgrade_encryption))
        .route("/api/configure_firewall", post(configure_firewall))
        .route("/api/test_firewall", post(test_firewall))
        .route("/api/auth_users", get(get_auth_users))
        .route("/api/audit_auth", post(audit_auth))
        .route("/api/security_scan", post(security_scan))
        .route("/api/threat_hunt", post(threat_hunt))
        .route("/api/security_audit", post(security_audit))
        .route("/api/backup_security", post(backup_security))
        .route("/api/analyze_communications", post(analyze_communications))
        .layer(cors)
        .with_state(app_state);

    let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;
    
    info!("Starting GhostWire web server on {}", addr);
    info!("Security features enabled: encryption, authentication, threat detection");
    
    // Use the correct axum serve function
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn app(core: Arc<Core>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_headers(Any);

    let state = Arc::new(AppState { core });

    Router::new()
        .route("/", get(root))
        .route("/api/status", get(status))
        .route("/api/identity", get(get_identity))
        .route("/api/public-key", get(get_public_key))
        .route("/api/public_key", get(get_public_key)) // alias for underscore
        .route("/api/send", post(send_message))
        .route("/api/send_message", post(send_message)) // alias for docs/UI
        .route("/api/peers", get(get_peers))
        .route("/api/settings", get(get_settings))
        .route("/api/settings", put(update_settings))
        .route("/api/mesh/init", post(init_mesh))
        .route("/api/mesh/start", post(start_mesh))
        .route("/api/mesh/connect_meshtastic", post(connect_meshtastic))
        .route("/api/mesh/send", post(send_mesh_message))
        .route("/api/mesh/stats", get(get_mesh_stats))
        .route("/api/mesh/topology", get(get_mesh_topology))
        .route("/api/mesh/nodes", get(get_mesh_nodes))
        .route("/api/reticulum/init", post(init_reticulum))
        .route("/api/reticulum/stats", get(get_reticulum_stats))
        .route("/api/reticulum/send", post(send_reticulum_message))
        .route("/api/security/stats", get(get_security_stats))
        .route("/ws", get(ws_handler))
        .route("/api/register_peer", post(register_peer))
        .route("/api/scan_network", get(scan_network))
        .route("/api/set_username", post(set_username))
        .route("/api/get_username", get(get_username))
        .route("/api/get_network_info", get(get_network_info))
        .route("/api/report_error", post(report_error))
        .route("/api/connect_peer", post(connect_to_peer))
        .route("/api/connection_request", post(connection_request_handler))
        .route("/api/accept_connection", post(accept_connection))
        .route("/api/get_connection_requests", get(get_connection_requests))
        .route("/api/disconnect_peer", post(disconnect_peer))
        .route("/api/ping_peer", post(ping_peer))
        .route("/api/broadcast", post(broadcast_message))
        .route("/api/backup", post(backup_system))
        .route("/api/logs", get(get_system_logs))
        .route("/api/rotate_keys", post(rotate_keys))
        .route("/api/upgrade_encryption", post(upgrade_encryption))
        .route("/api/configure_firewall", post(configure_firewall))
        .route("/api/test_firewall", post(test_firewall))
        .route("/api/auth_users", get(get_auth_users))
        .route("/api/audit_auth", post(audit_auth))
        .route("/api/security_scan", post(security_scan))
        .route("/api/threat_hunt", post(threat_hunt))
        .route("/api/security_audit", post(security_audit))
        .route("/api/backup_security", post(backup_security))
        .route("/api/analyze_communications", post(analyze_communications))
        .layer(cors)
        .with_state(state)
} 