# axum crate documentation

## Overview

axum is an HTTP routing and request-handling library built on tokio and tower.

## License

MIT

## Website

https://docs.rs/axum

## Source

https://github.com/tokio-rs/axum

---

## Key Concepts

### Router

```rust
use axum::{routing::get, Router};

let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/api/users", get(list_users).post(create_user));
```

### Handlers

```rust
async fn list_users() -> Json<Vec<User>> {
    Json(get_all_users().await)
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    Json(create_user(payload).await)
}
```

### Extractors

```rust
use axum::{
    extract::{Path, Query, Json, State},
    http::StatusCode,
};

// Path parameters
async fn get_user(Path(user_id): Path<u64>) -> Json<User> {}

// Query parameters
async fn search(Query(params): Query<SearchParams>) -> Json<Vec<User>> {}

// JSON body
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {}

// State
async fn handler(State(state): State<AppState>) -> impl IntoResponse {}
```

### Response

```rust
use axum::{
    response::{Html, Redirect, Json},
    http::StatusCode,
};

async fn html() -> Html<String> { Html("<h1>Hello</h1>".into()) }

async fn json() -> Json<Value> { Json(json!({"data": 42})) }

async fn error() -> (StatusCode, &'static str) { (StatusCode::NOT_FOUND, "Not found") }

async fn redirect() -> Redirect { Redirect::to("/other") }
```

### Middleware

```rust
use axum::{middleware, Router};
use tower::ServiceBuilder;

let app = Router::new()
    .route("/", get(handler))
    .layer(middleware::from_fn(auth))
    .layer(ServiceBuilder::new().rate_limit(100, std::time::Duration::SEC).into_inner());
```

### WebSocket

```rust
use axum::{
    extract::ws::{WebSocket, Message},
    response::IntoResponse,
};

async fn ws_handler(ws: WebSocket) {
    while let Some(msg) = ws.recv().await {
        ws.send(Message::Text("echo: ".to_string() + &msg.to_string())).await;
    }
}
```

---

## Feature Flags

| Feature | Description             |
| ------- | ----------------------- |
| http1   | HTTP/1.1 support        |
| http2   | HTTP/2 support          |
| json    | JSON extractor/response |
| ws      | WebSocket support       |
| form    | Form data extraction    |

---

## GhostWire Usage

GhostWire uses axum for:

- HTTP API server
- WebSocket connections for real-time messaging
- REST endpoints for identity, messaging, peers
