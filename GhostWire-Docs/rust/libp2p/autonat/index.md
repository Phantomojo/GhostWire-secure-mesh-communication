[libp2p](../index.html)

Crate autonat Copy item path

<span class="sub-heading"><a href="../../src/libp2p_autonat/lib.rs.html#1-10"
class="src">Source</a> </span>

# Modules<a href="#modules" class="anchor">§</a>

<a href="v1/index.html" class="mod"
title="mod libp2p::autonat::v1">v1</a>  
Implementation of the
[AutoNAT](https://github.com/libp2p/specs/blob/master/autonat/README.md)
protocol.

<a href="v2/index.html" class="mod"
title="mod libp2p::autonat::v2">v2</a>  
The second version of the autonat protocol.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Behaviour.html" class="struct"
title="struct libp2p::autonat::Behaviour">Behaviour</a>  
[`NetworkBehaviour`](../swarm/trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
for AutoNAT.

<a href="struct.Config.html" class="struct"
title="struct libp2p::autonat::Config">Config</a>  
Config for the
[`Behaviour`](struct.Behaviour.html "struct libp2p::autonat::Behaviour").

<a href="struct.ProbeId.html" class="struct"
title="struct libp2p::autonat::ProbeId">ProbeId</a>  
Unique identifier for a probe.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Event.html" class="enum"
title="enum libp2p::autonat::Event">Event</a>

Event produced by
[`Behaviour`](struct.Behaviour.html "struct libp2p::autonat::Behaviour").

<a href="enum.InboundFailure.html" class="enum"
title="enum libp2p::autonat::InboundFailure">InboundFailure</a>

Possible failures occurring in the context of receiving an inbound
request and sending a response.

<a href="enum.InboundProbeError.html" class="enum"
title="enum libp2p::autonat::InboundProbeError">InboundProbeError</a>

Inbound probe failed.

<a href="enum.InboundProbeEvent.html" class="enum"
title="enum libp2p::autonat::InboundProbeEvent">InboundProbeEvent</a>

<a href="enum.NatStatus.html" class="enum"
title="enum libp2p::autonat::NatStatus">NatStatus</a>

Assumed NAT status.

<a href="enum.OutboundFailure.html" class="enum"
title="enum libp2p::autonat::OutboundFailure">OutboundFailure</a>

Possible failures occurring in the context of sending an outbound
request and receiving the response.

<a href="enum.OutboundProbeError.html" class="enum"
title="enum libp2p::autonat::OutboundProbeError">OutboundProbeError</a>

Outbound probe failed or was aborted.

<a href="enum.OutboundProbeEvent.html" class="enum"
title="enum libp2p::autonat::OutboundProbeEvent">OutboundProbeEvent</a>

<a href="enum.ResponseError.html" class="enum"
title="enum libp2p::autonat::ResponseError">ResponseError</a>

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.DEFAULT_PROTOCOL_NAME.html" class="constant"
title="constant libp2p::autonat::DEFAULT_PROTOCOL_NAME">DEFAULT_PROTOCOL_NAME</a>  
The protocol name used for negotiating with multistream-select.
