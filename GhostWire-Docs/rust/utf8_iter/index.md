Crate utf8\_iter Copy item path

<span class="sub-heading"><a href="../src/utf8_iter/lib.rs.html#17-279" class="src">Source</a>
</span>

Expand description

Provides iteration by `char` over `&[u8]` containing potentially-invalid
UTF-8 such that errors are handled according to the [WHATWG Encoding
Standard](https://encoding.spec.whatwg.org/#utf-8-decoder) (i.e. the
same way as in `String::from_utf8_lossy`).

The trait `Utf8CharsEx` provides the convenience method `chars()` on
byte slices themselves instead of having to use the more verbose
`Utf8Chars::new(slice)`.

``` {.rust .rust-example-rendered}
use utf8_iter::Utf8CharsEx;
let data = b"\xFF\xC2\xE2\xE2\x98\xF0\xF0\x9F\xF0\x9F\x92\xE2\x98\x83";
let from_iter: String = data.chars().collect();
let from_std = String::from_utf8_lossy(data);
assert_eq!(from_iter, from_std);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ErrorReportingUtf8Chars.html" class="struct"
title="struct utf8_iter::ErrorReportingUtf8Chars">ErrorReportingUtf8Chars</a>  
Iterator by `Result<char,Utf8CharsError>` over `&[u8]` that contains
potentially-invalid UTF-8. There is exactly one `Utf8CharsError` per
each error as defined by the WHATWG Encoding Standard.

<a href="struct.Utf8CharIndices.html" class="struct"
title="struct utf8_iter::Utf8CharIndices">Utf8CharIndices</a>  
An iterator over the
[`char`](https://doc.rust-lang.org/1.93.1/core/primitive.char.html "primitive char")s
and their positions.

<a href="struct.Utf8Chars.html" class="struct"
title="struct utf8_iter::Utf8Chars">Utf8Chars</a>  
Iterator by `char` over `&[u8]` that contains potentially-invalid UTF-8.
See the crate documentation.

<a href="struct.Utf8CharsError.html" class="struct"
title="struct utf8_iter::Utf8CharsError">Utf8CharsError</a>  
A type for signaling UTF-8 errors.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Utf8CharsEx.html" class="trait"
title="trait utf8_iter::Utf8CharsEx">Utf8CharsEx</a>  
Convenience trait that adds `chars()` and `char_indices()` methods
similar to the ones on string slices to byte slices.
