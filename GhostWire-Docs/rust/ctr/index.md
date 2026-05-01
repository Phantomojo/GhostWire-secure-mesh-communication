Crate ctr Copy item path

<span class="sub-heading"><a href="../src/ctr/lib.rs.html#1-90" class="src">Source</a>
</span>

Expand description

Generic implementations of [CTR
mode](https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#CTR)
for block ciphers.

<img
src="https://raw.githubusercontent.com/RustCrypto/media/26acc39f/img/block-modes/ctr_enc.svg"
style="width:49.0%" /> <img
src="https://raw.githubusercontent.com/RustCrypto/media/26acc39f/img/block-modes/ctr_dec.svg"
style="width:49.0%" />

Mode functionality is accessed using traits from re-exported
\[`cipher`\] crate.

# <a href="#-security-warning-hazmat" class="doc-anchor">§</a>⚠️ Security Warning: Hazmat!

This crate does not ensure ciphertexts are authentic! Thus ciphertext
integrity is not verified, which can lead to serious vulnerabilities!

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use aes::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use hex_literal::hex;

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

let key = [0x42; 16];
let iv = [0x24; 16];
let plaintext = *b"hello world! this is my plaintext.";
let ciphertext = hex!(
    "3357121ebb5a29468bd861467596ce3da59bdee42dcc0614dea955368d8a5dc0cad4"
);

// encrypt in-place
let mut buf = plaintext.to_vec();
let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());
cipher.apply_keystream(&mut buf);
assert_eq!(buf[..], ciphertext[..]);

// CTR mode can be used with streaming messages
let mut cipher = Aes128Ctr64LE::new(&key.into(), &iv.into());
for chunk in buf.chunks_mut(3) {
    cipher.apply_keystream(chunk);
}
assert_eq!(buf[..], plaintext[..]);

// CTR mode supports seeking. The parameter is zero-based _bytes_ counter (not _blocks_).
cipher.seek(0u32);

// encrypt/decrypt from buffer to buffer
// buffer length must be equal to input length
let mut buf1 = [0u8; 34];
cipher
    .apply_keystream_b2b(&plaintext, &mut buf1)
    .unwrap();
assert_eq!(buf1[..], ciphertext[..]);

let mut buf2 = [0u8; 34];
cipher.seek(0u32);
cipher.apply_keystream_b2b(&buf1, &mut buf2).unwrap();
assert_eq!(buf2[..], plaintext[..]);
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use flavors::`<a href="flavors/trait.CtrFlavor.html" class="trait"
title="trait ctr::flavors::CtrFlavor"><code>CtrFlavor</code></a>`;`

`pub use cipher;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="flavors/index.html" class="mod"
title="mod ctr::flavors">flavors</a>  
CTR mode flavors

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CtrCore.html" class="struct"
title="struct ctr::CtrCore">CtrCore</a>  
Generic CTR block mode instance.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Ctr32BE.html" class="type"
title="type ctr::Ctr32BE">Ctr32BE</a>  
CTR mode with 32-bit big endian counter.

<a href="type.Ctr32LE.html" class="type"
title="type ctr::Ctr32LE">Ctr32LE</a>  
CTR mode with 32-bit little endian counter.

<a href="type.Ctr64BE.html" class="type"
title="type ctr::Ctr64BE">Ctr64BE</a>  
CTR mode with 64-bit big endian counter.

<a href="type.Ctr64LE.html" class="type"
title="type ctr::Ctr64LE">Ctr64LE</a>  
CTR mode with 64-bit little endian counter.

<a href="type.Ctr128BE.html" class="type"
title="type ctr::Ctr128BE">Ctr128BE</a>  
CTR mode with 128-bit big endian counter.

<a href="type.Ctr128LE.html" class="type"
title="type ctr::Ctr128LE">Ctr128LE</a>  
CTR mode with 128-bit little endian counter.
