Crate ecdsa Copy item path

<span class="sub-heading"><a href="../src/ecdsa/lib.rs.html#1-709" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-ecdsa" class="doc-anchor">§</a>[RustCrypto](https://github.com/RustCrypto): ECDSA

[![crate](https://buildstats.info/crate/ecdsa)](https://crates.io/crates/ecdsa)
[![Docs](https://docs.rs/ecdsa/badge.svg)](https://docs.rs/ecdsa/)
[![Build
Status](https://github.com/RustCrypto/signatures/actions/workflows/ecdsa.yml/badge.svg)](https://github.com/RustCrypto/signatures/actions/workflows/ecdsa.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![MSRV](https://img.shields.io/badge/rustc-1.65+-blue.svg) [![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260048-signatures)

[Elliptic Curve Digital Signature Algorithm
(ECDSA)](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm)
as specified in [FIPS
186-4](https://csrc.nist.gov/publications/detail/fips/186/4/final)
(Digital Signature Standard).

[Documentation](https://docs.rs/ecdsa/)

## <a href="#about" class="doc-anchor">§</a>About

This crate provides generic ECDSA support which can be used in the
following ways:

-   Generic implementation of ECDSA usable with the following crates:
    -   [`k256`](https://docs.rs/k256) (secp256k1)
    -   [`p256`](https://docs.rs/p256) (NIST P-256)
    -   [`p384`](https://docs.rs/p384) (NIST P-384)
-   Other crates which provide their own complete implementations of
    ECDSA can also leverage the types from this crate to export ECDSA
    functionality in a generic, interoperable way by leveraging
    [`ecdsa::Signature`](https://docs.rs/ecdsa/latest/ecdsa/struct.Signature.html)
    with the
    [`signature::Signer`](https://docs.rs/signature/latest/signature/trait.Signer.html)
    and
    [`signature::Verifier`](https://docs.rs/signature/latest/signature/trait.Verifier.html)
    traits.

## <a href="#-security-warning" class="doc-anchor">§</a>⚠️ Security Warning

The ECDSA implementation contained in this crate has never been
independently audited for security!

This crate contains a generic implementation of ECDSA which must be
instantiated using a separate crate providing a concrete implementation
of arithmetic for a particular curve. It’s possible timing variability
can exist in concrete curve implementations, and thus this crate’s
security can only be properly assessed for a specific elliptic curve.

USE AT YOUR OWN RISK!

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

This crate requires **Rust 1.65** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a
minor version bump.

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
`Deserialize` impls are provided for the
[`Signature`](struct.Signature.html "struct ecdsa::Signature") and
[`VerifyingKey`](struct.VerifyingKey.html "struct ecdsa::VerifyingKey")
types.

Please see type-specific documentation for more information.

## <a href="#interop" class="doc-anchor">§</a>Interop

Any crates which provide an implementation of ECDSA for a particular
elliptic curve can leverage the types from this crate, along with the
[`k256`](https://docs.rs/k256), [`p256`](https://docs.rs/p256), and/or
[`p384`](https://docs.rs/p384) crates to expose ECDSA functionality in a
generic, interoperable way by leveraging the
[`Signature`](struct.Signature.html "struct ecdsa::Signature") type with
in conjunction with the
[`signature::Signer`](https://docs.rs/signature/latest/signature/trait.Signer.html)
and
[`signature::Verifier`](https://docs.rs/signature/latest/signature/trait.Verifier.html)
traits.

For example, the [`ring-compat`](https://docs.rs/ring-compat) crate
implements the
[`signature::Signer`](https://docs.rs/signature/latest/signature/trait.Signer.html)
and
[`signature::Verifier`](https://docs.rs/signature/latest/signature/trait.Verifier.html)
traits in conjunction with the
[`p256::ecdsa::Signature`](https://docs.rs/p256/latest/p256/ecdsa/type.Signature.html)
and
[`p384::ecdsa::Signature`](https://docs.rs/p384/latest/p384/ecdsa/type.Signature.html)
types to wrap the ECDSA implementations from
[*ring*](https://docs.rs/ring) in a generic, interoperable API.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../elliptic_curve/index.html" class="mod"
title="mod elliptic_curve"><code>elliptic_curve</code></a>`;`

`pub use `<a href="../signature/index.html" class="mod"
title="mod signature"><code>signature</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="der/index.html" class="mod" title="mod ecdsa::der">der</a>  
Support for ASN.1 DER-encoded ECDSA signatures as specified in [RFC5912
Appendix A](https://www.rfc-editor.org/rfc/rfc5912#appendix-A).

<a href="hazmat/index.html" class="mod"
title="mod ecdsa::hazmat">hazmat</a>  
Low-level ECDSA primitives.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct ecdsa::Error">Error</a>  
Signature errors.

<a href="struct.NormalizedSignature.html" class="struct"
title="struct ecdsa::NormalizedSignature">NormalizedSignature</a>  
ECDSA signature with low-S normalization applied.

<a href="struct.RecoveryId.html" class="struct"
title="struct ecdsa::RecoveryId">RecoveryId</a>  
Recovery IDs, a.k.a. “recid”.

<a href="struct.Signature.html" class="struct"
title="struct ecdsa::Signature">Signature</a>  
ECDSA signature (fixed-size). Generic over elliptic curve types.

<a href="struct.SignatureWithOid.html" class="struct"
title="struct ecdsa::SignatureWithOid">SignatureWithOid</a>  
An extended
[`Signature`](struct.Signature.html "struct ecdsa::Signature") type
which is parameterized by an `ObjectIdentifier` which identifies the
ECDSA variant used by a particular signature.

<a href="struct.SigningKey.html" class="struct"
title="struct ecdsa::SigningKey">SigningKey</a>  
ECDSA secret key used for signing. Generic over prime order elliptic
curves (e.g. NIST P-curves)

<a href="struct.VerifyingKey.html" class="struct"
title="struct ecdsa::VerifyingKey">VerifyingKey</a>  
ECDSA public key used for verifying signatures. Generic over prime order
elliptic curves (e.g. NIST P-curves)

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ECDSA_SHA224_OID.html" class="constant"
title="constant ecdsa::ECDSA_SHA224_OID">ECDSA_SHA224_OID</a>  
OID for ECDSA with SHA-224 digests.

<a href="constant.ECDSA_SHA256_OID.html" class="constant"
title="constant ecdsa::ECDSA_SHA256_OID">ECDSA_SHA256_OID</a>  
OID for ECDSA with SHA-256 digests.

<a href="constant.ECDSA_SHA384_OID.html" class="constant"
title="constant ecdsa::ECDSA_SHA384_OID">ECDSA_SHA384_OID</a>  
OID for ECDSA with SHA-384 digests.

<a href="constant.ECDSA_SHA512_OID.html" class="constant"
title="constant ecdsa::ECDSA_SHA512_OID">ECDSA_SHA512_OID</a>  
OID for ECDSA with SHA-512 digests.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.PrimeCurve.html" class="trait"
title="trait ecdsa::PrimeCurve">PrimeCurve</a>  
Marker trait for elliptic curves with prime order.

<a href="trait.SignatureEncoding.html" class="trait"
title="trait ecdsa::SignatureEncoding">SignatureEncoding</a>  
Support for decoding/encoding signatures as bytes.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.EncodedPoint.html" class="type"
title="type ecdsa::EncodedPoint">EncodedPoint</a>  
Encoded elliptic curve point sized appropriately for a given curve.

<a href="type.Result.html" class="type"
title="type ecdsa::Result">Result</a>  
Result type.

<a href="type.SignatureBytes.html" class="type"
title="type ecdsa::SignatureBytes">SignatureBytes</a>  
Fixed-size byte array containing an ECDSA signature

<a href="type.SignatureSize.html" class="type"
title="type ecdsa::SignatureSize">SignatureSize</a>  
Size of a fixed sized signature for the given elliptic curve.
