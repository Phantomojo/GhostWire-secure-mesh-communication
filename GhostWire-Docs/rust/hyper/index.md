Crate hyper Copy item path

<span class="sub-heading"><a href="../src/hyper/lib.rs.html#1-139" class="src">Source</a>
</span>

Expand description

# <a href="#hyper" class="doc-anchor">§</a>hyper

hyper is a **fast** and **correct** HTTP implementation written in and
for Rust.

## <a href="#features" class="doc-anchor">§</a>Features

-   HTTP/1 and HTTP/2
-   Asynchronous design
-   Leading in performance
-   Tested and **correct**
-   Extensive production use
-   [Client](client/index.html) and [Server](server/index.html) APIs

If just starting out, **check out the
[Guides](https://hyper.rs/guides/1/) first.**

## <a href="#low-level" class="doc-anchor">§</a>“Low-level”

hyper is a lower-level HTTP library, meant to be a building block for
libraries and applications.

If looking for just a convenient HTTP client, consider the
[reqwest](https://crates.io/crates/reqwest) crate.

# <a href="#optional-features" class="doc-anchor">§</a>Optional Features

hyper uses a set of [feature
flags](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section)
to reduce the amount of compiled code. It is possible to just enable
certain features over others. By default, hyper does not enable any
features but allows one to enable a subset for their use case. Below is
a list of the available feature flags. You may also notice above each
function, struct and trait there is listed one or more feature flags
that are required for that item to be used.

If you are new to hyper it is possible to enable the `full` feature flag
which will enable all public APIs. Beware though that this will pull in
many extra dependencies that you may not need.

The following optional features are available:

-   `http1`: Enables HTTP/1 support.
-   `http2`: Enables HTTP/2 support.
-   `client`: Enables the HTTP `client`.
-   `server`: Enables the HTTP `server`.

## <a href="#unstable-features" class="doc-anchor">§</a>Unstable Features

hyper includes a set of unstable optional features that can be enabled
through the use of a feature flag and a [configuration
flag](https://doc.rust-lang.org/reference/conditional-compilation.html).

The following is a list of feature flags and their corresponding
`RUSTFLAG`:

-   `ffi`: Enables C API for hyper `hyper_unstable_ffi`.
-   `tracing`: Enables debug logging with `hyper_unstable_tracing`.

For example:

``` language-notrust
RUSTFLAGS="--cfg hyper_unstable_tracing" cargo build
```

# <a href="#stability" class="doc-anchor">§</a>Stability

It’s worth talking a bit about the stability of hyper. hyper’s API
follows [SemVer](https://semver.org). Breaking changes will only be
introduced in major versions, if ever. New additions to the API, such as
new types, methods, or traits will only be added in minor versions.

Some parts of hyper are documented as NOT being part of the stable API.
The following is a brief list, you can read more about each one in the
relevant part of the documentation.

-   Downcasting error types from `Error::source()` is not considered
    stable.
-   Private dependencies use of global variables is not considered
    stable. So, if a dependency uses `log` or `tracing`, hyper doesn’t
    promise it will continue to do so.
-   Behavior from default options is not stable. hyper reserves the
    right to add new options that are enabled by default which might
    alter the behavior, for the purposes of protection. It is also
    possible to *change* what the default options are set to, also in
    efforts to protect the most people possible.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use http::`<a href="../http/header/index.html" class="mod"
title="mod http::header"><code>header</code></a>`;`

`pub use http::`<a href="../http/header/map/struct.HeaderMap.html" class="struct"
title="struct http::header::map::HeaderMap"><code>HeaderMap</code></a>`;`

`pub use http::`<a href="../http/method/struct.Method.html" class="struct"
title="struct http::method::Method"><code>Method</code></a>`;`

`pub use http::`<a href="../http/request/struct.Request.html" class="struct"
title="struct http::request::Request"><code>Request</code></a>`;`

`pub use http::`<a href="../http/response/struct.Response.html" class="struct"
title="struct http::response::Response"><code>Response</code></a>`;`

`pub use http::`<a href="../http/status/struct.StatusCode.html" class="struct"
title="struct http::status::StatusCode"><code>StatusCode</code></a>`;`

`pub use http::`<a href="../http/uri/struct.Uri.html" class="struct"
title="struct http::uri::Uri"><code>Uri</code></a>`;`

`pub use http::`<a href="../http/version/struct.Version.html" class="struct"
title="struct http::version::Version"><code>Version</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="body/index.html" class="mod" title="mod hyper::body">body</a>  
Streaming bodies for Requests and Responses

<a href="client/index.html" class="mod"
title="mod hyper::client">client</a>  
HTTP Client

<a href="ext/index.html" class="mod" title="mod hyper::ext">ext</a>  
Extensions for HTTP messages in Hyper.

<a href="rt/index.html" class="mod" title="mod hyper::rt">rt</a>  
Runtime components

<a href="server/index.html" class="mod"
title="mod hyper::server">server</a>  
HTTP Server

<a href="service/index.html" class="mod"
title="mod hyper::service">service</a>  
Asynchronous Services

<a href="upgrade/index.html" class="mod"
title="mod hyper::upgrade">upgrade</a>  
HTTP Upgrades

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct hyper::Error">Error</a>  
Represents errors that can occur handling HTTP streams.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type hyper::Result">Result</a>  
Result type often returned from methods that can have hyper `Error`s.
