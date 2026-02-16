use crate::api_v2_types::*;
use crate::controllers::{
    AuthController, IdentityController, MessagingController, PeerController, ProfilesController,
    SystemController,
};
use crate::core::Core;
use axum::body::Body;
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::http::Method;
use axum::http::{header, HeaderName, HeaderValue, Request, StatusCode, Uri};
use axum::{
    extract::ConnectInfo,
    extract::State,
    middleware,
    response::IntoResponse,
    response::Response,
    routing::{delete, get, post, put},
    Json, Router,
};
// use chrono;
use jsonwebtoken;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_json;
use std::net::SocketAddr;
use std::sync::Arc;
// Rate limiter removed due to frontend init burst interference
// use tower_governor::governor::GovernorConfigBuilder;
// use tower_governor::key_extractor::SmartIpKeyExtractor;
// use tower_governor::GovernorLayer;
use tower_http::cors::CorsLayer;

use lazy_static::lazy_static;
use local_ip_address;
use rand::Rng;
use std::fs;
// use std::sync::Mutex;
// use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

#[derive(RustEmbed)]
#[folder = "../webui/dist/"]
struct Assets;

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.is_empty() {
        path = "index.html".to_string();
    }

    match Assets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content.data))
                .unwrap()
        }
        std::option::Option::None => {
            if let Some(index) = Assets::get("index.html") {
                Response::builder()
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(Body::from(index.data))
                    .unwrap()
            } else {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("404 Not Found"))
                    .unwrap()
            }
        }
    }
}

lazy_static! {
    pub static ref JWT_SECRET: String = {
        let secret: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        secret
    };
    pub static ref AUTH_TOKEN: String = {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("ghostwire");

        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }

        let token_file = config_dir.join("token");

        if let Ok(token) = fs::read_to_string(&token_file) {
            let token = token.trim().to_string();
            if !token.is_empty() {
                return token;
            }
        }

        let token: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let _ = fs::write(&token_file, &token);
        println!("\n\n🔒 GHOSTWIRE SECURITY 🔒");
        println!("Generated new persistent token in: {:?}", token_file);
        println!("X-Ghost-Token: {}\n", token);
        token
    };
}

pub fn get_client_ip(req: &Request<Body>, addr: SocketAddr) -> std::net::IpAddr {
    let headers = req.headers();
    let header_candidates = ["X-Forwarded-For", "X-Real-IP", "Forwarded"];

    for header_name in header_candidates {
        if let Some(val) = headers.get(header_name).and_then(|h| h.to_str().ok()) {
            let first_ip = val.split(',').next().unwrap_or("").trim();
            if let Ok(ip) = first_ip.parse::<std::net::IpAddr>() {
                return ip;
            }
        }
    }
    addr.ip()
}

pub fn get_local_ip() -> Option<String> {
    local_ip_address::local_ip().ok().map(|ip| ip.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn auth_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<axum::body::Body>,
    next: middleware::Next,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let client_ip = get_client_ip(&req, addr);

    if !path.starts_with("/api") {
        return Ok(next.run(req).await);
    }

    tracing::debug!("Checking auth path: {}", path);

    if path == "/api/dev/token" || path == "/api/auth/login" || path == "/api/status" {
        return Ok(next.run(req).await);
    }

    // Allow profile creation/listing without auth (bootstrapping)
    if path.starts_with("/api/profile") || path.starts_with("/api/identity") {
        return Ok(next.run(req).await);
    }

    let token_header = req
        .headers()
        .get("X-Ghost-Token")
        .and_then(|val| val.to_str().ok());

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok());

    let authenticated = if let Some(token) = token_header {
        token == *AUTH_TOKEN
    } else if let Some(auth_val) = auth_header {
        if auth_val.starts_with("Bearer ") {
            let jwt = &auth_val[7..];
            jsonwebtoken::decode::<Claims>(
                jwt,
                &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET.as_bytes()),
                &jsonwebtoken::Validation::default(),
            )
            .is_ok()
        } else {
            false
        }
    } else {
        false
    };

    if authenticated {
        Ok(next.run(req).await)
    } else {
        tracing::warn!(target: "web", "⛔ Unauthorized API access attempt to {} from {}", path, client_ip);
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Clone)]
pub struct StartupConfig {
    pub bootstrap_nodes: Vec<libp2p::Multiaddr>,
    pub sim_addr: Option<String>,
    pub mesh_host: String,
}

#[derive(Clone)]
pub struct AppState {
    pub core: Arc<RwLock<Arc<Core>>>,
    pub config: StartupConfig,
    pub full_config: crate::config::Config,
}

