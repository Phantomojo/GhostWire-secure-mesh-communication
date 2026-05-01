[libp2p](../index.html)

Crate gossipsub Copy item path

<span class="sub-heading"><a href="../../src/libp2p_gossipsub/lib.rs.html#21-141"
class="src">Source</a> </span>

Expand description

Implementation of the
[Gossipsub](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/README.md)
protocol.

Gossipsub is a P2P pubsub (publish/subscription) routing layer designed
to extend upon floodsub and meshsub routing protocols.

# <a href="#overview" class="doc-anchor">§</a>Overview

*Note: The gossipsub protocol specifications
(<https://github.com/libp2p/specs/tree/master/pubsub/gossipsub>) provide
an outline for the routing protocol. They should be consulted for
further detail.*

Gossipsub is a blend of meshsub for data and randomsub for mesh
metadata. It provides bounded degree and amplification factor with the
meshsub construction and augments it using gossip propagation of
metadata with the randomsub technique.

The router maintains an overlay mesh network of peers on which to
efficiently send messages and metadata. Peers use control messages to
broadcast and request known messages and subscribe/unsubscribe from
topics in the mesh network.

# <a href="#important-discrepancies" class="doc-anchor">§</a>Important Discrepancies

This section outlines the current implementation’s potential
discrepancies from that of other implementations, due to undefined
elements in the current specification.

-   **Topics** - In gossipsub, topics configurable by the `hash_topics`
    configuration parameter. Topics are of type
    [`TopicHash`](struct.TopicHash.html "struct libp2p::gossipsub::TopicHash").
    The current go implementation uses raw utf-8 strings, and this is
    default configuration in rust-libp2p. Topics can be hashed (SHA256
    hashed then base64 encoded) by setting the `hash_topics`
    configuration parameter to true.

-   **Sequence Numbers** - A message on the gossipsub network is
    identified by the source
    [`PeerId`](../struct.PeerId.html "struct libp2p::PeerId") and a
    nonce (sequence number) of the message. The sequence numbers in this
    implementation are sent as raw bytes across the wire. They are
    64-bit big-endian unsigned integers. When messages are signed, they
    are monotonically increasing integers starting from a random value
    and wrapping around u64::MAX. When messages are unsigned, they are
    chosen at random. NOTE: These numbers are sequential in the current
    go implementation.

# <a href="#peer-discovery" class="doc-anchor">§</a>Peer Discovery

Gossipsub does not provide peer discovery by itself. Peer discovery is
the process by which peers in a p2p network exchange information about
each other among other reasons to become resistant against the failure
or replacement of the [boot
nodes](https://docs.libp2p.io/reference/glossary/#boot-node) of the
network.

Peer discovery can e.g. be implemented with the help of the
[Kademlia](https://github.com/libp2p/specs/blob/master/kad-dht/README.md)
protocol in combination with the
[Identify](https://github.com/libp2p/specs/tree/master/identify)
protocol. See the Kademlia implementation documentation for more
information.

# <a href="#using-gossipsub" class="doc-anchor">§</a>Using Gossipsub

## <a href="#gossipsub-config" class="doc-anchor">§</a>Gossipsub Config

The [`Config`](struct.Config.html) struct specifies various network
performance/tuning configuration parameters. Specifically it specifies:

This struct implements the
[`Default`](https://doc.rust-lang.org/1.93.1/core/default/trait.Default.html "trait core::default::Default")
trait and can be initialised via
[`Config::default()`](struct.Config.html#method.default "associated function libp2p::gossipsub::Config::default").

## <a href="#behaviour" class="doc-anchor">§</a>Behaviour

The [`Behaviour`](struct.Behaviour.html) struct implements the
[`libp2p_swarm::NetworkBehaviour`](../swarm/trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
trait allowing it to act as the routing behaviour in a
[`libp2p_swarm::Swarm`](../struct.Swarm.html "struct libp2p::Swarm").
This struct requires an instance of
[`PeerId`](../struct.PeerId.html "struct libp2p::PeerId") and
[`Config`](struct.Config.html).

## <a href="#example" class="doc-anchor">§</a>Example

For an example on how to use gossipsub, see the
[chat-example](https://github.com/libp2p/rust-libp2p/tree/master/examples/chat).

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AllowAllSubscriptionFilter.html" class="struct"
title="struct libp2p::gossipsub::AllowAllSubscriptionFilter">AllowAllSubscriptionFilter</a>  
Allows all subscriptions

<a href="struct.Behaviour.html" class="struct"
title="struct libp2p::gossipsub::Behaviour">Behaviour</a>  
Network behaviour that handles the gossipsub protocol.

<a href="struct.CallbackSubscriptionFilter.html" class="struct"
title="struct libp2p::gossipsub::CallbackSubscriptionFilter">CallbackSubscriptionFilter</a>  
<a href="struct.CombinedSubscriptionFilters.html" class="struct"
title="struct libp2p::gossipsub::CombinedSubscriptionFilters">CombinedSubscriptionFilters</a>  
Combines two subscription filters

<a href="struct.Config.html" class="struct"
title="struct libp2p::gossipsub::Config">Config</a>  
Configuration parameters that define the performance of the gossipsub
network.

<a href="struct.ConfigBuilder.html" class="struct"
title="struct libp2p::gossipsub::ConfigBuilder">ConfigBuilder</a>  
The builder struct for constructing a gossipsub configuration.

<a href="struct.FailedMessages.html" class="struct"
title="struct libp2p::gossipsub::FailedMessages">FailedMessages</a>  
Messages that have expired while attempting to be sent to a peer.

<a href="struct.IdentityTransform.html" class="struct"
title="struct libp2p::gossipsub::IdentityTransform">IdentityTransform</a>  
The default transform, the raw data is propagated as is to the
application layer gossipsub.

<a href="struct.MaxCountSubscriptionFilter.html" class="struct"
title="struct libp2p::gossipsub::MaxCountSubscriptionFilter">MaxCountSubscriptionFilter</a>  
Adds a max count to a given subscription filter

<a href="struct.Message.html" class="struct"
title="struct libp2p::gossipsub::Message">Message</a>  
The message sent to the user after a
[`RawMessage`](struct.RawMessage.html "struct libp2p::gossipsub::RawMessage")
has been transformed by a
[`crate::DataTransform`](trait.DataTransform.html "trait libp2p::gossipsub::DataTransform").

<a href="struct.MessageId.html" class="struct"
title="struct libp2p::gossipsub::MessageId">MessageId</a>  
<a href="struct.PeerScoreParams.html" class="struct"
title="struct libp2p::gossipsub::PeerScoreParams">PeerScoreParams</a>  
<a href="struct.PeerScoreThresholds.html" class="struct"
title="struct libp2p::gossipsub::PeerScoreThresholds">PeerScoreThresholds</a>  
<a href="struct.RawMessage.html" class="struct"
title="struct libp2p::gossipsub::RawMessage">RawMessage</a>  
A message received by the gossipsub system and stored locally in
caches..

<a href="struct.RegexSubscriptionFilter.html" class="struct"
title="struct libp2p::gossipsub::RegexSubscriptionFilter">RegexSubscriptionFilter</a>  
A subscription filter that filters topics based on a regular expression.

<a href="struct.Topic.html" class="struct"
title="struct libp2p::gossipsub::Topic">Topic</a>  
A gossipsub topic.

<a href="struct.TopicHash.html" class="struct"
title="struct libp2p::gossipsub::TopicHash">TopicHash</a>  
<a href="struct.TopicScoreParams.html" class="struct"
title="struct libp2p::gossipsub::TopicScoreParams">TopicScoreParams</a>  
<a href="struct.WhitelistSubscriptionFilter.html" class="struct"
title="struct libp2p::gossipsub::WhitelistSubscriptionFilter">WhitelistSubscriptionFilter</a>  
Allows only whitelisted subscriptions

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ConfigBuilderError.html" class="enum"
title="enum libp2p::gossipsub::ConfigBuilderError">ConfigBuilderError</a>  
Error associated with Config building.

<a href="enum.Event.html" class="enum"
title="enum libp2p::gossipsub::Event">Event</a>  
Event that can be emitted by the gossipsub behaviour.

<a href="enum.MessageAcceptance.html" class="enum"
title="enum libp2p::gossipsub::MessageAcceptance">MessageAcceptance</a>  
Validation kinds from the application for received messages.

<a href="enum.MessageAuthenticity.html" class="enum"
title="enum libp2p::gossipsub::MessageAuthenticity">MessageAuthenticity</a>  
Determines if published messages should be signed or not.

<a href="enum.PublishError.html" class="enum"
title="enum libp2p::gossipsub::PublishError">PublishError</a>  
Error associated with publishing a gossipsub message.

<a href="enum.SubscriptionError.html" class="enum"
title="enum libp2p::gossipsub::SubscriptionError">SubscriptionError</a>  
Error associated with subscribing to a topic.

<a href="enum.ValidationError.html" class="enum"
title="enum libp2p::gossipsub::ValidationError">ValidationError</a>  
<a href="enum.ValidationMode.html" class="enum"
title="enum libp2p::gossipsub::ValidationMode">ValidationMode</a>  
The types of message validation that can be employed by gossipsub.

<a href="enum.Version.html" class="enum"
title="enum libp2p::gossipsub::Version">Version</a>  
Selector for custom Protocol Id

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.DataTransform.html" class="trait"
title="trait libp2p::gossipsub::DataTransform">DataTransform</a>

A general trait of transforming a
[`RawMessage`](struct.RawMessage.html "struct libp2p::gossipsub::RawMessage")
into a
[`Message`](struct.Message.html "struct libp2p::gossipsub::Message").
The

<a href="trait.Hasher.html" class="trait"
title="trait libp2p::gossipsub::Hasher">Hasher</a>

A generic trait that can be extended for various hashing types for a
topic.

<a href="trait.TopicSubscriptionFilter.html" class="trait"
title="trait libp2p::gossipsub::TopicSubscriptionFilter">TopicSubscriptionFilter</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.score_parameter_decay.html" class="fn"
title="fn libp2p::gossipsub::score_parameter_decay">score_parameter_decay</a>  
Computes the decay factor for a parameter, assuming the `decay_interval`
is 1s and that the value decays to zero if it drops below 0.01.

<a href="fn.score_parameter_decay_with_base.html" class="fn"
title="fn libp2p::gossipsub::score_parameter_decay_with_base">score_parameter_decay_with_base</a>  
Computes the decay factor for a parameter using base as the
`decay_interval`.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.IdentTopic.html" class="type"
title="type libp2p::gossipsub::IdentTopic">IdentTopic</a>

<a href="type.Rpc.html" class="type"
title="type libp2p::gossipsub::Rpc">Rpc</a><span class="stab deprecated"
title="">Deprecated</span>

<a href="type.Sha256Topic.html" class="type"
title="type libp2p::gossipsub::Sha256Topic">Sha256Topic</a>
