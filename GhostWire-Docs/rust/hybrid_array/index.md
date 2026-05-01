Crate hybrid\_array Copy item path

<span class="sub-heading"><a href="../src/hybrid_array/lib.rs.html#1-1227" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-hybrid-const-generic--typenum-arrays"
class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto): Hybrid Const Generic / Typenum Arrays

[![crate](https://img.shields.io/crates/v/hybrid-array?logo=rust)](https://crates.io/crates/hybrid-array)
[![Docs](https://docs.rs/hybrid-array/badge.svg)](https://docs.rs/hybrid-array/)
[![Build
Status](https://github.com/RustCrypto/hybrid-array/actions/workflows/hybrid-array.yml/badge.svg?branch=master)](https://github.com/RustCrypto/hybrid-array/actions/workflows/hybrid-array.yml?query=branch:master)
[![codecov](https://codecov.io/gh/RustCrypto/hybrid-array/graph/badge.svg?token=Z92NHFQ70G)](https://codecov.io/gh/RustCrypto/hybrid-array)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.85+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260052-utils)

Hybrid array type combining const generics with the expressiveness of
[`typenum`](https://github.com/paholg/typenum)-based constraints,
providing an alternative to
[`generic-array`](https://github.com/fizyk20/generic-array) and a
incremental transition path to const generics.

## <a href="#about" class="doc-anchor">§</a>About

This crate uses `typenum` to enable the following features which aren’t
yet possible with the stable implementation of const generics:

-   [\#60551: Associated constants in traits can not be used in const
    generics](https://github.com/rust-lang/rust/issues/60551)
-   [\#76560: Complex generic constants:
    `feature(generic_const_exprs)`](https://github.com/rust-lang/rust/issues/76560)

Internally the crate is built on const generics and provides traits
which make it possible to convert between const generic types and
`typenum` types.

## <a href="#minimum-supported-rust-version-msrv-policy"
class="doc-anchor">§</a>Minimum Supported Rust Version (MSRV) Policy

MSRV increases are not considered breaking changes and can happen in
patch releases.

The crate MSRV accounts for all supported targets and crate feature
combinations, excluding explicitly unstable features.

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

## <a href="#features" class="doc-anchor">§</a>Features

This crate exposes the following feature flags. The default is NO
features.

-   `bytemuck`: impls the `Pod` and `Zeroable` traits
-   `serde`: impls the `Deserialize` and `Serialize` traits for `Array`
-   `zeroize`: impls
    [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html)
    for `Array<T: Zeroize, U>`

## <a href="#usage" class="doc-anchor">§</a>Usage

The two core types in this crate are as follows:

-   [`Array<T, U>`](struct.Array.html "struct hybrid_array::Array"):
    wrapper for `[T; N]` where `U` is an
    [`ArraySize`](trait.ArraySize.html "trait hybrid_array::ArraySize")
    provided by [`typenum`](https://github.com/paholg/typenum) whose
    associated
    [`ArraySize::ArrayType<T>`](trait.ArraySize.html#associatedtype.ArrayType "associated type hybrid_array::ArraySize::ArrayType")
    determines the inner array size.
-   [`ArrayN<T, N>`](type.ArrayN.html "type hybrid_array::ArrayN"): type
    alias for [`Array`](struct.Array.html "struct hybrid_array::Array")
    which is const generic around `const N: usize`. This provides a
    linkage between const generics and
    [`typenum`](https://github.com/paholg/typenum).

The [`Array`](struct.Array.html "struct hybrid_array::Array") type has
an inner `pub [T; N]` field, which means writing a literal can be
expressed as follows:

``` {.rust .rust-example-rendered}
use hybrid_array::{Array, sizes::U4};

let arr: Array<u8, U4> = Array([1, 2, 3, 4]);
```

### <a href="#about-typenum" class="doc-anchor">§</a>About [`typenum`](https://github.com/paholg/typenum)

The [`typenum`](https://github.com/paholg/typenum) crate provides a
type-level implementation of numbers and arithmetic operations.

While [`typenum`](https://github.com/paholg/typenum) can be used to
express arbitrary integers using the type system, the `hybrid-array`
crate is limited to the array sizes in the
[`sizes`](sizes/index.html "mod hybrid_array::sizes") module, which have
names like [`U0`](sizes/type.U0.html "type hybrid_array::sizes::U0"),
[`U1`](sizes/type.U1.html "type hybrid_array::sizes::U1"),
[`U2`](sizes/type.U2.html "type hybrid_array::sizes::U2"),
[`U3`](sizes/type.U3.html "type hybrid_array::sizes::U3"), etc. All
supported sizes will have an impl of
[`ArraySize`](trait.ArraySize.html "trait hybrid_array::ArraySize"),
which is the trait providing linkage between
[`typenum`](https://github.com/paholg/typenum)-based types and core
arrays / const generics.

[`ArraySize`](trait.ArraySize.html "trait hybrid_array::ArraySize")
bounds on the
[`typenum::Unsigned`](../typenum/marker_traits/trait.Unsigned.html "trait typenum::marker_traits::Unsigned")
trait, which can be used to obtain integer sizes of arrays via
associated constants. For example, to obtain the size of an `ArraySize`
as a `usize`, use the associated
[`typenum::Unsigned::USIZE`](../typenum/marker_traits/trait.Unsigned.html#associatedconstant.USIZE "associated constant typenum::marker_traits::Unsigned::USIZE")
constant.

### <a href="#asarrayref-and-asarraymut-traits" class="doc-anchor">§</a>[`AsArrayRef`](trait.AsArrayRef.html "trait hybrid_array::AsArrayRef") and [`AsArrayMut`](trait.AsArrayMut.html "trait hybrid_array::AsArrayMut") traits

These traits simplify obtaining references to
[`Array`](struct.Array.html "struct hybrid_array::Array") and are impl’d
for both [`Array`](struct.Array.html "struct hybrid_array::Array") and
`[T; N]`. They’re analogous to traits like
[`AsRef`](https://doc.rust-lang.org/1.93.1/core/convert/trait.AsRef.html "trait core::convert::AsRef")
and
[`AsMut`](https://doc.rust-lang.org/1.93.1/core/convert/trait.AsMut.html "trait core::convert::AsMut").

They make it possible to write code which uses `[T; N]` or `&[T; N]` in
the external facing API which can obtain references to `&Array` and call
other functions which accept such references, without the caller having
to use `Array` in their code and while still supporting generic sizes.

For more information and a code example, see
[`AsArrayRef`](trait.AsArrayRef.html "trait hybrid_array::AsArrayRef").

## <a href="#relationship-with-generic-array" class="doc-anchor">§</a>Relationship with `generic-array`

`hybrid-array` is directly inspired by the
[`generic-array`](https://github.com/fizyk20/generic-array) crate.

However, where `generic-array` predates const generics and uses a core
which is built on `unsafe` code, `hybrid-array`’s core implementation is
built on safe code and const generic implementations. This allows the
inner `[T; N]` field of an `Array` to be `pub` as noted above, and in
general for the implementation to be significantly simpler,
easier-to-audit, and with significantly less use of `unsafe`.

The only places `hybrid-array` uses unsafe are where it is absolutely
necessary, primarily for reference conversions between `Array<T, U>` and
`[T; N]`, and also to provide features which are not yet stable in
`core`/`std`, such as
[`Array::try_from_fn`](struct.Array.html#method.try_from_fn "associated function hybrid_array::Array::try_from_fn").

## <a href="#migrating-from-generic-array" class="doc-anchor">§</a>Migrating from `generic-array`

*NOTE: this guide assumes a migration from `generic-array` v0.14*

`hybrid-array` has been designed to largely be a drop-in replacement for
`generic-array`, albeit with a public inner array type and significantly
less `unsafe` code.

The bulk of the migration work can be accomplished by making the
following find/replace-style substitutions in your `.rs` files:

-   Replace `generic_array` with `hybrid_array`
-   Replace `GenericArray<T, U>` with `Array<T, U>`
-   Replace `ArrayLength<T>` with `ArraySize`
-   Replace usages of the `Concat` and `Split` traits with
    [`Array::concat`](struct.Array.html#method.concat "method hybrid_array::Array::concat")
    and
    [`Array::split`](struct.Array.html#method.split "method hybrid_array::Array::split")
-   Replace `<U as ArrayLength<T>>::ArrayType` with
    `<U as ArraySize>::ArrayType<T>`
-   Replace usages of the `arr![N; A, B, C]` macro with
    `Array([A, B, C])`

If you have any questions, please [start a
discussion](https://github.com/RustCrypto/hybrid-array/discussions).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../typenum/index.html" class="mod"
title="mod typenum"><code>typenum</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="sizes/index.html" class="mod"
title="mod hybrid_array::sizes">sizes</a>  
Supported array sizes:
[`typenum::Unsigned`](../typenum/marker_traits/trait.Unsigned.html "trait typenum::marker_traits::Unsigned")
types with an
[`ArraySize`](trait.ArraySize.html "trait hybrid_array::ArraySize")
impl.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Array.html" class="struct"
title="struct hybrid_array::Array">Array</a>  
[`Array`](struct.Array.html "struct hybrid_array::Array") is a newtype
for an inner `[T; N]` array where `N` is determined by a generic
[`ArraySize`](trait.ArraySize.html "trait hybrid_array::ArraySize")
parameter, which is a marker trait for a numeric value determined by
ZSTs that impl the
[`typenum::Unsigned`](../typenum/marker_traits/trait.Unsigned.html "trait typenum::marker_traits::Unsigned")
trait.

<a href="struct.TryFromIteratorError.html" class="struct"
title="struct hybrid_array::TryFromIteratorError">TryFromIteratorError</a>  
Couldn’t construct an array from an iterator because the number of items
in the iterator didn’t match the array size.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ArraySize.html" class="trait"
title="trait hybrid_array::ArraySize">ArraySize</a>  
Trait which associates a
[`usize`](https://doc.rust-lang.org/1.93.1/core/primitive.usize.html "primitive usize")
size and `ArrayType` with a `typenum`-provided
[`Unsigned`](../typenum/marker_traits/trait.Unsigned.html "trait typenum::marker_traits::Unsigned")
integer.

<a href="trait.AsArrayMut.html" class="trait"
title="trait hybrid_array::AsArrayMut">AsArrayMut</a>  
Obtain a `&mut Array` reference for a given type.

<a href="trait.AsArrayRef.html" class="trait"
title="trait hybrid_array::AsArrayRef">AsArrayRef</a>  
Obtain an `&Array` reference for a given type.

<a href="trait.AssocArraySize.html" class="trait"
title="trait hybrid_array::AssocArraySize">AssocArraySize</a>  
Associates an
[`ArraySize`](trait.ArraySize.html "trait hybrid_array::ArraySize") with
a given type. Can be used to accept `[T; N]` const generic arguments and
convert to [`Array`](struct.Array.html "struct hybrid_array::Array")
internally.

<a href="trait.Flatten.html" class="trait"
title="trait hybrid_array::Flatten">Flatten</a>  
Defines a sequence of sequences that can be merged into a bigger overall
sequence.

<a href="trait.SliceExt.html" class="trait"
title="trait hybrid_array::SliceExt">SliceExt</a>  
Extension trait for `[T]` providing methods for working with
[`Array`](struct.Array.html "struct hybrid_array::Array").

<a href="trait.Unflatten.html" class="trait"
title="trait hybrid_array::Unflatten">Unflatten</a>  
Defines a sequence that can be split into a sequence of smaller
sequences of uniform size.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ArrayN.html" class="type"
title="type hybrid_array::ArrayN">ArrayN</a>  
Type alias for [`Array`](struct.Array.html "struct hybrid_array::Array")
which is const generic around a size `N`, ala `[T; N]`.
