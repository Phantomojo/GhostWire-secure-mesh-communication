use crate::api_v2_types::*;
use crate::controllers::{
    AuthController, HealthController, IdentityController, MessagingController, PeerController,
    ProfilesController, SystemController,
};
use crate::core::Core;
use axum::body::Body;
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::http::{header, HeaderName, HeaderValue, Method, Request, StatusCode, Uri};
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
// Rate limiter for API endpoints
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;
use tower_governor::GovernorLayer;
use tower_http::cors::{Any, CorsLayer};

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

    // Public endpoints (no auth required)
    if path == "/api/dev/token"
        || path == "/api/auth/login"
        || path == "/api/status"
        || path.starts_with("/api/profile")
        || path.starts_with("/api/identity")
        || path.starts_with("/api/v2/profile")
    // Allow v2 profile endpoints
    {
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
        // Restrict origins to localhost, 192.168.x.x, and Docker bridge
        .allow_origin(tower_http::cors::AllowOrigin::predicate(
            |origin: &HeaderValue, _request_parts: &axum::http::request::Parts| {
                if let Ok(origin_str) = origin.to_str() {
                    let host = origin_str
                        .trim_start_matches("http://")
                        .trim_start_matches("https://");
                    let host_no_port = host.split(':').next().unwrap_or("");
                    host_no_port == "localhost"
                        || host_no_port == "127.0.0.1"
                        || host_no_port.starts_with("192.168.")
                        || host_no_port.starts_with("172.")
                } else {
                    false
                }
            },
        ))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        // Note: allow_credentials disabled with wildcard origin (CORS spec requirement)
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::ORIGIN,
            HeaderName::from_static("x-ghost-token"),
        ]);

    // Configure Governor rate limiter for API protection
    let default_governor_conf = GovernorConfigBuilder::default()
        .per_second(60) // 60 requests per second per IP
        .burst_size(30) // Allow burst of 30 requests
        .key_extractor(SmartIpKeyExtractor) // Rate limit by IP address
        .finish()
        .unwrap();

    let default_rate_limiter = GovernorLayer {
        config: std::sync::Arc::new(default_governor_conf),
    };

    // Stricter rate limiter for auth endpoints (brute force protection)
    let auth_governor_conf = GovernorConfigBuilder::default()
        .per_second(5) // 5 requests per second per IP
        .burst_size(10) // Allow burst of 10 requests
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    let auth_rate_limiter = GovernorLayer {
        config: std::sync::Arc::new(auth_governor_conf),
    };

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
        // Auth routes with strict rate limiting (brute force protection)
        .nest("/auth", auth_routes.layer(auth_rate_limiter))
        .nest("/profile", profile_routes)
        .nest("/system", system_routes)
        .nest("/identity", identity_routes)
        // V2 endpoints
        .route(
            "/v2/system/mesh-health",
            get(crate::v2_routes::get_mesh_health),
        )
        // Health check endpoints
        .route("/v2/health", get(HealthController::health))
        .route("/v2/ready", get(HealthController::ready))
        .route("/v2/live", get(HealthController::live))
        // Note: /v2/security/dashboard already defined below with SystemController
        // .route("/identity", get(IdentityController::get_profile)) // Handled by nested route
        // .route("/identity/status", get(IdentityController::get_profile)) // Handled by nested route
        // .route("/public-key", get(IdentityController::get_public_key)) // Handled by nested route
        .route("/messages", get(MessagingController::get_messages))
        .route("/broadcast", post(MessagingController::broadcast))
        .route("/send", post(MessagingController::send_message))
        .route("/send_message", post(MessagingController::send_message))
        .route("/peers", get(PeerController::list))
        .route("/connect_peer", post(PeerController::request_connect))
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
        .route("/v2/profile/status", get(IdentityController::get_profile))
        .route("/v2/profile/lock", post(IdentityController::lock_profile))
        .route(
            "/v2/profile/unlock",
            post(IdentityController::unlock_profile),
        )
        .route("/v2/profile/save", put(IdentityController::update_metadata))
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
        .route("/help/tutorial", get(get_tutorial))
        // ── Wave 2-6 routes ──────────────────────────────────────────────
        // Peers (real controller)
        .route(
            "/v2/peers/nearby",
            get(crate::controllers::PeerController::nearby),
        )
        .route(
            "/v2/peers/connect",
            post(crate::controllers::PeerController::request_connect),
        )
        .route(
            "/v2/peers/requests",
            get(crate::controllers::PeerController::list_requests),
        )
        .route(
            "/v2/peers/requests/respond",
            post(crate::controllers::PeerController::respond_to_request),
        )
        .route(
            "/v2/peers/:id",
            axum::routing::delete(crate::controllers::PeerController::disconnect),
        )
        .route(
            "/v2/peers/:id/block",
            post(crate::controllers::PeerController::block),
        )
        .route(
            "/v2/peers/ping",
            post(crate::controllers::PeerController::ping),
        )
        // Trust
        .route(
            "/v2/trust",
            get(crate::controllers::TrustController::list_peers),
        )
        .route(
            "/v2/trust/:peer_id",
            get(crate::controllers::TrustController::get_trust),
        )
        .route(
            "/v2/trust/:peer_id/verify",
            post(crate::controllers::TrustController::mark_verified),
        )
        .route(
            "/v2/trust/:peer_id/block",
            post(crate::controllers::TrustController::block_peer),
        )
        // Knowledge Ark
        .route(
            "/v2/ark/collections",
            get(crate::controllers::ArkController::list_collections),
        )
        .route(
            "/v2/ark/collections",
            post(crate::controllers::ArkController::create_collection),
        )
        .route(
            "/v2/ark/entries",
            post(crate::controllers::ArkController::create_entry),
        )
        .route(
            "/v2/ark/entries/:cid",
            get(crate::controllers::ArkController::get_entry),
        )
        .route(
            "/v2/ark/entries/:cid",
            axum::routing::delete(crate::controllers::ArkController::delete_entry),
        )
        .route(
            "/v2/ark/search",
            get(crate::controllers::ArkController::search),
        )
        .route(
            "/v2/ark/tags/:tag",
            get(crate::controllers::ArkController::search_by_tag),
        )
        .route(
            "/v2/ark/stats",
            get(crate::controllers::ArkController::stats),
        )
        // Ghost Channels
        .route(
            "/v2/channels",
            get(crate::controllers::ChannelController::list),
        )
        .route(
            "/v2/channels",
            post(crate::controllers::ChannelController::create),
        )
        .route(
            "/v2/channels/:id/join",
            post(crate::controllers::ChannelController::join),
        )
        .route(
            "/v2/channels/:id/leave",
            post(crate::controllers::ChannelController::leave),
        )
        .route(
            "/v2/channels/:id/publish",
            post(crate::controllers::ChannelController::publish),
        )
        .route(
            "/v2/channels/:id/messages",
            get(crate::controllers::ChannelController::get_messages),
        )
        // Recovery (Shamir)
        .route(
            "/v2/recovery/setup",
            post(crate::controllers::RecoveryController::setup_shares),
        )
        .route(
            "/v2/recovery/my-shares",
            get(crate::controllers::RecoveryController::list_my_shares),
        )
        .route(
            "/v2/recovery/reconstruct",
            post(crate::controllers::RecoveryController::reconstruct_key),
        )
        // Network / mesh health
        .route(
            "/v2/network/status",
            get(crate::controllers::NetworkController::status),
        )
        .route(
            "/v2/network/qr-contact",
            get(crate::controllers::NetworkController::generate_contact_qr),
        )
        .route(
            "/v2/network/scan-qr",
            post(crate::controllers::NetworkController::scan_contact_qr),
        )
        // Note: /v2/system/mesh-health already defined above with v2_routes
        // Pairing
        .route(
            "/v2/pair/generate",
            post(crate::controllers::PairingController::generate_code),
        )
        .route(
            "/v2/pair/contact-card",
            get(crate::controllers::PairingController::get_contact_card),
        )
        .route(
            "/v2/pair/rendezvous",
            get(crate::controllers::PairingController::get_rendezvous_address),
        );

    let app: Router = Router::new()
        // All API routes with default rate limiting (auth already has stricter limits)
        .nest("/api", api_routes.layer(default_rate_limiter))
        .route("/ws", get(ws_handler))
        // Security headers middleware (HSTS, X-Frame-Options, CSP, etc.)
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-xss-protection"),
            HeaderValue::from_static("1; mode=block"),
        ))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
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

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    use axum::extract::ws::{Message, WebSocket};
    use futures_util::{SinkExt, StreamExt};
    use tokio::sync::broadcast;

    // Split socket into send/recv halves
    let (mut sender, mut receiver) = socket.split();

    // Subscribe to core event bus
    let mut event_rx = state.core.read().await.event_bus.subscribe();

    // Task 1: Forward CoreEvents to WebSocket
    let send_task = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            // Serialize CoreEvent to JSON
            let json = serde_json::to_string(&event).unwrap_or_default();
            if sender.send(Message::Text(json)).await.is_err() {
                break; // Client disconnected
            }
        }
    });

    // Task 2: Handle incoming messages (echo for now, can be extended)
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // Could handle incoming commands here
                tracing::debug!("WebSocket received: {}", text);
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
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
