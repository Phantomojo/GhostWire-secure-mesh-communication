Crate crypto\_common Copy item path

<span class="sub-heading"><a href="../src/crypto_common/lib.rs.html#1-382" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-common-cryptographic-traits"
class="doc-anchor">§</a>RustCrypto: Common Cryptographic Traits

[![crate](https://img.shields.io/crates/v/crypto-common.svg)](https://crates.io/crates/crypto-common)
[![Docs](https://docs.rs/crypto-common/badge.svg)](https://docs.rs/crypto-common/)
[![Build
Status](https://github.com/RustCrypto/traits/actions/workflows/crypto-common.yml/badge.svg?branch=master)](https://github.com/RustCrypto/traits/actions/workflows/crypto-common.yml?query=branch:master)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.85+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260041-hashes)

Common traits used by cryptographic algorithms. Users should generally
use higher-level trait crates instead of this one.

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

`pub use `<a href="../hybrid_array/index.html" class="mod"
title="mod hybrid_array"><code>hybrid_array</code></a>` as array;`

`pub use hybrid_array::`<a href="../typenum/index.html" class="mod"
title="mod typenum"><code>typenum</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="hazmat/index.html" class="mod"
title="mod crypto_common::hazmat">hazmat</a>  
Hazardous materials.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.InvalidKey.html" class="struct"
title="struct crypto_common::InvalidKey">InvalidKey</a>  
Error type for
[`TryKeyInit`](trait.TryKeyInit.html "trait crypto_common::TryKeyInit")
for cases where the provided bytes do not correspond to a valid key.

<a href="struct.InvalidLength.html" class="struct"
title="struct crypto_common::InvalidLength">InvalidLength</a>  
The error type returned when key and/or IV used in the
[`KeyInit`](trait.KeyInit.html "trait crypto_common::KeyInit"),
[`KeyIvInit`](trait.KeyIvInit.html "trait crypto_common::KeyIvInit"),
and
[`InnerIvInit`](trait.InnerIvInit.html "trait crypto_common::InnerIvInit")
slice-based methods had an invalid length.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AlgorithmName.html" class="trait"
title="trait crypto_common::AlgorithmName">AlgorithmName</a>  
Trait which stores algorithm name constant, used in `Debug`
implementations.

<a href="trait.BlockSizeUser.html" class="trait"
title="trait crypto_common::BlockSizeUser">BlockSizeUser</a>  
Types which process data in blocks.

<a href="trait.InnerInit.html" class="trait"
title="trait crypto_common::InnerInit">InnerInit</a>  
Types which can be initialized from another type (usually block
ciphers).

<a href="trait.InnerIvInit.html" class="trait"
title="trait crypto_common::InnerIvInit">InnerIvInit</a>  
Types which can be initialized from another type and additional
initialization vector/nonce.

<a href="trait.InnerUser.html" class="trait"
title="trait crypto_common::InnerUser">InnerUser</a>  
Types which use another type for initialization.

<a href="trait.IvSizeUser.html" class="trait"
title="trait crypto_common::IvSizeUser">IvSizeUser</a>  
Types which use initialization vector (nonce) for initialization.

<a href="trait.IvState.html" class="trait"
title="trait crypto_common::IvState">IvState</a>  
Trait for loading current IV state.

<a href="trait.KeyExport.html" class="trait"
title="trait crypto_common::KeyExport">KeyExport</a>  
Serialize a key to a byte array.

<a href="trait.KeyInit.html" class="trait"
title="trait crypto_common::KeyInit">KeyInit</a>  
Types which can be initialized from a key.

<a href="trait.KeyIvInit.html" class="trait"
title="trait crypto_common::KeyIvInit">KeyIvInit</a>  
Types which can be initialized from a key and initialization vector
(nonce).

<a href="trait.KeySizeUser.html" class="trait"
title="trait crypto_common::KeySizeUser">KeySizeUser</a>  
Types which use key for initialization.

<a href="trait.OutputSizeUser.html" class="trait"
title="trait crypto_common::OutputSizeUser">OutputSizeUser</a>  
Types which return data with the given size.

<a href="trait.ParBlocksSizeUser.html" class="trait"
title="trait crypto_common::ParBlocksSizeUser">ParBlocksSizeUser</a>  
Types which can process blocks in parallel.

<a href="trait.Reset.html" class="trait"
title="trait crypto_common::Reset">Reset</a>  
Resettable types.

<a href="trait.TryKeyInit.html" class="trait"
title="trait crypto_common::TryKeyInit">TryKeyInit</a>  
Types which can be fallibly initialized from a key.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.AddBlockSize.html" class="type"
title="type crypto_common::AddBlockSize">AddBlockSize</a>  
Alias for `AddBlockSize<A, B> = Sum<T, B::BlockSize>`

<a href="type.Block.html" class="type"
title="type crypto_common::Block">Block</a>  
Block on which
[`BlockSizeUser`](trait.BlockSizeUser.html "trait crypto_common::BlockSizeUser")
implementors operate.

<a href="type.Iv.html" class="type"
title="type crypto_common::Iv">Iv</a>  
Initialization vector (nonce) used by
[`IvSizeUser`](trait.IvSizeUser.html "trait crypto_common::IvSizeUser")
implementors.

<a href="type.Key.html" class="type"
title="type crypto_common::Key">Key</a>  
Key used by
[`KeySizeUser`](trait.KeySizeUser.html "trait crypto_common::KeySizeUser")
implementors.

<a href="type.Output.html" class="type"
title="type crypto_common::Output">Output</a>  
Output array of
[`OutputSizeUser`](trait.OutputSizeUser.html "trait crypto_common::OutputSizeUser")
implementors.

<a href="type.OutputSize.html" class="type"
title="type crypto_common::OutputSize">OutputSize</a>  
Alias for the output size of
[`OutputSizeUser`](trait.OutputSizeUser.html "trait crypto_common::OutputSizeUser")
implementors.

<a href="type.ParBlocks.html" class="type"
title="type crypto_common::ParBlocks">ParBlocks</a>  
Parallel blocks on which
[`ParBlocksSizeUser`](trait.ParBlocksSizeUser.html "trait crypto_common::ParBlocksSizeUser")
implementors operate.

<a href="type.SubBlockSize.html" class="type"
title="type crypto_common::SubBlockSize">SubBlockSize</a>  
Alias for `SubBlockSize<A, B> = Diff<T, B::BlockSize>`
