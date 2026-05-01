Crate rand\_xoshiro Copy item path

<span class="sub-heading"><a href="../src/rand_xoshiro/lib.rs.html#9-128" class="src">Source</a>
</span>

Expand description

This crate implements the [xoshiro](http://xoshiro.di.unimi.it/) family
of pseudorandom number generators designed by David Blackman and
Sebastiano Vigna. They feature high performance and a small state and
supersede the previous xorshift-based generators. However, they are not
cryptographically secure and their output can be predicted by observing
a few samples.

The following generators are implemented:

# <a href="#64-bit-generators" class="doc-anchor">§</a>64-bit generators

-   [`Xoshiro256StarStar`](struct.Xoshiro256StarStar.html "struct rand_xoshiro::Xoshiro256StarStar"):
    Recommended for all purposes. Excellent speed and a state space (256
    bits) large enough for any parallel application.
-   [`Xoshiro256PlusPlus`](struct.Xoshiro256PlusPlus.html "struct rand_xoshiro::Xoshiro256PlusPlus"):
    Recommended for all purposes. Excellent speed and a state space (256
    bits) large enough for any parallel application.
-   [`Xoshiro256Plus`](struct.Xoshiro256Plus.html "struct rand_xoshiro::Xoshiro256Plus"):
    Recommended for generating 64-bit floating-point numbers. About 15%
    faster than `Xoshiro256StarStar`, but has a [low linear
    complexity](http://xoshiro.di.unimi.it/lowcomp.php) in the lowest
    bits (which are discarded when generating floats), making it fail
    linearity tests. This is unlikely to have any impact in practise.
-   [`Xoroshiro128StarStar`](struct.Xoroshiro128StarStar.html "struct rand_xoshiro::Xoroshiro128StarStar"):
    An alternative to `Xoshiro256StarStar`, having the same speed but
    using half the state. Only suited for low-scale parallel
    applications.
-   [`Xoroshiro128PlusPlus`](struct.Xoroshiro128PlusPlus.html "struct rand_xoshiro::Xoroshiro128PlusPlus"):
    An alternative to `Xoshiro256PlusPlus`, having the same speed but
    using half the state. Only suited for low-scale parallel
    applications.
-   [`Xoroshiro128Plus`](struct.Xoroshiro128Plus.html "struct rand_xoshiro::Xoroshiro128Plus"):
    An alternative to `Xoshiro256Plus`, having the same speed but using
    half the state. Only suited for low-scale parallel applications. Has
    a [low linear complexity](http://xoshiro.di.unimi.it/lowcomp.php) in
    the lowest bits (which are discarded when generating floats), making
    it fail linearity tests. This is unlikely to have any impact in
    practise.
-   [`Xoshiro512StarStar`](struct.Xoshiro512StarStar.html "struct rand_xoshiro::Xoshiro512StarStar"):
    An alternative to `Xoshiro256StarStar` with more state and the same
    speed.
-   [`Xoshiro512PlusPlus`](struct.Xoshiro512PlusPlus.html "struct rand_xoshiro::Xoshiro512PlusPlus"):
    An alternative to `Xoshiro256PlusPlus` with more state and the same
    speed.
-   [`Xoshiro512Plus`](struct.Xoshiro512Plus.html "struct rand_xoshiro::Xoshiro512Plus"):
    An alternative to `Xoshiro512Plus` with more state and the same
    speed. Has a [low linear
    complexity](http://xoshiro.di.unimi.it/lowcomp.php) in the lowest
    bits (which are discarded when generating floats), making it fail
    linearity tests. This is unlikely to have any impact in practise.
-   [`SplitMix64`](struct.SplitMix64.html "struct rand_xoshiro::SplitMix64"):
    Recommended for initializing generators of the xoshiro family from a
    64-bit seed. Used for implementing `seed_from_u64`.

# <a href="#32-bit-generators" class="doc-anchor">§</a>32-bit generators

-   [`Xoshiro128StarStar`](struct.Xoshiro128StarStar.html "struct rand_xoshiro::Xoshiro128StarStar"):
    Recommended for all purposes. Excellent speed.
-   [`Xoshiro128PlusPlus`](struct.Xoshiro128PlusPlus.html "struct rand_xoshiro::Xoshiro128PlusPlus"):
    Recommended for all purposes. Excellent speed.
-   [`Xoshiro128Plus`](struct.Xoshiro128Plus.html "struct rand_xoshiro::Xoshiro128Plus"):
    Recommended for generating 32-bit floating-point numbers. Faster
    than `Xoshiro128StarStar`, but has a [low linear
    complexity](http://xoshiro.di.unimi.it/lowcomp.php) in the lowest
    bits (which are discarded when generating floats), making it fail
    linearity tests. This is unlikely to have any impact in practise.
-   [`Xoroshiro64StarStar`](struct.Xoroshiro64StarStar.html "struct rand_xoshiro::Xoroshiro64StarStar"):
    An alternative to `Xoshiro128StarStar`, having the same speed but
    using half the state.
-   [`Xoroshiro64Star`](struct.Xoroshiro64Star.html "struct rand_xoshiro::Xoroshiro64Star"):
    An alternative to `Xoshiro128Plus`, having the same speed but using
    half the state. Has a [low linear
    complexity](http://xoshiro.di.unimi.it/lowcomp.php) in the lowest
    bits (which are discarded when generating floats), making it fail
    linearity tests. This is unlikely to have any impact in practise.

The `*PlusPlus` generators perform similarly to the `*StarStar`
generators. See the [xoshiro
paper](http://vigna.di.unimi.it/ftp/papers/ScrambledLinear.pdf), where
the differences are discussed in detail.

# <a href="#example" class="doc-anchor">§</a>Example

To initialize a generator, use the
[`SeedableRng`](../rand_core/trait.SeedableRng.html "trait rand_core::SeedableRng")
trait:

``` {.rust .rust-example-rendered}
use rand_core::{SeedableRng, RngCore};
use rand_xoshiro::Xoshiro256PlusPlus;

let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);
let x = rng.next_u64();
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../rand_core/index.html" class="mod"
title="mod rand_core"><code>rand_core</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Seed512.html" class="struct"
title="struct rand_xoshiro::Seed512">Seed512</a>  
512-bit seed for a generator.

<a href="struct.SplitMix64.html" class="struct"
title="struct rand_xoshiro::SplitMix64">SplitMix64</a>  
A splitmix64 random number generator.

<a href="struct.Xoroshiro64Star.html" class="struct"
title="struct rand_xoshiro::Xoroshiro64Star">Xoroshiro64Star</a>  
A xoroshiro64\* random number generator.

<a href="struct.Xoroshiro64StarStar.html" class="struct"
title="struct rand_xoshiro::Xoroshiro64StarStar">Xoroshiro64StarStar</a>  
A xoroshiro64\*\* random number generator.

<a href="struct.Xoroshiro128Plus.html" class="struct"
title="struct rand_xoshiro::Xoroshiro128Plus">Xoroshiro128Plus</a>  
A xoroshiro128+ random number generator.

<a href="struct.Xoroshiro128PlusPlus.html" class="struct"
title="struct rand_xoshiro::Xoroshiro128PlusPlus">Xoroshiro128PlusPlus</a>  
A xoroshiro128++ random number generator.

<a href="struct.Xoroshiro128StarStar.html" class="struct"
title="struct rand_xoshiro::Xoroshiro128StarStar">Xoroshiro128StarStar</a>  
A xoroshiro128\*\* random number generator.

<a href="struct.Xoshiro128Plus.html" class="struct"
title="struct rand_xoshiro::Xoshiro128Plus">Xoshiro128Plus</a>  
A xoshiro128+ random number generator.

<a href="struct.Xoshiro128PlusPlus.html" class="struct"
title="struct rand_xoshiro::Xoshiro128PlusPlus">Xoshiro128PlusPlus</a>  
A xoshiro128++ random number generator.

<a href="struct.Xoshiro128StarStar.html" class="struct"
title="struct rand_xoshiro::Xoshiro128StarStar">Xoshiro128StarStar</a>  
A xoshiro128\*\* random number generator.

<a href="struct.Xoshiro256Plus.html" class="struct"
title="struct rand_xoshiro::Xoshiro256Plus">Xoshiro256Plus</a>  
A xoshiro256+ random number generator.

<a href="struct.Xoshiro256PlusPlus.html" class="struct"
title="struct rand_xoshiro::Xoshiro256PlusPlus">Xoshiro256PlusPlus</a>  
A xoshiro256++ random number generator.

<a href="struct.Xoshiro256StarStar.html" class="struct"
title="struct rand_xoshiro::Xoshiro256StarStar">Xoshiro256StarStar</a>  
A xoshiro256\*\* random number generator.

<a href="struct.Xoshiro512Plus.html" class="struct"
title="struct rand_xoshiro::Xoshiro512Plus">Xoshiro512Plus</a>  
A xoshiro512+ random number generator.

<a href="struct.Xoshiro512PlusPlus.html" class="struct"
title="struct rand_xoshiro::Xoshiro512PlusPlus">Xoshiro512PlusPlus</a>  
A xoshiro512++ random number generator.

<a href="struct.Xoshiro512StarStar.html" class="struct"
title="struct rand_xoshiro::Xoshiro512StarStar">Xoshiro512StarStar</a>  
A xoshiro512\*\* random number generator.
