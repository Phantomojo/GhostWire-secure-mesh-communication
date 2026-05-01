[axum](../index.html)

Module extract Copy item path

<span class="sub-heading"><a href="../../src/axum/extract/mod.rs.html#1-112"
class="src">Source</a> </span>

Expand description

Types and traits for extracting data from requests.

# <a href="#intro" class="doc-anchor">§</a>Intro

A handler function is an async function that takes any number of
“extractors” as arguments. An extractor is a type that implements
[`FromRequest`](trait.FromRequest.html "trait axum::extract::FromRequest")
or
[`FromRequestParts`](trait.FromRequestParts.html "trait axum::extract::FromRequestParts").

For example, [`Json`](../struct.Json.html "struct axum::Json") is an
extractor that consumes the request body and deserializes it as JSON
into some target type:

``` {.rust .rust-example-rendered}
use axum::{
    extract::Json,
    routing::post,
    handler::Handler,
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

async fn create_user(Json(payload): Json<CreateUser>) {
    // ...
}

let app = Router::new().route("/users", post(create_user));
```

# <a href="#common-extractors" class="doc-anchor">§</a>Common extractors

Some commonly used extractors are:

``` {.rust .rust-example-rendered}
use axum::{
    extract::{Request, Json, Path, Extension, Query},
    routing::post,
    http::header::HeaderMap,
    body::{Bytes, Body},
    Router,
};
use serde_json::Value;
use std::collections::HashMap;

// `Path` gives you the path parameters and deserializes them. See its docs for
// more details
async fn path(Path(user_id): Path<u32>) {}

// `Query` gives you the query parameters and deserializes them.
async fn query(Query(params): Query<HashMap<String, String>>) {}

// `HeaderMap` gives you all the headers
async fn headers(headers: HeaderMap) {}

// `String` consumes the request body and ensures it is valid utf-8
async fn string(body: String) {}

// `Bytes` gives you the raw request body
async fn bytes(body: Bytes) {}

// We've already seen `Json` for parsing the request body as json
async fn json(Json(payload): Json<Value>) {}

// `Request` gives you the whole request for maximum control
async fn request(request: Request) {}

// `Extension` extracts data from "request extensions"
// This is commonly used to share state with handlers
async fn extension(Extension(state): Extension<State>) {}

#[derive(Clone)]
struct State { /* ... */ }

let app = Router::new()
    .route("/path/:user_id", post(path))
    .route("/query", post(query))
    .route("/string", post(string))
    .route("/bytes", post(bytes))
    .route("/json", post(json))
    .route("/request", post(request))
    .route("/extension", post(extension));
```

# <a href="#applying-multiple-extractors" class="doc-anchor">§</a>Applying multiple extractors

You can also apply multiple extractors:

``` {.rust .rust-example-rendered}
use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use uuid::Uuid;
use serde::Deserialize;

let app = Router::new().route("/users/:id/things", get(get_user_things));

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 1, per_page: 30 }
    }
}

async fn get_user_things(
    Path(user_id): Path<Uuid>,
    pagination: Option<Query<Pagination>>,
) {
    let Query(pagination) = pagination.unwrap_or_default();

    // ...
}
```

# <a href="#the-order-of-extractors" class="doc-anchor">§</a>The order of extractors

Extractors always run in the order of the function parameters that is
from left to right.

The request body is an asynchronous stream that can only be consumed
once. Therefore you can only have one extractor that consumes the
request body. axum enforces this by requiring such extractors to be the
*last* argument your handler takes.

For example

``` {.rust .rust-example-rendered}
use axum::{extract::State, http::{Method, HeaderMap}};

async fn handler(
    // `Method` and `HeaderMap` don't consume the request body so they can
    // put anywhere in the argument list (but before `body`)
    method: Method,
    headers: HeaderMap,
    // `State` is also an extractor so it needs to be before `body`
    State(state): State<AppState>,
    // `String` consumes the request body and thus must be the last extractor
    body: String,
) {
    // ...
}
```

We get a compile error if `String` isn’t the last extractor:

<a href="#" class="tooltip"
title="This example deliberately fails to compile">ⓘ</a>

``` {.rust .rust-example-rendered}
use axum::http::Method;

async fn handler(
    // this doesn't work since `String` must be the last argument
    body: String,
    method: Method,
) {
    // ...
}
```

This also means you cannot consume the request body twice:

<a href="#" class="tooltip"
title="This example deliberately fails to compile">ⓘ</a>

