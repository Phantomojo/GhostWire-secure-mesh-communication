// API V2 Response Types - Standardized envelopes
// Add to web.rs near the top with other types

use serde::Serialize;
use uuid::Uuid;

/// Standardized API v2 response envelope
#[derive(Serialize)]
pub struct ApiV2Response<T> {
    /// API version
    version: String,
    /// Request timestamp (ISO 8601)
    timestamp: String,
    /// Unique request ID for tracking
    request_id: String,
    /// Response data (null on error)
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    /// Error details (null on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ApiV2Error>,
    /// Pagination metadata for list endpoints
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination: Option<PaginationMeta>,
    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<HateoasLinks>,
    /// Rate limiting info
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit: Option<RateLimitInfo>,
}

/// Enhanced error format with codes and help
#[derive(Serialize)]
pub struct ApiV2Error {
    /// Machine-readable error code
    code: String,
    /// Human-readable message
    message: String,
    /// Additional error details (validation errors, etc)
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
    /// Link to documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    help_url: Option<String>,
    /// Trace ID for support
    #[serde(skip_serializing_if = "Option::is_none")]
    trace_id: Option<String>,
}

/// Pagination metadata
#[derive(Serialize)]
pub struct PaginationMeta {
    total: usize,
    page: usize,
    per_page: usize,
    pages: usize,
}

/// HATEOAS links for resource discovery
#[derive(Serialize)]
pub struct HateoasLinks {
    #[serde(rename = "self")]
    self_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related: Option<std::collections::HashMap<String, String>>,
}

/// Rate limit information
#[derive(Serialize)]
pub struct RateLimitInfo {
    limit: usize,
    remaining: usize,
    reset: String, // ISO 8601 timestamp
}

impl<T: Serialize> ApiV2Response<T> {
    /// Create a successful response
    pub fn success(data: T) -> Self {
        Self {
            version: "2.0".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            request_id: Uuid::new_v4().to_string(),
            data: Some(data),
            error: None,
            pagination: None,
            links: None,
            rate_limit: None,
        }
    }

    /// Create an error response
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            version: "2.0".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            request_id: Uuid::new_v4().to_string(),
            data: None,
            error: Some(ApiV2Error {
                code: code.into(),
                message: message.into(),
                details: None,
                help_url: None,
                trace_id: None,
            }),
            pagination: None,
            links: None,
            rate_limit: None,
        }
    }

    /// Add pagination metadata
    pub fn with_pagination(mut self, total: usize, page: usize, per_page: usize) -> Self {
        self.pagination = Some(PaginationMeta {
            total,
            page,
            per_page,
            pages: (total + per_page - 1) / per_page,
        });
        self
    }

    /// Add HATEOAS links
    pub fn with_links(mut self, self_link: String) -> Self {
        self.links = Some(HateoasLinks {
            self_link,
            next: None,
            prev: None,
            related: None,
        });
        self
    }
}

/// Standard error codes for the API
#[derive(Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StandardErrorCode {
    #[allow(dead_code)] // ponytail: standard error code, used in API surface
    ProfileLoadFailed,
    InvalidCredentials,
    CoreInitializationFailed,
    Unauthorized,
    #[allow(dead_code)] // ponytail: standard error code, used in API surface
    NotFound,
    InternalError,
    InvalidInput,
    #[allow(dead_code)] // ponytail: standard error code, used in API surface
    IdentityLocked,
    RateLimited,
}

impl StandardErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProfileLoadFailed => "PROFILE_LOAD_FAILED",
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::CoreInitializationFailed => "CORE_INITIALIZATION_FAILED",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::NotFound => "NOT_FOUND",
            Self::InternalError => "INTERNAL_ERROR",
            Self::InvalidInput => "INVALID_INPUT",
            Self::IdentityLocked => "IDENTITY_LOCKED",
            Self::RateLimited => "RATE_LIMITED",
        }
    }
}

// Example usage:
/*
pub async fn get_profile_v2(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let profile = ProfileInfo {
        id: state.core.get_identity_id(),
        username: get_username().unwrap_or("Unknown".to_string()),
        public_key: base64::encode(state.core.get_public_key()),
        key_id: state.core.get_key_id(),
    };

    Json(ApiV2Response::success(profile)
        .with_links("/api/v2/profile".to_string()))
}
*/
