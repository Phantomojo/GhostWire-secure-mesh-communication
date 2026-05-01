Crate chacha20poly1305 Copy item path

<span class="sub-heading"><a href="../src/chacha20poly1305/lib.rs.html#1-301"
class="src">Source</a> </span>

Expand description

# <a href="#rustcrypto-chacha20poly1305" class="doc-anchor">§</a>RustCrypto: ChaCha20Poly1305

[![crate](https://buildstats.info/crate/chacha20poly1305)](https://crates.io/crates/chacha20poly1305)
[![Docs](https://docs.rs/chacha20poly1305/badge.svg)](https://docs.rs/chacha20poly1305/)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.56+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260038-AEADs)
[![Build
Status](https://github.com/RustCrypto/AEADs/workflows/chacha20poly1305/badge.svg?branch=master&event=push)](https://github.com/RustCrypto/AEADs/actions)

Pure Rust implementation of **ChaCha20Poly1305** ([RFC
8439](https://tools.ietf.org/html/rfc8439)): an [Authenticated
Encryption with Associated Data
(AEAD)](https://en.wikipedia.org/wiki/Authenticated_encryption) cipher
amenable to fast, constant-time implementations in software, based on
the
[ChaCha20](https://github.com/RustCrypto/stream-ciphers/tree/master/chacha20)
stream cipher and
[Poly1305](https://github.com/RustCrypto/universal-hashes/tree/master/poly1305)
universal hash function.

This crate also contains an implementation of **XChaCha20Poly1305**: a
variant of ChaCha20Poly1305 with an extended 192-bit (24-byte) nonce.

[Documentation](https://docs.rs/chacha20poly1305/)

## <a href="#about" class="doc-anchor">§</a>About

ChaCha20Poly1305 is notable for being simple and fast when implemented
in pure software. The underlying ChaCha20 stream cipher uses a simple
combination of add, rotate, and XOR instructions (a.k.a. “ARX”), and the
Poly1305 hash function is likewise extremely simple.

While it hasn’t received approval from certain standards bodies (i.e.
NIST) the algorithm is widely used and deployed. Notably it’s mandatory
to implement in the Transport Layer Security (TLS) protocol. The
underlying ChaCha20 cipher is also widely used as a cryptographically
secure random number generator, including internal use by the Rust
standard library.

## <a href="#security-notes" class="doc-anchor">§</a>Security Notes

This crate has received one [security audit by NCC
Group](https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/),
with no significant findings. We would like to thank
[MobileCoin](https://www.mobilecoin.com/) for funding the audit.

All implementations contained in the crate are designed to execute in
constant time, either by relying on hardware intrinsics (i.e. AVX2 on
x86/x86\_64), or using a portable implementation which is only constant
time on processors which implement constant-time multiplication.

It is not suitable for use on processors with a variable-time
multiplication operation (e.g. short circuit on multiply-by-zero /
multiply-by-one, such as certain 32-bit PowerPC CPUs and some non-ARM
microcontrollers).

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

## <a href="#supported-algorithms" class="doc-anchor">§</a>Supported Algorithms

This crate contains pure Rust implementations of
[`ChaCha20Poly1305`](type.ChaCha20Poly1305.html "type chacha20poly1305::ChaCha20Poly1305")
(with optional AVX2 acceleration) as well as the following variants
thereof:

-   [`XChaCha20Poly1305`](type.XChaCha20Poly1305.html "type chacha20poly1305::XChaCha20Poly1305") -
    ChaCha20Poly1305 variant with an extended 192-bit (24-byte) nonce.
-   \[`ChaCha8Poly1305`\] / \[`ChaCha12Poly1305`\] - non-standard,
    reduced-round variants (gated under the `reduced-round` Cargo
    feature). See the [Too Much
    Crypto](https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/)
    paper for background and rationale on when these constructions could
    be used. When in doubt, prefer
    [`ChaCha20Poly1305`](type.ChaCha20Poly1305.html "type chacha20poly1305::ChaCha20Poly1305").
-   \[`XChaCha8Poly1305`\] / \[`XChaCha12Poly1305`\] - same as above,
    but with an extended 192-bit (24-byte) nonce.

# <a href="#usage" class="doc-anchor">§</a>Usage

``` {.rust .rust-example-rendered}
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce
};

let key = ChaCha20Poly1305::generate_key(&mut OsRng);
let cipher = ChaCha20Poly1305::new(&key);
let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
assert_eq!(&plaintext, b"plaintext message");
```

## <a href="#in-place-usage-eliminates-alloc-requirement"
class="doc-anchor">§</a>In-place Usage (eliminates `alloc` requirement)

This crate has an optional `alloc` feature which can be disabled in e.g.
microcontroller environments that don’t have a heap.

The
[`AeadInPlace::encrypt_in_place`](trait.AeadInPlace.html#method.encrypt_in_place "method chacha20poly1305::AeadInPlace::encrypt_in_place")
and
[`AeadInPlace::decrypt_in_place`](trait.AeadInPlace.html#method.decrypt_in_place "method chacha20poly1305::AeadInPlace::decrypt_in_place")
methods accept any type that impls the
[`aead::Buffer`](../aead/trait.Buffer.html "trait aead::Buffer") trait
which contains the plaintext for encryption or ciphertext for
decryption.

Note that if you enable the `heapless` feature of this crate, you will
receive an impl of
[`aead::Buffer`](../aead/trait.Buffer.html "trait aead::Buffer") for
`heapless::Vec` (re-exported from the
[`aead`](../aead/index.html "mod aead") crate as
\[`aead::heapless::Vec`\]), which can then be passed as the `buffer`
parameter to the in-place encrypt and decrypt methods:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use chacha20poly1305::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng, heapless::Vec},
    ChaCha20Poly1305, Nonce,
};

let key = ChaCha20Poly1305::generate_key(&mut OsRng);
let cipher = ChaCha20Poly1305::new(&key);
let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

let mut buffer: Vec<u8, 128> = Vec::new(); // Note: buffer needs 16-bytes overhead for auth tag
buffer.extend_from_slice(b"plaintext message");

// Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
cipher.encrypt_in_place(&nonce, b"", &mut buffer)?;

// `buffer` now contains the message ciphertext
assert_ne!(&buffer, b"plaintext message");

// Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
cipher.decrypt_in_place(&nonce, b"", &mut buffer)?;
assert_eq!(&buffer, b"plaintext message");
```

## <a href="#xchacha20poly1305" class="doc-anchor">§</a>[`XChaCha20Poly1305`](type.XChaCha20Poly1305.html "type chacha20poly1305::XChaCha20Poly1305")

ChaCha20Poly1305 variant with an extended 192-bit (24-byte) nonce.

The construction is an adaptation of the same techniques used by
XSalsa20 as described in the paper “Extending the Salsa20 Nonce” to the
96-bit nonce variant of ChaCha20, which derive a separate subkey/nonce
for each extended nonce:

<https://cr.yp.to/snuffle/xsalsa-20081128.pdf>

No authoritative specification exists for XChaCha20Poly1305, however the
construction has “rough consensus and running code” in the form of
several interoperable libraries and protocols (e.g. libsodium,
WireGuard) and is documented in an (expired) IETF draft, which also
applies the proof from the XSalsa20 paper to the construction in order
to demonstrate that XChaCha20 is secure if ChaCha20 is secure (see
Section 3.1):

<https://tools.ietf.org/html/draft-arciszewski-xchacha-03>

It is worth noting that NaCl/libsodium’s default “secretbox” algorithm
is XSalsa20Poly1305, not XChaCha20Poly1305, and thus not compatible with
this library. If you are interested in that construction, please see the
`xsalsa20poly1305` crate:

<https://docs.rs/xsalsa20poly1305/>

# <a href="#usage-1" class="doc-anchor">§</a>Usage

``` {.rust .rust-example-rendered}
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    XChaCha20Poly1305, XNonce
};

let key = XChaCha20Poly1305::generate_key(&mut OsRng);
let cipher = XChaCha20Poly1305::new(&key);
let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng); // 192-bits; unique per message
let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
assert_eq!(&plaintext, b"plaintext message");
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../aead/index.html" class="mod"
title="mod aead"><code>aead</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="consts/index.html" class="mod"
title="mod chacha20poly1305::consts">consts</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ChaChaPoly1305.html" class="struct"
title="struct chacha20poly1305::ChaChaPoly1305">ChaChaPoly1305</a>  
Generic ChaCha+Poly1305 Authenticated Encryption with Additional Data
(AEAD) construction.

<a href="struct.Error.html" class="struct"
title="struct chacha20poly1305::Error">Error</a>  
Error type.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AeadCore.html" class="trait"
title="trait chacha20poly1305::AeadCore">AeadCore</a>  
Authenticated Encryption with Associated Data (AEAD) algorithm core
trait.

<a href="trait.AeadInPlace.html" class="trait"
title="trait chacha20poly1305::AeadInPlace">AeadInPlace</a>  
In-place stateless AEAD trait.

<a href="trait.KeyInit.html" class="trait"
title="trait chacha20poly1305::KeyInit">KeyInit</a>  
Types which can be initialized from key.

<a href="trait.KeySizeUser.html" class="trait"
title="trait chacha20poly1305::KeySizeUser">KeySizeUser</a>  
Types which use key for initialization.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ChaCha20Poly1305.html" class="type"
title="type chacha20poly1305::ChaCha20Poly1305">ChaCha20Poly1305</a>  
ChaCha20Poly1305 Authenticated Encryption with Additional Data (AEAD).

<a href="type.Key.html" class="type"
title="type chacha20poly1305::Key">Key</a>  
Key type (256-bits/32-bytes).

<a href="type.Nonce.html" class="type"
title="type chacha20poly1305::Nonce">Nonce</a>  
Nonce type (96-bits/12-bytes).

<a href="type.Tag.html" class="type"
title="type chacha20poly1305::Tag">Tag</a>  
Poly1305 tag.

<a href="type.XChaCha20Poly1305.html" class="type"
title="type chacha20poly1305::XChaCha20Poly1305">XChaCha20Poly1305</a>  
XChaCha20Poly1305 Authenticated Encryption with Additional Data (AEAD).

<a href="type.XNonce.html" class="type"
title="type chacha20poly1305::XNonce">XNonce</a>  
XNonce type (192-bits/24-bytes).
