Crate p256 Copy item path

<span class="sub-heading"><a href="../src/p256/lib.rs.html#1-183" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-nist-p-256-secp256r1-elliptic-curve"
class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto/): NIST P-256 (secp256r1) elliptic curve

[![crate](https://buildstats.info/crate/p256)](https://crates.io/crates/p256)
[![Docs](https://docs.rs/p256/badge.svg)](https://docs.rs/p256/)
[![Build
Status](https://github.com/RustCrypto/elliptic-curves/actions/workflows/p256.yml/badge.svg)](https://github.com/RustCrypto/elliptic-curves/actions/workflows/p256.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.65+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260040-elliptic-curves)

Pure Rust implementation of the NIST P-256 (a.k.a. secp256r1,
prime256v1) elliptic curve with support for ECDH, ECDSA
signing/verification, and general purpose curve arithmetic support
implemented in terms of traits from the
[`elliptic-curve`](https://github.com/RustCrypto/traits/tree/master/elliptic-curve)
crate.

[Documentation](https://docs.rs/p256/)

## <a href="#-security-warning" class="doc-anchor">§</a>⚠️ Security Warning

The elliptic curve arithmetic contained in this crate has never been
independently audited!

This crate has been designed with the goal of ensuring that
secret-dependent operations are performed in constant time (using the
`subtle` crate and constant-time formulas). However, it has not been
thoroughly assessed to ensure that generated assembly is constant time
on common CPU architectures.

USE AT YOUR OWN RISK!

## <a href="#supported-algorithms" class="doc-anchor">§</a>Supported Algorithms

-   [Elliptic Curve Diffie-Hellman
    (ECDH)](https://en.wikipedia.org/wiki/Elliptic-curve_Diffie-Hellman):
    gated under the `ecdh` feature.
-   [Elliptic Curve Digital Signature Algorithm
    (ECDSA)](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm):
    gated under the `ecdsa` feature.

## <a href="#about-nist-p-256" class="doc-anchor">§</a>About NIST P-256

NIST P-256 is a Weierstrass curve specified in [SP
800-186](https://csrc.nist.gov/publications/detail/sp/800-186/final):
Recommendations for Discrete Logarithm-based Cryptography: Elliptic
Curve Domain Parameters.

Also known as prime256v1 (ANSI X9.62) and secp256r1 (SECG), it’s
included in the US National Security Agency’s “Suite B” and is widely
used in protocols like TLS and the associated X.509 PKI.

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

Rust **1.65** or higher.

Minimum supported Rust version can be changed in the future, but it will
be done with a minor version bump.

## <a href="#semver-policy" class="doc-anchor">§</a>SemVer Policy

-   All on-by-default features of this library are covered by SemVer
-   MSRV is considered exempt from SemVer as noted above

## <a href="#license" class="doc-anchor">§</a>License

All crates licensed under either of

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

When the `serde` feature of this crate is enabled, `Serialize` and
`Deserialize` are impl’d for the following types:

-   [`AffinePoint`](type.AffinePoint.html "type p256::AffinePoint")
-   [`Scalar`](struct.Scalar.html "struct p256::Scalar")
-   [`ecdsa::VerifyingKey`](ecdsa/type.VerifyingKey.html "type p256::ecdsa::VerifyingKey")

Please see type-specific documentation for more information.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../elliptic_curve/index.html" class="mod"
title="mod elliptic_curve"><code>elliptic_curve</code></a>`;`

`pub use elliptic_curve::`<a href="../pkcs8/index.html" class="mod"
title="mod pkcs8"><code>pkcs8</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="ecdh/index.html" class="mod" title="mod p256::ecdh">ecdh</a>  
Elliptic Curve Diffie-Hellman (Ephemeral) Support.

<a href="ecdsa/index.html" class="mod" title="mod p256::ecdsa">ecdsa</a>  
Elliptic Curve Digital Signature Algorithm (ECDSA)

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.NistP256.html" class="struct"
title="struct p256::NistP256">NistP256</a>  
NIST P-256 elliptic curve.

<a href="struct.Scalar.html" class="struct"
title="struct p256::Scalar">Scalar</a>  
Scalars are elements in the finite field modulo n.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.AffinePoint.html" class="type"
title="type p256::AffinePoint">AffinePoint</a>  
Elliptic curve point in affine coordinates.

<a href="type.BlindedScalar.html" class="type"
title="type p256::BlindedScalar">BlindedScalar</a>  
Blinded scalar.

<a href="type.CompressedPoint.html" class="type"
title="type p256::CompressedPoint">CompressedPoint</a>  
Compressed SEC1-encoded NIST P-256 curve point.

<a href="type.EncodedPoint.html" class="type"
title="type p256::EncodedPoint">EncodedPoint</a>  
NIST P-256 SEC1 encoded point.

<a href="type.FieldBytes.html" class="type"
title="type p256::FieldBytes">FieldBytes</a>  
NIST P-256 field element serialized as bytes.

<a href="type.NonZeroScalar.html" class="type"
title="type p256::NonZeroScalar">NonZeroScalar</a>  
Non-zero NIST P-256 scalar field element.

<a href="type.ProjectivePoint.html" class="type"
title="type p256::ProjectivePoint">ProjectivePoint</a>  
Elliptic curve point in projective coordinates.

<a href="type.PublicKey.html" class="type"
title="type p256::PublicKey">PublicKey</a>  
NIST P-256 public key.

<a href="type.SecretKey.html" class="type"
title="type p256::SecretKey">SecretKey</a>  
NIST P-256 secret key.

<a href="type.U32.html" class="type" title="type p256::U32">U32</a>  
<a href="type.U256.html" class="type" title="type p256::U256">U256</a>  
256-bit unsigned big integer.
