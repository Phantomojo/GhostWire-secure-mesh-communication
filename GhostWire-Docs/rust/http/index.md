Crate http Copy item path

<span class="sub-heading"><a href="../src/http/lib.rs.html#1-210" class="src">Source</a>
</span>

Expand description

A general purpose library of common HTTP types

This crate is a general purpose library for common types found when
working with the HTTP protocol. You’ll find
[`Request`](request/struct.Request.html "struct http::request::Request")
and
[`Response`](response/struct.Response.html "struct http::response::Response")
types for working as either a client or a server as well as all of their
components. Notably you’ll find `Uri` for what a
[`Request`](request/struct.Request.html "struct http::request::Request")
is requesting, a
[`Method`](method/struct.Method.html "struct http::method::Method") for
how it’s being requested, a
[`StatusCode`](status/struct.StatusCode.html "struct http::status::StatusCode")
for what sort of response came back, a
[`Version`](version/struct.Version.html "struct http::version::Version")
for how this was communicated, and
[`HeaderName`](header/struct.HeaderName.html "struct http::header::HeaderName")/[`HeaderValue`](header/struct.HeaderValue.html "struct http::header::HeaderValue")
definitions to get grouped in a
[`HeaderMap`](header/struct.HeaderMap.html "struct http::header::HeaderMap")
to work with request/response headers.

You will notably *not* find an implementation of sending requests or
spinning up a server in this crate. It’s intended that this crate is the
“standard library” for HTTP clients and servers without dictating any
particular implementation.

## <a href="#requests-and-responses" class="doc-anchor">§</a>Requests and Responses

Perhaps the main two types in this crate are the
[`Request`](request/struct.Request.html "struct http::request::Request")
and
[`Response`](response/struct.Response.html "struct http::response::Response")
types. A
[`Request`](request/struct.Request.html "struct http::request::Request")
could either be constructed to get sent off as a client or it can also
be received to generate a
[`Response`](response/struct.Response.html "struct http::response::Response")
for a server. Similarly as a client a
[`Response`](response/struct.Response.html "struct http::response::Response")
is what you get after sending a
[`Request`](request/struct.Request.html "struct http::request::Request"),
whereas on a server you’ll be manufacturing a
[`Response`](response/struct.Response.html "struct http::response::Response")
to send back to the client.

Each type has a number of accessors for the component fields. For as a
server you might want to inspect a requests URI to dispatch it:

``` {.rust .rust-example-rendered}
use http::{Request, Response};

fn response(req: Request<()>) -> http::Result<Response<()>> {
    match req.uri().path() {
        "/" => index(req),
        "/foo" => foo(req),
        "/bar" => bar(req),
        _ => not_found(req),
    }
}
```

