Crate rand Copy item path

<span class="sub-heading"><a href="../src/rand/lib.rs.html#10-360" class="src">Source</a>
</span>

Expand description

Utilities for random number generation

Rand provides utilities to generate random numbers, to convert them to
useful types and distributions, and some randomness-related algorithms.

# <a href="#quick-start" class="doc-anchor">§</a>Quick Start

``` {.rust .rust-example-rendered}
// The prelude import enables methods we use below, specifically
// Rng::random, Rng::sample, SliceRandom::shuffle and IndexedRandom::choose.
use rand::prelude::*;

// Get an RNG:
let mut rng = rand::rng();

// Try printing a random unicode code point (probably a bad idea)!
println!("char: '{}'", rng.random::<char>());
// Try printing a random alphanumeric value instead!
println!("alpha: '{}'", rng.sample(rand::distr::Alphanumeric) as char);

// Generate and shuffle a sequence:
let mut nums: Vec<i32> = (1..100).collect();
nums.shuffle(&mut rng);
// And take a random pick (yes, we didn't need to shuffle first!):
let _ = nums.choose(&mut rng);
```

# <a href="#the-book" class="doc-anchor">§</a>The Book

For the user guide and further documentation, please read [The Rust Rand
Book](https://rust-random.github.io/book).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../rand_core/index.html" class="mod"
title="mod rand_core"><code>rand_core</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="distr/index.html" class="mod" title="mod rand::distr">distr</a>  
Generating random samples from probability distributions

<a href="prelude/index.html" class="mod"
title="mod rand::prelude">prelude</a>  
Convenience re-export of common members

<a href="rngs/index.html" class="mod" title="mod rand::rngs">rngs</a>  
Random number generators and adapters

<a href="seq/index.html" class="mod" title="mod rand::seq">seq</a>  
Sequence-related functionality

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.CryptoRng.html" class="trait"
title="trait rand::CryptoRng">CryptoRng</a>  
A marker trait over
[`RngCore`](trait.RngCore.html "trait rand::RngCore") for securely
unpredictable RNGs

<a href="trait.Fill.html" class="trait"
title="trait rand::Fill">Fill</a>  
Types which may be filled with random data

<a href="trait.Rng.html" class="trait" title="trait rand::Rng">Rng</a>  
User-level interface for RNGs

<a href="trait.RngCore.html" class="trait"
title="trait rand::RngCore">RngCore</a>  
Implementation-level interface for RNGs

<a href="trait.SeedableRng.html" class="trait"
title="trait rand::SeedableRng">SeedableRng</a>  
A random number generator that can be explicitly seeded.

<a href="trait.TryCryptoRng.html" class="trait"
title="trait rand::TryCryptoRng">TryCryptoRng</a>  
A marker trait over
[`TryRngCore`](trait.TryRngCore.html "trait rand::TryRngCore") for
securely unpredictable RNGs

<a href="trait.TryRngCore.html" class="trait"
title="trait rand::TryRngCore">TryRngCore</a>  
A potentially fallible variant of
[`RngCore`](trait.RngCore.html "trait rand::RngCore")

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.fill.html" class="fn" title="fn rand::fill">fill</a>  
Fill any type implementing [`Fill`](trait.Fill.html "trait rand::Fill")
with random data

<a href="fn.random.html" class="fn" title="fn rand::random">random</a>  
Generate a random value using the thread-local random number generator.

<a href="fn.random_bool.html" class="fn"
title="fn rand::random_bool">random_bool</a>  
Return a bool with a probability `p` of being true.

<a href="fn.random_iter.html" class="fn"
title="fn rand::random_iter">random_iter</a>  
Return an iterator over [`random()`](fn.random.html "fn rand::random")
variates

<a href="fn.random_range.html" class="fn"
title="fn rand::random_range">random_range</a>  
Generate a random value in the given range using the thread-local random
number generator.

<a href="fn.random_ratio.html" class="fn"
title="fn rand::random_ratio">random_ratio</a>  
Return a bool with a probability of `numerator/denominator` of being
true.

<a href="fn.rng.html" class="fn" title="fn rand::rng">rng</a>  
Access a fast, pre-initialized generator

<a href="fn.thread_rng.html" class="fn"
title="fn rand::thread_rng">thread_rng</a><span class="stab deprecated" title="">Deprecated</span>  
Access the thread-local generator
