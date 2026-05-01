Crate hyper\_rustls Copy item path

<span class="sub-heading"><a href="../src/hyper_rustls/lib.rs.html#1-76" class="src">Source</a>
</span>

Expand description

# <a href="#hyper-rustls" class="doc-anchor">§</a>hyper-rustls

A pure-Rust HTTPS connector for [hyper](https://hyper.rs), based on
[Rustls](https://github.com/rustls/rustls).

## <a href="#example-client" class="doc-anchor">§</a>Example client

``` {.rust .rust-example-rendered}
use http::StatusCode;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

let mut rt = tokio::runtime::Runtime::new().unwrap();
let url = ("https://hyper.rs").parse().unwrap();
let https = hyper_rustls::HttpsConnectorBuilder::new()
    .with_native_roots()
    .expect("no native root CA certificates found")
    .https_only()
    .enable_http1()
    .build();

let client: Client<_, Empty<Bytes>> = Client::builder(TokioExecutor::new()).build(https);

let res = rt.block_on(client.get(url)).unwrap();
assert_eq!(res.status(), StatusCode::OK);
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="builderstates/index.html" class="mod"
title="mod hyper_rustls::builderstates">builderstates</a>  
The various states of the
[`HttpsConnectorBuilder`](struct.HttpsConnectorBuilder.html "struct hyper_rustls::HttpsConnectorBuilder")

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DefaultServerNameResolver.html" class="struct"
title="struct hyper_rustls::DefaultServerNameResolver">DefaultServerNameResolver</a>  
The default server name resolver, which uses the hostname in the URI.

<a href="struct.FixedServerNameResolver.html" class="struct"
title="struct hyper_rustls::FixedServerNameResolver">FixedServerNameResolver</a>  
A server name resolver which always returns the same fixed name.

<a href="struct.HttpsConnector.html" class="struct"
title="struct hyper_rustls::HttpsConnector">HttpsConnector</a>  
A Connector for the `https` scheme.

<a href="struct.HttpsConnectorBuilder.html" class="struct"
title="struct hyper_rustls::HttpsConnectorBuilder">HttpsConnectorBuilder</a>  
A builder for an
[`HttpsConnector`](struct.HttpsConnector.html "struct hyper_rustls::HttpsConnector")

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.MaybeHttpsStream.html" class="enum"
title="enum hyper_rustls::MaybeHttpsStream">MaybeHttpsStream</a>  
A stream that might be protected with TLS.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ConfigBuilderExt.html" class="trait"
title="trait hyper_rustls::ConfigBuilderExt">ConfigBuilderExt</a>  
Methods for configuring roots

<a href="trait.ResolveServerName.html" class="trait"
title="trait hyper_rustls::ResolveServerName">ResolveServerName</a>  
A trait implemented by types that can resolve a
[`ServerName`](../rustls_pki_types/server_name/enum.ServerName.html "enum rustls_pki_types::server_name::ServerName")
for a request.
