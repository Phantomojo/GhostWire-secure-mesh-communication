use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum::body::Body;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::api_v2_types::*;
use crate::web::{get_client_ip, AppState, AUTH_TOKEN, JWT_SECRET, Claims, LoginRequest, LoginResponse};

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
                .as_secs() + 60 * 60 * 24; // 24 hours

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
                ).into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        format!("Failed to generate token: {}", e)
                    )),
                ).into_response(),
            }
        } else {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidCredentials.as_str(),
                    "Invalid authentication token"
                )),
            ).into_response()
        }
    }

    /// V2 Verify Token Handler
    pub async fn verify_token(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        Json(ApiV2Response::success("Token is valid".to_string()))
    }

    /// Get Dev Token - Restricted to local access
    pub async fn get_dev_token(
        State(_state): State<Arc<AppState>>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        req: Request<Body>
    ) -> impl IntoResponse {
        let ip = get_client_ip(&req, addr);
        
        // Loosened check: local loopback OR bridge IPs commonly used by containers
        let is_local = ip.is_loopback() || 
                       ip.to_string().starts_with("172.") || // Docker/Podman
                       ip.to_string().starts_with("10.") ||  // Internal private
                       ip.to_string().starts_with("192.168.") ||
                       ip.to_string() == "::1";

        if !is_local {
            tracing::warn!(target: "web", "Unauthorized remote access attempt to /api/dev/token from {}", ip);
            return (
                StatusCode::FORBIDDEN,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::Unauthorized.as_str(),
                    "Access restricted to localhost"
                )),
            ).into_response();
        }

        Json(ApiV2Response::success(AUTH_TOKEN.clone())).into_response()
    }
}
