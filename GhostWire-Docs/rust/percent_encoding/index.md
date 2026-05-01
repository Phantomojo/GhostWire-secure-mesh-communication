Crate percent\_encoding Copy item path

<span class="sub-heading"><a href="../src/percent_encoding/lib.rs.html#9-481"
class="src">Source</a> </span>

Expand description

URLs use special characters to indicate the parts of the request. For
example, a `?` question mark marks the end of a path and the start of a
query string. In order for that character to exist inside a path, it
needs to be encoded differently.

Percent encoding replaces reserved characters with the `%` escape
character followed by a byte value as two hexadecimal digits. For
example, an ASCII space is replaced with `%20`.

When encoding, the set of characters that can (and should, for
readability) be left alone depends on the context. The `?` question mark
mentioned above is not a separator when used literally inside of a query
string, and therefore does not need to be encoded. The
[`AsciiSet`](struct.AsciiSet.html "struct percent_encoding::AsciiSet")
parameter of
[`percent_encode`](fn.percent_encode.html "fn percent_encoding::percent_encode")
and
[`utf8_percent_encode`](fn.utf8_percent_encode.html "fn percent_encoding::utf8_percent_encode")
lets callers configure this.

This crate deliberately does not provide many different sets. Users
should consider in what context the encoded string will be used, read
relevant specifications, and define their own set. This is done by using
the `add` method of an existing set.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

assert_eq!(utf8_percent_encode("foo <bar>", FRAGMENT).to_string(), "foo%20%3Cbar%3E");
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AsciiSet.html" class="struct"
title="struct percent_encoding::AsciiSet">AsciiSet</a>  
Represents a set of characters or bytes in the ASCII range.

<a href="struct.PercentDecode.html" class="struct"
title="struct percent_encoding::PercentDecode">PercentDecode</a>  
The return type of
[`percent_decode`](fn.percent_decode.html "fn percent_encoding::percent_decode").

<a href="struct.PercentEncode.html" class="struct"
title="struct percent_encoding::PercentEncode">PercentEncode</a>  
The return type of
[`percent_encode`](fn.percent_encode.html "fn percent_encoding::percent_encode")
and
[`utf8_percent_encode`](fn.utf8_percent_encode.html "fn percent_encoding::utf8_percent_encode").

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.CONTROLS.html" class="constant"
title="constant percent_encoding::CONTROLS">CONTROLS</a>  
The set of 0x00 to 0x1F (C0 controls), and 0x7F (DEL).

<a href="constant.NON_ALPHANUMERIC.html" class="constant"
title="constant percent_encoding::NON_ALPHANUMERIC">NON_ALPHANUMERIC</a>  
Everything that is not an ASCII letter or digit.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.percent_decode.html" class="fn"
title="fn percent_encoding::percent_decode">percent_decode</a>  
Percent-decode the given bytes.

<a href="fn.percent_decode_str.html" class="fn"
title="fn percent_encoding::percent_decode_str">percent_decode_str</a>  
Percent-decode the given string.

<a href="fn.percent_encode.html" class="fn"
title="fn percent_encoding::percent_encode">percent_encode</a>  
Percent-encode the given bytes with the given set.

<a href="fn.percent_encode_byte.html" class="fn"
title="fn percent_encoding::percent_encode_byte">percent_encode_byte</a>  
Return the percent-encoding of the given byte.

<a href="fn.utf8_percent_encode.html" class="fn"
title="fn percent_encoding::utf8_percent_encode">utf8_percent_encode</a>  
Percent-encode the UTF-8 encoding of the given string.
