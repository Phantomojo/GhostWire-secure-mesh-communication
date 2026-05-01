[libp2p](../index.html)

Crate yamux Copy item path

<span class="sub-heading"><a href="../../src/libp2p_yamux/lib.rs.html#21-467"
class="src">Source</a> </span>

Expand description

Implementation of the
[Yamux](https://github.com/hashicorp/yamux/blob/master/spec.md)
multiplexing protocol for libp2p.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Config.html" class="struct"
title="struct libp2p::yamux::Config">Config</a>  
The yamux configuration.

<a href="struct.Error.html" class="struct"
title="struct libp2p::yamux::Error">Error</a>  
The Yamux
[`StreamMuxer`](../core/trait.StreamMuxer.html "trait libp2p::core::StreamMuxer")
error type.

<a href="struct.Muxer.html" class="struct"
title="struct libp2p::yamux::Muxer">Muxer</a>  
A Yamux connection.

<a href="struct.Stream.html" class="struct"
title="struct libp2p::yamux::Stream">Stream</a>  
A stream produced by the yamux multiplexer.

<a href="struct.WindowUpdateMode.html" class="struct"
title="struct libp2p::yamux::WindowUpdateMode">WindowUpdateMode</a>  
The window update mode determines when window updates are sent to the
remote, giving it new credit to send more data.
