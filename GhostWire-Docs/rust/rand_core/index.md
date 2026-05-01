Crate rand\_core Copy item path

<span class="sub-heading"><a href="../src/rand_core/lib.rs.html#10-771" class="src">Source</a>
</span>

Expand description

Random number generation traits

This crate is mainly of interest to crates publishing implementations of
[`RngCore`](trait.RngCore.html "trait rand_core::RngCore"). Other users
are encouraged to use the [`rand`](https://docs.rs/rand) crate instead
which re-exports the main traits and error types.

[`RngCore`](trait.RngCore.html "trait rand_core::RngCore") is the core
trait implemented by algorithmic pseudo-random number generators and
external random-number sources.

[`SeedableRng`](trait.SeedableRng.html "trait rand_core::SeedableRng")
is an extension trait for construction from fixed seeds and other random
number generators.

The [`impls`](impls/index.html "mod rand_core::impls") and
[`le`](le/index.html "mod rand_core::le") sub-modules include a few
small functions to assist implementation of
[`RngCore`](trait.RngCore.html "trait rand_core::RngCore").

# Modules<a href="#modules" class="anchor">§</a>

<a href="block/index.html" class="mod"
title="mod rand_core::block">block</a>  
The `BlockRngCore` trait and implementation helpers

<a href="impls/index.html" class="mod"
title="mod rand_core::impls">impls</a>  
Helper functions for implementing `RngCore` functions.

<a href="le/index.html" class="mod" title="mod rand_core::le">le</a>  
Little-Endian utilities

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.OsError.html" class="struct"
title="struct rand_core::OsError">OsError</a>  
Error type of [`OsRng`](struct.OsRng.html "struct rand_core::OsRng")

<a href="struct.OsRng.html" class="struct"
title="struct rand_core::OsRng">OsRng</a>  
An interface over the operating-system’s random data source

<a href="struct.RngReadAdapter.html" class="struct"
title="struct rand_core::RngReadAdapter">RngReadAdapter</a>  
Adapter that enables reading through a
[`io::Read`](https://doc.rust-lang.org/1.93.1/std/io/trait.Read.html "trait std::io::Read")
from a [`RngCore`](trait.RngCore.html "trait rand_core::RngCore").

<a href="struct.UnwrapErr.html" class="struct"
title="struct rand_core::UnwrapErr">UnwrapErr</a>  
Wrapper around
[`TryRngCore`](trait.TryRngCore.html "trait rand_core::TryRngCore")
implementation which implements
[`RngCore`](trait.RngCore.html "trait rand_core::RngCore") by panicking
on potential errors.

<a href="struct.UnwrapMut.html" class="struct"
title="struct rand_core::UnwrapMut">UnwrapMut</a>  
Wrapper around
[`TryRngCore`](trait.TryRngCore.html "trait rand_core::TryRngCore")
implementation which implements
[`RngCore`](trait.RngCore.html "trait rand_core::RngCore") by panicking
on potential errors.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.CryptoRng.html" class="trait"
title="trait rand_core::CryptoRng">CryptoRng</a>  
A marker trait over
[`RngCore`](trait.RngCore.html "trait rand_core::RngCore") for securely
unpredictable RNGs

<a href="trait.RngCore.html" class="trait"
title="trait rand_core::RngCore">RngCore</a>  
Implementation-level interface for RNGs

<a href="trait.SeedableRng.html" class="trait"
title="trait rand_core::SeedableRng">SeedableRng</a>  
A random number generator that can be explicitly seeded.

<a href="trait.TryCryptoRng.html" class="trait"
title="trait rand_core::TryCryptoRng">TryCryptoRng</a>  
A marker trait over
[`TryRngCore`](trait.TryRngCore.html "trait rand_core::TryRngCore") for
securely unpredictable RNGs

<a href="trait.TryRngCore.html" class="trait"
title="trait rand_core::TryRngCore">TryRngCore</a>  
A potentially fallible variant of
[`RngCore`](trait.RngCore.html "trait rand_core::RngCore")
