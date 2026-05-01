Crate blake3 Copy item path

<span class="sub-heading"><a href="../src/blake3/lib.rs.html#1-1835" class="src">Source</a>
</span>

Expand description

The official Rust implementation of the [BLAKE3](https://blake3.io)
cryptographic hash function.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
// Hash an input all at once.
let hash1 = blake3::hash(b"foobarbaz");

// Hash an input incrementally.
let mut hasher = blake3::Hasher::new();
hasher.update(b"foo");
hasher.update(b"bar");
hasher.update(b"baz");
let hash2 = hasher.finalize();
assert_eq!(hash1, hash2);

// Extended output. OutputReader also implements Read and Seek.
let mut output = [0; 1000];
let mut output_reader = hasher.finalize_xof();
output_reader.fill(&mut output);
assert_eq!(hash1, output[..32]);

// Print a hash as hex.
println!("{}", hash1);
```

# <a href="#cargo-features" class="doc-anchor">§</a>Cargo Features

The `std` feature (the only feature enabled by default) is required for
implementations of the
[`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) and
[`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) traits, the
[`update_reader`](struct.Hasher.html#method.update_reader "method blake3::Hasher::update_reader")
helper method, and runtime CPU feature detection on x86. If this feature
is disabled, the only way to use the x86 SIMD implementations is to
enable the corresponding instruction sets globally, with e.g.
`RUSTFLAGS="-C target-cpu=native"`. The resulting binary will not be
portable to other machines.

The `rayon` feature (disabled by default, but enabled for
[docs.rs](https://docs.rs/)) adds the
[`update_rayon`](Hasher::update_rayon) and (in combination with `mmap`
below) [`update_mmap_rayon`](Hasher::update_mmap_rayon) methods, for
multithreaded hashing. However, even if this feature is enabled, all
other APIs remain single-threaded.

The `mmap` feature (disabled by default, but enabled for
[docs.rs](https://docs.rs/)) adds the
[`update_mmap`](Hasher::update_mmap) and (in combination with `rayon`
above) [`update_mmap_rayon`](Hasher::update_mmap_rayon) helper methods
for memory-mapped IO.

The `zeroize` feature (disabled by default, but enabled for
[docs.rs](https://docs.rs/)) implements
[`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html)
for this crate’s types.

The `serde` feature (disabled by default, but enabled for
[docs.rs](https://docs.rs/)) implements
[`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html)
and
[`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html)
for [`Hash`](struct.Hash.html "struct blake3::Hash").

The NEON implementation is enabled by default for AArch64 but requires
the `neon` feature for other ARM targets. Not all ARMv7 CPUs support
NEON, and enabling this feature will produce a binary that’s not
portable to CPUs without NEON support.

The `wasm32_simd` feature enables the WASM SIMD implementation for all
`wasm32-` targets. Similar to the `neon` feature, if `wasm32_simd` is
enabled, WASM SIMD support is assumed. This may become the default in
the future.

The `traits-preview` feature enables implementations of traits from the
RustCrypto [`digest`](https://crates.io/crates/digest) crate, and
re-exports that crate as `traits::digest`. However, the traits aren’t
stable, and they’re expected to change in incompatible ways before that
crate reaches 1.0. For that reason, this crate makes no SemVer
guarantees for this feature, and callers who use it should expect
breaking changes between patch versions. (The “-preview” feature name
follows the conventions of the RustCrypto
[`signature`](https://crates.io/crates/signature) crate.)

# Modules<a href="#modules" class="anchor">§</a>

<a href="hazmat/index.html" class="mod"
title="mod blake3::hazmat">hazmat</a>  
Low-level tree manipulations and other sharp tools

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Hash.html" class="struct"
title="struct blake3::Hash">Hash</a>  
An output of the default size, 32 bytes, which provides constant-time
equality checking.

<a href="struct.Hasher.html" class="struct"
title="struct blake3::Hasher">Hasher</a>  
An incremental hash state that can accept any number of writes.

<a href="struct.HexError.html" class="struct"
title="struct blake3::HexError">HexError</a>  
The error type for
[`Hash::from_hex`](struct.Hash.html#method.from_hex "associated function blake3::Hash::from_hex").

<a href="struct.OutputReader.html" class="struct"
title="struct blake3::OutputReader">OutputReader</a>  
An incremental reader for extended output, returned by
[`Hasher::finalize_xof`](struct.Hasher.html#method.finalize_xof).

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.BLOCK_LEN.html" class="constant"
title="constant blake3::BLOCK_LEN">BLOCK_LEN</a>  
The number of bytes in a block, 64.

<a href="constant.CHUNK_LEN.html" class="constant"
title="constant blake3::CHUNK_LEN">CHUNK_LEN</a>  
The number of bytes in a chunk, 1024.

<a href="constant.KEY_LEN.html" class="constant"
title="constant blake3::KEY_LEN">KEY_LEN</a>  
The number of bytes in a key, 32.

<a href="constant.OUT_LEN.html" class="constant"
title="constant blake3::OUT_LEN">OUT_LEN</a>  
The number of bytes in a [`Hash`](struct.Hash.html), 32.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.derive_key.html" class="fn"
title="fn blake3::derive_key">derive_key</a>  
The key derivation function.

<a href="fn.hash.html" class="fn" title="fn blake3::hash">hash</a>  
The default hash function.

<a href="fn.keyed_hash.html" class="fn"
title="fn blake3::keyed_hash">keyed_hash</a>  
The keyed hash function.
