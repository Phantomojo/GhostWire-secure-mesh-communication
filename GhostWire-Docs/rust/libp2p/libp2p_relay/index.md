Crate libp2p\_relay Copy item path

<span class="sub-heading"><a href="../src/libp2p_relay/lib.rs.html#22-89" class="src">Source</a>
</span>

Expand description

Implementation of libp2p [circuit
relay](https://github.com/libp2p/specs/blob/master/relay/README.md)
protocol.

# Modules<a href="#modules" class="anchor">§</a>

<a href="client/index.html" class="mod"
title="mod libp2p_relay::client">client</a>  
Everything related to the relay protocol from a client’s perspective.

<a href="inbound/index.html" class="mod"
title="mod libp2p_relay::inbound">inbound</a>  
Types related to the relay protocol inbound.

<a href="outbound/index.html" class="mod"
title="mod libp2p_relay::outbound">outbound</a>  
Types related to the relay protocol outbound.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Behaviour.html" class="struct"
title="struct libp2p_relay::Behaviour">Behaviour</a>  
[`NetworkBehaviour`](../libp2p_swarm/behaviour/trait.NetworkBehaviour.html "trait libp2p_swarm::behaviour::NetworkBehaviour")
implementation of the relay server functionality of the circuit relay v2
protocol.

<a href="struct.CircuitId.html" class="struct"
title="struct libp2p_relay::CircuitId">CircuitId</a>  
<a href="struct.Config.html" class="struct"
title="struct libp2p_relay::Config">Config</a>  
Configuration for the relay
[`Behaviour`](struct.Behaviour.html "struct libp2p_relay::Behaviour").

<a href="struct.RequestId.html" class="struct"
title="struct libp2p_relay::RequestId">RequestId</a>  
The ID of an outgoing / incoming, relay / destination request.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Event.html" class="enum"
title="enum libp2p_relay::Event">Event</a>  
The events produced by the relay `Behaviour`.

<a href="enum.StatusCode.html" class="enum"
title="enum libp2p_relay::StatusCode">StatusCode</a>  
Status code for a relay reservation request that was denied.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.HOP_PROTOCOL_NAME.html" class="constant"
title="constant libp2p_relay::HOP_PROTOCOL_NAME">HOP_PROTOCOL_NAME</a>

<a href="constant.STOP_PROTOCOL_NAME.html" class="constant"
title="constant libp2p_relay::STOP_PROTOCOL_NAME">STOP_PROTOCOL_NAME</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.RateLimiter.html" class="trait"
title="trait libp2p_relay::RateLimiter">RateLimiter</a>  
Allows rate limiting access to some resource based on the
[`PeerId`](../libp2p_identity/peer_id/struct.PeerId.html "struct libp2p_identity::peer_id::PeerId")
and
[`Multiaddr`](../multiaddr/struct.Multiaddr.html "struct multiaddr::Multiaddr")
of a remote peer.
