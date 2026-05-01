Crate rtnetlink Copy item path

<span class="sub-heading"><a href="../src/rtnetlink/lib.rs.html#3-40" class="src">Source</a>
</span>

Expand description

This crate provides methods to manipulate networking resources (links,
addresses, arp tables, route tables) via the netlink protocol.

# Modules<a href="#modules" class="anchor">§</a>

<a href="constants/index.html" class="mod"
title="mod rtnetlink::constants">constants</a>

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.try_nl.html" class="macro"
title="macro rtnetlink::try_nl">try_nl</a>

<a href="macro.try_rtnl.html" class="macro"
title="macro rtnetlink::try_rtnl">try_rtnl</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AddressAddRequest.html" class="struct"
title="struct rtnetlink::AddressAddRequest">AddressAddRequest</a>  
A request to create a new address. This is equivalent to the
`ip address add` commands.

<a href="struct.AddressDelRequest.html" class="struct"
title="struct rtnetlink::AddressDelRequest">AddressDelRequest</a>  
<a href="struct.AddressGetRequest.html" class="struct"
title="struct rtnetlink::AddressGetRequest">AddressGetRequest</a>  
<a href="struct.AddressHandle.html" class="struct"
title="struct rtnetlink::AddressHandle">AddressHandle</a>  
<a href="struct.BondAddRequest.html" class="struct"
title="struct rtnetlink::BondAddRequest">BondAddRequest</a>  
<a href="struct.BondPortSetRequest.html" class="struct"
title="struct rtnetlink::BondPortSetRequest">BondPortSetRequest</a>  
<a href="struct.Handle.html" class="struct"
title="struct rtnetlink::Handle">Handle</a>  
<a href="struct.LinkAddRequest.html" class="struct"
title="struct rtnetlink::LinkAddRequest">LinkAddRequest</a>  
A request to create a new link. This is equivalent to the `ip link add`
commands.

<a href="struct.LinkDelPropRequest.html" class="struct"
title="struct rtnetlink::LinkDelPropRequest">LinkDelPropRequest</a>  
<a href="struct.LinkDelRequest.html" class="struct"
title="struct rtnetlink::LinkDelRequest">LinkDelRequest</a>  
<a href="struct.LinkGetRequest.html" class="struct"
title="struct rtnetlink::LinkGetRequest">LinkGetRequest</a>  
<a href="struct.LinkHandle.html" class="struct"
title="struct rtnetlink::LinkHandle">LinkHandle</a>  
<a href="struct.LinkNewPropRequest.html" class="struct"
title="struct rtnetlink::LinkNewPropRequest">LinkNewPropRequest</a>  
<a href="struct.LinkSetRequest.html" class="struct"
title="struct rtnetlink::LinkSetRequest">LinkSetRequest</a>  
<a href="struct.NeighbourAddRequest.html" class="struct"
title="struct rtnetlink::NeighbourAddRequest">NeighbourAddRequest</a>  
<a href="struct.NeighbourDelRequest.html" class="struct"
title="struct rtnetlink::NeighbourDelRequest">NeighbourDelRequest</a>  
<a href="struct.NeighbourGetRequest.html" class="struct"
title="struct rtnetlink::NeighbourGetRequest">NeighbourGetRequest</a>  
<a href="struct.NeighbourHandle.html" class="struct"
title="struct rtnetlink::NeighbourHandle">NeighbourHandle</a>  
<a href="struct.NetworkNamespace.html" class="struct"
title="struct rtnetlink::NetworkNamespace">NetworkNamespace</a>  
<a href="struct.QDiscDelRequest.html" class="struct"
title="struct rtnetlink::QDiscDelRequest">QDiscDelRequest</a>  
<a href="struct.QDiscGetRequest.html" class="struct"
title="struct rtnetlink::QDiscGetRequest">QDiscGetRequest</a>  
<a href="struct.QDiscHandle.html" class="struct"
title="struct rtnetlink::QDiscHandle">QDiscHandle</a>  
<a href="struct.QDiscNewRequest.html" class="struct"
title="struct rtnetlink::QDiscNewRequest">QDiscNewRequest</a>  
<a href="struct.QosMapping.html" class="struct"
title="struct rtnetlink::QosMapping">QosMapping</a>  
A quality-of-service mapping between the internal priority `from` to the
external vlan priority `to`.

