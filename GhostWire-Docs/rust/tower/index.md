Crate tower Copy item path

<span class="sub-heading"><a href="../src/tower/lib.rs.html#1-228" class="src">Source</a>
</span>

Expand description

`async fn(Request) -> Result<Response, Error>`

# <a href="#overview" class="doc-anchor">§</a>Overview

Tower is a library of modular and reusable components for building
robust networking clients and servers.

Tower provides a simple core abstraction, the
[`Service`](trait.Service.html "trait tower::Service") trait, which
represents an asynchronous function taking a request and returning
either a response or an error. This abstraction can be used to model
both clients and servers.

Generic components, like
[`timeout`](timeout/index.html "mod tower::timeout"), [rate
limiting](crate::limit::rate), and [load balancing](crate::balance), can
be modeled as [`Service`](trait.Service.html "trait tower::Service")s
that wrap some inner service and apply additional behavior before or
after the inner service is called. This allows implementing these
components in a protocol-agnostic, composable way. Typically, such
services are referred to as *middleware*.

An additional abstraction, the
[`Layer`](trait.Layer.html "trait tower::Layer") trait, is used to
compose middleware with
[`Service`](trait.Service.html "trait tower::Service")s. If a
[`Service`](trait.Service.html "trait tower::Service") can be thought of
as an asynchronous function from a request type to a response type, a
[`Layer`](trait.Layer.html "trait tower::Layer") is a function taking a
[`Service`](trait.Service.html "trait tower::Service") of one type and
returning a [`Service`](trait.Service.html "trait tower::Service") of a
different type. The
[`ServiceBuilder`](struct.ServiceBuilder.html "struct tower::ServiceBuilder")
type is used to add middleware to a service by composing it with
multiple [`Layer`](trait.Layer.html "trait tower::Layer")s.

## <a href="#the-tower-ecosystem" class="doc-anchor">§</a>The Tower Ecosystem

Tower is made up of the following crates:

