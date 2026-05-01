Crate fastrand Copy item path

<span class="sub-heading"><a href="../src/fastrand/lib.rs.html#1-689" class="src">Source</a>
</span>

Expand description

A simple and fast random number generator.

The implementation uses
[Wyrand](https://github.com/wangyi-fudan/wyhash), a simple and fast
generator but **not** cryptographically secure.

# <a href="#examples" class="doc-anchor">§</a>Examples

Flip a coin:

``` {.rust .rust-example-rendered}
if fastrand::bool() {
    println!("heads");
} else {
    println!("tails");
}
```

Generate a random `i32`:

``` {.rust .rust-example-rendered}
let num = fastrand::i32(..);
```

Choose a random element in an array:

``` {.rust .rust-example-rendered}
let v = vec![1, 2, 3, 4, 5];
let i = fastrand::usize(..v.len());
let elem = v[i];
```

Sample values from an array with `O(n)` complexity (`n` is the length of
array):

``` {.rust .rust-example-rendered}
fastrand::choose_multiple([1, 4, 5], 2);
fastrand::choose_multiple(0..20, 12);
```

Shuffle an array:

``` {.rust .rust-example-rendered}
let mut v = vec![1, 2, 3, 4, 5];
fastrand::shuffle(&mut v);
```

Generate a random
[`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")
or \[`String`\]:

``` {.rust .rust-example-rendered}
use std::iter::repeat_with;

let v: Vec<i32> = repeat_with(|| fastrand::i32(..)).take(10).collect();
let s: String = repeat_with(fastrand::alphanumeric).take(10).collect();
```

To get reproducible results on every run, initialize the generator with
a seed:

``` {.rust .rust-example-rendered}
// Pick an arbitrary number as seed.
fastrand::seed(7);

// Now this prints the same number on every run:
println!("{}", fastrand::u32(..));
```

To be more efficient, create a new
[`Rng`](struct.Rng.html "struct fastrand::Rng") instance instead of
using the thread-local generator:

``` {.rust .rust-example-rendered}
use std::iter::repeat_with;

let mut rng = fastrand::Rng::new();
let mut bytes: Vec<u8> = repeat_with(|| rng.u8(..)).take(10_000).collect();
```

This crate aims to expose a core set of useful randomness primitives.
For more niche algorithms, consider using the
[`fastrand-contrib`](https://crates.io/crates/fastrand-contrib) crate
alongside this one.

# <a href="#features" class="doc-anchor">§</a>Features

-   `std` (enabled by default): Enables the `std` library. This is
    required for the global generator and global entropy. Without this
    feature, [`Rng`](struct.Rng.html "struct fastrand::Rng") can only be
    instantiated using the
    [`with_seed`](struct.Rng.html#method.with_seed "associated function fastrand::Rng::with_seed")
    method.
-   `js`: Assumes that WebAssembly targets are being run in a JavaScript
    environment. See the [WebAssembly Notes](#webassembly-notes) section
    for more information.

# <a href="#webassembly-notes" class="doc-anchor">§</a>WebAssembly Notes

For non-WASI WASM targets, there is additional sublety to consider when
utilizing the global RNG. By default, `std` targets will use entropy
sources in the standard library to seed the global RNG. However, these
sources are not available by default on WASM targets outside of WASI.

If the `js` feature is enabled, this crate will assume that it is
running in a JavaScript environment. At this point, the
[`getrandom`](https://crates.io/crates/getrandom) crate will be used in
order to access the available entropy sources and seed the global RNG.
If the `js` feature is not enabled, the global RNG will use a predefined
seed.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Rng.html" class="struct"
title="struct fastrand::Rng">Rng</a>  
A random number generator.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.alphabetic.html" class="fn"
title="fn fastrand::alphabetic">alphabetic</a>  
Generates a random `char` in ranges a-z and A-Z.

<a href="fn.alphanumeric.html" class="fn"
title="fn fastrand::alphanumeric">alphanumeric</a>  
Generates a random `char` in ranges a-z, A-Z and 0-9.

<a href="fn.bool.html" class="fn" title="fn fastrand::bool">bool</a>  
Generates a random `bool`.

<a href="fn.char.html" class="fn" title="fn fastrand::char">char</a>  
Generates a random `char` in the given range.

<a href="fn.choice.html" class="fn"
title="fn fastrand::choice">choice</a>  
Choose an item from an iterator at random.

<a href="fn.choose_multiple.html" class="fn"
title="fn fastrand::choose_multiple">choose_multiple</a>  
Collects `amount` values at random from the iterable into a vector.

<a href="fn.digit.html" class="fn" title="fn fastrand::digit">digit</a>  
Generates a random digit in the given `base`.

<a href="fn.f32.html" class="fn" title="fn fastrand::f32">f32</a>  
Generates a random `f32` in range `0..1`.

<a href="fn.f64.html" class="fn" title="fn fastrand::f64">f64</a>  
Generates a random `f64` in range `0..1`.

<a href="fn.fill.html" class="fn" title="fn fastrand::fill">fill</a>  
Fill a byte slice with random data.

<a href="fn.get_seed.html" class="fn"
title="fn fastrand::get_seed">get_seed</a>  
Gives back **current** seed that is being held by the thread-local
generator.

<a href="fn.i8.html" class="fn" title="fn fastrand::i8">i8</a>  
Generates a random `i8` in the given range.

<a href="fn.i16.html" class="fn" title="fn fastrand::i16">i16</a>  
Generates a random `i16` in the given range.

<a href="fn.i32.html" class="fn" title="fn fastrand::i32">i32</a>  
Generates a random `i32` in the given range.

<a href="fn.i64.html" class="fn" title="fn fastrand::i64">i64</a>  
Generates a random `i64` in the given range.

<a href="fn.i128.html" class="fn" title="fn fastrand::i128">i128</a>  
Generates a random `i128` in the given range.

<a href="fn.isize.html" class="fn" title="fn fastrand::isize">isize</a>  
Generates a random `isize` in the given range.

<a href="fn.lowercase.html" class="fn"
title="fn fastrand::lowercase">lowercase</a>  
Generates a random `char` in range a-z.

<a href="fn.seed.html" class="fn" title="fn fastrand::seed">seed</a>  
Initializes the thread-local generator with the given seed.

<a href="fn.shuffle.html" class="fn"
title="fn fastrand::shuffle">shuffle</a>  
Shuffles a slice randomly.

<a href="fn.u8.html" class="fn" title="fn fastrand::u8">u8</a>  
Generates a random `u8` in the given range.

<a href="fn.u16.html" class="fn" title="fn fastrand::u16">u16</a>  
Generates a random `u16` in the given range.

<a href="fn.u32.html" class="fn" title="fn fastrand::u32">u32</a>  
Generates a random `u32` in the given range.

<a href="fn.u64.html" class="fn" title="fn fastrand::u64">u64</a>  
Generates a random `u64` in the given range.

<a href="fn.u128.html" class="fn" title="fn fastrand::u128">u128</a>  
Generates a random `u128` in the given range.

<a href="fn.uppercase.html" class="fn"
title="fn fastrand::uppercase">uppercase</a>  
Generates a random `char` in range A-Z.

<a href="fn.usize.html" class="fn" title="fn fastrand::usize">usize</a>  
Generates a random `usize` in the given range.
