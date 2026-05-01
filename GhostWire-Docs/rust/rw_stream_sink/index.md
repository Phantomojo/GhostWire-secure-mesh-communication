Crate rw\_stream\_sink Copy item path

<span class="sub-heading"><a href="../src/rw_stream_sink/lib.rs.html#21-219"
class="src">Source</a> </span>

Expand description

This crate provides the
[`RwStreamSink`](struct.RwStreamSink.html "struct rw_stream_sink::RwStreamSink")
type. It wraps around a
[`Stream`](../futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream")
and [`Sink`](../futures_sink/trait.Sink.html "trait futures_sink::Sink")
that produces and accepts byte arrays, and implements
[`AsyncRead`](../futures_io/if_std/trait.AsyncRead.html "trait futures_io::if_std::AsyncRead")
and
[`AsyncWrite`](../futures_io/if_std/trait.AsyncWrite.html "trait futures_io::if_std::AsyncWrite").

Each call to
[`AsyncWrite::poll_write`](../futures_io/if_std/trait.AsyncWrite.html#tymethod.poll_write "method futures_io::if_std::AsyncWrite::poll_write")
will send one packet to the sink. Calls to
[`AsyncRead::poll_read`](../futures_io/if_std/trait.AsyncRead.html#tymethod.poll_read "method futures_io::if_std::AsyncRead::poll_read")
will read from the stream’s incoming packets.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.RwStreamSink.html" class="struct"
title="struct rw_stream_sink::RwStreamSink">RwStreamSink</a>  
Wraps a
[`Stream`](../futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream")
and [`Sink`](../futures_sink/trait.Sink.html "trait futures_sink::Sink")
whose items are buffers. Implements
[`AsyncRead`](../futures_io/if_std/trait.AsyncRead.html "trait futures_io::if_std::AsyncRead")
and
[`AsyncWrite`](../futures_io/if_std/trait.AsyncWrite.html "trait futures_io::if_std::AsyncWrite").
