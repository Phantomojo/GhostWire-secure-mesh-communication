Crate bytemuck Copy item path

<span class="sub-heading"><a href="../src/bytemuck/lib.rs.html#1-578" class="src">Source</a>
</span>

Expand description

This crate gives small utilities for casting between plain data types.

## <a href="#basics" class="doc-anchor">§</a>Basics

Data comes in five basic forms in Rust, so we have five basic casting
functions:

-   `T` uses [`cast`](fn.cast.html "fn bytemuck::cast")
-   `&T` uses [`cast_ref`](fn.cast_ref.html "fn bytemuck::cast_ref")
-   `&mut T` uses [`cast_mut`](fn.cast_mut.html "fn bytemuck::cast_mut")
-   `&[T]` uses
    [`cast_slice`](fn.cast_slice.html "fn bytemuck::cast_slice")
-   `&mut [T]` uses
    [`cast_slice_mut`](fn.cast_slice_mut.html "fn bytemuck::cast_slice_mut")

Depending on the function, the
[`NoUninit`](trait.NoUninit.html "trait bytemuck::NoUninit") and/or
[`AnyBitPattern`](trait.AnyBitPattern.html "trait bytemuck::AnyBitPattern")
traits are used to maintain memory safety.

**Historical Note:** When the crate first started the
[`Pod`](trait.Pod.html "trait bytemuck::Pod") trait was used instead,
and so you may hear people refer to that, but it has the strongest
requirements and people eventually wanted the more fine-grained system,
so here we are. All types that impl `Pod` have a blanket impl to also
support `NoUninit` and `AnyBitPattern`. The traits unfortunately do not
have a perfectly clean hierarchy for semver reasons.

## <a href="#failures" class="doc-anchor">§</a>Failures

Some casts will never fail, and other casts might fail.