On a
[`Request`](request/struct.Request.html "struct http::request::Request")
you’ll also find accessors like
[`method`](request/struct.Request.html#method.method "method http::request::Request::method")
to return a
[`Method`](method/struct.Method.html "struct http::method::Method") and
[`headers`](request/struct.Request.html#method.method "method http::request::Request::method")
to inspect the various headers. A
[`Response`](response/struct.Response.html "struct http::response::Response")
has similar methods for headers, the status code, etc.

In addition to getters, request/response types also have mutable
accessors to edit the request/response:

``` {.rust .rust-example-rendered}
use http::{HeaderValue, Response, StatusCode};
use http::header::CONTENT_TYPE;

fn add_server_headers<T>(response: &mut Response<T>) {
    response.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    *response.status_mut() = StatusCode::OK;
}
```

And finally, one of the most important aspects of requests/responses,
the body! The
[`Request`](request/struct.Request.html "struct http::request::Request")
and
[`Response`](response/struct.Response.html "struct http::response::Response")
types in this crate are *generic* in what their body is. This allows
downstream libraries to use different representations such as
`Request<Vec<u8>>`, `Response<impl Read>`,
`Request<impl Stream<Item = Vec<u8>, Error = _>>`, or even
`Response<MyCustomType>` where the custom type was deserialized from
JSON.

The body representation is intentionally flexible to give downstream
libraries maximal flexibility in implementing the body as appropriate.

## <a href="#http-headers" class="doc-anchor">§</a>HTTP Headers

Another major piece of functionality in this library is HTTP header
interpretation and generation. The `HeaderName` type serves as a way to
define header *names*, or what’s to the left of the colon. A
`HeaderValue` conversely is the header *value*, or what’s to the right
of a colon.

For example, if you have an HTTP request that looks like:

``` language-http
GET /foo HTTP/1.1
Accept: text/html
```

Then `"Accept"` is a
[`HeaderName`](header/struct.HeaderName.html "struct http::header::HeaderName")
while `"text/html"` is a
[`HeaderValue`](header/struct.HeaderValue.html "struct http::header::HeaderValue").
Each of these is a dedicated type to allow for a number of interesting
optimizations and to also encode the static guarantees of each type. For
example a
[`HeaderName`](header/struct.HeaderName.html "struct http::header::HeaderName")
is always a valid `&str`, but a
[`HeaderValue`](header/struct.HeaderValue.html "struct http::header::HeaderValue")
may not be valid UTF-8.

The most common header names are already defined for you as constant
values in the [`header`](header/index.html "mod http::header") module of
this crate. For example:

``` {.rust .rust-example-rendered}
use http::header::{self, HeaderName};

let name: HeaderName = header::ACCEPT;
assert_eq!(name.as_str(), "accept");
```

You can, however, also parse header names from strings:

``` {.rust .rust-example-rendered}
use http::header::{self, HeaderName};

let name = "Accept".parse::<HeaderName>().unwrap();
assert_eq!(name, header::ACCEPT);
```

Header values can be created from string literals through the
[`from_static`](header/struct.HeaderValue.html#method.from_static "associated function http::header::HeaderValue::from_static")
function:

``` {.rust .rust-example-rendered}
use http::HeaderValue;

let value = HeaderValue::from_static("text/html");
assert_eq!(value.as_bytes(), b"text/html");
```

And header values can also be parsed like names:

``` {.rust .rust-example-rendered}
use http::HeaderValue;

let value = "text/html";
let value = value.parse::<HeaderValue>().unwrap();
```

Most HTTP requests and responses tend to come with more than one header,
so it’s not too useful to just work with names and values only! This
crate also provides a
[`HeaderMap`](header/struct.HeaderMap.html "struct http::header::HeaderMap")
type which is a specialized hash map for keys as
[`HeaderName`](header/struct.HeaderName.html "struct http::header::HeaderName")
and generic values. This type, like header names, is optimized for
common usage but should continue to scale with your needs over time.

# <a href="#uris" class="doc-anchor">§</a>URIs

Each HTTP
[`Request`](request/struct.Request.html "struct http::request::Request")
has an associated URI with it. This may just be a path like
`/index.html` but it could also be an absolute URL such as
`https://www.rust-lang.org/index.html`. A
[`URI`](uri/struct.Uri.html "struct http::uri::Uri") has a number of
accessors to interpret it:

``` {.rust .rust-example-rendered}
use http::Uri;
use http::uri::Scheme;

let uri = "https://www.rust-lang.org/index.html".parse::<Uri>().unwrap();

assert_eq!(uri.scheme(), Some(&Scheme::HTTPS));
assert_eq!(uri.host(), Some("www.rust-lang.org"));
assert_eq!(uri.path(), "/index.html");
assert_eq!(uri.query(), None);
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::header::`<a href="header/struct.HeaderMap.html" class="struct"
title="struct http::header::HeaderMap"><code>HeaderMap</code></a>`;`

`pub use crate::header::`<a href="header/struct.HeaderName.html" class="struct"
title="struct http::header::HeaderName"><code>HeaderName</code></a>`;`

`pub use crate::header::`<a href="header/struct.HeaderValue.html" class="struct"
title="struct http::header::HeaderValue"><code>HeaderValue</code></a>`;`

`pub use crate::method::`<a href="method/struct.Method.html" class="struct"
title="struct http::method::Method"><code>Method</code></a>`;`

`pub use crate::request::`<a href="request/struct.Request.html" class="struct"
title="struct http::request::Request"><code>Request</code></a>`;`

`pub use crate::response::`<a href="response/struct.Response.html" class="struct"
title="struct http::response::Response"><code>Response</code></a>`;`

`pub use crate::status::`<a href="status/struct.StatusCode.html" class="struct"
title="struct http::status::StatusCode"><code>StatusCode</code></a>`;`

`pub use crate::uri::`<a href="uri/struct.Uri.html" class="struct"
title="struct http::uri::Uri"><code>Uri</code></a>`;`

`pub use crate::version::`<a href="version/struct.Version.html" class="struct"
title="struct http::version::Version"><code>Version</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="header/index.html" class="mod"
title="mod http::header">header</a>  
HTTP header types

<a href="method/index.html" class="mod"
title="mod http::method">method</a>  
The HTTP request method

<a href="request/index.html" class="mod"
title="mod http::request">request</a>  
HTTP request types.

<a href="response/index.html" class="mod"
title="mod http::response">response</a>  
HTTP response types.

<a href="status/index.html" class="mod"
title="mod http::status">status</a>  
HTTP status codes

<a href="uri/index.html" class="mod" title="mod http::uri">uri</a>  
URI component of request and response lines

<a href="version/index.html" class="mod"
title="mod http::version">version</a>  
HTTP version

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct http::Error">Error</a>  
A generic “error” for HTTP connections

<a href="struct.Extensions.html" class="struct"
title="struct http::Extensions">Extensions</a>  
A type map of protocol extensions.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type http::Result">Result</a>  
A `Result` typedef to use with the `http::Error` type