-   [`tower`](https://crates.io/crates/tower) (this crate)
-   [`tower-service`](https://crates.io/crates/tower-service)
-   [`tower-layer`](https://crates.io/crates/tower-layer)
-   [`tower-test`](https://crates.io/crates/tower-test)

Since the [`Service`](trait.Service.html "trait tower::Service") and
[`Layer`](trait.Layer.html "trait tower::Layer") traits are important
integration points for all libraries using Tower, they are kept as
stable as possible, and breaking changes are made rarely. Therefore,
they are defined in separate crates,
[`tower-service`](https://crates.io/crates/tower-service) and
[`tower-layer`](https://crates.io/crates/tower-layer). This crate
contains re-exports of those core traits, implementations of
commonly-used middleware, and
[utilities](trait.ServiceExt.html "trait tower::ServiceExt") for working
with [`Service`](trait.Service.html "trait tower::Service")s and
[`Layer`](trait.Layer.html "trait tower::Layer")s. Finally, the
[`tower-test`](https://crates.io/crates/tower-test) crate provides tools
for testing programs using Tower.

# <a href="#usage" class="doc-anchor">§</a>Usage

Tower provides an abstraction layer, and generic implementations of
various middleware. This means that the `tower` crate on its own does
*not* provide a working implementation of a network client or server.
Instead, Tower’s [`Service`
trait](trait.Service.html "trait tower::Service") provides an
integration point between application code, libraries providing
middleware implementations, and libraries that implement servers and/or
clients for various network protocols.

Depending on your particular use case, you might use Tower in several
ways:

-   **Implementing application logic** for a networked program. You
    might use the [`Service`](trait.Service.html "trait tower::Service")
    trait to model your application’s behavior, and use the middleware
    [provided by this crate](#modules) and by other libraries to add
    functionality to clients and servers provided by one or more
    protocol implementations.

-   **Implementing middleware** to add custom behavior to network
    clients and servers in a reusable manner. This might be
    general-purpose middleware (and if it is, please consider releasing
    your middleware as a library for other Tower users!) or
    application-specific behavior that needs to be shared between
    multiple clients or servers.

-   **Implementing a network protocol**. Libraries that implement
    network protocols (such as HTTP) can depend on `tower-service` to
    use the [`Service`](trait.Service.html "trait tower::Service") trait
    as an integration point between the protocol and user code. For
    example, a client for some protocol might implement
    [`Service`](trait.Service.html "trait tower::Service"), allowing
    users to add arbitrary Tower middleware to those clients. Similarly,
    a server might be created from a user-provided
    [`Service`](trait.Service.html "trait tower::Service").

    Additionally, when a network protocol requires functionality already
    provided by existing Tower middleware, a protocol implementation
    might use Tower middleware internally, as well as as an integration
    point.

## <a href="#library-support" class="doc-anchor">§</a>Library Support

A number of third-party libraries support Tower and the
[`Service`](trait.Service.html "trait tower::Service") trait. The
following is an incomplete list of such libraries:

-   [`hyper`](https://crates.io/crates/hyper): A fast and correct
    low-level HTTP implementation.
-   [`tonic`](https://crates.io/crates/tonic): A
    [gRPC-over-HTTP/2](https://grpc.io) implementation built on top of
    [`hyper`](https://crates.io/crates/hyper). See
    [here](https://github.com/hyperium/tonic/tree/master/examples/src/tower)
    for examples of using [`tonic`](https://crates.io/crates/tonic) with
    Tower.
-   [`warp`](https://crates.io/crates/warp): A lightweight, composable
    web framework. See
    [here](https://docs.rs/warp/0.2.5/warp/fn.service.html) for details
    on using [`warp`](https://crates.io/crates/warp) with Tower.
-   [`tower-lsp`](https://crates.io/crates/tower-lsp): implementations
    of the [Language Server
    Protocol](https://microsoft.github.io/language-server-protocol/)
    based on Tower.

If you’re the maintainer of a crate that supports Tower, we’d love to
add your crate to this list! Please [open a
PR](https://github.com/tower-rs/tower/compare) adding a brief
description of your library!

## <a href="#getting-started" class="doc-anchor">§</a>Getting Started

If you’re brand new to Tower and want to start with the basics, we
recommend you check out some of our
[guides](https://github.com/tower-rs/tower/tree/master/guides).

The various middleware implementations provided by this crate are
feature flagged, so that users can only compile the parts of Tower they
need. By default, all the optional middleware are disabled.

To get started using all of Tower’s optional middleware, add this to
your `Cargo.toml`:

``` language-toml
tower = { version = "0.4", features = ["full"] }
```

Alternatively, you can only enable some features. For example, to enable
only the [`retry`](retry/index.html "mod tower::retry") and
[`timeout`](timeout/index.html "mod tower::timeout") middleware, write:

``` language-toml
tower = { version = "0.4", features = ["retry", "timeout"] }
```

See [here](#modules) for a complete list of all middleware provided by
Tower.

## <a href="#supported-rust-versions" class="doc-anchor">§</a>Supported Rust Versions

Tower will keep a rolling MSRV (minimum supported Rust version) policy
of **at least** 6 months. When increasing the MSRV, the new Rust version
must have been released at least six months ago. The current MSRV is
1.64.0.

# Modules<a href="#modules" class="anchor">§</a>

<a href="builder/index.html" class="mod"
title="mod tower::builder">builder</a>  
Builder types to compose layers and services

<a href="layer/index.html" class="mod"
title="mod tower::layer">layer</a>  
A collection of [`Layer`](trait.Layer.html "trait tower::Layer") based
tower services

<a href="make/index.html" class="mod" title="mod tower::make">make</a>  
Trait aliases for Services that produce specific types of Responses.

<a href="retry/index.html" class="mod"
title="mod tower::retry">retry</a>  
Middleware for retrying “failed” requests.

<a href="timeout/index.html" class="mod"
title="mod tower::timeout">timeout</a>  
Middleware that applies a timeout to requests.

<a href="util/index.html" class="mod" title="mod tower::util">util</a>  
Various utility types and functions that are generally used with Tower.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ServiceBuilder.html" class="struct"
title="struct tower::ServiceBuilder">ServiceBuilder</a>  
Declaratively construct
[`Service`](trait.Service.html "trait tower::Service") values.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Layer.html" class="trait"
title="trait tower::Layer">Layer</a>  
Decorates a
[`Service`](https://docs.rs/tower/*/tower/trait.Service.html),
transforming either the request or the response.

<a href="trait.MakeService.html" class="trait"
title="trait tower::MakeService">MakeService</a>  
Creates new [`Service`](trait.Service.html "trait tower::Service")
values.

<a href="trait.Service.html" class="trait"
title="trait tower::Service">Service</a>  
An asynchronous function from a `Request` to a `Response`.

<a href="trait.ServiceExt.html" class="trait"
title="trait tower::ServiceExt">ServiceExt</a>  
An extension trait for `Service`s that provides a variety of convenient
adapters

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.service_fn.html" class="fn"
title="fn tower::service_fn">service_fn</a>  
Returns a new
[`ServiceFn`](util/struct.ServiceFn.html "struct tower::util::ServiceFn")
with the given closure.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BoxError.html" class="type"
title="type tower::BoxError">BoxError</a>  
Alias for a type-erased error type.
