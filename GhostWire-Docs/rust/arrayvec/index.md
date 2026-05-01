Crate arrayvec Copy item path

<span class="sub-heading"><a href="../src/arrayvec/lib.rs.html#1-63" class="src">Source</a>
</span>

Expand description

**arrayvec** provides the types
[`ArrayVec`](struct.ArrayVec.html "struct arrayvec::ArrayVec") and
[`ArrayString`](struct.ArrayString.html "struct arrayvec::ArrayString"):
array-backed vector and string types, which store their contents inline.

The arrayvec package has the following cargo features:

-   `std`

    -   Optional, enabled by default
    -   Use libstd; disable to use `no_std` instead.

-   `serde`

    -   Optional
    -   Enable serialization for ArrayVec and ArrayString using serde
        1.x

-   `zeroize`

    -   Optional
    -   Implement `Zeroize` for ArrayVec and ArrayString

## <a href="#rust-version" class="doc-anchor">§</a>Rust Version

This version of arrayvec requires Rust 1.51 or later.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ArrayString.html" class="struct"
title="struct arrayvec::ArrayString">ArrayString</a>  
A string with a fixed capacity.

<a href="struct.ArrayVec.html" class="struct"
title="struct arrayvec::ArrayVec">ArrayVec</a>  
A vector with a fixed capacity.

<a href="struct.CapacityError.html" class="struct"
title="struct arrayvec::CapacityError">CapacityError</a>  
Error value indicating insufficient capacity

<a href="struct.Drain.html" class="struct"
title="struct arrayvec::Drain">Drain</a>  
A draining iterator for `ArrayVec`.

<a href="struct.IntoIter.html" class="struct"
title="struct arrayvec::IntoIter">IntoIter</a>  
By-value iterator for `ArrayVec`.
