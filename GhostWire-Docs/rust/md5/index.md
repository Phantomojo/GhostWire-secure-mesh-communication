Crate md5 Copy item path

<span class="sub-heading"><a href="../src/md5/lib.rs.html#1-144" class="src">Source</a>
</span>

Expand description

An implementation of the [MD5](https://en.wikipedia.org/wiki/MD5)
cryptographic hash algorithm.

# <a href="#usage" class="doc-anchor">§</a>Usage

``` {.rust .rust-example-rendered}
use md5::{Md5, Digest};
use hex_literal::hex;

// create a Md5 hasher instance
let mut hasher = Md5::new();

// process input message
hasher.update(b"hello world");

// acquire hash digest in the form of GenericArray,
// which in this case is equivalent to [u8; 16]
let result = hasher.finalize();
assert_eq!(result[..], hex!("5eb63bbbe01eeed093cb22bb8f5acdc3"));
```

Also see [RustCrypto/hashes](https://github.com/RustCrypto/hashes)
readme.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../digest/index.html" class="mod"
title="mod digest"><code>digest</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Md5Core.html" class="struct"
title="struct md5::Md5Core">Md5Core</a>  
Core MD5 hasher state.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Digest.html" class="trait"
title="trait md5::Digest">Digest</a>  
Convenience wrapper trait covering functionality of cryptographic hash
functions with fixed output size.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Md5.html" class="type" title="type md5::Md5">Md5</a>  
MD5 hasher state.
