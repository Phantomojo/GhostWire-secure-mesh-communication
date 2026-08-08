//! ARK Controller
//!
//! Handles knowledge archive operations: collections, entries, search.

use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateEntryRequest {
    pub title: String,
    pub body: String,
    pub collection: String,
    pub tags: Option<Vec<String>>,
    pub kind: Option<String>,   // "note" | "article" | "code" | "reference"
    pub access: Option<String>, // "private" | "trusted" | "public"
}

pub struct ArkController;

impl ArkController {
    pub async fn list_collections(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let collections = core.knowledge_ark.list_collections().await;
        Json(ApiV2Response::success(collections))
    }

    pub async fn create_collection(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let name = req["name"].as_str().unwrap_or("Untitled").to_string();
        let core = state.core.read().await;
        let col = core.knowledge_ark.create_collection(name).await;
        Json(ApiV2Response::success(col))
    }

    pub async fn create_entry(
        State(state): State<Arc<AppState>>,
        Json(req): Json<CreateEntryRequest>,
    ) -> impl IntoResponse {
        use crate::core::knowledge::knowledge_ark::{AccessLevel, ArkEntry, EntryKind};

        let core = state.core.read().await;
        let author = core.get_identity_id().await;

        let kind = match req.kind.as_deref().unwrap_or("note") {
            "article" => EntryKind::Article,
            "code" => EntryKind::Code {
                language: "unknown".into(),
            },
            "reference" => EntryKind::Reference {
                url: String::new(),
                archived_cid: None,
            },
            _ => EntryKind::Note,
        };

        let access = match req.access.as_deref().unwrap_or("private") {
            "public" => AccessLevel::Public,
            "trusted" => AccessLevel::Trusted,
            _ => AccessLevel::Private,
        };

        let entry = ArkEntry::new(req.title, req.body, kind, author, req.collection)
            .with_tags(req.tags.unwrap_or_default())
            .with_access(access);

        match core.knowledge_ark.add_entry(entry).await {
            Ok(cid) => {
                Json(ApiV2Response::success(serde_json::json!({ "cid": cid }))).into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("ARK_ERROR", e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn get_entry(
        State(state): State<Arc<AppState>>,
        Path(cid): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.knowledge_ark.get_entry(&cid).await {
            Some(entry) => Json(ApiV2Response::success(entry)).into_response(),
            None => (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error("NOT_FOUND", "Entry not found")),
            )
                .into_response(),
        }
    }

    pub async fn delete_entry(
        State(state): State<Arc<AppState>>,
        Path(cid): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.knowledge_ark.delete_entry(&cid).await {
            Ok(_) => Json(ApiV2Response::success(
                serde_json::json!({ "deleted": cid }),
            ))
            .into_response(),
            Err(e) => (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error("DELETE_ERROR", e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn search(
        State(state): State<Arc<AppState>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let q = params.get("q").map(|s| s.as_str()).unwrap_or("");
        if q.is_empty() {
            return Json(ApiV2Response::success(serde_json::json!({ "results": [] })))
                .into_response();
        }
        let core = state.core.read().await;
        let results = core.knowledge_ark.search(q).await;
        let payload: Vec<_> = results
            .into_iter()
            .map(|(entry, score)| serde_json::json!({ "entry": entry, "score": score }))
            .collect();
        Json(ApiV2Response::success(payload)).into_response()
    }

    pub async fn search_by_tag(
        State(state): State<Arc<AppState>>,
        Path(tag): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        let entries = core.knowledge_ark.search_by_tag(&tag).await;
        Json(ApiV2Response::success(entries))
    }

    pub async fn stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let stats = core.knowledge_ark.stats().await;
        Json(ApiV2Response::success(stats))
    }
}
