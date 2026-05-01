Crate serde\_spanned Copy item path

<span class="sub-heading"><a href="../src/serde_spanned/lib.rs.html#1-32" class="src">Source</a>
</span>

Expand description

A [serde](https://serde.rs/)-compatible spanned Value

This allows capturing the location, in bytes, for a value in the
original parsed document for compatible deserializers.

# Modules<a href="#modules" class="anchor">§</a>

<a href="de/index.html" class="mod" title="mod serde_spanned::de">de</a>  
Deserialization support for
[`Spanned`](struct.Spanned.html "struct serde_spanned::Spanned")

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Spanned.html" class="struct"
title="struct serde_spanned::Spanned">Spanned</a>  
A spanned value, indicating the range at which it is defined in the
source.
