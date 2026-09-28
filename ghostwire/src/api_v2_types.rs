// API V2 Response Types - Standardized envelopes
// Add to web.rs near the top with other types

use axum::Json;
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

    /// Build an error response PAIRED with a correct HTTP status code.
    ///
    /// `Json(Self::error(..))` on its own serialises the error envelope with an
    /// implicit HTTP 200, so any client that decides success from the status
    /// code alone — a script, monitoring, a CLI — treats the failure as a
    /// success. That is not hypothetical: `/api/v2/messages/send` was observed
    /// returning `200 OK` with
    /// `{"error":{"code":"INTERNAL_ERROR","message":"Transmission failed: ..."}}`,
    /// and a CLI built against it printed "Message sent" for a peer that does
    /// not exist.
    ///
    /// Returning the pair together makes the correct pairing the easy one, and
    /// removes the class of bug rather than just these instances of it.
    pub fn error_response(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> (axum::http::StatusCode, Json<Self>) {
        use axum::http::StatusCode;
        let code_str = code.into();
        let status = match code_str.as_str() {
            c if c.contains("INVALID")
                || c.contains("REQUIRED")
                || c.contains("TOO_LONG")
                || c.contains("EMPTY")
                || c.contains("MALFORMED") =>
            {
                StatusCode::BAD_REQUEST
            }
            c if c.contains("UNAUTHORIZED") || c.contains("AUTH") || c.contains("TOKEN") => {
                StatusCode::UNAUTHORIZED
            }
            c if c.contains("FORBIDDEN") || c.contains("BLOCKED") => StatusCode::FORBIDDEN,
            c if c.contains("NOT_FOUND") || c.contains("UNKNOWN_PEER") => StatusCode::NOT_FOUND,
            c if c.contains("CONFLICT") || c.contains("ALREADY") || c.contains("DEFERRED") => {
                StatusCode::CONFLICT
            }
            // A locked profile is a temporary, recoverable state: the node is
            // healthy and the caller unlocks it. 503 says "not now, try again",
            // where 500 would page someone for a self-inflicted call.
            c if c.contains("PERSISTENCE_LOCKED") => StatusCode::SERVICE_UNAVAILABLE,
            c if c.contains("UNAVAILABLE") => StatusCode::SERVICE_UNAVAILABLE,
            c if c.contains("TIMEOUT") => StatusCode::GATEWAY_TIMEOUT,
            c if c.contains("RATE_LIMIT") || c.contains("QUOTA") => StatusCode::TOO_MANY_REQUESTS,
            c if c.contains("UNIMPLEMENTED") || c.contains("NOT_IMPLEMENTED") => {
                StatusCode::NOT_IMPLEMENTED
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(Self::error(code_str, message)))
    }

    /// Add pagination metadata
    pub fn with_pagination(mut self, total: usize, page: usize, per_page: usize) -> Self {
        self.pagination = Some(PaginationMeta {
            total,
            page,
            per_page,
            pages: total.div_ceil(per_page),
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
