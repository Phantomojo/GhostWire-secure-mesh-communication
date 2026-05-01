Crate netlink\_packet\_core Copy item path

<span class="sub-heading"><a href="../src/netlink_packet_core/lib.rs.html#3-272"
class="src">Source</a> </span>

Expand description

`netlink-packet-core` provides a generic netlink message
`NetlinkMessage<T>` that is independant of the sub-protocol. Such
messages are not very useful by themselves, since they are just used to
carry protocol-dependant messages. That is what the `T` represent: `T`
is the `NetlinkMessage`’s protocol-dependant message. This can be any
type that implements `NetlinkSerializable` and `NetlinkDeserializable`.

For instance, the `netlink-packet-route` crate provides rtnetlink
messages via `netlink_packet_route::RtnlMessage`, and
`netlink-packet-audit` provides audit messages via
`netlink_packet_audit::AuditMessage`.

By itself, the `netlink-packet-core` crate is not very useful. However,
it is used in `netlink-proto` to provide an asynchronous implementation
of the netlink protocol for any sub-protocol. Thus, a crate that defines
messages for a given netlink sub-protocol could integrate with
`netlink-packet-core` and would get an asynchronous implementation for
free. See the second example below for such an integration, via the
`NetlinkSerializable` and `NetlinkDeserializable` traits.

# <a href="#example-usage-with-netlink-packet-route"
class="doc-anchor">§</a>Example: usage with `netlink-packet-route`

This example shows how to serialize and deserialize netlink packet for
the rtnetlink sub-protocol. It requires `netlink-packet-route`.

``` {.rust .rust-example-rendered}
use netlink_packet_core::{NLM_F_DUMP, NLM_F_REQUEST};
use netlink_packet_route::{LinkMessage, RtnlMessage, NetlinkMessage,
NetlinkHeader};

// Create the netlink message, that contains the rtnetlink
// message
let mut packet = NetlinkMessage {
    header: NetlinkHeader {
        sequence_number: 1,
        flags: NLM_F_DUMP | NLM_F_REQUEST,
        ..Default::default()
    },
    payload: RtnlMessage::GetLink(LinkMessage::default()).into(),
};

// Before serializing the packet, it is important to call
// finalize() to ensure the header of the message is consistent
// with its payload. Otherwise, a panic may occur when calling
// serialize()
packet.finalize();

// Prepare a buffer to serialize the packet. Note that we never
// set explicitely `packet.header.length` above. This was done
// automatically when we called `finalize()`
let mut buf = vec![0; packet.header.length as usize];
// Serialize the packet
packet.serialize(&mut buf[..]);

// Deserialize the packet
let deserialized_packet =
    NetlinkMessage::<RtnlMessage>::deserialize(&buf).expect("Failed to deserialize message");

// Normally, the deserialized packet should be exactly the same
// than the serialized one.
assert_eq!(deserialized_packet, packet);

println!("{:?}", packet);
```

# <a href="#example-adding-messages-for-new-netlink-sub-protocol"
class="doc-anchor">§</a>Example: adding messages for new netlink sub-protocol

Let’s assume we have a netlink protocol called “ping pong” that defines
two types of messages: “ping” messages, which payload can be any
sequence of bytes, and “pong” message, which payload is also a sequence
of bytes. The protocol works as follow: when an enpoint receives a
“ping” message, it answers with a “pong”, with the payload of the “ping”
it’s answering to.

“ping” messages have type 18 and “pong” have type “20”. Here is what a
“ping” message that would look like if its payload is `[0, 1, 2, 3]`:

``` language-no_rust
0                8                16              24               32
+----------------+----------------+----------------+----------------+
|                 packet length (including header) = 16 + 4 = 20    |
+----------------+----------------+----------------+----------------+
|     message type = 18 (ping)    |              flags              |
+----------------+----------------+----------------+----------------+
|                           sequence number                         |
+----------------+----------------+----------------+----------------+
|                            port number                            |
+----------------+----------------+----------------+----------------+
|       0        |         1      |         2      |        3       |
+----------------+----------------+----------------+----------------+
```

And the “pong” response would be:

``` language-no_rust
0                8                16              24               32
+----------------+----------------+----------------+----------------+
|                 packet length (including header) = 16 + 4 = 20    |
+----------------+----------------+----------------+----------------+
|     message type = 20 (pong)    |              flags              |
+----------------+----------------+----------------+----------------+
|                           sequence number                         |
+----------------+----------------+----------------+----------------+
|                            port number                            |
+----------------+----------------+----------------+----------------+
|       0        |         1      |         2      |        3       |
+----------------+----------------+----------------+----------------+
```

Here is how we could implement the messages for such a protocol and
integrate this implementation with `netlink-packet-core`:

