Crate axum Copy item path

<span class="sub-heading"><a href="../src/axum/lib.rs.html#1-518" class="src">Source</a>
</span>

Expand description

axum is a web application framework that focuses on ergonomics and
modularity.

# <a href="#high-level-features" class="doc-anchor">§</a>High-level features

-   Route requests to handlers with a macro-free API.
-   Declaratively parse requests using extractors.
-   Simple and predictable error handling model.
-   Generate responses with minimal boilerplate.
-   Take full advantage of the [`tower`](https://crates.io/crates/tower)
    and [`tower-http`](https://crates.io/crates/tower-http) ecosystem of
    middleware, services, and utilities.

In particular, the last point is what sets `axum` apart from other
frameworks. `axum` doesn’t have its own middleware system but instead
uses
[`tower::Service`](../tower_service/trait.Service.html "trait tower_service::Service").
This means `axum` gets timeouts, tracing, compression, authorization,
and more, for free. It also enables you to share middleware with
applications written using [`hyper`](http://crates.io/crates/hyper) or
[`tonic`](http://crates.io/crates/tonic).

# <a href="#compatibility" class="doc-anchor">§</a>Compatibility

axum is designed to work with [tokio](../tokio/index.html "mod tokio")
and [hyper](../hyper/index.html "mod hyper"). Runtime and transport
layer independence is not a goal, at least for the time being.

# <a href="#example" class="doc-anchor">§</a>Example

The “Hello, World!” of axum is:

``` {.rust .rust-example-rendered}
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Note using `#[tokio::main]` requires you enable tokio’s `macros` and
`rt-multi-thread` features or just `full` to enable all features
(`cargo add tokio --features macros,rt-multi-thread`).

# <a href="#routing" class="doc-anchor">§</a>Routing

[`Router`](struct.Router.html "struct axum::Router") is used to set up
which paths goes to which services:

``` {.rust .rust-example-rendered}
use axum::{Router, routing::get};

// our router
let app = Router::new()
    .route("/", get(root))
    .route("/foo", get(get_foo).post(post_foo))
    .route("/foo/bar", get(foo_bar));

// which calls one of these handlers
async fn root() {}
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}
```

See [`Router`](struct.Router.html "struct axum::Router") for more
details on routing.

# <a href="#handlers" class="doc-anchor">§</a>Handlers

In axum a “handler” is an async function that accepts zero or more
[“extractors”](extract/index.html "mod axum::extract") as arguments and
returns something that can be converted [into a
response](response/index.html "mod axum::response").

Handlers are where your application logic lives and axum applications
are built by routing between handlers.

See [`handler`](handler/index.html "mod axum::handler") for more details
on handlers.

# <a href="#extractors" class="doc-anchor">§</a>Extractors

An extractor is a type that implements
[`FromRequest`](extract/trait.FromRequest.html "trait axum::extract::FromRequest")
or
[`FromRequestParts`](extract/trait.FromRequestParts.html "trait axum::extract::FromRequestParts").
Extractors are how you pick apart the incoming request to get the parts
your handler needs.

``` {.rust .rust-example-rendered}
use axum::extract::{Path, Query, Json};
use std::collections::HashMap;

// `Path` gives you the path parameters and deserializes them.
async fn path(Path(user_id): Path<u32>) {}

// `Query` gives you the query parameters and deserializes them.
async fn query(Query(params): Query<HashMap<String, String>>) {}

// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports any type that implements
// `serde::Deserialize`.
async fn json(Json(payload): Json<serde_json::Value>) {}
```

See [`extract`](extract/index.html "mod axum::extract") for more details
on extractors.

# <a href="#responses" class="doc-anchor">§</a>Responses

Anything that implements
[`IntoResponse`](response/trait.IntoResponse.html "trait axum::response::IntoResponse")
can be returned from handlers.

``` {.rust .rust-example-rendered}
use axum::{
    body::Body,
    routing::get,
    response::Json,
    Router,
};
use serde_json::{Value, json};

// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
async fn plain_text() -> &'static str {
    "foo"
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

let app = Router::new()
    .route("/plain_text", get(plain_text))
    .route("/json", get(json));
```

See [`response`](response/index.html "mod axum::response") for more
details on building responses.

# <a href="#error-handling" class="doc-anchor">§</a>Error handling

axum aims to have a simple and predictable error handling model. That
means it is simple to convert errors into responses and you are
guaranteed that all errors are handled.

See
[`error_handling`](error_handling/index.html "mod axum::error_handling")
for more details on axum’s error handling model and how to handle errors
gracefully.

# <a href="#middleware" class="doc-anchor">§</a>Middleware

There are several different ways to write middleware for axum. See
[`middleware`](middleware/index.html "mod axum::middleware") for more
details.

# <a href="#sharing-state-with-handlers" class="doc-anchor">§</a>Sharing state with handlers

It is common to share some state between handlers. For example, a pool
of database connections or clients to other services may need to be
shared.

The three most common ways of doing that are:

-   Using the
    [`State`](extract/struct.State.html "struct axum::extract::State")
    extractor
-   Using request extensions
-   Using closure captures

## <a href="#using-the-state-extractor" class="doc-anchor">§</a>Using the [`State`](extract/struct.State.html "struct axum::extract::State") extractor

``` {.rust .rust-example-rendered}
use axum::{
    extract::State,
    routing::get,
    Router,
};
use std::sync::Arc;

struct AppState {
    // ...
}

let shared_state = Arc::new(AppState { /* ... */ });

let app = Router::new()
    .route("/", get(handler))
    .with_state(shared_state);

async fn handler(
    State(state): State<Arc<AppState>>,
) {
    // ...
}
```

You should prefer using
[`State`](extract/struct.State.html "struct axum::extract::State") if
possible since it’s more type safe. The downside is that it’s less
dynamic than request extensions.

See [`State`](extract/struct.State.html "struct axum::extract::State")
for more details about accessing state.

## <a href="#using-request-extensions" class="doc-anchor">§</a>Using request extensions

Another way to extract state in handlers is using
[`Extension`](struct.Extension.html "struct axum::Extension") as layer
and extractor:

``` {.rust .rust-example-rendered}
use axum::{
    extract::Extension,
    routing::get,
    Router,
};
use std::sync::Arc;

struct AppState {
    // ...
}

let shared_state = Arc::new(AppState { /* ... */ });

let app = Router::new()
    .route("/", get(handler))
    .layer(Extension(shared_state));

async fn handler(
    Extension(state): Extension<Arc<AppState>>,
) {
    // ...
}
```

The downside to this approach is that you’ll get runtime errors
(specifically a `500 Internal Server Error` response) if you try and
extract an extension that doesn’t exist, perhaps because you forgot to
add the middleware or because you’re extracting the wrong type.

## <a href="#using-closure-captures" class="doc-anchor">§</a>Using closure captures

State can also be passed directly to handlers using closure captures:

``` {.rust .rust-example-rendered}
use axum::{
    Json,
    extract::{Extension, Path},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use serde::Deserialize;

struct AppState {
    // ...
}

let shared_state = Arc::new(AppState { /* ... */ });

let app = Router::new()
    .route(
        "/users",
        post({
            let shared_state = Arc::clone(&shared_state);
            move |body| create_user(body, shared_state)
        }),
    )
    .route(
        "/users/:id",
        get({
            let shared_state = Arc::clone(&shared_state);
            move |path| get_user(path, shared_state)
        }),
    );

async fn get_user(Path(user_id): Path<String>, state: Arc<AppState>) {
    // ...
}

async fn create_user(Json(payload): Json<CreateUserPayload>, state: Arc<AppState>) {
    // ...
}

#[derive(Deserialize)]
struct CreateUserPayload {
    // ...
}
```

The downside to this approach is that it’s a little more verbose than
using [`State`](extract/struct.State.html "struct axum::extract::State")
or extensions.

## <a href="#using-tokios-task_local-macro" class="doc-anchor">§</a>Using [tokio’s `task_local` macro](https://docs.rs/tokio/1/tokio/macro.task_local.html):

This allows to share state with `IntoResponse` implementations.

``` {.rust .rust-example-rendered}
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::task_local;

#[derive(Clone)]
struct CurrentUser {
    name: String,
}
task_local! {
    pub static USER: CurrentUser;
}

async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if let Some(current_user) = authorize_current_user(auth_header).await {
        // State is setup here in the middleware
        Ok(USER.scope(current_user, next.run(req)).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    Some(CurrentUser {
        name: auth_token.to_string(),
    })
}

struct UserResponse;

impl IntoResponse for UserResponse {
    fn into_response(self) -> Response {
        // State is accessed here in the IntoResponse implementation
        let current_user = USER.with(|u| u.clone());
        (StatusCode::OK, current_user.name).into_response()
    }
}

async fn handler() -> UserResponse {
    UserResponse
}

let app: Router = Router::new()
    .route("/", get(handler))
    .route_layer(middleware::from_fn(auth));
```

# <a href="#building-integrations-for-axum" class="doc-anchor">§</a>Building integrations for axum

Libraries authors that want to provide
[`FromRequest`](extract/trait.FromRequest.html "trait axum::extract::FromRequest"),
[`FromRequestParts`](extract/trait.FromRequestParts.html "trait axum::extract::FromRequestParts"),
or
[`IntoResponse`](response/trait.IntoResponse.html "trait axum::response::IntoResponse")
implementations should depend on the
[`axum-core`](http://crates.io/crates/axum-core) crate, instead of
`axum` if possible. [`axum-core`](http://crates.io/crates/axum-core)
contains core types and traits and is less likely to receive breaking
changes.

# <a href="#required-dependencies" class="doc-anchor">§</a>Required dependencies

To use axum there are a few dependencies you have to pull in as well:

``` language-toml
[dependencies]
axum = "<latest-version>"
tokio = { version = "<latest-version>", features = ["full"] }
tower = "<latest-version>"
```

The `"full"` feature for tokio isn’t necessary but it’s the easiest way
to get started.

Tower isn’t strictly necessary either but helpful for testing. See the
testing example in the repo to learn more about testing axum apps.

# <a href="#examples" class="doc-anchor">§</a>Examples

The axum repo contains [a number of
examples](https://github.com/tokio-rs/axum/tree/main/examples) that show
how to put all the pieces together.

# <a href="#feature-flags" class="doc-anchor">§</a>Feature flags

axum uses a set of [feature
flags](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section)
to reduce the amount of compiled and optional dependencies.

The following optional features are available:

| Name | Description | Default? |
|----|----|----|
| `http1` | Enables hyper’s `http1` feature | Yes |
| `http2` | Enables hyper’s `http2` feature | No |
| `json` | Enables the [`Json`](struct.Json.html "struct axum::Json") type and some similar convenience functionality | Yes |
| `macros` | Enables optional utility macros | No |
| `matched-path` | Enables capturing of every request’s router path and the [`MatchedPath`](extract/struct.MatchedPath.html "struct axum::extract::MatchedPath") extractor | Yes |
| `multipart` | Enables parsing `multipart/form-data` requests with [`Multipart`](crate::extract::Multipart) | No |
| `original-uri` | Enables capturing of every request’s original URI and the [`OriginalUri`](extract/struct.OriginalUri.html "struct axum::extract::OriginalUri") extractor | Yes |
| `tokio` | Enables `tokio` as a dependency and `axum::serve`, `SSE` and `extract::connect_info` types. | Yes |
| `tower-log` | Enables `tower`’s `log` feature | Yes |
| `tracing` | Log rejections from built-in extractors | Yes |
| `ws` | Enables WebSockets support via [`extract::ws`](extract/ws/index.html "mod axum::extract::ws") | No |
| `form` | Enables the `Form` extractor | Yes |
| `query` | Enables the `Query` extractor | Yes |

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use async_trait::`<a href="../async_trait/attr.async_trait.html" class="attr"
title="attr async_trait::async_trait"><code>async_trait</code></a>`;`

`pub use `<a href="../http/index.html" class="mod"
title="mod http"><code>http</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="body/index.html" class="mod" title="mod axum::body">body</a>  
HTTP body utilities.

<a href="error_handling/index.html" class="mod"
title="mod axum::error_handling">error_handling</a>  
Error handling model and utilities

<a href="extract/index.html" class="mod"
title="mod axum::extract">extract</a>  
Types and traits for extracting data from requests.

<a href="handler/index.html" class="mod"
title="mod axum::handler">handler</a>  
Async functions that can be used to handle requests.

<a href="middleware/index.html" class="mod"
title="mod axum::middleware">middleware</a>  
Utilities for writing middleware

<a href="response/index.html" class="mod"
title="mod axum::response">response</a>  
Types and traits for generating responses.

<a href="routing/index.html" class="mod"
title="mod axum::routing">routing</a>  
Routing between
[`Service`](../tower_service/trait.Service.html "trait tower_service::Service")s
and handlers.

<a href="serve/index.html" class="mod" title="mod axum::serve">serve</a>  
Serve services.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct axum::Error">Error</a>  
Errors that can happen when using axum.

<a href="struct.Extension.html" class="struct"
title="struct axum::Extension">Extension</a>  
Extractor and response for extensions.

<a href="struct.Form.html" class="struct"
title="struct axum::Form">Form</a>  
URL encoded extractor and response.

<a href="struct.Json.html" class="struct"
title="struct axum::Json">Json</a>  
JSON Extractor / Response.

<a href="struct.Router.html" class="struct"
title="struct axum::Router">Router</a>  
The router type for composing handlers and services.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.RequestExt.html" class="trait"
title="trait axum::RequestExt">RequestExt</a>  
Extension trait that adds additional methods to
[`Request`](extract/type.Request.html "type axum::extract::Request").

<a href="trait.RequestPartsExt.html" class="trait"
title="trait axum::RequestPartsExt">RequestPartsExt</a>  
Extension trait that adds additional methods to
[`Parts`](../http/request/struct.Parts.html "struct http::request::Parts").

<a href="trait.ServiceExt.html" class="trait"
title="trait axum::ServiceExt">ServiceExt</a>  
Extension trait that adds additional methods to any
[`Service`](../tower_service/trait.Service.html "trait tower_service::Service").

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.serve.html" class="fn" title="fn axum::serve">serve</a>  
Serve the service with the supplied listener.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BoxError.html" class="type"
title="type axum::BoxError">BoxError</a>  
Alias for a type-erased error type.
