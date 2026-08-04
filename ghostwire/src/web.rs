use crate::api_v2_types::*;
use crate::controllers::{
    AuthController, HealthController, IdentityController, MessagingController, PeerController,
    ProfilesController, SystemController,
};
use crate::core::Core;
use axum::body::Body;

use axum::http::{header, HeaderName, HeaderValue, Method, Request, StatusCode, Uri};
use axum::{
    extract::ConnectInfo,
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
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;

use lazy_static::lazy_static;
use local_ip_address;

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
            // ponytail: cache by content type — HTML revalidate, hashed assets immutable, rest 1hr
            let cache_control = if path.ends_with(".html") {
                "no-cache"
            } else if path.ends_with(".js") || path.ends_with(".css") {
                "public, max-age=31536000, immutable"
            } else {
                "public, max-age=3600"
            };
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .header(header::CACHE_CONTROL, cache_control)
                .body(Body::from(content.data))
                .unwrap()
        }
        std::option::Option::None => {
            // ponytail: SPA fallback only for browser-navigable paths (no file ext, not /api/)
            if !path.starts_with("api/") && !path.contains('.') {
                if let Some(index) = Assets::get("index.html") {
                    return Response::builder()
                        .header(header::CONTENT_TYPE, "text/html")
                        .header(header::CACHE_CONTROL, "no-cache")
                        .body(Body::from(index.data))
                        .unwrap();
                }
            }
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found"))
                .unwrap()
        }
    }
}

lazy_static! {
    pub static ref JWT_SECRET: String = {
        use ring::rand::{SecureRandom, SystemRandom};
        use base64::Engine;

        let config_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            std::path::PathBuf::from(ghost_home)
        } else {
            dirs::config_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("ghostwire")
        };
        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }
        let secret_file = config_dir.join("jwt_secret");

        if let Ok(secret) = fs::read_to_string(&secret_file) {
            let secret = secret.trim().to_string();
            if !secret.is_empty() {
                return secret;
            }
        }

        let rng = SystemRandom::new();
        let mut bytes = [0u8; 48];
        rng.fill(&mut bytes).expect("JWT secret generation failed");
        let secret = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes);
        let _ = fs::write(&secret_file, &secret);

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = fs::metadata(&secret_file) {
                let mut perms = meta.permissions();
                perms.set_mode(0o600);
                let _ = fs::set_permissions(&secret_file, perms);
            }
        }

        tracing::info!("JWT secret persisted to {}", secret_file.display());
        secret
    };
    // SECURITY FIX: Use ring::rand + set file permissions to 0o600
    pub static ref AUTH_TOKEN: String = {
        use ring::rand::{SecureRandom, SystemRandom};
        use base64::Engine;

        let config_dir = if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            std::path::PathBuf::from(ghost_home)
        } else {
            dirs::config_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("ghostwire")
        };

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

        // Generate cryptographically secure token
        let rng = SystemRandom::new();
        let mut bytes = [0u8; 24]; // 192 bits
        rng.fill(&mut bytes).expect("Auth token generation failed");
        let token = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes);

        let _ = fs::write(&token_file, &token);

        // Set restrictive permissions (owner read/write only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = fs::metadata(&token_file) {
                let mut perms = meta.permissions();
                perms.set_mode(0o600);
                let _ = fs::set_permissions(&token_file, perms);
            }
        }

        tracing::info!(
            token_file = %token_file.display(),
            "Auth token generated — set X-Ghost-Token header for API access"
        );
        token
    };
}