``` {.rust .rust-example-rendered}
use netlink_packet_core::{
    NetlinkDeserializable, NetlinkHeader, NetlinkMessage, NetlinkPayload, NetlinkSerializable,
};
use std::error::Error;
use std::fmt;

// PingPongMessage represent the messages for the "ping-pong" netlink
// protocol. There are only two types of messages.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PingPongMessage {
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}

// The netlink header contains a "message type" field that identifies
// the message it carries. Some values are reserved, and we
// arbitrarily decided that "ping" type is 18 and "pong" type is 20.
pub const PING_MESSAGE: u16 = 18;
pub const PONG_MESSAGE: u16 = 20;

// A custom error type for when deserialization fails. This is
// required because `NetlinkDeserializable::Error` must implement
// `std::error::Error`, so a simple `String` won't cut it.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeserializeError(&'static str);

impl Error for DeserializeError {
    fn description(&self) -> &str {
        self.0
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// NetlinkDeserializable implementation
impl NetlinkDeserializable for PingPongMessage {
    type Error = DeserializeError;

    fn deserialize(header: &NetlinkHeader, payload: &[u8]) -> Result<Self, Self::Error> {
        match header.message_type {
            PING_MESSAGE => Ok(PingPongMessage::Ping(payload.to_vec())),
            PONG_MESSAGE => Ok(PingPongMessage::Pong(payload.to_vec())),
            _ => Err(DeserializeError(
                "invalid ping-pong message: invalid message type",
            )),
        }
    }
}

// NetlinkSerializable implementation
impl NetlinkSerializable for PingPongMessage {
    fn message_type(&self) -> u16 {
        match self {
            PingPongMessage::Ping(_) => PING_MESSAGE,
            PingPongMessage::Pong(_) => PONG_MESSAGE,
        }
    }

    fn buffer_len(&self) -> usize {
        match self {
            PingPongMessage::Ping(vec) | PingPongMessage::Pong(vec) => vec.len(),
        }
    }

    fn serialize(&self, buffer: &mut [u8]) {
        match self {
            PingPongMessage::Ping(vec) | PingPongMessage::Pong(vec) => {
                buffer.copy_from_slice(&vec[..])
            }
        }
    }
}

// It can be convenient to be able to create a NetlinkMessage directly
// from a PingPongMessage. Since NetlinkMessage<T> already implements
// From<NetlinkPayload<T>>, we just need to implement
// From<NetlinkPayload<PingPongMessage>> for this to work.
impl From<PingPongMessage> for NetlinkPayload<PingPongMessage> {
    fn from(message: PingPongMessage) -> Self {
        NetlinkPayload::InnerMessage(message)
    }
}

fn main() {
    let ping_pong_message = PingPongMessage::Ping(vec![0, 1, 2, 3]);
    let mut packet = NetlinkMessage::from(ping_pong_message);

    // Before serializing the packet, it is very important to call
    // finalize() to ensure the header of the message is consistent
    // with its payload. Otherwise, a panic may occur when calling
    // `serialize()`
    packet.finalize();

    // Prepare a buffer to serialize the packet. Note that we never
    // set explicitely `packet.header.length` above. This was done
    // automatically when we called `finalize()`
    let mut buf = vec![0; packet.header.length as usize];
    // Serialize the packet
    packet.serialize(&mut buf[..]);

    // Deserialize the packet
    let deserialized_packet = NetlinkMessage::<PingPongMessage>::deserialize(&buf)
        .expect("Failed to deserialize message");

    // Normally, the deserialized packet should be exactly the same
    // than the serialized one.
    assert_eq!(deserialized_packet, packet);

    // This should print:
    // NetlinkMessage { header: NetlinkHeader { length: 20, message_type: 18, flags: 0, sequence_number: 0, port_number: 0 }, payload: InnerMessage(Ping([0, 1, 2, 3])) }
    println!("{:?}", packet);
}
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::`<a href="done/index.html" class="mod"
title="mod netlink_packet_core::done"><code>done</code></a>`::*;`

`pub use self::`<a href="error/index.html" class="mod"
title="mod netlink_packet_core::error"><code>error</code></a>`::*;`

`pub use self::`<a href="buffer/index.html" class="mod"
title="mod netlink_packet_core::buffer"><code>buffer</code></a>`::*;`

`pub use self::`<a href="header/index.html" class="mod"
title="mod netlink_packet_core::header"><code>header</code></a>`::*;`

`pub use self::`<a href="constants/index.html" class="mod"
title="mod netlink_packet_core::constants"><code>constants</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="buffer/index.html" class="mod"
title="mod netlink_packet_core::buffer">buffer</a>

<a href="constants/index.html" class="mod"
title="mod netlink_packet_core::constants">constants</a>

<a href="done/index.html" class="mod"
title="mod netlink_packet_core::done">done</a>

<a href="error/index.html" class="mod"
title="mod netlink_packet_core::error">error</a>

<a href="header/index.html" class="mod"
title="mod netlink_packet_core::header">header</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.NetlinkMessage.html" class="struct"
title="struct netlink_packet_core::NetlinkMessage">NetlinkMessage</a>  
Represent a netlink message.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.NetlinkPayload.html" class="enum"
title="enum netlink_packet_core::NetlinkPayload">NetlinkPayload</a>

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.NLMSG_ALIGNTO.html" class="constant"
title="constant netlink_packet_core::NLMSG_ALIGNTO">NLMSG_ALIGNTO</a>

<a href="constant.NLMSG_DONE.html" class="constant"
title="constant netlink_packet_core::NLMSG_DONE">NLMSG_DONE</a>

The message terminates a multipart message. Data lost

<a href="constant.NLMSG_ERROR.html" class="constant"
title="constant netlink_packet_core::NLMSG_ERROR">NLMSG_ERROR</a>

The message signals an error and the payload contains a nlmsgerr
structure. This can be looked at as a NACK and typically it is from FEC
to CPC.

<a href="constant.NLMSG_NOOP.html" class="constant"
title="constant netlink_packet_core::NLMSG_NOOP">NLMSG_NOOP</a>

The message is ignored.

<a href="constant.NLMSG_OVERRUN.html" class="constant"
title="constant netlink_packet_core::NLMSG_OVERRUN">NLMSG_OVERRUN</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.NetlinkDeserializable.html" class="trait"
title="trait netlink_packet_core::NetlinkDeserializable">NetlinkDeserializable</a>

A `NetlinkDeserializable` type can be deserialized from a buffer

<a href="trait.NetlinkSerializable.html" class="trait"
title="trait netlink_packet_core::NetlinkSerializable">NetlinkSerializable</a>
