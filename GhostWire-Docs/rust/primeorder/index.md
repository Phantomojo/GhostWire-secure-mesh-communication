Crate primeorder Copy item path

<span class="sub-heading"><a href="../src/primeorder/lib.rs.html#1-53" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-prime-order-elliptic-curve-formulas"
class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto/): Prime Order Elliptic Curve Formulas

[![crate](https://buildstats.info/crate/primeorder)](https://crates.io/crates/primeorder)
[![Docs](https://docs.rs/primeorder/badge.svg)](https://docs.rs/primeorder/)
[![Build
Status](https://github.com/RustCrypto/elliptic-curves/actions/workflows/primeorder.yml/badge.svg)](https://github.com/RustCrypto/elliptic-curves/actions/workflows/primeorder.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.65+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260040-elliptic-curves)

Pure Rust implementation of complete addition formulas for prime order
elliptic curves ([Renes-Costello-Batina
2015](https://eprint.iacr.org/2015/1060)). Generic over field elements
and curve equation coefficients.

[Documentation](https://docs.rs/primeorder/)

## <a href="#about" class="doc-anchor">§</a>About

This crate provides a generic implementation of complete formulas for
prime order elliptic curves which are defined by the short [Weierstrass
equation](https://crypto.stanford.edu/pbc/notes/elliptic/weier.html):

``` language-text
y² = x³ + ax + b
```

It’s used to implement the following elliptic curves:

-   [`p192`](https://github.com/RustCrypto/elliptic-curves/tree/master/p192):
    NIST P-192
-   [`p224`](https://github.com/RustCrypto/elliptic-curves/tree/master/p224):
    NIST P-224
-   [`p256`](https://github.com/RustCrypto/elliptic-curves/tree/master/p256):
    NIST P-256
-   [`p384`](https://github.com/RustCrypto/elliptic-curves/tree/master/p384):
    NIST P-384
-   [`p521`](https://github.com/RustCrypto/elliptic-curves/tree/master/p384):
    NIST P-521
-   [`sm2`](https://github.com/RustCrypto/elliptic-curves/tree/master/sm2):
    ShangMi 2

## <a href="#-security-warning" class="doc-anchor">§</a>⚠️ Security Warning

The elliptic curve arithmetic contained in this crate has never been
independently audited!

This crate has been designed with the goal of ensuring that
secret-dependent operations are performed in constant time (using the
`subtle` crate and constant-time formulas). However, it has not been
thoroughly assessed to ensure that generated assembly is constant time
on common CPU architectures.

USE AT YOUR OWN RISK!

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

Rust **1.65** or higher.

Minimum supported Rust version can be changed in the future, but it will
be done with a minor version bump.

## <a href="#semver-policy" class="doc-anchor">§</a>SemVer Policy

-   All on-by-default features of this library are covered by SemVer
-   MSRV is considered exempt from SemVer as noted above

## <a href="#license" class="doc-anchor">§</a>License

All crates licensed under either of:

-   [Apache License, Version
    2.0](http://www.apache.org/licenses/LICENSE-2.0)
-   [MIT license](http://opensource.org/licenses/MIT)

at your option.

### <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../elliptic_curve/index.html" class="mod"
title="mod elliptic_curve"><code>elliptic_curve</code></a>`;`

`pub use elliptic_curve::`<a href="../generic_array/index.html" class="mod"
title="mod generic_array"><code>generic_array</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="point_arithmetic/index.html" class="mod"
title="mod primeorder::point_arithmetic">point_arithmetic</a>  
Point arithmetic implementation optimised for different curve equations

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.impl_bernstein_yang_invert.html" class="macro"
title="macro primeorder::impl_bernstein_yang_invert">impl_bernstein_yang_invert</a>  
Implement Bernstein-Yang field element inversion.

<a href="macro.impl_field_identity_tests.html" class="macro"
title="macro primeorder::impl_field_identity_tests">impl_field_identity_tests</a>  
Implement field element identity tests.

<a href="macro.impl_field_invert_tests.html" class="macro"
title="macro primeorder::impl_field_invert_tests">impl_field_invert_tests</a>  
Implement field element inversion tests.

<a href="macro.impl_field_op.html" class="macro"
title="macro primeorder::impl_field_op">impl_field_op</a>  
Emit impls for a `core::ops` trait for all combinations of reference
types, which thunk to the given function.

<a href="macro.impl_field_sqrt_tests.html" class="macro"
title="macro primeorder::impl_field_sqrt_tests">impl_field_sqrt_tests</a>  
Implement field element square root tests.

<a href="macro.impl_mont_field_element.html" class="macro"
title="macro primeorder::impl_mont_field_element">impl_mont_field_element</a>  
Implements a field element type whose internal representation is in
Montgomery form, providing a combination of trait impls and inherent
impls which are `const fn` where possible.

<a href="macro.impl_mont_field_element_arithmetic.html" class="macro"
title="macro primeorder::impl_mont_field_element_arithmetic">impl_mont_field_element_arithmetic</a>  
Add arithmetic impls to the given field element.

<a href="macro.impl_primefield_tests.html" class="macro"
title="macro primeorder::impl_primefield_tests">impl_primefield_tests</a>  
Implement tests for the `PrimeField` trait.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AffinePoint.html" class="struct"
title="struct primeorder::AffinePoint">AffinePoint</a>  
Point on a Weierstrass curve in affine coordinates.

<a href="struct.ProjectivePoint.html" class="struct"
title="struct primeorder::ProjectivePoint">ProjectivePoint</a>  
Point on a Weierstrass curve in projective coordinates.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Double.html" class="trait"
title="trait primeorder::Double">Double</a>  
Double a point (i.e. add it to itself)

<a href="trait.Field.html" class="trait"
title="trait primeorder::Field">Field</a>  
This trait represents an element of a field.

<a href="trait.PrimeCurve.html" class="trait"
title="trait primeorder::PrimeCurve">PrimeCurve</a>  
Marker trait for elliptic curves with prime order.

<a href="trait.PrimeCurveParams.html" class="trait"
title="trait primeorder::PrimeCurveParams">PrimeCurveParams</a>  
Parameters for elliptic curves of prime order which can be described by
the short Weierstrass equation.

<a href="trait.PrimeField.html" class="trait"
title="trait primeorder::PrimeField">PrimeField</a>  
This represents an element of a non-binary prime field.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.FieldBytes.html" class="type"
title="type primeorder::FieldBytes">FieldBytes</a>  
Byte representation of a base/scalar field element of a given curve.
