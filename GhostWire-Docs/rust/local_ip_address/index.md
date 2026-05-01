Crate local\_ip\_address Copy item path

<span class="sub-heading"><a href="../src/local_ip_address/lib.rs.html#1-330"
class="src">Source</a> </span>

Expand description

# <a href="#local-ip-address" class="doc-anchor">§</a>Local IP Address

Retrieve system’s local IP address and Network Interfaces/Adapters on
Linux, Windows, and macOS (and other BSD-based systems).

## <a href="#usage" class="doc-anchor">§</a>Usage

Get the local IP address of your system by executing the `local_ip`
function:

``` {.rust .rust-example-rendered}
use local_ip_address::local_ip;

let my_local_ip = local_ip();

if let Ok(my_local_ip) = my_local_ip {
    println!("This is my local IP address: {:?}", my_local_ip);
} else {
    println!("Error getting local IP: {:?}", my_local_ip);
}
```

Retrieve all the available network interfaces from both, the `AF_INET`
and the `AF_INET6` family by executing the `list_afinet_netifas`
function:

``` {.rust .rust-example-rendered}
use local_ip_address::list_afinet_netifas;

let network_interfaces = list_afinet_netifas();

if let Ok(network_interfaces) = network_interfaces {
    for (name, ip) in network_interfaces.iter() {
        println!("{}:\t{:?}", name, ip);
    }
} else {
    println!("Error getting network interfaces: {:?}", network_interfaces);
}
```

Underlying approach on retrieving network interfaces or the local IP
address may differ based on the running operative system.

| OS | Approach |
|----|----|
| Linux | Establishes a Netlink socket interchange to retrieve network interfaces |
| BSD-based & Android | Uses of `getifaddrs` to retrieve network interfaces |
| Windows | Consumes Win32 API’s to retrieve the network adapters table |

Supported BSD-based systems include:

-   macOS
-   FreeBSD
-   OpenBSD
-   NetBSD
-   DragonFly

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::`<a href="linux/index.html" class="mod"
title="mod local_ip_address::linux"><code>linux</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="linux/index.html" class="mod"
title="mod local_ip_address::linux">linux</a>

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum local_ip_address::Error">Error</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.local_ip.html" class="fn"
title="fn local_ip_address::local_ip">local_ip</a>  
Retrieves the local IPv4 address of the machine in the local network
from the `AF_INET` family.

<a href="fn.local_ipv6.html" class="fn"
title="fn local_ip_address::local_ipv6">local_ipv6</a>  
Retrieves the local IPv6 address of the machine in the local network
from the `AF_INET6` family.
