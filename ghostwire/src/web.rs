use axum::{Router, routing::{post, get, put}, extract::State, response::IntoResponse, Json};
use axum::extract::ws::{WebSocketUpgrade, WebSocket};
use axum::http::{HeaderValue, Method};
use axum::response::Response;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};

#[derive(Clone, Default)]
pub struct AppState {
    // Add shared state here (message cache, peers, etc.)
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub recipient: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct PeersResponse {
    pub peers: Vec<PeerInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PeerInfo {
    pub id: String,
    pub name: String,
    pub status: String,
    pub last_seen: String,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub stealth_mode: bool,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn status() -> impl IntoResponse {
    Json(ApiResponse {
        success: true,
        data: Some("GhostWire API is running"),
        error: None,
    })
}

pub async fn send_message(State(_state): State<Arc<AppState>>, Json(req): Json<SendMessageRequest>) -> impl IntoResponse {
    // TODO: Call core logic to send message
    println!("Sending message to {}: {}", req.recipient, req.message);
    
    Json(ApiResponse {
        success: true,
        data: Some("Message sent"),
        error: None,
    })
}

pub async fn get_peers(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    // TODO: Return actual list of peers from core logic
    let mock_peers = vec![
        PeerInfo {
            id: "peer1".to_string(),
            name: "Node-7A3F".to_string(),
            status: "online".to_string(),
            last_seen: "2 min ago".to_string(),
        },
        PeerInfo {
            id: "peer2".to_string(),
            name: "Node-B2E9".to_string(),
            status: "offline".to_string(),
            last_seen: "15 min ago".to_string(),
        },
    ];
    
    Json(ApiResponse {
        success: true,
        data: Some(PeersResponse { peers: mock_peers }),
        error: None,
    })
}

pub async fn get_settings(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    // TODO: Return actual settings from core logic
    Json(ApiResponse {
        success: true,
        data: Some(Settings { stealth_mode: false }),
        error: None,
    })
}

pub async fn update_settings(State(_state): State<Arc<AppState>>, Json(settings): Json<Settings>) -> impl IntoResponse {
    // TODO: Update settings in core logic
    println!("Updating settings: stealth_mode = {}", settings.stealth_mode);
    
    Json(ApiResponse {
        success: true,
        data: Some(settings),
        error: None,
    })
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // TODO: Real-time chat logic
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            // Echo back for now
            if let Err(_) = socket.send(msg).await {
                break;
            }
        }
    }
}

pub fn app(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_headers(Any);

    Router::new()
        .route("/api/status", get(status))
        .route("/api/send_message", post(send_message))
        .route("/api/peers", get(get_peers))
        .route("/api/settings", get(get_settings))
        .route("/api/settings", put(update_settings))
        .route("/ws", get(ws_handler))
        .layer(cors)
        .with_state(state)
} 