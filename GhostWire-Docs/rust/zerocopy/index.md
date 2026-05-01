Crate zerocopy Copy item path

<span class="sub-heading"><a href="../src/zerocopy/lib.rs.html#15-6775" class="src">Source</a>
</span>

Expand description

***<span style="font-size: 140%">Fast, safe,
<span style="color:red;">compile error</span>. Pick two.</span>***

Zerocopy makes zero-cost memory manipulation effortless. We write
`unsafe` so you don’t have to.

*For an overview of what’s changed from zerocopy 0.7, check out our
[release notes](https://github.com/google/zerocopy/discussions/1680),
which include a step-by-step upgrading guide.*

*Have questions? Need more out of zerocopy? Submit a [customer request
issue](https://github.com/google/zerocopy/issues/new/choose) or ask the
maintainers on
[GitHub](https://github.com/google/zerocopy/discussions/categories/q-a)
or [Discord](https://discord.gg/MAvWH2R6zk)!*

# <a href="#overview" class="doc-anchor">§</a>Overview

##### <a href="#conversion-traits" class="doc-anchor">§</a>Conversion Traits

Zerocopy provides four derivable traits for zero-cost conversions:

-   [`TryFromBytes`](trait.TryFromBytes.html "trait zerocopy::TryFromBytes")
    indicates that a type may safely be converted from certain byte
    sequences (conditional on runtime checks)
-   [`FromZeros`](trait.FromZeros.html "trait zerocopy::FromZeros")
    indicates that a sequence of zero bytes represents a valid instance
    of a type
-   [`FromBytes`](trait.FromBytes.html "trait zerocopy::FromBytes")
    indicates that a type may safely be converted from an arbitrary byte
    sequence
-   [`IntoBytes`](trait.IntoBytes.html "trait zerocopy::IntoBytes")
    indicates that a type may safely be converted *to* a byte sequence

These traits support sized types, slices, and [slice
DSTs](trait.KnownLayout.html#dynamically-sized-types "trait zerocopy::KnownLayout").

##### <a href="#marker-traits" class="doc-anchor">§</a>Marker Traits

Zerocopy provides three derivable marker traits that do not provide any
functionality themselves, but are required to call certain methods
provided by the conversion traits:

-   [`KnownLayout`](trait.KnownLayout.html "trait zerocopy::KnownLayout")
    indicates that zerocopy can reason about certain layout qualities of
    a type
-   [`Immutable`](trait.Immutable.html "trait zerocopy::Immutable")
    indicates that a type is free from interior mutability, except by
    ownership or an exclusive (`&mut`) borrow
-   [`Unaligned`](trait.Unaligned.html "trait zerocopy::Unaligned")
    indicates that a type’s alignment requirement is 1

You should generally derive these marker traits whenever possible.

##### <a href="#conversion-macros" class="doc-anchor">§</a>Conversion Macros

Zerocopy provides six macros for safe casting between types:

-   ([`try_`](macro.try_transmute.html "macro zerocopy::try_transmute"))[`transmute`](macro.transmute.html "macro zerocopy::transmute")
    (conditionally) converts a value of one type to a value of another
    type of the same size
-   ([`try_`](macro.try_transmute_mut.html "macro zerocopy::try_transmute_mut"))[`transmute_mut`](macro.transmute_mut.html "macro zerocopy::transmute_mut")
    (conditionally) converts a mutable reference of one type to a
    mutable reference of another type of the same size
-   ([`try_`](macro.try_transmute_ref.html "macro zerocopy::try_transmute_ref"))[`transmute_ref`](macro.transmute_ref.html "macro zerocopy::transmute_ref")
    (conditionally) converts a mutable or immutable reference of one
    type to an immutable reference of another type of the same size

These macros perform *compile-time* size and alignment checks, meaning
that unconditional casts have zero cost at runtime. Conditional casts do
not need to validate size or alignment runtime, but do need to validate
contents.

These macros cannot be used in generic contexts. For generic
conversions, use the methods defined by the [conversion
traits](#conversion-traits).

##### <a href="#byteorder-aware-numerics" class="doc-anchor">§</a>Byteorder-Aware Numerics

Zerocopy provides byte-order aware integer types that support these
conversions; see the
[`byteorder`](byteorder/index.html "mod zerocopy::byteorder") module.
These types are especially useful for network parsing.

# <a href="#cargo-features" class="doc-anchor">§</a>Cargo Features

-   **`alloc`** By default, `zerocopy` is `no_std`. When the `alloc`
    feature is enabled, the `alloc` crate is added as a dependency, and
    some allocation-related functionality is added.

-   **`std`** By default, `zerocopy` is `no_std`. When the `std` feature
    is enabled, the `std` crate is added as a dependency (ie, `no_std`
    is disabled), and support for some `std` types is added. `std`
    implies `alloc`.

-   **`derive`** Provides derives for the core marker traits via the
    `zerocopy-derive` crate. These derives are re-exported from
    `zerocopy`, so it is not necessary to depend on `zerocopy-derive`
    directly.

    However, you may experience better compile times if you instead
    directly depend on both `zerocopy` and `zerocopy-derive` in your
    `Cargo.toml`, since doing so will allow Rust to compile these crates
    in parallel. To do so, do *not* enable the `derive` feature, and
    list both dependencies in your `Cargo.toml` with the same leading
    non-zero version number; e.g:

    ``` language-toml
    [dependencies]
    zerocopy = "0.X"
    zerocopy-derive = "0.X"
    ```

    To avoid the risk of [duplicate import
    errors](https://github.com/google/zerocopy/issues/1587) if one of
    your dependencies enables zerocopy’s `derive` feature, import
    derives as `use zerocopy_derive::*` rather than by name (e.g.,
    `use zerocopy_derive::FromBytes`).

-   **`simd`** When the `simd` feature is enabled, `FromZeros`,
    `FromBytes`, and `IntoBytes` impls are emitted for all stable SIMD
    types which exist on the target platform. Note that the layout of
    SIMD types is not yet stabilized, so these impls may be removed in
    the future if layout changes make them invalid. For more
    information, see the Unsafe Code Guidelines Reference page on the
    [layout of packed SIMD
    vectors](https://rust-lang.github.io/unsafe-code-guidelines/layout/packed-simd-vectors.html).

-   **`simd-nightly`** Enables the `simd` feature and adds support for
    SIMD types which are only available on nightly. Since these types
    are unstable, support for any type may be removed at any point in
    the future.

-   **`float-nightly`** Adds support for the unstable `f16` and `f128`
    types. These types are not yet fully implemented and may not be
    supported on all platforms.

# <a href="#security-ethos" class="doc-anchor">§</a>Security Ethos

Zerocopy is expressly designed for use in security-critical contexts. We
strive to ensure that that zerocopy code is sound under Rust’s current
memory model, and *any future memory model*. We ensure this by:

-   **…not ‘guessing’ about Rust’s semantics.** We annotate `unsafe`
    code with a precise rationale for its soundness that cites a
    relevant section of Rust’s official documentation. When Rust’s
    documented semantics are unclear, we work with the Rust Operational
    Semantics Team to clarify Rust’s documentation.
-   **…rigorously testing our implementation.** We run tests using
    [Miri](https://github.com/rust-lang/miri), ensuring that zerocopy is
    sound across a wide array of supported target platforms of varying
    endianness and pointer width, and across both current and
    experimental memory models of Rust.
-   **…formally proving the correctness of our implementation.** We
    apply formal verification tools like
    [Kani](https://github.com/model-checking/kani) to prove zerocopy’s
    correctness.

For more information, see our full [soundness
policy](https://github.com/google/zerocopy/blob/main/POLICIES.md#soundness).

# <a href="#relationship-to-project-safe-transmute"
class="doc-anchor">§</a>Relationship to Project Safe Transmute

[Project Safe
Transmute](https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html)
is an official initiative of the Rust Project to develop language-level
support for safer transmutation. The Project consults with crates like
zerocopy to identify aspects of safer transmutation that would benefit
from compiler support, and has developed an [experimental,
compiler-supported
analysis](https://github.com/rust-lang/compiler-team/issues/411) which
determines whether, for a given type, any value of that type may be
soundly transmuted into another type. Once this functionality is
sufficiently mature, zerocopy intends to replace its internal
transmutability analysis (implemented by our custom derives) with the
compiler-supported one. This change will likely be an implementation
detail that is invisible to zerocopy’s users.

Project Safe Transmute will not replace the need for most of zerocopy’s
higher-level abstractions. The experimental compiler analysis is a tool
for checking the soundness of `unsafe` code, not a tool to avoid writing
`unsafe` code altogether. For the foreseeable future, crates like
zerocopy will still be required in order to provide higher-level
abstractions on top of the building block provided by Project Safe
Transmute.

# <a href="#msrv" class="doc-anchor">§</a>MSRV

See our [MSRV
policy](https://github.com/google/zerocopy/blob/main/POLICIES.md#msrv).

# <a href="#changelog" class="doc-anchor">§</a>Changelog

Zerocopy uses [GitHub
Releases](https://github.com/google/zerocopy/releases).

# <a href="#thanks" class="doc-anchor">§</a>Thanks

Zerocopy is maintained by engineers at Google with help from [many
wonderful
contributors](https://github.com/google/zerocopy/graphs/contributors).
Thank you to everyone who has lent a hand in making Rust a little more
secure!

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::`<a href="byte_slice/index.html" class="mod"
title="mod zerocopy::byte_slice"><code>byte_slice</code></a>`::*;`

`pub use crate::`<a href="byteorder/index.html" class="mod"
title="mod zerocopy::byteorder"><code>byteorder</code></a>`::*;`

`pub use crate::`<a href="error/index.html" class="mod"
title="mod zerocopy::error"><code>error</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="byte_slice/index.html" class="mod"
title="mod zerocopy::byte_slice">byte_slice</a>  
Traits for types that encapsulate a `[u8]`.

<a href="byteorder/index.html" class="mod"
title="mod zerocopy::byteorder">byteorder</a>  
Byte order-aware numeric primitives.

<a href="error/index.html" class="mod"
title="mod zerocopy::error">error</a>  
Types related to error reporting.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.include_value.html" class="macro"
title="macro zerocopy::include_value">include_value</a>  
Includes a file and safely transmutes it to a value of an arbitrary
type.

<a href="macro.transmute.html" class="macro"
title="macro zerocopy::transmute">transmute</a>  
Safely transmutes a value of one type to a value of another type of the
same size.

<a href="macro.transmute_mut.html" class="macro"
title="macro zerocopy::transmute_mut">transmute_mut</a>  
Safely transmutes a mutable reference of one type to a mutable reference
of another type of the same size and compatible alignment.

<a href="macro.transmute_ref.html" class="macro"
title="macro zerocopy::transmute_ref">transmute_ref</a>  
Safely transmutes a mutable or immutable reference of one type to an
immutable reference of another type of the same size and compatible
alignment.

<a href="macro.try_transmute.html" class="macro"
title="macro zerocopy::try_transmute">try_transmute</a>  
Conditionally transmutes a value of one type to a value of another type
of the same size.

<a href="macro.try_transmute_mut.html" class="macro"
title="macro zerocopy::try_transmute_mut">try_transmute_mut</a>  
Conditionally transmutes a mutable reference of one type to a mutable
reference of another type of the same size and compatible alignment.

<a href="macro.try_transmute_ref.html" class="macro"
title="macro zerocopy::try_transmute_ref">try_transmute_ref</a>  
Conditionally transmutes a mutable or immutable reference of one type to
an immutable reference of another type of the same size and compatible
alignment.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Ref.html" class="struct"
title="struct zerocopy::Ref">Ref</a>  
A typed reference derived from a byte slice.

<a href="struct.Split.html" class="struct"
title="struct zerocopy::Split">Split</a>  
A `T` that has been split into two possibly-overlapping parts.

<a href="struct.Unalign.html" class="struct"
title="struct zerocopy::Unalign">Unalign</a>  
A type with no alignment requirement.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.FromBytes.html" class="trait"
title="trait zerocopy::FromBytes">FromBytes</a>  
Types for which any bit pattern is valid.

<a href="trait.FromZeros.html" class="trait"
title="trait zerocopy::FromZeros">FromZeros</a>  
Types for which a sequence of `0` bytes is a valid instance.

<a href="trait.Immutable.html" class="trait"
title="trait zerocopy::Immutable">Immutable</a>  
Types which are free from interior mutability.

<a href="trait.IntoBytes.html" class="trait"
title="trait zerocopy::IntoBytes">IntoBytes</a>  
Types that can be converted to an immutable slice of initialized bytes.

<a href="trait.KnownLayout.html" class="trait"
title="trait zerocopy::KnownLayout">KnownLayout</a>  
Indicates that zerocopy can reason about certain aspects of a type’s
layout.

<a href="trait.SplitAt.html" class="trait"
title="trait zerocopy::SplitAt">SplitAt</a>  
Types that can be split in two.

<a href="trait.TryFromBytes.html" class="trait"
title="trait zerocopy::TryFromBytes">TryFromBytes</a>  
Types for which some bit patterns are valid.

<a href="trait.Unaligned.html" class="trait"
title="trait zerocopy::Unaligned">Unaligned</a>  
Types with no alignment requirement.

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.ByteEq.html" class="derive"
title="derive zerocopy::ByteEq">ByteEq</a>  
Derives optimized
[`PartialEq`](https://doc.rust-lang.org/1.93.1/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")
and
[`Eq`](https://doc.rust-lang.org/1.93.1/core/cmp/trait.Eq.html "trait core::cmp::Eq")
implementations.

<a href="derive.ByteHash.html" class="derive"
title="derive zerocopy::ByteHash">ByteHash</a>  
Derives an optimized
[`Hash`](https://doc.rust-lang.org/1.93.1/core/hash/trait.Hash.html "trait core::hash::Hash")
implementation.

<a href="derive.FromBytes.html" class="derive"
title="derive zerocopy::FromBytes">FromBytes</a>  
Analyzes whether a type is
[`FromBytes`](trait.FromBytes.html "trait zerocopy::FromBytes").

<a href="derive.FromZeros.html" class="derive"
title="derive zerocopy::FromZeros">FromZeros</a>  
Analyzes whether a type is
[`FromZeros`](trait.FromZeros.html "trait zerocopy::FromZeros").

<a href="derive.Immutable.html" class="derive"
title="derive zerocopy::Immutable">Immutable</a>  
Analyzes whether a type is
[`Immutable`](trait.Immutable.html "trait zerocopy::Immutable").

<a href="derive.IntoBytes.html" class="derive"
title="derive zerocopy::IntoBytes">IntoBytes</a>  
Analyzes whether a type is
[`IntoBytes`](trait.IntoBytes.html "trait zerocopy::IntoBytes").

<a href="derive.KnownLayout.html" class="derive"
title="derive zerocopy::KnownLayout">KnownLayout</a>  
Implements
[`KnownLayout`](trait.KnownLayout.html "trait zerocopy::KnownLayout").

<a href="derive.SplitAt.html" class="derive"
title="derive zerocopy::SplitAt">SplitAt</a>  
Implements [`SplitAt`](trait.SplitAt.html "trait zerocopy::SplitAt").

<a href="derive.TryFromBytes.html" class="derive"
title="derive zerocopy::TryFromBytes">TryFromBytes</a>  
Implements
[`TryFromBytes`](trait.TryFromBytes.html "trait zerocopy::TryFromBytes").

<a href="derive.Unaligned.html" class="derive"
title="derive zerocopy::Unaligned">Unaligned</a>  
Analyzes whether a type is
[`Unaligned`](trait.Unaligned.html "trait zerocopy::Unaligned").
