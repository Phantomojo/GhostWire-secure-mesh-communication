[axum](../index.html)

Module error\_handling Copy item path

<span class="sub-heading"><a href="../../src/axum/error_handling/mod.rs.html#1-262"
class="src">Source</a> </span>

Expand description

Error handling model and utilities

# <a href="#axums-error-handling-model" class="doc-anchor">§</a>axum’s error handling model

axum is based on
[`tower::Service`](../../tower_service/trait.Service.html "trait tower_service::Service")
which bundles errors through its associated `Error` type. If you have a
[`Service`](../../tower_service/trait.Service.html "trait tower_service::Service")
that produces an error and that error makes it all the way up to hyper,
the connection will be terminated *without* sending a response. This is
generally not desirable so axum makes sure you always produce a response
by relying on the type system.

axum does this by requiring all services have
[`Infallible`](https://doc.rust-lang.org/1.93.1/core/convert/enum.Infallible.html "enum core::convert::Infallible")
as their error type. `Infallible` is the error type for errors that can
never happen.

This means if you define a handler like:

``` {.rust .rust-example-rendered}
use axum::http::StatusCode;

async fn handler() -> Result<String, StatusCode> {
    // ...
}
```

While it looks like it might fail with a `StatusCode` this actually
isn’t an “error”. If this handler returns `Err(some_status_code)` that
will still be converted into a
[`Response`](../response/type.Response.html "type axum::response::Response")
and sent back to the client. This is done through `StatusCode`’s
[`IntoResponse`](../response/trait.IntoResponse.html "trait axum::response::IntoResponse")
implementation.

It doesn’t matter whether you return `Err(StatusCode::NOT_FOUND)` or
`Err(StatusCode::INTERNAL_SERVER_ERROR)`. These are not considered
errors in axum.

Instead of a direct `StatusCode`, it makes sense to use intermediate
error type that can ultimately be converted to `Response`. This allows
using `?` operator in handlers. See those examples:

-   [`anyhow-error-response`](https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs)
    for generic boxed errors
-   [`error-handling`](https://github.com/tokio-rs/axum/blob/main/examples/error-handling/src/main.rs)
    for application-specific detailed errors

This also applies to extractors. If an extractor doesn’t match the
request the request will be rejected and a response will be returned
without calling your handler. See
[`extract`](../extract/index.html "mod axum::extract") to learn more
about handling extractor failures.

# <a href="#routing-to-fallible-services" class="doc-anchor">§</a>Routing to fallible services

You generally don’t have to think about errors if you’re only using
async functions as handlers. However if you’re embedding general
`Service`s or applying middleware, which might produce errors you have
to tell axum how to convert those errors into responses.

``` {.rust .rust-example-rendered}
use axum::{
    Router,
    body::Body,
    http::{Request, Response, StatusCode},
    error_handling::HandleError,
};

async fn thing_that_might_fail() -> Result<(), anyhow::Error> {
    // ...
}

// this service might fail with `anyhow::Error`
let some_fallible_service = tower::service_fn(|_req| async {
    thing_that_might_fail().await?;
    Ok::<_, anyhow::Error>(Response::new(Body::empty()))
});

let app = Router::new().route_service(
    "/",
    // we cannot route to `some_fallible_service` directly since it might fail.
    // we have to use `handle_error` which converts its errors into responses
    // and changes its error type from `anyhow::Error` to `Infallible`.
    HandleError::new(some_fallible_service, handle_anyhow_error),
);

// handle errors by converting them into something that implements
// `IntoResponse`
async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {err}"),
    )
}
```

# <a href="#applying-fallible-middleware" class="doc-anchor">§</a>Applying fallible middleware

Similarly axum requires you to handle errors from middleware. That is
done with
[`HandleErrorLayer`](struct.HandleErrorLayer.html "struct axum::error_handling::HandleErrorLayer"):

``` {.rust .rust-example-rendered}
use axum::{
    Router,
    BoxError,
    routing::get,
    http::StatusCode,
    error_handling::HandleErrorLayer,
};
use std::time::Duration;
use tower::ServiceBuilder;

let app = Router::new()
    .route("/", get(|| async {}))
    .layer(
        ServiceBuilder::new()
            // `timeout` will produce an error if the handler takes
            // too long so we must handle those
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .timeout(Duration::from_secs(30))
    );

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {err}"),
        )
    }
}
```

# <a href="#running-extractors-for-error-handling"
class="doc-anchor">§</a>Running extractors for error handling

`HandleErrorLayer` also supports running extractors:

``` {.rust .rust-example-rendered}
use axum::{
    Router,
    BoxError,
    routing::get,
    http::{StatusCode, Method, Uri},
    error_handling::HandleErrorLayer,
};
use std::time::Duration;
use tower::ServiceBuilder;

let app = Router::new()
    .route("/", get(|| async {}))
    .layer(
        ServiceBuilder::new()
            // `timeout` will produce an error if the handler takes
            // too long so we must handle those
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .timeout(Duration::from_secs(30))
    );

async fn handle_timeout_error(
    // `Method` and `Uri` are extractors so they can be used here
    method: Method,
    uri: Uri,
    // the last argument must be the error itself
    err: BoxError,
) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("`{method} {uri}` failed with {err}"),
    )
}
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="future/index.html" class="mod"
title="mod axum::error_handling::future">future</a>  
Future types.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.HandleError.html" class="struct"
title="struct axum::error_handling::HandleError">HandleError</a>  
A
[`Service`](../../tower_service/trait.Service.html "trait tower_service::Service")
adapter that handles errors by converting them into responses.

<a href="struct.HandleErrorLayer.html" class="struct"
title="struct axum::error_handling::HandleErrorLayer">HandleErrorLayer</a>  
[`Layer`](../../tower_layer/trait.Layer.html "trait tower_layer::Layer")
that applies
[`HandleError`](struct.HandleError.html "struct axum::error_handling::HandleError")
which is a
[`Service`](../../tower_service/trait.Service.html "trait tower_service::Service")
adapter that handles errors by converting them into responses.
