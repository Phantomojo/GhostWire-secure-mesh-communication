Crate async\_compression Copy item path

<span class="sub-heading"><a href="../src/async_compression/lib.rs.html#1-172"
class="src">Source</a> </span>

Expand description

Adaptors between compression crates and Rust’s modern asynchronous IO
types.

# <a href="#feature-organization" class="doc-anchor">§</a>Feature Organization

This crate is divided up along two axes, which can each be individually
selected via Cargo features.

All features are disabled by default, you should enable just the ones
you need from the lists below.

If you want to pull in everything there are three group features
defined:

| Feature | Does |
|----|----|
| `all` | Activates all implementations and algorithms. |
| `all-implementations` | Activates all implementations, needs to be paired with a selection of algorithms |
| `all-algorithms` | Activates all algorithms, needs to be paired with a selection of implementations |

## <a href="#io-implementation" class="doc-anchor">§</a>IO implementation

The first division is which underlying asynchronous IO trait will be
wrapped, these are available as separate features that have
corresponding top-level modules:

| Feature | Type |
|----|----|
| `futures-io` (*inactive*) | `futures::io::AsyncBufRead`, `futures::io::AsyncWrite` |
| [`tokio`](tokio/index.html "mod async_compression::tokio") | [`tokio::io::AsyncBufRead`](../tokio/io/async_buf_read/trait.AsyncBufRead.html "trait tokio::io::async_buf_read::AsyncBufRead"), [`tokio::io::AsyncWrite`](../tokio/io/async_write/trait.AsyncWrite.html "trait tokio::io::async_write::AsyncWrite") |

## <a href="#compression-algorithm" class="doc-anchor">§</a>Compression algorithm

The second division is which compression schemes to support, there are
currently a few available choices, these determine which types will be
available inside the above modules:

| Feature | Types |
|----|----|
| `brotli` (*inactive*) | `BrotliEncoder`, `BrotliDecoder` |
| `bzip2` (*inactive*) | `BzEncoder`, `BzDecoder` |
| `deflate` (*inactive*) | `DeflateEncoder`, `DeflateDecoder` |
| `gzip` | [`GzipEncoder`](?search=GzipEncoder), [`GzipDecoder`](?search=GzipDecoder) |
| `lz4` (*inactive*) | `Lz4Encoder`, `Lz4Decoder` |
| `lzma` (*inactive*) | `LzmaEncoder`, `LzmaDecoder` |
| `xz` (*inactive*) | `XzEncoder`, `XzDecoder` |
| `zlib` (*inactive*) | `ZlibEncoder`, `ZlibDecoder` |
| `zstd` (*inactive*) | `ZstdEncoder`, `ZstdDecoder` |
| `deflate64` (*inactive*) | (encoder not implemented), `Deflate64Decoder` |

## <a href="#multi-thread-support" class="doc-anchor">§</a>Multi-thread support

The `xz` compression algorithm supports multi-threaded compression and
decompression. Enable the `xz-parallel` feature to enable
multi-threading support.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../compression_codecs/index.html" class="mod"
title="mod compression_codecs"><code>compression_codecs</code></a>` as codecs;`

`pub use `<a href="../compression_core/index.html" class="mod"
title="mod compression_core"><code>compression_core</code></a>` as core;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="tokio/index.html" class="mod"
title="mod async_compression::tokio">tokio</a>  
Implementations for IO traits exported by [`tokio`
v1.x](../tokio/index.html "mod tokio").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Level.html" class="enum"
title="enum async_compression::Level">Level</a>  
Level of compression data should be compressed with.
