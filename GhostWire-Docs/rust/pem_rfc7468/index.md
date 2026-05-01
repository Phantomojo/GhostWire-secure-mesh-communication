Crate pem\_rfc7468 Copy item path

<span class="sub-heading"><a href="../src/pem_rfc7468/lib.rs.html#1-122" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-pem-encoding-rfc-7468" class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto): PEM Encoding ([RFC 7468](https://datatracker.ietf.org/doc/html/rfc7468))

[![crate](https://buildstats.info/crate/pem-rfc7468)](https://crates.io/crates/pem-rfc7468)
[![Docs](https://docs.rs/pem-rfc7468/badge.svg)](https://docs.rs/pem-rfc7468/)
[![Build
Status](https://github.com/RustCrypto/formats/actions/workflows/pem-rfc7468.yml/badge.svg)](https://github.com/RustCrypto/formats/actions/workflows/pem-rfc7468.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats)

Pure Rust implementation of PEM Encoding ([RFC
7468](https://datatracker.ietf.org/doc/html/rfc7468)) for PKIX, PKCS,
and CMS Structures, a strict subset of the original Privacy-Enhanced
Mail encoding intended specifically for use with cryptographic keys,
certificates, and other messages.

Provides a `no_std`-friendly, constant-time implementation suitable for
use with cryptographic private keys.

[Documentation](https://docs.rs/pem-rfc7468/)

## <a href="#about" class="doc-anchor">§</a>About

Many cryptography-related document formats, such as certificates (PKIX),
private and public keys/keypairs (PKCS), and other cryptographic
messages (CMS) provide an ASCII encoding which can be traced back to
Privacy-Enhanced Mail (PEM) as defined [RFC
1421](https://datatracker.ietf.org/doc/html/rfc1421), which look like
the following:

``` language-text
-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIBftnHPp22SewYmmEoMcX8VwI4IHwaqd+9LFPj/15eqF
-----END PRIVATE KEY-----
```

However, all of these formats actually implement a text-based encoding
that is similar but *not* identical to the legacy PEM encoding as
described in [RFC 1421](https://datatracker.ietf.org/doc/html/rfc1421).

For this reason, [RFC
7468](https://datatracker.ietf.org/doc/html/rfc7468) was created to
describe a stricter form of “PEM encoding” for use in these applications
which codifies the previously de facto rules that most implementations
operate by, and makes recommendations to promote interoperability.

This crate provides a strict interpretation of the [RFC
7468](https://datatracker.ietf.org/doc/html/rfc7468) rules, implementing
MUSTs and SHOULDs while avoiding the MAYs, targeting the “ABNF (Strict)”
subset of the grammar as described in [RFC 7468 Section 3 Figure 3
(p6)](https://datatracker.ietf.org/doc/html/rfc7468#page-6).

## <a href="#implementation-notes" class="doc-anchor">§</a>Implementation notes

-   `no_std`-friendly core implementation which requires no heap
    allocations and avoids copies and temporary buffers.
-   Optional `alloc`-dependent convenience features and buffered
    decoder/encoder.
-   Uses the
    [`base64ct`](https://github.com/RustCrypto/formats/tree/master/base64ct)
    crate to decode/encode Base64 in constant-time.
-   PEM parser avoids branching on potentially secret data as much as
    possible.

The paper [Util::Lookup: Exploiting key decoding in cryptographic
libraries](https://arxiv.org/pdf/2108.04600.pdf) demonstrates how the
leakage from non-constant-time PEM parsers can be used to practically
extract RSA private keys from SGX enclaves.

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

This crate requires **Rust 1.60** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a
minor version bump.

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

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
/// Example PEM document
/// NOTE: do not actually put private key literals into your source code!!!
let example_pem = "\
-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIBftnHPp22SewYmmEoMcX8VwI4IHwaqd+9LFPj/15eqF
-----END PRIVATE KEY-----
";

// Decode PEM
let (type_label, data) = pem_rfc7468::decode_vec(example_pem.as_bytes())?;
assert_eq!(type_label, "PRIVATE KEY");
assert_eq!(
   data,
   &[
       48, 46, 2, 1, 0, 48, 5, 6, 3, 43, 101, 112, 4, 34, 4, 32, 23, 237, 156, 115, 233, 219,
       100, 158, 193, 137, 166, 18, 131, 28, 95, 197, 112, 35, 130, 7, 193, 170, 157, 251,
       210, 197, 62, 63, 245, 229, 234, 133
   ]
);

// Encode PEM
use pem_rfc7468::LineEnding;
let encoded_pem = pem_rfc7468::encode_string(type_label, LineEnding::default(), &data)?;
assert_eq!(&encoded_pem, example_pem);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Decoder.html" class="struct"
title="struct pem_rfc7468::Decoder">Decoder</a>  
Buffered PEM decoder.

<a href="struct.Encoder.html" class="struct"
title="struct pem_rfc7468::Encoder">Encoder</a>  
Buffered PEM encoder.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum pem_rfc7468::Error">Error</a>  
PEM errors.

<a href="enum.LineEnding.html" class="enum"
title="enum pem_rfc7468::LineEnding">LineEnding</a>  
Line endings: variants of newline characters that can be used with
Base64.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.BASE64_WRAP_WIDTH.html" class="constant"
title="constant pem_rfc7468::BASE64_WRAP_WIDTH">BASE64_WRAP_WIDTH</a>  
Width at which the Base64 body of RFC7468-compliant PEM is wrapped.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.PemLabel.html" class="trait"
title="trait pem_rfc7468::PemLabel">PemLabel</a>  
Marker trait for types with an associated PEM type label.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decode.html" class="fn"
title="fn pem_rfc7468::decode">decode</a>  
Decode a PEM document according to RFC 7468’s “Strict” grammar.

<a href="fn.decode_label.html" class="fn"
title="fn pem_rfc7468::decode_label">decode_label</a>  
Decode the encapsulation boundaries of a PEM document according to RFC
7468’s “Strict” grammar.

<a href="fn.decode_vec.html" class="fn"
title="fn pem_rfc7468::decode_vec">decode_vec</a>  
Decode a PEM document according to RFC 7468’s “Strict” grammar,
returning the result as a
[`Vec`](https://doc.rust-lang.org/1.93.1/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")
upon success.

<a href="fn.encapsulated_len.html" class="fn"
title="fn pem_rfc7468::encapsulated_len">encapsulated_len</a>  
Compute the length of a PEM encoded document which encapsulates a
Base64-encoded body including line endings every 64 characters.

<a href="fn.encapsulated_len_wrapped.html" class="fn"
title="fn pem_rfc7468::encapsulated_len_wrapped">encapsulated_len_wrapped</a>  
Compute the length of a PEM encoded document with the Base64 body line
wrapped at the specified `width`.

<a href="fn.encode.html" class="fn"
title="fn pem_rfc7468::encode">encode</a>  
Encode a PEM document according to RFC 7468’s “Strict” grammar.

<a href="fn.encode_string.html" class="fn"
title="fn pem_rfc7468::encode_string">encode_string</a>  
Encode a PEM document according to RFC 7468’s “Strict” grammar,
returning the result as a
[`String`](https://doc.rust-lang.org/1.93.1/alloc/string/struct.String.html "struct alloc::string::String").

<a href="fn.encoded_len.html" class="fn"
title="fn pem_rfc7468::encoded_len">encoded_len</a>  
Get the length of a PEM encoded document with the given bytes and label.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Base64Decoder.html" class="type"
title="type pem_rfc7468::Base64Decoder">Base64Decoder</a>  
Buffered Base64 decoder type.

<a href="type.Base64Encoder.html" class="type"
title="type pem_rfc7468::Base64Encoder">Base64Encoder</a>  
Buffered Base64 encoder type.

<a href="type.Result.html" class="type"
title="type pem_rfc7468::Result">Result</a>  
Result type with the `pem-rfc7468` crate’s
[`Error`](enum.Error.html "enum pem_rfc7468::Error") type.
