Crate multistream\_select Copy item path

<span class="sub-heading"><a href="../src/multistream_select/lib.rs.html#21-144"
class="src">Source</a> </span>

Expand description

# <a href="#multistream-select-protocol-negotiation"
class="doc-anchor">§</a>Multistream-select Protocol Negotiation

This crate implements the `multistream-select` protocol, which is the
protocol used by libp2p to negotiate which application-layer protocol to
use with the remote on a connection or substream.

> **Note**: This crate is used primarily by core components of *libp2p*
> and it is usually not used directly on its own.

## <a href="#roles" class="doc-anchor">§</a>Roles

Two peers using the multistream-select negotiation protocol on an I/O
stream are distinguished by their role as a *dialer* (or *initiator*) or
as a *listener* (or *responder*). Thereby the dialer plays the active
part, driving the protocol, whereas the listener reacts to the messages
received.

The dialer has two options: it can either pick a protocol from the
complete list of protocols that the listener supports, or it can
directly suggest a protocol. Either way, a selected protocol is sent to
the listener who can either accept (by echoing the same protocol) or
reject (by responding with a message stating “not available”). If a
suggested protocol is not available, the dialer may suggest another
protocol. This process continues until a protocol is agreed upon,
yielding a
[`Negotiated`](struct.Negotiated.html "struct multistream_select::Negotiated")
stream, or the dialer has run out of alternatives.

See
[`dialer_select_proto`](fn.dialer_select_proto.html "fn multistream_select::dialer_select_proto")
and
[`listener_select_proto`](fn.listener_select_proto.html "fn multistream_select::listener_select_proto").

## <a href="#negotiated" class="doc-anchor">§</a>[`Negotiated`](struct.Negotiated.html "struct multistream_select::Negotiated")

A `Negotiated` represents an I/O stream that has settled on a protocol
to use. By default, with
[`Version::V1`](enum.Version.html#variant.V1 "variant multistream_select::Version::V1"),
protocol negotiation is always at least one dedicated round-trip message
exchange, before application data for the negotiated protocol can be
sent by the dialer. There is a variant
[`Version::V1Lazy`](enum.Version.html#variant.V1Lazy "variant multistream_select::Version::V1Lazy")
that permits 0-RTT negotiation if the dialer only supports a single
protocol. In that case, when a dialer settles on a protocol to use, the
[`DialerSelectFuture`](struct.DialerSelectFuture.html "struct multistream_select::DialerSelectFuture")
yields a
[`Negotiated`](struct.Negotiated.html "struct multistream_select::Negotiated")
I/O stream before the negotiation data has been flushed. It is then
expecting confirmation for that protocol as the first messages read from
the stream. This behaviour allows the dialer to immediately send data
relating to the negotiated protocol together with the remaining
negotiation message(s). Note, however, that a dialer that performs
multiple 0-RTT negotiations in sequence for different protocols layered
on top of each other may trigger undesirable behaviour for a listener
not supporting one of the intermediate protocols. See
[`dialer_select_proto`](fn.dialer_select_proto.html "fn multistream_select::dialer_select_proto")
and the documentation of
[`Version::V1Lazy`](enum.Version.html#variant.V1Lazy "variant multistream_select::Version::V1Lazy")
for further details.

## <a href="#examples" class="doc-anchor">§</a>Examples

For a dialer:

``` {.rust .rust-example-rendered}
use async_std::net::TcpStream;
use multistream_select::{dialer_select_proto, Version};
use futures::prelude::*;

async_std::task::block_on(async move {
    let socket = TcpStream::connect("127.0.0.1:10333").await.unwrap();

    let protos = vec!["/echo/1.0.0", "/echo/2.5.0"];
    let (protocol, _io) = dialer_select_proto(socket, protos, Version::V1).await.unwrap();

    println!("Negotiated protocol: {:?}", protocol);
    // You can now use `_io` to communicate with the remote.
});
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DialerSelectFuture.html" class="struct"
title="struct multistream_select::DialerSelectFuture">DialerSelectFuture</a>  
A `Future` returned by
[`dialer_select_proto`](fn.dialer_select_proto.html "fn multistream_select::dialer_select_proto")
which negotiates a protocol iteratively by considering one protocol
after the other.

<a href="struct.ListenerSelectFuture.html" class="struct"
title="struct multistream_select::ListenerSelectFuture">ListenerSelectFuture</a>  
The `Future` returned by
[`listener_select_proto`](fn.listener_select_proto.html "fn multistream_select::listener_select_proto")
that performs a multistream-select protocol negotiation on an underlying
I/O stream.

<a href="struct.Negotiated.html" class="struct"
title="struct multistream_select::Negotiated">Negotiated</a>  
An I/O stream that has settled on an (application-layer) protocol to
use.

<a href="struct.NegotiatedComplete.html" class="struct"
title="struct multistream_select::NegotiatedComplete">NegotiatedComplete</a>  
A `Future` that waits on the completion of protocol negotiation.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.NegotiationError.html" class="enum"
title="enum multistream_select::NegotiationError">NegotiationError</a>  
Error that can happen when negotiating a protocol with the remote.

<a href="enum.ProtocolError.html" class="enum"
title="enum multistream_select::ProtocolError">ProtocolError</a>  
A protocol error.

<a href="enum.Version.html" class="enum"
title="enum multistream_select::Version">Version</a>  
Supported multistream-select versions.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.dialer_select_proto.html" class="fn"
title="fn multistream_select::dialer_select_proto">dialer_select_proto</a>  
Returns a `Future` that negotiates a protocol on the given I/O stream
for a peer acting as the *dialer* (or *initiator*).

<a href="fn.listener_select_proto.html" class="fn"
title="fn multistream_select::listener_select_proto">listener_select_proto</a>  
Returns a `Future` that negotiates a protocol on the given I/O stream
for a peer acting as the *listener* (or *responder*).
