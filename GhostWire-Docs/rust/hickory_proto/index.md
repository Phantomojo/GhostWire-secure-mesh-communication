Crate hickory\_proto Copy item path

<span class="sub-heading"><a href="../src/hickory_proto/lib.rs.html#9-149" class="src">Source</a>
</span>

Expand description

Hickory DNS Protocol library

# Modules<a href="#modules" class="anchor">§</a>

<a href="multicast/index.html" class="mod"
title="mod hickory_proto::multicast">multicast</a>  
Multicast protocol related components for DNS

<a href="op/index.html" class="mod" title="mod hickory_proto::op">op</a>  
Operations to send with a `Client` or server, e.g. `Query`, `Message`,
or `UpdateMessage` can be used together to either query or update
resource records sets.

<a href="rr/index.html" class="mod" title="mod hickory_proto::rr">rr</a>  
Resource record related components, e.g. `Name` aka label, `Record`,
`RData`, …

<a href="runtime/index.html" class="mod"
title="mod hickory_proto::runtime">runtime</a>  
Abstractions to deal with different async runtimes.

<a href="serialize/index.html" class="mod"
title="mod hickory_proto::serialize">serialize</a>  
Contains serialization libraries for `binary` and text, `txt`.

<a href="tcp/index.html" class="mod"
title="mod hickory_proto::tcp">tcp</a>  
TCP protocol related components for DNS

<a href="udp/index.html" class="mod"
title="mod hickory_proto::udp">udp</a>  
UDP protocol related components for DNS

<a href="xfer/index.html" class="mod"
title="mod hickory_proto::xfer">xfer</a>  
DNS high level transit implementations.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ForwardData.html" class="struct"
title="struct hickory_proto::ForwardData">ForwardData</a>  
Data needed to process a SOA-record-based referral.

<a href="struct.ForwardNSData.html" class="struct"
title="struct hickory_proto::ForwardNSData">ForwardNSData</a>  
Data needed to process a NS-record-based referral.

<a href="struct.ProtoError.html" class="struct"
title="struct hickory_proto::ProtoError">ProtoError</a>  
The error type for errors that get returned in the crate

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ProtoErrorKind.html" class="enum"
title="enum hickory_proto::ProtoErrorKind">ProtoErrorKind</a>  
The error kind for errors that get returned in the crate
