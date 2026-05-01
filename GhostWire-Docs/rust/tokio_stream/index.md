Crate tokio\_stream Copy item path

<span class="sub-heading"><a href="../src/tokio_stream/lib.rs.html#1-118" class="src">Source</a>
</span>

Expand description

Stream utilities for Tokio.

A `Stream` is an asynchronous sequence of values. It can be thought of
as an asynchronous version of the standard library’s `Iterator` trait.

This crate provides helpers to work with them. For examples of usage and
a more in-depth description of streams you can also refer to the
[streams tutorial](https://tokio.rs/tokio/tutorial/streams) on the tokio
website.

# <a href="#iterating-over-a-stream" class="doc-anchor">§</a>Iterating over a Stream

Due to similarities with the standard library’s `Iterator` trait, some
new users may assume that they can use `for in` syntax to iterate over a
`Stream`, but this is unfortunately not possible. Instead, you can use a
`while let` loop as follows:

``` {.rust .rust-example-rendered}
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream = stream::iter(vec![0, 1, 2]);

    while let Some(value) = stream.next().await {
        println!("Got {}", value);
    }
}
```

# <a href="#returning-a-stream-from-a-function" class="doc-anchor">§</a>Returning a Stream from a function

A common way to stream values from a function is to pass in the sender
half of a channel and use the receiver as the stream. This requires
awaiting both futures to ensure progress is made. Another alternative is
the [async-stream](https://docs.rs/async-stream) crate, which contains
macros that provide a `yield` keyword and allow you to return an
`impl Stream`.

# <a href="#conversion-to-and-from-asyncreadasyncwrite"
class="doc-anchor">§</a>Conversion to and from `AsyncRead`/`AsyncWrite`

It is often desirable to convert a `Stream` into an
[`AsyncRead`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncRead.html),
especially when dealing with plaintext formats streamed over the
network. The opposite conversion from an
[`AsyncRead`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncRead.html)
into a `Stream` is also another commonly required feature. To enable
these conversions,
[`tokio-util`](https://docs.rs/tokio-util/latest/tokio_util/codec/index.html)
provides the
[`StreamReader`](https://docs.rs/tokio-util/latest/tokio_util/io/struct.StreamReader.html)
and
[`ReaderStream`](https://docs.rs/tokio-util/latest/tokio_util/io/struct.ReaderStream.html)
types when the io feature is enabled.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use futures_core::`<a href="../futures_core/stream/trait.Stream.html" class="trait"
title="trait futures_core::stream::Stream"><code>Stream</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="adapters/index.html" class="mod"
title="mod tokio_stream::adapters">adapters</a>  
Adapters for
[`Stream`](../futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream")s
created by methods in
[`StreamExt`](trait.StreamExt.html "trait tokio_stream::StreamExt").

<a href="wrappers/index.html" class="mod"
title="mod tokio_stream::wrappers">wrappers</a>  
Wrappers for Tokio types that implement `Stream`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Elapsed.html" class="struct"
title="struct tokio_stream::Elapsed">Elapsed</a>  
Error returned by `Timeout` and `TimeoutRepeating`.

<a href="struct.Empty.html" class="struct"
title="struct tokio_stream::Empty">Empty</a>  
Stream for the [`empty`](fn.empty.html "fn tokio_stream::empty")
function.

<a href="struct.Iter.html" class="struct"
title="struct tokio_stream::Iter">Iter</a>  
Stream for the [`iter`](fn.iter.html "fn tokio_stream::iter") function.

<a href="struct.Once.html" class="struct"
title="struct tokio_stream::Once">Once</a>  
Stream for the [`once`](fn.once.html "fn tokio_stream::once") function.

<a href="struct.Pending.html" class="struct"
title="struct tokio_stream::Pending">Pending</a>  
Stream for the [`pending`](fn.pending.html "fn tokio_stream::pending")
function.

<a href="struct.StreamMap.html" class="struct"
title="struct tokio_stream::StreamMap">StreamMap</a>  
Combine many streams into one, indexing each source stream with a unique
key.

<a href="struct.StreamNotifyClose.html" class="struct"
title="struct tokio_stream::StreamNotifyClose">StreamNotifyClose</a>  
A `Stream` that wraps the values in an `Option`.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.FromStream.html" class="trait"
title="trait tokio_stream::FromStream">FromStream</a>  
Convert from a
[`Stream`](../futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream").

<a href="trait.StreamExt.html" class="trait"
title="trait tokio_stream::StreamExt">StreamExt</a>  
An extension trait for the
[`Stream`](../futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream")
trait that provides a variety of convenient combinator functions.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.empty.html" class="fn"
title="fn tokio_stream::empty">empty</a>  
Creates a stream that yields nothing.

<a href="fn.iter.html" class="fn" title="fn tokio_stream::iter">iter</a>  
Converts an `Iterator` into a `Stream` which is always ready to yield
the next value.

<a href="fn.once.html" class="fn" title="fn tokio_stream::once">once</a>  
Creates a stream that emits an element exactly once.

<a href="fn.pending.html" class="fn"
title="fn tokio_stream::pending">pending</a>  
Creates a stream that is never ready
