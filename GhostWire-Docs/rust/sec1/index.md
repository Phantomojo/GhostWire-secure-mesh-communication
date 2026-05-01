Crate sec1 Copy item path

<span class="sub-heading"><a href="../src/sec1/lib.rs.html#1-77" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-sec1-elliptic-curve-cryptography-formats"
class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto): SEC1 Elliptic Curve Cryptography Formats

[![crate](https://buildstats.info/crate/sec1)](https://crates.io/crates/sec1)
[![Docs](https://docs.rs/sec1/badge.svg)](https://docs.rs/sec1/)
[![Build
Status](https://github.com/RustCrypto/formats/workflows/sec1/badge.svg?branch=master&event=push)](https://github.com/RustCrypto/formats/actions)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.65+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats)

[Documentation](https://docs.rs/sec1/)

## <a href="#about" class="doc-anchor">§</a>About

Pure Rust implementation of [SEC1: Elliptic Curve
Cryptography](https://www.secg.org/sec1-v2.pdf) encoding formats
including ASN.1 DER-serialized private keys (also described in
[RFC5915](https://datatracker.ietf.org/doc/html/rfc5915)) as well as the
`Elliptic-Curve-Point-to-Octet-String` and
`Octet-String-to-Elliptic-Curve-Point` encoding algorithms.

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

This crate requires **Rust 1.65** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a
minor version bump.

## <a href="#license" class="doc-anchor">§</a>License

Licensed under either of:

-   [Apache License, Version
    2.0](http://www.apache.org/licenses/LICENSE-2.0)
-   [MIT license](http://opensource.org/licenses/MIT)

at your option.

### <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

## <a href="#serde-support" class="doc-anchor">§</a>`serde` support

When the `serde` feature of this crate is enabled, the
[`EncodedPoint`](point/struct.EncodedPoint.html "struct sec1::point::EncodedPoint")
type receives impls of \[`serde::Serialize`\] and
\[`serde::Deserialize`\].

Additionally, when both the `alloc` and `serde` features are enabled,
the serializers/deserializers will autodetect if a “human friendly”
textual encoding is being used, and if so encode the points as
hexadecimal.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::point::`<a href="point/struct.EncodedPoint.html" class="struct"
title="struct sec1::point::EncodedPoint"><code>EncodedPoint</code></a>`;`

`pub use `<a href="../der/index.html" class="mod"
title="mod der"><code>der</code></a>`;`

`pub use der::`<a href="../pem_rfc7468/index.html" class="mod"
title="mod pem_rfc7468"><code>pem</code></a>`;`

`pub use `<a href="../pkcs8/index.html" class="mod"
title="mod pkcs8"><code>pkcs8</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="consts/index.html" class="mod"
title="mod sec1::consts">consts</a>  
<a href="point/index.html" class="mod" title="mod sec1::point">point</a>  
Support for the SEC1 `Elliptic-Curve-Point-to-Octet-String` and
`Octet-String-to-Elliptic-Curve-Point` encoding algorithms.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.EcPrivateKey.html" class="struct"
title="struct sec1::EcPrivateKey">EcPrivateKey</a>  
SEC1 elliptic curve private key.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.EcParameters.html" class="enum"
title="enum sec1::EcParameters">EcParameters</a>  
Elliptic curve parameters as described in [RFC5480 Section
2.1.1](https://datatracker.ietf.org/doc/html/rfc5480#section-2.1.1):

<a href="enum.Error.html" class="enum"
title="enum sec1::Error">Error</a>  
Error type

<a href="enum.LineEnding.html" class="enum"
title="enum sec1::LineEnding">LineEnding</a>  
Line endings: variants of newline characters that can be used with
Base64.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ALGORITHM_OID.html" class="constant"
title="constant sec1::ALGORITHM_OID">ALGORITHM_OID</a>  
Algorithm
[`ObjectIdentifier`](../const_oid/struct.ObjectIdentifier.html "struct const_oid::ObjectIdentifier")
for elliptic curve public key cryptography (`id-ecPublicKey`).

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.DecodeEcPrivateKey.html" class="trait"
title="trait sec1::DecodeEcPrivateKey">DecodeEcPrivateKey</a>  
Parse an
[`EcPrivateKey`](struct.EcPrivateKey.html "struct sec1::EcPrivateKey")
from a SEC1-encoded document.

<a href="trait.EncodeEcPrivateKey.html" class="trait"
title="trait sec1::EncodeEcPrivateKey">EncodeEcPrivateKey</a>  
Serialize a
[`EcPrivateKey`](struct.EcPrivateKey.html "struct sec1::EcPrivateKey")
to a SEC1 encoded document.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type sec1::Result">Result</a>  
Result type with `sec1` crate’s
[`Error`](enum.Error.html "enum sec1::Error") type.
