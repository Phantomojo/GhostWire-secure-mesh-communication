Crate neli\_proc\_macros Copy item path

<span class="sub-heading"><a href="../src/neli_proc_macros/lib.rs.html#1-205"
class="src">Source</a> </span>

Expand description

Procedural macros to be used with the library
[`neli`](https://github.com/jbaublitz/neli).

All derive macros other than `Header` generate implicit type parameter
bounds on every type parameter which can be overriden with struct
attributes.

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.neli_enum.html" class="attr"
title="attr neli_proc_macros::neli_enum">neli_enum</a>  
Converts an enum from the form:

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.FromBytes.html" class="derive"
title="derive neli_proc_macros::FromBytes">FromBytes</a>  
Derives the neli `FromBytes` trait for a struct.

<a href="derive.FromBytesWithInput.html" class="derive"
title="derive neli_proc_macros::FromBytesWithInput">FromBytesWithInput</a>  
Derives the neli `FromBytesWithInput` trait for a struct.

<a href="derive.Header.html" class="derive"
title="derive neli_proc_macros::Header">Header</a>  
Derives the neli `Header` trait for a struct or enum. Unlike other
derive macros in this crate, the `Header` derive macro does not impose
type parameter bounds on type parameters. See the accepted attribute for
more information. The reason for this is that the last field is
considered to be the payload. Because the payload may be represented by
a type parameter, we cannot blindly restrict type parameters or else we
impose an artificial restriction of `TypeSize` on the payload type
parameter. This is a problem for the `Header` trait as the payload may
be unsized even if the rest of the header is composed exclusively of
statically sized types and are therefore compatible with the `TypeSize`
trait.

<a href="derive.Size.html" class="derive"
title="derive neli_proc_macros::Size">Size</a>  
Derives the neli `Size` trait for a struct or enum.

<a href="derive.ToBytes.html" class="derive"
title="derive neli_proc_macros::ToBytes">ToBytes</a>  
Derives the neli `ToBytes` trait for a struct or enum.
