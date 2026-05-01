Crate const\_oid Copy item path

<span class="sub-heading"><a href="../src/const_oid/lib.rs.html#1-280" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-object-identifiers-oids" class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto): Object Identifiers (OIDs)

[![crate](https://buildstats.info/crate/const-oid)](https://crates.io/crates/const-oid)
[![Docs](https://docs.rs/const-oid/badge.svg)](https://docs.rs/const-oid/)
[![Build
Status](https://github.com/RustCrypto/formats/workflows/const-oid/badge.svg?branch=master&event=push)](https://github.com/RustCrypto/formats/actions)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.57+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats)

Const-friendly implementation of the ISO/IEC Object Identifier (OID)
standard as defined in ITU [X.660](https://www.itu.int/rec/T-REC-X.660),
with support for BER/DER encoding/decoding as well as heapless `no_std`
(i.e. embedded) environments.

[Documentation](https://docs.rs/const-oid/)

## <a href="#about-oids" class="doc-anchor">§</a>About OIDs

Object Identifiers, a.k.a. OIDs, are an International Telecommunications
Union (ITU) and ISO/IEC standard for naming any object, concept, or
“thing” with a globally unambiguous persistent name.

The ITU’s [X.660](https://www.itu.int/rec/T-REC-X.660) standard provides
the OID specification. Every OID is part of a hierarchical namespace
which begins with a *root OID*, which is either the ITU’s root OID (0),
the ISO’s root OID (1), or the joint ISO/ITU root OID (2).

The following is an example of an OID, in this case identifying the
`rsaEncryption` algorithm:

``` language-text
1.2.840.113549.1.1.1
```

For more information, see:
<https://en.wikipedia.org/wiki/Object_identifier>

## <a href="#implementation" class="doc-anchor">§</a>Implementation

This library supports parsing OIDs in const contexts, e.g.:

``` {.rust .rust-example-rendered}
use const_oid::ObjectIdentifier;

pub const MY_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.1");
```

The OID parser is implemented entirely in terms of `const fn` and
without the use of proc macros.

Additionally, it also includes a `const fn` OID serializer, and stores
the OIDs parsed from const contexts encoded using the BER/DER
serialization (sans header).

This allows `ObjectIdentifier` to impl `AsRef<[u8]>` which can be used
to obtain the BER/DER serialization of an OID, even one declared
`const`.

Additionally, it impls `FromStr` and `TryFrom<&[u8]>` and functions just
as well as a runtime OID library.

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

This crate requires **Rust 1.57** at a minimum.

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

# Modules<a href="#modules" class="anchor">§</a>

<a href="db/index.html" class="mod" title="mod const_oid::db">db</a>  
OID Names Database

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Arcs.html" class="struct"
title="struct const_oid::Arcs">Arcs</a>  
[`Iterator`](https://doc.rust-lang.org/1.93.1/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")
over [`Arc`](type.Arc.html "type const_oid::Arc") values (a.k.a. nodes)
in an
[`ObjectIdentifier`](struct.ObjectIdentifier.html "struct const_oid::ObjectIdentifier").

<a href="struct.ObjectIdentifier.html" class="struct"
title="struct const_oid::ObjectIdentifier">ObjectIdentifier</a>  
Object identifier (OID).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum const_oid::Error">Error</a>  
OID errors.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AssociatedOid.html" class="trait"
title="trait const_oid::AssociatedOid">AssociatedOid</a>  
A trait which associates an OID with a type.

<a href="trait.DynAssociatedOid.html" class="trait"
title="trait const_oid::DynAssociatedOid">DynAssociatedOid</a>  
A trait which associates a dynamic, `&self`-dependent OID with a type,
which may change depending on the type’s value.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Arc.html" class="type" title="type const_oid::Arc">Arc</a>  
Type alias used to represent an “arc” (i.e. integer identifier value).

<a href="type.Result.html" class="type"
title="type const_oid::Result">Result</a>  
Result type