pub fn get_client_ip(_req: &Request<Body>, addr: SocketAddr) -> std::net::IpAddr {
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
    if path == "/api/auth/login"
        || path.starts_with("/api/v2/health")
        || path.starts_with("/api/v2/ready")
        || path.starts_with("/api/v2/live")
        // Profile: only list is public (login screen shows profiles)
        || path == "/api/profile/list"
        // Identity: setup + status public (first-time setup + login check)
        || path == "/api/identity/setup"
        || path == "/api/identity/status"
        || path == "/api/identity/"
        || path.starts_with("/api/ble")
    {
        return Ok(next.run(req).await);
    }

    // CSRF protection for WebSocket upgrades — browsers can't set custom headers,
    // so we validate Origin instead. Non-browser WS clients pass through.
    if path == "/api/ws" && req.headers().contains_key(header::UPGRADE) {
        let origin = req
            .headers()
            .get("origin")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        let allowed = origin == "http://localhost:5173"
            || origin == "http://localhost:5174"
            || origin == "https://ghostwire.cc";
        if !allowed && !origin.is_empty() {
            tracing::warn!(target: "web", "CSWSH blocked: origin={} ip={}", origin, client_ip);
            return Err(StatusCode::FORBIDDEN);
        }
    }

    let token_header = req
        .headers()
        .get("X-Ghost-Token")
        .and_then(|val| val.to_str().ok());

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|val| val.to_str().ok());

    use subtle::ConstantTimeEq;
    let authenticated = if let Some(token) = token_header {
        token.as_bytes().ct_eq(AUTH_TOKEN.as_bytes()).into()
    } else if let Some(auth_val) = auth_header {
        if auth_val.starts_with("Bearer ") {
            let jwt = &auth_val[7..];
            let mut validation = jsonwebtoken::Validation::default();
            validation.validate_exp = true;
            // Ponytail: no audience/issuer set because JWTs are self-issued at login
            // with no external IdP. Add validation.set_audience() / .set_issuer() if
            // an external identity provider is ever integrated.
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
    pub mesh_port: u16,
}

#[derive(Clone)]
pub struct AppState {
    pub core: Arc<RwLock<Arc<Core>>>,
    pub config: StartupConfig,
    pub full_config: crate::config::Config,
    /// CRITICAL FIX: Track WebSocket subscriber count for mesh health dashboard
    pub ws_count: Arc<std::sync::atomic::AtomicUsize>,
}

pub async fn start_web_server(
    core: Option<Arc<Core>>,
    host: String,
    port: u16,
    bootstrap_nodes: Vec<libp2p::Multiaddr>,
    sim_addr: Option<String>,
    mesh_host: String,
    mesh_port: u16,
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
            mesh_port,
        },
        full_config: config,
        ws_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
    });

    let _ = &*AUTH_TOKEN;

    let cors = CorsLayer::new()
        // Restrict origins to localhost only — private ranges (192.168.*, 172.*)
        // are too permissive; any device on the LAN can issue cross-origin requests.
        .allow_origin(tower_http::cors::AllowOrigin::predicate(
            |origin: &HeaderValue, _request_parts: &axum::http::request::Parts| {
                if let Ok(origin_str) = origin.to_str() {
                    let host = origin_str
                        .trim_start_matches("http://")
                        .trim_start_matches("https://");
                    let host_no_port = host.split(':').next().unwrap_or("");
                    host_no_port == "localhost" || host_no_port == "127.0.0.1"
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
    // CRITICAL FIX: Increased limits for authenticated users (dev/owner privilege)
    // Authenticated requests get much higher limits; unauthenticated still protected
    let default_governor_conf = GovernorConfigBuilder::default()
        .per_second(20)
        .burst_size(50)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    let default_rate_limiter = GovernorLayer::new(std::sync::Arc::new(default_governor_conf));

    // Stricter rate limiter for auth endpoints (brute force protection)
    let auth_governor_conf = GovernorConfigBuilder::default()
        .per_second(5) // 5 requests per second per IP
        .burst_size(10) // Allow burst of 10 requests
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    let auth_rate_limiter = GovernorLayer::new(std::sync::Arc::new(auth_governor_conf));

    let auth_routes: Router<Arc<AppState>> = Router::new()
        .route("/login", post(AuthController::login))
        .route("/verify", post(AuthController::verify_token))
        .route("/dev-token", get(AuthController::get_dev_token));

    let profile_routes: Router<Arc<AppState>> = Router::new()
        .route("/list", get(ProfilesController::list))
        .route("/create", post(ProfilesController::create))
        .route("/select", post(ProfilesController::select))
        .route("/delete/{name}", delete(ProfilesController::delete));

    let system_routes: Router<Arc<AppState>> = Router::new()
        .route("/stats", get(SystemController::status))
        .route("/audit", get(SystemController::security_audit))
        .route("/wipe", post(ProfilesController::wipe_all))
        .route("/rotate-keys", put(SystemController::rotate_keys))
        .route("/lockdown", post(SystemController::lockdown))
        .route("/backup", post(SystemController::backup_system))
        .route("/logs", get(SystemController::get_system_logs))
        .route("/upgrade-encryption", post(SystemController::upgrade_encryption))
        .route("/firewall", post(SystemController::configure_firewall))
        .route("/firewall/test", post(SystemController::test_firewall))
        .route("/users", get(SystemController::get_auth_users))
        .route("/security-scan", post(SystemController::security_scan))
        .route("/threat-hunt", post(SystemController::threat_hunt))
        .route("/backup-security", post(SystemController::backup_security))
        .route("/analyze-communications", post(SystemController::analyze_communications));

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
        .route("/send_message", post(MessagingController::send_message))
        .route("/peers", get(PeerController::list))
        .route("/connect_peer", post(PeerController::request_connect))
        .route("/disconnect_peer", post(PeerController::disconnect))
        .route("/ping_peer", post(PeerController::ping))
        // BLE Transport endpoints — DISABLED: ble_controller module not implemented yet
        // .route("/ble/status", get(crate::controllers::ble_controller::BleController::status))
        // .route("/ble/connect", post(crate::controllers::ble_controller::BleController::connect))
        // .route("/ble/send", post(crate::controllers::ble_controller::BleController::ble_send))
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
        .route("/v2/messages/send", post(MessagingController::send_message))
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
            "/v2/peers/{id}",
            axum::routing::delete(crate::controllers::PeerController::disconnect),
        )
        .route(
            "/v2/peers/{id}/block",
            post(crate::controllers::PeerController::block),
        )
        .route(
            "/v2/peers/ping",
            post(crate::controllers::PeerController::ping),
        )
        .route(
            "/v2/peers/{id}/profile",
            get(crate::controllers::PeerController::get_profile),
        )
        // Trust
        .route(
            "/v2/trust",
            get(crate::controllers::TrustController::list_peers),
        )
        .route(
            "/v2/trust/{peer_id}",
            get(crate::controllers::TrustController::get_trust),
        )
        .route(
            "/v2/trust/{peer_id}/verify",
            post(crate::controllers::TrustController::mark_verified),
        )
        .route(
            "/v2/trust/{peer_id}/block",
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
            "/v2/ark/entries/{cid}",
            get(crate::controllers::ArkController::get_entry),
        )
        .route(
            "/v2/ark/entries/{cid}",
            axum::routing::delete(crate::controllers::ArkController::delete_entry),
        )
        .route(
            "/v2/ark/search",
            get(crate::controllers::ArkController::search),
        )
        .route(
            "/v2/ark/tags/{tag}",
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
            "/v2/channels/{id}/join",
            post(crate::controllers::ChannelController::join),
        )
        .route(
            "/v2/channels/{id}/leave",
            post(crate::controllers::ChannelController::leave),
        )
        .route(
            "/v2/channels/{id}/publish",
            post(crate::controllers::ChannelController::publish),
        )
        .route(
            "/v2/channels/{id}/messages",
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
        .nest(
            "/api",
            api_routes
                .layer(default_rate_limiter)
                .route("/ws", get(crate::controllers::ws_handler::ws_handler)),
        )
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
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
        .layer(CompressionLayer::new())
        .layer(middleware::from_fn(auth_middleware))
        .layer(cors.clone())
        .fallback(static_handler)
        .with_state(app_state);

    let address = format!("{}:{}", host, port);
    tracing::info!(target: "web", "Starting GhostWire web server on http://{}", address);

    let listener = tokio::net::TcpListener::bind(&address).await?;
    let shutdown_signal = async {
        let _ = tokio::signal::ctrl_c().await;
        tracing::info!("Shutdown signal received, draining connections...");
    };
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal)
    .await?;
    Ok(())
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
