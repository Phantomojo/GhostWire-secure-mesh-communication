Crate sha2 Copy item path

<span class="sub-heading"><a href="../src/sha2/lib.rs.html#1-110" class="src">Source</a>
</span>

Expand description

An implementation of the [SHA-2](https://en.wikipedia.org/wiki/SHA-2)
cryptographic hash algorithms.

There are 6 standard algorithms specified in the SHA-2 standard:
[`Sha224`](type.Sha224.html "type sha2::Sha224"),
[`Sha256`](type.Sha256.html "type sha2::Sha256"),
[`Sha512_224`](type.Sha512_224.html "type sha2::Sha512_224"),
[`Sha512_256`](type.Sha512_256.html "type sha2::Sha512_256"),
[`Sha384`](type.Sha384.html "type sha2::Sha384"), and
[`Sha512`](type.Sha512.html "type sha2::Sha512").

Algorithmically, there are only 2 core algorithms: SHA-256 and SHA-512.
All other algorithms are just applications of these with different
initial hash values, and truncated to different digest bit lengths. The
first two algorithms in the list are based on SHA-256, while the last
four are based on SHA-512.

# <a href="#usage" class="doc-anchor">§</a>Usage

## <a href="#one-shot-api" class="doc-anchor">§</a>One-shot API

``` {.rust .rust-example-rendered}
use hex_literal::hex;
use sha2::{Sha256, Digest};

let result = Sha256::digest(b"hello world");
assert_eq!(result[..], hex!("
    b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
")[..]);
```

## <a href="#incremental-api" class="doc-anchor">§</a>Incremental API

``` {.rust .rust-example-rendered}
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};

// create a Sha256 object
let mut hasher = Sha256::new();

// write input message
hasher.update(b"hello world");

// read hash digest and consume hasher
let result = hasher.finalize();

assert_eq!(result[..], hex!("
    b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
")[..]);

// same for Sha512
let mut hasher = Sha512::new();
hasher.update(b"hello world");
let result = hasher.finalize();

assert_eq!(result[..], hex!("
    309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f
    989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f
")[..]);
```

Also see [RustCrypto/hashes](https://github.com/RustCrypto/hashes)
readme.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../digest/index.html" class="mod"
title="mod digest"><code>digest</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Sha256VarCore.html" class="struct"
title="struct sha2::Sha256VarCore">Sha256VarCore</a>  
Core block-level SHA-256 hasher with variable output size.

<a href="struct.Sha512VarCore.html" class="struct"
title="struct sha2::Sha512VarCore">Sha512VarCore</a>  
Core block-level SHA-512 hasher with variable output size.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Digest.html" class="trait"
title="trait sha2::Digest">Digest</a>  
Convenience wrapper trait covering functionality of cryptographic hash
functions with fixed output size.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Sha224.html" class="type"
title="type sha2::Sha224">Sha224</a>  
SHA-224 hasher.

<a href="type.Sha256.html" class="type"
title="type sha2::Sha256">Sha256</a>  
SHA-256 hasher.

<a href="type.Sha384.html" class="type"
title="type sha2::Sha384">Sha384</a>  
SHA-384 hasher.

<a href="type.Sha512.html" class="type"
title="type sha2::Sha512">Sha512</a>  
SHA-512 hasher.

<a href="type.Sha512_224.html" class="type"
title="type sha2::Sha512_224">Sha512_224</a>  
SHA-512/224 hasher.

<a href="type.Sha512_256.html" class="type"
title="type sha2::Sha512_256">Sha512_256</a>  
SHA-512/256 hasher.
