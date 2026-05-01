Crate quick\_protobuf\_codec Copy item path

<span class="sub-heading"><a href="../src/quick_protobuf_codec/lib.rs.html#1-274"
class="src">Source</a> </span>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Codec.html" class="struct"
title="struct quick_protobuf_codec::Codec">Codec</a>

[`Codec`](struct.Codec.html "struct quick_protobuf_codec::Codec")
implements
[`Encoder`](../asynchronous_codec/encoder/trait.Encoder.html "trait asynchronous_codec::encoder::Encoder")
and
[`Decoder`](../asynchronous_codec/decoder/trait.Decoder.html "trait asynchronous_codec::decoder::Decoder"),
uses
[`unsigned_varint`](../unsigned_varint/index.html "mod unsigned_varint")
to prefix messages with their length and uses
[`quick_protobuf`](../quick_protobuf/index.html "mod quick_protobuf")
and a provided `struct` implementing
[`MessageRead`](../quick_protobuf/message/trait.MessageRead.html "trait quick_protobuf::message::MessageRead")
and
[`MessageWrite`](../quick_protobuf/message/trait.MessageWrite.html "trait quick_protobuf::message::MessageWrite")
to do the encoding.

<a href="struct.Error.html" class="struct"
title="struct quick_protobuf_codec::Error">Error</a>
