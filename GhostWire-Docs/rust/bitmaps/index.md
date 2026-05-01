Crate bitmaps Copy item path

<span class="sub-heading"><a href="../src/bitmaps/lib.rs.html#5-61" class="src">Source</a>
</span>

Expand description

This crate provides the [`Bitmap`](struct.Bitmap.html) type as a
convenient and efficient way of declaring and working with fixed size
bitmaps in Rust.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
let mut bitmap: Bitmap<10> = Bitmap::new();
assert_eq!(bitmap.set(5, true), false);
assert_eq!(bitmap.set(5, true), true);
assert_eq!(bitmap.get(5), true);
assert_eq!(bitmap.get(6), false);
assert_eq!(bitmap.len(), 1);
assert_eq!(bitmap.set(3, true), false);
assert_eq!(bitmap.len(), 2);
assert_eq!(bitmap.first_index(), Some(3));
```

# <a href="#x86-arch-support" class="doc-anchor">§</a>X86 Arch Support

On `x86` and `x86_64` architectures, [`Bitmap`](struct.Bitmap.html)s of
size 256, 512, 768 and 1024 gain the
[`load_m256i()`](struct.Bitmap.html#method.load_m256i) method, which
reads the bitmap into an
[`__m256i`](https://doc.rust-lang.org/core/arch/x86_64/struct.__m256i.html)
or an array of
[`__m256i`](https://doc.rust-lang.org/core/arch/x86_64/struct.__m256i.html)
using
[`_mm256_loadu_si256()`](https://doc.rust-lang.org/core/arch/x86_64/fn._mm256_loadu_si256.html).
[`Bitmap`](struct.Bitmap.html)s of size 128 as well as the previous gain
the [`load_m128i()`](struct.Bitmap.html#method.load_m128i) method, which
does the same for
[`__m128i`](https://doc.rust-lang.org/core/arch/x86_64/struct.__m128i.html).

In addition, [`Bitmap<U128>`](struct.Bitmap.html) and
[`Bitmap<U256>`](struct.Bitmap.html) will have `From` and `Into`
implementations for
[`__m128i`](https://doc.rust-lang.org/core/arch/x86_64/struct.__m128i.html)
and
[`__m256i`](https://doc.rust-lang.org/core/arch/x86_64/struct.__m256i.html)
respectively.

Note that alignment is unaffected - your bitmaps will be aligned
appropriately for `u128`, not
[`__m128i`](https://doc.rust-lang.org/core/arch/x86_64/struct.__m128i.html)
or
[`__m256i`](https://doc.rust-lang.org/core/arch/x86_64/struct.__m256i.html),
unless you arrange for it to be otherwise. This may affect the
performance of SIMD instructions.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Bitmap.html" class="struct"
title="struct bitmaps::Bitmap">Bitmap</a>  
A compact array of bits.

<a href="struct.BitsImpl.html" class="struct"
title="struct bitmaps::BitsImpl">BitsImpl</a>  
<a href="struct.Iter.html" class="struct"
title="struct bitmaps::Iter">Iter</a>  
An iterator over the indices in a bitmap which are `true`.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.BitOps.html" class="trait"
title="trait bitmaps::BitOps">BitOps</a>  
A trait that defines generalised operations on a `Bits::Store` type.

<a href="trait.Bits.html" class="trait"
title="trait bitmaps::Bits">Bits</a>  
A type level number signifying the number of bits in a bitmap.
