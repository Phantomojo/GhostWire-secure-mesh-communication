use axum::{Router, routing::{post, get, put}, extract::State, response::IntoResponse, Json};
use axum::extract::ws::{WebSocketUpgrade, WebSocket};
use axum::http::{HeaderValue, Method};
use axum::response::{Response, Html};
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
    </style>
</head>
<body>
    <div class="container">
        <h1>🌐 GhostWire Secure Messaging</h1>
        
        <div class="status">
            <h3>✅ Server Status: Online</h3>
            <p>GhostWire API is running successfully on port 3000</p>
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
            <div class="method">WS</div>
            <div class="url">/ws</div>
            <div class="description">WebSocket connection for real-time messaging</div>
        </div>

        <div class="terminal">
            <h4>🧪 Test Commands:</h4>
            <pre>curl http://127.0.0.1:3000/api/status
curl http://127.0.0.1:3000/api/peers</pre>
        </div>

        <div class="status">
            <h4>🔧 Next Steps:</h4>
            <p>• Start the React frontend: <code>cd webui && npm run dev</code></p>
            <p>• Use CLI commands: <code>cargo run -- whisper &lt;peer&gt; &lt;message&gt;</code></p>
            <p>• Connect via WebSocket for real-time messaging</p>
        </div>
    </div>
</body>
</html>
    "#)
}

pub fn app(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_headers(Any);

    Router::new()
        .route("/", get(root))
        .route("/api/status", get(status))
        .route("/api/send_message", post(send_message))
        .route("/api/peers", get(get_peers))
        .route("/api/settings", get(get_settings))
        .route("/api/settings", put(update_settings))
        .route("/ws", get(ws_handler))
        .layer(cors)
        .with_state(state)
} 