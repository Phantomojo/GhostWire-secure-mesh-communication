Crate cipher Copy item path

<span class="sub-heading"><a href="../src/cipher/lib.rs.html#1-40" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-cipher-traits" class="doc-anchor">§</a>RustCrypto: Cipher Traits

[![crate](https://img.shields.io/crates/v/cipher.svg)](https://crates.io/crates/cipher)
[![Docs](https://docs.rs/cipher/badge.svg)](https://docs.rs/cipher/)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.85+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260050-traits)
[![Build
Status](https://github.com/RustCrypto/traits/actions/workflows/cipher.yml/badge.svg?branch=master)](https://github.com/RustCrypto/traits/actions/workflows/cipher.yml?query=branch:master)

Traits which define the functionality of [block
ciphers](https://en.wikipedia.org/wiki/Block_cipher), [block
modes](https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation) and
[stream ciphers](https://en.wikipedia.org/wiki/Stream_cipher).

See
[RustCrypto/block-ciphers](https://github.com/RustCrypto/block-ciphers)
and
[RustCrypto/stream-ciphers](https://github.com/RustCrypto/stream-ciphers)
for algorithm implementations which use these traits.

## <a href="#semver-policy" class="doc-anchor">§</a>SemVer Policy

-   All on-by-default features of this library are covered by SemVer
-   MSRV is considered exempt from SemVer as noted above

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

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../crypto_common/index.html" class="mod"
title="mod crypto_common"><code>common</code></a>`;`

`pub use `<a href="../inout/index.html" class="mod"
title="mod inout"><code>inout</code></a>`;`

`pub use common::`<a href="../hybrid_array/index.html" class="mod"
title="mod hybrid_array"><code>array</code></a>`;`

`pub use common::`<a href="../typenum/index.html" class="mod"
title="mod typenum"><code>typenum</code></a>`;`

`pub use `<a href="block/index.html" class="mod"
title="mod cipher::block"><code>block</code></a>`::*;`

`pub use `<a href="stream/index.html" class="mod"
title="mod cipher::stream"><code>stream</code></a>`::*;`

`pub use `<a href="tweak/index.html" class="mod"
title="mod cipher::tweak"><code>tweak</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="block/index.html" class="mod"
title="mod cipher::block">block</a>  
Traits used to define functionality of [block
ciphers](https://en.wikipedia.org/wiki/Block_cipher) and [modes of
operation](https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation).

<a href="consts/index.html" class="mod"
title="mod cipher::consts">consts</a>  
<a href="stream/index.html" class="mod"
title="mod cipher::stream">stream</a>  
Traits which define functionality of stream ciphers.

<a href="tweak/index.html" class="mod"
title="mod cipher::tweak">tweak</a>  
Traits used to define functionality of [tweakable block
ciphers](https://people.eecs.berkeley.edu/~daw/papers/tweak-crypto02.pdf).

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Array.html" class="struct"
title="struct cipher::Array">Array</a>  
[`Array`](struct.Array.html "struct cipher::Array") is a newtype for an
inner `[T; N]` array where `N` is determined by a generic
[`ArraySize`](../hybrid_array/traits/trait.ArraySize.html "trait hybrid_array::traits::ArraySize")
parameter, which is a marker trait for a numeric value determined by
ZSTs that impl the
[`typenum::Unsigned`](../typenum/marker_traits/trait.Unsigned.html "trait typenum::marker_traits::Unsigned")
trait.

<a href="struct.InOut.html" class="struct"
title="struct cipher::InOut">InOut</a>  
Custom pointer type which contains one immutable (input) and one mutable
(output) pointer, which are either equal or non-overlapping.

<a href="struct.InOutBuf.html" class="struct"
title="struct cipher::InOutBuf">InOutBuf</a>  
Custom slice type which references one immutable (input) slice and one
mutable (output) slice of equal length. Input and output slices are
either the same or do not overlap.

<a href="struct.InvalidLength.html" class="struct"
title="struct cipher::InvalidLength">InvalidLength</a>  
The error type returned when key and/or IV used in the
[`KeyInit`](trait.KeyInit.html "trait cipher::KeyInit"),
[`KeyIvInit`](trait.KeyIvInit.html "trait cipher::KeyIvInit"), and
[`InnerIvInit`](trait.InnerIvInit.html "trait cipher::InnerIvInit")
slice-based methods had an invalid length.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AlgorithmName.html" class="trait"
title="trait cipher::AlgorithmName">AlgorithmName</a>  
Trait which stores algorithm name constant, used in `Debug`
implementations.

<a href="trait.BlockSizeUser.html" class="trait"
title="trait cipher::BlockSizeUser">BlockSizeUser</a>  
Types which process data in blocks.

<a href="trait.InnerIvInit.html" class="trait"
title="trait cipher::InnerIvInit">InnerIvInit</a>  
Types which can be initialized from another type and additional
initialization vector/nonce.

<a href="trait.IvSizeUser.html" class="trait"
title="trait cipher::IvSizeUser">IvSizeUser</a>  
Types which use initialization vector (nonce) for initialization.

<a href="trait.IvState.html" class="trait"
title="trait cipher::IvState">IvState</a>  
Trait for loading current IV state.

<a href="trait.KeyInit.html" class="trait"
title="trait cipher::KeyInit">KeyInit</a>  
Types which can be initialized from a key.

<a href="trait.KeyIvInit.html" class="trait"
title="trait cipher::KeyIvInit">KeyIvInit</a>  
Types which can be initialized from a key and initialization vector
(nonce).

<a href="trait.KeySizeUser.html" class="trait"
title="trait cipher::KeySizeUser">KeySizeUser</a>  
Types which use key for initialization.

<a href="trait.ParBlocksSizeUser.html" class="trait"
title="trait cipher::ParBlocksSizeUser">ParBlocksSizeUser</a>  
Types which can process blocks in parallel.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Block.html" class="type"
title="type cipher::Block">Block</a>  
Block on which
[`BlockSizeUser`](trait.BlockSizeUser.html "trait cipher::BlockSizeUser")
implementors operate.

<a href="type.Iv.html" class="type" title="type cipher::Iv">Iv</a>  
Initialization vector (nonce) used by
[`IvSizeUser`](trait.IvSizeUser.html "trait cipher::IvSizeUser")
implementors.

<a href="type.Key.html" class="type" title="type cipher::Key">Key</a>  
Key used by
[`KeySizeUser`](trait.KeySizeUser.html "trait cipher::KeySizeUser")
implementors.

<a href="type.ParBlocks.html" class="type"
title="type cipher::ParBlocks">ParBlocks</a>  
Parallel blocks on which
[`ParBlocksSizeUser`](trait.ParBlocksSizeUser.html "trait cipher::ParBlocksSizeUser")
implementors operate.
