Crate bluez\_async Copy item path

<span class="sub-heading"><a href="../src/bluez_async/lib.rs.html#1-929" class="src">Source</a>
</span>

Expand description

An async wrapper around the D-Bus interface of BlueZ, the Linux
Bluetooth daemon. This provides type-safe interfaces to a subset of the
Bluetooth client (i.e. central, in Bluetooth terminology) interfaces
exposed by BlueZ, focussing on the Generic Attribute Profile (GATT) of
Bluetooth Low Energy (BLE).

Start by creating a
[`BluetoothSession`](struct.BluetoothSession.html "struct bluez_async::BluetoothSession").

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AdapterId.html" class="struct"
title="struct bluez_async::AdapterId">AdapterId</a>  
Opaque identifier for a Bluetooth adapter on the system.

<a href="struct.AdapterInfo.html" class="struct"
title="struct bluez_async::AdapterInfo">AdapterInfo</a>  
Information about a Bluetooth adapter on the system.

<a href="struct.BluetoothSession.html" class="struct"
title="struct bluez_async::BluetoothSession">BluetoothSession</a>  
A connection to the Bluetooth daemon. This can be cheaply cloned and
passed around to be used from different places. It is the main entry
point to the library.

<a href="struct.CharacteristicFlags.html" class="struct"
title="struct bluez_async::CharacteristicFlags">CharacteristicFlags</a>  
The set of flags (a.k.a. properties) of a characteristic, defining how
the characteristic can be used.

<a href="struct.CharacteristicId.html" class="struct"
title="struct bluez_async::CharacteristicId">CharacteristicId</a>  
Opaque identifier for a GATT characteristic on a Bluetooth device.

<a href="struct.CharacteristicInfo.html" class="struct"
title="struct bluez_async::CharacteristicInfo">CharacteristicInfo</a>  
Information about a GATT characteristic on a Bluetooth device.

<a href="struct.DescriptorId.html" class="struct"
title="struct bluez_async::DescriptorId">DescriptorId</a>  
Opaque identifier for a GATT characteristic descriptor on a Bluetooth
device.

<a href="struct.DescriptorInfo.html" class="struct"
title="struct bluez_async::DescriptorInfo">DescriptorInfo</a>  
Information about a GATT descriptor on a Bluetooth device.

<a href="struct.DeviceId.html" class="struct"
title="struct bluez_async::DeviceId">DeviceId</a>  
Opaque identifier for a Bluetooth device which the system knows about.
This includes a reference to which Bluetooth adapter it was discovered
on, which means that any attempt to connect to it will also happen from
that adapter (in case the system has more than one).

<a href="struct.DeviceInfo.html" class="struct"
title="struct bluez_async::DeviceInfo">DeviceInfo</a>  
Information about a Bluetooth device which was discovered. See
https://git.kernel.org/pub/scm/bluetooth/bluez.git/tree/doc/device-api.txt

<a href="struct.DiscoveryFilter.html" class="struct"
title="struct bluez_async::DiscoveryFilter">DiscoveryFilter</a>  
A set of filter parameters for discovery. Parameters may be set to
`None` to use the BlueZ defaults.

<a href="struct.MacAddress.html" class="struct"
title="struct bluez_async::MacAddress">MacAddress</a>  
MAC address of a Bluetooth device.

<a href="struct.Modalias.html" class="struct"
title="struct bluez_async::Modalias">Modalias</a>  
A parsed modalias string.

<a href="struct.ParseMacAddressError.html" class="struct"
title="struct bluez_async::ParseMacAddressError">ParseMacAddressError</a>  
An error parsing a MAC address from a string.

<a href="struct.ParseModaliasError.html" class="struct"
title="struct bluez_async::ParseModaliasError">ParseModaliasError</a>  
An error parsing a
[`Modalias`](struct.Modalias.html "struct bluez_async::Modalias") from a
string.

<a href="struct.ServiceId.html" class="struct"
title="struct bluez_async::ServiceId">ServiceId</a>  
Opaque identifier for a GATT service on a Bluetooth device.

<a href="struct.ServiceInfo.html" class="struct"
title="struct bluez_async::ServiceInfo">ServiceInfo</a>  
Information about a GATT service on a Bluetooth device.

<a href="struct.WriteOptions.html" class="struct"
title="struct bluez_async::WriteOptions">WriteOptions</a>  
A set of options for a characteristic write operation.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AdapterEvent.html" class="enum"
title="enum bluez_async::AdapterEvent">AdapterEvent</a>  
Details of an event related to a Bluetooth adapter.

<a href="enum.AddressType.html" class="enum"
title="enum bluez_async::AddressType">AddressType</a>  
MAC address type of a Bluetooth device.

<a href="enum.BluetoothError.html" class="enum"
title="enum bluez_async::BluetoothError">BluetoothError</a>  
An error carrying out a Bluetooth operation.

<a href="enum.BluetoothEvent.html" class="enum"
title="enum bluez_async::BluetoothEvent">BluetoothEvent</a>  
An event relating to a Bluetooth device or adapter.

<a href="enum.CharacteristicEvent.html" class="enum"
title="enum bluez_async::CharacteristicEvent">CharacteristicEvent</a>  
Details of an event related to a GATT characteristic.

<a href="enum.DeviceEvent.html" class="enum"
title="enum bluez_async::DeviceEvent">DeviceEvent</a>  
Details of an event related to a Bluetooth device.

<a href="enum.SpawnError.html" class="enum"
title="enum bluez_async::SpawnError">SpawnError</a>  
Error type for futures representing tasks spawned by this crate.

<a href="enum.Transport.html" class="enum"
title="enum bluez_async::Transport">Transport</a>  
The type of transport to use for a scan.

<a href="enum.WriteType.html" class="enum"
title="enum bluez_async::WriteType">WriteType</a>  
The type of write operation to use.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.BleUuid.html" class="trait"
title="trait bluez_async::BleUuid">BleUuid</a>  
An extension trait for `Uuid` which provides BLE-specific methods.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.uuid_from_u16.html" class="fn"
title="fn bluez_async::uuid_from_u16">uuid_from_u16</a>  
Convert a 16-bit BLE short UUID to a full 128-bit UUID by filling in the
standard Bluetooth Base UUID.

<a href="fn.uuid_from_u32.html" class="fn"
title="fn bluez_async::uuid_from_u32">uuid_from_u32</a>  
Convert a 32-bit BLE short UUID to a full 128-bit UUID by filling in the
standard Bluetooth Base UUID.
