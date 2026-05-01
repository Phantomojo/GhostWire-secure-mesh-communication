Crate blake2b\_simd Copy item path

<span class="sub-heading"><a href="../src/blake2b_simd/lib.rs.html#1-689" class="src">Source</a>
</span>

Expand description

[![GitHub](https://img.shields.io/github/tag/oconnor663/blake2_simd.svg?label=GitHub)](https://github.com/oconnor663/blake2_simd)
[![crates.io](https://img.shields.io/crates/v/blake2b_simd.svg)](https://crates.io/crates/blake2b_simd)
[![Actions
Status](https://github.com/oconnor663/blake2_simd/workflows/tests/badge.svg)](https://github.com/oconnor663/blake2_simd/actions)

An implementation of the BLAKE2b and BLAKE2bp hash functions. See also
[`blake2s_simd`](https://docs.rs/blake2s_simd).

This crate includes:

-   100% stable Rust.
-   SIMD implementations based on Samuel Neves’
    [`blake2-avx2`](https://github.com/sneves/blake2-avx2). These are
    very fast. For benchmarks, see [the Performance section of the
    README](https://github.com/oconnor663/blake2_simd#performance).
-   Portable, safe implementations for other platforms.
-   Dynamic CPU feature detection. Binaries include multiple
    implementations by default and choose the fastest one the processor
    supports at runtime.
-   All the features from the [the BLAKE2
    spec](https://blake2.net/blake2.pdf), like adjustable length,
    keying, and associated data for tree hashing.
-   `no_std` support. The `std` Cargo feature is on by default, for CPU
    feature detection and for implementing `std::io::Write`.
-   Support for computing multiple BLAKE2b hashes in parallel, matching
    the efficiency of BLAKE2bp. See the [`many`](many/index.html)
    module.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use blake2b_simd::{blake2b, Params};

let expected = "ca002330e69d3e6b84a46a56a6533fd79d51d97a3bb7cad6c2ff43b354185d6d\
                c1e723fb3db4ae0737e120378424c714bb982d9dc5bbd7a0ab318240ddd18f8d";
let hash = blake2b(b"foo");
assert_eq!(expected, &hash.to_hex());

let hash = Params::new()
    .hash_length(16)
    .key(b"The Magic Words are Squeamish Ossifrage")
    .personal(b"L. P. Waterhouse")
    .to_state()
    .update(b"foo")
    .update(b"bar")
    .update(b"baz")
    .finalize();
assert_eq!("ee8ff4e9be887297cf79348dc35dab56", &hash.to_hex());
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="blake2bp/index.html" class="mod"
title="mod blake2b_simd::blake2bp">blake2bp</a>  
BLAKE2bp, a variant of BLAKE2b that uses SIMD more efficiently.

<a href="many/index.html" class="mod"
title="mod blake2b_simd::many">many</a>  
Interfaces for hashing multiple inputs at once, using SIMD more
efficiently.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Hash.html" class="struct"
title="struct blake2b_simd::Hash">Hash</a>  
A finalized BLAKE2 hash, with constant-time equality.

<a href="struct.Params.html" class="struct"
title="struct blake2b_simd::Params">Params</a>  
A parameter builder that exposes all the non-default BLAKE2 features.

<a href="struct.State.html" class="struct"
title="struct blake2b_simd::State">State</a>  
An incremental hasher for BLAKE2b.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.BLOCKBYTES.html" class="constant"
title="constant blake2b_simd::BLOCKBYTES">BLOCKBYTES</a>  
The number input bytes passed to each call to the compression function.
Small benchmarks need to use an even multiple of `BLOCKBYTES`, or else
their apparent throughput will be low.

<a href="constant.KEYBYTES.html" class="constant"
title="constant blake2b_simd::KEYBYTES">KEYBYTES</a>  
The max key length.

<a href="constant.OUTBYTES.html" class="constant"
title="constant blake2b_simd::OUTBYTES">OUTBYTES</a>  
The max hash length.

<a href="constant.PERSONALBYTES.html" class="constant"
title="constant blake2b_simd::PERSONALBYTES">PERSONALBYTES</a>  
The max personalization length.

<a href="constant.SALTBYTES.html" class="constant"
title="constant blake2b_simd::SALTBYTES">SALTBYTES</a>  
The max salt length.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.blake2b.html" class="fn"
title="fn blake2b_simd::blake2b">blake2b</a>  
Compute the BLAKE2b hash of a slice of bytes all at once, using default
parameters.
