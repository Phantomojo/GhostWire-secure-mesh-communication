Crate bluer Copy item path

<span class="sub-heading"><a href="../src/bluer/lib.rs.html#1-1102" class="src">Source</a>
</span>

Expand description

# <a href="#bluer--official-bluez-bindings-for-rust"
class="doc-anchor">§</a>BlueR — Official BlueZ Bindings for Rust

This library provides the official Rust interface to the [Linux
Bluetooth protocol stack (BlueZ)](http://www.bluez.org/). Both
publishing local and consuming remote [GATT
services](https://www.oreilly.com/library/view/getting-started-with/9781491900550/ch04.html)
using *idiomatic* Rust code is supported. L2CAP and RFCOMM sockets are
presented using an API similar to Tokio networking.

This library depends on the [tokio](../tokio/index.html "mod tokio")
asynchronous runtime.

The following functionality is provided.

-   [Bluetooth adapters](struct.Adapter.html "struct bluer::Adapter")
    -   [enumeration](struct.Session.html#method.adapter_names "method bluer::Session::adapter_names")
    -   configuration of power, discoverability, name, etc.
    -   hot-plug support through change events stream
-   [Bluetooth devices](struct.Device.html "struct bluer::Device")
    -   [discovery](struct.Adapter.html#method.discover_devices "method bluer::Adapter::discover_devices")
        with custom filters
    -   querying of address, name, class, signal strength (RSSI), etc.
    -   Bluetooth Low Energy advertisements
    -   [change events
        stream](struct.Adapter.html#method.events "method bluer::Adapter::events")
    -   connecting and pairing
    -   [passive LE advertisement
        monitoring](struct.Adapter.html#method.monitor "method bluer::Adapter::monitor")
-   [consumption of remote GATT
    services](struct.Device.html#method.services "method bluer::Device::services")
    -   GATT service discovery
    -   read, write and notify operations on characteristics
    -   read and write operations on characteristic descriptors
    -   optional use of low-overhead
        [AsyncRead](../tokio/io/async_read/trait.AsyncRead.html "trait tokio::io::async_read::AsyncRead")
        and
        [AsyncWrite](../tokio/io/async_write/trait.AsyncWrite.html "trait tokio::io::async_write::AsyncWrite")
        streams for notify and write operations
-   [publishing local GATT
    services](struct.Adapter.html#method.serve_gatt_application "method bluer::Adapter::serve_gatt_application")
    -   read, write and notify operations on characteristics
    -   read and write operations on characteristic descriptors
    -   two programming models supported
        -   callback-based interface
        -   low-overhead
            [AsyncRead](../tokio/io/async_read/trait.AsyncRead.html "trait tokio::io::async_read::AsyncRead")
            and
            [AsyncWrite](../tokio/io/async_write/trait.AsyncWrite.html "trait tokio::io::async_write::AsyncWrite")
            streams
-   [sending Bluetooth Low Energy
    advertisements](struct.Adapter.html#method.advertise "method bluer::Adapter::advertise")
-   [Bluetooth authorization
    agent](agent/struct.Agent.html "struct bluer::agent::Agent")
-   efficient event dispatching
    -   not affected by D-Bus match rule count
    -   O(1) in number of subscriptions
-   [L2CAP sockets](l2cap)
    -   support for both classic Bluetooth (BR/EDR) and Bluetooth LE
    -   stream oriented
    -   sequential packet oriented
    -   datagram oriented
    -   async IO interface with
        [AsyncRead](../tokio/io/async_read/trait.AsyncRead.html "trait tokio::io::async_read::AsyncRead")
        and
        [AsyncWrite](../tokio/io/async_write/trait.AsyncWrite.html "trait tokio::io::async_write::AsyncWrite")
        support
-   [RFCOMM sockets](rfcomm)
    -   support for classic Bluetooth (BR/EDR)
    -   stream oriented
    -   async IO interface with
        [AsyncRead](../tokio/io/async_read/trait.AsyncRead.html "trait tokio::io::async_read::AsyncRead")
        and
        [AsyncWrite](../tokio/io/async_write/trait.AsyncWrite.html "trait tokio::io::async_write::AsyncWrite")
        support
-   [Bluetooth Mesh](mesh)
    -   provision and join networks
    -   send and receive messages
-   [database of assigned numbers](id)
    -   manufacturer ids
    -   services classes, GATT services, characteristics and descriptors

Currently, some classic Bluetooth (BR/EDR) functionality is missing.
However, pull requests and contributions are welcome!

## <a href="#crate-features" class="doc-anchor">§</a>Crate features

The following crate features are available.

-   `bluetoothd`: Enables all functions requiring a running Bluetooth
    daemon.
-   `id`: Enables database of assigned numbers.
-   `l2cap`: Enables L2CAP sockets.
-   `rfcomm`: Enables RFCOMM sockets.
-   `mesh`: Enables Bluetooth mesh functionality.
-   `serde`: Enables serialization and deserialization of some data
    types.

To enable all crate features specify the `full` crate feature.

## <a href="#basic-usage" class="doc-anchor">§</a>Basic usage

Create a [Session](struct.Session.html "struct bluer::Session") using
[Session::new](struct.Session.html#method.new "associated function bluer::Session::new");
this establishes a connection to the Bluetooth daemon. Then obtain a
Bluetooth adapter using
[Session::adapter](struct.Session.html#method.adapter "method bluer::Session::adapter").
From there on you can access most of the functionality using the methods
provided by [Adapter](struct.Adapter.html "struct bluer::Adapter").

## <a href="#l2cap-and-rfcomm-sockets" class="doc-anchor">§</a>L2CAP and RFCOMM sockets

Refer to the \[l2cap\] and \[rfcomm\] modules. No
[Session](struct.Session.html "struct bluer::Session") and therefore no
running Bluetooth daemon is required.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use uuid::`<a href="../uuid/struct.Uuid.html" class="struct"
title="struct uuid::Uuid"><code>Uuid</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="adv/index.html" class="mod" title="mod bluer::adv">adv</a>  
Bluetooth LE advertising.

<a href="agent/index.html" class="mod"
title="mod bluer::agent">agent</a>  
Bluetooth authorization agent.

<a href="gatt/index.html" class="mod" title="mod bluer::gatt">gatt</a>  
Local and remote GATT services.

<a href="monitor/index.html" class="mod"
title="mod bluer::monitor">monitor</a>  
Bluetooth advertisement monitor.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Adapter.html" class="struct"
title="struct bluer::Adapter">Adapter</a>  
Interface to a Bluetooth adapter.

<a href="struct.Address.html" class="struct"
title="struct bluer::Address">Address</a>  
Bluetooth address.

<a href="struct.Device.html" class="struct"
title="struct bluer::Device">Device</a>  
Interface to a Bluetooth device.

<a href="struct.DiscoveryFilter.html" class="struct"
title="struct bluer::DiscoveryFilter">DiscoveryFilter</a>  
Bluetooth device discovery filter.

<a href="struct.Error.html" class="struct"
title="struct bluer::Error">Error</a>  
Bluetooth error.

<a href="struct.InvalidAddress.html" class="struct"
title="struct bluer::InvalidAddress">InvalidAddress</a>  
Invalid Bluetooth address error.

<a href="struct.Modalias.html" class="struct"
title="struct bluer::Modalias">Modalias</a>  
Linux kernel modalias information.

<a href="struct.Session.html" class="struct"
title="struct bluer::Session">Session</a>  
Bluetooth session.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AdapterEvent.html" class="enum"
title="enum bluer::AdapterEvent">AdapterEvent</a>  
Bluetooth adapter event.

<a href="enum.AdapterProperty.html" class="enum"
title="enum bluer::AdapterProperty">AdapterProperty</a>  
Bluetooth adapter property.

<a href="enum.AddressType.html" class="enum"
title="enum bluer::AddressType">AddressType</a>  
Bluetooth device address type.

<a href="enum.DeviceEvent.html" class="enum"
title="enum bluer::DeviceEvent">DeviceEvent</a>  
Bluetooth device event.

<a href="enum.DeviceProperty.html" class="enum"
title="enum bluer::DeviceProperty">DeviceProperty</a>  
Bluetooth device property.

<a href="enum.DiscoveryTransport.html" class="enum"
title="enum bluer::DiscoveryTransport">DiscoveryTransport</a>  
Transport parameter determines the type of scan.

<a href="enum.ErrorKind.html" class="enum"
title="enum bluer::ErrorKind">ErrorKind</a>  
Bluetooth error kind.

<a href="enum.InternalErrorKind.html" class="enum"
title="enum bluer::InternalErrorKind">InternalErrorKind</a>  
Internal Bluetooth error kind.

<a href="enum.SessionEvent.html" class="enum"
title="enum bluer::SessionEvent">SessionEvent</a>  
Bluetooth session event.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.UuidExt.html" class="trait"
title="trait bluer::UuidExt">UuidExt</a>  
UUID extension trait to convert to and from Bluetooth short UUIDs.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type bluer::Result">Result</a>  
Bluetooth result.
