Crate sha1 Copy item path

<span class="sub-heading"><a href="../src/sha1/lib.rs.html#1-154" class="src">Source</a>
</span>

Expand description

Pure Rust implementation of the
[SHA-1](https://en.wikipedia.org/wiki/SHA-1) cryptographic hash
algorithm with optional hardware-specific optimizations.

# <a href="#-warning-cryptographically-broken-" class="doc-anchor">§</a>🚨 Warning: Cryptographically Broken! 🚨

The SHA-1 hash function should be considered cryptographically broken
and unsuitable for further use in any security critical capacity, as it
is [practically vulnerable to chosen-prefix
collisions](https://sha-mbles.github.io/).

We provide this crate for legacy interoperability purposes only.

# <a href="#usage" class="doc-anchor">§</a>Usage

``` {.rust .rust-example-rendered}
use hex_literal::hex;
use sha1::{Sha1, Digest};

// create a Sha1 object
let mut hasher = Sha1::new();

// process input message
hasher.update(b"hello world");

// acquire hash digest in the form of GenericArray,
// which in this case is equivalent to [u8; 20]
let result = hasher.finalize();
assert_eq!(result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
```

Also see [RustCrypto/hashes](https://github.com/RustCrypto/hashes)
readme.

# <a href="#note-for-users-of-sha1-v06" class="doc-anchor">§</a>Note for users of `sha1 v0.6`

This crate has been transferred to the RustCrypto organization and uses
implementation previously published as the `sha-1` crate. The previous
zero dependencies version is now published as the
[`sha1_smol`](https://github.com/mitsuhiko/sha1-smol/) crate.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../digest/index.html" class="mod"
title="mod digest"><code>digest</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Sha1Core.html" class="struct"
title="struct sha1::Sha1Core">Sha1Core</a>  
Core SHA-1 hasher state.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Digest.html" class="trait"
title="trait sha1::Digest">Digest</a>  
Convenience wrapper trait covering functionality of cryptographic hash
functions with fixed output size.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Sha1.html" class="type" title="type sha1::Sha1">Sha1</a>  
SHA-1 hasher state.
