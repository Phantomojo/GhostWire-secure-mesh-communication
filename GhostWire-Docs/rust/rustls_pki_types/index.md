Crate rustls\_pki\_types Copy item path

<span class="sub-heading"><a href="../src/rustls_pki_types/lib.rs.html#1-1127"
class="src">Source</a> </span>

Expand description

This crate provides types for representing X.509 certificates, keys and
other types as commonly used in the rustls ecosystem. It is intended to
be used by crates that need to work with such X.509 types, such as
[rustls](https://crates.io/crates/rustls),
[rustls-webpki](https://crates.io/crates/rustls-webpki),
[rustls-pemfile](https://crates.io/crates/rustls-pemfile), and others.

Some of these crates used to define their own trivial wrappers around
DER-encoded bytes. However, in order to avoid inconvenient dependency
edges, these were all disconnected. By using a common low-level crate of
types with long-term stable API, we hope to avoid the downsides of
unnecessary dependency edges while providing good interoperability
between crates.

## <a href="#der-and-pem" class="doc-anchor">§</a>DER and PEM

Many of the types defined in this crate represent DER-encoded data. DER
is a binary encoding of the ASN.1 format commonly used in web PKI
specifications. It is a binary encoding, so it is relatively compact
when stored in memory. However, as a binary format, it is not very easy
to work with for humans and in contexts where binary data is
inconvenient. For this reason, many tools and protocols use a
ASCII-based encoding of DER, called PEM. In addition to the
base64-encoded DER, PEM objects are delimited by header and footer lines
which indicate the type of object contained in the PEM blob.

Types here can be created from:

-   DER using (for example)
    [`PrivatePkcs8KeyDer::from()`](struct.PrivatePkcs8KeyDer.html#method.from "associated function rustls_pki_types::PrivatePkcs8KeyDer::from").
-   PEM using (for example)
    [`pem::PemObject::from_pem_slice()`](pem/trait.PemObject.html#method.from_pem_slice "associated function rustls_pki_types::pem::PemObject::from_pem_slice").

The
[`pem::PemObject`](pem/trait.PemObject.html "trait rustls_pki_types::pem::PemObject")
trait contains the full selection of ways to construct these types from
PEM encodings. That includes ways to open and read from a file, from a
slice, or from an `std::io` stream.

There is also a lower-level API that allows a given PEM file to be fully
consumed in one pass, even if it contains different data types: see the
implementation of the
[`pem::PemObject`](pem/trait.PemObject.html "trait rustls_pki_types::pem::PemObject")
trait on the `(pem::SectionKind, Vec<u8>)` tuple.

## <a href="#creating-new-certificates-and-keys" class="doc-anchor">§</a>Creating new certificates and keys

This crate does not provide any functionality for creating new
certificates or keys. However, the [rcgen](https://docs.rs/rcgen) crate
can be used to create new certificates and keys.

## <a href="#cloning-private-keys" class="doc-anchor">§</a>Cloning private keys

This crate intentionally **does not** implement `Clone` on private key
types in order to minimize the exposure of private key data in memory.

If you want to extend the lifetime of a `PrivateKeyDer<'_>`, consider
[`PrivateKeyDer::clone_key()`](https://docs.rs/rustls-pki-types/latest/rustls_pki_types/enum.PrivateKeyDer.html#method.clone_key).
Alternatively since these types are immutable, consider wrapping the
`PrivateKeyDer<'_>` in a
[`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) or an
[`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).

## <a href="#target-wasm32-unknown-unknown-with-the-web-feature"
class="doc-anchor">§</a>Target `wasm32-unknown-unknown` with the `web` feature

[`std::time::SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
is unavailable in `wasm32-unknown-unknown` targets, so calls to
[`UnixTime::now()`](https://docs.rs/rustls-pki-types/latest/rustls_pki_types/struct.UnixTime.html#method.now),
otherwise enabled by the
[`std`](https://docs.rs/crate/rustls-pki-types/latest/features#std)
feature, require building instead with the
[`web`](https://docs.rs/crate/rustls-pki-types/latest/features#web)
feature. It gets time by calling
[`Date.now()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
in the browser.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use alg_id::`<a href="alg_id/struct.AlgorithmIdentifier.html" class="struct"
title="struct rustls_pki_types::alg_id::AlgorithmIdentifier"><code>AlgorithmIdentifier</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="alg_id/index.html" class="mod"
title="mod rustls_pki_types::alg_id">alg_id</a>  
The PKIX
[`AlgorithmIdentifier`](alg_id/struct.AlgorithmIdentifier.html "struct rustls_pki_types::alg_id::AlgorithmIdentifier")
type, and common values.

<a href="pem/index.html" class="mod"
title="mod rustls_pki_types::pem">pem</a>  
Low-level PEM decoding APIs.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AddrParseError.html" class="struct"
title="struct rustls_pki_types::AddrParseError">AddrParseError</a>  
Failure to parse an IP address

<a href="struct.CertificateDer.html" class="struct"
title="struct rustls_pki_types::CertificateDer">CertificateDer</a>  
A DER-encoded X.509 certificate; as specified in RFC 5280

<a href="struct.CertificateRevocationListDer.html" class="struct"
title="struct rustls_pki_types::CertificateRevocationListDer">CertificateRevocationListDer</a>  
A Certificate Revocation List; as specified in RFC 5280

<a href="struct.CertificateSigningRequestDer.html" class="struct"
title="struct rustls_pki_types::CertificateSigningRequestDer">CertificateSigningRequestDer</a>  
A Certificate Signing Request; as specified in RFC 2986

<a href="struct.Der.html" class="struct"
title="struct rustls_pki_types::Der">Der</a>  
DER-encoded data, either owned or borrowed

<a href="struct.DnsName.html" class="struct"
title="struct rustls_pki_types::DnsName">DnsName</a>  
A type which encapsulates a string (borrowed or owned) that is a
syntactically valid DNS name.

<a href="struct.EchConfigListBytes.html" class="struct"
title="struct rustls_pki_types::EchConfigListBytes">EchConfigListBytes</a>  
A TLS-encoded Encrypted Client Hello (ECH) configuration list
(`ECHConfigList`); as specified in [draft-ietf-tls-esni-18
§4](https://datatracker.ietf.org/doc/html/draft-ietf-tls-esni-18#section-4)

<a href="struct.InvalidDnsNameError.html" class="struct"
title="struct rustls_pki_types::InvalidDnsNameError">InvalidDnsNameError</a>  
The provided input could not be parsed because it is not a
syntactically-valid DNS Name.

<a href="struct.InvalidSignature.html" class="struct"
title="struct rustls_pki_types::InvalidSignature">InvalidSignature</a>  
A detail-less error when a signature is not valid.

<a href="struct.Ipv4Addr.html" class="struct"
title="struct rustls_pki_types::Ipv4Addr">Ipv4Addr</a>  
`no_std` implementation of `std::net::Ipv4Addr`.

<a href="struct.Ipv6Addr.html" class="struct"
title="struct rustls_pki_types::Ipv6Addr">Ipv6Addr</a>  
`no_std` implementation of `std::net::Ipv6Addr`.

<a href="struct.PrivatePkcs1KeyDer.html" class="struct"
title="struct rustls_pki_types::PrivatePkcs1KeyDer">PrivatePkcs1KeyDer</a>  
A DER-encoded plaintext RSA private key; as specified in PKCS#1/RFC 3447

<a href="struct.PrivatePkcs8KeyDer.html" class="struct"
title="struct rustls_pki_types::PrivatePkcs8KeyDer">PrivatePkcs8KeyDer</a>  
A DER-encoded plaintext private key; as specified in PKCS#8/RFC 5958

<a href="struct.PrivateSec1KeyDer.html" class="struct"
title="struct rustls_pki_types::PrivateSec1KeyDer">PrivateSec1KeyDer</a>  
A Sec1-encoded plaintext private key; as specified in RFC 5915

<a href="struct.SubjectPublicKeyInfoDer.html" class="struct"
title="struct rustls_pki_types::SubjectPublicKeyInfoDer">SubjectPublicKeyInfoDer</a>  
A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.

<a href="struct.TrustAnchor.html" class="struct"
title="struct rustls_pki_types::TrustAnchor">TrustAnchor</a>  
A trust anchor (a.k.a. root CA)

<a href="struct.UnixTime.html" class="struct"
title="struct rustls_pki_types::UnixTime">UnixTime</a>  
A timestamp, tracking the number of non-leap seconds since the Unix
epoch.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.IpAddr.html" class="enum"
title="enum rustls_pki_types::IpAddr">IpAddr</a>  
`no_std` implementation of `std::net::IpAddr`.

<a href="enum.PrivateKeyDer.html" class="enum"
title="enum rustls_pki_types::PrivateKeyDer">PrivateKeyDer</a>  
A DER-encoded X.509 private key, in one of several formats

<a href="enum.ServerName.html" class="enum"
title="enum rustls_pki_types::ServerName">ServerName</a>  
Encodes ways a client can know the expected name of the server.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.SignatureVerificationAlgorithm.html" class="trait"
title="trait rustls_pki_types::SignatureVerificationAlgorithm">SignatureVerificationAlgorithm</a>  
An abstract signature verification algorithm.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.SubjectPublicKeyInfo.html" class="type"
title="type rustls_pki_types::SubjectPublicKeyInfo">SubjectPublicKeyInfo</a><span class="stab deprecated" title="">Deprecated</span>  
A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280.
