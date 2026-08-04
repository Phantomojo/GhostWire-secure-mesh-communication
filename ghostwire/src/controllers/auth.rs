use crate::api_v2_types::*;
use crate::web::{
    get_client_ip, AppState, Claims, LoginRequest, LoginResponse, AUTH_TOKEN, JWT_SECRET,
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

pub struct AuthController;

impl AuthController {
    /// V2 Login Handler - Standardized response format
    pub async fn login(
        State(_state): State<Arc<AppState>>,
        Json(payload): Json<LoginRequest>,
    ) -> impl IntoResponse {
        if payload.token == *AUTH_TOKEN {
            let expiration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
                + 60 * 60 * 24; // 24 hours

            let claims = Claims {
                sub: "admin".to_owned(),
                exp: expiration as usize,
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
        let is_local = ip.is_loopback() ||
                       ip.to_string().starts_with("172.") || // Docker/Podman
                       ip.to_string() == "::1";

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
