Crate smallvec Copy item path

<span class="sub-heading"><a href="../src/smallvec/lib.rs.html#7-2605" class="src">Source</a>
</span>

Expand description

Small vectors in various sizes. These store a certain number of elements
inline, and fall back to the heap for larger allocations. This can be a
useful optimization for improving cache locality and reducing allocator
traffic for workloads that fit within the inline buffer.

## <a href="#no_std-support" class="doc-anchor">§</a>`no_std` support

By default, `smallvec` does not depend on `std`. However, the optional
`write` feature implements the `std::io::Write` trait for vectors of
`u8`. When this feature is enabled, `smallvec` depends on `std`.

## <a href="#optional-features" class="doc-anchor">§</a>Optional features

### <a href="#serde" class="doc-anchor">§</a>`serde`

When this optional dependency is enabled, `SmallVec` implements the
`serde::Serialize` and `serde::Deserialize` traits.

### <a href="#write" class="doc-anchor">§</a>`write`

When this feature is enabled, `SmallVec<[u8; _]>` implements the
`std::io::Write` trait. This feature is not compatible with `#![no_std]`
programs.

### <a href="#union" class="doc-anchor">§</a>`union`

**This feature requires Rust 1.49.**

When the `union` feature is enabled `smallvec` will track its state
(inline or spilled) without the use of an enum tag, reducing the size of
the `smallvec` by one machine word. This means that there is potentially
no space overhead compared to `Vec`. Note that `smallvec` can still be
larger than `Vec` if the inline buffer is larger than two machine words.

To use this feature add `features = ["union"]` in the `smallvec` section
of Cargo.toml. Note that this feature requires Rust 1.49.

Tracking issue:
[rust-lang/rust#55149](https://github.com/rust-lang/rust/issues/55149)

### <a href="#const_generics" class="doc-anchor">§</a>`const_generics`

**This feature requires Rust 1.51.**

When this feature is enabled, `SmallVec` works with any arrays of any
size, not just a fixed list of sizes.

### <a href="#const_new" class="doc-anchor">§</a>`const_new`

**This feature requires Rust 1.51.**

This feature exposes the functions
[`SmallVec::new_const`](struct.SmallVec.html#method.new_const "associated function smallvec::SmallVec::new_const"),
[`SmallVec::from_const`](struct.SmallVec.html#method.from_const "associated function smallvec::SmallVec::from_const"),
and
[`smallvec_inline`](macro.smallvec_inline.html "macro smallvec::smallvec_inline")
which enables the `SmallVec` to be initialized from a const context. For
details, see the [Rust
Reference](https://doc.rust-lang.org/reference/const_eval.html#const-functions).

### <a href="#drain_filter" class="doc-anchor">§</a>`drain_filter`

**This feature is unstable.** It may change to match the unstable
`drain_filter` method in libstd.

Enables the `drain_filter` method, which produces an iterator that calls
a user-provided closure to determine which elements of the vector to
remove and yield from the iterator.

### <a href="#drain_keep_rest" class="doc-anchor">§</a>`drain_keep_rest`

**This feature is unstable.** It may change to match the unstable
`drain_keep_rest` method in libstd.

Enables the `DrainFilter::keep_rest` method.

### <a href="#specialization" class="doc-anchor">§</a>`specialization`

**This feature is unstable and requires a nightly build of the Rust
toolchain.**

When this feature is enabled, `SmallVec::from(slice)` has improved
performance for slices of `Copy` types. (Without this feature, you can
use `SmallVec::from_slice` to get optimal performance for `Copy` types.)

Tracking issue:
[rust-lang/rust#31844](https://github.com/rust-lang/rust/issues/31844)

### <a href="#may_dangle" class="doc-anchor">§</a>`may_dangle`

**This feature is unstable and requires a nightly build of the Rust
toolchain.**

This feature makes the Rust compiler less strict about use of vectors
that contain borrowed references. For details, see the
[Rustonomicon](https://doc.rust-lang.org/1.42.0/nomicon/dropck.html#an-escape-hatch).

Tracking issue:
[rust-lang/rust#34761](https://github.com/rust-lang/rust/issues/34761)

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.smallvec.html" class="macro"
title="macro smallvec::smallvec">smallvec</a>  
Creates a [`SmallVec`](struct.SmallVec.html "struct smallvec::SmallVec")
containing the arguments.

<a href="macro.smallvec_inline.html" class="macro"
title="macro smallvec::smallvec_inline">smallvec_inline</a>  
Creates an inline
[`SmallVec`](struct.SmallVec.html "struct smallvec::SmallVec")
containing the arguments. This macro is enabled by the feature
`const_new`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Drain.html" class="struct"
title="struct smallvec::Drain">Drain</a>  
An iterator that removes the items from a `SmallVec` and yields them by
value.

<a href="struct.IntoIter.html" class="struct"
title="struct smallvec::IntoIter">IntoIter</a>  
An iterator that consumes a `SmallVec` and yields its items by value.

<a href="struct.SmallVec.html" class="struct"
title="struct smallvec::SmallVec">SmallVec</a>  
A `Vec`-like container that can store a small number of elements inline.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.CollectionAllocErr.html" class="enum"
title="enum smallvec::CollectionAllocErr">CollectionAllocErr</a>  
Error type for APIs with fallible heap allocation

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Array.html" class="trait"
title="trait smallvec::Array">Array</a>  
Types that can be used as the backing store for a
[`SmallVec`](struct.SmallVec.html "struct smallvec::SmallVec").

<a href="trait.ToSmallVec.html" class="trait"
title="trait smallvec::ToSmallVec">ToSmallVec</a>  
Convenience trait for constructing a `SmallVec`
