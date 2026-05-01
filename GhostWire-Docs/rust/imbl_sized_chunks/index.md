Crate imbl\_sized\_chunks Copy item path

<span class="sub-heading"><a href="../src/imbl_sized_chunks/lib.rs.html#5-146"
class="src">Source</a> </span>

Expand description

# <a href="#sized-chunks" class="doc-anchor">§</a>Sized Chunks

This crate contains three fixed size low level array like data
structures, primarily intended for use in
[immutable.rs](https://immutable.rs/), but fully supported as a
standalone crate.

Their sizing information is encoded in the type using const generics.
For example, `SparseChunk<A, 32>` would give you a sparse array with
room for 32 elements of type `A`. You can also omit the size, as they
all default to a size of 64, so `SparseChunk<A>` would be a sparse array
with a capacity of 64.

All data structures always allocate the same amount of space, as
determined by their capacity, regardless of how many elements they
contain, and when they run out of space, they will panic.

## <a href="#data-structures" class="doc-anchor">§</a>Data Structures

| Type | Description | Push | Pop | Deref to `&[A]` |
|----|----|----|----|----|
| [`Chunk`](sized_chunk/struct.Chunk.html "struct imbl_sized_chunks::sized_chunk::Chunk") | Contiguous array | O(1)/O(n) | O(1) | Yes |
| [`RingBuffer`](crate::RingBuffer) | Non-contiguous array | O(1) | O(1) | No |
| [`SparseChunk`](sparse_chunk/struct.SparseChunk.html "struct imbl_sized_chunks::sparse_chunk::SparseChunk") | Sparse array | N/A | N/A | No |

The
[`Chunk`](sized_chunk/struct.Chunk.html "struct imbl_sized_chunks::sized_chunk::Chunk")
and [`RingBuffer`](crate::RingBuffer) are very similar in practice, in
that they both work like a plain array, except that you can push to
either end with some expectation of performance. The difference is that
[`RingBuffer`](crate::RingBuffer) always allows you to do this in
constant time, but in order to give that guarantee, it doesn’t lay out
its elements contiguously in memory, which means that you can’t
dereference it to a slice `&[A]`.

[`Chunk`](sized_chunk/struct.Chunk.html "struct imbl_sized_chunks::sized_chunk::Chunk"),
on the other hand, will shift its contents around when necessary to
accommodate a push to a full side, but is able to guarantee a contiguous
memory layout in this way, so it can always be dereferenced into a
slice. Performance wise, repeated pushes to the same side will always
run in constant time, but a push to one side followed by a push to the
other side will cause the latter to run in linear time if there’s no
room (which there would only be if you’ve popped from that side).

To choose between them, you can use the following rules:

-   I only ever want to push to the back: you don’t need this crate, try
    [`ArrayVec`](https://docs.rs/arrayvec/).
-   I need to push to either side but probably not both on the same
    array: use
    [`Chunk`](sized_chunk/struct.Chunk.html "struct imbl_sized_chunks::sized_chunk::Chunk").
-   I need to push to both sides and I don’t need slices: use
    [`RingBuffer`](crate::RingBuffer).
-   I need to push to both sides but I do need slices: use
    [`Chunk`](sized_chunk/struct.Chunk.html "struct imbl_sized_chunks::sized_chunk::Chunk").

Finally,
[`SparseChunk`](sparse_chunk/struct.SparseChunk.html "struct imbl_sized_chunks::sparse_chunk::SparseChunk")
is a more efficient version of `Vec<Option<A>>`: each index is either
inhabited or not, but instead of using the `Option` discriminant to
decide which is which, it uses a compact bitmap. You can also think of
`SparseChunk<A, N>` as a `BTreeMap<usize, A>` where the `usize` must be
less than `N`, but without the performance overhead. Its API is also
more consistent with a map than an array - there’s no push, pop, append,
etc, just insert, remove and lookup.

# <a href="#inlinearray" class="doc-anchor">§</a>[`InlineArray`](inline_array/struct.InlineArray.html "struct imbl_sized_chunks::inline_array::InlineArray")

Finally, there’s
[`InlineArray`](inline_array/struct.InlineArray.html "struct imbl_sized_chunks::inline_array::InlineArray"),
which is a simple vector that’s sized to fit inside any `Sized` type
that’s big enough to hold a size counter and at least one instance of
the array element type. This can be a useful optimisation when
implementing a list like data structure with a nontrivial set of
pointers in its full form, where you could plausibly fit several
elements inside the space allocated for the pointers. `imbl::Vector` is
a good example of that, and the use case for which
[`InlineArray`](inline_array/struct.InlineArray.html "struct imbl_sized_chunks::inline_array::InlineArray")
was implemented.

# <a href="#feature-flags" class="doc-anchor">§</a>Feature Flags

The following feature flags are available:

| Feature | Description |
|----|----|
| `arbitrary` | Provides [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html) implementations from the [`arbitrary`](https://docs.rs/arbitrary) crate. Requires the `std` flag. |
| `refpool` | Provides [`PoolDefault`](https://docs.rs/refpool/latest/refpool/trait.PoolDefault.html) and [`PoolClone`](https://docs.rs/refpool/latest/refpool/trait.PoolClone.html) implemetations from the [`refpool`](https://docs.rs/refpool) crate. |
| `ringbuffer` | Enables the [`RingBuffer`](crate::RingBuffer) data structure. |
| `std` | Without this flag (enabled by default), the crate will be `no_std`, and absent traits relating to `std::collections` and `std::io`. |

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::inline_array::`<a href="inline_array/struct.InlineArray.html" class="struct"
title="struct imbl_sized_chunks::inline_array::InlineArray"><code>InlineArray</code></a>`;`

`pub use crate::sized_chunk::`<a href="sized_chunk/struct.Chunk.html" class="struct"
title="struct imbl_sized_chunks::sized_chunk::Chunk"><code>Chunk</code></a>`;`

`pub use crate::sparse_chunk::`<a href="sparse_chunk/struct.SparseChunk.html" class="struct"
title="struct imbl_sized_chunks::sparse_chunk::SparseChunk"><code>SparseChunk</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="inline_array/index.html" class="mod"
title="mod imbl_sized_chunks::inline_array">inline_array</a>  
A fixed capacity array sized to match some other type `T`.

<a href="sized_chunk/index.html" class="mod"
title="mod imbl_sized_chunks::sized_chunk">sized_chunk</a>  
A fixed capacity smart array.

<a href="sparse_chunk/index.html" class="mod"
title="mod imbl_sized_chunks::sparse_chunk">sparse_chunk</a>  
A fixed capacity sparse array.
