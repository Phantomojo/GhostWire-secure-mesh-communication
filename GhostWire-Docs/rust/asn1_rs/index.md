Crate asn1\_rs Copy item path

<span class="sub-heading"><a href="../src/asn1_rs/lib.rs.html#1-215" class="src">Source</a>
</span>

Expand description

[![License:
MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE-MIT)
[![Apache License
2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)
[![docs.rs](https://docs.rs/asn1-rs/badge.svg)](https://docs.rs/asn1-rs)
[![crates.io](https://img.shields.io/crates/v/asn1-rs.svg)](https://crates.io/crates/asn1-rs)
[![Download
numbers](https://img.shields.io/crates/d/asn1-rs.svg)](https://crates.io/crates/asn1-rs)
[![Github
CI](https://github.com/rusticata/asn1-rs/workflows/Continuous%20integration/badge.svg)](https://github.com/rusticata/asn1-rs/actions)
[![Minimum rustc
version](https://img.shields.io/badge/rustc-1.63.0+-lightgray.svg)](#rust-version-requirements)

# <a href="#berder-parsersencoders" class="doc-anchor">§</a>BER/DER Parsers/Encoders

A set of parsers/encoders for Basic Encoding Rules (BER
\[[X.690](https://www.itu.int/rec/T-REC-X.690/en "ASN.1 encoding rules: Specification of
Basic Encoding Rules (BER), Canonical Encoding Rules (CER) and Distinguished Encoding Rules
(DER).")\]) and Distinguished Encoding Rules(DER
\[[X.690](https://www.itu.int/rec/T-REC-X.690/en "ASN.1 encoding rules: Specification of
Basic Encoding Rules (BER), Canonical Encoding Rules (CER) and Distinguished Encoding Rules
(DER).")\]) formats, implemented with the
[nom](https://github.com/Geal/nom "Nom parser combinator framework")
parser combinator framework.

It is written in pure Rust, fast, and makes extensive use of zero-copy.
A lot of care is taken to ensure security and safety of this crate,
including design (recursion limit, defensive programming), tests, and
fuzzing. It also aims to be panic-free.

This crate is a rewrite of
[der-parser](https://crates.io/crates/der-parser) to propose a more
data-oriented API, and add generalized support for serialization.

Many ideas were borrowed from the
[crypto/utils/der](https://github.com/RustCrypto/utils/tree/master/der)
crate (like the `Any`/`TryFrom`/`FromDer` mechanism), adapted and merged
into a generalized BER/DER crate. Credits (and many thanks) go to Tony
Arcieri for writing the original crate.

# <a href="#berder-parsers" class="doc-anchor">§</a>BER/DER parsers

BER stands for Basic Encoding Rules, and is defined in
\[[X.690](https://www.itu.int/rec/T-REC-X.690/en "ASN.1 encoding rules: Specification of
Basic Encoding Rules (BER), Canonical Encoding Rules (CER) and Distinguished Encoding Rules
(DER).")\]. It defines a set of rules to encode and decode ASN.1
\[[X.680](http://www.itu.int/rec/T-REC-X.680/en "Abstract Syntax Notation One (ASN.1):
Specification of basic notation.")\] objects in binary.

\[[X.690](https://www.itu.int/rec/T-REC-X.690/en "ASN.1 encoding rules: Specification of
Basic Encoding Rules (BER), Canonical Encoding Rules (CER) and Distinguished Encoding Rules
(DER).")\] also defines Distinguished Encoding Rules (DER), which is BER
with added rules to ensure canonical and unequivocal binary
representation of objects.

The choice of which one to use is usually guided by the speficication of
the data format based on BER or DER: for example, X.509 uses DER as
encoding representation.

The main traits for parsing are the
[`FromBer`](trait.FromBer.html "trait asn1_rs::FromBer") and
[`FromDer`](trait.FromDer.html "trait asn1_rs::FromDer") traits. These
traits provide methods to parse binary input, and return either the
remaining (unparsed) bytes and the parsed object, or an error.

The parsers follow the interface from
[nom](https://github.com/Geal/nom "Nom parser combinator framework"),
and the
[`ParseResult`](type.ParseResult.html "type asn1_rs::ParseResult")
object is a specialized version of `nom::IResult`. This means that most
`nom` combinators (`map`, `many0`, etc.) can be used in combination to
objects and methods from this crate. Reading the nom documentation may
help understanding how to write and combine parsers and use the output.

**Minimum Supported Rust Version**: 1.63.0

# <a href="#recipes" class="doc-anchor">§</a>Recipes

See [doc::recipes](doc/recipes/index.html "mod asn1_rs::doc::recipes")
and [doc::derive](doc/derive/index.html "mod asn1_rs::doc::derive") for
more examples and recipes.

See [doc::debug](doc/debug/index.html "mod asn1_rs::doc::debug") for
advice and tools to debug parsers.

## <a href="#examples" class="doc-anchor">§</a>Examples

Parse 2 BER integers:

``` {.rust .rust-example-rendered}
use asn1_rs::{Integer, FromBer};

let bytes = [ 0x02, 0x03, 0x01, 0x00, 0x01,
              0x02, 0x03, 0x01, 0x00, 0x00,
];

let (rem, obj1) = Integer::from_ber(&bytes).expect("parsing failed");
let (rem, obj2) = Integer::from_ber(&bytes).expect("parsing failed");

assert_eq!(obj1, Integer::from_u32(65537));
```

In the above example, the generic
[`Integer`](struct.Integer.html "struct asn1_rs::Integer") type is used.
This type can contain integers of any size, but do not provide a simple
API to manipulate the numbers.

In most cases, the integer either has a limit, or is expected to fit
into a primitive type. To get a simple value, just use the
`from_ber`/`from_der` methods on the primitive types:

``` {.rust .rust-example-rendered}
use asn1_rs::FromBer;

let bytes = [ 0x02, 0x03, 0x01, 0x00, 0x01,
              0x02, 0x03, 0x01, 0x00, 0x00,
];

let (rem, obj1) = u32::from_ber(&bytes).expect("parsing failed");
let (rem, obj2) = u32::from_ber(&rem).expect("parsing failed");

assert_eq!(obj1, 65537);
assert_eq!(obj2, 65536);
```

If the parsing succeeds, but the integer cannot fit into the expected
type, the method will return an `IntegerTooLarge` error.

# <a href="#berder-encoders" class="doc-anchor">§</a>BER/DER encoders

BER/DER encoding is symmetrical to decoding, using the traits `ToBer`
and [`ToDer`](trait.ToDer.html "trait asn1_rs::ToDer") traits. These
traits provide methods to write encoded content to objects with the
`io::Write` trait, or return an allocated `Vec<u8>` with the encoded
data. If the serialization fails, an error is returned.

## <a href="#examples-1" class="doc-anchor">§</a>Examples

Writing 2 BER integers:

``` {.rust .rust-example-rendered}
use asn1_rs::{Integer, ToDer};

let mut writer = Vec::new();

let obj1 = Integer::from_u32(65537);
let obj2 = Integer::from_u32(65536);

let _ = obj1.write_der(&mut writer).expect("serialization failed");
let _ = obj2.write_der(&mut writer).expect("serialization failed");

let bytes = &[ 0x02, 0x03, 0x01, 0x00, 0x01,
               0x02, 0x03, 0x01, 0x00, 0x00,
];
assert_eq!(&writer, bytes);
```

Similarly to `FromBer`/`FromDer`, serialization methods are also
implemented for primitive types:

``` {.rust .rust-example-rendered}
use asn1_rs::ToDer;

let mut writer = Vec::new();

let _ = 65537.write_der(&mut writer).expect("serialization failed");
let _ = 65536.write_der(&mut writer).expect("serialization failed");

let bytes = &[ 0x02, 0x03, 0x01, 0x00, 0x01,
               0x02, 0x03, 0x01, 0x00, 0x00,
];
assert_eq!(&writer, bytes);
```

If the parsing succeeds, but the integer cannot fit into the expected
type, the method will return an `IntegerTooLarge` error.

## <a href="#changes" class="doc-anchor">§</a>Changes

See `CHANGELOG.md`.

# <a href="#references" class="doc-anchor">§</a>References

-   \[[X.680](http://www.itu.int/rec/T-REC-X.680/en "Abstract Syntax Notation One (ASN.1):
    Specification of basic notation.")\] Abstract Syntax Notation One
    (ASN.1): Specification of basic notation.
-   \[[X.690](https://www.itu.int/rec/T-REC-X.690/en "ASN.1 encoding rules: Specification of
    Basic Encoding Rules (BER), Canonical Encoding Rules (CER) and Distinguished Encoding Rules
    (DER).")\] ASN.1 encoding rules: Specification of Basic Encoding
    Rules (BER), Canonical Encoding Rules (CER) and Distinguished
    Encoding Rules (DER).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::`<a href="enum.Error.html" class="enum"
title="enum asn1_rs::Error"><code>Error</code></a>`;`

`pub use crate::`<a href="struct.SequenceIterator.html" class="struct"
title="struct asn1_rs::SequenceIterator"><code>SequenceIterator</code></a>`;`

`pub use `<a href="../nom/index.html" class="mod"
title="mod nom"><code>nom</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="doc/index.html" class="mod" title="mod asn1_rs::doc">doc</a>  
Additional documentation: recipes, specific use cases and examples, etc.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.int.html" class="macro"
title="macro asn1_rs::int">int</a>  
Helper macro to declare integers at compile-time

<a href="macro.oid.html" class="macro"
title="macro asn1_rs::oid">oid</a>  
Helper macro to declare integers at compile-time

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ASN1DateTime.html" class="struct"
title="struct asn1_rs::ASN1DateTime">ASN1DateTime</a>  
<a href="struct.Any.html" class="struct"
title="struct asn1_rs::Any">Any</a>  
The `Any` object is not strictly an ASN.1 type, but holds a generic
description of any object that could be encoded.

<a href="struct.BerClassFromIntError.html" class="struct"
title="struct asn1_rs::BerClassFromIntError">BerClassFromIntError</a>  
<a href="struct.BitString.html" class="struct"
title="struct asn1_rs::BitString">BitString</a>  
ASN.1 `BITSTRING` type

<a href="struct.BmpString.html" class="struct"
title="struct asn1_rs::BmpString">BmpString</a>  
ASN.1 `BMPSTRING` type

<a href="struct.Boolean.html" class="struct"
title="struct asn1_rs::Boolean">Boolean</a>  
ASN.1 `BOOLEAN` type

<a href="struct.EmbeddedPdv.html" class="struct"
title="struct asn1_rs::EmbeddedPdv">EmbeddedPdv</a>  
<a href="struct.EndOfContent.html" class="struct"
title="struct asn1_rs::EndOfContent">EndOfContent</a>  
End-of-contents octets

<a href="struct.Enumerated.html" class="struct"
title="struct asn1_rs::Enumerated">Enumerated</a>  
ASN.1 `ENUMERATED` type

<a href="struct.GeneralString.html" class="struct"
title="struct asn1_rs::GeneralString">GeneralString</a>  
ASN.1 restricted character string type (`GeneralString`)

<a href="struct.GeneralizedTime.html" class="struct"
title="struct asn1_rs::GeneralizedTime">GeneralizedTime</a>  
<a href="struct.GraphicString.html" class="struct"
title="struct asn1_rs::GraphicString">GraphicString</a>  
ASN.1 restricted character string type (`GraphicString`)

<a href="struct.Header.html" class="struct"
title="struct asn1_rs::Header">Header</a>  
BER/DER object header (identifier and length)

<a href="struct.Ia5String.html" class="struct"
title="struct asn1_rs::Ia5String">Ia5String</a>  
ASN.1 restricted character string type (`Ia5String`)

<a href="struct.Integer.html" class="struct"
title="struct asn1_rs::Integer">Integer</a>  
ASN.1 `INTEGER` type

<a href="struct.Null.html" class="struct"
title="struct asn1_rs::Null">Null</a>  
ASN.1 `NULL` type

<a href="struct.NumericString.html" class="struct"
title="struct asn1_rs::NumericString">NumericString</a>  
ASN.1 restricted character string type (`NumericString`)

<a href="struct.ObjectDescriptor.html" class="struct"
title="struct asn1_rs::ObjectDescriptor">ObjectDescriptor</a>  
ASN.1 restricted character string type (`ObjectDescriptor`)

<a href="struct.OctetString.html" class="struct"
title="struct asn1_rs::OctetString">OctetString</a>  
ASN.1 `OCTETSTRING` type

<a href="struct.Oid.html" class="struct"
title="struct asn1_rs::Oid">Oid</a>  
Object ID (OID) representation which can be relative or non-relative. An
example for an OID in string representation is `"1.2.840.113549.1.1.5"`.

<a href="struct.OptTaggedParser.html" class="struct"
title="struct asn1_rs::OptTaggedParser">OptTaggedParser</a>  
Helper object to parse TAGGED OPTIONAL types (explicit or implicit)

<a href="struct.PrintableString.html" class="struct"
title="struct asn1_rs::PrintableString">PrintableString</a>  
ASN.1 restricted character string type (`PrintableString`)

<a href="struct.Sequence.html" class="struct"
title="struct asn1_rs::Sequence">Sequence</a>  
The `SEQUENCE` object is an ordered list of heteregeneous types.

<a href="struct.SequenceIterator.html" class="struct"
title="struct asn1_rs::SequenceIterator">SequenceIterator</a>  
An Iterator over binary data, parsing elements of type `T`

<a href="struct.SequenceOf.html" class="struct"
title="struct asn1_rs::SequenceOf">SequenceOf</a>  
The `SEQUENCE OF` object is an ordered list of homogeneous types.

<a href="struct.Set.html" class="struct"
title="struct asn1_rs::Set">Set</a>  
The `SET` object is an unordered list of heteregeneous types.

<a href="struct.SetOf.html" class="struct"
title="struct asn1_rs::SetOf">SetOf</a>  
The `SET OF` object is an unordered list of homogeneous types.

<a href="struct.Tag.html" class="struct"
title="struct asn1_rs::Tag">Tag</a>  
BER/DER Tag as defined in X.680 section 8.4

<a href="struct.TaggedParser.html" class="struct"
title="struct asn1_rs::TaggedParser">TaggedParser</a>  
<a href="struct.TaggedParserBuilder.html" class="struct"
title="struct asn1_rs::TaggedParserBuilder">TaggedParserBuilder</a>  
A builder for parsing tagged values (`IMPLICIT` or `EXPLICIT`)

<a href="struct.TaggedValue.html" class="struct"
title="struct asn1_rs::TaggedValue">TaggedValue</a>  
Helper object for creating `FromBer`/`FromDer` types for TAGGED OPTIONAL
types

<a href="struct.TeletexString.html" class="struct"
title="struct asn1_rs::TeletexString">TeletexString</a>  
ASN.1 restricted character string type (`TeletexString`)

<a href="struct.UniversalString.html" class="struct"
title="struct asn1_rs::UniversalString">UniversalString</a>  
ASN.1 `UniversalString` type

<a href="struct.UtcTime.html" class="struct"
title="struct asn1_rs::UtcTime">UtcTime</a>  
<a href="struct.Utf8String.html" class="struct"
title="struct asn1_rs::Utf8String">Utf8String</a>  
ASN.1 restricted character string type (`Utf8String`)

<a href="struct.VideotexString.html" class="struct"
title="struct asn1_rs::VideotexString">VideotexString</a>  
ASN.1 restricted character string type (`VideotexString`)

<a href="struct.VisibleString.html" class="struct"
title="struct asn1_rs::VisibleString">VisibleString</a>  
ASN.1 restricted character string type (`VisibleString`)

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ASN1TimeZone.html" class="enum"
title="enum asn1_rs::ASN1TimeZone">ASN1TimeZone</a>  
<a href="enum.Class.html" class="enum"
title="enum asn1_rs::Class">Class</a>  
BER Object class of tag

<a href="enum.DerConstraint.html" class="enum"
title="enum asn1_rs::DerConstraint">DerConstraint</a>  
Error types for DER constraints

<a href="enum.Err.html" class="enum" title="enum asn1_rs::Err">Err</a>  
The `Err` enum indicates the parser was not successful

<a href="enum.Error.html" class="enum"
title="enum asn1_rs::Error">Error</a>  
The error type for operations of the
[`FromBer`](trait.FromBer.html "trait asn1_rs::FromBer"),
[`FromDer`](trait.FromDer.html "trait asn1_rs::FromDer"), and associated
traits.

<a href="enum.Explicit.html" class="enum"
title="enum asn1_rs::Explicit">Explicit</a>  
A type parameter for `EXPLICIT` tagged values.

<a href="enum.Implicit.html" class="enum"
title="enum asn1_rs::Implicit">Implicit</a>  
A type parameter for `IMPLICIT` tagged values.

<a href="enum.Length.html" class="enum"
title="enum asn1_rs::Length">Length</a>  
BER Object Length

<a href="enum.Needed.html" class="enum"
title="enum asn1_rs::Needed">Needed</a>  
Contains information on needed data if a parser returned `Incomplete`

<a href="enum.OidParseError.html" class="enum"
title="enum asn1_rs::OidParseError">OidParseError</a>  
An error for OID parsing functions.

<a href="enum.PdvIdentification.html" class="enum"
title="enum asn1_rs::PdvIdentification">PdvIdentification</a>  
<a href="enum.Real.html" class="enum"
title="enum asn1_rs::Real">Real</a>  
ASN.1 `REAL` type

<a href="enum.SerializeError.html" class="enum"
title="enum asn1_rs::SerializeError">SerializeError</a>  
The error type for serialization operations of the
[`ToDer`](trait.ToDer.html "trait asn1_rs::ToDer") trait.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsTaggedExplicit.html" class="trait"
title="trait asn1_rs::AsTaggedExplicit">AsTaggedExplicit</a>

Helper trait for creating tagged EXPLICIT values

<a href="trait.AsTaggedImplicit.html" class="trait"
title="trait asn1_rs::AsTaggedImplicit">AsTaggedImplicit</a>

Helper trait for creating tagged IMPLICIT values

<a href="trait.BerChoice.html" class="trait"
title="trait asn1_rs::BerChoice">BerChoice</a>

<a href="trait.CheckDerConstraints.html" class="trait"
title="trait asn1_rs::CheckDerConstraints">CheckDerConstraints</a>

Verification of DER constraints

<a href="trait.Choice.html" class="trait"
title="trait asn1_rs::Choice">Choice</a>

<a href="trait.DerAutoDerive.html" class="trait"
title="trait asn1_rs::DerAutoDerive">DerAutoDerive</a>

Trait to automatically derive `FromDer`

<a href="trait.DerChoice.html" class="trait"
title="trait asn1_rs::DerChoice">DerChoice</a>

<a href="trait.DynTagged.html" class="trait"
title="trait asn1_rs::DynTagged">DynTagged</a>

<a href="trait.FromBer.html" class="trait"
title="trait asn1_rs::FromBer">FromBer</a>

Base trait for BER object parsers

<a href="trait.FromDer.html" class="trait"
title="trait asn1_rs::FromDer">FromDer</a>

Base trait for DER object parsers

<a href="trait.TagKind.html" class="trait"
title="trait asn1_rs::TagKind">TagKind</a>

A type parameter for tagged values either
[`Explicit`](enum.Explicit.html "enum asn1_rs::Explicit") or
[`Implicit`](enum.Implicit.html "enum asn1_rs::Implicit").

<a href="trait.Tagged.html" class="trait"
title="trait asn1_rs::Tagged">Tagged</a>

<a href="trait.TestValidCharset.html" class="trait"
title="trait asn1_rs::TestValidCharset">TestValidCharset</a>

Base trait for BER string objects and character set validation

<a href="trait.ToDer.html" class="trait"
title="trait asn1_rs::ToDer">ToDer</a>

Common trait for all objects that can be encoded using the DER
representation

<a href="trait.ToStatic.html" class="trait"
title="trait asn1_rs::ToStatic">ToStatic</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.from_nom_error.html" class="fn"
title="fn asn1_rs::from_nom_error">from_nom_error</a>

Flatten all `nom::Err` variants error into a single error type

<a href="fn.parse_der_tagged_explicit.html" class="fn"
title="fn asn1_rs::parse_der_tagged_explicit">parse_der_tagged_explicit</a>

<a href="fn.parse_der_tagged_explicit_g.html" class="fn"
title="fn asn1_rs::parse_der_tagged_explicit_g">parse_der_tagged_explicit_g</a>

<a href="fn.parse_der_tagged_implicit.html" class="fn"
title="fn asn1_rs::parse_der_tagged_implicit">parse_der_tagged_implicit</a>

<a href="fn.parse_der_tagged_implicit_g.html" class="fn"
title="fn asn1_rs::parse_der_tagged_implicit_g">parse_der_tagged_implicit_g</a>

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ApplicationExplicit.html" class="type"
title="type asn1_rs::ApplicationExplicit">ApplicationExplicit</a>  
A helper object to parse `[APPLICATION n] EXPLICIT T`

<a href="type.ApplicationImplicit.html" class="type"
title="type asn1_rs::ApplicationImplicit">ApplicationImplicit</a>  
A helper object to parse `[APPLICATION n] IMPLICIT T`

<a href="type.IResult.html" class="type"
title="type asn1_rs::IResult">IResult</a>  
Holds the result of parsing functions

<a href="type.OptTaggedExplicit.html" class="type"
title="type asn1_rs::OptTaggedExplicit">OptTaggedExplicit</a>  
A helper object to parse `[ n ] EXPLICIT T OPTIONAL`

<a href="type.OptTaggedImplicit.html" class="type"
title="type asn1_rs::OptTaggedImplicit">OptTaggedImplicit</a>  
A helper object to parse `[ n ] IMPLICIT T OPTIONAL`

<a href="type.ParseResult.html" class="type"
title="type asn1_rs::ParseResult">ParseResult</a>  
Holds the result of BER/DER serialization functions

<a href="type.PrivateExplicit.html" class="type"
title="type asn1_rs::PrivateExplicit">PrivateExplicit</a>  
A helper object to parse `[PRIVATE n] EXPLICIT T`

<a href="type.PrivateImplicit.html" class="type"
title="type asn1_rs::PrivateImplicit">PrivateImplicit</a>  
A helper object to parse `[PRIVATE n] IMPLICIT T`

<a href="type.Result.html" class="type"
title="type asn1_rs::Result">Result</a>  
A specialized `Result` type for all operations from this crate.

<a href="type.SerializeResult.html" class="type"
title="type asn1_rs::SerializeResult">SerializeResult</a>  
Holds the result of BER/DER encoding functions

<a href="type.SetIterator.html" class="type"
title="type asn1_rs::SetIterator">SetIterator</a>  
An Iterator over binary data, parsing elements of type `T`

<a href="type.TaggedExplicit.html" class="type"
title="type asn1_rs::TaggedExplicit">TaggedExplicit</a>  
A helper object to parse `[ n ] EXPLICIT T`

<a href="type.TaggedImplicit.html" class="type"
title="type asn1_rs::TaggedImplicit">TaggedImplicit</a>  
A helper object to parse `[ n ] IMPLICIT T`

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.BerAlias.html" class="derive"
title="derive asn1_rs::BerAlias">BerAlias</a>  
BerAlias custom derive

<a href="derive.BerSequence.html" class="derive"
title="derive asn1_rs::BerSequence">BerSequence</a>  
BerSequence custom derive

<a href="derive.BerSet.html" class="derive"
title="derive asn1_rs::BerSet">BerSet</a>  
BerSet custom derive

<a href="derive.DerAlias.html" class="derive"
title="derive asn1_rs::DerAlias">DerAlias</a>  
DerAlias custom derive

<a href="derive.DerSequence.html" class="derive"
title="derive asn1_rs::DerSequence">DerSequence</a>  
DerSequence custom derive

<a href="derive.DerSet.html" class="derive"
title="derive asn1_rs::DerSet">DerSet</a>  
DerSet custom derive
