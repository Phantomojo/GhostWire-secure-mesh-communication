[axum](../index.html)

Module handler Copy item path

<span class="sub-heading"><a href="../../src/axum/handler/mod.rs.html#1-430"
class="src">Source</a> </span>

Expand description

Async functions that can be used to handle requests.

In axum a “handler” is an async function that accepts zero or more
[“extractors”](../extract/index.html "mod axum::extract") as arguments
and returns something that can be converted [into a
response](../response/index.html "mod axum::response").

Handlers are where your application logic lives and axum applications
are built by routing between handlers.

Some examples of handlers:

``` {.rust .rust-example-rendered}
use axum::{body::Bytes, http::StatusCode};

// Handler that immediately returns an empty `200 OK` response.
async fn unit_handler() {}

// Handler that immediately returns a `200 OK` response with a plain text
// body.
async fn string_handler() -> String {
    "Hello, World!".to_string()
}

// Handler that buffers the request body and returns it.
//
// This works because `Bytes` implements `FromRequest`
// and therefore can be used as an extractor.
//
// `String` and `StatusCode` both implement `IntoResponse` and
// therefore `Result<String, StatusCode>` also implements `IntoResponse`
async fn echo(body: Bytes) -> Result<String, StatusCode> {
    if let Ok(string) = String::from_utf8(body.to_vec()) {
        Ok(string)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
```

Instead of a direct `StatusCode`, it makes sense to use intermediate
error type that can ultimately be converted to `Response`. This allows
using `?` operator in handlers. See those examples:

-   [`anyhow-error-response`](https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs)
    for generic boxed errors
-   [`error-handling`](https://github.com/tokio-rs/axum/blob/main/examples/error-handling/src/main.rs)
    for application-specific detailed errors

## <a href="#debugging-handler-type-errors" class="doc-anchor">§</a>Debugging handler type errors

For a function to be used as a handler it must implement the
[`Handler`](trait.Handler.html "trait axum::handler::Handler") trait.
axum provides blanket implementations for functions that:

-   Are `async fn`s.
-   Take no more than 16 arguments that all implement `Send`.
    -   All except the last argument implement
        [`FromRequestParts`](../extract/trait.FromRequestParts.html "trait axum::extract::FromRequestParts").
    -   The last argument implements
        [`FromRequest`](../extract/trait.FromRequest.html "trait axum::extract::FromRequest").
-   Returns something that implements
    [`IntoResponse`](../response/trait.IntoResponse.html "trait axum::response::IntoResponse").
-   If a closure is used it must implement `Clone + Send` and be
    `'static`.
-   Returns a future that is `Send`. The most common way to accidentally
    make a future `!Send` is to hold a `!Send` type across an await.

Unfortunately Rust gives poor error messages if you try to use a
function that doesn’t quite match what’s required by
[`Handler`](trait.Handler.html "trait axum::handler::Handler").

You might get an error like this:

``` language-not_rust
error[E0277]: the trait bound `fn(bool) -> impl Future {handler}: Handler<_, _>` is not satisfied
   --> src/main.rs:13:44
    |
13  |     let app = Router::new().route("/", get(handler));
    |                                            ^^^^^^^ the trait `Handler<_, _>` is not implemented for `fn(bool) -> impl Future {handler}`
    |
   ::: axum/src/handler/mod.rs:116:8
    |
116 |     H: Handler<T, B>,
    |        ------------- required by this bound in `axum::routing::get`
```

This error doesn’t tell you *why* your function doesn’t implement
[`Handler`](trait.Handler.html "trait axum::handler::Handler"). It’s
possible to improve the error with the
[`debug_handler`](https://docs.rs/axum-macros/latest/axum_macros/attr.debug_handler.html)
proc-macro from the [axum-macros](https://docs.rs/axum-macros) crate.

# Modules<a href="#modules" class="anchor">§</a>

<a href="future/index.html" class="mod"
title="mod axum::handler::future">future</a>  
Handler future types.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.HandlerService.html" class="struct"
title="struct axum::handler::HandlerService">HandlerService</a>  
An adapter that makes a
[`Handler`](trait.Handler.html "trait axum::handler::Handler") into a
[`Service`](../../tower_service/trait.Service.html "trait tower_service::Service").

<a href="struct.Layered.html" class="struct"
title="struct axum::handler::Layered">Layered</a>  
A
[`Service`](../../tower_service/trait.Service.html "trait tower_service::Service")
created from a
[`Handler`](trait.Handler.html "trait axum::handler::Handler") by
applying a Tower middleware.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Handler.html" class="trait"
title="trait axum::handler::Handler">Handler</a>  
Trait for async functions that can be used to handle requests.

<a href="trait.HandlerWithoutStateExt.html" class="trait"
title="trait axum::handler::HandlerWithoutStateExt">HandlerWithoutStateExt</a>  
Extension trait for
[`Handler`](trait.Handler.html "trait axum::handler::Handler")s that
don’t have state.
