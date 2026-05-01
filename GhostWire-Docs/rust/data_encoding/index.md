Crate data\_encoding Copy item path

<span class="sub-heading"><a href="../src/data_encoding/lib.rs.html#1-2697" class="src">Source</a>
</span>

Expand description

Efficient and customizable data-encoding functions like base64, base32,
and hex

This [crate](https://crates.io/crates/data-encoding) provides
little-endian ASCII base-conversion encodings for bases of size 2, 4, 8,
16, 32, and 64. It supports:

-   [padding](struct.Specification.html#structfield.padding) for
    streaming
-   canonical encodings (e.g. [trailing
    bits](struct.Specification.html#structfield.check_trailing_bits) are
    checked)
-   in-place [encoding](struct.Encoding.html#method.encode_mut) and
    [decoding](struct.Encoding.html#method.decode_mut) functions
-   partial [decoding](struct.Encoding.html#method.decode_mut) functions
    (e.g. for error recovery)
-   character
    [translation](struct.Specification.html#structfield.translate) (e.g.
    for case-insensitivity)
-   most and least significant
    [bit-order](struct.Specification.html#structfield.bit_order)
-   [ignoring](struct.Specification.html#structfield.ignore) characters
    when decoding (e.g. for skipping newlines)
-   [wrapping](struct.Specification.html#structfield.wrap) the output
    when encoding
-   no-std environments with
    `default-features = false, features = ["alloc"]`
-   no-alloc environments with `default-features = false`

You may use the [binary](https://crates.io/crates/data-encoding-bin) or
the [website](https://data-encoding.rs) to play around.

# <a href="#examples" class="doc-anchor">§</a>Examples

This crate provides predefined encodings as
[constants](index.html#constants). These constants are of type
[`Encoding`](struct.Encoding.html). This type provides encoding and
decoding functions with in-place or allocating variants. Here is an
example using the allocating encoding function of
[`BASE64`](constant.BASE64.html):

``` {.rust .rust-example-rendered}
use data_encoding::BASE64;
assert_eq!(BASE64.encode(b"Hello world"), "SGVsbG8gd29ybGQ=");
```

Here is an example using the in-place decoding function of
[`BASE32`](constant.BASE32.html):

``` {.rust .rust-example-rendered}
use data_encoding::BASE32;
let input = b"JBSWY3DPEB3W64TMMQ======";
let mut output = vec![0; BASE32.decode_len(input.len()).unwrap()];
let len = BASE32.decode_mut(input, &mut output).unwrap();
assert_eq!(&output[0 .. len], b"Hello world");
```

You are not limited to the predefined encodings. You may define your own
encodings (with the same correctness and performance properties as the
predefined ones) using the [`Specification`](struct.Specification.html)
type:

``` {.rust .rust-example-rendered}
use data_encoding::Specification;
let hex = {
    let mut spec = Specification::new();
    spec.symbols.push_str("0123456789abcdef");
    spec.encoding().unwrap()
};
assert_eq!(hex.encode(b"hello"), "68656c6c6f");
```

You may use the [macro](https://crates.io/crates/data-encoding-macro)
library to define a compile-time custom encoding:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use data_encoding::Encoding;
use data_encoding_macro::new_encoding;
const HEX: Encoding = new_encoding!{
    symbols: "0123456789abcdef",
    translate_from: "ABCDEF",
    translate_to: "abcdef",
};
const BASE64: Encoding = new_encoding!{
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    padding: '=',
};
```

# <a href="#properties" class="doc-anchor">§</a>Properties

The [`HEXUPPER`](constant.HEXUPPER.html),
[`BASE32`](constant.BASE32.html),
[`BASE32HEX`](constant.BASE32HEX.html),
[`BASE64`](constant.BASE64.html), and
[`BASE64URL`](constant.BASE64URL.html) predefined encodings conform to
[RFC4648](https://tools.ietf.org/html/rfc4648).

In general, the encoding and decoding functions satisfy the following
properties:

-   They are deterministic: their output only depends on their input
-   They have no side-effects: they do not modify any hidden mutable
    state
-   They are correct: encoding followed by decoding gives the initial
    data
-   They are canonical (unless
    [`is_canonical`](struct.Encoding.html#method.is_canonical) returns
    false): decoding followed by encoding gives the initial data

This last property is usually not satisfied by base64 implementations.
This is a matter of choice and this crate has made the choice to let the
user choose. Support for canonical encoding as described by the
[RFC](https://tools.ietf.org/html/rfc4648#section-3.5) is provided. But
it is also possible to disable checking trailing bits, to add characters
translation, to decode concatenated padded inputs, and to ignore some
characters. Note that non-canonical encodings may be an attack vector as
described in [Base64 Malleability in
Practice](https://eprint.iacr.org/2022/361.pdf).

Since the RFC specifies the encoding function on all inputs and the
decoding function on all possible encoded outputs, the differences
between implementations come from the decoding function which may be
more or less permissive. In this crate, the decoding function of
canonical encodings rejects all inputs that are not a possible output of
the encoding function. Here are some concrete examples of decoding
differences between this crate, the `base64` crate, and the `base64` GNU
program:

| Input      | `data-encoding` | `base64`  | GNU `base64`  |
|------------|-----------------|-----------|---------------|
| `AAB=`     | `Trailing(2)`   | `Last(2)` | `\x00\x00`    |
| `AA\nB=`   | `Length(4)`     | `Byte(2)` | `\x00\x00`    |
| `AAB`      | `Length(0)`     | `Padding` | Invalid input |
| `AAA`      | `Length(0)`     | `Padding` | Invalid input |
| `A\rA\nB=` | `Length(4)`     | `Byte(1)` | Invalid input |
| `-_\r\n`   | `Symbol(0)`     | `Byte(0)` | Invalid input |
| `AA==AA==` | `[0, 0]`        | `Byte(2)` | `\x00\x00`    |

We can summarize these discrepancies as follows:

| Discrepancy                | `data-encoding` | `base64` | GNU `base64` |
|----------------------------|-----------------|----------|--------------|
| Check trailing bits        | Yes             | Yes      | No           |
| Ignored characters         | None            | None     | `\n`         |
| Translated characters      | None            | None     | None         |
| Check padding              | Yes             | No       | Yes          |
| Support concatenated input | Yes             | No       | Yes          |

This crate permits to disable checking trailing bits. It permits to
ignore some characters. It permits to translate characters. It permits
to use unpadded encodings. However, for padded encodings, support for
concatenated inputs cannot be disabled. This is simply because it
doesn’t make sense to use padding if it is not to support concatenated
inputs.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DecodeError.html" class="struct"
title="struct data_encoding::DecodeError">DecodeError</a>  
Decoding error

<a href="struct.DecodePartial.html" class="struct"
title="struct data_encoding::DecodePartial">DecodePartial</a>  
Decoding error with partial result

<a href="struct.Display.html" class="struct"
title="struct data_encoding::Display">Display</a>  
Wraps an encoding and input for display purposes.

<a href="struct.Encoder.html" class="struct"
title="struct data_encoding::Encoder">Encoder</a>  
Encodes fragmented input to an output

<a href="struct.Encoding.html" class="struct"
title="struct data_encoding::Encoding">Encoding</a>  
Base-conversion encoding

<a href="struct.Specification.html" class="struct"
title="struct data_encoding::Specification">Specification</a>  
Base-conversion specification

<a href="struct.SpecificationError.html" class="struct"
title="struct data_encoding::SpecificationError">SpecificationError</a>  
Specification error

<a href="struct.Translate.html" class="struct"
title="struct data_encoding::Translate">Translate</a>  
How to translate characters when decoding

<a href="struct.Wrap.html" class="struct"
title="struct data_encoding::Wrap">Wrap</a>  
How to wrap the output when encoding

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.BitOrder.html" class="enum"
title="enum data_encoding::BitOrder">BitOrder</a>  
Order in which bits are read from a byte

<a href="enum.DecodeKind.html" class="enum"
title="enum data_encoding::DecodeKind">DecodeKind</a>  
Decoding error kind

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.BASE32.html" class="constant"
title="constant data_encoding::BASE32">BASE32</a>  
Padded base32 encoding

<a href="constant.BASE64.html" class="constant"
title="constant data_encoding::BASE64">BASE64</a>  
Padded base64 encoding

<a href="constant.BASE32HEX.html" class="constant"
title="constant data_encoding::BASE32HEX">BASE32HEX</a>  
Padded base32hex encoding

<a href="constant.BASE32HEX_NOPAD.html" class="constant"
title="constant data_encoding::BASE32HEX_NOPAD">BASE32HEX_NOPAD</a>  
Unpadded base32hex encoding

<a href="constant.BASE32_DNSCURVE.html" class="constant"
title="constant data_encoding::BASE32_DNSCURVE">BASE32_DNSCURVE</a>  
DNSCurve base32 encoding

<a href="constant.BASE32_DNSSEC.html" class="constant"
title="constant data_encoding::BASE32_DNSSEC">BASE32_DNSSEC</a>  
DNSSEC base32 encoding

<a href="constant.BASE32_NOPAD.html" class="constant"
title="constant data_encoding::BASE32_NOPAD">BASE32_NOPAD</a>  
Unpadded base32 encoding

<a href="constant.BASE32_NOPAD_NOCASE.html" class="constant"
title="constant data_encoding::BASE32_NOPAD_NOCASE">BASE32_NOPAD_NOCASE</a>  
Unpadded base32 encoding with case-insensitive decoding

<a href="constant.BASE32_NOPAD_VISUAL.html" class="constant"
title="constant data_encoding::BASE32_NOPAD_VISUAL">BASE32_NOPAD_VISUAL</a>  
Unpadded base32 encoding with visual error correction during decoding

<a href="constant.BASE64URL.html" class="constant"
title="constant data_encoding::BASE64URL">BASE64URL</a>  
Padded base64url encoding

<a href="constant.BASE64URL_NOPAD.html" class="constant"
title="constant data_encoding::BASE64URL_NOPAD">BASE64URL_NOPAD</a>  
Unpadded base64url encoding

<a href="constant.BASE64_MIME.html" class="constant"
title="constant data_encoding::BASE64_MIME">BASE64_MIME</a>  
MIME base64 encoding

<a href="constant.BASE64_MIME_PERMISSIVE.html" class="constant"
title="constant data_encoding::BASE64_MIME_PERMISSIVE">BASE64_MIME_PERMISSIVE</a>  
MIME base64 encoding without trailing bits check

<a href="constant.BASE64_NOPAD.html" class="constant"
title="constant data_encoding::BASE64_NOPAD">BASE64_NOPAD</a>  
Unpadded base64 encoding

<a href="constant.HEXLOWER.html" class="constant"
title="constant data_encoding::HEXLOWER">HEXLOWER</a>  
Lowercase hexadecimal encoding

<a href="constant.HEXLOWER_PERMISSIVE.html" class="constant"
title="constant data_encoding::HEXLOWER_PERMISSIVE">HEXLOWER_PERMISSIVE</a>  
Lowercase hexadecimal encoding with case-insensitive decoding

<a href="constant.HEXUPPER.html" class="constant"
title="constant data_encoding::HEXUPPER">HEXUPPER</a>  
Uppercase hexadecimal encoding

<a href="constant.HEXUPPER_PERMISSIVE.html" class="constant"
title="constant data_encoding::HEXUPPER_PERMISSIVE">HEXUPPER_PERMISSIVE</a>  
Uppercase hexadecimal encoding with case-insensitive decoding
