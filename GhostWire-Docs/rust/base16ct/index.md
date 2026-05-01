Crate base16ct Copy item path

<span class="sub-heading"><a href="../src/base16ct/lib.rs.html#1-124" class="src">Source</a>
</span>

Expand description

Pure Rust implementation of Base16 ([RFC
4648](https://tools.ietf.org/html/rfc4648), a.k.a. hex).

Implements lower and upper case Base16 variants without data-dependent
branches or lookup tables, thereby providing portable “best effort”
constant-time operation. Not constant-time with respect to message
length (only data).

Supports `no_std` environments and avoids heap allocations in the core
API (but also provides optional `alloc` support for convenience).

Based on code from:
<https://github.com/Sc00bz/ConstTimeEncoding/blob/master/hex.cpp>

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
let lower_hex_str = "abcd1234";
let upper_hex_str = "ABCD1234";
let mixed_hex_str = "abCD1234";
let raw = b"\xab\xcd\x12\x34";

let mut buf = [0u8; 16];
// length of return slice can be different from the input buffer!
let res = base16ct::lower::decode(lower_hex_str, &mut buf).unwrap();
assert_eq!(res, raw);
let res = base16ct::lower::encode(raw, &mut buf).unwrap();
assert_eq!(res, lower_hex_str.as_bytes());
// you also can use `encode_str` and `encode_string` to get
// `&str` and `String` respectively
let res: &str = base16ct::lower::encode_str(raw, &mut buf).unwrap();
assert_eq!(res, lower_hex_str);

let res = base16ct::upper::decode(upper_hex_str, &mut buf).unwrap();
assert_eq!(res, raw);
let res = base16ct::upper::encode(raw, &mut buf).unwrap();
assert_eq!(res, upper_hex_str.as_bytes());

// In cases when you don't know if input contains upper or lower
// hex-encoded value, then use functions from the `mixed` module
let res = base16ct::mixed::decode(lower_hex_str, &mut buf).unwrap();
assert_eq!(res, raw);
let res = base16ct::mixed::decode(upper_hex_str, &mut buf).unwrap();
assert_eq!(res, raw);
let res = base16ct::mixed::decode(mixed_hex_str, &mut buf).unwrap();
assert_eq!(res, raw);
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="lower/index.html" class="mod"
title="mod base16ct::lower">lower</a>  
Function for decoding and encoding lower Base16 (hex)

<a href="mixed/index.html" class="mod"
title="mod base16ct::mixed">mixed</a>  
Function for decoding mixed Base16 (hex)

<a href="upper/index.html" class="mod"
title="mod base16ct::upper">upper</a>  
Function for decoding and encoding upper Base16 (hex)

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.HexDisplay.html" class="struct"
title="struct base16ct::HexDisplay">HexDisplay</a>  
`core::fmt` presenter for binary data encoded as hexadecimal (Base16).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum base16ct::Error">Error</a>  
Error type

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decoded_len.html" class="fn"
title="fn base16ct::decoded_len">decoded_len</a>  
Compute decoded length of the given hex-encoded input.

<a href="fn.encoded_len.html" class="fn"
title="fn base16ct::encoded_len">encoded_len</a>  
Get the length of Base16 (hex) produced by encoding the given bytes.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type base16ct::Result">Result</a>  
Result type with the `base16ct` crate’s
[`Error`](enum.Error.html "enum base16ct::Error") type.
