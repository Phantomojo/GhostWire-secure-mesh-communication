Crate elliptic\_curve Copy item path

<span class="sub-heading"><a href="../src/elliptic_curve/lib.rs.html#1-196" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-elliptic-curve-traits" class="doc-anchor">§</a>RustCrypto: Elliptic Curve Traits

[![crate](https://buildstats.info/crate/elliptic-curve)](https://crates.io/crates/elliptic-curve)
[![Docs](https://docs.rs/elliptic-curve/badge.svg)](https://docs.rs/elliptic-curve/)
[![Build
Status](https://github.com/RustCrypto/traits/actions/workflows/elliptic-curve.yml/badge.svg)](https://github.com/RustCrypto/traits/actions/workflows/elliptic-curve.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.65+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260040-elliptic-curves)

General purpose Elliptic Curve Cryptography (ECC) support, including
types and traits for representing various elliptic curve forms, scalars,
points, and public/secret keys composed thereof.

[Documentation](https://docs.rs/elliptic-curve/)

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

Requires Rust **1.65** or higher.

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

## <a href="#usage" class="doc-anchor">§</a>Usage

This crate provides traits for describing elliptic curves, along with
types which are generic over elliptic curves which can be used as the
basis of curve-agnostic code.

It’s intended to be used with the following concrete elliptic curve
implementations from the
[`RustCrypto/elliptic-curves`](https://github.com/RustCrypto/elliptic-curves)
project:

-   [`bp256`](https://github.com/RustCrypto/elliptic-curves/tree/master/bp256):
    brainpoolP256r1 and brainpoolP256t1
-   [`bp384`](https://github.com/RustCrypto/elliptic-curves/tree/master/bp384):
    brainpoolP384r1 and brainpoolP384t1
-   [`k256`](https://github.com/RustCrypto/elliptic-curves/tree/master/k256):
    secp256k1 a.k.a. K-256
-   [`p224`](https://github.com/RustCrypto/elliptic-curves/tree/master/p224):
    NIST P-224 a.k.a. secp224r1
-   [`p256`](https://github.com/RustCrypto/elliptic-curves/tree/master/p256):
    NIST P-256 a.k.a secp256r1, prime256v1
-   [`p384`](https://github.com/RustCrypto/elliptic-curves/tree/master/p384):
    NIST P-384 a.k.a. secp384r1
-   [`p521`](https://github.com/RustCrypto/elliptic-curves/tree/master/p521):
    NIST P-521 a.k.a. secp521r1

The
[`ecdsa`](https://github.com/RustCrypto/signatures/tree/master/ecdsa)
crate provides a generic implementation of the Elliptic Curve Digital
Signature Algorithm which can be used with any of the above crates,
either via an external ECDSA implementation, or using native curve
arithmetic where applicable.

## <a href="#type-conversions" class="doc-anchor">§</a>Type conversions

The following chart illustrates the various conversions possible between
the various types defined by this crate.

![Type Conversion
Map](https://raw.githubusercontent.com/RustCrypto/media/master/img/elliptic-curve/type-transforms.svg)

## <a href="#serde-support" class="doc-anchor">§</a>`serde` support

When the `serde` feature of this crate is enabled, `Serialize` and
`Deserialize` impls are provided for the following types:

-   \[`JwkEcKey`\]
-   [`PublicKey`](struct.PublicKey.html "struct elliptic_curve::PublicKey")
-   [`ScalarPrimitive`](scalar/struct.ScalarPrimitive.html "struct elliptic_curve::scalar::ScalarPrimitive")

Please see type-specific documentation for more information.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::scalar::`<a href="scalar/struct.ScalarPrimitive.html" class="struct"
title="struct elliptic_curve::scalar::ScalarPrimitive"><code>ScalarPrimitive</code></a>`;`

`pub use crate::point::`<a href="point/type.AffinePoint.html" class="type"
title="type elliptic_curve::point::AffinePoint"><code>AffinePoint</code></a>`;`

`pub use crate::point::`<a href="point/trait.BatchNormalize.html" class="trait"
title="trait elliptic_curve::point::BatchNormalize"><code>BatchNormalize</code></a>`;`

`pub use crate::point::`<a href="point/type.ProjectivePoint.html" class="type"
title="type elliptic_curve::point::ProjectivePoint"><code>ProjectivePoint</code></a>`;`

`pub use crate::scalar::`<a href="scalar/struct.NonZeroScalar.html" class="struct"
title="struct elliptic_curve::scalar::NonZeroScalar"><code>NonZeroScalar</code></a>`;`

`pub use crate::scalar::`<a href="scalar/type.Scalar.html" class="type"
title="type elliptic_curve::scalar::Scalar"><code>Scalar</code></a>`;`

`pub use `<a href="../crypto_bigint/index.html" class="mod"
title="mod crypto_bigint"><code>crypto_bigint</code></a>` as bigint;`

`pub use `<a href="../generic_array/index.html" class="mod"
title="mod generic_array"><code>generic_array</code></a>`;`

`pub use `<a href="../rand_core/index.html" class="mod"
title="mod rand_core"><code>rand_core</code></a>`;`

`pub use `<a href="../subtle/index.html" class="mod"
title="mod subtle"><code>subtle</code></a>`;`

`pub use `<a href="../zeroize/index.html" class="mod"
title="mod zeroize"><code>zeroize</code></a>`;`

`pub use `<a href="../ff/index.html" class="mod"
title="mod ff"><code>ff</code></a>`;`

`pub use `<a href="../group/index.html" class="mod"
title="mod group"><code>group</code></a>`;`

`pub use `<a href="../pkcs8/index.html" class="mod"
title="mod pkcs8"><code>pkcs8</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="consts/index.html" class="mod"
title="mod elliptic_curve::consts">consts</a>  
<a href="ecdh/index.html" class="mod"
title="mod elliptic_curve::ecdh">ecdh</a>  
Elliptic Curve Diffie-Hellman Support.

<a href="ops/index.html" class="mod"
title="mod elliptic_curve::ops">ops</a>  
Traits for arithmetic operations on elliptic curve field elements.

<a href="point/index.html" class="mod"
title="mod elliptic_curve::point">point</a>  
Traits for elliptic curve points.

<a href="scalar/index.html" class="mod"
title="mod elliptic_curve::scalar">scalar</a>  
Scalar types.

<a href="sec1/index.html" class="mod"
title="mod elliptic_curve::sec1">sec1</a>  
Support for SEC1 elliptic curve encoding formats.

<a href="weierstrass/index.html" class="mod"
title="mod elliptic_curve::weierstrass">weierstrass</a>  
Complete projective formulas for prime order elliptic curves as
described in [Renes-Costello-Batina
2015](https://eprint.iacr.org/2015/1060).

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct elliptic_curve::Error">Error</a>  
Elliptic curve errors.

<a href="struct.PublicKey.html" class="struct"
title="struct elliptic_curve::PublicKey">PublicKey</a>  
Elliptic curve public keys.

<a href="struct.SecretKey.html" class="struct"
title="struct elliptic_curve::SecretKey">SecretKey</a>  
Elliptic curve secret keys.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ALGORITHM_OID.html" class="constant"
title="constant elliptic_curve::ALGORITHM_OID">ALGORITHM_OID</a>  
Algorithm
[`ObjectIdentifier`](../const_oid/struct.ObjectIdentifier.html "struct const_oid::ObjectIdentifier")
for elliptic curve public key cryptography (`id-ecPublicKey`).

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Curve.html" class="trait"
title="trait elliptic_curve::Curve">Curve</a>  
Elliptic curve.

<a href="trait.CurveArithmetic.html" class="trait"
title="trait elliptic_curve::CurveArithmetic">CurveArithmetic</a>  
Elliptic curve with an arithmetic implementation.

<a href="trait.Field.html" class="trait"
title="trait elliptic_curve::Field">Field</a>  
This trait represents an element of a field.

<a href="trait.FieldBytesEncoding.html" class="trait"
title="trait elliptic_curve::FieldBytesEncoding">FieldBytesEncoding</a>  
Trait for decoding/encoding `Curve::Uint` from/to
[`FieldBytes`](type.FieldBytes.html "type elliptic_curve::FieldBytes")
using curve-specific rules.

<a href="trait.Group.html" class="trait"
title="trait elliptic_curve::Group">Group</a>  
This trait represents an element of a cryptographic group.

<a href="trait.PrimeCurve.html" class="trait"
title="trait elliptic_curve::PrimeCurve">PrimeCurve</a>  
Marker trait for elliptic curves with prime order.

<a href="trait.PrimeCurveArithmetic.html" class="trait"
title="trait elliptic_curve::PrimeCurveArithmetic">PrimeCurveArithmetic</a>  
Prime order elliptic curve with projective arithmetic implementation.

<a href="trait.PrimeField.html" class="trait"
title="trait elliptic_curve::PrimeField">PrimeField</a>  
This represents an element of a non-binary prime field.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.FieldBytes.html" class="type"
title="type elliptic_curve::FieldBytes">FieldBytes</a>  
Byte representation of a base/scalar field element of a given curve.

<a href="type.FieldBytesSize.html" class="type"
title="type elliptic_curve::FieldBytesSize">FieldBytesSize</a>  
Size of serialized field elements of this elliptic curve.

<a href="type.Result.html" class="type"
title="type elliptic_curve::Result">Result</a>  
Result type with the `elliptic-curve` crate’s
[`Error`](struct.Error.html "struct elliptic_curve::Error") type.