-   `cast::<u32, f32>` always works (and
    [`f32::from_bits`](https://doc.rust-lang.org/1.93.1/core/primitive.f32.html#method.from_bits "associated function f32::from_bits")).
-   `cast_ref::<[u8; 4], u32>` might fail if the specific array
    reference given at runtime doesn’t have alignment 4.

In addition to the “normal” forms of each function, which will panic on
invalid input, there’s also `try_` versions which will return a
`Result`.

If you would like to statically ensure that a cast will work at runtime
you can use the `must_cast` crate feature and the `must_` casting
functions. A “must cast” that can’t be statically known to be valid will
cause a compilation error (and sometimes a very hard to read compilation
error).

## <a href="#using-your-own-types" class="doc-anchor">§</a>Using Your Own Types

All the functions listed above are guarded by the
[`Pod`](trait.Pod.html "trait bytemuck::Pod") trait, which is a
sub-trait of the
[`Zeroable`](trait.Zeroable.html "trait bytemuck::Zeroable") trait.

If you enable the crate’s `derive` feature then these traits can be
derived on your own types. The derive macros will perform the necessary
checks on your type declaration, and trigger an error if your type does
not qualify.

The derive macros might not cover all edge cases, and sometimes they
will error when actually everything is fine. As a last resort you can
impl these traits manually. However, these traits are `unsafe`, and you
should carefully read the requirements before using a manual
implementation.

## <a href="#cargo-features" class="doc-anchor">§</a>Cargo Features

The crate supports Rust 1.34 when no features are enabled, and so
there’s cargo features for thing that you might consider “obvious”.

The cargo features **do not** promise any particular MSRV, and they may
increase their MSRV in new versions.

-   `derive`: Provide derive macros for the various traits.
-   `extern_crate_alloc`: Provide utilities for `alloc` related types
    such as Box and Vec.
-   `zeroable_maybe_uninit` and `zeroable_atomics`: Provide more
    [`Zeroable`](trait.Zeroable.html "trait bytemuck::Zeroable") impls.
-   `pod_saturating`: Provide more
    [`Pod`](trait.Pod.html "trait bytemuck::Pod") and
    [`Zeroable`](trait.Zeroable.html "trait bytemuck::Zeroable") impls.
-   `wasm_simd` and `aarch64_simd`: Support more SIMD types.
-   `min_const_generics`: Provides appropriate impls for arrays of all
    lengths instead of just for a select list of array lengths.
-   `must_cast`: Provides the `must_` functions, which will compile
    error if the requested cast can’t be statically verified.
-   `const_zeroed`: Provides a const version of the `zeroed` function.

## <a href="#related-crates" class="doc-anchor">§</a>Related Crates

-   [`pack1`](https://docs.rs/pack1), which contains
    `bytemuck`-compatible packed little-endian, big-endian and
    native-endian integer and floating point number types.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use checked::`<a href="checked/trait.CheckedBitPattern.html" class="trait"
title="trait bytemuck::checked::CheckedBitPattern"><code>CheckedBitPattern</code></a>`;`

`pub use `<a href="allocation/index.html" class="mod"
title="mod bytemuck::allocation"><code>allocation</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="allocation/index.html" class="mod"
title="mod bytemuck::allocation">allocation</a>  
Stuff to boost things in the `alloc` crate.

<a href="checked/index.html" class="mod"
title="mod bytemuck::checked">checked</a>  
Checked versions of the casting functions exposed in crate root that
support
[`CheckedBitPattern`](checked/trait.CheckedBitPattern.html "trait bytemuck::checked::CheckedBitPattern")
types.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.offset_of.html" class="macro"
title="macro bytemuck::offset_of">offset_of</a>  
Find the offset in bytes of the given `$field` of `$Type`. Requires an
already initialized `$instance` value to work with.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.PodCastError.html" class="enum"
title="enum bytemuck::PodCastError">PodCastError</a>  
The things that can go wrong when casting between
[`Pod`](trait.Pod.html "trait bytemuck::Pod") data forms.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AnyBitPattern.html" class="trait"
title="trait bytemuck::AnyBitPattern">AnyBitPattern</a>  
Marker trait for “plain old data” types that are valid for any bit
pattern.

<a href="trait.Contiguous.html" class="trait"
title="trait bytemuck::Contiguous">Contiguous</a>  
A trait indicating that:

<a href="trait.NoUninit.html" class="trait"
title="trait bytemuck::NoUninit">NoUninit</a>  
Marker trait for “plain old data” types with no uninit (or padding)
bytes.

<a href="trait.Pod.html" class="trait"
title="trait bytemuck::Pod">Pod</a>  
Marker trait for “plain old data”.

<a href="trait.PodInOption.html" class="trait"
title="trait bytemuck::PodInOption">PodInOption</a>  
Trait for types which are [Pod](trait.Pod.html "trait bytemuck::Pod")
when wrapped in
[Option](https://doc.rust-lang.org/1.93.1/core/option/enum.Option.html "enum core::option::Option").

<a href="trait.TransparentWrapper.html" class="trait"
title="trait bytemuck::TransparentWrapper">TransparentWrapper</a>  
A trait which indicates that a type is a `#[repr(transparent)]` wrapper
around the `Inner` value.

<a href="trait.Zeroable.html" class="trait"
title="trait bytemuck::Zeroable">Zeroable</a>  
Trait for types that can be safely created with
[`zeroed`](https://doc.rust-lang.org/1.93.1/core/mem/fn.zeroed.html "fn core::mem::zeroed").

<a href="trait.ZeroableInOption.html" class="trait"
title="trait bytemuck::ZeroableInOption">ZeroableInOption</a>  
Trait for types which are
[Zeroable](trait.Zeroable.html "trait bytemuck::Zeroable") when wrapped
in
[Option](https://doc.rust-lang.org/1.93.1/core/option/enum.Option.html "enum core::option::Option").

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.bytes_of.html" class="fn"
title="fn bytemuck::bytes_of">bytes_of</a>  
Re-interprets `&T` as `&[u8]`.

<a href="fn.bytes_of_mut.html" class="fn"
title="fn bytemuck::bytes_of_mut">bytes_of_mut</a>  
Re-interprets `&mut T` as `&mut [u8]`.

<a href="fn.cast.html" class="fn" title="fn bytemuck::cast">cast</a>  
Cast `A` into `B`

<a href="fn.cast_mut.html" class="fn"
title="fn bytemuck::cast_mut">cast_mut</a>  
Cast `&mut A` into `&mut B`.

<a href="fn.cast_ref.html" class="fn"
title="fn bytemuck::cast_ref">cast_ref</a>  
Cast `&A` into `&B`.

<a href="fn.cast_slice.html" class="fn"
title="fn bytemuck::cast_slice">cast_slice</a>  
Cast `&[A]` into `&[B]`.

<a href="fn.cast_slice_mut.html" class="fn"
title="fn bytemuck::cast_slice_mut">cast_slice_mut</a>  
Cast `&mut [A]` into `&mut [B]`.

<a href="fn.fill_zeroes.html" class="fn"
title="fn bytemuck::fill_zeroes">fill_zeroes</a>  
Fill all bytes of `slice` with zeroes (see
[`Zeroable`](trait.Zeroable.html "trait bytemuck::Zeroable")).

<a href="fn.from_bytes.html" class="fn"
title="fn bytemuck::from_bytes">from_bytes</a>  
Re-interprets `&[u8]` as `&T`.

<a href="fn.from_bytes_mut.html" class="fn"
title="fn bytemuck::from_bytes_mut">from_bytes_mut</a>  
Re-interprets `&mut [u8]` as `&mut T`.

<a href="fn.pod_align_to.html" class="fn"
title="fn bytemuck::pod_align_to">pod_align_to</a>  
As
[`align_to`](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to),
but safe because of the [`Pod`](trait.Pod.html "trait bytemuck::Pod")
bound.

<a href="fn.pod_align_to_mut.html" class="fn"
title="fn bytemuck::pod_align_to_mut">pod_align_to_mut</a>  
As
[`align_to_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to_mut),
but safe because of the [`Pod`](trait.Pod.html "trait bytemuck::Pod")
bound.

<a href="fn.pod_read_unaligned.html" class="fn"
title="fn bytemuck::pod_read_unaligned">pod_read_unaligned</a>  
Reads the slice into a `T` value.

<a href="fn.try_cast.html" class="fn"
title="fn bytemuck::try_cast">try_cast</a>  
Try to cast `A` into `B`.

<a href="fn.try_cast_mut.html" class="fn"
title="fn bytemuck::try_cast_mut">try_cast_mut</a>  
Try to convert a `&mut A` into `&mut B`.

<a href="fn.try_cast_ref.html" class="fn"
title="fn bytemuck::try_cast_ref">try_cast_ref</a>  
Try to convert a `&A` into `&B`.

<a href="fn.try_cast_slice.html" class="fn"
title="fn bytemuck::try_cast_slice">try_cast_slice</a>  
Try to convert `&[A]` into `&[B]` (possibly with a change in length).

<a href="fn.try_cast_slice_mut.html" class="fn"
title="fn bytemuck::try_cast_slice_mut">try_cast_slice_mut</a>  
Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in
length).

<a href="fn.try_from_bytes.html" class="fn"
title="fn bytemuck::try_from_bytes">try_from_bytes</a>  
Re-interprets `&[u8]` as `&T`.

<a href="fn.try_from_bytes_mut.html" class="fn"
title="fn bytemuck::try_from_bytes_mut">try_from_bytes_mut</a>  
Re-interprets `&mut [u8]` as `&mut T`.

<a href="fn.try_pod_read_unaligned.html" class="fn"
title="fn bytemuck::try_pod_read_unaligned">try_pod_read_unaligned</a>  
Reads from the bytes as if they were a `T`.

<a href="fn.write_zeroes.html" class="fn"
title="fn bytemuck::write_zeroes">write_zeroes</a>  
Fill all bytes of `target` with zeroes (see
[`Zeroable`](trait.Zeroable.html "trait bytemuck::Zeroable")).
