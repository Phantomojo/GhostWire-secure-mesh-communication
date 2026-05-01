Crate serde\_bytes Copy item path

<span class="sub-heading"><a href="../src/serde_bytes/lib.rs.html#1-126" class="src">Source</a>
</span>

Expand description

Wrapper types to enable optimized handling of `&[u8]` and `Vec<u8>`.

Without specialization, Rust forces Serde to treat `&[u8]` just like any
other slice and `Vec<u8>` just like any other vector. In reality this
particular slice and vector can often be serialized and deserialized in
a more efficient, compact representation in many formats.

When working with such a format, you can opt into specialized handling
of `&[u8]` by wrapping it in `serde_bytes::Bytes` and `Vec<u8>` by
wrapping it in `serde_bytes::ByteBuf`.

Additionally this crate supports the Serde `with` attribute to enable
efficient handling of `&[u8]` and `Vec<u8>` in structs without needing a
wrapper type.

``` {.rust .rust-example-rendered}
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Efficient<'a> {
    #[serde(with = "serde_bytes")]
    bytes: &'a [u8],

    #[serde(with = "serde_bytes")]
    byte_buf: Vec<u8>,

    #[serde(with = "serde_bytes")]
    byte_array: [u8; 314],
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ByteArray.html" class="struct"
title="struct serde_bytes::ByteArray">ByteArray</a>  
Wrapper around `[u8; N]` to serialize and deserialize efficiently.

<a href="struct.ByteBuf.html" class="struct"
title="struct serde_bytes::ByteBuf">ByteBuf</a>  
Wrapper around `Vec<u8>` to serialize and deserialize efficiently.

<a href="struct.Bytes.html" class="struct"
title="struct serde_bytes::Bytes">Bytes</a>  
Wrapper around `[u8]` to serialize and deserialize efficiently.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Deserialize.html" class="trait"
title="trait serde_bytes::Deserialize">Deserialize</a>  
Types that can be deserialized via `#[serde(with = "serde_bytes")]`.

<a href="trait.Serialize.html" class="trait"
title="trait serde_bytes::Serialize">Serialize</a>  
Types that can be serialized via `#[serde(with = "serde_bytes")]`.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.deserialize.html" class="fn"
title="fn serde_bytes::deserialize">deserialize</a>  
Serde `deserialize_with` function to deserialize bytes efficiently.

<a href="fn.serialize.html" class="fn"
title="fn serde_bytes::serialize">serialize</a>  
Serde `serialize_with` function to serialize bytes efficiently.
