Crate futures\_util Copy item path

<span class="sub-heading"><a href="../src/futures_util/lib.rs.html#1-332" class="src">Source</a>
</span>

Expand description

Combinators and utilities for working with `Future`s, `Stream`s,
`Sink`s, and the `AsyncRead` and `AsyncWrite` traits.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::future::`<a
href="https://doc.rust-lang.org/1.93.1/core/future/future/trait.Future.html"
class="trait"
title="trait core::future::future::Future"><code>Future</code></a>`;`

`pub use crate::future::`<a href="future/trait.FutureExt.html" class="trait"
title="trait futures_util::future::FutureExt"><code>FutureExt</code></a>`;`

`pub use crate::future::`<a href="future/trait.TryFuture.html" class="trait"
title="trait futures_util::future::TryFuture"><code>TryFuture</code></a>`;`

`pub use crate::future::`<a href="future/trait.TryFutureExt.html" class="trait"
title="trait futures_util::future::TryFutureExt"><code>TryFutureExt</code></a>`;`

`pub use crate::stream::`<a href="stream/trait.Stream.html" class="trait"
title="trait futures_util::stream::Stream"><code>Stream</code></a>`;`

`pub use crate::stream::`<a href="stream/trait.StreamExt.html" class="trait"
title="trait futures_util::stream::StreamExt"><code>StreamExt</code></a>`;`

`pub use crate::stream::`<a href="stream/trait.TryStream.html" class="trait"
title="trait futures_util::stream::TryStream"><code>TryStream</code></a>`;`

`pub use crate::stream::`<a href="stream/trait.TryStreamExt.html" class="trait"
title="trait futures_util::stream::TryStreamExt"><code>TryStreamExt</code></a>`;`

`pub use crate::sink::`<a href="sink/trait.Sink.html" class="trait"
title="trait futures_util::sink::Sink"><code>Sink</code></a>`;`

`pub use crate::sink::`<a href="sink/trait.SinkExt.html" class="trait"
title="trait futures_util::sink::SinkExt"><code>SinkExt</code></a>`;`

`pub use crate::io::`<a href="io/trait.AsyncBufRead.html" class="trait"
title="trait futures_util::io::AsyncBufRead"><code>AsyncBufRead</code></a>`;`

`pub use crate::io::`<a href="io/trait.AsyncBufReadExt.html" class="trait"
title="trait futures_util::io::AsyncBufReadExt"><code>AsyncBufReadExt</code></a>`;`

`pub use crate::io::`<a href="io/trait.AsyncRead.html" class="trait"
title="trait futures_util::io::AsyncRead"><code>AsyncRead</code></a>`;`

`pub use crate::io::`<a href="io/trait.AsyncReadExt.html" class="trait"
title="trait futures_util::io::AsyncReadExt"><code>AsyncReadExt</code></a>`;`

`pub use crate::io::`<a href="io/trait.AsyncSeek.html" class="trait"
title="trait futures_util::io::AsyncSeek"><code>AsyncSeek</code></a>`;`

`pub use crate::io::`<a href="io/trait.AsyncSeekExt.html" class="trait"
title="trait futures_util::io::AsyncSeekExt"><code>AsyncSeekExt</code></a>`;`

`pub use crate::io::`<a href="io/trait.AsyncWrite.html" class="trait"
title="trait futures_util::io::AsyncWrite"><code>AsyncWrite</code></a>`;`

`pub use crate::io::`<a href="io/trait.AsyncWriteExt.html" class="trait"
title="trait futures_util::io::AsyncWriteExt"><code>AsyncWriteExt</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="future/index.html" class="mod"
title="mod futures_util::future">future</a>  
Asynchronous values.

<a href="io/index.html" class="mod" title="mod futures_util::io">io</a>  
Asynchronous I/O.

<a href="lock/index.html" class="mod"
title="mod futures_util::lock">lock</a>  
Futures-powered synchronization primitives.

<a href="never/index.html" class="mod"
title="mod futures_util::never">never</a>  
This module contains the `Never` type.

<a href="sink/index.html" class="mod"
title="mod futures_util::sink">sink</a>  
Asynchronous sinks.

<a href="stream/index.html" class="mod"
title="mod futures_util::stream">stream</a>  
Asynchronous streams.

<a href="task/index.html" class="mod"
title="mod futures_util::task">task</a>  
Tools for working with tasks.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.join.html" class="macro"
title="macro futures_util::join">join</a>  
Polls multiple futures simultaneously, returning a tuple of all results
once complete.

<a href="macro.pending.html" class="macro"
title="macro futures_util::pending">pending</a>  
A macro which yields to the event loop once.

<a href="macro.pin_mut.html" class="macro"
title="macro futures_util::pin_mut">pin_mut</a>  
Pins a value on the stack.

<a href="macro.poll.html" class="macro"
title="macro futures_util::poll">poll</a>  
A macro which returns the result of polling a future once within the
current `async` context.

<a href="macro.ready.html" class="macro"
title="macro futures_util::ready">ready</a>  
Extracts the successful type of a `Poll<T>`.

<a href="macro.select.html" class="macro"
title="macro futures_util::select">select</a>  
Polls multiple futures and streams simultaneously, executing the branch
for the future that finishes first. If multiple futures are ready, one
will be pseudo-randomly selected at runtime. Futures directly passed to
`select!` must be `Unpin` and implement `FusedFuture`.

<a href="macro.select_biased.html" class="macro"
title="macro futures_util::select_biased">select_biased</a>  
Polls multiple futures and streams simultaneously, executing the branch
for the future that finishes first. Unlike
[`select!`](macro.select.html), if multiple futures are ready, one will
be selected in order of declaration. Futures directly passed to
`select_biased!` must be `Unpin` and implement `FusedFuture`.

<a href="macro.stream_select.html" class="macro"
title="macro futures_util::stream_select">stream_select</a>  
Combines several streams, all producing the same `Item` type, into one
stream. This is similar to `select_all` but does not require the streams
to all be the same type. It also keeps the streams inline, and does not
require `Box<dyn Stream>`s to be allocated. Streams passed to this macro
must be `Unpin`.

<a href="macro.try_join.html" class="macro"
title="macro futures_util::try_join">try_join</a>  
Polls multiple futures simultaneously, resolving to a
[`Result`](https://doc.rust-lang.org/1.93.1/core/result/enum.Result.html "enum core::result::Result")
containing either a tuple of the successful outputs or an error.