``` {.rust .rust-example-rendered}
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct Payload {}

async fn handler(
    // `String` and `Json` both consume the request body
    // so they cannot both be used
    string_body: String,
    json_body: Json<Payload>,
) {
    // ...
}
```

axum enforces this by requiring the last extractor implements
[`FromRequest`](trait.FromRequest.html "trait axum::extract::FromRequest")
and all others implement
[`FromRequestParts`](trait.FromRequestParts.html "trait axum::extract::FromRequestParts").

# <a href="#optional-extractors" class="doc-anchor">§</a>Optional extractors

All extractors defined in axum will reject the request if it doesn’t
match. If you wish to make an extractor optional you can wrap it in
`Option`:

``` {.rust .rust-example-rendered}
use axum::{
    extract::Json,
    routing::post,
    Router,
};
use serde_json::Value;

async fn create_user(payload: Option<Json<Value>>) {
    if let Some(payload) = payload {
        // We got a valid JSON payload
    } else {
        // Payload wasn't valid JSON
    }
}

let app = Router::new().route("/users", post(create_user));
```

Wrapping extractors in `Result` makes them optional and gives you the
reason the extraction failed:

``` {.rust .rust-example-rendered}
use axum::{
    extract::{Json, rejection::JsonRejection},
    routing::post,
    Router,
};
use serde_json::Value;

async fn create_user(payload: Result<Json<Value>, JsonRejection>) {
    match payload {
        Ok(payload) => {
            // We got a valid JSON payload
        }
        Err(JsonRejection::MissingJsonContentType(_)) => {
            // Request didn't have `Content-Type: application/json`
            // header
        }
        Err(JsonRejection::JsonDataError(_)) => {
            // Couldn't deserialize the body into the target type
        }
        Err(JsonRejection::JsonSyntaxError(_)) => {
            // Syntax error in the body
        }
        Err(JsonRejection::BytesRejection(_)) => {
            // Failed to extract the request body
        }
        Err(_) => {
            // `JsonRejection` is marked `#[non_exhaustive]` so match must
            // include a catch-all case.
        }
    }
}

let app = Router::new().route("/users", post(create_user));
```

Another option is to make use of the optional extractors in
[axum-extra](https://docs.rs/axum-extra/latest/axum_extra/extract/index.html)
that either returns `None` if there are no query parameters in the
request URI, or returns `Some(T)` if deserialization was successful. If
the deserialization was not successful, the request is rejected.

# <a href="#customizing-extractor-responses" class="doc-anchor">§</a>Customizing extractor responses

If an extractor fails it will return a response with the error and your
handler will not be called. To customize the error response you have two
options:

1.  Use `Result<T, T::Rejection>` as your extractor like shown in
    [“Optional extractors”](#optional-extractors). This works well if
    you’re only using the extractor in a single handler.
2.  Create your own extractor that in its
    [`FromRequest`](trait.FromRequest.html "trait axum::extract::FromRequest")
    implementation calls one of axum’s built in extractors but returns a
    different response for rejections. See the
    [customize-extractor-error](https://github.com/tokio-rs/axum/blob/main/examples/customize-extractor-error/src/main.rs)
    example for more details.

# <a href="#accessing-inner-errors" class="doc-anchor">§</a>Accessing inner errors

axum’s built-in extractors don’t directly expose the inner error. This
gives us more flexibility and allows us to change internal
implementations without breaking the public API.

For example that means while
[`Json`](../struct.Json.html "struct axum::Json") is implemented using
[`serde_json`](../../serde_json/index.html "mod serde_json") it doesn’t
directly expose the
[`serde_json::Error`](../../serde_json/error/struct.Error.html "struct serde_json::error::Error")
thats contained in
[`JsonRejection::JsonDataError`](rejection/enum.JsonRejection.html#variant.JsonDataError "variant axum::extract::rejection::JsonRejection::JsonDataError").
However it is still possible to access via methods from
[`std::error::Error`](https://doc.rust-lang.org/1.93.1/core/error/trait.Error.html "trait core::error::Error"):

``` {.rust .rust-example-rendered}
use std::error::Error;
use axum::{
    extract::{Json, rejection::JsonRejection},
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::{json, Value};

async fn handler(
    result: Result<Json<Value>, JsonRejection>,
) -> Result<Json<Value>, (StatusCode, String)> {
    match result {
        // if the client sent valid JSON then we're good
        Ok(Json(payload)) => Ok(Json(json!({ "payload": payload }))),

        Err(err) => match err {
            JsonRejection::JsonDataError(err) => {
                Err(serde_json_error_response(err))
            }
            JsonRejection::JsonSyntaxError(err) => {
                Err(serde_json_error_response(err))
            }
            // handle other rejections from the `Json` extractor
            JsonRejection::MissingJsonContentType(_) => Err((
                StatusCode::BAD_REQUEST,
                "Missing `Content-Type: application/json` header".to_string(),
            )),
            JsonRejection::BytesRejection(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to buffer request body".to_string(),
            )),
            // we must provide a catch-all case since `JsonRejection` is marked
            // `#[non_exhaustive]`
            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error".to_string(),
            )),
        },
    }
}