pub async fn start_web_server(
    core: Option<Arc<Core>>,
    host: String,
    port: u16,
    bootstrap_nodes: Vec<libp2p::Multiaddr>,
    sim_addr: Option<String>,
    mesh_host: String,
    config: crate::config::Config,
) -> anyhow::Result<()> {
    let initial_core = if let Some(c) = core {
        c
    } else {
        tracing::info!(target: "web", "👤 No profile selected. Initializing Guest Core...");
        let guest_core = Core::new(
            config.clone(),
            bootstrap_nodes.clone(),
            sim_addr.clone(),
            None,
        )
        .await?;
        Arc::new(guest_core)
    };

    let app_state = Arc::new(AppState {
        core: Arc::new(RwLock::new(initial_core)),
        config: StartupConfig {
            bootstrap_nodes,
            sim_addr,
            mesh_host,
        },
        full_config: config,
    });

    let _ = &*AUTH_TOKEN;

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            HeaderName::from_static("x-ghost-token"),
        ]);

    /*
    let default_governor_conf = GovernorConfigBuilder::default()
        .per_second(200) // Increased from 60 to handle frontend init burst
        .burst_size(100) // Increased from 60 to handle parallel requests
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();
    let default_rate_limiter = GovernorLayer {
        config: std::sync::Arc::new(default_governor_conf),
    };
    */

    let auth_routes: Router<Arc<AppState>> = Router::new()
        .route("/login", post(AuthController::login))
        .route("/verify", post(AuthController::verify_token));

    let profile_routes: Router<Arc<AppState>> = Router::new()
        .route("/list", get(ProfilesController::list))
        .route("/create", post(ProfilesController::create))
        .route("/select", post(ProfilesController::select))
        .route("/delete/:name", delete(ProfilesController::delete));

    let system_routes: Router<Arc<AppState>> = Router::new()
        .route("/stats", get(SystemController::status))
        .route("/audit", get(SystemController::security_audit))
        .route("/wipe", post(ProfilesController::wipe_all));

    let identity_routes: Router<Arc<AppState>> = Router::new()
        .route("/", get(IdentityController::get_profile))
        .route("/status", get(IdentityController::get_profile))
        .route("/setup", post(IdentityController::setup))
        .route("/public-key", get(IdentityController::get_public_key));

    let api_routes: Router<Arc<AppState>> = Router::new()
        .route(
            "/",
            get(|| async { Json(ApiV2Response::success("GhostWire API v2")) }),
        )
        .nest("/auth", auth_routes)
        .nest("/profile", profile_routes)
        .nest("/system", system_routes)
        .nest("/identity", identity_routes)
        // .route("/identity", get(IdentityController::get_profile)) // Handled by nested route
        // .route("/identity/status", get(IdentityController::get_profile)) // Handled by nested route
        // .route("/public-key", get(IdentityController::get_public_key)) // Handled by nested route
        .route("/messages", get(MessagingController::get_messages))
        .route("/broadcast", post(MessagingController::broadcast))
        .route("/send", post(MessagingController::send_message))
        .route("/send_message", post(MessagingController::send_message))
        .route("/peers", get(PeerController::list))
        .route("/connect_peer", post(PeerController::connect))
        .route("/disconnect_peer", post(PeerController::disconnect))
        .route("/ping_peer", post(PeerController::ping))
        .route("/settings", get(SystemController::get_settings))
        .route("/settings", put(SystemController::update_settings))
        .route("/v2/profile", get(IdentityController::get_profile))
        .route(
            "/v2/profile/username",
            put(IdentityController::update_username),
        )
        .route(
            "/v2/profile/metadata",
            put(IdentityController::update_metadata),
        )
        .route(
            "/v2/profile/public-key",
            get(IdentityController::get_public_key),
        )
        .route(
            "/v2/security/dashboard",
            get(SystemController::security_dashboard),
        )
        .route("/v2/peers", get(PeerController::list))
        .route("/v2/messages", get(MessagingController::get_messages))
        .route("/report_error", post(report_error))
        .route("/help/tutorial", get(get_tutorial));
    // .layer(default_rate_limiter);

    let app: Router = Router::new()
        .nest("/api", api_routes)
        .route("/ws", get(ws_handler))
        .layer(middleware::from_fn(auth_middleware))
        .layer(cors.clone())
        .fallback(static_handler)
        .with_state(app_state);

    let address = format!("{}:{}", host, port);
    tracing::info!(target: "web", "Starting GhostWire web server on http://{}", address);

    let listener = tokio::net::TcpListener::bind(&address).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, _state: Arc<AppState>) {
    use axum::extract::ws::Message;
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            let _ = socket.send(Message::Text(format!("Echo: {}", text))).await;
        }
    }
}

pub async fn report_error(Json(req): Json<ErrorReportRequest>) -> impl IntoResponse {
    eprintln!("[REMOTE ERROR REPORT] {}", req.error);
    Json(ApiV2Response::success(
        serde_json::json!({ "status": "reported" }),
    ))
}

pub async fn get_tutorial() -> impl IntoResponse {
    Json(ApiV2Response::success(
        crate::core::tutorial::get_tutorial_steps(),
    ))
}

#[derive(Deserialize)]
pub struct ErrorReportRequest {
    pub error: String,
}
