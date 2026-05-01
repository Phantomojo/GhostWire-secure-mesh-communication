Crate libp2p\_identity Copy item path

<span class="sub-heading"><a href="../src/libp2p_identity/lib.rs.html#22-135"
class="src">Source</a> </span>

Expand description

A node’s network identity keys.

Such identity keys can be randomly generated on every startup, but using
already existing, fixed keys is usually required. Though libp2p uses
other crates (e.g. `ed25519_dalek`) internally, such details are not
exposed as part of libp2p’s public interface to keep them easily
upgradable or replaceable (e.g. to `ed25519_zebra`). Consequently, keys
of external ed25519 or secp256k1 crates cannot be directly converted
into libp2p network identities. Instead, loading fixed keys must use the
standard, thus more portable binary representation of the specific key
type (e.g. [ed25519 binary
format](https://datatracker.ietf.org/doc/html/rfc8032#section-5.1.5)).
All key types have functions to enable conversion to/from their binary
representations.

# Modules<a href="#modules" class="anchor">§</a>

<a href="ed25519/index.html" class="mod"
title="mod libp2p_identity::ed25519">ed25519</a>  
Ed25519 keys.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DecodingError.html" class="struct"
title="struct libp2p_identity::DecodingError">DecodingError</a>  
An error during decoding of key material.

<a href="struct.Keypair.html" class="struct"
title="struct libp2p_identity::Keypair">Keypair</a>  
Identity keypair of a node.

<a href="struct.OtherVariantError.html" class="struct"
title="struct libp2p_identity::OtherVariantError">OtherVariantError</a>  
Error produced when failing to convert
[`Keypair`](struct.Keypair.html "struct libp2p_identity::Keypair") to a
more concrete keypair.

<a href="struct.PeerId.html" class="struct"
title="struct libp2p_identity::PeerId">PeerId</a>  
Identifier of a peer of the network.

<a href="struct.PublicKey.html" class="struct"
title="struct libp2p_identity::PublicKey">PublicKey</a>  
The public key of a node’s identity keypair.

<a href="struct.SigningError.html" class="struct"
title="struct libp2p_identity::SigningError">SigningError</a>  
An error during signing of a message.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.KeyType.html" class="enum"
title="enum libp2p_identity::KeyType">KeyType</a>  
The type of key a `KeyPair` is holding.

<a href="enum.ParseError.html" class="enum"
title="enum libp2p_identity::ParseError">ParseError</a>  
Error when parsing a
[`PeerId`](struct.PeerId.html "struct libp2p_identity::PeerId") from
string or bytes.
