Crate h2 Copy item path

<span class="sub-heading"><a href="../src/h2/lib.rs.html#1-162" class="src">Source</a>
</span>

Expand description

An asynchronous, HTTP/2 server and client implementation.

This library implements the [HTTP/2](https://http2.github.io/)
specification. The implementation is asynchronous, using
[futures](https://docs.rs/futures/) as the basis for the API. The
implementation is also decoupled from TCP or TLS details. The user must
handle ALPN and HTTP/1.1 upgrades themselves.

# <a href="#getting-started" class="doc-anchor">§</a>Getting started

Add the following to your `Cargo.toml` file:

``` language-toml
[dependencies]
h2 = "0.4"
```

# <a href="#layout-1" class="doc-anchor">§</a>Layout

The crate is split into [`client`](client/index.html) and
[`server`](server/index.html) modules. Types that are common to both
clients and servers are located at the root of the crate.

See module level documentation for more details on how to use `h2`.

# <a href="#handshake" class="doc-anchor">§</a>Handshake

Both the client and the server require a connection to already be in a
state ready to start the HTTP/2 handshake. This library does not provide
facilities to do this.

There are three ways to reach an appropriate state to start the HTTP/2
handshake.

-   Opening an HTTP/1.1 connection and performing an
    [upgrade](https://developer.mozilla.org/en-US/docs/Web/HTTP/Protocol_upgrade_mechanism).
-   Opening a connection with TLS and use ALPN to negotiate the
    protocol.
-   Open a connection with prior knowledge, i.e. both the client and the
    server assume that the connection is immediately ready to start the
    HTTP/2 handshake once opened.

Once the connection is ready to start the HTTP/2 handshake, it can be
passed to [`server::handshake`](server/fn.handshake.html) or
[`client::handshake`](client/fn.handshake.html). At this point, the
library will start the handshake process, which consists of:

-   The client sends the connection preface (a predefined sequence of 24
    octets).
-   Both the client and the server sending a SETTINGS frame.

See the [Starting HTTP/2](http://httpwg.org/specs/rfc7540.html#starting)
in the specification for more details.

# <a href="#flow-control" class="doc-anchor">§</a>Flow control

[Flow control](http://httpwg.org/specs/rfc7540.html#FlowControl) is a
fundamental feature of HTTP/2. The `h2` library exposes flow control to
the user.

An HTTP/2 client or server may not send unlimited data to the peer. When
a stream is initiated, both the client and the server are provided with
an initial window size for that stream. A window size is the number of
bytes the endpoint can send to the peer. At any point in time, the peer
may increase this window size by sending a `WINDOW_UPDATE` frame. Once a
client or server has sent data filling the window for a stream, no
further data may be sent on that stream until the peer increases the
window.

There is also a **connection level** window governing data sent across
all streams.

Managing flow control for inbound data is done through
[`FlowControl`](struct.FlowControl.html). Managing flow control for
outbound data is done through [`SendStream`](struct.SendStream.html).
See the struct level documentation for those two types for more details.

# Modules<a href="#modules" class="anchor">§</a>

<a href="client/index.html" class="mod"
title="mod h2::client">client</a>  
Client implementation of the HTTP/2 protocol.

<a href="ext/index.html" class="mod" title="mod h2::ext">ext</a>  
Extensions specific to the HTTP/2 protocol.

<a href="server/index.html" class="mod"
title="mod h2::server">server</a>  
Server implementation of the HTTP/2 protocol.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct h2::Error">Error</a>  
Represents HTTP/2 operation errors.

<a href="struct.FlowControl.html" class="struct"
title="struct h2::FlowControl">FlowControl</a>  
A handle to release window capacity to a remote stream.

<a href="struct.Ping.html" class="struct"
title="struct h2::Ping">Ping</a>  
Sent via [`PingPong`](struct.PingPong.html) to send a PING frame to a
peer.

<a href="struct.PingPong.html" class="struct"
title="struct h2::PingPong">PingPong</a>  
A handle to send and receive PING frames with the peer.

<a href="struct.Pong.html" class="struct"
title="struct h2::Pong">Pong</a>  
Received via [`PingPong`](struct.PingPong.html) when a peer acknowledges
a [`Ping`](struct.Ping.html).

<a href="struct.Reason.html" class="struct"
title="struct h2::Reason">Reason</a>  
HTTP/2 error codes.

<a href="struct.RecvStream.html" class="struct"
title="struct h2::RecvStream">RecvStream</a>  
Receives the body stream and trailers from the remote peer.

<a href="struct.SendStream.html" class="struct"
title="struct h2::SendStream">SendStream</a>  
Sends the body stream and trailers to the remote peer.

<a href="struct.StreamId.html" class="struct"
title="struct h2::StreamId">StreamId</a>  
A stream identifier, as described in [Section
5.1.1](https://tools.ietf.org/html/rfc7540#section-5.1.1) of RFC 7540.
