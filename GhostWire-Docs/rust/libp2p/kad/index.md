[libp2p](../index.html)

Crate kad Copy item path

<span class="sub-heading"><a href="../../src/libp2p_kad/lib.rs.html#21-107" class="src">Source</a>
</span>

Expand description

Implementation of the libp2p-specific
[Kademlia](https://github.com/libp2p/specs/blob/master/kad-dht/README.md)
protocol.

# <a href="#important-discrepancies" class="doc-anchor">§</a>Important Discrepancies

-   **Peer Discovery with Identify** In other libp2p implementations,
    the [Identify](https://github.com/libp2p/specs/tree/master/identify)
    protocol might be seen as a core protocol. Rust-libp2p tries to stay
    as generic as possible, and does not make this assumption. This
    means that the Identify protocol must be manually hooked up to
    Kademlia through calls to
    [`Behaviour::add_address`](struct.Behaviour.html#method.add_address "method libp2p::kad::Behaviour::add_address").
    If you choose not to use the Identify protocol, and do not provide
    an alternative peer discovery mechanism, a Kademlia node will not
    discover nodes beyond the network’s [boot
    nodes](https://docs.libp2p.io/concepts/glossary/#boot-node). Without
    the Identify protocol, existing nodes in the kademlia network cannot
    obtain the listen addresses of nodes querying them, and thus will
    not be able to add them to their routing table.

# Modules<a href="#modules" class="anchor">§</a>

<a href="store/index.html" class="mod"
title="mod libp2p::kad::store">store</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AddProviderOk.html" class="struct"
title="struct libp2p::kad::AddProviderOk">AddProviderOk</a>  
The successful result of publishing a provider record.

<a href="struct.Addresses.html" class="struct"
title="struct libp2p::kad::Addresses">Addresses</a>  
A non-empty list of (unique) addresses of a peer in the routing table.
Every address must be a fully-qualified /p2p address.

<a href="struct.Behaviour.html" class="struct"
title="struct libp2p::kad::Behaviour">Behaviour</a>  
`Behaviour` is a `NetworkBehaviour` that implements the libp2p Kademlia
protocol.

<a href="struct.BootstrapOk.html" class="struct"
title="struct libp2p::kad::BootstrapOk">BootstrapOk</a>  
The successful result of
[`Behaviour::bootstrap`](struct.Behaviour.html#method.bootstrap "method libp2p::kad::Behaviour::bootstrap").

<a href="struct.Config.html" class="struct"
title="struct libp2p::kad::Config">Config</a>  
The configuration for the `Kademlia` behaviour.

<a href="struct.EntryView.html" class="struct"
title="struct libp2p::kad::EntryView">EntryView</a>  
A cloned, immutable view of an entry that is either present in a bucket
or pending insertion.

<a href="struct.GetClosestPeersOk.html" class="struct"
title="struct libp2p::kad::GetClosestPeersOk">GetClosestPeersOk</a>  
The successful result of
[`Behaviour::get_closest_peers`](struct.Behaviour.html#method.get_closest_peers "method libp2p::kad::Behaviour::get_closest_peers").

<a href="struct.KBucketDistance.html" class="struct"
title="struct libp2p::kad::KBucketDistance">KBucketDistance</a>  
A distance between two keys in the DHT keyspace.

<a href="struct.KBucketKey.html" class="struct"
title="struct libp2p::kad::KBucketKey">KBucketKey</a>  
A `Key` in the DHT keyspace with preserved preimage.

<a href="struct.KBucketRef.html" class="struct"
title="struct libp2p::kad::KBucketRef">KBucketRef</a>  
A reference to a bucket.

<a href="struct.KadPeer.html" class="struct"
title="struct libp2p::kad::KadPeer">KadPeer</a>  
Information about a peer, as known by the sender.

<a href="struct.NoKnownPeers.html" class="struct"
title="struct libp2p::kad::NoKnownPeers">NoKnownPeers</a>  
An operation failed to due no known peers in the routing table.

<a href="struct.PeerInfo.html" class="struct"
title="struct libp2p::kad::PeerInfo">PeerInfo</a>  
Peer Info combines a Peer ID with a set of multiaddrs that the peer is
listening on.

<a href="struct.PeerRecord.html" class="struct"
title="struct libp2p::kad::PeerRecord">PeerRecord</a>  
A record either received by the given peer or retrieved from the local
record store.

<a href="struct.ProgressStep.html" class="struct"
title="struct libp2p::kad::ProgressStep">ProgressStep</a>  
Information about progress events.

<a href="struct.ProviderRecord.html" class="struct"
title="struct libp2p::kad::ProviderRecord">ProviderRecord</a>  
A record stored in the DHT whose value is the ID of a peer who can
provide the value on-demand.

<a href="struct.PutRecordOk.html" class="struct"
title="struct libp2p::kad::PutRecordOk">PutRecordOk</a>  
The successful result of
[`Behaviour::put_record`](struct.Behaviour.html#method.put_record "method libp2p::kad::Behaviour::put_record").

<a href="struct.QueryId.html" class="struct"
title="struct libp2p::kad::QueryId">QueryId</a>  
Unique identifier for an active query.

<a href="struct.QueryMut.html" class="struct"
title="struct libp2p::kad::QueryMut">QueryMut</a>  
A mutable reference to a running query.

<a href="struct.QueryRef.html" class="struct"
title="struct libp2p::kad::QueryRef">QueryRef</a>  
An immutable reference to a running query.

<a href="struct.QueryStats.html" class="struct"
title="struct libp2p::kad::QueryStats">QueryStats</a>  
Execution statistics of a query.

<a href="struct.Record.html" class="struct"
title="struct libp2p::kad::Record">Record</a>  
A record stored in the DHT.

<a href="struct.RecordKey.html" class="struct"
title="struct libp2p::kad::RecordKey">RecordKey</a>  
The (opaque) key of a record.

<a href="struct.U256.html" class="struct"
title="struct libp2p::kad::U256">U256</a>  
Little-endian large integer type 256-bit unsigned integer.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AddProviderContext.html" class="enum"
title="enum libp2p::kad::AddProviderContext">AddProviderContext</a>  
The context of a
[`QueryInfo::AddProvider`](enum.QueryInfo.html#variant.AddProvider "variant libp2p::kad::QueryInfo::AddProvider")
query.

<a href="enum.AddProviderError.html" class="enum"
title="enum libp2p::kad::AddProviderError">AddProviderError</a>  
The possible errors when publishing a provider record.

<a href="enum.AddProviderPhase.html" class="enum"
title="enum libp2p::kad::AddProviderPhase">AddProviderPhase</a>  
The phases of a
[`QueryInfo::AddProvider`](enum.QueryInfo.html#variant.AddProvider "variant libp2p::kad::QueryInfo::AddProvider")
query.

<a href="enum.BootstrapError.html" class="enum"
title="enum libp2p::kad::BootstrapError">BootstrapError</a>  
The error result of
[`Behaviour::bootstrap`](struct.Behaviour.html#method.bootstrap "method libp2p::kad::Behaviour::bootstrap").

<a href="enum.BucketInserts.html" class="enum"
title="enum libp2p::kad::BucketInserts">BucketInserts</a>  
The configurable strategies for the insertion of peers and their
addresses into the k-buckets of the Kademlia routing table.

<a href="enum.Caching.html" class="enum"
title="enum libp2p::kad::Caching">Caching</a>  
The configuration for Kademlia “write-back” caching after successful
lookups via
[`Behaviour::get_record`](struct.Behaviour.html#method.get_record "method libp2p::kad::Behaviour::get_record").

<a href="enum.ConnectionType.html" class="enum"
title="enum libp2p::kad::ConnectionType">ConnectionType</a>  
Status of our connection to a node reported by the Kademlia protocol.

<a href="enum.Event.html" class="enum"
title="enum libp2p::kad::Event">Event</a>  
The events produced by the `Kademlia` behaviour.

<a href="enum.GetClosestPeersError.html" class="enum"
title="enum libp2p::kad::GetClosestPeersError">GetClosestPeersError</a>  
The error result of
[`Behaviour::get_closest_peers`](struct.Behaviour.html#method.get_closest_peers "method libp2p::kad::Behaviour::get_closest_peers").

<a href="enum.GetProvidersError.html" class="enum"
title="enum libp2p::kad::GetProvidersError">GetProvidersError</a>  
The error result of
[`Behaviour::get_providers`](struct.Behaviour.html#method.get_providers "method libp2p::kad::Behaviour::get_providers").

<a href="enum.GetProvidersOk.html" class="enum"
title="enum libp2p::kad::GetProvidersOk">GetProvidersOk</a>  
The successful result of
[`Behaviour::get_providers`](struct.Behaviour.html#method.get_providers "method libp2p::kad::Behaviour::get_providers").

<a href="enum.GetRecordError.html" class="enum"
title="enum libp2p::kad::GetRecordError">GetRecordError</a>  
The error result of
[`Behaviour::get_record`](struct.Behaviour.html#method.get_record "method libp2p::kad::Behaviour::get_record").

<a href="enum.GetRecordOk.html" class="enum"
title="enum libp2p::kad::GetRecordOk">GetRecordOk</a>  
The successful result of
[`Behaviour::get_record`](struct.Behaviour.html#method.get_record "method libp2p::kad::Behaviour::get_record").

<a href="enum.InboundRequest.html" class="enum"
title="enum libp2p::kad::InboundRequest">InboundRequest</a>  
Information about a received and handled inbound request.

<a href="enum.Mode.html" class="enum"
title="enum libp2p::kad::Mode">Mode</a>  
<a href="enum.NodeStatus.html" class="enum"
title="enum libp2p::kad::NodeStatus">NodeStatus</a>  
The status of a node in a bucket.

<a href="enum.PutRecordContext.html" class="enum"
title="enum libp2p::kad::PutRecordContext">PutRecordContext</a>  
The context of a
[`QueryInfo::PutRecord`](enum.QueryInfo.html#variant.PutRecord "variant libp2p::kad::QueryInfo::PutRecord")
query.

<a href="enum.PutRecordError.html" class="enum"
title="enum libp2p::kad::PutRecordError">PutRecordError</a>  
The error result of
[`Behaviour::put_record`](struct.Behaviour.html#method.put_record "method libp2p::kad::Behaviour::put_record").

<a href="enum.PutRecordPhase.html" class="enum"
title="enum libp2p::kad::PutRecordPhase">PutRecordPhase</a>  
The phases of a
[`QueryInfo::PutRecord`](enum.QueryInfo.html#variant.PutRecord "variant libp2p::kad::QueryInfo::PutRecord")
query.

<a href="enum.QueryInfo.html" class="enum"
title="enum libp2p::kad::QueryInfo">QueryInfo</a>  
Information about a running query.

<a href="enum.QueryResult.html" class="enum"
title="enum libp2p::kad::QueryResult">QueryResult</a>  
The results of Kademlia queries.

<a href="enum.Quorum.html" class="enum"
title="enum libp2p::kad::Quorum">Quorum</a>  
A quorum w.r.t. the configured replication factor specifies the minimum
number of distinct nodes that must be successfully contacted in order
for a query to succeed.

<a href="enum.RoutingUpdate.html" class="enum"
title="enum libp2p::kad::RoutingUpdate">RoutingUpdate</a>  
The possible outcomes of
[`Behaviour::add_address`](struct.Behaviour.html#method.add_address "method libp2p::kad::Behaviour::add_address").

<a href="enum.StoreInserts.html" class="enum"
title="enum libp2p::kad::StoreInserts">StoreInserts</a>  
The configurable filtering strategies for the acceptance of incoming
records.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ALPHA_VALUE.html" class="constant"
title="constant libp2p::kad::ALPHA_VALUE">ALPHA_VALUE</a>

The `α` parameter of the Kademlia specification.

<a href="constant.K_VALUE.html" class="constant"
title="constant libp2p::kad::K_VALUE">K_VALUE</a>

The `k` parameter of the Kademlia specification.

<a href="constant.PROTOCOL_NAME.html" class="constant"
title="constant libp2p::kad::PROTOCOL_NAME">PROTOCOL_NAME</a>

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.AddProviderResult.html" class="type"
title="type libp2p::kad::AddProviderResult">AddProviderResult</a>  
The result of publishing a provider record.

<a href="type.BootstrapResult.html" class="type"
title="type libp2p::kad::BootstrapResult">BootstrapResult</a>  
The result of
[`Behaviour::bootstrap`](struct.Behaviour.html#method.bootstrap "method libp2p::kad::Behaviour::bootstrap").

<a href="type.GetClosestPeersResult.html" class="type"
title="type libp2p::kad::GetClosestPeersResult">GetClosestPeersResult</a>  
The result of
[`Behaviour::get_closest_peers`](struct.Behaviour.html#method.get_closest_peers "method libp2p::kad::Behaviour::get_closest_peers").

<a href="type.GetProvidersResult.html" class="type"
title="type libp2p::kad::GetProvidersResult">GetProvidersResult</a>  
The result of
[`Behaviour::get_providers`](struct.Behaviour.html#method.get_providers "method libp2p::kad::Behaviour::get_providers").

<a href="type.GetRecordResult.html" class="type"
title="type libp2p::kad::GetRecordResult">GetRecordResult</a>  
The result of
[`Behaviour::get_record`](struct.Behaviour.html#method.get_record "method libp2p::kad::Behaviour::get_record").

<a href="type.PutRecordResult.html" class="type"
title="type libp2p::kad::PutRecordResult">PutRecordResult</a>  
The result of
[`Behaviour::put_record`](struct.Behaviour.html#method.put_record "method libp2p::kad::Behaviour::put_record").
