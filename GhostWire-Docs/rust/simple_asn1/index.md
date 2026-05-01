Crate simple\_asn1 Copy item path

<span class="sub-heading"><a href="../src/simple_asn1/lib.rs.html#1-1619" class="src">Source</a>
</span>

Expand description

A small ASN.1 parsing library for Rust. In particular, this library is
used to translate the binary DER encoding of an ASN.1-formatted document
into the core primitives of ASN.1. It is assumed that you can do what
you need to from there.

The critical items for this document are the traits `ToASN1` and
`FromASN1`. The first takes your data type and encodes it into a `Vec`
of simple ASN.1 structures (`ASN1Block`s). The latter inverts the
process.

Items that implement `ToASN1` can be used with the function `der_encode`
to provide single-step encoding of a data type to binary DER encoding.
Similarly, items that are `FromASN` can be single-step decoded using the
helper function `der_decode`.

You can implement one or both traits, depending on your needs. If you do
implement both, the obvious encode/decode quickcheck property is
strongly advised.

For decoding schemes that require the actual bytes associated with the
binary representation, we also provide `FromASN1WithBody`. This can be
used with the offset information in the primitive `ASN1Block`s to, for
example, validate signatures in X509 documents.

Finally, this library supports ASN.1 class information. I’m still not
sure why it’s useful, but there it is.

Please send any bug reports, patches, and curses to the GitHub
repository at `https://github.com/acw/simple_asn1`.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.oid.html" class="macro"
title="macro simple_asn1::oid">oid</a>  
A handy macro for generating OIDs from a sequence of `u64`s.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BigInt.html" class="struct"
title="struct simple_asn1::BigInt">BigInt</a>  
A big signed integer type.

<a href="struct.BigUint.html" class="struct"
title="struct simple_asn1::BigUint">BigUint</a>  
A big unsigned integer type.

<a href="struct.OID.html" class="struct"
title="struct simple_asn1::OID">OID</a>  
An ASN.1 OID.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ASN1Block.html" class="enum"
title="enum simple_asn1::ASN1Block">ASN1Block</a>  
A primitive block from ASN.1.

<a href="enum.ASN1Class.html" class="enum"
title="enum simple_asn1::ASN1Class">ASN1Class</a>  
An ASN.1 block class.

<a href="enum.ASN1DecodeErr.html" class="enum"
title="enum simple_asn1::ASN1DecodeErr">ASN1DecodeErr</a>  
An error that can arise decoding ASN.1 primitive blocks.

<a href="enum.ASN1EncodeErr.html" class="enum"
title="enum simple_asn1::ASN1EncodeErr">ASN1EncodeErr</a>  
An error that can arise encoding ASN.1 primitive blocks.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.FromASN1.html" class="trait"
title="trait simple_asn1::FromASN1">FromASN1</a>  
A trait defining types that can be decoded from an `ASN1Block` stream.
Any member of this trait is also automatically a member of
`FromASN1WithBody`, as it can obviously just ignore the body.

<a href="trait.FromASN1WithBody.html" class="trait"
title="trait simple_asn1::FromASN1WithBody">FromASN1WithBody</a>  
A trait defining types that can be decoded from an `ASN1Block` stream,
assuming they also have access to the underlying bytes making up the
stream.

<a href="trait.ToASN1.html" class="trait"
title="trait simple_asn1::ToASN1">ToASN1</a>  
The set of types that can automatically converted into a sequence of
`ASN1Block`s. You should probably use to\_asn1() but implement
to\_asn1\_class(). The former has a default implementation that passes
`ASN1Class::Universal` as the tag to use, which should be good for most
people.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.der_decode.html" class="fn"
title="fn simple_asn1::der_decode">der_decode</a>  
Automatically decode a type via DER encoding, assuming that the type is
a member of `FromASN1` or `FromASN1WithBody`.

<a href="fn.der_encode.html" class="fn"
title="fn simple_asn1::der_encode">der_encode</a>  
Automatically encode a type into binary via DER encoding, assuming that
the type is a member of `ToASN1`.

<a href="fn.from_der.html" class="fn"
title="fn simple_asn1::from_der">from_der</a>  
Translate a binary blob into a series of `ASN1Block`s, or provide an
error if it didn’t work.

<a href="fn.to_der.html" class="fn"
title="fn simple_asn1::to_der">to_der</a>  
Given an `ASN1Block`, covert it to its DER encoding, or return an error
if something broke along the way.
