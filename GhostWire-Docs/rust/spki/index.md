Crate spki Copy item path

<span class="sub-heading"><a href="../src/spki/lib.rs.html#1-71" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-x509-subject-public-key-info-spki"
class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto): X.509 Subject Public Key Info (SPKI)

[![crate](https://buildstats.info/crate/spki)](https://crates.io/crates/spki)
[![Docs](https://docs.rs/spki/badge.svg)](https://docs.rs/spki/)
[![Build
Status](https://github.com/RustCrypto/formats/actions/workflows/spki.yml/badge.svg)](https://github.com/RustCrypto/formats/actions/workflows/spki.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.65+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats)

[X.509](https://en.wikipedia.org/wiki/X.509) Subject Public Key Info
types describing public keys as well as their associated
AlgorithmIdentifiers (i.e. OIDs).

Specified in [RFC 5280 §
4.1](https://tools.ietf.org/html/rfc5280#section-4.1).

[Documentation](https://docs.rs/spki/)

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

# <a href="#usage" class="doc-anchor">§</a>Usage

The following example demonstrates how to use an OID as the `parameters`
of an
[`AlgorithmIdentifier`](struct.AlgorithmIdentifier.html "struct spki::AlgorithmIdentifier").

Borrow the
[`ObjectIdentifier`](struct.ObjectIdentifier.html "struct spki::ObjectIdentifier")
first then use
[`der::AnyRef::from`](../der/asn1/any/struct.AnyRef.html#method.from "associated function der::asn1::any::AnyRef::from")
or `.into()`:

``` {.rust .rust-example-rendered}
use spki::{AlgorithmIdentifier, ObjectIdentifier};

let alg_oid = "1.2.840.10045.2.1".parse::<ObjectIdentifier>().unwrap();
let params_oid = "1.2.840.10045.3.1.7".parse::<ObjectIdentifier>().unwrap();

let alg_id = AlgorithmIdentifier {
    oid: alg_oid,
    parameters: Some(params_oid)
};
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../der/index.html" class="mod"
title="mod der"><code>der</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AlgorithmIdentifier.html" class="struct"
title="struct spki::AlgorithmIdentifier">AlgorithmIdentifier</a>  
X.509 `AlgorithmIdentifier` as defined in [RFC 5280 Section
4.1.1.2](https://tools.ietf.org/html/rfc5280#section-4.1.1.2).

<a href="struct.Document.html" class="struct"
title="struct spki::Document">Document</a>  
ASN.1 DER-encoded document.

<a href="struct.ObjectIdentifier.html" class="struct"
title="struct spki::ObjectIdentifier">ObjectIdentifier</a>  
Object identifier (OID).

<a href="struct.SubjectPublicKeyInfo.html" class="struct"
title="struct spki::SubjectPublicKeyInfo">SubjectPublicKeyInfo</a>  
X.509 `SubjectPublicKeyInfo` (SPKI) as defined in [RFC 5280 §
4.1.2.7](https://tools.ietf.org/html/rfc5280#section-4.1.2.7).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum spki::Error">Error</a>  
Error type

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AssociatedAlgorithmIdentifier.html" class="trait"
title="trait spki::AssociatedAlgorithmIdentifier">AssociatedAlgorithmIdentifier</a>  
Returns `AlgorithmIdentifier` associated with the structure.

<a href="trait.DecodePublicKey.html" class="trait"
title="trait spki::DecodePublicKey">DecodePublicKey</a>  
Parse a public key object from an encoded SPKI document.

<a href="trait.DynAssociatedAlgorithmIdentifier.html" class="trait"
title="trait spki::DynAssociatedAlgorithmIdentifier">DynAssociatedAlgorithmIdentifier</a>  
Returns `AlgorithmIdentifier` associated with the structure.

<a href="trait.DynSignatureAlgorithmIdentifier.html" class="trait"
title="trait spki::DynSignatureAlgorithmIdentifier">DynSignatureAlgorithmIdentifier</a>  
Returns `AlgorithmIdentifier` associated with the signature system.

<a href="trait.EncodePublicKey.html" class="trait"
title="trait spki::EncodePublicKey">EncodePublicKey</a>  
Serialize a public key object to a SPKI-encoded document.

<a href="trait.SignatureAlgorithmIdentifier.html" class="trait"
title="trait spki::SignatureAlgorithmIdentifier">SignatureAlgorithmIdentifier</a>  
Returns `AlgorithmIdentifier` associated with the signature system.

<a href="trait.SignatureBitStringEncoding.html" class="trait"
title="trait spki::SignatureBitStringEncoding">SignatureBitStringEncoding</a>  
Returns the `BitString` encoding of the signature.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.AlgorithmIdentifierOwned.html" class="type"
title="type spki::AlgorithmIdentifierOwned">AlgorithmIdentifierOwned</a>  
`AlgorithmIdentifier` reference which has `Any` parameters.

<a href="type.AlgorithmIdentifierRef.html" class="type"
title="type spki::AlgorithmIdentifierRef">AlgorithmIdentifierRef</a>  
`AlgorithmIdentifier` reference which has `AnyRef` parameters.

<a href="type.AlgorithmIdentifierWithOid.html" class="type"
title="type spki::AlgorithmIdentifierWithOid">AlgorithmIdentifierWithOid</a>  
`AlgorithmIdentifier` with `ObjectIdentifier` parameters.

<a href="type.Result.html" class="type"
title="type spki::Result">Result</a>  
Result type with `spki` crate’s
[`Error`](enum.Error.html "enum spki::Error") type.

<a href="type.SubjectPublicKeyInfoOwned.html" class="type"
title="type spki::SubjectPublicKeyInfoOwned">SubjectPublicKeyInfoOwned</a>  
[`SubjectPublicKeyInfo`](struct.SubjectPublicKeyInfo.html "struct spki::SubjectPublicKeyInfo")
with
[`Any`](../der/asn1/any/allocating/struct.Any.html "struct der::asn1::any::allocating::Any")
algorithm parameters, and
[`BitString`](../der/asn1/bit_string/allocating/struct.BitString.html "struct der::asn1::bit_string::allocating::BitString")
params.

<a href="type.SubjectPublicKeyInfoRef.html" class="type"
title="type spki::SubjectPublicKeyInfoRef">SubjectPublicKeyInfoRef</a>  
[`SubjectPublicKeyInfo`](struct.SubjectPublicKeyInfo.html "struct spki::SubjectPublicKeyInfo")
with
[`AnyRef`](../der/asn1/any/struct.AnyRef.html "struct der::asn1::any::AnyRef")
algorithm parameters, and
[`BitStringRef`](../der/asn1/bit_string/struct.BitStringRef.html "struct der::asn1::bit_string::BitStringRef")
params.
