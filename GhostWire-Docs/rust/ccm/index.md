Crate ccm Copy item path

<span class="sub-heading"><a href="../src/ccm/lib.rs.html#1-379" class="src">Source</a>
</span>

Expand description

Counter with CBC-MAC ([CCM](https://en.wikipedia.org/wiki/CCM_mode)):
[Authenticated Encryption and Associated Data
(AEAD)](https://en.wikipedia.org/wiki/Authenticated_encryption)
algorithm generic over block ciphers with block size equal to 128 bits
as specified in [RFC 3610](https://tools.ietf.org/html/rfc3610).

# <a href="#usage" class="doc-anchor">§</a>Usage

Simple usage (allocating, no associated data):

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use aes::Aes256;
use ccm::{
    aead::{Aead, KeyInit, OsRng, generic_array::GenericArray},
    consts::{U10, U13},
    Ccm,
};

// AES-256-CCM type with tag and nonce size equal to 10 and 13 bytes respectively
pub type Aes256Ccm = Ccm<Aes256, U10, U13>;

let key = Aes256Ccm::generate_key(&mut OsRng);
let cipher = Aes256Ccm::new(&key);
let nonce = GenericArray::from_slice(b"unique nonce."); // 13-bytes; unique per message
let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())?;
let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;
assert_eq!(&plaintext, b"plaintext message");
```

This crate implements traits from the
[`aead`](../aead/index.html "mod aead") crate and is capable to perfrom
encryption and decryption in-place wihout relying on `alloc`.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../aead/index.html" class="mod"
title="mod aead"><code>aead</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="consts/index.html" class="mod"
title="mod ccm::consts">consts</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Ccm.html" class="struct" title="struct ccm::Ccm">Ccm</a>  
CCM instance generic over an underlying block cipher.

<a href="struct.Error.html" class="struct"
title="struct ccm::Error">Error</a>  
Error type.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AeadCore.html" class="trait"
title="trait ccm::AeadCore">AeadCore</a>  
Authenticated Encryption with Associated Data (AEAD) algorithm core
trait.

<a href="trait.AeadInPlace.html" class="trait"
title="trait ccm::AeadInPlace">AeadInPlace</a>  
In-place stateless AEAD trait.

<a href="trait.KeyInit.html" class="trait"
title="trait ccm::KeyInit">KeyInit</a>  
Types which can be initialized from key.

<a href="trait.KeySizeUser.html" class="trait"
title="trait ccm::KeySizeUser">KeySizeUser</a>  
Types which use key for initialization.

<a href="trait.NonceSize.html" class="trait"
title="trait ccm::NonceSize">NonceSize</a>  
Trait implemented for valid nonce sizes, i.e.
[`U7`](consts/type.U7.html "type ccm::consts::U7"),
[`U8`](consts/type.U8.html "type ccm::consts::U8"),
[`U9`](consts/type.U9.html "type ccm::consts::U9"),
[`U10`](consts/type.U10.html "type ccm::consts::U10"),
[`U11`](consts/type.U11.html "type ccm::consts::U11"),
[`U12`](consts/type.U12.html "type ccm::consts::U12"), and
[`U13`](consts/type.U13.html "type ccm::consts::U13").

<a href="trait.TagSize.html" class="trait"
title="trait ccm::TagSize">TagSize</a>  
Trait implemented for valid tag sizes, i.e.
[`U4`](consts/type.U4.html "type ccm::consts::U4"),
[`U6`](consts/type.U6.html "type ccm::consts::U6"),
[`U8`](consts/type.U8.html "type ccm::consts::U8"),
[`U10`](consts/type.U10.html "type ccm::consts::U10"),
[`U12`](consts/type.U12.html "type ccm::consts::U12"),
[`U14`](consts/type.U14.html "type ccm::consts::U14"), and
[`U16`](consts/type.U16.html "type ccm::consts::U16").

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Key.html" class="type" title="type ccm::Key">Key</a>  
Key used by
[`KeySizeUser`](trait.KeySizeUser.html "trait ccm::KeySizeUser")
implementors.

<a href="type.Nonce.html" class="type" title="type ccm::Nonce">Nonce</a>  
CCM nonces

<a href="type.Tag.html" class="type" title="type ccm::Tag">Tag</a>  
CCM tags
