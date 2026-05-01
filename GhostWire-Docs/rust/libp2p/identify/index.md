[libp2p](../index.html)

Crate identify Copy item path

<span class="sub-heading"><a href="../../src/libp2p_identify/lib.rs.html#21-57"
class="src">Source</a> </span>

Expand description

Implementation of the
[Identify](https://github.com/libp2p/specs/blob/master/identify/README.md)
protocol.

This implementation of the protocol periodically exchanges
[`Info`](struct.Info.html "struct libp2p::identify::Info") messages
between the peers on an established connection.

At least one identification request is sent on a newly established
connection, beyond which the behaviour does not keep connections alive.

# <a href="#important-discrepancies" class="doc-anchor">§</a>Important Discrepancies

-   **Using Identify with other protocols** Unlike some other libp2p
    implementations, rust-libp2p does not treat Identify as a core
    protocol. This means that other protocols cannot rely upon the
    existence of Identify, and need to be manually hooked up to Identify
    in order to make use of its capabilities.

# <a href="#usage" class="doc-anchor">§</a>Usage

The
[`Behaviour`](struct.Behaviour.html "struct libp2p::identify::Behaviour")
struct implements a
[`NetworkBehaviour`](../swarm/trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
that negotiates and executes the protocol on every established
connection, emitting
[`Event`](enum.Event.html "enum libp2p::identify::Event")s.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Behaviour.html" class="struct"
title="struct libp2p::identify::Behaviour">Behaviour</a>  
Network behaviour that automatically identifies nodes periodically,
returns information about them, and answers identify queries from other
nodes.

<a href="struct.Config.html" class="struct"
title="struct libp2p::identify::Config">Config</a>  
Configuration for the
[`identify::Behaviour`](struct.Behaviour.html "struct libp2p::identify::Behaviour").

<a href="struct.Info.html" class="struct"
title="struct libp2p::identify::Info">Info</a>  
Identify information of a peer sent in protocol messages.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Event.html" class="enum"
title="enum libp2p::identify::Event">Event</a>

Event emitted by the `Identify` behaviour.

<a href="enum.UpgradeError.html" class="enum"
title="enum libp2p::identify::UpgradeError">UpgradeError</a>

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.PROTOCOL_NAME.html" class="constant"
title="constant libp2p::identify::PROTOCOL_NAME">PROTOCOL_NAME</a>

<a href="constant.PUSH_PROTOCOL_NAME.html" class="constant"
title="constant libp2p::identify::PUSH_PROTOCOL_NAME">PUSH_PROTOCOL_NAME</a>
