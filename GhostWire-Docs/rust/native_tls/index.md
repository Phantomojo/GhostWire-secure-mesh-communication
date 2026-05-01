Crate native\_tls Copy item path

<span class="sub-heading"><a href="../src/native_tls/lib.rs.html#1-719" class="src">Source</a>
</span>

Expand description

An abstraction over platform-specific TLS implementations.

Many applications require TLS/SSL communication in one form or another
as part of their implementation, but finding a library for this isn’t
always trivial! The purpose of this crate is to provide a seamless
integration experience on all platforms with a cross-platform API that
deals with all the underlying details for you.

# <a href="#how-is-this-implemented" class="doc-anchor">§</a>How is this implemented?

This crate uses SChannel on Windows (via the `schannel` crate), Secure
Transport on OSX (via the `security-framework` crate), and OpenSSL (via
the `openssl` crate) on all other platforms. Future features may also
enable other TLS frameworks as well, but these initial libraries are
likely to remain as the defaults.

Note that this crate also strives to be secure-by-default. For example
when using OpenSSL it will configure validation callbacks to ensure that
hostnames match certificates, use strong ciphers, etc. This implies that
this crate is *not* just a thin abstraction around the underlying
libraries, but also an implementation that strives to strike reasonable
defaults.

# <a href="#supported-features" class="doc-anchor">§</a>Supported features

This crate supports the following features out of the box:

-   TLS/SSL client communication
-   TLS/SSL server communication
-   PKCS#12 encoded identities
-   X.509/PKCS#8 encoded identities
-   Secure-by-default for client and server
    -   Includes hostname verification for clients
-   Supports asynchronous I/O for both the server and the client

# <a href="#cargo-features" class="doc-anchor">§</a>Cargo Features

-   `vendored` - If enabled, the crate will compile and statically link
    to a vendored copy of OpenSSL. This feature has no effect on Windows
    and macOS, where OpenSSL is not used.

# <a href="#examples" class="doc-anchor">§</a>Examples

To connect as a client to a remote server:

``` {.rust .rust-example-rendered}
use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

let connector = TlsConnector::new().unwrap();

let stream = TcpStream::connect("google.com:443").unwrap();
let mut stream = connector.connect("google.com", stream).unwrap();

stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
let mut res = vec![];
stream.read_to_end(&mut res).unwrap();
println!("{}", String::from_utf8_lossy(&res));
```

To accept connections as a server from remote clients:

``` {.rust .rust-example-rendered}
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::{Read};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

let mut file = File::open("identity.pfx").unwrap();
let mut identity = vec![];
file.read_to_end(&mut identity).unwrap();
let identity = Identity::from_pkcs12(&identity, "hunter2").unwrap();

let listener = TcpListener::bind("0.0.0.0:8443").unwrap();
let acceptor = TlsAcceptor::new(identity).unwrap();
let acceptor = Arc::new(acceptor);

fn handle_client(stream: TlsStream<TcpStream>) {
    // ...
}

for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            let acceptor = acceptor.clone();
            thread::spawn(move || {
                let stream = acceptor.accept(stream).unwrap();
                handle_client(stream);
            });
        }
        Err(e) => { /* connection failed */ }
    }
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Certificate.html" class="struct"
title="struct native_tls::Certificate">Certificate</a>  
An X509 certificate.

<a href="struct.Error.html" class="struct"
title="struct native_tls::Error">Error</a>  
An error returned from the TLS implementation.

<a href="struct.Identity.html" class="struct"
title="struct native_tls::Identity">Identity</a>  
A cryptographic identity.

<a href="struct.MidHandshakeTlsStream.html" class="struct"
title="struct native_tls::MidHandshakeTlsStream">MidHandshakeTlsStream</a>  
A TLS stream which has been interrupted midway through the handshake
process.

<a href="struct.TlsAcceptor.html" class="struct"
title="struct native_tls::TlsAcceptor">TlsAcceptor</a>  
A builder for server-side TLS connections.

<a href="struct.TlsAcceptorBuilder.html" class="struct"
title="struct native_tls::TlsAcceptorBuilder">TlsAcceptorBuilder</a>  
A builder for `TlsAcceptor`s.

<a href="struct.TlsConnector.html" class="struct"
title="struct native_tls::TlsConnector">TlsConnector</a>  
A builder for client-side TLS connections.

<a href="struct.TlsConnectorBuilder.html" class="struct"
title="struct native_tls::TlsConnectorBuilder">TlsConnectorBuilder</a>  
A builder for `TlsConnector`s.

<a href="struct.TlsStream.html" class="struct"
title="struct native_tls::TlsStream">TlsStream</a>  
A stream managing a TLS session.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.HandshakeError.html" class="enum"
title="enum native_tls::HandshakeError">HandshakeError</a>  
An error returned from `ClientBuilder::handshake`.

<a href="enum.Protocol.html" class="enum"
title="enum native_tls::Protocol">Protocol</a>  
SSL/TLS protocol versions.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type native_tls::Result">Result</a>  
A typedef of the result-type returned by many methods.
