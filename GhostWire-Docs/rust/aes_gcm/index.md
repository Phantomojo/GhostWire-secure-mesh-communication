Crate aes\_gcm Copy item path

<span class="sub-heading"><a href="../src/aes_gcm/lib.rs.html#1-373" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-aes-gcm" class="doc-anchor">§</a>RustCrypto: AES-GCM

[![crate](https://buildstats.info/crate/aes-gcm)](https://crates.io/crates/aes-gcm)
[![Docs](https://docs.rs/aes-gcm/badge.svg)](https://docs.rs/aes-gcm/)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.56+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260038-AEADs)
[![Build
Status](https://github.com/RustCrypto/AEADs/workflows/aes-gcm/badge.svg?branch=master&event=push)](https://github.com/RustCrypto/AEADs/actions)

Pure Rust implementation of the AES-GCM [Authenticated Encryption with
Associated Data
(AEAD)](https://en.wikipedia.org/wiki/Authenticated_encryption) cipher.

[Documentation](https://docs.rs/aes-gcm/)

## <a href="#security-notes" class="doc-anchor">§</a>Security Notes

This crate has received one [security audit by NCC
Group](https://research.nccgroup.com/2020/02/26/public-report-rustcrypto-aes-gcm-and-chacha20poly1305-implementation-review/),
with no significant findings. We would like to thank
[MobileCoin](https://www.mobilecoin.com/) for funding the audit.

All implementations contained in the crate are designed to execute in
constant time, either by relying on hardware intrinsics (i.e. AES-NI and
CLMUL on x86/x86\_64), or using a portable implementation which is only
constant time on processors which implement constant-time
multiplication.

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

# <a href="#usage" class="doc-anchor">§</a>Usage

Simple usage (allocating, no associated data):

``` {.rust .rust-example-rendered}
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};

// The encryption key can be generated randomly:
let key = Aes256Gcm::generate_key(OsRng);

// Transformed from a byte array:
let key: &[u8; 32] = &[42; 32];
let key: &Key<Aes256Gcm> = key.into();

// Note that you can get byte array from slice using the `TryInto` trait:
let key: &[u8] = &[42; 32];
let key: [u8; 32] = key.try_into()?;

// Alternatively, the key can be transformed directly from a byte slice
// (panicks on length mismatch):
let key = Key::<Aes256Gcm>::from_slice(key);

let cipher = Aes256Gcm::new(&key);
let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
assert_eq!(&plaintext, b"plaintext message");
```

## <a href="#in-place-usage-eliminates-alloc-requirement"
class="doc-anchor">§</a>In-place Usage (eliminates `alloc` requirement)

This crate has an optional `alloc` feature which can be disabled in e.g.
microcontroller environments that don’t have a heap.

The
[`AeadInPlace::encrypt_in_place`](trait.AeadInPlace.html#method.encrypt_in_place "method aes_gcm::AeadInPlace::encrypt_in_place")
and
[`AeadInPlace::decrypt_in_place`](trait.AeadInPlace.html#method.decrypt_in_place "method aes_gcm::AeadInPlace::decrypt_in_place")
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
use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng, heapless::Vec},
    Aes256Gcm, Nonce, // Or `Aes128Gcm`
};

let key = Aes256Gcm::generate_key(&mut OsRng);
let cipher = Aes256Gcm::new(&key);
let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message

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

Similarly, enabling the `arrayvec` feature of this crate will provide an
impl of [`aead::Buffer`](../aead/trait.Buffer.html "trait aead::Buffer")
for `arrayvec::ArrayVec` (re-exported from the
[`aead`](../aead/index.html "mod aead") crate as
\[`aead::arrayvec::ArrayVec`\]).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../aead/index.html" class="mod"
title="mod aead"><code>aead</code></a>`;`

`pub use `<a href="../aes/index.html" class="mod"
title="mod aes"><code>aes</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AesGcm.html" class="struct"
title="struct aes_gcm::AesGcm">AesGcm</a>  
AES-GCM: generic over an underlying AES implementation and nonce size.

<a href="struct.Error.html" class="struct"
title="struct aes_gcm::Error">Error</a>  
Error type.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.A_MAX.html" class="constant"
title="constant aes_gcm::A_MAX">A_MAX</a>  
Maximum length of associated data.

<a href="constant.C_MAX.html" class="constant"
title="constant aes_gcm::C_MAX">C_MAX</a>  
Maximum length of ciphertext.

<a href="constant.P_MAX.html" class="constant"
title="constant aes_gcm::P_MAX">P_MAX</a>  
Maximum length of plaintext.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AeadCore.html" class="trait"
title="trait aes_gcm::AeadCore">AeadCore</a>  
Authenticated Encryption with Associated Data (AEAD) algorithm core
trait.

<a href="trait.AeadInPlace.html" class="trait"
title="trait aes_gcm::AeadInPlace">AeadInPlace</a>  
In-place stateless AEAD trait.

<a href="trait.KeyInit.html" class="trait"
title="trait aes_gcm::KeyInit">KeyInit</a>  
Types which can be initialized from key.

<a href="trait.KeySizeUser.html" class="trait"
title="trait aes_gcm::KeySizeUser">KeySizeUser</a>  
Types which use key for initialization.

<a href="trait.TagSize.html" class="trait"
title="trait aes_gcm::TagSize">TagSize</a>  
Trait implemented for valid tag sizes, i.e. \[`U12`\]\[consts::U12\],
\[`U13`\]\[consts::U13\], \[`U14`\]\[consts::U14\],
\[`U15`\]\[consts::U15\] and \[`U16`\]\[consts::U16\].

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Aes128Gcm.html" class="type"
title="type aes_gcm::Aes128Gcm">Aes128Gcm</a>  
AES-GCM with a 128-bit key and 96-bit nonce.

<a href="type.Aes256Gcm.html" class="type"
title="type aes_gcm::Aes256Gcm">Aes256Gcm</a>  
AES-GCM with a 256-bit key and 96-bit nonce.

<a href="type.Key.html" class="type" title="type aes_gcm::Key">Key</a>  
Key used by
[`KeySizeUser`](trait.KeySizeUser.html "trait aes_gcm::KeySizeUser")
implementors.

<a href="type.Nonce.html" class="type"
title="type aes_gcm::Nonce">Nonce</a>  
AES-GCM nonces.

<a href="type.Tag.html" class="type" title="type aes_gcm::Tag">Tag</a>  
AES-GCM tags.
