Crate libp2p\_mdns Copy item path

<span class="sub-heading"><a href="../src/libp2p_mdns/lib.rs.html#21-82" class="src">Source</a>
</span>

Expand description

Implementation of the libp2p-specific
[mDNS](https://github.com/libp2p/specs/blob/master/discovery/mdns.md)
protocol.

mDNS is a protocol defined by [RFC
6762](https://tools.ietf.org/html/rfc6762) that allows querying nodes
that correspond to a certain domain name.

In the context of libp2p, the mDNS protocol is used to discover other
nodes on the local network that support libp2p.

# <a href="#usage" class="doc-anchor">§</a>Usage

This crate provides `TokioMdns` which implements the `NetworkBehaviour`
trait. This struct will automatically discover other libp2p nodes on the
local network.

# Modules<a href="#modules" class="anchor">§</a>

<a href="tokio/index.html" class="mod"
title="mod libp2p_mdns::tokio">tokio</a>  
The type of a
[`Behaviour`](struct.Behaviour.html "struct libp2p_mdns::Behaviour")
using the `tokio` implementation.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Behaviour.html" class="struct"
title="struct libp2p_mdns::Behaviour">Behaviour</a>  
A `NetworkBehaviour` for mDNS. Automatically discovers peers on the
local network and adds them to the topology.

<a href="struct.Config.html" class="struct"
title="struct libp2p_mdns::Config">Config</a>  
Configuration for mDNS.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Event.html" class="enum"
title="enum libp2p_mdns::Event">Event</a>  
Event that can be produced by the `Mdns` behaviour.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.IPV4_MDNS_MULTICAST_ADDRESS.html" class="constant"
title="constant libp2p_mdns::IPV4_MDNS_MULTICAST_ADDRESS">IPV4_MDNS_MULTICAST_ADDRESS</a>

<a href="constant.IPV6_MDNS_MULTICAST_ADDRESS.html" class="constant"
title="constant libp2p_mdns::IPV6_MDNS_MULTICAST_ADDRESS">IPV6_MDNS_MULTICAST_ADDRESS</a>
