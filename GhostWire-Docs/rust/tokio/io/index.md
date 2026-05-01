[tokio](../index.html)

Module io Copy item path

<span class="sub-heading"><a href="../../src/tokio/io/mod.rs.html#1-308" class="src">Source</a>
</span>

Expand description

Traits, helpers, and type definitions for asynchronous I/O
functionality.

This module is the asynchronous version of `std::io`. Primarily, it
defines two traits,
[`AsyncRead`](trait.AsyncRead.html "trait tokio::io::AsyncRead") and
[`AsyncWrite`](trait.AsyncWrite.html "trait tokio::io::AsyncWrite"),
which are asynchronous versions of the
[`Read`](https://doc.rust-lang.org/1.93.1/std/io/trait.Read.html "trait std::io::Read")
and
[`Write`](https://doc.rust-lang.org/1.93.1/std/io/trait.Write.html "trait std::io::Write")
traits in the standard library.

# <a href="#asyncread-and-asyncwrite" class="doc-anchor">§</a>`AsyncRead` and `AsyncWrite`

Like the standard library’s
[`Read`](https://doc.rust-lang.org/1.93.1/std/io/trait.Read.html "trait std::io::Read")
and
[`Write`](https://doc.rust-lang.org/1.93.1/std/io/trait.Write.html "trait std::io::Write")
traits, [`AsyncRead`](trait.AsyncRead.html "trait tokio::io::AsyncRead")
and [`AsyncWrite`](trait.AsyncWrite.html "trait tokio::io::AsyncWrite")
provide the most general interface for reading and writing input and
output. Unlike the standard library’s traits, however, they are
*asynchronous* — meaning that reading from or writing to a `tokio::io`
type will *yield* to the Tokio scheduler when IO is not ready, rather
than blocking. This allows other tasks to run while waiting on IO.

Another difference is that `AsyncRead` and `AsyncWrite` only contain
core methods needed to provide asynchronous reading and writing
functionality. Instead, utility methods are defined in the
[`AsyncReadExt`](trait.AsyncReadExt.html "trait tokio::io::AsyncReadExt")
and
[`AsyncWriteExt`](trait.AsyncWriteExt.html "trait tokio::io::AsyncWriteExt")
extension traits. These traits are automatically implemented for all
values that implement `AsyncRead` and `AsyncWrite` respectively.

End users will rarely interact directly with `AsyncRead` and
`AsyncWrite`. Instead, they will use the async functions defined in the
extension traits. Library authors are expected to implement `AsyncRead`
and `AsyncWrite` in order to provide types that behave like byte
streams.

Even with these differences, Tokio’s `AsyncRead` and `AsyncWrite` traits
can be used in almost exactly the same manner as the standard library’s
`Read` and `Write`. Most types in the standard library that implement
`Read` and `Write` have asynchronous equivalents in `tokio` that
implement `AsyncRead` and `AsyncWrite`, such as
[`File`](../fs/struct.File.html "struct tokio::fs::File") and
[`TcpStream`](../net/struct.TcpStream.html "struct tokio::net::TcpStream").

For example, the standard library documentation introduces `Read` by
[demonstrating](https://doc.rust-lang.org/1.93.1/std/io/index.html#read-and-write "mod std::io")
reading some bytes from a
[`std::fs::File`](https://doc.rust-lang.org/1.93.1/std/fs/struct.File.html "struct std::fs::File").
We can do the same with
[`tokio::fs::File`](../fs/struct.File.html "struct tokio::fs::File"):

``` {.rust .rust-example-rendered}
use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
```

## <a href="#buffered-readers-and-writers" class="doc-anchor">§</a>Buffered Readers and Writers

Byte-based interfaces are unwieldy and can be inefficient, as we’d need
to be making near-constant calls to the operating system. To help with
this, `std::io` comes with [support for *buffered* readers and
writers](https://doc.rust-lang.org/1.93.1/std/io/index.html#bufreader-and-bufwriter "mod std::io"),
and therefore, `tokio::io` does as well.

Tokio provides an async version of the
[`std::io::BufRead`](https://doc.rust-lang.org/1.93.1/std/io/trait.BufRead.html "trait std::io::BufRead")
trait,
[`AsyncBufRead`](trait.AsyncBufRead.html "trait tokio::io::AsyncBufRead");
and async
[`BufReader`](struct.BufReader.html "struct tokio::io::BufReader") and
[`BufWriter`](struct.BufWriter.html "struct tokio::io::BufWriter")
structs, which wrap readers and writers. These wrappers use a buffer,
reducing the number of calls and providing nicer methods for accessing
exactly what you want.

For example,
[`BufReader`](struct.BufReader.html "struct tokio::io::BufReader") works
with the
[`AsyncBufRead`](trait.AsyncBufRead.html "trait tokio::io::AsyncBufRead")
trait to add extra methods to any async reader:

``` {.rust .rust-example-rendered}
use tokio::io::{self, BufReader, AsyncBufReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let f = File::open("foo.txt").await?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    // read a line into buffer
    reader.read_line(&mut buffer).await?;

    println!("{}", buffer);
    Ok(())
}
```

[`BufWriter`](struct.BufWriter.html "struct tokio::io::BufWriter")
doesn’t add any new ways of writing; it just buffers every call to
[`write`](trait.AsyncWriteExt.html#method.write "method tokio::io::AsyncWriteExt::write").
However, you **must** flush
[`BufWriter`](struct.BufWriter.html "struct tokio::io::BufWriter") to
ensure that any buffered data is written.

``` {.rust .rust-example-rendered}
use tokio::io::{self, BufWriter, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let f = File::create("foo.txt").await?;
    {
        let mut writer = BufWriter::new(f);

        // Write a byte to the buffer.
        writer.write(&[42u8]).await?;

        // Flush the buffer before it goes out of scope.
        writer.flush().await?;

    } // Unless flushed or shut down, the contents of the buffer is discarded on drop.

    Ok(())
}
```

## <a href="#implementing-asyncread-and-asyncwrite"
class="doc-anchor">§</a>Implementing `AsyncRead` and `AsyncWrite`

Because they are traits, we can implement
[`AsyncRead`](trait.AsyncRead.html "trait tokio::io::AsyncRead") and
[`AsyncWrite`](trait.AsyncWrite.html "trait tokio::io::AsyncWrite") for
our own types, as well. Note that these traits must only be implemented
for non-blocking I/O types that integrate with the futures type system.
In other words, these types must never block the thread, and instead the
current task is notified when the I/O resource is ready.

## <a href="#conversion-to-and-from-streamsink" class="doc-anchor">§</a>Conversion to and from Stream/Sink

It is often convenient to encapsulate the reading and writing of bytes
in a
[`Stream`](https://docs.rs/futures/0.3/futures/stream/trait.Stream.html)
or [`Sink`](https://docs.rs/futures/0.3/futures/sink/trait.Sink.html) of
data.

Tokio provides simple wrappers for converting
[`AsyncRead`](trait.AsyncRead.html "trait tokio::io::AsyncRead") to
[`Stream`](https://docs.rs/futures/0.3/futures/stream/trait.Stream.html)
and vice-versa in the [tokio-util](https://docs.rs/tokio-util) crate,
see
[`ReaderStream`](https://docs.rs/tokio-util/latest/tokio_util/io/struct.ReaderStream.html)
and
[`StreamReader`](https://docs.rs/tokio-util/latest/tokio_util/io/struct.StreamReader.html).

There are also utility traits that abstract the asynchronous buffering
necessary to write your own adaptors for encoding and decoding bytes
to/from your structured data, allowing to transform something that
implements
[`AsyncRead`](trait.AsyncRead.html "trait tokio::io::AsyncRead")/[`AsyncWrite`](trait.AsyncWrite.html "trait tokio::io::AsyncWrite")
into a
[`Stream`](https://docs.rs/futures/0.3/futures/stream/trait.Stream.html)/[`Sink`](https://docs.rs/futures/0.3/futures/sink/trait.Sink.html),
see
[`Decoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Decoder.html)
and
[`Encoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Encoder.html)
in the
[tokio-util::codec](https://docs.rs/tokio-util/latest/tokio_util/codec/index.html)
module.

# <a href="#standard-input-and-output" class="doc-anchor">§</a>Standard input and output

Tokio provides asynchronous APIs to standard
[input](fn.stdin.html "fn tokio::io::stdin"),
[output](fn.stdout.html "fn tokio::io::stdout"), and
[error](fn.stderr.html "fn tokio::io::stderr"). These APIs are very
similar to the ones provided by `std`, but they also implement
[`AsyncRead`](trait.AsyncRead.html "trait tokio::io::AsyncRead") and
[`AsyncWrite`](trait.AsyncWrite.html "trait tokio::io::AsyncWrite").

Note that the standard input / output APIs **must** be used from the
context of the Tokio runtime, as they require Tokio-specific features to
function. Calling these functions outside of a Tokio runtime will panic.

# <a href="#std-re-exports" class="doc-anchor">§</a>`std` re-exports

Additionally,
[`Error`](https://doc.rust-lang.org/1.93.1/std/io/error/struct.Error.html "struct std::io::error::Error"),
[`ErrorKind`](https://doc.rust-lang.org/1.93.1/std/io/error/enum.ErrorKind.html "enum std::io::error::ErrorKind"),
[`Result`](https://doc.rust-lang.org/1.93.1/std/io/error/type.Result.html "type std::io::error::Result"),
and
[`SeekFrom`](https://doc.rust-lang.org/1.93.1/std/io/enum.SeekFrom.html "enum std::io::SeekFrom")
are re-exported from `std::io` for ease of use.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use std::io::`<a
href="https://doc.rust-lang.org/1.93.1/std/io/error/struct.Error.html"
class="struct"
title="struct std::io::error::Error"><code>Error</code></a>`;`

`pub use std::io::`<a
href="https://doc.rust-lang.org/1.93.1/std/io/error/enum.ErrorKind.html"
class="enum"
title="enum std::io::error::ErrorKind"><code>ErrorKind</code></a>`;`

`pub use std::io::`<a href="https://doc.rust-lang.org/1.93.1/std/io/error/type.Result.html"
class="type" title="type std::io::error::Result"><code>Result</code></a>`;`

`pub use std::io::`<a href="https://doc.rust-lang.org/1.93.1/std/io/enum.SeekFrom.html"
class="enum" title="enum std::io::SeekFrom"><code>SeekFrom</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="unix/index.html" class="mod"
title="mod tokio::io::unix">unix</a>  
Asynchronous IO structures specific to Unix-like operating systems.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BufReader.html" class="struct"
title="struct tokio::io::BufReader">BufReader</a>  
The `BufReader` struct adds buffering to any reader.

<a href="struct.BufStream.html" class="struct"
title="struct tokio::io::BufStream">BufStream</a>  
Wraps a type that is
[`AsyncWrite`](trait.AsyncWrite.html "trait tokio::io::AsyncWrite") and
[`AsyncRead`](trait.AsyncRead.html "trait tokio::io::AsyncRead"), and
buffers its input and output.

<a href="struct.BufWriter.html" class="struct"
title="struct tokio::io::BufWriter">BufWriter</a>  
Wraps a writer and buffers its output.

<a href="struct.Chain.html" class="struct"
title="struct tokio::io::Chain">Chain</a>  
Stream for the
[`chain`](trait.AsyncReadExt.html#method.chain "method tokio::io::AsyncReadExt::chain")
method.

<a href="struct.DuplexStream.html" class="struct"
title="struct tokio::io::DuplexStream">DuplexStream</a>  
A bidirectional pipe to read and write bytes in memory.

<a href="struct.Empty.html" class="struct"
title="struct tokio::io::Empty">Empty</a>  
`Empty` ignores any data written via
[`AsyncWrite`](trait.AsyncWrite.html "trait tokio::io::AsyncWrite"), and
will always be empty (returning zero bytes) when read via
[`AsyncRead`](trait.AsyncRead.html "trait tokio::io::AsyncRead").

<a href="struct.Interest.html" class="struct"
title="struct tokio::io::Interest">Interest</a>  
Readiness event interest.

<a href="struct.Join.html" class="struct"
title="struct tokio::io::Join">Join</a>  
Joins two values implementing `AsyncRead` and `AsyncWrite` into a single
handle.

<a href="struct.Lines.html" class="struct"
title="struct tokio::io::Lines">Lines</a>  
Reads lines from an
[`AsyncBufRead`](trait.AsyncBufRead.html "trait tokio::io::AsyncBufRead").

<a href="struct.ReadBuf.html" class="struct"
title="struct tokio::io::ReadBuf">ReadBuf</a>  
A wrapper around a byte buffer that is incrementally filled and
initialized.

<a href="struct.ReadHalf.html" class="struct"
title="struct tokio::io::ReadHalf">ReadHalf</a>  
The readable half of a value returned from
[`split`](fn.split.html "fn tokio::io::split").

<a href="struct.Ready.html" class="struct"
title="struct tokio::io::Ready">Ready</a>  
Describes the readiness state of an I/O resources.

<a href="struct.Repeat.html" class="struct"
title="struct tokio::io::Repeat">Repeat</a>  
An async reader which yields one byte over and over and over and over
and over and…

<a href="struct.SimplexStream.html" class="struct"
title="struct tokio::io::SimplexStream">SimplexStream</a>  
A unidirectional pipe to read and write bytes in memory.

<a href="struct.Sink.html" class="struct"
title="struct tokio::io::Sink">Sink</a>  
An async writer which will move data into the void.

<a href="struct.Split.html" class="struct"
title="struct tokio::io::Split">Split</a>  
Splitter for the
[`split`](trait.AsyncBufReadExt.html#method.split "method tokio::io::AsyncBufReadExt::split")
method.

<a href="struct.Stderr.html" class="struct"
title="struct tokio::io::Stderr">Stderr</a>  
A handle to the standard error stream of a process.

<a href="struct.Stdin.html" class="struct"
title="struct tokio::io::Stdin">Stdin</a>  
A handle to the standard input stream of a process.

<a href="struct.Stdout.html" class="struct"
title="struct tokio::io::Stdout">Stdout</a>  
A handle to the standard output stream of a process.

<a href="struct.Take.html" class="struct"
title="struct tokio::io::Take">Take</a>  
Stream for the
[`take`](trait.AsyncReadExt.html#method.take "method tokio::io::AsyncReadExt::take")
method.

<a href="struct.WriteHalf.html" class="struct"
title="struct tokio::io::WriteHalf">WriteHalf</a>  
The writable half of a value returned from
[`split`](fn.split.html "fn tokio::io::split").

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsyncBufRead.html" class="trait"
title="trait tokio::io::AsyncBufRead">AsyncBufRead</a>  
Reads bytes asynchronously.

<a href="trait.AsyncBufReadExt.html" class="trait"
title="trait tokio::io::AsyncBufReadExt">AsyncBufReadExt</a>  
An extension trait which adds utility methods to
[`AsyncBufRead`](trait.AsyncBufRead.html "trait tokio::io::AsyncBufRead")
types.

<a href="trait.AsyncRead.html" class="trait"
title="trait tokio::io::AsyncRead">AsyncRead</a>  
Reads bytes from a source.

<a href="trait.AsyncReadExt.html" class="trait"
title="trait tokio::io::AsyncReadExt">AsyncReadExt</a>  
Reads bytes from a source.

<a href="trait.AsyncSeek.html" class="trait"
title="trait tokio::io::AsyncSeek">AsyncSeek</a>  
Seek bytes asynchronously.

<a href="trait.AsyncSeekExt.html" class="trait"
title="trait tokio::io::AsyncSeekExt">AsyncSeekExt</a>  
An extension trait that adds utility methods to
[`AsyncSeek`](trait.AsyncSeek.html "trait tokio::io::AsyncSeek") types.

<a href="trait.AsyncWrite.html" class="trait"
title="trait tokio::io::AsyncWrite">AsyncWrite</a>  
Writes bytes asynchronously.

<a href="trait.AsyncWriteExt.html" class="trait"
title="trait tokio::io::AsyncWriteExt">AsyncWriteExt</a>  
Writes bytes to a sink.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.copy.html" class="fn" title="fn tokio::io::copy">copy</a>  
Asynchronously copies the entire contents of a reader into a writer.

<a href="fn.copy_bidirectional.html" class="fn"
title="fn tokio::io::copy_bidirectional">copy_bidirectional</a>  
Copies data in both directions between `a` and `b`.

<a href="fn.copy_bidirectional_with_sizes.html" class="fn"
title="fn tokio::io::copy_bidirectional_with_sizes">copy_bidirectional_with_sizes</a>  
Copies data in both directions between `a` and `b` using buffers of the
specified size.

<a href="fn.copy_buf.html" class="fn"
title="fn tokio::io::copy_buf">copy_buf</a>  
Asynchronously copies the entire contents of a reader into a writer.

<a href="fn.duplex.html" class="fn"
title="fn tokio::io::duplex">duplex</a>  
Create a new pair of `DuplexStream`s that act like a pair of connected
sockets.

<a href="fn.empty.html" class="fn" title="fn tokio::io::empty">empty</a>  
Creates a value that is always at EOF for reads, and ignores all data
written.

<a href="fn.join.html" class="fn" title="fn tokio::io::join">join</a>  
Join two values implementing `AsyncRead` and `AsyncWrite` into a single
handle.

<a href="fn.repeat.html" class="fn"
title="fn tokio::io::repeat">repeat</a>  
Creates an instance of an async reader that infinitely repeats one byte.

<a href="fn.simplex.html" class="fn"
title="fn tokio::io::simplex">simplex</a>  
Creates unidirectional buffer that acts like in memory pipe.

<a href="fn.sink.html" class="fn" title="fn tokio::io::sink">sink</a>  
Creates an instance of an async writer which will successfully consume
all data.

<a href="fn.split.html" class="fn" title="fn tokio::io::split">split</a>  
Splits a single value implementing `AsyncRead + AsyncWrite` into
separate `AsyncRead` and `AsyncWrite` handles.

<a href="fn.stderr.html" class="fn"
title="fn tokio::io::stderr">stderr</a>  
Constructs a new handle to the standard error of the current process.

<a href="fn.stdin.html" class="fn" title="fn tokio::io::stdin">stdin</a>  
Constructs a new handle to the standard input of the current process.

<a href="fn.stdout.html" class="fn"
title="fn tokio::io::stdout">stdout</a>  
Constructs a new handle to the standard output of the current process.
