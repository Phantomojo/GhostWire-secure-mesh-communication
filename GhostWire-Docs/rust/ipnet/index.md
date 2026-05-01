Crate ipnet Copy item path

<span class="sub-heading"><a href="../src/ipnet/lib.rs.html#1-106" class="src">Source</a>
</span>

Expand description

Types for IPv4 and IPv6 network addresses.

This module provides types and useful methods for working with IPv4 and
IPv6 network addresses, commonly called IP prefixes. The new
[`IpNet`](enum.IpNet.html), [`Ipv4Net`](struct.Ipv4Net.html), and
[`Ipv6Net`](struct.Ipv6Net.html) types build on the existing
[`IpAddr`](https://doc.rust-lang.org/std/net/enum.IpAddr.html),
[`Ipv4Addr`](https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html),
and [`Ipv6Addr`](https://doc.rust-lang.org/std/net/struct.Ipv6Addr.html)
types already provided in Rust’s standard library and align to their
design to stay consistent.

The module also provides the [`IpSubnets`](enum.IpSubnets.html),
[`Ipv4Subnets`](struct.Ipv4Subnets.html), and
[`Ipv6Subnets`](struct.Ipv6Subnets.html) types for iterating over the
subnets contained in an IP address range. The
[`IpAddrRange`](enum.IpAddrRange.html),
[`Ipv4AddrRange`](struct.Ipv4AddrRange.html), and
[`Ipv6AddrRange`](struct.Ipv6AddrRange.html) types for iterating over IP
addresses in a range. And traits that extend `Ipv4Addr` and `Ipv6Addr`
with methods for addition, subtraction, bitwise-and, and bitwise-or
operations that are missing in Rust’s standard library.

The module only uses stable features so it is guaranteed to compile
using the stable toolchain.

# <a href="#organization" class="doc-anchor">§</a>Organization

-   [`IpNet`](enum.IpNet.html) represents an IP network address, either
    IPv4 or IPv6.
-   [`Ipv4Net`](struct.Ipv4Net.html) and
    [`Ipv6Net`](struct.Ipv6Net.html) are respectively IPv4 and IPv6
    network addresses.
-   [`IpSubnets`](enum.IpSubnets.html),
    [`Ipv4Subnets`](struct.Ipv4Subnets.html), and
    [`Ipv6Subnets`](struct.Ipv6Subnets.html) are iterators that generate
    the smallest set of IP network addresses bound by an IP address
    range and minimum prefix length. These can be created using their
    constructors. They are also returned by the
    [`subnets()`](enum.IpNet.html#method.subnets) methods and used
    within the [`aggregate()`](enum.IpNet.html#method.aggregate)
    methods.
-   [`IpAddrRange`](enum.IpAddrRange.html),
    [`Ipv4AddrRange`](struct.Ipv4AddrRange.html), and
    [`Ipv6AddrRange`](struct.Ipv6AddrRange.html) are iterators that
    generate IP addresses. These can be created using their
    constructors. They are also returned by the
    [`hosts()`](enum.IpNet.html#method.hosts) methods.
-   The [`IpAdd`](trait.IpAdd.html), [`IpSub`](trait.IpSub.html),
    [`IpBitAnd`](trait.IpBitAnd.html), [`IpBitOr`](trait.IpBitOr.html)
    traits extend the
    [`Ipv4Addr`](https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html)
    and
    [`Ipv6Addr`](https://doc.rust-lang.org/std/net/struct.Ipv6Addr.html)
    types with methods to perform these operations.

# <a href="#serde-support" class="doc-anchor">§</a>Serde support

This library comes with support for [serde](https://serde.rs) but it’s
not enabled by default. Use the `serde`
[feature](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section)
to enable.

``` language-toml
[dependencies]
ipnet = { version = "2", features = ["serde"] }
```

For human readable formats (e.g. JSON) the `IpNet`, `Ipv4Net`, and
`Ipv6Net` types will serialize to their `Display` strings.

For compact binary formats (e.g. Bincode) the `Ipv4Net` and `Ipv6Net`
types will serialize to a string of 5 and 17 bytes that consist of the
network address octects followed by the prefix length. The `IpNet` type
will serialize to an Enum with the V4 or V6 variant index prepending the
above string of 5 or 17 bytes.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AddrParseError.html" class="struct"
title="struct ipnet::AddrParseError">AddrParseError</a>  
An error which can be returned when parsing an IP network address.

<a href="struct.Ipv4AddrRange.html" class="struct"
title="struct ipnet::Ipv4AddrRange">Ipv4AddrRange</a>  
An `Iterator` over a range of IPv4 addresses.

<a href="struct.Ipv4Net.html" class="struct"
title="struct ipnet::Ipv4Net">Ipv4Net</a>  
An IPv4 network address.

<a href="struct.Ipv4Subnets.html" class="struct"
title="struct ipnet::Ipv4Subnets">Ipv4Subnets</a>  
An `Iterator` that generates IPv4 network addresses.

<a href="struct.Ipv6AddrRange.html" class="struct"
title="struct ipnet::Ipv6AddrRange">Ipv6AddrRange</a>  
An `Iterator` over a range of IPv6 addresses.

<a href="struct.Ipv6Net.html" class="struct"
title="struct ipnet::Ipv6Net">Ipv6Net</a>  
An IPv6 network address.

<a href="struct.Ipv6Subnets.html" class="struct"
title="struct ipnet::Ipv6Subnets">Ipv6Subnets</a>  
An `Iterator` that generates IPv6 network addresses.

<a href="struct.PrefixLenError.html" class="struct"
title="struct ipnet::PrefixLenError">PrefixLenError</a>  
An error which can be returned when the prefix length is invalid.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.IpAddrRange.html" class="enum"
title="enum ipnet::IpAddrRange">IpAddrRange</a>  
An `Iterator` over a range of IP addresses, either IPv4 or IPv6.

<a href="enum.IpNet.html" class="enum"
title="enum ipnet::IpNet">IpNet</a>  
An IP network address, either IPv4 or IPv6.

<a href="enum.IpSubnets.html" class="enum"
title="enum ipnet::IpSubnets">IpSubnets</a>  
An `Iterator` that generates IP network addresses, either IPv4 or IPv6.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.IpAdd.html" class="trait"
title="trait ipnet::IpAdd">IpAdd</a>  
Provides a `saturating_add()` method for `Ipv4Addr` and `Ipv6Addr`.

<a href="trait.IpBitAnd.html" class="trait"
title="trait ipnet::IpBitAnd">IpBitAnd</a>  
Provides a `bitand()` method for `Ipv4Addr` and `Ipv6Addr`.

<a href="trait.IpBitOr.html" class="trait"
title="trait ipnet::IpBitOr">IpBitOr</a>  
Provides a `bitor()` method for `Ipv4Addr` and `Ipv6Addr`.

<a href="trait.IpSub.html" class="trait"
title="trait ipnet::IpSub">IpSub</a>  
Provides a `saturating_sub()` method for `Ipv4Addr` and `Ipv6Addr`.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.ip_mask_to_prefix.html" class="fn"
title="fn ipnet::ip_mask_to_prefix">ip_mask_to_prefix</a>  
Converts a `IpAddr` network mask into a prefix.

<a href="fn.ipv4_mask_to_prefix.html" class="fn"
title="fn ipnet::ipv4_mask_to_prefix">ipv4_mask_to_prefix</a>  
Converts a `Ipv4Addr` network mask into a prefix.

<a href="fn.ipv6_mask_to_prefix.html" class="fn"
title="fn ipnet::ipv6_mask_to_prefix">ipv6_mask_to_prefix</a>  
Converts a `Ipv6Addr` network mask into a prefix.
