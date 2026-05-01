Crate reqwest Copy item path

<span class="sub-heading"><a href="../src/reqwest/lib.rs.html#1-395" class="src">Source</a>
</span>

Expand description

# <a href="#reqwest" class="doc-anchor">§</a>reqwest

The `reqwest` crate provides a convenient, higher-level HTTP
[`Client`](./struct.Client.html).

It handles many of the things that most people just expect an HTTP
client to do for them.

-   Async and [blocking](./blocking/index.html) Clients
-   Plain bodies, [JSON](#json), [urlencoded](#forms), \[multipart\]
-   Customizable [redirect policy](#redirect-policies)
-   HTTP [Proxies](#proxies)
-   Uses [TLS](#tls) by default
-   Cookies

The [`reqwest::Client`](./struct.Client.html) is asynchronous (requiring
Tokio). For applications wishing to only make a few HTTP requests, the
[`reqwest::blocking`](blocking/index.html "mod reqwest::blocking") API
may be more convenient.

Additional learning resources include:

-   [The Rust
    Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/web/clients.html)
-   [reqwest Repository
    Examples](https://github.com/seanmonstar/reqwest/tree/master/examples)

## <a href="#commercial-support" class="doc-anchor">§</a>Commercial Support

For private advice, support, reviews, access to the maintainer, and the
like, reach out for [commercial
support](https://seanmonstar.com/sponsor).

## <a href="#making-a-get-request" class="doc-anchor">§</a>Making a GET request

For a single request, you can use the [`get`](./fn.get.html) shortcut
method.

``` {.rust .rust-example-rendered}
let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;

println!("body = {body:?}");
```

**NOTE**: If you plan to perform multiple requests, it is best to create
a [`Client`](./struct.Client.html) and reuse it, taking advantage of
keep-alive connection pooling.

## <a href="#making-post-requests-or-setting-request-bodies"
class="doc-anchor">§</a>Making POST requests (or setting request bodies)

There are several ways you can set the body of a request. The basic one
is by using the `body()` method of a
[`RequestBuilder`](./struct.RequestBuilder.html). This lets you set the
exact raw bytes of what the body should be. It accepts various types,
including `String` and `Vec<u8>`. If you wish to pass a custom type, you
can use the `reqwest::Body` constructors.

``` {.rust .rust-example-rendered}
let client = reqwest::Client::new();
let res = client.post("http://httpbin.org/post")
    .body("the exact body that is sent")
    .send()
    .await?;
```

### <a href="#forms" class="doc-anchor">§</a>Forms

It’s very common to want to send form data in a request body. This can
be done with any type that can be serialized into form data.

This can be an array of tuples, or a `HashMap`, or a custom type that
implements [`Serialize`](http://serde.rs).

The feature `form` is required.

``` {.rust .rust-example-rendered}
// This will POST a body of `foo=bar&baz=quux`
let params = [("foo", "bar"), ("baz", "quux")];
let client = reqwest::Client::new();
let res = client.post("http://httpbin.org/post")
    .form(&params)
    .send()
    .await?;
```

### <a href="#json" class="doc-anchor">§</a>JSON

There is also a `json` method helper on the
[`RequestBuilder`](./struct.RequestBuilder.html) that works in a similar
fashion the `form` method. It can take any value that can be serialized
into JSON.

The feature `json` is required.

``` {.rust .rust-example-rendered}
// This will POST a body of `{"lang":"rust","body":"json"}`
let mut map = HashMap::new();
map.insert("lang", "rust");
map.insert("body", "json");

let client = reqwest::Client::new();
let res = client.post("http://httpbin.org/post")
    .json(&map)
    .send()
    .await?;
```

## <a href="#redirect-policies" class="doc-anchor">§</a>Redirect Policies

By default, a `Client` will automatically handle HTTP redirects, having
a maximum redirect chain of 10 hops. To customize this behavior, a
[`redirect::Policy`](redirect/index.html "mod reqwest::redirect") can be
used with a `ClientBuilder`.

## <a href="#cookies" class="doc-anchor">§</a>Cookies

The automatic storing and sending of session cookies can be enabled with
the \[`cookie_store`\]\[ClientBuilder::cookie\_store\] method on
`ClientBuilder`.

## <a href="#proxies" class="doc-anchor">§</a>Proxies

**NOTE**: System proxies are enabled by default.

System proxies look in environment variables to set HTTP or HTTPS
proxies.

`HTTP_PROXY` or `http_proxy` provide HTTP proxies for HTTP connections
while `HTTPS_PROXY` or `https_proxy` provide HTTPS proxies for HTTPS
connections. `ALL_PROXY` or `all_proxy` provide proxies for both HTTP
and HTTPS connections. If both the all proxy and HTTP or HTTPS proxy
variables are set the more specific HTTP or HTTPS proxies take
precedence.

These can be overwritten by adding a
[`Proxy`](struct.Proxy.html "struct reqwest::Proxy") to `ClientBuilder`
i.e. `let proxy = reqwest::Proxy::http("https://secure.example")?;` or
disabled by calling `ClientBuilder::no_proxy()`.

`socks` feature is required if you have configured socks proxy like
this:

``` language-bash
export https_proxy=socks5://127.0.0.1:1086
```

## <a href="#tls" class="doc-anchor">§</a>TLS

A `Client` will use transport layer security (TLS) by default to connect
to HTTPS destinations.

-   Additional server certificates can be configured on a
    `ClientBuilder` with the
    [`Certificate`](tls/struct.Certificate.html "struct reqwest::tls::Certificate")
    type.
-   Client certificates can be added to a `ClientBuilder` with the
    [`Identity`](tls/struct.Identity.html "struct reqwest::tls::Identity")
    type.
-   Various parts of TLS can also be configured or even disabled on the
    `ClientBuilder`.

See more details in the [`tls`](tls/index.html "mod reqwest::tls")
module.

## <a href="#wasm" class="doc-anchor">§</a>WASM

The Client implementation automatically switches to the WASM one when
the target\_arch is wasm32, the usage is basically the same as the async
api. Some of the features are disabled in wasm :
[`tls`](tls/index.html "mod reqwest::tls"), \[`cookie`\],
[`blocking`](blocking/index.html "mod reqwest::blocking"), as well as
various `ClientBuilder` methods such as `timeout()` and
`connector_layer()`.

TLS and cookies are provided through the browser environment, so reqwest
can issue TLS requests with cookies, but has limited configuration.

## <a href="#optional-features" class="doc-anchor">§</a>Optional Features

The following are a list of [Cargo
features](https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-features-section)
that can be enabled or disabled:

-   **http2** *(enabled by default)*: Enables HTTP/2 support.
-   **default-tls** *(enabled by default)*: Provides TLS support to
    connect over HTTPS.
-   **rustls**: Enables TLS functionality provided by `rustls`.
-   **native-tls**: Enables TLS functionality provided by `native-tls`.
-   **native-tls-vendored**: Enables the `vendored` feature of
    `native-tls`.
-   **native-tls-no-alpn**: Enables `native-tls` without its `alpn`
    feature.
-   **blocking**: Provides the [blocking](./blocking/index.html) client
    API.
-   **charset** *(enabled by default)*: Improved support for decoding
    text.
-   **cookies**: Provides cookie session support.
-   **gzip**: Provides response body gzip decompression.
-   **brotli**: Provides response body brotli decompression.
-   **zstd**: Provides response body zstd decompression.
-   **deflate**: Provides response body deflate decompression.
-   **query**: Provides query parameter serialization.
-   **form**: Provides form data serialization.
-   **json**: Provides serialization and deserialization for JSON
    bodies.
-   **multipart**: Provides functionality for multipart forms.
-   **stream**: Adds support for `futures::Stream`.
-   **socks**: Provides SOCKS5 proxy support.
-   **hickory-dns**: Enables a hickory-dns async resolver instead of
    default threadpool using `getaddrinfo`.
-   **system-proxy** *(enabled by default)*: Use Windows and macOS
    system proxy settings automatically.

## <a href="#unstable-features" class="doc-anchor">§</a>Unstable Features

Some feature flags require additional opt-in by the application, by
setting a `reqwest_unstable` flag.

-   **http3** *(unstable)*: Enables support for sending HTTP/3 requests.

These features are unstable, and experimental. Details about them may be
changed in patch releases.

You can pass such a flag to the compiler via `.cargo/config`, or
environment variables, such as:

``` language-notrust
RUSTFLAGS="--cfg reqwest_unstable" cargo build
```

## <a href="#sponsors" class="doc-anchor">§</a>Sponsors

Support this project by becoming a
[sponsor](https://seanmonstar.com/sponsor).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use tls::`<a href="tls/struct.Certificate.html" class="struct"
title="struct reqwest::tls::Certificate"><code>Certificate</code></a>`;`

`pub use tls::`<a href="tls/struct.Identity.html" class="struct"
title="struct reqwest::tls::Identity"><code>Identity</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="blocking/index.html" class="mod"
title="mod reqwest::blocking">blocking</a>  
A blocking Client API.

<a href="dns/index.html" class="mod" title="mod reqwest::dns">dns</a>  
DNS resolution

<a href="header/index.html" class="mod"
title="mod reqwest::header">header</a>  
HTTP header types

<a href="redirect/index.html" class="mod"
title="mod reqwest::redirect">redirect</a>  
Redirect Handling

<a href="retry/index.html" class="mod"
title="mod reqwest::retry">retry</a>  
Retry requests

<a href="tls/index.html" class="mod" title="mod reqwest::tls">tls</a>  
TLS configuration and types

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Body.html" class="struct"
title="struct reqwest::Body">Body</a>  
An asynchronous request body.

<a href="struct.Client.html" class="struct"
title="struct reqwest::Client">Client</a>  
An asynchronous `Client` to make Requests with.

<a href="struct.ClientBuilder.html" class="struct"
title="struct reqwest::ClientBuilder">ClientBuilder</a>  
A `ClientBuilder` can be used to create a `Client` with custom
configuration.

<a href="struct.Error.html" class="struct"
title="struct reqwest::Error">Error</a>  
The Errors that may occur when processing a `Request`.

<a href="struct.Method.html" class="struct"
title="struct reqwest::Method">Method</a>  
The Request Method (VERB)

<a href="struct.NoProxy.html" class="struct"
title="struct reqwest::NoProxy">NoProxy</a>  
A configuration for filtering out requests that shouldn’t be proxied

<a href="struct.Proxy.html" class="struct"
title="struct reqwest::Proxy">Proxy</a>  
Configuration of a proxy that a `Client` should pass requests to.

<a href="struct.Request.html" class="struct"
title="struct reqwest::Request">Request</a>  
A request which can be executed with `Client::execute()`.

<a href="struct.RequestBuilder.html" class="struct"
title="struct reqwest::RequestBuilder">RequestBuilder</a>  
A builder to construct the properties of a `Request`.

<a href="struct.Response.html" class="struct"
title="struct reqwest::Response">Response</a>  
A Response to a submitted `Request`.

<a href="struct.StatusCode.html" class="struct"
title="struct reqwest::StatusCode">StatusCode</a>  
An HTTP status code (`status-code` in RFC 9110 et al.).

<a href="struct.Upgraded.html" class="struct"
title="struct reqwest::Upgraded">Upgraded</a>  
An upgraded HTTP connection.

<a href="struct.Url.html" class="struct"
title="struct reqwest::Url">Url</a>  
A parsed URL record.

<a href="struct.Version.html" class="struct"
title="struct reqwest::Version">Version</a>  
Represents a version of the HTTP spec.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.IntoUrl.html" class="trait"
title="trait reqwest::IntoUrl">IntoUrl</a>  
A trait to try to convert some type into a `Url`.

<a href="trait.ResponseBuilderExt.html" class="trait"
title="trait reqwest::ResponseBuilderExt">ResponseBuilderExt</a>  
Extension trait for http::response::Builder objects

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.get.html" class="fn" title="fn reqwest::get">get</a>  
Shortcut method to quickly make a `GET` request.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type reqwest::Result">Result</a>  
A `Result` alias where the `Err` case is `reqwest::Error`.
