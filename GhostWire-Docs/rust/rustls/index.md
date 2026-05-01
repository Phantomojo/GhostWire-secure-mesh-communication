Crate rustls Copy item path

<span class="sub-heading"><a href="../src/rustls/lib.rs.html#1-717" class="src">Source</a>
</span>

Expand description

# <a href="#rustls---a-modern-tls-library" class="doc-anchor">§</a>Rustls - a modern TLS library

Rustls is a TLS library that aims to provide a good level of
cryptographic security, requires no configuration to achieve that
security, and provides no unsafe features or obsolete cryptography by
default.

Rustls implements TLS1.2 and TLS1.3 for both clients and servers. See
[the full list of protocol
features](manual/_04_features/index.html "mod rustls::manual::_04_features").

### <a href="#platform-support" class="doc-anchor">§</a>Platform support

While Rustls itself is platform independent, by default it uses
[`aws-lc-rs`](https://crates.io/crates/aws-lc-rs) for implementing the
cryptography in TLS. See [the aws-lc-rs
FAQ](https://aws.github.io/aws-lc-rs/faq.html#can-i-run-aws-lc-rs-on-x-platform-or-architecture)
for more details of the platform/architecture support constraints in
aws-lc-rs.

[`ring`](https://crates.io/crates/ring) is also available via the `ring`
crate feature: see [the supported `ring` target
platforms](https://github.com/briansmith/ring/blob/2e8363b433fa3b3962c877d9ed2e9145612f3160/include/ring-core/target.h#L18-L64).

By providing a custom instance of the
[`crypto::CryptoProvider`](crypto/struct.CryptoProvider.html "struct rustls::crypto::CryptoProvider")
struct, you can replace all cryptography dependencies of rustls. This is
a route to being portable to a wider set of architectures and
environments, or compliance requirements. See the
[`crypto::CryptoProvider`](crypto/struct.CryptoProvider.html "struct rustls::crypto::CryptoProvider")
documentation for more details.

Specifying `default-features = false` when depending on rustls will
remove the implicit dependency on aws-lc-rs.

Rustls requires Rust 1.71 or later. It has an optional dependency on
zlib-rs which requires 1.75 or later.

### <a href="#cryptography-providers" class="doc-anchor">§</a>Cryptography providers

Since Rustls 0.22 it has been possible to choose the provider of the
cryptographic primitives that Rustls uses. This may be appealing if you
have specific platform, compliance or feature requirements that aren’t
met by the default provider,
[`aws-lc-rs`](https://crates.io/crates/aws-lc-rs).

Users that wish to customize the provider in use can do so when
constructing `ClientConfig` and `ServerConfig` instances using the
`with_crypto_provider` method on the respective config builder types.
See the
[`crypto::CryptoProvider`](crypto/struct.CryptoProvider.html "struct rustls::crypto::CryptoProvider")
documentation for more details.

#### <a href="#built-in-providers" class="doc-anchor">§</a>Built-in providers

Rustls ships with two built-in providers controlled by associated crate
features:

-   [`aws-lc-rs`](https://crates.io/crates/aws-lc-rs) - enabled by
    default, available with the `aws_lc_rs` crate feature enabled.
-   [`ring`](https://crates.io/crates/ring) - available with the `ring`
    crate feature enabled.

See the documentation for
[`crypto::CryptoProvider`](crypto/struct.CryptoProvider.html "struct rustls::crypto::CryptoProvider")
for details on how providers are selected.

#### <a href="#third-party-providers" class="doc-anchor">§</a>Third-party providers

The community has also started developing third-party providers for
Rustls:

-   [`boring-rustls-provider`](https://github.com/janrueth/boring-rustls-provider) -
    a work-in-progress provider that uses
    [`boringssl`](https://github.com/google/boringssl) for cryptography.
-   [`rustls-graviola`](https://crates.io/crates/rustls-graviola) - a
    provider that uses [`graviola`](https://github.com/ctz/graviola) for
    cryptography.
-   [`rustls-mbedtls-provider`](https://github.com/fortanix/rustls-mbedtls-provider) -
    a provider that uses
    [`mbedtls`](https://github.com/Mbed-TLS/mbedtls) for cryptography.
-   [`rustls-openssl`](https://github.com/tofay/rustls-openssl) - a
    provider that uses [OpenSSL](https://openssl-library.org/) for
    cryptography.
-   [`rustls-rustcrypto`](https://github.com/RustCrypto/rustls-rustcrypto) -
    an experimental provider that uses the crypto primitives from
    [`RustCrypto`](https://github.com/RustCrypto) for cryptography.
-   [`rustls-symcrypt`](https://github.com/microsoft/rustls-symcrypt) -
    a provider that uses Microsoft’s
    [SymCrypt](https://github.com/microsoft/SymCrypt) library.
-   [`rustls-wolfcrypt-provider`](https://github.com/wolfSSL/rustls-wolfcrypt-provider) -
    a work-in-progress provider that uses
    [`wolfCrypt`](https://www.wolfssl.com/products/wolfcrypt) for
    cryptography.

#### <a href="#custom-provider" class="doc-anchor">§</a>Custom provider

We also provide a simple example of writing your own provider in the
[custom provider
example](https://github.com/rustls/rustls/tree/main/provider-example/).
This example implements a minimal provider using parts of the
[`RustCrypto`](https://github.com/RustCrypto) ecosystem.

See the [Making a custom
CryptoProvider](https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html#making-a-custom-cryptoprovider)
section of the documentation for more information on this topic.

## <a href="#design-overview" class="doc-anchor">§</a>Design overview

Rustls is a low-level library. If your goal is to make HTTPS connections
you may prefer to use a library built on top of Rustls like
[hyper](https://crates.io/crates/hyper) or
[ureq](https://crates.io/crates/ureq).

### <a href="#rustls-does-not-take-care-of-network-io"
class="doc-anchor">§</a>Rustls does not take care of network IO

It doesn’t make or accept TCP connections, or do DNS, or read or write
files.

Our [examples](https://github.com/rustls/rustls/tree/main/examples)
directory contains demos that show how to handle I/O using the
[`stream::Stream`](struct.Stream.html "struct rustls::Stream") helper,
as well as more complex asynchronous I/O using
[`mio`](https://docs.rs/mio/latest/mio/). If you’re already using Tokio
for an async runtime you may prefer to use
[`tokio-rustls`](https://github.com/rustls/tokio-rustls) instead of
interacting with rustls directly.

### <a href="#rustls-provides-encrypted-pipes" class="doc-anchor">§</a>Rustls provides encrypted pipes

These are the
[`ServerConnection`](server/struct.ServerConnection.html "struct rustls::server::ServerConnection")
and
[`ClientConnection`](client/struct.ClientConnection.html "struct rustls::client::ClientConnection")
types. You supply raw TLS traffic on the left (via the
[`read_tls()`](enum.Connection.html#method.read_tls "method rustls::Connection::read_tls")
and
[`write_tls()`](enum.Connection.html#method.read_tls "method rustls::Connection::read_tls")
methods) and then read/write the plaintext on the right:

``` language-text
         TLS                                   Plaintext
         ===                                   =========
    read_tls()      +-----------------------+      reader() as io::Read
                    |                       |
          +--------->   ClientConnection    +--------->
                    |          or           |
          <---------+   ServerConnection    <---------+
                    |                       |
    write_tls()     +-----------------------+      writer() as io::Write
```

### <a href="#rustls-takes-care-of-server-certificate-verification"
class="doc-anchor">§</a>Rustls takes care of server certificate verification

You do not need to provide anything other than a set of root
certificates to trust. Certificate verification cannot be turned off or
disabled in the main API.

## <a href="#getting-started" class="doc-anchor">§</a>Getting started

This is the minimum you need to do to make a TLS client connection.

First we load some root certificates. These are used to authenticate the
server. The simplest way is to depend on the
[`webpki_roots`](https://crates.io/crates/webpki-roots) crate which
contains the Mozilla set of root certificates.

``` {.rust .rust-example-rendered}
let root_store = rustls::RootCertStore::from_iter(
    webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned(),
);
```

Next, we make a `ClientConfig`. You’re likely to make one of these per
process, and use it for all connections made by that process.

``` {.rust .rust-example-rendered}
let config = rustls::ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth();
```

Now we can make a connection. You need to provide the server’s hostname
so we know what to expect to find in the server’s certificate.

``` {.rust .rust-example-rendered}
let rc_config = Arc::new(config);
let example_com = "example.com".try_into().unwrap();
let mut client = rustls::ClientConnection::new(rc_config, example_com);
```

Now you should do appropriate IO for the `client` object. If
`client.wants_read()` yields true, you should call `client.read_tls()`
when the underlying connection has data. Likewise, if
`client.wants_write()` yields true, you should call `client.write_tls()`
when the underlying connection is able to send data. You should continue
doing this as long as the connection is valid.

The return types of `read_tls()` and `write_tls()` only tell you if the
IO worked. No parsing or processing of the TLS messages is done. After
each `read_tls()` you should therefore call
`client.process_new_packets()` which parses and processes the messages.
Any error returned from `process_new_packets` is fatal to the
connection, and will tell you why. For example, if the server’s
certificate is expired `process_new_packets` will return
`Err(InvalidCertificate(Expired))`. From this point on,
`process_new_packets` will not do any new work and will return that
error continually.

You can extract newly received data by calling `client.reader()` (which
implements the `io::Read` trait). You can send data to the peer by
calling `client.writer()` (which implements `io::Write` trait). Note
that `client.writer().write()` buffers data you send if the TLS
connection is not yet established: this is useful for writing (say) a
HTTP request, but this is buffered so avoid large amounts of data.

The following code uses a fictional socket IO API for illustration, and
does not handle errors.

``` {.rust .rust-example-rendered}
use std::io;
use rustls::Connection;

client.writer().write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
let mut socket = connect("example.com", 443);
loop {
  if client.wants_read() && socket.ready_for_read() {
    client.read_tls(&mut socket).unwrap();
    client.process_new_packets().unwrap();

    let mut plaintext = Vec::new();
    client.reader().read_to_end(&mut plaintext).unwrap();
    io::stdout().write(&plaintext).unwrap();
  }

  if client.wants_write() && socket.ready_for_write() {
    client.write_tls(&mut socket).unwrap();
  }

  socket.wait_for_something_to_happen();
}
```

# <a href="#examples" class="doc-anchor">§</a>Examples

You can find several client and server examples of varying complexity in
the [examples](https://github.com/rustls/rustls/tree/main/examples)
directory, including
[`tlsserver-mio`](https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsserver-mio.rs)
and
[`tlsclient-mio`](https://github.com/rustls/rustls/blob/main/examples/src/bin/tlsclient-mio.rs) -
full worked examples using [`mio`](https://docs.rs/mio/latest/mio/).

# <a href="#manual" class="doc-anchor">§</a>Manual

The [rustls manual](manual/index.html "mod rustls::manual") explains
design decisions and includes how-to guidance.

# <a href="#crate-features" class="doc-anchor">§</a>Crate features

Here’s a list of what features are exposed by the rustls crate and what
they mean.

-   `std` (enabled by default): enable the high-level (buffered)
    Connection API and other functionality which relies on the `std`
    library.

-   `aws_lc_rs` (enabled by default): makes the rustls crate depend on
    the [`aws-lc-rs`](https://crates.io/crates/aws-lc-rs) crate. Use
    `rustls::crypto::aws_lc_rs::default_provider().install_default()` to
    use it as the default `CryptoProvider`, or provide it explicitly
    when making a `ClientConfig` or `ServerConfig`.

    Note that aws-lc-rs has additional build-time dependencies like
    cmake. See [the
    documentation](https://aws.github.io/aws-lc-rs/requirements/index.html)
    for details.

-   `ring`: makes the rustls crate depend on the *ring* crate for
    cryptography. Use
    `rustls::crypto::ring::default_provider().install_default()` to use
    it as the default `CryptoProvider`, or provide it explicitly when
    making a `ClientConfig` or `ServerConfig`.

-   `fips`: enable support for FIPS140-3-approved cryptography, via the
    [`aws-lc-rs`](https://crates.io/crates/aws-lc-rs) crate. This
    feature enables the `aws_lc_rs` crate feature, which makes the
    rustls crate depend on
    [aws-lc-rs](https://github.com/aws/aws-lc-rs). It also changes the
    default for
    [`ServerConfig::require_ems`](server/struct.ServerConfig.html#structfield.require_ems "field rustls::server::ServerConfig::require_ems")
    and
    [`ClientConfig::require_ems`](client/struct.ClientConfig.html#structfield.require_ems "field rustls::client::ClientConfig::require_ems").

    See
    [manual::\_06\_fips](manual/_06_fips/index.html "mod rustls::manual::_06_fips")
    for more details.

-   `prefer-post-quantum` (enabled by default): for the
    [`aws-lc-rs`](https://crates.io/crates/aws-lc-rs)-backed provider,
    prioritizes post-quantum secure key exchange by default (using
    X25519MLKEM768). This feature merely alters the order of
    `rustls::crypto::aws_lc_rs::DEFAULT_KX_GROUPS`. See [the
    manual](manual/_05_defaults/index.html#about-the-post-quantum-secure-key-exchange-x25519mlkem768 "mod rustls::manual::_05_defaults")
    for more details.

-   `custom-provider`: disables implicit use of built-in providers
    (`aws-lc-rs` or `ring`). This forces applications to manually
    install one, for instance, when using a custom `CryptoProvider`.

-   `tls12` (enabled by default): enable support for TLS version 1.2.
    Note that, due to the additive nature of Cargo features and because
    it is enabled by default, other crates in your dependency graph
    could re-enable it for your application. If you want to disable TLS
    1.2 for security reasons, consider explicitly enabling TLS 1.3 only
    in the config builder API.

-   `logging` (enabled by default): make the rustls crate depend on the
    `log` crate. rustls outputs interesting protocol-level messages at
    `trace!` and `debug!` level, and protocol-level errors at `warn!`
    and `error!` level. The log messages do not contain secret key data,
    and so are safe to archive without affecting session security.

-   `read_buf`: when building with Rust Nightly, adds support for the
    unstable `std::io::ReadBuf` and related APIs. This reduces costs
    from initializing buffers. Will do nothing on non-Nightly releases.

-   `brotli`: uses the `brotli` crate for RFC8879 certificate
    compression support.

-   `zlib`: uses the `zlib-rs` crate for RFC8879 certificate compression
    support.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::ticketer::`<a href="ticketer/struct.TicketRotator.html" class="struct"
title="struct rustls::ticketer::TicketRotator"><code>TicketRotator</code></a>`;`

`pub use crate::ticketer::`<a href="ticketer/struct.TicketSwitcher.html" class="struct"
title="struct rustls::ticketer::TicketSwitcher"><code>TicketSwitcher</code></a>`;`

`pub use client::`<a href="client/struct.ClientConfig.html" class="struct"
title="struct rustls::client::ClientConfig"><code>ClientConfig</code></a>`;`

`pub use client::`<a href="client/struct.ClientConnection.html" class="struct"
title="struct rustls::client::ClientConnection"><code>ClientConnection</code></a>`;`

`pub use server::`<a href="server/struct.ServerConfig.html" class="struct"
title="struct rustls::server::ServerConfig"><code>ServerConfig</code></a>`;`

`pub use server::`<a href="server/struct.ServerConnection.html" class="struct"
title="struct rustls::server::ServerConnection"><code>ServerConnection</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="client/index.html" class="mod"
title="mod rustls::client">client</a>  
Items for use in a client.

<a href="compress/index.html" class="mod"
title="mod rustls::compress">compress</a>  
Certificate compression and decompression support

<a href="crypto/index.html" class="mod"
title="mod rustls::crypto">crypto</a>  
Crypto provider interface.

<a href="ffdhe_groups/index.html" class="mod"
title="mod rustls::ffdhe_groups">ffdhe_groups</a>  
This module contains parameters for FFDHE named groups as defined in
[RFC 7919 Appendix
A](https://datatracker.ietf.org/doc/html/rfc7919#appendix-A).

<a href="kernel/index.html" class="mod"
title="mod rustls::kernel">kernel</a>  
Kernel connection API.

<a href="lock/index.html" class="mod" title="mod rustls::lock">lock</a>  
APIs abstracting over locking primitives.

<a href="manual/index.html" class="mod"
title="mod rustls::manual">manual</a>  
This is the rustls manual.

<a href="pki_types/index.html" class="mod"
title="mod rustls::pki_types">pki_types</a>  
Re-exports the contents of the
[rustls-pki-types](https://docs.rs/rustls-pki-types) crate for easy
access

<a href="quic/index.html" class="mod" title="mod rustls::quic">quic</a>  
APIs for implementing QUIC TLS

<a href="server/index.html" class="mod"
title="mod rustls::server">server</a>  
Items for use in a server.

<a href="sign/index.html" class="mod" title="mod rustls::sign">sign</a>  
Message signing interfaces.

<a href="ticketer/index.html" class="mod"
title="mod rustls::ticketer">ticketer</a>  
APIs for implementing TLS tickets

<a href="time_provider/index.html" class="mod"
title="mod rustls::time_provider">time_provider</a>  
The library’s source of time.

<a href="unbuffered/index.html" class="mod"
title="mod rustls::unbuffered">unbuffered</a>  
Unbuffered connection API

<a href="version/index.html" class="mod"
title="mod rustls::version">version</a>  
All defined protocol versions appear in this module.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CipherSuiteCommon.html" class="struct"
title="struct rustls::CipherSuiteCommon">CipherSuiteCommon</a>  
Common state for cipher suites (both for TLS 1.2 and TLS 1.3)

<a href="struct.CommonState.html" class="struct"
title="struct rustls::CommonState">CommonState</a>  
Connection state common to both client and server connections.

<a href="struct.ConfigBuilder.html" class="struct"
title="struct rustls::ConfigBuilder">ConfigBuilder</a>  
A
[builder](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
for
[`ServerConfig`](server/struct.ServerConfig.html "struct rustls::server::ServerConfig")
or
[`ClientConfig`](client/struct.ClientConfig.html "struct rustls::client::ClientConfig")
values.

<a href="struct.ConnectionCommon.html" class="struct"
title="struct rustls::ConnectionCommon">ConnectionCommon</a>  
Interface shared by client and server connections.

<a href="struct.DigitallySignedStruct.html" class="struct"
title="struct rustls::DigitallySignedStruct">DigitallySignedStruct</a>  
This type combines a
[`SignatureScheme`](enum.SignatureScheme.html "enum rustls::SignatureScheme")
and a signature payload produced with that scheme.

<a href="struct.DistinguishedName.html" class="struct"
title="struct rustls::DistinguishedName">DistinguishedName</a>  
A `DistinguishedName` is a `Vec<u8>` wrapped in internal types.

<a href="struct.ExtractedSecrets.html" class="struct"
title="struct rustls::ExtractedSecrets">ExtractedSecrets</a>  
Secrets for transmitting/receiving data over a TLS session.

<a href="struct.IoState.html" class="struct"
title="struct rustls::IoState">IoState</a>  
Values of this structure are returned from
[`Connection::process_new_packets`](enum.Connection.html#method.process_new_packets "method rustls::Connection::process_new_packets")
and tell the caller the current I/O state of the TLS connection.

<a href="struct.KeyLogFile.html" class="struct"
title="struct rustls::KeyLogFile">KeyLogFile</a>  
[`KeyLog`](trait.KeyLog.html "trait rustls::KeyLog") implementation that
opens a file whose name is given by the `SSLKEYLOGFILE` environment
variable, and writes keys into it.

<a href="struct.NoKeyLog.html" class="struct"
title="struct rustls::NoKeyLog">NoKeyLog</a>  
KeyLog that does exactly nothing.

<a href="struct.OtherError.html" class="struct"
title="struct rustls::OtherError">OtherError</a>  
Any other error that cannot be expressed by a more specific
[`Error`](enum.Error.html "enum rustls::Error") variant.

<a href="struct.Reader.html" class="struct"
title="struct rustls::Reader">Reader</a>  
A structure that implements
[`std::io::Read`](https://doc.rust-lang.org/1.93.1/std/io/trait.Read.html "trait std::io::Read")
for reading plaintext.

<a href="struct.RootCertStore.html" class="struct"
title="struct rustls::RootCertStore">RootCertStore</a>  
A container for root certificates able to provide a root-of-trust for
connection authentication.

<a href="struct.Stream.html" class="struct"
title="struct rustls::Stream">Stream</a>  
This type implements `io::Read` and `io::Write`, encapsulating a
Connection `C` and an underlying transport `T`, such as a socket.

<a href="struct.StreamOwned.html" class="struct"
title="struct rustls::StreamOwned">StreamOwned</a>  
This type implements `io::Read` and `io::Write`, encapsulating and
owning a Connection `C` and an underlying transport `T`, such as a
socket.

<a href="struct.SupportedProtocolVersion.html" class="struct"
title="struct rustls::SupportedProtocolVersion">SupportedProtocolVersion</a>  
A TLS protocol version supported by rustls.

<a href="struct.Tls12CipherSuite.html" class="struct"
title="struct rustls::Tls12CipherSuite">Tls12CipherSuite</a>  
A TLS 1.2 cipher suite supported by rustls.

<a href="struct.Tls13CipherSuite.html" class="struct"
title="struct rustls::Tls13CipherSuite">Tls13CipherSuite</a>  
A TLS 1.3 cipher suite supported by rustls.

<a href="struct.WantsVerifier.html" class="struct"
title="struct rustls::WantsVerifier">WantsVerifier</a>  
Config builder state where the caller must supply a verifier.

<a href="struct.WantsVersions.html" class="struct"
title="struct rustls::WantsVersions">WantsVersions</a>  
Config builder state where the caller must supply TLS protocol versions.

<a href="struct.Writer.html" class="struct"
title="struct rustls::Writer">Writer</a>  
A structure that implements
[`std::io::Write`](https://doc.rust-lang.org/1.93.1/std/io/trait.Write.html "trait std::io::Write")
for writing plaintext.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AlertDescription.html" class="enum"
title="enum rustls::AlertDescription">AlertDescription</a>  
The `AlertDescription` TLS protocol enum. Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA. The
`Unknown` item is used when processing unrecognised ordinals.

<a href="enum.CertRevocationListError.html" class="enum"
title="enum rustls::CertRevocationListError">CertRevocationListError</a>  
The ways in which a certificate revocation list (CRL) can be invalid.

<a href="enum.CertificateCompressionAlgorithm.html" class="enum"
title="enum rustls::CertificateCompressionAlgorithm">CertificateCompressionAlgorithm</a>  
The “TLS Certificate Compression Algorithm IDs” TLS protocol enum.
Values in this enum are taken from
[RFC8879](https://www.rfc-editor.org/rfc/rfc8879.html#section-7.3).

<a href="enum.CertificateError.html" class="enum"
title="enum rustls::CertificateError">CertificateError</a>  
The ways in which certificate validators can express errors.

<a href="enum.CipherSuite.html" class="enum"
title="enum rustls::CipherSuite">CipherSuite</a>  
The `CipherSuite` TLS protocol enum. Values in this enum are taken from
the various RFCs covering TLS, and are listed by IANA. The `Unknown`
item is used when processing unrecognised ordinals.

<a href="enum.Connection.html" class="enum"
title="enum rustls::Connection">Connection</a>  
A client or server connection.

<a href="enum.ConnectionTrafficSecrets.html" class="enum"
title="enum rustls::ConnectionTrafficSecrets">ConnectionTrafficSecrets</a>  
Secrets used to encrypt/decrypt data in a TLS session.

<a href="enum.ContentType.html" class="enum"
title="enum rustls::ContentType">ContentType</a>  
The `ContentType` TLS protocol enum. Values in this enum are taken from
the various RFCs covering TLS, and are listed by IANA. The `Unknown`
item is used when processing unrecognised ordinals.

<a href="enum.EncryptedClientHelloError.html" class="enum"
title="enum rustls::EncryptedClientHelloError">EncryptedClientHelloError</a>  
An error that occurred while handling Encrypted Client Hello (ECH).

<a href="enum.Error.html" class="enum"
title="enum rustls::Error">Error</a>  
rustls reports protocol errors using this type.

<a href="enum.ExtendedKeyPurpose.html" class="enum"
title="enum rustls::ExtendedKeyPurpose">ExtendedKeyPurpose</a>  
Extended Key Usage (EKU) purpose values.

<a href="enum.HandshakeKind.html" class="enum"
title="enum rustls::HandshakeKind">HandshakeKind</a>  
Describes which sort of handshake happened.

<a href="enum.HandshakeType.html" class="enum"
title="enum rustls::HandshakeType">HandshakeType</a>  
The `HandshakeType` TLS protocol enum. Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA. The
`Unknown` item is used when processing unrecognised ordinals.

<a href="enum.InconsistentKeys.html" class="enum"
title="enum rustls::InconsistentKeys">InconsistentKeys</a>  
Specific failure cases from
[`keys_match`](sign/struct.CertifiedKey.html#method.keys_match "method rustls::sign::CertifiedKey::keys_match")
or a
[`crate::crypto::signer::SigningKey`](sign/trait.SigningKey.html "trait rustls::sign::SigningKey")
that cannot produce a corresponding public key.

<a href="enum.InvalidMessage.html" class="enum"
title="enum rustls::InvalidMessage">InvalidMessage</a>  
A corrupt TLS message payload that resulted in an error.

<a href="enum.NamedGroup.html" class="enum"
title="enum rustls::NamedGroup">NamedGroup</a>  
The `NamedGroup` TLS protocol enum. Values in this enum are taken from
the various RFCs covering TLS, and are listed by IANA. The `Unknown`
item is used when processing unrecognised ordinals.

<a href="enum.PeerIncompatible.html" class="enum"
title="enum rustls::PeerIncompatible">PeerIncompatible</a>  
The set of cases where we failed to make a connection because a peer
doesn’t support a TLS version/feature we require.

<a href="enum.PeerMisbehaved.html" class="enum"
title="enum rustls::PeerMisbehaved">PeerMisbehaved</a>  
The set of cases where we failed to make a connection because we thought
the peer was misbehaving.

<a href="enum.ProtocolVersion.html" class="enum"
title="enum rustls::ProtocolVersion">ProtocolVersion</a>  
The `ProtocolVersion` TLS protocol enum. Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA. The
`Unknown` item is used when processing unrecognised ordinals.

<a href="enum.Side.html" class="enum" title="enum rustls::Side">Side</a>  
Side of the connection.

<a href="enum.SignatureAlgorithm.html" class="enum"
title="enum rustls::SignatureAlgorithm">SignatureAlgorithm</a>  
The `SignatureAlgorithm` TLS protocol enum. Values in this enum are
taken from the various RFCs covering TLS, and are listed by IANA. The
`Unknown` item is used when processing unrecognised ordinals.

<a href="enum.SignatureScheme.html" class="enum"
title="enum rustls::SignatureScheme">SignatureScheme</a>  
The `SignatureScheme` TLS protocol enum. Values in this enum are taken
from the various RFCs covering TLS, and are listed by IANA. The
`Unknown` item is used when processing unrecognised ordinals.

<a href="enum.SupportedCipherSuite.html" class="enum"
title="enum rustls::SupportedCipherSuite">SupportedCipherSuite</a>  
A cipher suite supported by rustls.

# Statics<a href="#statics" class="anchor">§</a>

<a href="static.ALL_VERSIONS.html" class="static"
title="static rustls::ALL_VERSIONS">ALL_VERSIONS</a>  
A list of all the protocol versions supported by rustls.

<a href="static.DEFAULT_VERSIONS.html" class="static"
title="static rustls::DEFAULT_VERSIONS">DEFAULT_VERSIONS</a>  
The version configuration that an application should use by default.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ConfigSide.html" class="trait"
title="trait rustls::ConfigSide">ConfigSide</a>  
Helper trait to abstract
[`ConfigBuilder`](struct.ConfigBuilder.html "struct rustls::ConfigBuilder")
over building a
[`ClientConfig`](client/struct.ClientConfig.html "struct rustls::client::ClientConfig")
or
[`ServerConfig`](server/struct.ServerConfig.html "struct rustls::server::ServerConfig").

<a href="trait.KeyLog.html" class="trait"
title="trait rustls::KeyLog">KeyLog</a>  
This trait represents the ability to do something useful with key
material, such as logging it to a file for debugging.

<a href="trait.SideData.html" class="trait"
title="trait rustls::SideData">SideData</a>  
Data specific to the peer’s side (client or server).
