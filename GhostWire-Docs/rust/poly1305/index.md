Crate poly1305 Copy item path

<span class="sub-heading"><a href="../src/poly1305/lib.rs.html#1-171" class="src">Source</a>
</span>

Expand description

The Poly1305 universal hash function and message authentication code.

# <a href="#about" class="doc-anchor">§</a>About

Poly1305 is a universal hash function suitable for use as a one-time
authenticator and, when combined with a cipher, a message authentication
code (MAC).

It takes a 32-byte one-time key and a message and produces a 16-byte
tag, which can be used to authenticate the message.

Poly1305 is primarily notable for its use in the
[`ChaCha20Poly1305`](https://docs.rs/chacha20poly1305) and
[`XSalsa20Poly1305`](https://docs.rs/xsalsa20poly1305) authenticated
encryption algorithms.

# <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

Rust **1.56** or higher.

Minimum supported Rust version may be changed in the future, but such
changes will be accompanied with a minor version bump.

# <a href="#security-notes" class="doc-anchor">§</a>Security Notes

This crate has received one [security audit by NCC
Group](https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/),
with no significant findings. We would like to thank
[MobileCoin](https://mobilecoin.com) for funding the audit.

NOTE: the audit predates the AVX2 backend, which has not yet been
audited.

All implementations contained in the crate are designed to execute in
constant time, either by relying on hardware intrinsics (e.g. AVX2 on
x86/x86\_64), or using a portable implementation which is only constant
time on processors which implement constant-time multiplication.

It is not suitable for use on processors with a variable-time
multiplication operation (e.g. short circuit on multiply-by-zero /
multiply-by-one, such as certain 32-bit PowerPC CPUs and some non-ARM
microcontrollers).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../universal_hash/index.html" class="mod"
title="mod universal_hash"><code>universal_hash</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Poly1305.html" class="struct"
title="struct poly1305::Poly1305">Poly1305</a>  
The Poly1305 universal hash function.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.BLOCK_SIZE.html" class="constant"
title="constant poly1305::BLOCK_SIZE">BLOCK_SIZE</a>  
Size of the blocks Poly1305 acts upon

<a href="constant.KEY_SIZE.html" class="constant"
title="constant poly1305::KEY_SIZE">KEY_SIZE</a>  
Size of a Poly1305 key

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Block.html" class="type"
title="type poly1305::Block">Block</a>  
Poly1305 blocks (16-bytes)

<a href="type.Key.html" class="type" title="type poly1305::Key">Key</a>  
Poly1305 keys (32-bytes)

<a href="type.Tag.html" class="type" title="type poly1305::Tag">Tag</a>  
Poly1305 tags (16-bytes)
