Crate if\_watch Copy item path

<span class="sub-heading"><a href="../src/if_watch/lib.rs.html#1-120" class="src">Source</a>
</span>

Expand description

IP address watching.

# Modules<a href="#modules" class="anchor">§</a>

<a href="tokio/index.html" class="mod"
title="mod if_watch::tokio">tokio</a>  
An interface watcher that uses `netlink`’s
[`TokioSocket`](../netlink_sys/tokio/struct.TokioSocket.html "struct netlink_sys::tokio::TokioSocket")

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Ipv4Net.html" class="struct"
title="struct if_watch::Ipv4Net">Ipv4Net</a>  
An IPv4 network address.

<a href="struct.Ipv6Net.html" class="struct"
title="struct if_watch::Ipv6Net">Ipv6Net</a>  
An IPv6 network address.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.IfEvent.html" class="enum"
title="enum if_watch::IfEvent">IfEvent</a>  
An address change event.

<a href="enum.IpNet.html" class="enum"
title="enum if_watch::IpNet">IpNet</a>  
An IP network address, either IPv4 or IPv6.
