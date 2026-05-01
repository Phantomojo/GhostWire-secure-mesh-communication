Crate multiaddr Copy item path

<span class="sub-heading"><a href="../src/multiaddr/lib.rs.html#1-495" class="src">Source</a>
</span>

Expand description

Implementation of [multiaddr](https://github.com/multiformats/multiaddr)
in Rust.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../multihash/index.html" class="mod"
title="mod multihash"><code>multihash</code></a>`;`

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.multiaddr.html" class="macro"
title="macro multiaddr::multiaddr">multiaddr</a>  
Easy way for a user to create a `Multiaddr`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Iter.html" class="struct"
title="struct multiaddr::Iter">Iter</a>  
Iterator over `Multiaddr`
[`Protocol`](enum.Protocol.html "enum multiaddr::Protocol")s.

<a href="struct.Multiaddr.html" class="struct"
title="struct multiaddr::Multiaddr">Multiaddr</a>  
Representation of a Multiaddr.

<a href="struct.Onion3Addr.html" class="struct"
title="struct multiaddr::Onion3Addr">Onion3Addr</a>  
Represents an Onion v3 address

<a href="struct.PeerId.html" class="struct"
title="struct multiaddr::PeerId">PeerId</a>  
Identifier of a peer of the network.

<a href="struct.ProtoStackIter.html" class="struct"
title="struct multiaddr::ProtoStackIter">ProtoStackIter</a>  
Iterator over the string idtenfiers of the protocols (not addrs) in a
multiaddr

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum multiaddr::Error">Error</a>  
Error types

<a href="enum.FromUrlErr.html" class="enum"
title="enum multiaddr::FromUrlErr">FromUrlErr</a>  
Error while parsing an URL.

<a href="enum.Protocol.html" class="enum"
title="enum multiaddr::Protocol">Protocol</a>  
`Protocol` describes all possible multiaddress protocols.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.from_url.html" class="fn"
title="fn multiaddr::from_url">from_url</a>  
Attempts to parse an URL into a multiaddress.

<a href="fn.from_url_lossy.html" class="fn"
title="fn multiaddr::from_url_lossy">from_url_lossy</a>  
Attempts to parse an URL into a multiaddress. Ignores possible loss of
information.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type multiaddr::Result">Result</a>
