[libp2p](../index.html)

Crate multihash Copy item path

<span class="sub-heading"><a href="../../src/multihash/lib.rs.html#1-44" class="src">Source</a>
</span>

Expand description

Bare-minimum multihash data structure.

This crate defines a `no_std` compatible data structures for
representing a `Multihash`.

It does not offer any hashing, instead you are encouraged to either do
the hashing yourself. Alternatively, you can use an existing code table
or make your own code table.

The [`multihash-codetable`](https://docs.rs/multihash-codetable) crate
defines a set of hashes to get started quickly. To make your own
codetable, use the
[`multihash-derive`](https://docs.rs/multihash-derive) crate.

The `arb` feature flag enables the quickcheck arbitrary implementation
for property based testing.

For serializing the multihash there is support for
[Serde](https://serde.rs) via the `serde-codec` feature and the [SCALE
Codec](https://github.com/paritytech/parity-scale-codec) via the
`scale-codec` feature.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct libp2p::multihash::Error">Error</a>  
Opaque error struct for operations involving a
[`Multihash`](struct.Multihash.html "struct libp2p::multihash::Multihash").

<a href="struct.Multihash.html" class="struct"
title="struct libp2p::multihash::Multihash">Multihash</a>  
A Multihash instance that only supports the basic functionality and no
hashing.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.MultihashGeneric.html" class="type"
title="type libp2p::multihash::MultihashGeneric">MultihashGeneric</a><span class="stab deprecated" title="">Deprecated</span>  
Deprecated type-alias for the
[`Multihash`](struct.Multihash.html "struct libp2p::multihash::Multihash")
type.

<a href="type.Result.html" class="type"
title="type libp2p::multihash::Result">Result</a><span class="stab deprecated" title="">Deprecated</span>  
Multihash result.