// attempt to extract the inner `serde_path_to_error::Error<serde_json::Error>`,
// if that succeeds we can provide a more specific error.
//
// `Json` uses `serde_path_to_error` so the error will be wrapped in `serde_path_to_error::Error`.
fn serde_json_error_response<E>(err: E) -> (StatusCode, String)
where
    E: Error + 'static,
{
    if let Some(err) = find_error_source::<serde_path_to_error::Error<serde_json::Error>>(&err) {
        let serde_json_err = err.inner();
        (
            StatusCode::BAD_REQUEST,
            format!(
                "Invalid JSON at line {} column {}",
                serde_json_err.line(),
                serde_json_err.column()
            ),
        )
    } else {
        (StatusCode::BAD_REQUEST, "Unknown error".to_string())
    }
}

// attempt to downcast `err` into a `T` and if that fails recursively try and
// downcast `err`'s source
fn find_error_source<'a, T>(err: &'a (dyn Error + 'static)) -> Option<&'a T>
where
    T: Error + 'static,
{
    if let Some(err) = err.downcast_ref::<T>() {
        Some(err)
    } else if let Some(source) = err.source() {
        find_error_source(source)
    } else {
        None
    }
}
```

Note that while this approach works it might break in the future if axum
changes its implementation to use a different error type internally.
Such changes might happen without major breaking versions.

# <a href="#defining-custom-extractors" class="doc-anchor">§</a>Defining custom extractors

You can also define your own extractors by implementing either
[`FromRequestParts`](trait.FromRequestParts.html "trait axum::extract::FromRequestParts")
or
[`FromRequest`](trait.FromRequest.html "trait axum::extract::FromRequest").

## <a href="#implementing-fromrequestparts" class="doc-anchor">§</a>Implementing `FromRequestParts`

Implement `FromRequestParts` if your extractor doesn’t need access to
the request body:

``` {.rust .rust-example-rendered}
use axum::{
    async_trait,
    extract::FromRequestParts,
    routing::get,
    Router,
    http::{
        StatusCode,
        header::{HeaderValue, USER_AGENT},
        request::Parts,
    },
};

struct ExtractUserAgent(HeaderValue);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUserAgent
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_agent) = parts.headers.get(USER_AGENT) {
            Ok(ExtractUserAgent(user_agent.clone()))
        } else {
            Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
        }
    }
}

async fn handler(ExtractUserAgent(user_agent): ExtractUserAgent) {
    // ...
}

let app = Router::new().route("/foo", get(handler));
```

## <a href="#implementing-fromrequest" class="doc-anchor">§</a>Implementing `FromRequest`

If your extractor needs to consume the request body you must implement
[`FromRequest`](trait.FromRequest.html "trait axum::extract::FromRequest")

``` {.rust .rust-example-rendered}
use axum::{
    async_trait,
    extract::{Request, FromRequest},
    response::{Response, IntoResponse},
    body::{Bytes, Body},
    routing::get,
    Router,
    http::{
        StatusCode,
        header::{HeaderValue, USER_AGENT},
    },
};

struct ValidatedBody(Bytes);

#[async_trait]
impl<S> FromRequest<S> for ValidatedBody
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        // do validation...

        Ok(Self(body))
    }
}

async fn handler(ValidatedBody(body): ValidatedBody) {
    // ...
}

