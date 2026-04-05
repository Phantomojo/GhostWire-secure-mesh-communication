// Health check endpoint for GhostWire - Simplified version
// File: ghostwire/src/controllers/health.rs

use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub message: String,
    pub timestamp: String,
}

pub struct HealthController;

impl HealthController {
    /// GET /api/v2/health - Simplified health check endpoint
    pub async fn health(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        let response = HealthResponse {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            message: "GhostWire API is running".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        (StatusCode::OK, Json(ApiV2Response::success(response))).into_response()
    }

    /// GET /api/v2/ready - Readiness check
    pub async fn ready(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        Json(ApiV2Response::success("ready")).into_response()
    }

    /// GET /api/v2/live - Liveness check (is process alive?)
    pub async fn live() -> impl IntoResponse {
        Json(ApiV2Response::success("alive")).into_response()
    }
}
