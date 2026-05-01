//! BLE Transport Controller — API endpoints for BLE status, discovery, and messaging
//!
//! Requires the `bluetooth` feature (enabled by default).

use crate::core::transport::TransportKind;
use crate::web::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

pub struct BleController;

#[derive(Deserialize)]
pub struct BleConnectRequest {
    pub address: String,
}

#[derive(Deserialize)]
pub struct BleSendRequest {
    pub peer_id: String,
    pub data: String,
}

// TODO: Helper function for ble_send - commented out due to Send bound issue
/*
async fn do_ble_send(
    state: Arc<AppState>,
    peer_id: String,
    data: Vec<u8>,
) -> Result<(), String> {
    use crate::core::transport::TransportKind;

    let core = state.core.read().await;
    let registry = core.transport_registry.read().await;

    if let Some(TransportKind::Bluetooth(ble_arc)) = registry.get("bluetooth") {
        let ble = ble_arc.read().await;
        ble.send_ble_data(&peer_id, &data).await
            .map_err(|e| e.to_string())
    } else {
        Err("BLE transport not found".to_string())
    }
}
*/

impl BleController {
    /// GET /api/ble/status — BLE transport status
    pub async fn status(
        State(state): State<Arc<AppState>>,
    ) -> Response {
        let core = state.core.read().await;
        let registry = core.transport_registry.read().await;

        let response = if let Some(TransportKind::Bluetooth(ble_arc)) = registry.get("bluetooth") {
            let ble = ble_arc.read().await;
            serde_json::json!({
                "available": ble.is_available(),
                "adapter_name": if ble.is_available() { Some("hci0".to_string()) } else { None },
                "advertising": true,
                "scanning": true,
                "discovered_peers": ble.discovered_peers(),
                "connected_peers": ble.connected_peers(),
                "encrypted_sessions": ble.encrypted_peers(),
            })
        } else {
            serde_json::json!({
                "available": false,
                "adapter_name": null,
                "advertising": false,
                "scanning": false,
                "discovered_peers": [],
                "connected_peers": [],
                "encrypted_sessions": [],
            })
        };

        (StatusCode::OK, Json(response)).into_response()
    }

    /// POST /api/ble/connect — Connect to a BLE peer
    pub async fn connect(
        State(state): State<Arc<AppState>>,
        Json(req): Json<BleConnectRequest>,
    ) -> Response {
        let core = state.core.read().await;
        let registry = core.transport_registry.read().await;

        let response = if let Some(TransportKind::Bluetooth(ble_arc)) = registry.get("bluetooth") {
            let ble = ble_arc.read().await;
            match ble.connect_peer(&req.address).await {
                Ok(()) => (StatusCode::OK, serde_json::json!({
                    "status": "connected",
                    "peer": req.address
                })),
                Err(e) => (StatusCode::BAD_REQUEST, serde_json::json!({
                    "status": "failed",
                    "error": e.to_string()
                })),
            }
        } else {
            (StatusCode::SERVICE_UNAVAILABLE, serde_json::json!({
                "status": "unavailable",
                "error": "BLE transport not available"
            }))
        };

        (response.0, Json(response.1)).into_response()
    }

    // TODO: ble_send blocked by axum Handler trait issue with non-Send future
    // The function exists but cannot be used as an axum handler due to Send bounds
    /*
    /// POST /api/ble/send — Send a message via BLE
    pub async fn ble_send(
        State(state): State<Arc<AppState>>,
        Json(req): Json<BleSendRequest>,
    ) -> impl IntoResponse {
        // Check if BLE transport is available
        let core = state.core.read().await;
        let has_ble = core.transport_registry.read().await.get("bluetooth").is_some();
        drop(core);

        if !has_ble {
            return (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
                "status": "unavailable",
                "error": "BLE transport not available"
            })));
        }

        // Spawn send task
        let peer_id = req.peer_id.clone();
        let data = req.data.clone();
        let result = tokio::spawn(async move {
            do_ble_send(state, peer_id, data.into_bytes()).await
        }).await;

        match result {
            Ok(Ok(())) => (StatusCode::OK, Json(serde_json::json!({
                "status": "sent",
                "peer": req.peer_id,
                "bytes": req.data.len()
            }))),
            Ok(Err(e)) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({
                "status": "failed",
                "error": e
            }))),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "error",
                "error": e.to_string()
            }))),
        }
    }
    */
}