let app = Router::new().route("/foo", get(handler));
```

## <a href="#cannot-implement-both-fromrequest-and-fromrequestparts"
class="doc-anchor">§</a>Cannot implement both `FromRequest` and `FromRequestParts`

Note that you will make your extractor unusable by implementing both
`FromRequest` and `FromRequestParts` directly for the same type, unless
it is wrapping another extractor:

<a href="#" class="tooltip"
title="This example deliberately fails to compile">ⓘ</a>

``` {.rust .rust-example-rendered}
use axum::{
    Router,
    routing::get,
    extract::{FromRequest, Request, FromRequestParts},
    http::request::Parts,
    body::Body,
    async_trait,
};
use std::convert::Infallible;

// Some extractor that doesn't wrap another extractor
struct MyExtractor;

// `MyExtractor` implements both `FromRequest`
#[async_trait]
impl<S> FromRequest<S> for MyExtractor
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // ...
    }
}

// and `FromRequestParts`
#[async_trait]
impl<S> FromRequestParts<S> for MyExtractor
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // ...
    }
}

let app = Router::new().route(
    "/",
    // This fails when we go to actually use `MyExtractor` in a handler function.
    // This is due to a limit in Rust's type system.
    //
    // The workaround is to implement either `FromRequest` or `FromRequestParts`
    // but not both, if your extractor doesn't wrap another extractor.
    //
    // See "Wrapping extractors" for how to wrap other extractors.
    get(|_: MyExtractor| async {}),
);
```

# <a
href="#accessing-other-extractors-in-fromrequest-or-fromrequestparts-implementations"
class="doc-anchor">§</a>Accessing other extractors in `FromRequest` or `FromRequestParts` implementations

When defining custom extractors you often need to access another
extractor in your implementation.

``` {.rust .rust-example-rendered}
use axum::{
    async_trait,
    extract::{Extension, FromRequestParts},
    http::{StatusCode, HeaderMap, request::Parts},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[derive(Clone)]
struct State {
    // ...
}

struct AuthenticatedUser {
    // ...
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // You can either call them directly...
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(|err| match err {})?;

        // ... or use `extract` / `extract_with_state` from `RequestExt` / `RequestPartsExt`
        use axum::RequestPartsExt;
        let Extension(state) = parts.extract::<Extension<State>>()
            .await
            .map_err(|err| err.into_response())?;

        unimplemented!("actually perform the authorization")
    }
}

async fn handler(user: AuthenticatedUser) {
    // ...
}

let state = State { /* ... */ };

let app = Router::new().route("/", get(handler)).layer(Extension(state));
```

# <a href="#request-body-limits" class="doc-anchor">§</a>Request body limits

For security reasons,
[`Bytes`](../../bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")
will, by default, not accept bodies larger than 2MB. This also applies
to extractors that uses
[`Bytes`](../../bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")
internally such as `String`,
[`Json`](../struct.Json.html "struct axum::Json"), and
[`Form`](../struct.Form.html "struct axum::Form").

For more details, including how to disable this limit, see
[`DefaultBodyLimit`](struct.DefaultBodyLimit.html "struct axum::extract::DefaultBodyLimit").

# <a href="#wrapping-extractors" class="doc-anchor">§</a>Wrapping extractors

If you want write an extractor that generically wraps another extractor
(that may or may not consume the request body) you should implement both
[`FromRequest`](trait.FromRequest.html "trait axum::extract::FromRequest")
and
[`FromRequestParts`](trait.FromRequestParts.html "trait axum::extract::FromRequestParts"):

``` {.rust .rust-example-rendered}
use axum::{
    Router,
    body::Body,
    routing::get,
    extract::{Request, FromRequest, FromRequestParts},
    http::{HeaderMap, request::Parts},
    async_trait,
};
use std::time::{Instant, Duration};

// an extractor that wraps another and measures how long time it takes to run
struct Timing<E> {
    extractor: E,
    duration: Duration,
}

// we must implement both `FromRequestParts`
#[async_trait]
impl<S, T> FromRequestParts<S> for Timing<T>
where
    S: Send + Sync,
    T: FromRequestParts<S>,
{
    type Rejection = T::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let start = Instant::now();
        let extractor = T::from_request_parts(parts, state).await?;
        let duration = start.elapsed();
        Ok(Timing {
            extractor,
            duration,
        })
    }
}

// and `FromRequest`
#[async_trait]
impl<S, T> FromRequest<S> for Timing<T>
where
    S: Send + Sync,
    T: FromRequest<S>,
{
    type Rejection = T::Rejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let start = Instant::now();
        let extractor = T::from_request(req, state).await?;
        let duration = start.elapsed();
        Ok(Timing {
            extractor,
            duration,
        })
    }
}

