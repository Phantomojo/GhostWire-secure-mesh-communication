Crate base64 Copy item path

<span class="sub-heading"><a href="../src/base64/lib.rs.html#1-277" class="src">Source</a>
</span>

Expand description

Correct, fast, and configurable
[base64](https://developer.mozilla.org/en-US/docs/Glossary/Base64)
decoding and encoding. Base64 transports binary data efficiently in
contexts where only plain text is allowed.

# <a href="#usage" class="doc-anchor">§</a>Usage

Use an
[`Engine`](engine/trait.Engine.html "trait base64::engine::Engine") to
decode or encode base64, configured with the base64 alphabet and padding
behavior best suited to your application.

## <a href="#engine-setup" class="doc-anchor">§</a>Engine setup

There is more than one way to encode a stream of bytes as “base64”.
Different applications use different encoding
[alphabets](alphabet/struct.Alphabet.html "struct base64::alphabet::Alphabet")
and [padding
behaviors](engine/general_purpose/struct.GeneralPurposeConfig.html "struct base64::engine::general_purpose::GeneralPurposeConfig").

### <a href="#encoding-alphabet" class="doc-anchor">§</a>Encoding alphabet

Almost all base64
[alphabets](alphabet/struct.Alphabet.html "struct base64::alphabet::Alphabet")
use `A-Z`, `a-z`, and `0-9`, which gives nearly 64 characters (26 + 26 +
10 = 62), but they differ in their choice of their final 2.

Most applications use the
[standard](alphabet/constant.STANDARD.html "constant base64::alphabet::STANDARD")
alphabet specified in [RFC
4648](https://datatracker.ietf.org/doc/html/rfc4648#section-4). If
that’s all you need, you can get started quickly by using the
pre-configured
[`STANDARD`](engine/general_purpose/constant.STANDARD.html "constant base64::engine::general_purpose::STANDARD")
engine, which is also available in the
[`prelude`](prelude/index.html "mod base64::prelude") module as shown
here, if you prefer a minimal `use` footprint.

``` {.rust .rust-example-rendered}
use base64::prelude::*;

assert_eq!(BASE64_STANDARD.decode(b"+uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
assert_eq!(BASE64_STANDARD.encode(b"\xFF\xEC\x20\x55\0"), "/+wgVQA=");
```

Other common alphabets are available in the
[`alphabet`](alphabet/index.html "mod base64::alphabet") module.

#### <a href="#url-safe-alphabet" class="doc-anchor">§</a>URL-safe alphabet

The standard alphabet uses `+` and `/` as its two non-alphanumeric
tokens, which cannot be safely used in URL’s without encoding them as
`%2B` and `%2F`.

To avoid that, some applications use a [“URL-safe”
alphabet](alphabet/constant.URL_SAFE.html "constant base64::alphabet::URL_SAFE"),
which uses `-` and `_` instead. To use that alternative alphabet, use
the
[`URL_SAFE`](engine/general_purpose/constant.URL_SAFE.html "constant base64::engine::general_purpose::URL_SAFE")
engine. This example doesn’t use
[`prelude`](prelude/index.html "mod base64::prelude") to show what a
more explicit `use` would look like.

``` {.rust .rust-example-rendered}
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

assert_eq!(URL_SAFE.decode(b"-uwgVQA=")?, b"\xFA\xEC\x20\x55\0");
assert_eq!(URL_SAFE.encode(b"\xFF\xEC\x20\x55\0"), "_-wgVQA=");
```

### <a href="#padding-characters" class="doc-anchor">§</a>Padding characters

Each base64 character represents 6 bits (2⁶ = 64) of the original binary
data, and every 3 bytes of input binary data will encode to 4 base64
characters (8 bits × 3 = 6 bits × 4 = 24 bits).

When the input is not an even multiple of 3 bytes in length,
[canonical](https://datatracker.ietf.org/doc/html/rfc4648#section-3.5)
base64 encoders insert padding characters at the end, so that the output
length is always a multiple of 4:

``` {.rust .rust-example-rendered}
use base64::{engine::general_purpose::STANDARD, Engine as _};

assert_eq!(STANDARD.encode(b""),    "");
assert_eq!(STANDARD.encode(b"f"),   "Zg==");
assert_eq!(STANDARD.encode(b"fo"),  "Zm8=");
assert_eq!(STANDARD.encode(b"foo"), "Zm9v");
```

Canonical encoding ensures that base64 encodings will be exactly the
same, byte-for-byte, regardless of input length. But the `=` padding
characters aren’t necessary for decoding, and they may be omitted by
using a
[`NO_PAD`](engine/general_purpose/constant.NO_PAD.html "constant base64::engine::general_purpose::NO_PAD")
configuration:

``` {.rust .rust-example-rendered}
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};

assert_eq!(STANDARD_NO_PAD.encode(b""),    "");
assert_eq!(STANDARD_NO_PAD.encode(b"f"),   "Zg");
assert_eq!(STANDARD_NO_PAD.encode(b"fo"),  "Zm8");
assert_eq!(STANDARD_NO_PAD.encode(b"foo"), "Zm9v");
```

The pre-configured `NO_PAD` engines will reject inputs containing
padding `=` characters. To encode without padding and still accept
padding while decoding, create an
[engine](engine/general_purpose/struct.GeneralPurpose.html "struct base64::engine::general_purpose::GeneralPurpose")
with that [padding
mode](engine/enum.DecodePaddingMode.html "enum base64::engine::DecodePaddingMode").

``` {.rust .rust-example-rendered}
assert_eq!(STANDARD_NO_PAD.decode(b"Zm8="), Err(base64::DecodeError::InvalidPadding));
```

### <a href="#further-customization" class="doc-anchor">§</a>Further customization

Decoding and encoding behavior can be customized by creating an
[engine](engine/general_purpose/struct.GeneralPurpose.html "struct base64::engine::general_purpose::GeneralPurpose")
with an
[alphabet](alphabet/struct.Alphabet.html "struct base64::alphabet::Alphabet")
and [padding
configuration](engine/general_purpose/struct.GeneralPurposeConfig.html "struct base64::engine::general_purpose::GeneralPurposeConfig"):

``` {.rust .rust-example-rendered}
use base64::{engine, alphabet, Engine as _};

// bizarro-world base64: +/ as the first symbols instead of the last
let alphabet =
    alphabet::Alphabet::new("+/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
    .unwrap();

// a very weird config that encodes with padding but requires no padding when decoding...?
let crazy_config = engine::GeneralPurposeConfig::new()
    .with_decode_allow_trailing_bits(true)
    .with_encode_padding(true)
    .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);

let encoded = crazy_engine.encode(b"abc 123");
```

## <a href="#memory-allocation" class="doc-anchor">§</a>Memory allocation

The
[decode](engine/trait.Engine.html#method.decode "method base64::engine::Engine::decode")
and
[encode](engine/trait.Engine.html#method.encode "method base64::engine::Engine::encode")
engine methods allocate memory for their results – `decode` returns a
`Vec<u8>` and `encode` returns a `String`. To instead decode or encode
into a buffer that you allocated, use one of the alternative methods:

#### <a href="#decoding" class="doc-anchor">§</a>Decoding

| Method | Output | Allocates memory |
|----|----|----|
| [`Engine::decode`](engine/trait.Engine.html#method.decode "method base64::engine::Engine::decode") | returns a new `Vec<u8>` | always |
| [`Engine::decode_vec`](engine/trait.Engine.html#method.decode_vec "method base64::engine::Engine::decode_vec") | appends to provided `Vec<u8>` | if `Vec` lacks capacity |
| [`Engine::decode_slice`](engine/trait.Engine.html#method.decode_slice "method base64::engine::Engine::decode_slice") | writes to provided `&[u8]` | never |

#### <a href="#encoding" class="doc-anchor">§</a>Encoding

| Method | Output | Allocates memory |
|----|----|----|
| [`Engine::encode`](engine/trait.Engine.html#method.encode "method base64::engine::Engine::encode") | returns a new `String` | always |
| [`Engine::encode_string`](engine/trait.Engine.html#method.encode_string "method base64::engine::Engine::encode_string") | appends to provided `String` | if `String` lacks capacity |
| [`Engine::encode_slice`](engine/trait.Engine.html#method.encode_slice "method base64::engine::Engine::encode_slice") | writes to provided `&[u8]` | never |

## <a href="#input-and-output" class="doc-anchor">§</a>Input and output

The `base64` crate can
[decode](engine/trait.Engine.html#method.decode "method base64::engine::Engine::decode")
and
[encode](engine/trait.Engine.html#method.encode "method base64::engine::Engine::encode")
values in memory, or
[`DecoderReader`](read/struct.DecoderReader.html "struct base64::read::DecoderReader")
and
[`EncoderWriter`](write/struct.EncoderWriter.html "struct base64::write::EncoderWriter")
provide streaming decoding and encoding for any
[readable](https://doc.rust-lang.org/1.93.1/std/io/trait.Read.html "trait std::io::Read")
or
[writable](https://doc.rust-lang.org/1.93.1/std/io/trait.Write.html "trait std::io::Write")
byte stream.

#### <a href="#decoding-1" class="doc-anchor">§</a>Decoding

``` {.rust .rust-example-rendered}
use base64::{engine::general_purpose::STANDARD, read::DecoderReader};

let mut input = io::stdin();
let mut decoder = DecoderReader::new(&mut input, &STANDARD);
io::copy(&mut decoder, &mut io::stdout())?;
```

#### <a href="#encoding-1" class="doc-anchor">§</a>Encoding

``` {.rust .rust-example-rendered}
use base64::{engine::general_purpose::STANDARD, write::EncoderWriter};

let mut output = io::stdout();
let mut encoder = EncoderWriter::new(&mut output, &STANDARD);
io::copy(&mut io::stdin(), &mut encoder)?;
```

#### <a href="#display" class="doc-anchor">§</a>Display

If you only need a base64 representation for implementing the
[`Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display")
trait, use
[`Base64Display`](display/struct.Base64Display.html "struct base64::display::Base64Display"):

``` {.rust .rust-example-rendered}
use base64::{display::Base64Display, engine::general_purpose::STANDARD};

let value = Base64Display::new(b"\0\x01\x02\x03", &STANDARD);
assert_eq!("base64: AAECAw==", format!("base64: {}", value));
```

# <a href="#panics" class="doc-anchor">§</a>Panics

If length calculations result in overflowing `usize`, a panic will
result.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use engine::`<a href="engine/trait.Engine.html" class="trait"
title="trait base64::engine::Engine"><code>Engine</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="alphabet/index.html" class="mod"
title="mod base64::alphabet">alphabet</a>  
Provides
[Alphabet](alphabet/struct.Alphabet.html "struct base64::alphabet::Alphabet")
and constants for alphabets commonly used in the wild.

<a href="display/index.html" class="mod"
title="mod base64::display">display</a>  
Enables base64’d output anywhere you might use a `Display`
implementation, like a format string.

<a href="engine/index.html" class="mod"
title="mod base64::engine">engine</a>  
Provides the
[Engine](engine/trait.Engine.html "trait base64::engine::Engine")
abstraction and out of the box implementations.

<a href="prelude/index.html" class="mod"
title="mod base64::prelude">prelude</a>  
Preconfigured engines for common use cases.

<a href="read/index.html" class="mod" title="mod base64::read">read</a>  
Implementations of `io::Read` to transparently decode base64.

<a href="write/index.html" class="mod"
title="mod base64::write">write</a>  
Implementations of `io::Write` to transparently handle base64.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DecodeError.html" class="enum"
title="enum base64::DecodeError">DecodeError</a>  
Errors that can occur while decoding.

<a href="enum.DecodeSliceError.html" class="enum"
title="enum base64::DecodeSliceError">DecodeSliceError</a>  
Errors that can occur while decoding into a slice.

<a href="enum.EncodeSliceError.html" class="enum"
title="enum base64::EncodeSliceError">EncodeSliceError</a>  
Errors that can occur while encoding into a slice.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decode.html" class="fn" title="fn base64::decode">decode</a><span class="stab deprecated" title="">Deprecated</span>  
Decode base64 using the [`STANDARD`
engine](engine/general_purpose/constant.STANDARD.html "constant base64::engine::general_purpose::STANDARD").

<a href="fn.decode_engine.html" class="fn"
title="fn base64::decode_engine">decode_engine</a><span class="stab deprecated" title="">Deprecated</span>  
Decode from string reference as octets using the specified
[Engine](engine/trait.Engine.html "trait base64::engine::Engine").

<a href="fn.decode_engine_slice.html" class="fn"
title="fn base64::decode_engine_slice">decode_engine_slice</a><span class="stab deprecated" title="">Deprecated</span>  
Decode the input into the provided output slice.

<a href="fn.decode_engine_vec.html" class="fn"
title="fn base64::decode_engine_vec">decode_engine_vec</a><span class="stab deprecated" title="">Deprecated</span>  
Decode from string reference as octets.

<a href="fn.decoded_len_estimate.html" class="fn"
title="fn base64::decoded_len_estimate">decoded_len_estimate</a>  
Returns a conservative estimate of the decoded size of `encoded_len`
base64 symbols (rounded up to the next group of 3 decoded bytes).

<a href="fn.encode.html" class="fn" title="fn base64::encode">encode</a><span class="stab deprecated" title="">Deprecated</span>  
Encode arbitrary octets as base64 using the [`STANDARD`
engine](engine/general_purpose/constant.STANDARD.html "constant base64::engine::general_purpose::STANDARD").

<a href="fn.encode_engine.html" class="fn"
title="fn base64::encode_engine">encode_engine</a><span class="stab deprecated" title="">Deprecated</span>  
Encode arbitrary octets as base64 using the provided `Engine` into a new
`String`.

<a href="fn.encode_engine_slice.html" class="fn"
title="fn base64::encode_engine_slice">encode_engine_slice</a><span class="stab deprecated" title="">Deprecated</span>  
Encode arbitrary octets as base64 into a supplied slice.

<a href="fn.encode_engine_string.html" class="fn"
title="fn base64::encode_engine_string">encode_engine_string</a><span class="stab deprecated" title="">Deprecated</span>  
Encode arbitrary octets as base64 into a supplied `String`.

<a href="fn.encoded_len.html" class="fn"
title="fn base64::encoded_len">encoded_len</a>  
Calculate the base64 encoded length for a given input length, optionally
including any appropriate padding bytes.
