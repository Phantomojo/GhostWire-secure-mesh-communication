Crate yamux Copy item path

<span class="sub-heading"><a href="../src/yamux/lib.rs.html#11-196" class="src">Source</a>
</span>

Expand description

This crate implements the [Yamux
specification](https://github.com/hashicorp/yamux/blob/master/spec.md).

It multiplexes independent I/O streams over reliable, ordered
connections, such as TCP/IP.

The two primary objects, clients of this crate interact with, are:

-   [`Connection`](struct.Connection.html "struct yamux::Connection"),
    which wraps the underlying I/O resource, e.g. a socket, and provides
    methods for opening outbound or accepting inbound streams.
-   [`Stream`](struct.Stream.html "struct yamux::Stream"), which
    implements
    [`futures::io::AsyncRead`](../futures_io/if_std/trait.AsyncRead.html "trait futures_io::if_std::AsyncRead")
    and
    [`futures::io::AsyncWrite`](../futures_io/if_std/trait.AsyncWrite.html "trait futures_io::if_std::AsyncWrite").

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Config.html" class="struct"
title="struct yamux::Config">Config</a>  
Yamux configuration.

<a href="struct.Connection.html" class="struct"
title="struct yamux::Connection">Connection</a>  
A Yamux connection object.

<a href="struct.Packet.html" class="struct"
title="struct yamux::Packet">Packet</a>  
Byte data produced by the
[`futures::stream::Stream`](../futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream")
impl of [`Stream`](struct.Stream.html "struct yamux::Stream").

<a href="struct.Stream.html" class="struct"
title="struct yamux::Stream">Stream</a>  
A multiplexed Yamux stream.

<a href="struct.StreamId.html" class="struct"
title="struct yamux::StreamId">StreamId</a>  
The ID of a stream.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ConnectionError.html" class="enum"
title="enum yamux::ConnectionError">ConnectionError</a>  
The various error cases a connection may encounter.

<a href="enum.FrameDecodeError.html" class="enum"
title="enum yamux::FrameDecodeError">FrameDecodeError</a>  
Possible errors while decoding a message frame.

<a href="enum.HeaderDecodeError.html" class="enum"
title="enum yamux::HeaderDecodeError">HeaderDecodeError</a>  
Possible errors while decoding a message frame header.

<a href="enum.Mode.html" class="enum" title="enum yamux::Mode">Mode</a>  
How the connection is used.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.DEFAULT_CREDIT.html" class="constant"
title="constant yamux::DEFAULT_CREDIT">DEFAULT_CREDIT</a>

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type yamux::Result">Result</a>
