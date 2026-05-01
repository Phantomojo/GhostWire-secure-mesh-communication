Crate urlencoding Copy item path

<span class="sub-heading"><a href="../src/urlencoding/lib.rs.html#1-133" class="src">Source</a>
</span>

Expand description

To encode a string, do the following:

``` {.rust .rust-example-rendered}
use urlencoding::encode;

let encoded = encode("This string will be URL encoded.");
println!("{}", encoded);
// This%20string%20will%20be%20URL%20encoded.
```

To decode a string, it’s only slightly different:

``` {.rust .rust-example-rendered}
use urlencoding::decode;

let decoded = decode("%F0%9F%91%BE%20Exterminate%21").expect("UTF-8");
println!("{}", decoded);
// 👾 Exterminate!
```

To decode allowing arbitrary bytes and invalid UTF-8:

``` {.rust .rust-example-rendered}
use urlencoding::decode_binary;

let binary = decode_binary(b"%F1%F2%F3%C0%C1%C2");
let decoded = String::from_utf8_lossy(&binary);
```

This library returns
[`Cow`](https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html) to
avoid allocating when decoding/encoding is not needed. Call
`.into_owned()` on the `Cow` to get a `Vec` or `String`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Encoded.html" class="struct"
title="struct urlencoding::Encoded">Encoded</a>  
Wrapper type that implements `Display`. Encodes on the fly, without
allocating. Percent-encodes every byte except alphanumerics and `-`,
`_`, `.`, `~`. Assumes UTF-8 encoding.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decode.html" class="fn"
title="fn urlencoding::decode">decode</a>  
Decode percent-encoded string assuming UTF-8 encoding.

<a href="fn.decode_binary.html" class="fn"
title="fn urlencoding::decode_binary">decode_binary</a>  
Decode percent-encoded string as binary data, in any encoding.

<a href="fn.encode.html" class="fn"
title="fn urlencoding::encode">encode</a>  
Percent-encodes every byte except alphanumerics and `-`, `_`, `.`, `~`.
Assumes UTF-8 encoding.

<a href="fn.encode_binary.html" class="fn"
title="fn urlencoding::encode_binary">encode_binary</a>  
Percent-encodes every byte except alphanumerics and `-`, `_`, `.`, `~`.
