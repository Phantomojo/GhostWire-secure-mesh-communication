Crate cbc Copy item path

<span class="sub-heading"><a href="../src/cbc/lib.rs.html#1-113" class="src">Source</a>
</span>

Expand description

[Cipher Block
Chaining](https://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#CBC)
(CBC) mode.

<img
src="https://raw.githubusercontent.com/RustCrypto/media/26acc39f/img/block-modes/cbc_enc.svg"
style="width:49.0%" /> <img
src="https://raw.githubusercontent.com/RustCrypto/media/26acc39f/img/block-modes/cbc_dec.svg"
style="width:49.0%" />

Mode functionality is accessed using traits from re-exported
\[`cipher`\] crate.

# <a href="#-security-warning-hazmat" class="doc-anchor">§</a>⚠️ Security Warning: Hazmat!

This crate does not ensure ciphertexts are authentic! Thus ciphertext
integrity is not verified, which can lead to serious vulnerabilities!

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use hex_literal::hex;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

let key = [0x42; 16];
let iv = [0x24; 16];
let plaintext = *b"hello world! this is my plaintext.";
let ciphertext = hex!(
    "c7fe247ef97b21f07cbdd26cb5d346bf"
    "d27867cb00d9486723e159978fb9a5f9"
    "14cfb228a710de4171e396e7b6cf859e"
);

// encrypt/decrypt in-place
// buffer must be big enough for padded plaintext
let mut buf = [0u8; 48];
let pt_len = plaintext.len();
buf[..pt_len].copy_from_slice(&plaintext);
let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
    .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
    .unwrap();
assert_eq!(ct, &ciphertext[..]);

let pt = Aes128CbcDec::new(&key.into(), &iv.into())
    .decrypt_padded_mut::<Pkcs7>(&mut buf)
    .unwrap();
assert_eq!(pt, &plaintext);

// encrypt/decrypt from buffer to buffer
let mut buf = [0u8; 48];
let ct = Aes128CbcEnc::new(&key.into(), &iv.into())
    .encrypt_padded_b2b_mut::<Pkcs7>(&plaintext, &mut buf)
    .unwrap();
assert_eq!(ct, &ciphertext[..]);

let mut buf = [0u8; 48];
let pt = Aes128CbcDec::new(&key.into(), &iv.into())
    .decrypt_padded_b2b_mut::<Pkcs7>(&ct, &mut buf)
    .unwrap();
assert_eq!(pt, &plaintext);
```

With enabled `alloc` (or `std`) feature you also can use allocating
convinience methods:

``` {.rust .rust-example-rendered}
let res = Aes128CbcEnc::new(&key.into(), &iv.into())
    .encrypt_padded_vec_mut::<Pkcs7>(&plaintext);
assert_eq!(res[..], ciphertext[..]);
let res = Aes128CbcDec::new(&key.into(), &iv.into())
    .decrypt_padded_vec_mut::<Pkcs7>(&res)
    .unwrap();
assert_eq!(res[..], plaintext[..]);
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use cipher;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Decryptor.html" class="struct"
title="struct cbc::Decryptor">Decryptor</a>  
CBC mode decryptor.

<a href="struct.Encryptor.html" class="struct"
title="struct cbc::Encryptor">Encryptor</a>  
CBC mode encryptor.
