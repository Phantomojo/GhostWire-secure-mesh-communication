use crate::api_v2_types::*;
use crate::web::{
    current_session_epoch, get_client_ip, AppState, Claims, LoginRequest, LoginResponse,
    AUTH_TOKEN, JWT_SECRET,
};
use axum::body::Body;
use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Whether an address is local enough to be handed the master `AUTH_TOKEN`.
///
/// Numeric range matching, NOT string prefixes. A `starts_with("172.")` test
/// wrongly accepts 172.0-172.15 and 172.32-172.255, which are PUBLIC address
/// space rather than RFC1918, letting a remote Internet host mint fresh admin
/// tokens. This is the same class of bug fixed in
/// `web.rs::allowed_network_host`; both are now range-checked.
pub(crate) fn is_local_address(ip: std::net::IpAddr) -> bool {
    match ip {
        std::net::IpAddr::V4(v4) => {
            let o = v4.octets();
            v4.is_loopback()
                // 172.16.0.0/12 - the Docker/Podman default bridge lives here
                || (o[0] == 172 && (16..=31).contains(&o[1]))
                // 192.168.0.0/16
                || (o[0] == 192 && o[1] == 168)
                // 100.64.0.0/10 - Tailscale CGNAT
                || (o[0] == 100 && (64..=127).contains(&o[1]))
        }
        std::net::IpAddr::V6(v6) => {
            v6.is_loopback()
                // fc00::/7 unique-local
                || (v6.segments()[0] & 0xfe00) == 0xfc00
                // fe80::/10 link-local
                || (v6.segments()[0] & 0xffc0) == 0xfe80
        }
    }
}

pub struct AuthController;

impl AuthController {
    /// V2 Login Handler - Standardized response format
    pub async fn login(
        State(_state): State<Arc<AppState>>,
        Json(payload): Json<LoginRequest>,
    ) -> impl IntoResponse {
        // Constant-time comparison. A plain `==` on a secret leaks its length and
        // the position of the first differing byte via response timing, which is
        // enough to recover a token byte-by-byte. web.rs already does this.
        use subtle::ConstantTimeEq;
        if bool::from(payload.token.as_bytes().ct_eq(AUTH_TOKEN.as_bytes())) {
            let expiration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
                + 60 * 60 * 24; // 24 hours

            let claims = Claims {
                sub: "admin".to_owned(),
                exp: expiration as usize,
                session_epoch: current_session_epoch(),
            };

            match jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET.as_bytes()),
            ) {
                Ok(token) => (
                    StatusCode::OK,
                    Json(ApiV2Response::success(LoginResponse { token })),
                )
                    .into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        format!("Failed to generate token: {}", e),
                    )),
                )
                    .into_response(),
            }
        } else {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidCredentials.as_str(),
                    "Invalid authentication token",
                )),
            )
                .into_response()
        }
    }

    /// V2 Verify Token Handler
    pub async fn verify_token(
        State(_state): State<Arc<AppState>>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        let auth_header = req
            .headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        let jwt = match auth_header {
            Some(t) => t,
            None => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::Unauthorized.as_str(),
                        "Missing Authorization header",
                    )),
                )
                    .into_response();
            }
        };

        let validation = jsonwebtoken::Validation::default();
        match jsonwebtoken::decode::<crate::web::Claims>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(crate::web::JWT_SECRET.as_bytes()),
            &validation,
        ) {
            Ok(token_data) => (
                StatusCode::OK,
                Json(ApiV2Response::success(serde_json::json!({
                    "valid": true,
                    "sub": token_data.claims.sub,
                    "exp": token_data.claims.exp,
                }))),
            )
                .into_response(),
            Err(e) => (
                StatusCode::UNAUTHORIZED,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::Unauthorized.as_str(),
                    format!("Invalid token: {}", e),
                )),
            )
                .into_response(),
        }
    }

    /// Get Dev Token - Restricted to local access + DEBUG builds only
    #[cfg(debug_assertions)]
    pub async fn get_dev_token(
        State(_state): State<Arc<AppState>>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        let ip = get_client_ip(&req, addr);

        // Strict check: local loopback OR Docker bridge IPs only
        // Numeric range matching, not string prefixes. A `172.` prefix test
        // wrongly accepts 172.0-172.15 and 172.32-172.255, which are PUBLIC
        // address space, not RFC1918 - so a remote Internet host could mint
        // fresh admin tokens from a token it already held. Same class of bug
        // as the one fixed in `web.rs::allowed_network_host`.
        let is_local = is_local_address(ip);

        if !is_local {
            tracing::warn!(target: "web", "Unauthorized remote access attempt to /api/dev/token from {}", ip);
            return (
                StatusCode::FORBIDDEN,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::Unauthorized.as_str(),
                    "Access restricted to localhost",
                )),
            )
                .into_response();
        }

        Json(ApiV2Response::success(AUTH_TOKEN.clone())).into_response()
    }

    /// Stub for release builds (dev token disabled)
    #[cfg(not(debug_assertions))]
    pub async fn get_dev_token(
        _: State<Arc<AppState>>,
        _: ConnectInfo<SocketAddr>,
        _: Request<Body>,
    ) -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            Json(ApiV2Response::<()>::error(
                StandardErrorCode::NotFound.as_str(),
                "Dev token endpoint disabled in release builds",
            )),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::is_local_address;
    use std::net::IpAddr;

    fn v4(s: &str) -> IpAddr {
        s.parse().expect("valid ip")
    }

    /// The dev-token route hands out the MASTER `AUTH_TOKEN`, which is a
    /// full-privilege credential. Anything that is not genuinely on the local
    /// network must be refused.
    #[test]
    fn accepts_genuine_local_addresses() {
        for ip in [
            "127.0.0.1",
            "127.1.2.3",
            "::1",
            "172.17.0.2",     // Docker default bridge
            "172.16.0.1",     // 172.16/12 lower edge
            "172.31.255.254", // 172.16/12 upper edge
            "192.168.1.10",
            "100.64.0.1", // Tailscale
            "100.127.255.254",
            "fd00::1",
            "fe80::1",
        ] {
            assert!(is_local_address(v4(ip)), "{ip} should be treated as local");
        }
    }

    /// 172.0-172.15 and 172.32-172.255 are PUBLIC address space. A string
    /// prefix check on "172." accepted all of them, so a remote host that had
    /// obtained the token could mint new ones.
    #[test]
    fn rejects_public_addresses_including_172_neighbours() {
        for ip in [
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
            "2001:4860:4860::8888",
        ] {
            assert!(
                !is_local_address(v4(ip)),
                "{ip} must NOT be treated as local"
            );
        }
    }
}
