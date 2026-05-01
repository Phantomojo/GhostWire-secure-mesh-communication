Crate fdeflate Copy item path

<span class="sub-heading"><a href="../src/fdeflate/lib.rs.html#1-124" class="src">Source</a>
</span>

Expand description

A fast deflate implementation.

This crate contains an optimized implementation of the deflate algorithm
tuned to compress PNG images. It is compatible with standard zlib, but
make a bunch of simplifying assumptions that drastically improve
encoding performance:

-   Exactly one block per deflate stream.
-   No distance codes except for run length encoding of zeros.
-   A single fixed huffman tree trained on a large corpus of PNG images.
-   All huffman codes are 12 bits or less.

It also contains a fast decompressor that supports arbitrary zlib
streams but does especially well on streams that meet the above
assumptions.

# <a href="#inspiration" class="doc-anchor">§</a>Inspiration

The algorithms in this crate take inspiration from multiple sources:

-   [fpnge](https://github.com/veluca93/fpnge)
-   [zune-inflate](https://github.com/etemesi254/zune-image/tree/main/zune-inflate)
-   [RealTime Data Compression
    blog](https://fastcompression.blogspot.com/2015/10/huffman-revisited-part-4-multi-bytes.html)

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Compressor.html" class="struct"
title="struct fdeflate::Compressor">Compressor</a>  
Compressor that produces fdeflate compressed streams.

<a href="struct.Decompressor.html" class="struct"
title="struct fdeflate::Decompressor">Decompressor</a>  
Decompressor for arbitrary zlib streams.

<a href="struct.StoredOnlyCompressor.html" class="struct"
title="struct fdeflate::StoredOnlyCompressor">StoredOnlyCompressor</a>  
Compressor that only writes the stored blocks.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.BoundedDecompressionError.html" class="enum"
title="enum fdeflate::BoundedDecompressionError">BoundedDecompressionError</a>  
An error encountered while decompressing a deflate stream given a
bounded maximum output.

<a href="enum.DecompressionError.html" class="enum"
title="enum fdeflate::DecompressionError">DecompressionError</a>  
An error encountered while decompressing a deflate stream.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.compress_to_vec.html" class="fn"
title="fn fdeflate::compress_to_vec">compress_to_vec</a>  
Compresses the given data.

<a href="fn.decompress_to_vec.html" class="fn"
title="fn fdeflate::decompress_to_vec">decompress_to_vec</a>  
Decompress the given data.

<a href="fn.decompress_to_vec_bounded.html" class="fn"
title="fn fdeflate::decompress_to_vec_bounded">decompress_to_vec_bounded</a>  
Decompress the given data, returning an error if the output is larger
than `maxlen` bytes.
