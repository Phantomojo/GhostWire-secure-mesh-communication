Crate hex Copy item path

<span class="sub-heading"><a href="../src/hex/lib.rs.html#9-525" class="src">Source</a>
</span>

Expand description

Encoding and decoding hex strings.

For most cases, you can simply use the
[`decode`](fn.decode.html "fn hex::decode"),
[`encode`](fn.encode.html "fn hex::encode") and
[`encode_upper`](fn.encode_upper.html "fn hex::encode_upper") functions.
If you need a bit more control, use the traits
[`ToHex`](trait.ToHex.html "trait hex::ToHex") and
[`FromHex`](trait.FromHex.html "trait hex::FromHex") instead.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
let hex_string = hex::encode("Hello world!");

println!("{}", hex_string); // Prints "48656c6c6f20776f726c6421"
```

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.FromHexError.html" class="enum"
title="enum hex::FromHexError">FromHexError</a>  
The error type for decoding a hex string into `Vec<u8>` or `[u8; N]`.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.FromHex.html" class="trait"
title="trait hex::FromHex">FromHex</a>  
Types that can be decoded from a hex string.

<a href="trait.ToHex.html" class="trait"
title="trait hex::ToHex">ToHex</a>  
Encoding values as hex string.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decode.html" class="fn" title="fn hex::decode">decode</a>  
Decodes a hex string into raw bytes.

<a href="fn.decode_to_slice.html" class="fn"
title="fn hex::decode_to_slice">decode_to_slice</a>  
Decode a hex string into a mutable bytes slice.

<a href="fn.encode.html" class="fn" title="fn hex::encode">encode</a>  
Encodes `data` as hex string using lowercase characters.

<a href="fn.encode_to_slice.html" class="fn"
title="fn hex::encode_to_slice">encode_to_slice</a>  
Encodes some bytes into a mutable slice of bytes.

<a href="fn.encode_upper.html" class="fn"
title="fn hex::encode_upper">encode_upper</a>  
Encodes `data` as hex string using uppercase characters.
