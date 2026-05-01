Crate tinyvec Copy item path

<span class="sub-heading"><a href="../src/tinyvec/lib.rs.html#1-103" class="src">Source</a>
</span>

Expand description

`tinyvec` provides 100% safe vec-like data structures.

## <a href="#provided-types" class="doc-anchor">§</a>Provided Types

With no features enabled, this crate provides the
[`ArrayVec`](struct.ArrayVec.html "struct tinyvec::ArrayVec") type,
which is an array-backed storage. You can push values into the array and
pop them out of the array and so on. If the array is made to overflow it
will panic.

Similarly, there is also a
[`SliceVec`](struct.SliceVec.html "struct tinyvec::SliceVec") type
available, which is a vec-like that’s backed by a slice you provide. You
can add and remove elements, but if you overflow the slice it will
panic.

With the `alloc` feature enabled, the crate also has a
[`TinyVec`](enum.TinyVec.html "enum tinyvec::TinyVec") type. This is an
enum type which is either an `Inline(ArrayVec)` or a `Heap(Vec)`. If a
`TinyVec` is `Inline` and would overflow it automatically transitions
itself into being `Heap` mode instead of a panic.

All of this is done with no `unsafe` code within the crate. Technically
the `Vec` type from the standard library uses `unsafe` internally, but
*this crate* introduces no new `unsafe` code into your project.

The limitation is that the element type of a vec from this crate must
support the
[`Default`](https://doc.rust-lang.org/1.93.1/core/default/trait.Default.html "trait core::default::Default")
trait. This means that this crate isn’t suitable for all situations, but
a very surprising number of types do support `Default`.

## <a href="#other-features" class="doc-anchor">§</a>Other Features

-   `grab_spare_slice` lets you get access to the “inactive” portions of
    an ArrayVec.
-   `serde` provides a `Serialize` and `Deserialize` implementation for
    [`TinyVec`](enum.TinyVec.html "enum tinyvec::TinyVec") and
    [`ArrayVec`](struct.ArrayVec.html "struct tinyvec::ArrayVec") types,
    provided the inner item also has an implementation.
-   `borsh` provides a `BorshSerialize` and `BorshDeserialize`
    implementation for
    [`TinyVec`](enum.TinyVec.html "enum tinyvec::TinyVec") and
    [`ArrayVec`](struct.ArrayVec.html "struct tinyvec::ArrayVec") types,
    provided the inner item also has an implementation.

## <a href="#api" class="doc-anchor">§</a>API

The general goal of the crate is that, as much as possible, the vecs
here should be a “drop in” replacement for the standard library `Vec`
type. We strive to provide all of the `Vec` methods with the same names
and signatures. The exception is that the element type of some methods
will have a `Default` bound that’s not part of the normal `Vec` type.

The vecs here also have a few additional methods that aren’t on the
`Vec` type. In this case, the names tend to be fairly long so that they
are unlikely to clash with any future methods added to `Vec`.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.array_vec.html" class="macro"
title="macro tinyvec::array_vec">array_vec</a>  
Helper to make an `ArrayVec`.

<a href="macro.tiny_vec.html" class="macro"
title="macro tinyvec::tiny_vec">tiny_vec</a>  
Helper to make a `TinyVec`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ArrayVec.html" class="struct"
title="struct tinyvec::ArrayVec">ArrayVec</a>  
An array-backed, vector-like data structure.

<a href="struct.ArrayVecDrain.html" class="struct"
title="struct tinyvec::ArrayVecDrain">ArrayVecDrain</a>  
Draining iterator for
[`ArrayVec`](struct.ArrayVec.html "struct tinyvec::ArrayVec")

<a href="struct.ArrayVecIterator.html" class="struct"
title="struct tinyvec::ArrayVecIterator">ArrayVecIterator</a>  
Iterator for consuming an `ArrayVec` and returning owned elements.

<a href="struct.ArrayVecSplice.html" class="struct"
title="struct tinyvec::ArrayVecSplice">ArrayVecSplice</a>  
Splicing iterator for `ArrayVec` See
[`ArrayVec::splice`](struct.ArrayVec.html#method.splice "method tinyvec::ArrayVec::splice")

<a href="struct.SliceVec.html" class="struct"
title="struct tinyvec::SliceVec">SliceVec</a>  
A slice-backed vector-like data structure.

<a href="struct.SliceVecDrain.html" class="struct"
title="struct tinyvec::SliceVecDrain">SliceVecDrain</a>  
Draining iterator for
[`SliceVec`](struct.SliceVec.html "struct tinyvec::SliceVec")

<a href="struct.TinyVecSplice.html" class="struct"
title="struct tinyvec::TinyVecSplice">TinyVecSplice</a>  
Splicing iterator for `TinyVec` See
[`TinyVec::splice`](enum.TinyVec.html#method.splice "method tinyvec::TinyVec::splice")

<a href="struct.TryFromSliceError.html" class="struct"
title="struct tinyvec::TryFromSliceError">TryFromSliceError</a>  
The error type returned when a conversion from a slice to an
[`ArrayVec`](struct.ArrayVec.html "struct tinyvec::ArrayVec") fails.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.TinyVec.html" class="enum"
title="enum tinyvec::TinyVec">TinyVec</a>  
A vector that starts inline, but can automatically move to the heap.

<a href="enum.TinyVecDrain.html" class="enum"
title="enum tinyvec::TinyVecDrain">TinyVecDrain</a>  
Draining iterator for `TinyVecDrain`

<a href="enum.TinyVecIterator.html" class="enum"
title="enum tinyvec::TinyVecIterator">TinyVecIterator</a>  
Iterator for consuming an `TinyVec` and returning owned elements.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Array.html" class="trait"
title="trait tinyvec::Array">Array</a>  
A trait for types that are an array.
