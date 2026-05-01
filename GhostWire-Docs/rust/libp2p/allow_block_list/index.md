[libp2p](../index.html)

Crate allow\_block\_list Copy item path

<span class="sub-heading"><a href="../../src/libp2p_allow_block_list/lib.rs.html#21-494"
class="src">Source</a> </span>

Expand description

A libp2p module for managing allow and blocks lists to peers.

# <a href="#allow-list-example" class="doc-anchor">§</a>Allow list example

``` {.rust .rust-example-rendered}
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    allowed_peers: allow_block_list::Behaviour<AllowedPeers>,
}

let behaviour = MyBehaviour {
    allowed_peers: allow_block_list::Behaviour::default(),
};
```

# <a href="#block-list-example" class="doc-anchor">§</a>Block list example

``` {.rust .rust-example-rendered}
#[derive(NetworkBehaviour)]
struct MyBehaviour {
    blocked_peers: allow_block_list::Behaviour<BlockedPeers>,
}

let behaviour = MyBehaviour {
    blocked_peers: allow_block_list::Behaviour::default(),
};
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AllowedPeers.html" class="struct"
title="struct libp2p::allow_block_list::AllowedPeers">AllowedPeers</a>  
The list of explicitly allowed peers.

<a href="struct.Behaviour.html" class="struct"
title="struct libp2p::allow_block_list::Behaviour">Behaviour</a>  
A
[`NetworkBehaviour`](../swarm/trait.NetworkBehaviour.html "trait libp2p::swarm::NetworkBehaviour")
that can act as an allow or block list.

<a href="struct.Blocked.html" class="struct"
title="struct libp2p::allow_block_list::Blocked">Blocked</a>  
A connection to this peer was explicitly blocked and was thus
[`denied`](../swarm/struct.ConnectionDenied.html "struct libp2p::swarm::ConnectionDenied").

<a href="struct.BlockedPeers.html" class="struct"
title="struct libp2p::allow_block_list::BlockedPeers">BlockedPeers</a>  
The list of explicitly blocked peers.

<a href="struct.NotAllowed.html" class="struct"
title="struct libp2p::allow_block_list::NotAllowed">NotAllowed</a>  
A connection to this peer is not explicitly allowed and was thus
[`denied`](../swarm/struct.ConnectionDenied.html "struct libp2p::swarm::ConnectionDenied").
