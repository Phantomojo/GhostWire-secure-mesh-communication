Crate hyper\_tls Copy item path

<span class="sub-heading"><a href="../src/hyper_tls/lib.rs.html#1-41" class="src">Source</a>
</span>

Expand description

# <a href="#hyper-tls" class="doc-anchor">§</a>hyper-tls

An HTTPS connector to be used with [hyper](https://hyper.rs).

## <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use bytes::Bytes;
use http_body_util::Empty;
use hyper_tls::HttpsConnector;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);

    let res = client.get("https://hyper.rs".parse()?).await?;
    assert_eq!(res.status(), 200);
    Ok(())
}
```

## <a href="#crate-features" class="doc-anchor">§</a>Crate Features

-   `alpn`: Enables `native-tls/alpn`, and if `h2` is negotiated, tells
    hyper.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.HttpsConnecting.html" class="struct"
title="struct hyper_tls::HttpsConnecting">HttpsConnecting</a>  
A Future representing work to connect to a URL, and a TLS handshake.

<a href="struct.HttpsConnector.html" class="struct"
title="struct hyper_tls::HttpsConnector">HttpsConnector</a>  
A Connector for the `https` scheme.

<a href="struct.TlsStream.html" class="struct"
title="struct hyper_tls::TlsStream">TlsStream</a>  
A wrapper around an underlying raw stream which implements the TLS or
SSL protocol.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.MaybeHttpsStream.html" class="enum"
title="enum hyper_tls::MaybeHttpsStream">MaybeHttpsStream</a>  
A stream that might be protected with TLS.
