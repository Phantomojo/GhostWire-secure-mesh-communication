use crate::api_v2_types::*;
use crate::controllers::{
    AuthController, HealthController, IdentityController, MessagingController, PeerController,
    ProfilesController, SystemController,
};
use crate::core::Core;
use axum::body::Body;

use axum::http::{header, HeaderName, HeaderValue, Method, Request, StatusCode, Uri};
use axum::{
    extract::{ConnectInfo, State},
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
use std::sync::atomic::{AtomicU64, Ordering};
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
use std::sync::LazyLock;
// use std::sync::Mutex;
use std::time::Instant;
use tokio::sync::RwLock;

#[derive(RustEmbed)]
#[folder = "../webui/dist/"]
struct Assets;

static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

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
            if path.is_empty() {
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

/// Browser UI origins are accepted only from loopback, the private LAN
/// ranges already supported by GhostWire, or the Tailscale CGNAT range.
/// Public Internet origins remain rejected by default.
fn allowed_network_host(host: &str) -> bool {
    if host == "localhost" || host == "127.0.0.1" {
        return true;
    }
    // IPv4 ranges are matched numerically, not by string prefix. A
    // `host.starts_with("172.")` test wrongly accepts 172.0.0.0/12's public
    // neighbours (172.0-172.15 and 172.32-172.255 are public address space),
    // which would let a public origin pass the CSWSH check.
    match host.parse::<std::net::IpAddr>() {
        Ok(std::net::IpAddr::V4(ip)) => {
            let o = ip.octets();
            // 192.168.0.0/16
            (o[0] == 192 && o[1] == 168)
            // 172.16.0.0/12 - the second octet must be 16..=31
            || (o[0] == 172 && (16..=31).contains(&o[1]))
            // 100.64.0.0/10, CGNAT range used by Tailscale
            || (o[0] == 100 && (64..=127).contains(&o[1]))
        }
        Ok(std::net::IpAddr::V6(ip6)) => {
            // Unique-local (fc00::/7) and link-local (fe80::/10) addresses.
            let s = ip6.segments();
            (s[0] & 0xfe00) == 0xfc00 || (s[0] & 0xffc0) == 0xfe80
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod network_host_tests {
    use super::allowed_network_host;

    #[test]
    fn accepts_loopback_and_private_ranges() {
        for host in [
            "localhost",
            "127.0.0.1",
            "192.168.1.10",
            "192.168.255.254",
            "172.16.0.1",
            "172.20.10.5",
            "172.31.255.254",
            "100.64.0.1",
            "100.127.255.254",
        ] {
            assert!(allowed_network_host(host), "{host} should be allowed");
        }
    }

    #[test]
    fn rejects_public_addresses_including_172_neighbours() {
        // 172.0-172.15 and 172.32-172.255 are PUBLIC address space. A
        // string-prefix check on "172." wrongly allowed all of them, letting a
        // public origin pass the cross-site WebSocket hijack check.
        for host in [
            "172.0.0.1",
            "172.15.255.254",
            "172.32.0.1",
            "172.255.255.254",
            "8.8.8.8",
            "1.1.1.1",
            "100.63.255.255",
            "100.128.0.1",
            "193.169.1.1",
            "104.16.0.1",
            "evil.example.com",
            "",
        ] {
            assert!(!allowed_network_host(host), "{host} must be rejected");
        }
    }

    #[test]
    fn accepts_ipv6_local_ranges() {
        for host in ["fd00::1", "fdff::1", "fe80::1", "febf::1"] {
            assert!(allowed_network_host(host), "{host} should be allowed");
        }
        for host in ["2001:4860:4860::8888", "::1", "2606:4700::1111"] {
            assert!(!allowed_network_host(host), "{host} must be rejected");
        }
    }
}

static SESSION_EPOCH: AtomicU64 = AtomicU64::new(1);

pub fn current_session_epoch() -> u64 {
    SESSION_EPOCH.load(Ordering::Relaxed)
}

pub fn revoke_sessions() {
    SESSION_EPOCH.fetch_add(1, Ordering::Relaxed);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    #[serde(default)]
    pub session_epoch: u64,
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

    // Static pages remain public, but Prometheus metrics are an operator
    // surface and must pass the same credential check as API commands.
    if !path.starts_with("/api") && path != "/metrics" {
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
        // Locked-screen lifecycle endpoints are public only because they
        // perform their own first-run/locked-state checks.
        || path == "/api/v2/profile/create-first"
        || path == "/api/v2/profile/select-locked"
        // Identity: setup + status public (first-time setup + login check)
        || path == "/api/identity/setup"
        || path == "/api/identity/status"
        || path == "/api/v2/identity/status"
        // Local passphrase unlock is the authentication bootstrap. The
        // handler performs failed-attempt tracking before issuing a session.
        || path == "/api/v2/profile/unlock"
        || path == "/api/v2/profile/recover-legacy"
    // BLE and legacy identity-prefix endpoints are not registered public
    // routes; command surfaces must remain behind authentication.
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
        let host = origin
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .split(':')
            .next()
            .unwrap_or("");
        let allowed = origin == "https://ghostwire.cc" || allowed_network_host(host);
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

    // Browser WebSocket constructors cannot set Authorization headers. Accept
    // the short-lived browser bootstrap token on /api/ws only; all normal HTTP
    // requests remain header-authenticated. Raw token and JWT query values are
    // both supported for compatibility with the current Web UI.
    let ws_query_token = (path == "/api/ws")
        .then(|| {
            req.uri()
                .query()
                .unwrap_or("")
                .split('&')
                .find_map(|pair| pair.strip_prefix("token="))
        })
        .flatten();

    use subtle::ConstantTimeEq;
    let authenticated = if let Some(token) = token_header {
        token.as_bytes().ct_eq(AUTH_TOKEN.as_bytes()).into()
    } else if let Some(token) = ws_query_token {
        if token.as_bytes().ct_eq(AUTH_TOKEN.as_bytes()).into() {
            true
        } else {
            jsonwebtoken::decode::<Claims>(
                token,
                &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET.as_bytes()),
                &jsonwebtoken::Validation::default(),
            )
            .map(|token_data| token_data.claims.session_epoch == current_session_epoch())
            .unwrap_or(false)
        }
    } else if let Some(auth_val) = auth_header {
        if auth_val.starts_with("Bearer ") {
            let jwt = &auth_val[7..];
            jsonwebtoken::decode::<Claims>(
                jwt,
                &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET.as_bytes()),
                &jsonwebtoken::Validation::default(),
            )
            .map(|token_data| token_data.claims.session_epoch == current_session_epoch())
            .unwrap_or(false)
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

    let core_ref = Arc::new(RwLock::new(initial_core));
    let shutdown_core = core_ref.clone();
    let app_state = Arc::new(AppState {
        core: core_ref,
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
        // Restrict origins to localhost, 192.168.x.x, and Docker bridge
        .allow_origin(tower_http::cors::AllowOrigin::predicate(
            |origin: &HeaderValue, _request_parts: &axum::http::request::Parts| {
                if let Ok(origin_str) = origin.to_str() {
                    let host = origin_str
                        .trim_start_matches("http://")
                        .trim_start_matches("https://");
                    let host_no_port = host.split(':').next().unwrap_or("");
                    allowed_network_host(host_no_port) || host_no_port == "ghostwire.cc"
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
        .route(
            "/upgrade-encryption",
            post(SystemController::upgrade_encryption),
        )
        .route("/firewall", post(SystemController::configure_firewall))
        .route("/firewall/test", post(SystemController::test_firewall))
        .route("/users", get(SystemController::get_auth_users))
        .route("/security-scan", post(SystemController::security_scan))
        .route("/threat-hunt", post(SystemController::threat_hunt))
        .route("/backup-security", post(SystemController::backup_security))
        .route(
            "/analyze-communications",
            post(SystemController::analyze_communications),
        );

    let identity_routes: Router<Arc<AppState>> = Router::new()
        .route("/", get(IdentityController::get_profile))
        .route("/status", get(IdentityController::get_profile))
        .route("/setup", post(IdentityController::setup))
        .route(
            "/addresses",
            get(IdentityController::local_addresses)
        )
        .route("/pair", post(IdentityController::pair_peer))
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
        // Compatibility alias for clients using the API-v2 identity namespace.
        .route("/v2/identity/status", get(IdentityController::get_profile))
        .route("/v2/profile/lock", post(IdentityController::lock_profile))
        .route(
            "/v2/profile/unlock",
            post(IdentityController::unlock_profile),
        )
        .route(
            "/v2/profile/recover-legacy",
            post(IdentityController::recover_legacy_profile),
        )
        .route("/v2/profile/save", put(IdentityController::update_metadata))
        .route(
            "/v2/profile/create-first",
            post(ProfilesController::create_first),
        )
        .route(
            "/v2/profile/select-locked",
            post(ProfilesController::select_locked),
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
            "/v2/system/version",
            get(crate::controllers::NetworkController::version),
        )
        .route(
            "/v2/system/bootstrap",
            get(crate::controllers::NetworkController::bootstrap_health),
        )
        .route(
            "/v2/system/bootstrap/configure",
            post(crate::controllers::NetworkController::bootstrap_configure),
        )
        .route(
            "/v2/system/bootstrap/authorize",
            post(crate::controllers::NetworkController::bootstrap_authorize),
        )
        .route(
            "/v2/system/bootstrap/start",
            post(crate::controllers::NetworkController::bootstrap_start),
        )
        .route(
            "/v2/system/bootstrap/stop",
            post(crate::controllers::NetworkController::bootstrap_stop),
        )
        .route(
            "/v2/system/discovery",
            get(crate::controllers::NetworkController::discovery_health),
        )
        .route(
            "/v2/system/discovery/publish",
            post(crate::controllers::NetworkController::discovery_publish),
        )
        .route(
            "/v2/system/discovery/refresh",
            post(crate::controllers::NetworkController::discovery_refresh),
        )
        .route(
            "/v2/discovery/snapshot",
            get(crate::controllers::NetworkController::discovery_snapshot),
        )
        .route(
            "/v2/discovery/scan",
            post(crate::controllers::NetworkController::discovery_scan),
        )
        .route(
            "/v2/system/transports",
            get(crate::controllers::NetworkController::transport_health),
        )
        .route(
            "/v2/system/transports/{transport}/action",
            post(crate::controllers::NetworkController::transport_action),
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

    async fn node_info_page() -> impl IntoResponse {
        match Assets::get("node-info.html") {
            Some(content) => Response::builder()
                .header(header::CONTENT_TYPE, "text/html")
                .header(header::CACHE_CONTROL, "no-cache")
                .body(Body::from(content.data))
                .unwrap(),
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Node info page not found"))
                .unwrap(),
        }
    }

    let app: Router = Router::new()
        .route("/", get(static_handler))
        .route("/info", get(node_info_page))
        // Metrics expose peer/security state; keep them behind the same
        // authenticated middleware as API command surfaces.
        .route(
            "/metrics",
            get(prometheus_metrics).layer(middleware::from_fn(auth_middleware)),
        )
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
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to register SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
        }
        tracing::info!("Shutdown signal received, draining connections...");
    };
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal)
    .await?;

    // ponytail: this is the only shutdown path for web-server mode — without it,
    // SIGTERM/SIGINT leaves mesh state and sled persistence unflushed.
    tracing::info!("HTTP server stopped, flushing core state...");
    let core_guard = shutdown_core.read().await;
    if let Err(e) = core_guard.shutdown().await {
        tracing::error!("Core shutdown error: {}", e);
    }
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

/// Prometheus metrics endpoint — requires authentication.
///
/// This was previously documented as "no auth required (scraped by
/// monitoring)", which was wrong: `auth_middleware` is applied as a global
/// `.layer()` and therefore covers this route. Metrics expose peer and security
/// state, so keeping them authenticated is the correct default; a scraper should
/// present `X-Ghost-Token`.
async fn prometheus_metrics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let uptime = START_TIME.elapsed().as_secs();
    let core = state.core.read().await;
    let peer_count = core.get_peer_count();
    let sec = core.get_security_stats().await;
    let ws = state.ws_count.load(std::sync::atomic::Ordering::Relaxed);

    // ponytail: string-push fmt, not prometheus crate — one endpoint, no dep
    let mut m = String::with_capacity(1024);
    m.push_str("# HELP ghostwire_uptime_seconds Process uptime in seconds\n");
    m.push_str("# TYPE ghostwire_uptime_seconds gauge\n");
    m.push_str(&format!("ghostwire_uptime_seconds {}\n", uptime));

    m.push_str("# HELP ghostwire_peers_total Number of connected peers\n");
    m.push_str("# TYPE ghostwire_peers_total gauge\n");
    m.push_str(&format!("ghostwire_peers_total {}\n", peer_count));

    m.push_str("# HELP ghostwire_websocket_connections Active WebSocket connections\n");
    m.push_str("# TYPE ghostwire_websocket_connections gauge\n");
    m.push_str(&format!("ghostwire_websocket_connections {}\n", ws));

    m.push_str("# HELP ghostwire_security_events_total Total security events\n");
    m.push_str("# TYPE ghostwire_security_events_total counter\n");
    m.push_str(&format!(
        "ghostwire_security_events_total {}\n",
        sec.total_events
    ));

    m.push_str("# HELP ghostwire_blocked_connections_total Blocked connections\n");
    m.push_str("# TYPE ghostwire_blocked_connections_total counter\n");
    m.push_str(&format!(
        "ghostwire_blocked_connections_total {}\n",
        sec.blocked_connections
    ));

    m.push_str("# HELP ghostwire_rate_limit_violations_total Rate limit violations\n");
    m.push_str("# TYPE ghostwire_rate_limit_violations_total counter\n");
    m.push_str(&format!(
        "ghostwire_rate_limit_violations_total {}\n",
        sec.rate_limit_violations
    ));

    m.push_str(
        "# HELP ghostwire_threat_level Current threat level (0=low 1=medium 2=high 3=critical)\n",
    );
    m.push_str("# TYPE ghostwire_threat_level gauge\n");
    m.push_str(&format!(
        "ghostwire_threat_level {}\n",
        match sec.threat_level {
            crate::core::security_v2::ThreatLevel::Low => 0,
            crate::core::security_v2::ThreatLevel::Medium => 1,
            crate::core::security_v2::ThreatLevel::High => 2,
            crate::core::security_v2::ThreatLevel::Critical => 3,
        }
    ));

    (
        [(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        m,
    )
}

#[derive(Deserialize)]
pub struct ErrorReportRequest {
    pub error: String,
}
