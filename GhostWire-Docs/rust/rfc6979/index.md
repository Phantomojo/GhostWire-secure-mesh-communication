Crate rfc6979 Copy item path

<span class="sub-heading"><a href="../src/rfc6979/lib.rs.html#1-149" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-rfc6979-deterministic-signatures"
class="doc-anchor">§</a>[RustCrypto](https://github.com/RustCrypto): RFC6979 Deterministic Signatures

[![crate](https://buildstats.info/crate/rfc6979)](https://crates.io/crates/rfc6979)
[![Docs](https://docs.rs/rfc6979/badge.svg)](https://docs.rs/rfc6979/)
[![Build
Status](https://github.com/RustCrypto/signatures/actions/workflows/rfc6979.yml/badge.svg)](https://github.com/RustCrypto/signatures/actions/workflows/rfc6979.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![MSRV](https://img.shields.io/badge/rustc-1.61+-blue.svg) [![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/260048-signatures)

Pure Rust implementation of RFC6979: Deterministic Usage of the Digital
Signature Algorithm (DSA) and Elliptic Curve Digital Signature Algorithm
(ECDSA).

Algorithm described in RFC 6979 § 3.2:
<https://tools.ietf.org/html/rfc6979#section-3>

[Documentation](https://docs.rs/rfc6979/)

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

This crate requires **Rust 1.61** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a
minor version bump.

## <a href="#license" class="doc-anchor">§</a>License

All crates licensed under either of

-   [Apache License, Version
    2.0](http://www.apache.org/licenses/LICENSE-2.0)
-   [MIT license](http://opensource.org/licenses/MIT)

at your option.

### <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

## <a href="#usage" class="doc-anchor">§</a>Usage

See also: the documentation for the
[`generate_k`](fn.generate_k.html "fn rfc6979::generate_k") function.

``` {.rust .rust-example-rendered}
use hex_literal::hex;
use rfc6979::consts::U32;
use sha2::{Digest, Sha256};

// NIST P-256 field modulus
const NIST_P256_MODULUS: [u8; 32] =
    hex!("FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551");

// Public key for RFC6979 NIST P256/SHA256 test case
const RFC6979_KEY: [u8; 32] =
    hex!("C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721");

// Test message for RFC6979 NIST P256/SHA256 test case
const RFC6979_MSG: &[u8; 6] = b"sample";

// Expected K for RFC6979 NIST P256/SHA256 test case
const RFC6979_EXPECTED_K: [u8; 32] =
    hex!("A6E3C57DD01ABE90086538398355DD4C3B17AA873382B0F24D6129493D8AAD60");

let h = Sha256::digest(RFC6979_MSG);
let aad = b"";
let k = rfc6979::generate_k::<Sha256, U32>(&RFC6979_KEY.into(), &NIST_P256_MODULUS.into(), &h, aad);
assert_eq!(k.as_slice(), &RFC6979_EXPECTED_K);
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="consts/index.html" class="mod"
title="mod rfc6979::consts">consts</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.HmacDrbg.html" class="struct"
title="struct rfc6979::HmacDrbg">HmacDrbg</a>  
Internal implementation of `HMAC_DRBG` as described in NIST SP800-90A.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.generate_k.html" class="fn"
title="fn rfc6979::generate_k">generate_k</a>  
Deterministically generate ephemeral scalar `k`.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ByteArray.html" class="type"
title="type rfc6979::ByteArray">ByteArray</a>  
Array of bytes representing a scalar serialized as a big endian integer.
