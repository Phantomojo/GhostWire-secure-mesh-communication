Crate macaddr Copy item path

<span class="sub-heading"><a href="../src/macaddr/lib.rs.html#1-48" class="src">Source</a>
</span>

Expand description

This crate provides types for a [MAC
address](https://en.wikipedia.org/wiki/MAC_address) identifiers, both in
IEEE *EUI-48* and *EUI-64* formats.

It is like a `std::net::SocketAddr` enum with `std::net::SocketAddrV4`
and `std::net::SocketAddrV6` members, but for MAC addresses instead.

Obviously, MAC address can be represented as a `[u8; 6]` or `[u8; 8]`,
but it is error-prone and inconvenient, so here they are —
[MacAddr6](struct.MacAddr6.html) and [MacAddr8](struct.MacAddr8.html)
structs with helpful methods and standard Rust traits implementations to
make them first-class Rust objects.

## <a href="#serde-support" class="doc-anchor">§</a>Serde support

[Serde](https://serde.rs) support can be enabled with a `"serde_std"`
feature (disabled by default) if used in `std`-enabled builds.

This feature is called like this because of [this Cargo
bug](https://github.com/rust-lang/cargo/issues/3494).  
`"serde"` feature is exists also, but it is intended to be used in the
`no_std` builds.

## <a href="#no-std-support" class="doc-anchor">§</a>No-std support

This crate can be used in a `no_std` builds with disabled `"std"`
feature (enabled by default).

Enabled `"serde"` feature will add support for `no_std` serde
serialization and deserialization.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.MacAddr6.html" class="struct"
title="struct macaddr::MacAddr6">MacAddr6</a>  
MAC address in *EUI-48* format.

<a href="struct.MacAddr8.html" class="struct"
title="struct macaddr::MacAddr8">MacAddr8</a>  
MAC address in *EUI-64* format.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.MacAddr.html" class="enum"
title="enum macaddr::MacAddr">MacAddr</a>  
A MAC address, either in *EUI-48* or *EUI-64* format.

<a href="enum.ParseError.html" class="enum"
title="enum macaddr::ParseError">ParseError</a>  
An error which can be returned when parsing MAC address.
