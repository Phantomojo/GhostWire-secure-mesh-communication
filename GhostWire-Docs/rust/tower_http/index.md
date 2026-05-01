Crate tower\_http Copy item path

<span class="sub-heading"><a href="../src/tower_http/lib.rs.html#1-373" class="src">Source</a>
</span>

Expand description

`async fn(HttpRequest) -> Result<HttpResponse, Error>`

# <a href="#overview" class="doc-anchor">§</a>Overview

tower-http is a library that provides HTTP-specific middleware and
utilities built on top of [tower](https://crates.io/crates/tower).

All middleware uses the [http](https://crates.io/crates/http) and
[http-body](https://crates.io/crates/http-body) crates as the HTTP
abstractions. That means they’re compatible with any library or
framework that also uses those crates, such as
[hyper](https://crates.io/crates/hyper),
[tonic](https://crates.io/crates/tonic), and
[warp](https://crates.io/crates/warp).

# <a href="#example-server" class="doc-anchor">§</a>Example server

This example shows how to apply middleware from tower-http to a
[`Service`](https://docs.rs/tower/latest/tower/trait.Service.html) and
then run that service using [hyper](https://crates.io/crates/hyper).

``` {.rust .rust-example-rendered}
use tower_http::{
    add_extension::AddExtensionLayer,
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
};
use tower::{ServiceBuilder, service_fn, BoxError};
use http::{Request, Response, header::{HeaderName, CONTENT_TYPE, AUTHORIZATION}};
use std::{sync::Arc, net::SocketAddr, convert::Infallible, iter::once};
use bytes::Bytes;
use http_body_util::Full;

// Our request handler. This is where we would implement the application logic
// for responding to HTTP requests...
async fn handler(request: Request<Full<Bytes>>) -> Result<Response<Full<Bytes>>, BoxError> {
    // ...
}

// Shared state across all request handlers --- in this case, a pool of database connections.
struct State {
    pool: DatabaseConnectionPool,
}

#[tokio::main]
async fn main() {
    // Construct the shared state.
    let state = State {
        pool: DatabaseConnectionPool::new(),
    };

    // Use tower's `ServiceBuilder` API to build a stack of tower middleware
    // wrapping our request handler.
    let service = ServiceBuilder::new()
        // Mark the `Authorization` request header as sensitive so it doesn't show in logs
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        // High level logging of requests and responses
        .layer(TraceLayer::new_for_http())
        // Share an `Arc<State>` with all requests
        .layer(AddExtensionLayer::new(Arc::new(state)))
        // Compress responses
        .layer(CompressionLayer::new())
        // Propagate `X-Request-Id`s from requests to responses
        .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
        // If the response has a known size set the `Content-Length` header
        .layer(SetResponseHeaderLayer::overriding(CONTENT_TYPE, content_length_from_response))
        // Authorize requests using a token
        .layer(ValidateRequestHeaderLayer::bearer("passwordlol"))
        // Accept only application/json, application/* and */* in a request's ACCEPT header
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        // Wrap a `Service` in our middleware stack
        .service_fn(handler);
}
```

Keep in mind that while this example uses
[hyper](https://crates.io/crates/hyper), tower-http supports any HTTP
client/server implementation that uses the
[http](https://crates.io/crates/http) and
[http-body](https://crates.io/crates/http-body) crates.

# <a href="#example-client" class="doc-anchor">§</a>Example client

tower-http middleware can also be applied to HTTP clients:

``` {.rust .rust-example-rendered}
use tower_http::{
    decompression::DecompressionLayer,
    set_header::SetRequestHeaderLayer,
    trace::TraceLayer,
    classify::StatusInRangeAsFailures,
};
use tower::{ServiceBuilder, Service, ServiceExt};
use hyper_util::{rt::TokioExecutor, client::legacy::Client};
use http_body_util::Full;
use bytes::Bytes;
use http::{Request, HeaderValue, header::USER_AGENT};

#[tokio::main]
async fn main() {
let client = Client::builder(TokioExecutor::new()).build_http();
    let mut client = ServiceBuilder::new()
        // Add tracing and consider server errors and client
        // errors as failures.
        .layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier()
        ))
        // Set a `User-Agent` header on all requests.
        .layer(SetRequestHeaderLayer::overriding(
            USER_AGENT,
            HeaderValue::from_static("tower-http demo")
        ))
        // Decompress response bodies
        .layer(DecompressionLayer::new())
        // Wrap a `Client` in our middleware stack.
        // This is possible because `Client` implements
        // `tower::Service`.
        .service(client);

    // Make a request
    let request = Request::builder()
        .uri("http://example.com")
        .body(Full::<Bytes>::default())
        .unwrap();

    let response = client
        .ready()
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
}
```

# <a href="#feature-flags" class="doc-anchor">§</a>Feature Flags

All middleware are disabled by default and can be enabled using [cargo
features](https://doc.rust-lang.org/cargo/reference/features.html).

For example, to enable the [`Trace`](crate::trace::Trace) middleware,
add the “trace” feature flag in your `Cargo.toml`:

``` language-toml
tower-http = { version = "0.1", features = ["trace"] }
```

You can use `"full"` to enable everything:

``` language-toml
tower-http = { version = "0.1", features = ["full"] }
```

# <a href="#getting-help" class="doc-anchor">§</a>Getting Help

If you’re new to tower its
[guides](https://github.com/tower-rs/tower/tree/master/guides) might
help. In the tower-http repo we also have a [number of
examples](https://github.com/tower-rs/tower-http/tree/master/examples)
showing how to put everything together. You’re also welcome to ask in
the [`#tower` Discord channel](https://discord.gg/tokio) or open an
[issue](https://github.com/tower-rs/tower-http/issues/new) with your
question.

# Modules<a href="#modules" class="anchor">§</a>

<a href="body/index.html" class="mod"
title="mod tower_http::body">body</a>  
Body types.

<a href="classify/index.html" class="mod"
title="mod tower_http::classify">classify</a>  
Tools for classifying responses as either success or failure.

<a href="decompression/index.html" class="mod"
title="mod tower_http::decompression">decompression</a>  
Middleware that decompresses request and response bodies.

<a href="follow_redirect/index.html" class="mod"
title="mod tower_http::follow_redirect">follow_redirect</a>  
Middleware for following redirections.

<a href="services/index.html" class="mod"
title="mod tower_http::services">services</a>  
[`Service`](https://docs.rs/tower/latest/tower/trait.Service.html)s that
return responses without wrapping other
[`Service`](https://docs.rs/tower/latest/tower/trait.Service.html)s.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.CompressionLevel.html" class="enum"
title="enum tower_http::CompressionLevel">CompressionLevel</a>  
Level of compression data should be compressed with.

<a href="enum.LatencyUnit.html" class="enum"
title="enum tower_http::LatencyUnit">LatencyUnit</a>  
The latency unit used to report latencies by middleware.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BoxError.html" class="type"
title="type tower_http::BoxError">BoxError</a>  
Alias for a type-erased error type.