async fn handler(
    // this uses the `FromRequestParts` impl
    _: Timing<HeaderMap>,
    // this uses the `FromRequest` impl
    _: Timing<String>,
) {}
```

# <a href="#logging-rejections" class="doc-anchor">§</a>Logging rejections

All built-in extractors will log rejections for easier debugging. To see
the logs, enable the `tracing` feature for axum (enabled by default) and
the `axum::rejection=trace` tracing target, for example with
`RUST_LOG=info,axum::rejection=trace cargo run`.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::`<a href="../struct.Json.html" class="struct"
title="struct axum::Json"><code>Json</code></a>`;`

`pub use crate::`<a href="../struct.Extension.html" class="struct"
title="struct axum::Extension"><code>Extension</code></a>`;`

`pub use crate::form::`<a href="../struct.Form.html" class="struct"
title="struct axum::Form"><code>Form</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="connect_info/index.html" class="mod"
title="mod axum::extract::connect_info">connect_info</a>  
Extractor for getting connection information from a client.

<a href="path/index.html" class="mod"
title="mod axum::extract::path">path</a>  
Extractor that will get captures from the URL and parse them using
[`serde`](../../serde/index.html "mod serde").

<a href="rejection/index.html" class="mod"
title="mod axum::extract::rejection">rejection</a>  
Rejection response types.

<a href="ws/index.html" class="mod" title="mod axum::extract::ws">ws</a>  
Handle WebSocket connections.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ConnectInfo.html" class="struct"
title="struct axum::extract::ConnectInfo">ConnectInfo</a>  
Extractor for getting connection information produced by a
[`Connected`](connect_info/trait.Connected.html "trait axum::extract::connect_info::Connected").

<a href="struct.DefaultBodyLimit.html" class="struct"
title="struct axum::extract::DefaultBodyLimit">DefaultBodyLimit</a>  
Layer for configuring the default request body limit.

<a href="struct.Host.html" class="struct"
title="struct axum::extract::Host">Host</a>  
Extractor that resolves the hostname of the request.

<a href="struct.MatchedPath.html" class="struct"
title="struct axum::extract::MatchedPath">MatchedPath</a>  
Access the path in the router that matches the request.

<a href="struct.NestedPath.html" class="struct"
title="struct axum::extract::NestedPath">NestedPath</a>  
Access the path the matched the route is nested at.

<a href="struct.OriginalUri.html" class="struct"
title="struct axum::extract::OriginalUri">OriginalUri</a>  
Extractor that gets the original request URI regardless of nesting.

<a href="struct.Path.html" class="struct"
title="struct axum::extract::Path">Path</a>  
Extractor that will get captures from the URL and parse them using
[`serde`](https://crates.io/crates/serde).

<a href="struct.Query.html" class="struct"
title="struct axum::extract::Query">Query</a>  
Extractor that deserializes query strings into some type.

<a href="struct.RawForm.html" class="struct"
title="struct axum::extract::RawForm">RawForm</a>  
Extractor that extracts raw form requests.

<a href="struct.RawPathParams.html" class="struct"
title="struct axum::extract::RawPathParams">RawPathParams</a>  
Extractor that will get captures from the URL without deserializing
them.

<a href="struct.RawQuery.html" class="struct"
title="struct axum::extract::RawQuery">RawQuery</a>  
Extractor that extracts the raw query string, without parsing it.

<a href="struct.State.html" class="struct"
title="struct axum::extract::State">State</a>  
Extractor for state.

<a href="struct.WebSocketUpgrade.html" class="struct"
title="struct axum::extract::WebSocketUpgrade">WebSocketUpgrade</a>  
Extractor for establishing WebSocket connections.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.FromRef.html" class="trait"
title="trait axum::extract::FromRef">FromRef</a>  
Used to do reference-to-value conversions thus not consuming the input
value.

<a href="trait.FromRequest.html" class="trait"
title="trait axum::extract::FromRequest">FromRequest</a>  
Types that can be created from requests.

<a href="trait.FromRequestParts.html" class="trait"
title="trait axum::extract::FromRequestParts">FromRequestParts</a>  
Types that can be created from request parts.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Request.html" class="type"
title="type axum::extract::Request">Request</a>  
Type alias for
[`http::Request`](../../http/request/struct.Request.html "struct http::request::Request")
whose body type defaults to
[`Body`](../body/struct.Body.html "struct axum::body::Body"), the most
common body type used with axum.
