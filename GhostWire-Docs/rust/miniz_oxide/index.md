Crate miniz\_oxide Copy item path

<span class="sub-heading"><a href="../src/miniz_oxide/lib.rs.html#1-222" class="src">Source</a>
</span>

Expand description

A pure rust replacement for the
[miniz](https://github.com/richgel999/miniz) DEFLATE/zlib
encoder/decoder. Used a rust back-end for the
[flate2](https://github.com/alexcrichton/flate2-rs) crate.

# <a href="#usage" class="doc-anchor">§</a>Usage

## <a href="#simple-compressiondecompression" class="doc-anchor">§</a>Simple compression/decompression:

``` {.rust .rust-example-rendered}
use miniz_oxide::inflate::decompress_to_vec;
use miniz_oxide::deflate::compress_to_vec;

fn roundtrip(data: &[u8]) {
    let compressed = compress_to_vec(data, 6);
    let decompressed = decompress_to_vec(compressed.as_slice()).expect("Failed to decompress!");
}
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="deflate/index.html" class="mod"
title="mod miniz_oxide::deflate">deflate</a>  
This module contains functionality for compression.

<a href="inflate/index.html" class="mod"
title="mod miniz_oxide::inflate">inflate</a>  
This module contains functionality for decompression.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.StreamResult.html" class="struct"
title="struct miniz_oxide::StreamResult">StreamResult</a>  
A structure containing the result of a call to the inflate or deflate
streaming functions.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DataFormat.html" class="enum"
title="enum miniz_oxide::DataFormat">DataFormat</a>  
How compressed data is wrapped.

<a href="enum.MZError.html" class="enum"
title="enum miniz_oxide::MZError">MZError</a>  
A list of miniz failed status codes.

<a href="enum.MZFlush.html" class="enum"
title="enum miniz_oxide::MZFlush">MZFlush</a>  
A list of flush types.

<a href="enum.MZStatus.html" class="enum"
title="enum miniz_oxide::MZStatus">MZStatus</a>  
A list of miniz successful status codes.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.MZResult.html" class="type"
title="type miniz_oxide::MZResult">MZResult</a>  
`Result` alias for all miniz status codes both successful and failed.
