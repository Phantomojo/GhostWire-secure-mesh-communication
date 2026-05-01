Crate aes Copy item path

<span class="sub-heading"><a href="../src/aes/lib.rs.html#1-233" class="src">Source</a>
</span>

Expand description

Pure Rust implementation of the [Advanced Encryption
Standard](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard)
(AES, a.k.a. Rijndael).

# <a href="#-security-warning-hazmat" class="doc-anchor">§</a>⚠️ Security Warning: Hazmat!

This crate implements only the low-level block cipher function, and is
intended for use for implementing higher-level constructions *only*. It
is NOT intended for direct use in applications.

USE AT YOUR OWN RISK!

# <a href="#supported-backends" class="doc-anchor">§</a>Supported backends

This crate provides multiple backends including a portable pure Rust
backend as well as ones based on CPU intrinsics.

By default, it performs runtime detection of CPU intrinsics and uses
them if they are available.

## <a href="#soft-portable-backend" class="doc-anchor">§</a>“soft” portable backend

As a baseline implementation, this crate provides a constant-time pure
Rust implementation based on
[fixslicing](https://eprint.iacr.org/2020/1123.pdf), a more advanced
form of bitslicing implemented entirely in terms of bitwise arithmetic
with no use of any lookup tables or data-dependent branches.

Enabling the `aes_compact` configuration flag will reduce the code size
of this backend at the cost of decreased performance (using a modified
form of the fixslicing technique called “semi-fixslicing”).

## <a href="#armv8-intrinsics-rust-161" class="doc-anchor">§</a>ARMv8 intrinsics (Rust 1.61+)

On `aarch64` targets including `aarch64-apple-darwin` (Apple M1) and
Linux targets such as `aarch64-unknown-linux-gnu` and
`aarch64-unknown-linux-musl`, support for using AES intrinsics provided
by the ARMv8 Cryptography Extensions is available when using Rust 1.61
or above, and can be enabled using the `aes_armv8` configuration flag.

On Linux and macOS, when the `aes_armv8` flag is enabled support for AES
intrinsics is autodetected at runtime. On other platforms the `aes`
target feature must be enabled via RUSTFLAGS.

## <a href="#x86x86_64-intrinsics-aes-ni" class="doc-anchor">§</a>`x86`/`x86_64` intrinsics (AES-NI)

By default this crate uses runtime detection on `i686`/`x86_64` targets
in order to determine if AES-NI is available, and if it is not, it will
fallback to using a constant-time software implementation.

Passing `RUSTFLAGS=-C target-feature=+aes,+ssse3` explicitly at
compile-time will override runtime detection and ensure that AES-NI is
always used. Programs built in this manner will crash with an illegal
instruction on CPUs which do not have AES-NI enabled.

Note: runtime detection is not possible on SGX targets. Please use the
afforementioned `RUSTFLAGS` to leverage AES-NI on these targets.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

let key = GenericArray::from([0u8; 16]);
let mut block = GenericArray::from([42u8; 16]);

// Initialize cipher
let cipher = Aes128::new(&key);

let block_copy = block.clone();

// Encrypt block in-place
cipher.encrypt_block(&mut block);

// And decrypt it back
cipher.decrypt_block(&mut block);
assert_eq!(block, block_copy);

// Implementation supports parallel block processing. Number of blocks
// processed in parallel depends in general on hardware capabilities.
// This is achieved by instruction-level parallelism (ILP) on a single
// CPU core, which is differen from multi-threaded parallelism.
let mut blocks = [block; 100];
cipher.encrypt_blocks(&mut blocks);

for block in blocks.iter_mut() {
    cipher.decrypt_block(block);
    assert_eq!(block, &block_copy);
}

// `decrypt_blocks` also supports parallel block processing.
cipher.decrypt_blocks(&mut blocks);

for block in blocks.iter_mut() {
    cipher.encrypt_block(block);
    assert_eq!(block, &block_copy);
}
```

For implementation of block cipher modes of operation see
[`block-modes`](https://github.com/RustCrypto/block-modes/) repository.

# <a href="#configuration-flags" class="doc-anchor">§</a>Configuration Flags

You can modify crate using the following configuration flags:

-   `aes_armv8`: enable ARMv8 AES intrinsics (Rust 1.61+).
-   `aes_force_soft`: force software implementation.
-   `aes_compact`: reduce code size at the cost of slower performance
    (affects only software backend).

It can be enabled using `RUSTFLAGS` environmental variable (e.g.
`RUSTFLAGS="--cfg aes_compact"`) or by modifying `.cargo/config`.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use cipher;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Aes128.html" class="struct"
title="struct aes::Aes128">Aes128</a>  
AES-128 block cipher

<a href="struct.Aes192.html" class="struct"
title="struct aes::Aes192">Aes192</a>  
AES-192 block cipher

<a href="struct.Aes256.html" class="struct"
title="struct aes::Aes256">Aes256</a>  
AES-256 block cipher

<a href="struct.Aes128Dec.html" class="struct"
title="struct aes::Aes128Dec">Aes128Dec</a>  
AES-128 block cipher (decrypt-only)

<a href="struct.Aes128Enc.html" class="struct"
title="struct aes::Aes128Enc">Aes128Enc</a>  
AES-128 block cipher (encrypt-only)

<a href="struct.Aes192Dec.html" class="struct"
title="struct aes::Aes192Dec">Aes192Dec</a>  
AES-192 block cipher (decrypt-only)

<a href="struct.Aes192Enc.html" class="struct"
title="struct aes::Aes192Enc">Aes192Enc</a>  
AES-192 block cipher (encrypt-only)

<a href="struct.Aes256Dec.html" class="struct"
title="struct aes::Aes256Dec">Aes256Dec</a>  
AES-256 block cipher (decrypt-only)

<a href="struct.Aes256Enc.html" class="struct"
title="struct aes::Aes256Enc">Aes256Enc</a>  
AES-256 block cipher (encrypt-only)

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Block.html" class="type" title="type aes::Block">Block</a>  
128-bit AES block

<a href="type.Block8.html" class="type"
title="type aes::Block8">Block8</a>  
Eight 128-bit AES blocks
