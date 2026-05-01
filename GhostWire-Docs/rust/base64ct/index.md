Crate base64ct Copy item path

<span class="sub-heading"><a href="../src/base64ct/lib.rs.html#1-105" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-constant-time-base64" class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto): Constant-Time Base64

[![crate](https://img.shields.io/crates/v/base64ct?logo=rust)](https://crates.io/crates/base64ct)
[![Docs](https://docs.rs/base64ct/badge.svg)](https://docs.rs/base64ct/)
[![Build
Status](https://github.com/RustCrypto/formats/actions/workflows/base64ct.yml/badge.svg)](https://github.com/RustCrypto/formats/actions/workflows/base64ct.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.85+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats)

Pure Rust implementation of Base64 ([RFC
4648](https://tools.ietf.org/html/rfc4648)).

Implements multiple Base64 alphabets without data-dependent branches or
lookup tables, thereby providing portable “best effort” constant-time
operation.

Supports `no_std` environments and avoids heap allocations in the core
API (but also provides optional `alloc` support for convenience).

[Documentation](https://docs.rs/base64ct/)

## <a href="#about" class="doc-anchor">§</a>About

This crate implements several Base64 alphabets in constant-time for
sidechannel resistance, aimed at purposes like encoding/decoding the
“PEM” format used to store things like cryptographic private keys (i.e.
in the
[`pem-rfc7468`](https://github.com/RustCrypto/formats/tree/master/pem-rfc7468)
crate).

The paper [Util::Lookup: Exploiting key decoding in cryptographic
libraries](https://arxiv.org/pdf/2108.04600.pdf) demonstrates how the
leakage from non-constant-time Base64 parsers can be used to practically
extract RSA private keys from SGX enclaves.

The padded variants require (`=`) padding. Unpadded variants expressly
reject such padding.

Whitespace is expressly disallowed, with the exception of the
[`Decoder::new_wrapped`](https://docs.rs/base64ct/latest/base64ct/struct.Decoder.html#method.new_wrapped)
and
[`Encoder::new_wrapped`](https://docs.rs/base64ct/latest/base64ct/struct.Encoder.html#method.new_wrapped)
modes which provide fixed-width line wrapping.

## <a href="#supported-base64-variants" class="doc-anchor">§</a>Supported Base64 variants

-   Standard Base64: `[A-Z]`, `[a-z]`, `[0-9]`, `+`, `/`
-   URL-safe Base64: `[A-Z]`, `[a-z]`, `[0-9]`, `-`, `_`
-   bcrypt Base64: `.`, `/`, `[A-Z]`, `[a-z]`, `[0-9]`
-   `crypt(3)` Base64: `.`, `-`, `[0-9]`, `[A-Z]`, `[a-z]`

## <a href="#minimum-supported-rust-version-msrv-policy"
class="doc-anchor">§</a>Minimum Supported Rust Version (MSRV) Policy

MSRV increases are not considered breaking changes and can happen in
patch releases.

The crate MSRV accounts for all supported targets and crate feature
combinations, excluding explicitly unstable features.

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

## <a href="#allocating-enable-alloc-crate-feature"
class="doc-anchor">§</a>Allocating (enable `alloc` crate feature)

``` {.rust .rust-example-rendered}
use base64ct::{Base64, Encoding};

let bytes = b"example bytestring!";
let encoded = Base64::encode_string(bytes);
assert_eq!(encoded, "ZXhhbXBsZSBieXRlc3RyaW5nIQ==");

let decoded = Base64::decode_vec(&encoded).unwrap();
assert_eq!(decoded, bytes);
```

## <a href="#heapless-no_std-usage" class="doc-anchor">§</a>Heapless `no_std` usage

``` {.rust .rust-example-rendered}
use base64ct::{Base64, Encoding};

const BUF_SIZE: usize = 128;

let bytes = b"example bytestring!";
assert!(Base64::encoded_len(bytes) <= BUF_SIZE);

let mut enc_buf = [0u8; BUF_SIZE];
let encoded = Base64::encode(bytes, &mut enc_buf).unwrap();
assert_eq!(encoded, "ZXhhbXBsZSBieXRlc3RyaW5nIQ==");

let mut dec_buf = [0u8; BUF_SIZE];
let decoded = Base64::decode(encoded, &mut dec_buf).unwrap();
assert_eq!(decoded, bytes);
```

# <a href="#implementation" class="doc-anchor">§</a>Implementation

Implemented using integer arithmetic alone without any lookup tables or
data-dependent branches, thereby providing portable “best effort”
constant-time operation.

Not constant-time with respect to message length (only data).

Adapted from the following constant-time C++ implementation of Base64:

<https://github.com/Sc00bz/ConstTimeEncoding/blob/master/base64.cpp>

Copyright (c) 2014 Steve “Sc00bz” Thomas (steve at tobtu dot com).
Derived code is dual licensed MIT + Apache 2 (with permission from
Sc00bz).

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Base64.html" class="struct"
title="struct base64ct::Base64">Base64</a>  
Standard Base64 encoding with `=` padding.

<a href="struct.Base64Bcrypt.html" class="struct"
title="struct base64ct::Base64Bcrypt">Base64Bcrypt</a>  
bcrypt Base64 encoding.

<a href="struct.Base64Crypt.html" class="struct"
title="struct base64ct::Base64Crypt">Base64Crypt</a>  
`crypt(3)` Base64 encoding.

<a href="struct.Base64ShaCrypt.html" class="struct"
title="struct base64ct::Base64ShaCrypt">Base64ShaCrypt</a>  
Little endian variant of the `crypt(3)` Base64 encoding.

<a href="struct.Base64Unpadded.html" class="struct"
title="struct base64ct::Base64Unpadded">Base64Unpadded</a>  
Standard Base64 encoding *without* padding.

<a href="struct.Base64Url.html" class="struct"
title="struct base64ct::Base64Url">Base64Url</a>  
URL-safe Base64 encoding with `=` padding.

<a href="struct.Base64UrlUnpadded.html" class="struct"
title="struct base64ct::Base64UrlUnpadded">Base64UrlUnpadded</a>  
URL-safe Base64 encoding *without* padding.

<a href="struct.Decoder.html" class="struct"
title="struct base64ct::Decoder">Decoder</a>  
Stateful Base64 decoder with support for buffered, incremental decoding.

<a href="struct.Encoder.html" class="struct"
title="struct base64ct::Encoder">Encoder</a>  
Stateful Base64 encoder with support for buffered, incremental encoding.

<a href="struct.InvalidEncodingError.html" class="struct"
title="struct base64ct::InvalidEncodingError">InvalidEncodingError</a>  
Invalid encoding of provided Base64 string.

<a href="struct.InvalidLengthError.html" class="struct"
title="struct base64ct::InvalidLengthError">InvalidLengthError</a>  
Insufficient output buffer length.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum base64ct::Error">Error</a>  
Generic error, union of
[`InvalidLengthError`](struct.InvalidLengthError.html "struct base64ct::InvalidLengthError")
and
[`InvalidEncodingError`](struct.InvalidEncodingError.html "struct base64ct::InvalidEncodingError").

<a href="enum.LineEnding.html" class="enum"
title="enum base64ct::LineEnding">LineEnding</a>  
Line endings: variants of newline characters that can be used with
Base64.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Encoding.html" class="trait"
title="trait base64ct::Encoding">Encoding</a>  
Base64 encoding trait.
