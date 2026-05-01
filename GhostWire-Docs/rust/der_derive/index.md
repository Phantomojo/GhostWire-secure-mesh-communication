Crate der\_derive Copy item path

<span class="sub-heading"><a href="../src/der_derive/lib.rs.html#1-295" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-der-custom-derive-support" class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto): DER Custom Derive Support

[![crate](https://img.shields.io/crates/v/der_derive.svg)](https://crates.io/crates/der_derive)
[![Docs](https://docs.rs/der_derive/badge.svg)](https://docs.rs/der_derive/)
[![Build
Status](https://github.com/RustCrypto/formats/actions/workflows/der.yml/badge.svg)](https://github.com/RustCrypto/formats/actions/workflows/der.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.65+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats)

Custom derive support for the `der` crate’s `Choice` and `Sequence`
traits:

<https://github.com/RustCrypto/formats/tree/master/der>

[Documentation](https://docs.rs/der_derive/)

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

## <a href="#about" class="doc-anchor">§</a>About

Custom derive support for the [`der`](https://docs.rs/der/) crate.

This crate contains custom derive macros intended to be used in the
following way:

-   [`Choice`](derive.Choice.html "derive der_derive::Choice"): map
    ASN.1 `CHOICE` to a Rust enum.
-   [`Enumerated`](derive.Enumerated.html "derive der_derive::Enumerated"):
    map ASN.1 `ENUMERATED` to a C-like Rust enum.
-   [`Sequence`](derive.Sequence.html "derive der_derive::Sequence"):
    map ASN.1 `SEQUENCE` to a Rust struct.
-   [`ValueOrd`](derive.ValueOrd.html "derive der_derive::ValueOrd"):
    determine DER ordering for ASN.1 `SET OF`.

Note that this crate shouldn’t be used directly, but instead accessed by
using the `derive` feature of the `der` crate, which re-exports the
above macros from the toplevel.

## <a href="#why-not-serde" class="doc-anchor">§</a>Why not `serde`?

The `der` crate is designed to be easily usable in embedded
environments, including ones where code size comes at a premium.

This crate (i.e. `der_derive`) is able to generate code which is
significantly smaller than `serde_derive`. This is because the `der`
crate has been designed with high-level abstractions which reduce code
size, including trait object-based encoders which allow encoding logic
which is duplicated in `serde` serializers to be implemented in a single
place in the `der` crate.

This is a deliberate tradeoff in terms of performance, flexibility, and
code size. At least for now, the `der` crate is optimizing for
leveraging as many abstractions as it can to minimize code size.

## <a href="#toplevel-attributes" class="doc-anchor">§</a>Toplevel attributes

The following attributes can be added to an `enum` or `struct` when
deriving either
[`Choice`](derive.Choice.html "derive der_derive::Choice") or
[`Sequence`](derive.Sequence.html "derive der_derive::Sequence")
respectively:

### <a href="#asn1tag_mode---attribute-explicit-vs-implicit"
class="doc-anchor">§</a>`#[asn1(tag_mode = "...")]` attribute: `EXPLICIT` vs `IMPLICIT`

This attribute can be used to declare the tagging mode used by a
particular ASN.1 module.

It’s used when parsing `CONTEXT-SENSITIVE` fields.

The default is `EXPLICIT`, so the attribute only needs to be added when
a particular module is declared `IMPLICIT`.

## <a href="#field-level-attributes" class="doc-anchor">§</a>Field-level attributes

The following attributes can be added to either the fields of a
particular `struct` or the variants of a particular `enum`:

### <a href="#asn1context_specific---attribute-context-specific-support"
class="doc-anchor">§</a>`#[asn1(context_specific = "...")]` attribute: `CONTEXT-SPECIFIC` support

This attribute can be added to associate a particular `CONTEXT-SPECIFIC`
tag number with a given enum variant or struct field.

The value must be quoted and contain a number, e.g.
`#[asn1(context_specific = "29")]`.

### <a href="#asn1default---attribute-default-support"
class="doc-anchor">§</a>`#[asn1(default = "...")]` attribute: `DEFAULT` support

This behaves like `serde_derive`’s `default` attribute, allowing you to
specify the path to a function which returns a default value.

### <a
href="#asn1extensible--true-attribute-support-for--extensibility-operator"
class="doc-anchor">§</a>`#[asn1(extensible = "true")]` attribute: support for `...` extensibility operator

This attribute can be applied to the fields of `struct` types, and will
skip over unrecognized lower-numbered `CONTEXT-SPECIFIC` fields when
looking for a particular field of a struct.

### <a href="#asn1optional--true-attribute-support-for-optional-fields"
class="doc-anchor">§</a>`#[asn1(optional = "true")]` attribute: support for `OPTIONAL` fields

This attribute explicitly annotates a field as `OPTIONAL`.

### <a href="#asn1type---attribute-asn1-type-declaration"
class="doc-anchor">§</a>`#[asn1(type = "...")]` attribute: ASN.1 type declaration

This attribute can be used to specify the ASN.1 type for a particular
`enum` variant or `struct` field.

It’s presently mandatory for all `enum` variants, even when using one of
the ASN.1 types defined by this crate.

For structs, placing this attribute on a field makes it possible to
decode/encode types which don’t directly implement the `Decode`/`Encode`
traits but do impl `From` and `TryInto` and `From` for one of the ASN.1
types listed below (use the ASN.1 type keywords as the `type`):

-   `BIT STRING`: performs an intermediate conversion to
    [`der::asn1::BitString`](https://docs.rs/der/latest/der/asn1/struct.BitString.html)
-   `IA5String`: performs an intermediate conversion to
    [`der::asn1::IA5String`](https://docs.rs/der/latest/der/asn1/struct.Ia5String.html)
-   `GeneralizedTime`: performs an intermediate conversion to
    [`der::asn1::GeneralizedTime`](https://docs.rs/der/latest/der/asn1/struct.GeneralizedTime.html)
-   `OCTET STRING`: performs an intermediate conversion to
    [`der::asn1::OctetString`](https://docs.rs/der/latest/der/asn1/struct.OctetString.html)
-   `PrintableString`: performs an intermediate conversion to
    [`der::asn1::PrintableString`](https://docs.rs/der/latest/der/asn1/struct.PrintableString.html)
-   `UTCTime`: performs an intermediate conversion to
    [`der::asn1::UtcTime`](https://docs.rs/der/latest/der/asn1/struct.UtcTime.html)
-   `UTF8String`: performs an intermediate conversion to
    [`der::asn1::Utf8String`](https://docs.rs/der/latest/der/asn1/struct.Utf8String.html)

### <a
href="#asn1constructed---attribute-support-for-constructed-inner-types"
class="doc-anchor">§</a>`#[asn1(constructed = "...")]` attribute: support for constructed inner types

This attribute can be used to specify that an “inner” type is
constructed. It is most commonly used when a `CHOICE` has a constructed
inner type.

Note: please open a GitHub Issue if you would like to request support
for additional ASN.1 types.

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.Choice.html" class="derive"
title="derive der_derive::Choice">Choice</a>  
Derive the [`Choice`](https://docs.rs/der/latest/der/trait.Choice.html)
trait on an `enum`.

<a href="derive.Enumerated.html" class="derive"
title="derive der_derive::Enumerated">Enumerated</a>  
Derive decoders and encoders for ASN.1
[`Enumerated`](derive.Enumerated.html "derive der_derive::Enumerated")
types on a C-like `enum` type.

<a href="derive.Sequence.html" class="derive"
title="derive der_derive::Sequence">Sequence</a>  
Derive the
[`Sequence`](https://docs.rs/der/latest/der/trait.Sequence.html) trait
on a `struct`.

<a href="derive.ValueOrd.html" class="derive"
title="derive der_derive::ValueOrd">ValueOrd</a>  
Derive the
[`ValueOrd`](https://docs.rs/der/latest/der/trait.ValueOrd.html) trait
on a `struct`.
