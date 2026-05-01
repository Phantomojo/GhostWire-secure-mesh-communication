Crate bytes Copy item path

<span class="sub-heading"><a href="../src/bytes/lib.rs.html#1-182" class="src">Source</a>
</span>

Expand description

Provides abstractions for working with bytes.

The `bytes` crate provides an efficient byte buffer structure
([`Bytes`](struct.Bytes.html "struct bytes::Bytes")) and traits for
working with buffer implementations
([`Buf`](buf/trait.Buf.html "trait bytes::buf::Buf"),
[`BufMut`](buf/trait.BufMut.html "trait bytes::buf::BufMut")).

# <a href="#bytes" class="doc-anchor">§</a>`Bytes`

`Bytes` is an efficient container for storing and operating on
contiguous slices of memory. It is intended for use primarily in
networking code, but could have applications elsewhere as well.

`Bytes` values facilitate zero-copy network programming by allowing
multiple `Bytes` objects to point to the same underlying memory. This is
managed by using a reference count to track when the memory is no longer
needed and can be freed.

A `Bytes` handle can be created directly from an existing byte store
(such as `&[u8]` or `Vec<u8>`), but usually a `BytesMut` is used first
and written to. For example:

``` {.rust .rust-example-rendered}
use bytes::{BytesMut, BufMut};

let mut buf = BytesMut::with_capacity(1024);
buf.put(&b"hello world"[..]);
buf.put_u16(1234);

let a = buf.split();
assert_eq!(a, b"hello world\x04\xD2"[..]);

buf.put(&b"goodbye world"[..]);

let b = buf.split();
assert_eq!(b, b"goodbye world"[..]);

assert_eq!(buf.capacity(), 998);
```

In the above example, only a single buffer of 1024 is allocated. The
handles `a` and `b` will share the underlying buffer and maintain
indices tracking the view into the buffer represented by the handle.

See the [struct docs](struct.Bytes.html "struct bytes::Bytes") for more
details.

# <a href="#buf-bufmut" class="doc-anchor">§</a>`Buf`, `BufMut`

These two traits provide read and write access to buffers. The
underlying storage may or may not be in contiguous memory. For example,
`Bytes` is a buffer that guarantees contiguous memory, but a
[rope](https://en.wikipedia.org/wiki/Rope_(data_structure)) stores the
bytes in disjoint chunks. `Buf` and `BufMut` maintain cursors tracking
the current position in the underlying byte storage. When bytes are read
or written, the cursor is advanced.

## <a href="#relation-with-read-and-write" class="doc-anchor">§</a>Relation with `Read` and `Write`

At first glance, it may seem that `Buf` and `BufMut` overlap in
functionality with
[`std::io::Read`](https://doc.rust-lang.org/1.93.1/std/io/trait.Read.html "trait std::io::Read")
and
[`std::io::Write`](https://doc.rust-lang.org/1.93.1/std/io/trait.Write.html "trait std::io::Write").
However, they serve different purposes. A buffer is the value that is
provided as an argument to `Read::read` and `Write::write`. `Read` and
`Write` may then perform a syscall, which has the potential of failing.
Operations on `Buf` and `BufMut` are infallible.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::buf::`<a href="buf/trait.Buf.html" class="trait"
title="trait bytes::buf::Buf"><code>Buf</code></a>`;`

`pub use crate::buf::`<a href="buf/trait.BufMut.html" class="trait"
title="trait bytes::buf::BufMut"><code>BufMut</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="buf/index.html" class="mod" title="mod bytes::buf">buf</a>  
Utilities for working with buffers.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Bytes.html" class="struct"
title="struct bytes::Bytes">Bytes</a>  
A cheaply cloneable and sliceable chunk of contiguous memory.

<a href="struct.BytesMut.html" class="struct"
title="struct bytes::BytesMut">BytesMut</a>  
A unique reference to a contiguous slice of memory.

<a href="struct.TryGetError.html" class="struct"
title="struct bytes::TryGetError">TryGetError</a>  
Error type for the `try_get_` methods of
[`Buf`](buf/trait.Buf.html "trait bytes::buf::Buf"). Indicates that
there were not enough remaining bytes in the buffer while attempting to
get a value from a [`Buf`](buf/trait.Buf.html "trait bytes::buf::Buf")
with one of the `try_get_` methods.
