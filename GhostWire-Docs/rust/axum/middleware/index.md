[axum](../index.html)

Module middleware Copy item path

<span class="sub-heading"><a href="../../src/axum/middleware/mod.rs.html#1-29"
class="src">Source</a> </span>

Expand description

Utilities for writing middleware

# <a href="#intro" class="doc-anchor">§</a>Intro

axum is unique in that it doesn’t have its own bespoke middleware system
and instead integrates with [`tower`](https://crates.io/crates/tower).
This means the ecosystem of [`tower`](https://crates.io/crates/tower)
and [`tower-http`](https://crates.io/crates/tower-http) middleware all
work with axum.

While it’s not necessary to fully understand tower to write or use
middleware with axum, having at least a basic understanding of tower’s
concepts is recommended. See [tower’s
guides](https://github.com/tower-rs/tower/tree/master/guides) for a
general introduction. Reading the documentation for
[`tower::ServiceBuilder`](../../tower/builder/struct.ServiceBuilder.html "struct tower::builder::ServiceBuilder")
is also recommended.

# <a href="#applying-middleware" class="doc-anchor">§</a>Applying middleware

axum allows you to add middleware just about anywhere

-   To entire routers with
    [`Router::layer`](../struct.Router.html#method.layer "method axum::Router::layer")
    and
    [`Router::route_layer`](../struct.Router.html#method.route_layer "method axum::Router::route_layer").
-   To method routers with
    [`MethodRouter::layer`](../routing/method_routing/struct.MethodRouter.html#method.layer "method axum::routing::method_routing::MethodRouter::layer")
    and
    [`MethodRouter::route_layer`](../routing/method_routing/struct.MethodRouter.html#method.route_layer "method axum::routing::method_routing::MethodRouter::route_layer").
-   To individual handlers with
    [`Handler::layer`](../handler/trait.Handler.html#method.layer "method axum::handler::Handler::layer").

## <a href="#applying-multiple-middleware" class="doc-anchor">§</a>Applying multiple middleware

It’s recommended to use
[`tower::ServiceBuilder`](../../tower/builder/struct.ServiceBuilder.html "struct tower::builder::ServiceBuilder")
to apply multiple middleware at once, instead of calling `layer` (or
`route_layer`) repeatedly:

``` {.rust .rust-example-rendered}
use axum::{
    routing::get,
    Extension,
    Router,
};
use tower_http::{trace::TraceLayer};
use tower::ServiceBuilder;

async fn handler() {}

#[derive(Clone)]
struct State {}

let app = Router::new()
    .route("/", get(handler))
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(Extension(State {}))
    );
```

# <a href="#commonly-used-middleware" class="doc-anchor">§</a>Commonly used middleware

Some commonly used middleware are:

-   [`TraceLayer`](tower_http::trace) for high level tracing/logging.
-   [`CorsLayer`](tower_http::cors) for handling CORS.
-   [`CompressionLayer`](tower_http::compression) for automatic
    compression of responses.
-   [`RequestIdLayer`](tower_http::request_id) and
    [`PropagateRequestIdLayer`](tower_http::request_id) set and
    propagate request ids.
-   [`TimeoutLayer`](tower_http::timeout::TimeoutLayer) for timeouts.

# <a href="#ordering" class="doc-anchor">§</a>Ordering

When you add middleware with
[`Router::layer`](../struct.Router.html#method.layer "method axum::Router::layer")
(or similar) all previously added routes will be wrapped in the
middleware. Generally speaking, this results in middleware being
executed from bottom to top.

So if you do this:

``` {.rust .rust-example-rendered}
use axum::{routing::get, Router};

async fn handler() {}

let app = Router::new()
    .route("/", get(handler))
    .layer(layer_one)
    .layer(layer_two)
    .layer(layer_three);
```

Think of the middleware as being layered like an onion where each new
layer wraps all previous layers:

``` language-not_rust
        requests
           |
           v
+----- layer_three -----+
| +---- layer_two ----+ |
| | +-- layer_one --+ | |
| | |               | | |
| | |    handler    | | |
| | |               | | |
| | +-- layer_one --+ | |
| +---- layer_two ----+ |
+----- layer_three -----+
           |
           v
        responses
```

That is:

-   First `layer_three` receives the request
-   It then does its thing and passes the request onto `layer_two`
-   Which passes the request onto `layer_one`
-   Which passes the request onto `handler` where a response is produced
-   That response is then passed to `layer_one`
-   Then to `layer_two`
-   And finally to `layer_three` where it’s returned out of your app

It’s a little more complicated in practice because any middleware is
free to return early and not call the next layer, for example if a
request cannot be authorized, but it’s a useful mental model to have.

As previously mentioned it’s recommended to add multiple middleware
using `tower::ServiceBuilder`, however this impacts ordering:

``` {.rust .rust-example-rendered}
use tower::ServiceBuilder;
use axum::{routing::get, Router};

async fn handler() {}

let app = Router::new()
    .route("/", get(handler))
    .layer(
        ServiceBuilder::new()
            .layer(layer_one)
            .layer(layer_two)
            .layer(layer_three),
    );
```

`ServiceBuilder` works by composing all layers into one such that they
run top to bottom. So with the previous code `layer_one` would receive
the request first, then `layer_two`, then `layer_three`, then `handler`,
and then the response would bubble back up through `layer_three`, then
`layer_two`, and finally `layer_one`.

Executing middleware top to bottom is generally easier to understand and
follow mentally which is one of the reasons `ServiceBuilder` is
recommended.

# <a href="#writing-middleware" class="doc-anchor">§</a>Writing middleware

axum offers many ways of writing middleware, at different levels of
abstraction and with different pros and cons.

## <a href="#axummiddlewarefrom_fn" class="doc-anchor">§</a>`axum::middleware::from_fn`

Use
[`axum::middleware::from_fn`](fn.from_fn.html "fn axum::middleware::from_fn")
to write your middleware when:

-   You’re not comfortable with implementing your own futures and would
    rather use the familiar `async`/`await` syntax.
-   You don’t intend to publish your middleware as a crate for others to
    use. Middleware written like this are only compatible with axum.

## <a href="#axummiddlewarefrom_extractor" class="doc-anchor">§</a>`axum::middleware::from_extractor`

Use
[`axum::middleware::from_extractor`](fn.from_extractor.html "fn axum::middleware::from_extractor")
to write your middleware when:

-   You have a type that you sometimes want to use as an extractor and
    sometimes as a middleware. If you only need your type as a
    middleware prefer
    [`middleware::from_fn`](fn.from_fn.html "fn axum::middleware::from_fn").

## <a href="#towers-combinators" class="doc-anchor">§</a>tower’s combinators

tower has several utility combinators that can be used to perform simple
modifications to requests or responses. The most commonly used ones are

-   [`ServiceBuilder::map_request`](../../tower/builder/struct.ServiceBuilder.html#method.map_request "method tower::builder::ServiceBuilder::map_request")
-   [`ServiceBuilder::map_response`](../../tower/builder/struct.ServiceBuilder.html#method.map_response "method tower::builder::ServiceBuilder::map_response")
-   [`ServiceBuilder::then`](../../tower/builder/struct.ServiceBuilder.html#method.then "method tower::builder::ServiceBuilder::then")
-   [`ServiceBuilder::and_then`](../../tower/builder/struct.ServiceBuilder.html#method.and_then "method tower::builder::ServiceBuilder::and_then")

You should use these when

-   You want to perform a small ad hoc operation, such as adding a
    header.
-   You don’t intend to publish your middleware as a crate for others to
    use.

## <a href="#towerservice-and-pinboxdyn-future" class="doc-anchor">§</a>`tower::Service` and `Pin<Box<dyn Future>>`

For maximum control (and a more low level API) you can write your own
middleware by implementing
[`tower::Service`](../../tower_service/trait.Service.html "trait tower_service::Service"):

Use
[`tower::Service`](../../tower_service/trait.Service.html "trait tower_service::Service")
with `Pin<Box<dyn Future>>` to write your middleware when:

-   Your middleware needs to be configurable for example via builder
    methods on your
    [`tower::Layer`](../../tower_layer/trait.Layer.html "trait tower_layer::Layer")
    such as \[`tower_http::trace::TraceLayer`\].
-   You do intend to publish your middleware as a crate for others to
    use.
-   You’re not comfortable with implementing your own futures.

A decent template for such a middleware could be:

``` {.rust .rust-example-rendered}
use axum::{
    response::Response,
    body::Body,
    extract::Request,
};
use futures_util::future::BoxFuture;
use tower::{Service, Layer};
use std::task::{Context, Poll};

#[derive(Clone)]
struct MyLayer;

impl<S> Layer<S> for MyLayer {
    type Service = MyMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MyMiddleware { inner }
    }
}

#[derive(Clone)]
struct MyMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for MyMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            Ok(response)
        })
    }
}
```

Note that your error type being defined as `S::Error` means that your
middleware typically *returns no errors*. As a principle always try to
return a response and try not to bail out with a custom error type. For
example, if a 3rd party library you are using inside your new middleware
returns its own specialized error type, try to convert it to some
reasonable response and return `Ok` with that response.

If you choose to implement a custom error type such as
`type Error = BoxError` (a boxed opaque error), or any other error type
that is not `Infallible`, you must use a `HandleErrorLayer`, here is an
example using a `ServiceBuilder`:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            // because Axum uses infallible errors, you must handle your custom error type from your middleware here
            StatusCode::BAD_REQUEST
        }))
        .layer(
             // <your actual layer which DOES return an error>
        );
```

## <a href="#towerservice-and-custom-futures" class="doc-anchor">§</a>`tower::Service` and custom futures

If you’re comfortable implementing your own futures (or want to learn
it) and need as much control as possible then using `tower::Service`
without boxed futures is the way to go.

Use
[`tower::Service`](../../tower_service/trait.Service.html "trait tower_service::Service")
with manual futures to write your middleware when:

-   You want your middleware to have the lowest possible overhead.
-   Your middleware needs to be configurable for example via builder
    methods on your
    [`tower::Layer`](../../tower_layer/trait.Layer.html "trait tower_layer::Layer")
    such as \[`tower_http::trace::TraceLayer`\].
-   You do intend to publish your middleware as a crate for others to
    use, perhaps as part of tower-http.
-   You’re comfortable with implementing your own futures, or want to
    learn how the lower levels of async Rust works.

tower’s [“Building a middleware from
scratch”](https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md)
guide is a good place to learn how to do this.

# <a href="#error-handling-for-middleware" class="doc-anchor">§</a>Error handling for middleware

axum’s error handling model requires handlers to always return a
response. However middleware is one possible way to introduce errors
into an application. If hyper receives an error the connection will be
closed without sending a response. Thus axum requires those errors to be
handled gracefully:

``` {.rust .rust-example-rendered}
use axum::{
    routing::get,
    error_handling::HandleErrorLayer,
    http::StatusCode,
    BoxError,
    Router,
};
use tower::{ServiceBuilder, timeout::TimeoutLayer};
use std::time::Duration;

async fn handler() {}

let app = Router::new()
    .route("/", get(handler))
    .layer(
        ServiceBuilder::new()
            // this middleware goes above `TimeoutLayer` because it will receive
            // errors returned by `TimeoutLayer`
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::REQUEST_TIMEOUT
            }))
            .layer(TimeoutLayer::new(Duration::from_secs(10)))
    );
```

See
[`error_handling`](../error_handling/index.html "mod axum::error_handling")
for more details on axum’s error handling model.

# <a href="#routing-to-servicesmiddleware-and-backpressure"
class="doc-anchor">§</a>Routing to services/middleware and backpressure

Generally routing to one of multiple services and backpressure doesn’t
mix well. Ideally you would want ensure a service is ready to receive a
request before calling it. However, in order to know which service to
call, you need the request…

One approach is to not consider the router service itself ready until
all destination services are ready. That is the approach used by
\[`tower::steer::Steer`\].

Another approach is to always consider all services ready (always return
`Poll::Ready(Ok(()))`) from `Service::poll_ready` and then actually
drive readiness inside the response future returned by `Service::call`.
This works well when your services don’t care about backpressure and are
always ready anyway.

axum expects that all services used in your app won’t care about
backpressure and so it uses the latter strategy. However that means you
should avoid routing to a service (or using a middleware) that *does*
care about backpressure. At the very least you should \[load
shed\]\[tower::load\_shed\] so requests are dropped quickly and don’t
keep piling up.

It also means that if `poll_ready` returns an error then that error will
be returned in the response future from `call` and *not* from
`poll_ready`. In that case, the underlying service will *not* be
discarded and will continue to be used for future requests. Services
that expect to be discarded if `poll_ready` fails should *not* be used
with axum.

One possible approach is to only apply backpressure sensitive middleware
around your entire app. This is possible because axum applications are
themselves services:

``` {.rust .rust-example-rendered}
use axum::{
    routing::get,
    Router,
};
use tower::ServiceBuilder;

async fn handler() { /* ... */ }

let app = Router::new().route("/", get(handler));

let app = ServiceBuilder::new()
    .layer(some_backpressure_sensitive_middleware)
    .service(app);
```

However when applying middleware around your whole application in this
way you have to take care that errors are still being handled
appropriately.

Also note that handlers created from async functions don’t care about
backpressure and are always ready. So if you’re not using any Tower
middleware you don’t have to worry about any of this.

# <a href="#accessing-state-in-middleware" class="doc-anchor">§</a>Accessing state in middleware

How to make state available to middleware depends on how the middleware
is written.

## <a href="#accessing-state-in-axummiddlewarefrom_fn"
class="doc-anchor">§</a>Accessing state in `axum::middleware::from_fn`

Use
[`axum::middleware::from_fn_with_state`](fn.from_fn_with_state.html "fn axum::middleware::from_fn_with_state").

## <a href="#accessing-state-in-custom-towerlayers"
class="doc-anchor">§</a>Accessing state in custom `tower::Layer`s

``` {.rust .rust-example-rendered}
use axum::{
    Router,
    routing::get,
    middleware::{self, Next},
    response::Response,
    extract::{State, Request},
};
use tower::{Layer, Service};
use std::task::{Context, Poll};

#[derive(Clone)]
struct AppState {}

#[derive(Clone)]
struct MyLayer {
    state: AppState,
}

impl<S> Layer<S> for MyLayer {
    type Service = MyService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MyService {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
struct MyService<S> {
    inner: S,
    state: AppState,
}

impl<S, B> Service<Request<B>> for MyService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        // Do something with `self.state`.
        //
        // See `axum::RequestExt` for how to run extractors directly from
        // a `Request`.

        self.inner.call(req)
    }
}

async fn handler(_: State<AppState>) {}

let state = AppState {};

let app = Router::new()
    .route("/", get(handler))
    .layer(MyLayer { state: state.clone() })
    .with_state(state);
```

# <a href="#passing-state-from-middleware-to-handlers"
class="doc-anchor">§</a>Passing state from middleware to handlers

State can be passed from middleware to handlers using [request
extensions](https://docs.rs/http/latest/http/request/struct.Request.html#method.extensions):

``` {.rust .rust-example-rendered}
use axum::{
    Router,
    http::StatusCode,
    routing::get,
    response::{IntoResponse, Response},
    middleware::{self, Next},
    extract::{Request, Extension},
};

#[derive(Clone)]
struct CurrentUser { /* ... */ }

async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(auth_header).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<CurrentUser> {
    // ...
}

async fn handler(
    // extract the current user, set by the middleware
    Extension(current_user): Extension<CurrentUser>,
) {
    // ...
}

let app = Router::new()
    .route("/", get(handler))
    .route_layer(middleware::from_fn(auth));
```

[Response
extensions](https://docs.rs/http/latest/http/response/struct.Response.html#method.extensions)
can also be used but note that request extensions are not automatically
moved to response extensions. You need to manually do that for the
extensions you need.

# <a href="#rewriting-request-uri-in-middleware" class="doc-anchor">§</a>Rewriting request URI in middleware

Middleware added with
[`Router::layer`](../struct.Router.html#method.layer "method axum::Router::layer")
will run after routing. That means it cannot be used to run middleware
that rewrites the request URI. By the time the middleware runs the
routing is already done.

The workaround is to wrap the middleware around the entire `Router`
(this works because `Router` implements
[`Service`](../../tower_service/trait.Service.html "trait tower_service::Service")):

``` {.rust .rust-example-rendered}
use tower::Layer;
use axum::{
    Router,
    ServiceExt, // for `into_make_service`
    response::Response,
    middleware::Next,
    extract::Request,
};

fn rewrite_request_uri<B>(req: Request<B>) -> Request<B> {
    // ...
}

// this can be any `tower::Layer`
let middleware = tower::util::MapRequestLayer::new(rewrite_request_uri);

let app = Router::new();

// apply the layer around the whole `Router`
// this way the middleware will run before `Router` receives the request
let app_with_middleware = middleware.layer(app);

let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app_with_middleware.into_make_service()).await.unwrap();
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="future/index.html" class="mod"
title="mod axum::middleware::future">future</a>  
Future types.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AddExtension.html" class="struct"
title="struct axum::middleware::AddExtension">AddExtension</a>  
Middleware for adding some shareable value to [request
extensions](https://docs.rs/http/latest/http/struct.Extensions.html).

<a href="struct.FromExtractor.html" class="struct"
title="struct axum::middleware::FromExtractor">FromExtractor</a>  
Middleware that runs an extractor and discards the value.

<a href="struct.FromExtractorLayer.html" class="struct"
title="struct axum::middleware::FromExtractorLayer">FromExtractorLayer</a>  
[`Layer`](../../tower_layer/trait.Layer.html "trait tower_layer::Layer")
that applies
[`FromExtractor`](struct.FromExtractor.html "struct axum::middleware::FromExtractor")
that runs an extractor and discards the value.

<a href="struct.FromFn.html" class="struct"
title="struct axum::middleware::FromFn">FromFn</a>  
A middleware created from an async function.

<a href="struct.FromFnLayer.html" class="struct"
title="struct axum::middleware::FromFnLayer">FromFnLayer</a>  
A
[`tower::Layer`](../../tower_layer/trait.Layer.html "trait tower_layer::Layer")
from an async function.

<a href="struct.MapRequest.html" class="struct"
title="struct axum::middleware::MapRequest">MapRequest</a>  
A middleware created from an async function that transforms a request.

<a href="struct.MapRequestLayer.html" class="struct"
title="struct axum::middleware::MapRequestLayer">MapRequestLayer</a>  
A
[`tower::Layer`](../../tower_layer/trait.Layer.html "trait tower_layer::Layer")
from an async function that transforms a request.

<a href="struct.MapResponse.html" class="struct"
title="struct axum::middleware::MapResponse">MapResponse</a>  
A middleware created from an async function that transforms a response.

<a href="struct.MapResponseLayer.html" class="struct"
title="struct axum::middleware::MapResponseLayer">MapResponseLayer</a>  
A
[`tower::Layer`](../../tower_layer/trait.Layer.html "trait tower_layer::Layer")
from an async function that transforms a response.

<a href="struct.Next.html" class="struct"
title="struct axum::middleware::Next">Next</a>  
The remainder of a middleware stack, including the handler.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.IntoMapRequestResult.html" class="trait"
title="trait axum::middleware::IntoMapRequestResult">IntoMapRequestResult</a>  
Trait implemented by types that can be returned from
[`map_request`](fn.map_request.html "fn axum::middleware::map_request"),
[`map_request_with_state`](fn.map_request_with_state.html "fn axum::middleware::map_request_with_state").

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.from_extractor.html" class="fn"
title="fn axum::middleware::from_extractor">from_extractor</a>  
Create a middleware from an extractor.

<a href="fn.from_extractor_with_state.html" class="fn"
title="fn axum::middleware::from_extractor_with_state">from_extractor_with_state</a>  
Create a middleware from an extractor with the given state.

<a href="fn.from_fn.html" class="fn"
title="fn axum::middleware::from_fn">from_fn</a>  
Create a middleware from an async function.

<a href="fn.from_fn_with_state.html" class="fn"
title="fn axum::middleware::from_fn_with_state">from_fn_with_state</a>  
Create a middleware from an async function with the given state.

<a href="fn.map_request.html" class="fn"
title="fn axum::middleware::map_request">map_request</a>  
Create a middleware from an async function that transforms a request.

<a href="fn.map_request_with_state.html" class="fn"
title="fn axum::middleware::map_request_with_state">map_request_with_state</a>  
Create a middleware from an async function that transforms a request,
with the given state.

<a href="fn.map_response.html" class="fn"
title="fn axum::middleware::map_response">map_response</a>  
Create a middleware from an async function that transforms a response.

<a href="fn.map_response_with_state.html" class="fn"
title="fn axum::middleware::map_response_with_state">map_response_with_state</a>  
Create a middleware from an async function that transforms a response,
with the given state.
