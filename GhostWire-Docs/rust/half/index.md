Crate half Copy item path

<span class="sub-heading"><a href="../src/half/lib.rs.html#1-283" class="src">Source</a>
</span>

Expand description

A crate that provides support for half-precision 16-bit floating point
types.

This crate provides the [`f16`](struct.f16.html "struct half::f16")
type, which is an implementation of the IEEE 754-2008 standard
[`binary16`](https://en.wikipedia.org/wiki/Half-precision_floating-point_format)
a.k.a “half” floating point type. This 16-bit floating point type is
intended for efficient storage where the full range and precision of a
larger floating point value is not required. This is especially useful
for image storage formats.

This crate also provides a
[`bf16`](struct.bf16.html "struct half::bf16") type, an alternative
16-bit floating point format. The
[`bfloat16`](https://en.wikipedia.org/wiki/Bfloat16_floating-point_format)
format is a truncated IEEE 754 standard `binary32` float that preserves
the exponent to allow the same range as
[`f32`](https://doc.rust-lang.org/1.93.1/std/primitive.f32.html "primitive f32")
but with only 8 bits of precision (instead of 11 bits for
[`f16`](struct.f16.html "struct half::f16")). See the
[`bf16`](struct.bf16.html "struct half::bf16") type for details.

Because [`f16`](struct.f16.html "struct half::f16") and
[`bf16`](struct.bf16.html "struct half::bf16") are primarily for
efficient storage, floating point operations such as addition,
multiplication, etc. are not always implemented by hardware. When
hardware does not support these operations, this crate emulates them by
converting the value to
[`f32`](https://doc.rust-lang.org/1.93.1/std/primitive.f32.html "primitive f32")
before performing the operation and then back afterward.

Note that conversion from
[`f32`](https://doc.rust-lang.org/1.93.1/std/primitive.f32.html "primitive f32")/[`f64`](https://doc.rust-lang.org/1.93.1/std/primitive.f64.html "primitive f64")
to both [`f16`](struct.f16.html "struct half::f16") and
[`bf16`](struct.bf16.html "struct half::bf16") are lossy operations, and
just as converting a
[`f64`](https://doc.rust-lang.org/1.93.1/std/primitive.f64.html "primitive f64")
to
[`f32`](https://doc.rust-lang.org/1.93.1/std/primitive.f32.html "primitive f32")
is lossy and does not have `Into`/`From` trait implementations, so too
do these smaller types not have those trait implementations either.
Instead, use `from_f32`/`from_f64` functions for the types in this
crate. If you don’t care about lossy conversions and need trait
conversions, use the appropriate
[`num-traits`](https://crates.io/crates/num-traits) traits that are
implemented.

This crate also provides a [`slice`](slice/index.html "mod half::slice")
module for zero-copy in-place conversions of
[`u16`](https://doc.rust-lang.org/1.93.1/std/primitive.u16.html "primitive u16")
slices to both [`f16`](struct.f16.html "struct half::f16") and
[`bf16`](struct.bf16.html "struct half::bf16"), as well as efficient
vectorized conversions of larger buffers of floating point values to and
from these half formats.

The crate supports `#[no_std]` when the `std` cargo feature is not
enabled, so can be used in embedded environments without using the Rust
[`std`](https://doc.rust-lang.org/std/) library. The `std` feature
enables support for the standard library and is enabled by default, see
the [Cargo Features](#cargo-features) section below.

A [`prelude`](prelude/index.html "mod half::prelude") module is provided
for easy importing of available utility traits.

# <a href="#serialization" class="doc-anchor">§</a>Serialization

When the `serde` feature is enabled,
[`f16`](struct.f16.html "struct half::f16") and
[`bf16`](struct.bf16.html "struct half::bf16") will be serialized as a
newtype of
[`u16`](https://doc.rust-lang.org/1.93.1/std/primitive.u16.html "primitive u16")
by default. In binary formats this is ideal, as it will generally use
just two bytes for storage. For string formats like JSON, however, this
isn’t as useful, and due to design limitations of serde, it’s not
possible for the default `Serialize` implementation to support different
serialization for different formats.

Instead, it’s up to the containter type of the floats to control how it
is serialized. This can easily be controlled when using the derive
macros using `#[serde(serialize_with="")]` attributes. For both
[`f16`](struct.f16.html "struct half::f16") and
[`bf16`](struct.bf16.html "struct half::bf16") a `serialize_as_f32` and
`serialize_as_string` are provided for use with this attribute.

Deserialization of both float types supports deserializing from the
default serialization, strings, and `f32`/`f64` values, so no additional
work is required.

# <a href="#hardware-support" class="doc-anchor">§</a>Hardware support

Hardware support for these conversions and arithmetic will be used
whenever hardware support is available—either through instrinsics or
targeted assembly—although a nightly Rust toolchain may be required for
some hardware. When hardware supports it the functions and traits in the
[`slice`](slice/index.html "mod half::slice") and
[`vec`](vec/index.html "mod half::vec") modules will also use vectorized
SIMD intructions for increased efficiency.

The following list details hardware support for floating point types in
this crate. When using `std` cargo feature, runtime CPU target detection
will be used. To get the most performance benefits, compile for specific
CPU features which avoids the runtime overhead and works in a `no_std`
environment.

| Architecture | CPU Target Feature | Notes |
|----|----|----|
| `x86`/`x86_64` | `f16c` | This supports conversion to/from [`f16`](struct.f16.html "struct half::f16") only (including vector SIMD) and does not support any [`bf16`](struct.bf16.html "struct half::bf16") or arithmetic operations. |
| `aarch64` | `fp16` | This supports all operations on [`f16`](struct.f16.html "struct half::f16") only. |
| `loongarch64` | `lsx` | This supports conversion to/from [`f16`](struct.f16.html "struct half::f16") only (including vector SIMD) and does not support any [`bf16`](struct.bf16.html "struct half::bf16") or arithmetic operations. |

# <a href="#cargo-features" class="doc-anchor">§</a>Cargo Features

This crate supports a number of optional cargo features. None of these
features are enabled by default, even `std`.

-   **`alloc`** — Enable use of the
    [`alloc`](https://doc.rust-lang.org/alloc/) crate when not using the
    `std` library.

    Among other functions, this enables the
    [`vec`](vec/index.html "mod half::vec") module, which contains
    zero-copy conversions for the
    [`Vec`](vec/index.html "mod half::vec") type. This allows fast
    conversion between raw `Vec<u16>` bits and `Vec<f16>` or `Vec<bf16>`
    arrays, and vice versa.

-   **`std`** — Enable features that depend on the Rust
    [`std`](https://doc.rust-lang.org/std/) library. This also enables
    the `alloc` feature automatically.

    Enabling the `std` feature enables runtime CPU feature detection of
    hardware support. Without this feature detection, harware is only
    used when compiler target supports them.

-   **`serde`** — Adds support for the
    [`serde`](https://crates.io/crates/serde) crate by implementing
    [`Serialize`](https://docs.rs/serde/*/serde/trait.Serialize.html)
    and
    [`Deserialize`](https://docs.rs/serde/*/serde/trait.Deserialize.html)
    traits for both [`f16`](struct.f16.html "struct half::f16") and
    [`bf16`](struct.bf16.html "struct half::bf16").

-   **`num-traits`** — Adds support for the
    [`num-traits`](https://crates.io/crates/num-traits) crate by
    implementing
    [`ToPrimitive`](../num_traits/cast/trait.ToPrimitive.html "trait num_traits::cast::ToPrimitive"),
    [`FromPrimitive`](../num_traits/cast/trait.FromPrimitive.html "trait num_traits::cast::FromPrimitive"),
    [`ToBytes`](../num_traits/ops/bytes/trait.ToBytes.html "trait num_traits::ops::bytes::ToBytes"),
    `FromBytes`,
    [`AsPrimitive`](../num_traits/cast/trait.AsPrimitive.html "trait num_traits::cast::AsPrimitive"),
    [`Num`](../num_traits/trait.Num.html "trait num_traits::Num"),
    [`Float`](../num_traits/float/trait.Float.html "trait num_traits::float::Float"),
    [`FloatCore`](../num_traits/float/trait.FloatCore.html "trait num_traits::float::FloatCore"),
    [`Signed`](../num_traits/sign/trait.Signed.html "trait num_traits::sign::Signed"),
    and
    [`Bounded`](../num_traits/bounds/trait.Bounded.html "trait num_traits::bounds::Bounded")
    traits for both [`f16`](struct.f16.html "struct half::f16") and
    [`bf16`](struct.bf16.html "struct half::bf16").

-   **`bytemuck`** — Adds support for the
    [`bytemuck`](https://crates.io/crates/bytemuck) crate by
    implementing
    [`Zeroable`](https://docs.rs/bytemuck/*/bytemuck/trait.Zeroable.html)
    and [`Pod`](https://docs.rs/bytemuck/*bytemuck/trait.Pod.html)
    traits for both [`f16`](struct.f16.html "struct half::f16") and
    [`bf16`](struct.bf16.html "struct half::bf16").

-   **`rand_distr`** — Adds support for the
    [`rand_distr`](https://crates.io/crates/rand_distr) crate by
    implementing
    [`Distribution`](https://docs.rs/rand/*/rand/distr/trait.Distribution.html)
    and other traits for both
    [`f16`](struct.f16.html "struct half::f16") and
    [`bf16`](struct.bf16.html "struct half::bf16").

-   **`rkyv`** – Enable zero-copy deserializtion with
    [`rkyv`]((https://crates.io/crates/rkyv)) crate.

-   **`aribtrary`** – Enable fuzzing support with
    [`arbitrary`]((https://crates.io/crates/arbitrary)) crate by
    implementing [`Arbitrary`]((https://crates.io/crates/arbitrary))
    trait.

-   **`nightly`** – Enable nightly-only features.

# Modules<a href="#modules" class="anchor">§</a>

<a href="prelude/index.html" class="mod"
title="mod half::prelude">prelude</a>  
A collection of the most used items and traits in this crate for easy
importing.

<a href="slice/index.html" class="mod" title="mod half::slice">slice</a>  
Contains utility functions and traits to convert between slices of
[`u16`](https://doc.rust-lang.org/1.93.1/std/primitive.u16.html "primitive u16")
bits and [`f16`](struct.f16.html "struct half::f16") or
[`bf16`](struct.bf16.html "struct half::bf16") numbers.

<a href="vec/index.html" class="mod" title="mod half::vec">vec</a>  
Contains utility functions and traits to convert between vectors of
[`u16`](https://doc.rust-lang.org/1.93.1/std/primitive.u16.html "primitive u16")
bits and [`f16`](struct.f16.html "struct half::f16") or
[`bf16`](struct.bf16.html "struct half::bf16") vectors.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.bf16.html" class="struct"
title="struct half::bf16">bf16</a>  
A 16-bit floating point type implementing the
[`bfloat16`](https://en.wikipedia.org/wiki/Bfloat16_floating-point_format)
format.

<a href="struct.f16.html" class="struct"
title="struct half::f16">f16</a>  
A 16-bit floating point type implementing the IEEE 754-2008 standard
[`binary16`](https://en.wikipedia.org/wiki/Half-precision_floating-point_format)
a.k.a “half” format.
