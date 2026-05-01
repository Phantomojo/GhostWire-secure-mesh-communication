[libp2p](../index.html)

Crate request\_response Copy item path

<span class="sub-heading"><a href="../../src/libp2p_request_response/lib.rs.html#21-1059"
class="src">Source</a> </span>

Expand description

Generic request/response protocols.

## <a href="#general-usage" class="doc-anchor">§</a>General Usage

The
[`Behaviour`](struct.Behaviour.html "struct libp2p::request_response::Behaviour")
struct is a
[`NetworkBehaviour`](../swarm/trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
that implements a generic request/response protocol or protocol family,
whereby each request is sent over a new substream on a connection.
`Behaviour` is generic over the actual messages being sent, which are
defined in terms of a
[`Codec`](trait.Codec.html "trait libp2p::request_response::Codec").
Creating a request/response protocol thus amounts to providing an
implementation of this trait which can then be given to
[`Behaviour::with_codec`](struct.Behaviour.html#method.with_codec "associated function libp2p::request_response::Behaviour::with_codec").
Further configuration options are available via the
[`Config`](struct.Config.html "struct libp2p::request_response::Config").

Requests are sent using
[`Behaviour::send_request`](struct.Behaviour.html#method.send_request "method libp2p::request_response::Behaviour::send_request")
and the responses received as
[`Message::Response`](enum.Message.html#variant.Response "variant libp2p::request_response::Message::Response")
via
[`Event::Message`](enum.Event.html#variant.Message "variant libp2p::request_response::Event::Message").

Responses are sent using
[`Behaviour::send_response`](struct.Behaviour.html#method.send_response "method libp2p::request_response::Behaviour::send_response")
upon receiving a
[`Message::Request`](enum.Message.html#variant.Request "variant libp2p::request_response::Message::Request")
via
[`Event::Message`](enum.Event.html#variant.Message "variant libp2p::request_response::Event::Message").

## <a href="#predefined-codecs" class="doc-anchor">§</a>Predefined codecs

In case your message types implement
[`serde::Serialize`](../../serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize")
and
[`serde::Deserialize`](../../serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize"),
you can use two predefined behaviours:

-   [`cbor::Behaviour`](cbor/type.Behaviour.html "type libp2p::request_response::cbor::Behaviour")
    for CBOR-encoded messages
-   \[`json::Behaviour`\] for JSON-encoded messages

## <a href="#protocol-families" class="doc-anchor">§</a>Protocol Families

A single
[`Behaviour`](struct.Behaviour.html "struct libp2p::request_response::Behaviour")
instance can be used with an entire protocol family that share the same
request and response types. For that purpose,
[`Codec::Protocol`](trait.Codec.html#associatedtype.Protocol "associated type libp2p::request_response::Codec::Protocol")
is typically instantiated with a sum type.

## <a href="#limited-protocol-support" class="doc-anchor">§</a>Limited Protocol Support

It is possible to only support inbound or outbound requests for a
particular protocol. This is achieved by instantiating `Behaviour` with
protocols using
[`ProtocolSupport::Inbound`](enum.ProtocolSupport.html#variant.Inbound "variant libp2p::request_response::ProtocolSupport::Inbound")
or
[`ProtocolSupport::Outbound`](enum.ProtocolSupport.html#variant.Outbound "variant libp2p::request_response::ProtocolSupport::Outbound").
Any subset of protocols of a protocol family can be configured in this
way. Such protocols will not be advertised during inbound respectively
outbound protocol negotiation on the substreams.

# Modules<a href="#modules" class="anchor">§</a>

<a href="cbor/index.html" class="mod"
title="mod libp2p::request_response::cbor">cbor</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Behaviour.html" class="struct"
title="struct libp2p::request_response::Behaviour">Behaviour</a>  
A request/response protocol for some message codec.

<a href="struct.Config.html" class="struct"
title="struct libp2p::request_response::Config">Config</a>  
The configuration for a `Behaviour` protocol.

<a href="struct.InboundRequestId.html" class="struct"
title="struct libp2p::request_response::InboundRequestId">InboundRequestId</a>  
The ID of an inbound request.

<a href="struct.OutboundRequestId.html" class="struct"
title="struct libp2p::request_response::OutboundRequestId">OutboundRequestId</a>  
The ID of an outbound request.

<a href="struct.ResponseChannel.html" class="struct"
title="struct libp2p::request_response::ResponseChannel">ResponseChannel</a>  
A channel for sending a response to an inbound request.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Event.html" class="enum"
title="enum libp2p::request_response::Event">Event</a>  
The events emitted by a request-response
[`Behaviour`](struct.Behaviour.html "struct libp2p::request_response::Behaviour").

<a href="enum.InboundFailure.html" class="enum"
title="enum libp2p::request_response::InboundFailure">InboundFailure</a>  
Possible failures occurring in the context of receiving an inbound
request and sending a response.

<a href="enum.Message.html" class="enum"
title="enum libp2p::request_response::Message">Message</a>  
An inbound request or response.

<a href="enum.OutboundFailure.html" class="enum"
title="enum libp2p::request_response::OutboundFailure">OutboundFailure</a>  
Possible failures occurring in the context of sending an outbound
request and receiving the response.

<a href="enum.ProtocolSupport.html" class="enum"
title="enum libp2p::request_response::ProtocolSupport">ProtocolSupport</a>  
The level of support for a particular protocol.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Codec.html" class="trait"
title="trait libp2p::request_response::Codec">Codec</a>  
A `Codec` defines the request and response types for a request-response
[`Behaviour`](struct.Behaviour.html "struct libp2p::request_response::Behaviour")
protocol or protocol family and how they are encoded / decoded on an I/O
stream.
