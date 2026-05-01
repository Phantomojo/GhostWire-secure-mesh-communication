Crate netlink\_proto Copy item path

<span class="sub-heading"><a href="../src/netlink_proto/lib.rs.html#3-314" class="src">Source</a>
</span>

Expand description

`netlink-proto` is an asynchronous implementation of the Netlink
protocol.

# <a href="#example-listening-for-audit-events" class="doc-anchor">§</a>Example: listening for audit events

This example shows how to use `netlink-proto` with the `tokio` runtime
to print audit events. It requires extra external dependencies:

-   `futures = "^0.3"`
-   `tokio = "^1.0"`
-   `netlink-packet-audit = "^0.1"`

``` {.rust .rust-example-rendered}
use futures::stream::StreamExt;
use netlink_packet_core::{NetlinkMessage, NetlinkPayload, NLM_F_ACK,
    NLM_F_REQUEST};
use netlink_packet_audit::{
    AuditMessage,
    StatusMessage,
};
use std::process;

use netlink_proto::{
    new_connection,
    sys::{protocols::NETLINK_AUDIT, SocketAddr},
};

const AUDIT_STATUS_ENABLED: u32 = 1;
const AUDIT_STATUS_PID: u32 = 4;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Create a netlink socket. Here:
    //
    // - `conn` is a `Connection` that has the netlink socket. It's a
    //   `Future` that keeps polling the socket and must be spawned an
    //   the event loop.
    //
    // - `handle` is a `Handle` to the `Connection`. We use it to send
    //   netlink messages and receive responses to these messages.
    //
    // - `messages` is a channel receiver through which we receive
    //   messages that we have not solicited, ie that are not
    //   response to a request we made. In this example, we'll receive
    //   the audit event through that channel.
    let (conn, mut handle, mut messages) = new_connection(NETLINK_AUDIT)
        .map_err(|e| format!("Failed to create a new netlink connection: {}", e))?;

    // Spawn the `Connection` so that it starts polling the netlink
    // socket in the background.
    tokio::spawn(conn);

    // Use the `ConnectionHandle` to send a request to the kernel
    // asking it to start multicasting audit event messages.
    tokio::spawn(async move {
        // Craft the packet to enable audit events
        let mut status = StatusMessage::new();
        status.enabled = 1;
        status.pid = process::id();
        status.mask = AUDIT_STATUS_ENABLED | AUDIT_STATUS_PID;
        let payload = AuditMessage::SetStatus(status);
        let mut nl_msg = NetlinkMessage::from(payload);
        nl_msg.header.flags = NLM_F_REQUEST | NLM_F_ACK;

        // We'll send unicast messages to the kernel.
        let kernel_unicast: SocketAddr = SocketAddr::new(0, 0);
        let mut response = match handle.request(nl_msg, kernel_unicast) {
            Ok(response) => response,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        while let Some(message) = response.next().await {
            if let NetlinkPayload::Error(err_message) = message.payload {
                eprintln!("Received an error message: {:?}", err_message);
                return;
            }
        }
    });

    // Finally, start receiving event through the `messages` channel.
    println!("Starting to print audit events... press ^C to interrupt");
    while let Some((message, _addr)) = messages.next().await {
        if let NetlinkPayload::Error(err_message) = message.payload {
            eprintln!("received an error message: {:?}", err_message);
        } else {
            println!("{:?}", message);
        }
    }

    Ok(())
}
```

# <a href="#example-dumping-all-the-machines-links"
class="doc-anchor">§</a>Example: dumping all the machine’s links

This example shows how to use `netlink-proto` with the ROUTE protocol.

Here we do not use `netlink_proto::new_connection()`, and instead create
the socket manually and use call `send()` and `receive()` directly. In
the previous example, the `NetlinkFramed` was wrapped in a `Connection`
which was polled automatically by the runtime.

``` {.rust .rust-example-rendered}
use futures::StreamExt;

use netlink_packet_route::{link::LinkMessage, RouteNetlinkMessage};
use netlink_packet_core::{
    NetlinkHeader,
    NetlinkMessage,
    NLM_F_REQUEST, NLM_F_DUMP
};

use netlink_proto::{
    new_connection,
    sys::{protocols::NETLINK_ROUTE, SocketAddr},
};

#[tokio::main]
async fn main() -> Result<(), String> {
    // Create the netlink socket. Here, we won't use the channel that
    // receives unsolicited messages.
    let (conn, mut handle, _) = new_connection(NETLINK_ROUTE)
        .map_err(|e| format!("Failed to create a new netlink connection: {}", e))?;

    // Spawn the `Connection` in the background
    tokio::spawn(conn);

    // Create the netlink message that requests the links to be dumped
    let mut nl_hdr = NetlinkHeader::default();
    nl_hdr.flags = NLM_F_DUMP | NLM_F_REQUEST;

    let msg = NetlinkMessage::new(
        nl_hdr,
        RouteNetlinkMessage::GetLink(LinkMessage::default()).into(),
    );

    // Send the request
    let mut response = handle
        .request(msg, SocketAddr::new(0, 0))
        .map_err(|e| format!("Failed to send request: {}", e))?;

    // Print all the messages received in response
    loop {
        if let Some(packet) = response.next().await {
            println!("<<< {:?}", packet);
        } else {
            break;
        }
    }

    Ok(())
}
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="sys/index.html" class="mod"
title="mod netlink_proto::sys">sys</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Connection.html" class="struct"
title="struct netlink_proto::Connection">Connection</a>

Connection to a Netlink socket, running in the background.

<a href="struct.ConnectionHandle.html" class="struct"
title="struct netlink_proto::ConnectionHandle">ConnectionHandle</a>

A handle to pass requests to a [`Connection`](struct.Connection.html).

<a href="struct.NetlinkCodec.html" class="struct"
title="struct netlink_proto::NetlinkCodec">NetlinkCodec</a>

Standard implementation of `NetlinkMessageCodec`

<a href="struct.NetlinkFramed.html" class="struct"
title="struct netlink_proto::NetlinkFramed">NetlinkFramed</a>

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum netlink_proto::Error">Error</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.NetlinkMessageCodec.html" class="trait"
title="trait netlink_proto::NetlinkMessageCodec">NetlinkMessageCodec</a>  
Protocol to serialize and deserialize messages to and from datagrams

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.from_socket_with_codec.html" class="fn"
title="fn netlink_proto::from_socket_with_codec">from_socket_with_codec</a>  
Variant of
[`new_connection`](fn.new_connection.html "fn netlink_proto::new_connection")
that allows specifying a socket type to use for async handling, a
special codec and a socket

<a href="fn.new_connection.html" class="fn"
title="fn netlink_proto::new_connection">new_connection</a>  
Create a new Netlink connection for the given Netlink protocol, and
returns a handle to that connection as well as a stream of unsolicited
messages received by that connection (unsolicited here means messages
that are not a response to a request made by the `Connection`).
`Connection<T>` wraps a Netlink socket and implements the Netlink
protocol.

<a href="fn.new_connection_with_codec.html" class="fn"
title="fn netlink_proto::new_connection_with_codec">new_connection_with_codec</a>  
Variant of
[`new_connection`](fn.new_connection.html "fn netlink_proto::new_connection")
that allows specifying a socket type to use for async handling and a
special codec

<a href="fn.new_connection_with_socket.html" class="fn"
title="fn netlink_proto::new_connection_with_socket">new_connection_with_socket</a>  
Variant of
[`new_connection`](fn.new_connection.html "fn netlink_proto::new_connection")
that allows specifying a socket type to use for async handling