<a href="struct.RouteAddRequest.html" class="struct"
title="struct rtnetlink::RouteAddRequest">RouteAddRequest</a>  
A request to create a new route. This is equivalent to the
`ip route add` commands.

<a href="struct.RouteDelRequest.html" class="struct"
title="struct rtnetlink::RouteDelRequest">RouteDelRequest</a>  
<a href="struct.RouteGetRequest.html" class="struct"
title="struct rtnetlink::RouteGetRequest">RouteGetRequest</a>  
<a href="struct.RouteHandle.html" class="struct"
title="struct rtnetlink::RouteHandle">RouteHandle</a>  
<a href="struct.RuleAddRequest.html" class="struct"
title="struct rtnetlink::RuleAddRequest">RuleAddRequest</a>  
A request to create a new rule. This is equivalent to the `ip rule add`
command.

<a href="struct.RuleDelRequest.html" class="struct"
title="struct rtnetlink::RuleDelRequest">RuleDelRequest</a>  
<a href="struct.RuleGetRequest.html" class="struct"
title="struct rtnetlink::RuleGetRequest">RuleGetRequest</a>  
<a href="struct.RuleHandle.html" class="struct"
title="struct rtnetlink::RuleHandle">RuleHandle</a>  
<a href="struct.TrafficChainGetRequest.html" class="struct"
title="struct rtnetlink::TrafficChainGetRequest">TrafficChainGetRequest</a>  
<a href="struct.TrafficChainHandle.html" class="struct"
title="struct rtnetlink::TrafficChainHandle">TrafficChainHandle</a>  
<a href="struct.TrafficClassGetRequest.html" class="struct"
title="struct rtnetlink::TrafficClassGetRequest">TrafficClassGetRequest</a>  
<a href="struct.TrafficClassHandle.html" class="struct"
title="struct rtnetlink::TrafficClassHandle">TrafficClassHandle</a>  
<a href="struct.TrafficFilterGetRequest.html" class="struct"
title="struct rtnetlink::TrafficFilterGetRequest">TrafficFilterGetRequest</a>  
<a href="struct.TrafficFilterHandle.html" class="struct"
title="struct rtnetlink::TrafficFilterHandle">TrafficFilterHandle</a>  
<a href="struct.TrafficFilterNewRequest.html" class="struct"
title="struct rtnetlink::TrafficFilterNewRequest">TrafficFilterNewRequest</a>  
<a href="struct.VxlanAddRequest.html" class="struct"
title="struct rtnetlink::VxlanAddRequest">VxlanAddRequest</a>  
A request to create a new vxlan link. This is equivalent to
`ip link add NAME vxlan id ID ...` commands. It provides methods to
customize the creation of the vxlan interface It provides almost all
parameters that are listed by `man ip link`.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum rtnetlink::Error">Error</a>  
<a href="enum.IpVersion.html" class="enum"
title="enum rtnetlink::IpVersion">IpVersion</a>  
Internet Protocol (IP) version.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.NETNS_PATH.html" class="constant"
title="constant rtnetlink::NETNS_PATH">NETNS_PATH</a>

<a href="constant.NONE_FS.html" class="constant"
title="constant rtnetlink::NONE_FS">NONE_FS</a>

<a href="constant.SELF_NS_PATH.html" class="constant"
title="constant rtnetlink::SELF_NS_PATH">SELF_NS_PATH</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.new_connection.html" class="fn"
title="fn rtnetlink::new_connection">new_connection</a>

<a href="fn.new_connection_with_socket.html" class="fn"
title="fn rtnetlink::new_connection_with_socket">new_connection_with_socket</a>
