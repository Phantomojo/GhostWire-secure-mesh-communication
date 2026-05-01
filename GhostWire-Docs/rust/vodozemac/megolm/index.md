[vodozemac](../index.html)

Module megolm Copy item path

<span class="sub-heading"><a href="../../src/vodozemac/megolm/mod.rs.html#15-342"
class="src">Source</a> </span>

Expand description

An implementation of the Megolm ratchet.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DecryptedMessage.html" class="struct"
title="struct vodozemac::megolm::DecryptedMessage">DecryptedMessage</a>  
A message successfully decrypted by an
[`InboundGroupSession`](struct.InboundGroupSession.html "struct vodozemac::megolm::InboundGroupSession").

<a href="struct.ExportedSessionKey.html" class="struct"
title="struct vodozemac::megolm::ExportedSessionKey">ExportedSessionKey</a>  
The exported session key.

<a href="struct.GroupSession.html" class="struct"
title="struct vodozemac::megolm::GroupSession">GroupSession</a>  
A Megolm group session represents a single sending participant in an
encrypted group communication context containing multiple receiving
parties.

<a href="struct.GroupSessionPickle.html" class="struct"
title="struct vodozemac::megolm::GroupSessionPickle">GroupSessionPickle</a>  
A format suitable for serialization which implements
[`serde::Serialize`](../../serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize")
and
[`serde::Deserialize`](../../serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize").
Obtainable by calling
[`GroupSession::pickle`](struct.GroupSession.html#method.pickle "method vodozemac::megolm::GroupSession::pickle").

<a href="struct.InboundGroupSession.html" class="struct"
title="struct vodozemac::megolm::InboundGroupSession">InboundGroupSession</a>  
A Megolm inbound group session represents a single receiving participant
in an encrypted group communication involving multiple recipients.

<a href="struct.InboundGroupSessionPickle.html" class="struct"
title="struct vodozemac::megolm::InboundGroupSessionPickle">InboundGroupSessionPickle</a>  
A format suitable for serialization which implements
[`serde::Serialize`](../../serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize")
and
[`serde::Deserialize`](../../serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize").
Obtainable by calling
[`InboundGroupSession::pickle`](struct.InboundGroupSession.html#method.pickle "method vodozemac::megolm::InboundGroupSession::pickle").

<a href="struct.MegolmMessage.html" class="struct"
title="struct vodozemac::megolm::MegolmMessage">MegolmMessage</a>  
An encrypted Megolm message.

<a href="struct.SessionConfig.html" class="struct"
title="struct vodozemac::megolm::SessionConfig">SessionConfig</a>  
A struct to configure how Megolm sessions should work under the hood.
Currently only the MAC truncation behaviour can be configured.

<a href="struct.SessionKey.html" class="struct"
title="struct vodozemac::megolm::SessionKey">SessionKey</a>  
The session key, can be used to create a
[`InboundGroupSession`](#crate.megolm.InboundGroupSession).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DecryptionError.html" class="enum"
title="enum vodozemac::megolm::DecryptionError">DecryptionError</a>  
Error type for Megolm-based decryption failures.

<a href="enum.SessionKeyDecodeError.html" class="enum"
title="enum vodozemac::megolm::SessionKeyDecodeError">SessionKeyDecodeError</a>  
Error type describing failure modes for the `SessionKey` and
`ExportedSessionKey` decoding.

<a href="enum.SessionOrdering.html" class="enum"
title="enum vodozemac::megolm::SessionOrdering">SessionOrdering</a>  
The result of a comparison between two
[`InboundGroupSession`](struct.InboundGroupSession.html "struct vodozemac::megolm::InboundGroupSession")
types.
