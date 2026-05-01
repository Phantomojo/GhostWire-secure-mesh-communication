Crate futures\_io Copy item path

<span class="sub-heading"><a href="../src/futures_io/lib.rs.html#1-561" class="src">Source</a>
</span>

Expand description

Asynchronous I/O

This crate contains the `AsyncRead`, `AsyncWrite`, `AsyncSeek`, and
`AsyncBufRead` traits, the asynchronous analogs to
`std::io::{Read, Write, Seek, BufRead}`. The primary difference is that
these traits integrate with the asynchronous task system.

All items of this library are only available when the `std` feature of
this library is activated, and it is activated by default.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use io::`<a
href="https://doc.rust-lang.org/1.93.1/std/io/error/struct.Error.html"
class="struct"
title="struct std::io::error::Error"><code>Error</code></a>`;`

`pub use io::`<a
href="https://doc.rust-lang.org/1.93.1/std/io/error/enum.ErrorKind.html"
class="enum"
title="enum std::io::error::ErrorKind"><code>ErrorKind</code></a>`;`

`pub use io::`<a href="https://doc.rust-lang.org/1.93.1/std/io/struct.IoSlice.html"
class="struct" title="struct std::io::IoSlice"><code>IoSlice</code></a>`;`

`pub use io::`<a href="https://doc.rust-lang.org/1.93.1/std/io/struct.IoSliceMut.html"
class="struct"
title="struct std::io::IoSliceMut"><code>IoSliceMut</code></a>`;`

`pub use io::`<a href="https://doc.rust-lang.org/1.93.1/std/io/error/type.Result.html"
class="type" title="type std::io::error::Result"><code>Result</code></a>`;`

`pub use io::`<a href="https://doc.rust-lang.org/1.93.1/std/io/enum.SeekFrom.html"
class="enum" title="enum std::io::SeekFrom"><code>SeekFrom</code></a>`;`

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsyncBufRead.html" class="trait"
title="trait futures_io::AsyncBufRead">AsyncBufRead</a>  
Read bytes asynchronously.

<a href="trait.AsyncRead.html" class="trait"
title="trait futures_io::AsyncRead">AsyncRead</a>  
Read bytes asynchronously.

<a href="trait.AsyncSeek.html" class="trait"
title="trait futures_io::AsyncSeek">AsyncSeek</a>  
Seek bytes asynchronously.

<a href="trait.AsyncWrite.html" class="trait"
title="trait futures_io::AsyncWrite">AsyncWrite</a>  
Write bytes asynchronously.
