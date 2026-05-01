Crate flate2 Copy item path

<span class="sub-heading"><a href="../src/flate2/lib.rs.html#1-232" class="src">Source</a>
</span>

Expand description

A DEFLATE-based stream compression/decompression library

This library provides support for compression and decompression of
DEFLATE-based streams:

-   the DEFLATE format itself
-   the zlib format
-   gzip

These three formats are all closely related and largely only differ in
their headers/footers. This crate has three types in each submodule for
dealing with these three formats.

# <a href="#implementation" class="doc-anchor">§</a>Implementation

In addition to supporting three formats, this crate supports several
different backends, controlled through this crate’s features:

-   `default`, or `rust_backend` - this implementation uses the
    `miniz_oxide` crate which is a port of `miniz.c` to Rust. This
    feature does not require a C compiler, and only uses safe Rust code.

-   `zlib-rs` - this implementation utilizes the `zlib-rs` crate, a Rust
    rewrite of zlib. This backend is the fastest, at the cost of some
    `unsafe` Rust code.

Several backends implemented in C are also available. These are useful
in case you are already using a specific C implementation and need the
result of compression to be bit-identical. See the crate’s README for
details on the available C backends.

The `zlib-rs` backend typically outperforms all the C implementations.

# <a href="#organization" class="doc-anchor">§</a>Organization

This crate consists mainly of three modules, [`read`](read/index.html),
[`write`](write/index.html), and [`bufread`](bufread/index.html). Each
module contains a number of types used to encode and decode various
streams of data.

All types in the [`write`](write/index.html) module work on instances of
[`Write`](https://doc.rust-lang.org/std/io/trait.Write.html), whereas
all types in the [`read`](read/index.html) module work on instances of
[`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) and
[`bufread`](bufread/index.html) works with
[`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html). If you
are decoding directly from a `&[u8]`, use the
[`bufread`](bufread/index.html) types.

``` {.rust .rust-example-rendered}
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io;
use std::io::prelude::*;

let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
encoder.write_all(b"Example")?;
```

Other various types are provided at the top-level of the crate for
management and dealing with encoders/decoders. Also note that types
which operate over a specific trait often implement the mirroring trait
as well. For example a `flate2::read::DeflateDecoder<T>` *also*
implements the `Write` trait if `T: Write`. That is, the “dual trait” is
forwarded directly to the underlying object if available.

# <a href="#about-multi-member-gzip-files" class="doc-anchor">§</a>About multi-member Gzip files

While most `gzip` files one encounters will have a single *member* that
can be read with the [`GzDecoder`](read/struct.GzDecoder.html), there
may be some files which have multiple members.

A [`GzDecoder`](read/struct.GzDecoder.html) will only read the first
member of gzip data, which may unexpectedly provide partial results when
a multi-member gzip file is encountered. `GzDecoder` is appropriate for
data that is designed to be read as single members from a multi-member
file. `bufread::GzDecoder` and `write::GzDecoder` also allow non-gzip
data following gzip data to be handled.

The [`MultiGzDecoder`](read/struct.MultiGzDecoder.html) on the other
hand will decode all members of a `gzip` file into one consecutive
stream of bytes, which hides the underlying *members* entirely. If a
file contains non-gzip data after the gzip data, MultiGzDecoder will
emit an error after decoding the gzip data. This behavior matches the
`gzip`, `gunzip`, and `zcat` command line tools.

# Modules<a href="#modules" class="anchor">§</a>

<a href="bufread/index.html" class="mod"
title="mod flate2::bufread">bufread</a>  
Types which operate over
[`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)
streams, both encoders and decoders for various formats.

<a href="read/index.html" class="mod" title="mod flate2::read">read</a>  
Types which operate over
[`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) streams, both
encoders and decoders for various formats.

<a href="write/index.html" class="mod"
title="mod flate2::write">write</a>  
Types which operate over
[`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) streams,
both encoders and decoders for various formats.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Compress.html" class="struct"
title="struct flate2::Compress">Compress</a>  
Raw in-memory compression stream for blocks of data.

<a href="struct.CompressError.html" class="struct"
title="struct flate2::CompressError">CompressError</a>  
Error returned when a compression object is used incorrectly or
otherwise generates an error.

<a href="struct.Compression.html" class="struct"
title="struct flate2::Compression">Compression</a>  
When compressing data, the compression level can be specified by a value
in this struct.

<a href="struct.Crc.html" class="struct"
title="struct flate2::Crc">Crc</a>  
The CRC calculated by a [`CrcReader`](struct.CrcReader.html).

<a href="struct.CrcReader.html" class="struct"
title="struct flate2::CrcReader">CrcReader</a>  
A wrapper around a
[`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) that
calculates the CRC.

<a href="struct.CrcWriter.html" class="struct"
title="struct flate2::CrcWriter">CrcWriter</a>  
A wrapper around a
[`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) that
calculates the CRC.

<a href="struct.Decompress.html" class="struct"
title="struct flate2::Decompress">Decompress</a>  
Raw in-memory decompression stream for blocks of data.

<a href="struct.DecompressError.html" class="struct"
title="struct flate2::DecompressError">DecompressError</a>  
Error returned when a decompression object finds that the input stream
of bytes was not a valid input stream of bytes.

<a href="struct.GzBuilder.html" class="struct"
title="struct flate2::GzBuilder">GzBuilder</a>  
A builder structure to create a new gzip Encoder.

<a href="struct.GzHeader.html" class="struct"
title="struct flate2::GzHeader">GzHeader</a>  
A structure representing the header of a gzip stream.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.FlushCompress.html" class="enum"
title="enum flate2::FlushCompress">FlushCompress</a>  
Values which indicate the form of flushing to be used when compressing
in-memory data.

<a href="enum.FlushDecompress.html" class="enum"
title="enum flate2::FlushDecompress">FlushDecompress</a>  
Values which indicate the form of flushing to be used when decompressing
in-memory data.

<a href="enum.Status.html" class="enum"
title="enum flate2::Status">Status</a>  
Possible status results of compressing some data or successfully
decompressing a block of data.
