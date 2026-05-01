Crate der Copy item path

<span class="sub-heading"><a href="../src/der/lib.rs.html#1-402" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-asn1-der" class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto): ASN.1 DER

[![Crate](https://buildstats.info/crate/der)](https://crates.io/crates/der)
[![Docs](https://docs.rs/der/badge.svg)](https://docs.rs/der/) [![Build
Status](https://github.com/RustCrypto/formats/actions/workflows/der.yml/badge.svg)](https://github.com/RustCrypto/formats/actions/workflows/der.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.65+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats)

Pure Rust embedded-friendly implementation of the Distinguished Encoding
Rules (DER) for Abstract Syntax Notation One (ASN.1) as described in ITU
X.690.

[Documentation](https://docs.rs/der/)

## <a href="#about" class="doc-anchor">§</a>About

This crate provides a `no_std`-friendly implementation of a subset of
ASN.1 DER necessary for decoding/encoding the following
cryptography-related formats implemented as crates maintained by the
[RustCrypto](https://github.com/rustcrypto) project:

-   [`pkcs1`](https://github.com/RustCrypto/formats/tree/master/pkcs1):
    RSA Cryptography Specifications
-   [`pkcs5`](https://github.com/RustCrypto/formats/tree/master/pkcs5):
    Password-Based Cryptography Specification
-   [`pkcs7`](https://github.com/RustCrypto/formats/tree/master/pkcs7):
    Cryptographic Message Syntax
-   [`pkcs8`](https://github.com/RustCrypto/formats/tree/master/pkcs8):
    Private-Key Information Syntax Specification
-   [`pkcs10`](https://github.com/RustCrypto/formats/tree/master/pkcs10):
    Certification Request Syntax Specification
-   [`sec1`](https://github.com/RustCrypto/formats/tree/master/sec1):
    Elliptic Curve Cryptography
-   [`spki`](https://github.com/RustCrypto/formats/tree/master/spki):
    X.509 Subject Public Key Info
-   [`x501`](https://github.com/RustCrypto/formats/tree/master/x501):
    Directory Services Types
-   [`x509`](https://github.com/RustCrypto/formats/tree/master/x509):
    Public Key Infrastructure Certificate

The core implementation avoids any heap usage (with convenience methods
that allocate gated under the off-by-default `alloc` feature).

The DER decoder in this crate performs checks to ensure that the input
document is in canonical form, and will return errors if non-canonical
productions are encountered. There is currently no way to disable these
checks.

### <a href="#features" class="doc-anchor">§</a>Features

-   Rich support for ASN.1 types used by PKCS/PKIX documents
-   Performs DER canonicalization checks at decoding time
-   `no_std` friendly: supports “heapless” usage
-   Optionally supports `alloc` and `std` if desired
-   No hard dependencies! Self-contained implementation with optional
    integrations with the following crates, all of which are `no_std`
    friendly:
    -   `const-oid`: const-friendly OID implementation
    -   `pem-rfc7468`: PKCS/PKIX-flavored PEM library with constant-time
        decoder/encoders
    -   `time` crate: date/time library

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

## <a href="#decode-and-encode-traits" class="doc-anchor">§</a>[`Decode`](trait.Decode.html "trait der::Decode") and [`Encode`](trait.Encode.html "trait der::Encode") traits

The [`Decode`](trait.Decode.html "trait der::Decode") and
[`Encode`](trait.Encode.html "trait der::Encode") traits provide the
decoding/encoding API respectively, and are designed to work in
conjunction with concrete ASN.1 types, including all types which impl
the [`Sequence`](asn1/trait.Sequence.html "trait der::asn1::Sequence")
trait.

The traits are impl’d for the following Rust core types:

-   `()`: ASN.1 `NULL`. See also
    [`Null`](asn1/struct.Null.html "struct der::asn1::Null").
-   [`bool`](https://doc.rust-lang.org/1.93.1/std/primitive.bool.html "primitive bool"):
    ASN.1 `BOOLEAN`.
-   [`i8`](https://doc.rust-lang.org/1.93.1/std/primitive.i8.html "primitive i8"),
    [`i16`](https://doc.rust-lang.org/1.93.1/std/primitive.i16.html "primitive i16"),
    [`i32`](https://doc.rust-lang.org/1.93.1/std/primitive.i32.html "primitive i32"),
    [`i64`](https://doc.rust-lang.org/1.93.1/std/primitive.i64.html "primitive i64"),
    [`i128`](https://doc.rust-lang.org/1.93.1/std/primitive.i128.html "primitive i128"):
    ASN.1 `INTEGER`.
-   [`u8`](https://doc.rust-lang.org/1.93.1/std/primitive.u8.html "primitive u8"),
    [`u16`](https://doc.rust-lang.org/1.93.1/std/primitive.u16.html "primitive u16"),
    [`u32`](https://doc.rust-lang.org/1.93.1/std/primitive.u32.html "primitive u32"),
    [`u64`](https://doc.rust-lang.org/1.93.1/std/primitive.u64.html "primitive u64"),
    [`u128`](https://doc.rust-lang.org/1.93.1/std/primitive.u128.html "primitive u128"):
    ASN.1 `INTEGER`.
-   [`f64`](https://doc.rust-lang.org/1.93.1/std/primitive.f64.html "primitive f64"):
    ASN.1 `REAL` (gated on `real` crate feature)
-   [`str`](https://doc.rust-lang.org/1.93.1/std/primitive.str.html "primitive str"),
    [`String`](https://doc.rust-lang.org/1.93.1/alloc/string/struct.String.html "struct alloc::string::String"):
    ASN.1 `UTF8String`. `String` requires `alloc` feature. See also
    [`Utf8StringRef`](asn1/struct.Utf8StringRef.html "struct der::asn1::Utf8StringRef").
-   [`Option`](https://doc.rust-lang.org/1.93.1/core/option/enum.Option.html "enum core::option::Option"):
    ASN.1 `OPTIONAL`.
-   [`SystemTime`](https://doc.rust-lang.org/1.93.1/std/time/struct.SystemTime.html "struct std::time::SystemTime"):
    ASN.1 `GeneralizedTime`. Requires `std` feature.
-   [`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"):
    ASN.1 `SEQUENCE OF`. Requires `alloc` feature.
-   `[T; N]`: ASN.1 `SEQUENCE OF`. See also
    [`SequenceOf`](asn1/struct.SequenceOf.html "struct der::asn1::SequenceOf").

The following ASN.1 types provided by this crate also impl these traits:

-   [`Any`](asn1/struct.Any.html "struct der::asn1::Any"),
    [`AnyRef`](asn1/struct.AnyRef.html "struct der::asn1::AnyRef"):
    ASN.1 `ANY`.
-   [`BitString`](asn1/struct.BitString.html "struct der::asn1::BitString"),
    [`BitStringRef`](asn1/struct.BitStringRef.html "struct der::asn1::BitStringRef"):
    ASN.1 `BIT STRING`
-   [`GeneralizedTime`](asn1/struct.GeneralizedTime.html "struct der::asn1::GeneralizedTime"):
    ASN.1 `GeneralizedTime`.
-   [`Ia5StringRef`](asn1/struct.Ia5StringRef.html "struct der::asn1::Ia5StringRef"):
    ASN.1 `IA5String`.
-   [`Null`](asn1/struct.Null.html "struct der::asn1::Null"): ASN.1
    `NULL`.
-   [`ObjectIdentifier`](asn1/struct.ObjectIdentifier.html "struct der::asn1::ObjectIdentifier"):
    ASN.1 `OBJECT IDENTIFIER`.
-   [`OctetString`](asn1/struct.OctetString.html "struct der::asn1::OctetString"),
    [`OctetStringRef`](asn1/struct.OctetStringRef.html "struct der::asn1::OctetStringRef"):
    ASN.1 `OCTET STRING`.
-   [`PrintableStringRef`](asn1/struct.PrintableStringRef.html "struct der::asn1::PrintableStringRef"):
    ASN.1 `PrintableString` (ASCII subset).
-   [`TeletexStringRef`](asn1/struct.TeletexStringRef.html "struct der::asn1::TeletexStringRef"):
    ASN.1 `TeletexString`.
-   [`VideotexStringRef`](asn1/struct.VideotexStringRef.html "struct der::asn1::VideotexStringRef"):
    ASN.1 `VideotexString`.
-   [`SequenceOf`](asn1/struct.SequenceOf.html "struct der::asn1::SequenceOf"):
    ASN.1 `SEQUENCE OF`.
-   [`SetOf`](asn1/struct.SetOf.html "struct der::asn1::SetOf"),
    [`SetOfVec`](asn1/struct.SetOfVec.html "struct der::asn1::SetOfVec"):
    ASN.1 `SET OF`.
-   [`UintRef`](asn1/struct.UintRef.html "struct der::asn1::UintRef"):
    ASN.1 unsigned `INTEGER` with raw access to encoded bytes.
-   [`UtcTime`](asn1/struct.UtcTime.html "struct der::asn1::UtcTime"):
    ASN.1 `UTCTime`.
-   [`Utf8StringRef`](asn1/struct.Utf8StringRef.html "struct der::asn1::Utf8StringRef"):
    ASN.1 `UTF8String`.

Context specific fields can be modeled using these generic types:

-   [`ContextSpecific`](asn1/struct.ContextSpecific.html "struct der::asn1::ContextSpecific"):
    decoder/encoder for owned context-specific fields
-   [`ContextSpecificRef`](asn1/struct.ContextSpecificRef.html "struct der::asn1::ContextSpecificRef"):
    encode-only type for references to context-specific fields

## <a href="#example" class="doc-anchor">§</a>Example

The following example implements X.509’s `AlgorithmIdentifier` message
type as defined in [RFC 5280 Section
4.1.1.2](https://tools.ietf.org/html/rfc5280#section-4.1.1.2).

The ASN.1 schema for this message type is as follows:

``` language-text
AlgorithmIdentifier  ::=  SEQUENCE  {
     algorithm               OBJECT IDENTIFIER,
     parameters              ANY DEFINED BY algorithm OPTIONAL  }
```

Structured ASN.1 messages are typically encoded as a `SEQUENCE`, which
this crate maps to a Rust struct using the
[`Sequence`](asn1/trait.Sequence.html "trait der::asn1::Sequence")
trait. This trait is bounded on the
[`Decode`](trait.Decode.html "trait der::Decode") trait and provides a
blanket impl of the [`Encode`](trait.Encode.html "trait der::Encode")
trait, so any type which impls
[`Sequence`](asn1/trait.Sequence.html "trait der::asn1::Sequence") can
be used for both decoding and encoding.

The following code example shows how to define a struct which maps to
the above schema, as well as impl the
[`Sequence`](asn1/trait.Sequence.html "trait der::asn1::Sequence") trait
for that struct:

``` {.rust .rust-example-rendered}
// Note: the following example does not require the `std` feature at all.
// It does leverage the `alloc` feature, but also provides instructions for
// "heapless" usage when the `alloc` feature is disabled.
use der::{
    asn1::{AnyRef, ObjectIdentifier},
    DecodeValue, Decode, SliceReader, Encode, Header, Reader, Sequence
};

/// X.509 `AlgorithmIdentifier`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AlgorithmIdentifier<'a> {
    /// This field contains an ASN.1 `OBJECT IDENTIFIER`, a.k.a. OID.
    pub algorithm: ObjectIdentifier,

    /// This field is `OPTIONAL` and contains the ASN.1 `ANY` type, which
    /// in this example allows arbitrary algorithm-defined parameters.
    pub parameters: Option<AnyRef<'a>>
}

impl<'a> DecodeValue<'a> for AlgorithmIdentifier<'a> {
    fn decode_value<R: Reader<'a>>(reader: &mut R, _header: Header) -> der::Result<Self> {
       // The `der::Decoder::Decode` method can be used to decode any
       // type which impls the `Decode` trait, which is impl'd for
       // all of the ASN.1 built-in types in the `der` crate.
       //
       // Note that if your struct's fields don't contain an ASN.1
       // built-in type specifically, there are also helper methods
       // for all of the built-in types supported by this library
       // which can be used to select a specific type.
       //
       // For example, another way of decoding this particular field,
       // which contains an ASN.1 `OBJECT IDENTIFIER`, is by calling
       // `decoder.oid()`. Similar methods are defined for other
       // ASN.1 built-in types.
       let algorithm = reader.decode()?;

       // This field contains an ASN.1 `OPTIONAL` type. The `der` crate
       // maps this directly to Rust's `Option` type and provides
       // impls of the `Decode` and `Encode` traits for `Option`.
       // To explicitly request an `OPTIONAL` type be decoded, use the
       // `decoder.optional()` method.
       let parameters = reader.decode()?;

       // The value returned from the provided `FnOnce` will be
       // returned from the `any.sequence(...)` call above.
       // Note that the entire sequence body *MUST* be consumed
       // or an error will be returned.
       Ok(Self { algorithm, parameters })
    }
}

impl<'a> ::der::EncodeValue for AlgorithmIdentifier<'a> {
    fn value_len(&self) -> ::der::Result<::der::Length> {
        self.algorithm.encoded_len()? + self.parameters.encoded_len()?
    }

    fn encode_value(&self, writer: &mut impl ::der::Writer) -> ::der::Result<()> {
        self.algorithm.encode(writer)?;
        self.parameters.encode(writer)?;
        Ok(())
    }
}

impl<'a> Sequence<'a> for AlgorithmIdentifier<'a> {}

// Example parameters value: OID for the NIST P-256 elliptic curve.
let parameters = "1.2.840.10045.3.1.7".parse::<ObjectIdentifier>().unwrap();

// We need to convert `parameters` into an `Any<'a>` type, which wraps a
// `&'a [u8]` byte slice.
//
// To do that, we need owned DER-encoded data so that we can have
// `AnyRef` borrow a reference to it, so we have to serialize the OID.
//
// When the `alloc` feature of this crate is enabled, any type that impls
// the `Encode` trait including all ASN.1 built-in types and any type
// which impls `Sequence` can be serialized by calling `Encode::to_der()`.
//
// If you would prefer to avoid allocations, you can create a byte array
// as backing storage instead, pass that to `der::Encoder::new`, and then
// encode the `parameters` value using `encoder.encode(parameters)`.
let der_encoded_parameters = parameters.to_der().unwrap();

let algorithm_identifier = AlgorithmIdentifier {
    // OID for `id-ecPublicKey`, if you're curious
    algorithm: "1.2.840.10045.2.1".parse().unwrap(),

    // `Any<'a>` impls `TryFrom<&'a [u8]>`, which parses the provided
    // slice as an ASN.1 DER-encoded message.
    parameters: Some(der_encoded_parameters.as_slice().try_into().unwrap())
};

// Serialize the `AlgorithmIdentifier` created above as ASN.1 DER,
// allocating a `Vec<u8>` for storage.
//
// As mentioned earlier, if you don't have the `alloc` feature enabled you
// can create a fix-sized array instead, then call `Encoder::new` with a
// reference to it, then encode the message using
// `encoder.encode(algorithm_identifier)`, then finally `encoder.finish()`
// to obtain a byte slice containing the encoded message.
let der_encoded_algorithm_identifier = algorithm_identifier.to_der().unwrap();

// Deserialize the `AlgorithmIdentifier` we just serialized from ASN.1 DER
// using `der::Decode::from_bytes`.
let decoded_algorithm_identifier = AlgorithmIdentifier::from_der(
    &der_encoded_algorithm_identifier
).unwrap();

// Ensure the original `AlgorithmIdentifier` is the same as the one we just
// decoded from ASN.1 DER.
assert_eq!(algorithm_identifier, decoded_algorithm_identifier);
```

## <a href="#custom-derive-support" class="doc-anchor">§</a>Custom derive support

When the `derive` feature of this crate is enabled, the following custom
derive macros are available:

-   [`Choice`](asn1/trait.Choice.html "trait der::asn1::Choice"): derive
    for `CHOICE` enum (see
    [`der_derive::Choice`](derive.Choice.html "derive der::Choice"))
-   [`Enumerated`](derive.Enumerated.html "derive der::Enumerated"):
    derive for `ENUMERATED` enum (see
    [`der_derive::Enumerated`](derive.Enumerated.html "derive der::Enumerated"))
-   [`Sequence`](asn1/trait.Sequence.html "trait der::asn1::Sequence"):
    derive for `SEQUENCE` struct (see
    [`der_derive::Sequence`](derive.Sequence.html "derive der::Sequence"))

### <a href="#derive-sequence-for-struct" class="doc-anchor">§</a>Derive [`Sequence`](asn1/trait.Sequence.html "trait der::asn1::Sequence") for struct

The following is a code example of how to use the
[`Sequence`](asn1/trait.Sequence.html "trait der::asn1::Sequence")
custom derive:

``` {.rust .rust-example-rendered}
use der::{asn1::{AnyRef, ObjectIdentifier}, Encode, Decode, Sequence};

/// X.509 `AlgorithmIdentifier` (same as above)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Sequence)] // NOTE: added `Sequence`
pub struct AlgorithmIdentifier<'a> {
    /// This field contains an ASN.1 `OBJECT IDENTIFIER`, a.k.a. OID.
    pub algorithm: ObjectIdentifier,

    /// This field is `OPTIONAL` and contains the ASN.1 `ANY` type, which
    /// in this example allows arbitrary algorithm-defined parameters.
    pub parameters: Option<AnyRef<'a>>
}

// Example parameters value: OID for the NIST P-256 elliptic curve.
let parameters_oid = "1.2.840.10045.3.1.7".parse::<ObjectIdentifier>().unwrap();

let algorithm_identifier = AlgorithmIdentifier {
    // OID for `id-ecPublicKey`, if you're curious
    algorithm: "1.2.840.10045.2.1".parse().unwrap(),

    // `Any<'a>` impls `From<&'a ObjectIdentifier>`, allowing OID constants to
    // be directly converted to an `AnyRef` type for this use case.
    parameters: Some(AnyRef::from(&parameters_oid))
};

// Encode
let der_encoded_algorithm_identifier = algorithm_identifier.to_der().unwrap();

// Decode
let decoded_algorithm_identifier = AlgorithmIdentifier::from_der(
    &der_encoded_algorithm_identifier
).unwrap();

assert_eq!(algorithm_identifier, decoded_algorithm_identifier);
```

For fields which don’t directly impl
[`Decode`](trait.Decode.html "trait der::Decode") and
[`Encode`](trait.Encode.html "trait der::Encode"), you can add
annotations to convert to an intermediate ASN.1 type first, so long as
that type impls `TryFrom` and `Into` for the ASN.1 type.

For example, structs containing `&'a [u8]` fields may want them encoded
as either a `BIT STRING` or `OCTET STRING`. By using the
`#[asn1(type = "BIT STRING")]` annotation it’s possible to select which
ASN.1 type should be used.

Building off the above example:

``` {.rust .rust-example-rendered}
/// X.509 `SubjectPublicKeyInfo` (SPKI)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Sequence)]
pub struct SubjectPublicKeyInfo<'a> {
    /// X.509 `AlgorithmIdentifier`
    pub algorithm: AlgorithmIdentifier<'a>,

    /// Public key data
    pub subject_public_key: BitStringRef<'a>,
}
```

# <a href="#see-also" class="doc-anchor">§</a>See also

For more information about ASN.1 DER we recommend the following guides:

-   [A Layman’s Guide to a Subset of ASN.1, BER, and
    DER](https://luca.ntop.org/Teaching/Appunti/asn1.html) (RSA
    Laboratories)
-   [A Warm Welcome to ASN.1 and
    DER](https://letsencrypt.org/docs/a-warm-welcome-to-asn1-and-der/)
    (Let’s Encrypt)

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::asn1::`<a href="asn1/struct.AnyRef.html" class="struct"
title="struct der::asn1::AnyRef"><code>AnyRef</code></a>`;`

`pub use crate::asn1::`<a href="asn1/trait.Choice.html" class="trait"
title="trait der::asn1::Choice"><code>Choice</code></a>`;`

`pub use crate::asn1::`<a href="asn1/trait.Sequence.html" class="trait"
title="trait der::asn1::Sequence"><code>Sequence</code></a>`;`

`pub use crate::asn1::`<a href="asn1/struct.Any.html" class="struct"
title="struct der::asn1::Any"><code>Any</code></a>`;`

`pub use `<a href="../flagset/index.html" class="mod"
title="mod flagset"><code>flagset</code></a>`;`

`pub use `<a href="../const_oid/index.html" class="mod"
title="mod const_oid"><code>const_oid</code></a>` as oid;`

`pub use `<a href="../pem_rfc7468/index.html" class="mod"
title="mod pem_rfc7468"><code>pem_rfc7468</code></a>` as pem;`

`pub use `<a href="../zeroize/index.html" class="mod"
title="mod zeroize"><code>zeroize</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="asn1/index.html" class="mod" title="mod der::asn1">asn1</a>  
Module containing all of the various ASN.1 built-in types supported by
this library.

<a href="referenced/index.html" class="mod"
title="mod der::referenced">referenced</a>  
A module for working with referenced data.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DateTime.html" class="struct"
title="struct der::DateTime">DateTime</a>  
Date-and-time type shared by multiple ASN.1 types (e.g.
`GeneralizedTime`, `UTCTime`).

<a href="struct.Document.html" class="struct"
title="struct der::Document">Document</a>  
ASN.1 DER-encoded document.

<a href="struct.EncodeRef.html" class="struct"
title="struct der::EncodeRef">EncodeRef</a>  
Reference encoder: wrapper type which impls `Encode` for any reference
to a type which impls the same.

<a href="struct.EncodeValueRef.html" class="struct"
title="struct der::EncodeValueRef">EncodeValueRef</a>  
Reference value encoder: wrapper type which impls `EncodeValue` and
`Tagged` for any reference type which impls the same.

<a href="struct.Error.html" class="struct"
title="struct der::Error">Error</a>  
Error type.

<a href="struct.Header.html" class="struct"
title="struct der::Header">Header</a>  
ASN.1 DER headers: tag + length component of TLV-encoded values

<a href="struct.IndefiniteLength.html" class="struct"
title="struct der::IndefiniteLength">IndefiniteLength</a>  
Length type with support for indefinite lengths as used by ASN.1 BER, as
described in X.690 Section 8.1.3.6:

<a href="struct.Length.html" class="struct"
title="struct der::Length">Length</a>  
ASN.1-encoded length.

<a href="struct.NestedReader.html" class="struct"
title="struct der::NestedReader">NestedReader</a>  
Reader type used by
[`Reader::read_nested`](trait.Reader.html#method.read_nested "method der::Reader::read_nested").

<a href="struct.PemReader.html" class="struct"
title="struct der::PemReader">PemReader</a>  
`Reader` type which decodes PEM on-the-fly.

<a href="struct.PemWriter.html" class="struct"
title="struct der::PemWriter">PemWriter</a>  
`Writer` type which outputs PEM-encoded data.

<a href="struct.SecretDocument.html" class="struct"
title="struct der::SecretDocument">SecretDocument</a>  
Secret [`Document`](struct.Document.html "struct der::Document") type.

<a href="struct.SliceReader.html" class="struct"
title="struct der::SliceReader">SliceReader</a>  
[`Reader`](trait.Reader.html "trait der::Reader") which consumes an
input byte slice.

<a href="struct.SliceWriter.html" class="struct"
title="struct der::SliceWriter">SliceWriter</a>  
[`Writer`](trait.Writer.html "trait der::Writer") which encodes DER into
a mutable output byte slice.

<a href="struct.TagNumber.html" class="struct"
title="struct der::TagNumber">TagNumber</a>  
ASN.1 tag numbers (i.e. lower 5 bits of a
[`Tag`](enum.Tag.html "enum der::Tag")).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Class.html" class="enum" title="enum der::Class">Class</a>  
Class of an ASN.1 tag.

<a href="enum.ErrorKind.html" class="enum"
title="enum der::ErrorKind">ErrorKind</a>  
Error type.

<a href="enum.Tag.html" class="enum" title="enum der::Tag">Tag</a>  
ASN.1 tags.

<a href="enum.TagMode.html" class="enum"
title="enum der::TagMode">TagMode</a>  
Tagging modes: `EXPLICIT` versus `IMPLICIT`.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Decode.html" class="trait"
title="trait der::Decode">Decode</a>  
Decoding trait.

<a href="trait.DecodeOwned.html" class="trait"
title="trait der::DecodeOwned">DecodeOwned</a>  
Marker trait for data structures that can be decoded from DER without
borrowing any data from the decoder.

<a href="trait.DecodePem.html" class="trait"
title="trait der::DecodePem">DecodePem</a>  
PEM decoding trait.

<a href="trait.DecodeValue.html" class="trait"
title="trait der::DecodeValue">DecodeValue</a>  
Decode the value part of a Tag-Length-Value encoded field, sans the
[`Tag`](enum.Tag.html "enum der::Tag") and
[`Length`](struct.Length.html "struct der::Length").

<a href="trait.DerOrd.html" class="trait"
title="trait der::DerOrd">DerOrd</a>  
DER ordering trait.

<a href="trait.Encode.html" class="trait"
title="trait der::Encode">Encode</a>  
Encoding trait.

<a href="trait.EncodePem.html" class="trait"
title="trait der::EncodePem">EncodePem</a>  
PEM encoding trait.

<a href="trait.EncodeValue.html" class="trait"
title="trait der::EncodeValue">EncodeValue</a>  
Encode the value part of a Tag-Length-Value encoded field, sans the
[`Tag`](enum.Tag.html "enum der::Tag") and
[`Length`](struct.Length.html "struct der::Length").

<a href="trait.FixedTag.html" class="trait"
title="trait der::FixedTag">FixedTag</a>  
Types which have a constant ASN.1
[`Tag`](enum.Tag.html "enum der::Tag").

<a href="trait.Reader.html" class="trait"
title="trait der::Reader">Reader</a>  
Reader trait which reads DER-encoded input.

<a href="trait.Tagged.html" class="trait"
title="trait der::Tagged">Tagged</a>  
Types which have an ASN.1 [`Tag`](enum.Tag.html "enum der::Tag").

<a href="trait.ValueOrd.html" class="trait"
title="trait der::ValueOrd">ValueOrd</a>  
DER value ordering trait.

<a href="trait.Writer.html" class="trait"
title="trait der::Writer">Writer</a>  
Writer trait which outputs encoded DER.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type der::Result">Result</a>  
Result type.

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.Choice.html" class="derive"
title="derive der::Choice">Choice</a>  
Derive the [`Choice`](https://docs.rs/der/latest/der/trait.Choice.html)
trait on an `enum`.

<a href="derive.Enumerated.html" class="derive"
title="derive der::Enumerated">Enumerated</a>  
Derive decoders and encoders for ASN.1
[`Enumerated`](derive.Enumerated.html "derive der::Enumerated") types on
a C-like `enum` type.

<a href="derive.Sequence.html" class="derive"
title="derive der::Sequence">Sequence</a>  
Derive the
[`Sequence`](https://docs.rs/der/latest/der/trait.Sequence.html) trait
on a `struct`.

<a href="derive.ValueOrd.html" class="derive"
title="derive der::ValueOrd">ValueOrd</a>  
Derive the
[`ValueOrd`](https://docs.rs/der/latest/der/trait.ValueOrd.html) trait
on a `struct`.
