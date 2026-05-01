Crate rtcp Copy item path

<span class="sub-heading"><a href="../src/rtcp/lib.rs.html#1-59" class="src">Source</a>
</span>

Expand description

Package rtcp implements encoding and decoding of RTCP packets according
to RFCs 3550 and 5506.

RTCP is a sister protocol of the Real-time Transport Protocol (RTP). Its
basic functionality and packet structure is defined in RFC 3550. RTCP
provides out-of-band statistics and control information for an RTP
session. It partners with RTP in the delivery and packaging of
multimedia data, but does not transport any media data itself.

The primary function of RTCP is to provide feedback on the quality of
service (QoS) in media distribution by periodically sending statistics
information such as transmitted octet and packet counts, packet loss,
packet delay variation, and round-trip delay time to participants in a
streaming multimedia session. An application may use this information to
control quality of service parameters, perhaps by limiting flow, or
using a different codec.

Decoding RTCP packets:

``` language-nobuild
     let pkt = rtcp::unmarshal(&rtcp_data).unwrap();

     if let Some(e) = pkt
          .as_any()
          .downcast_ref::<PictureLossIndication>()
      {

      }
     else if let Some(e) = packet
          .as_any()
          .downcast_ref::<Goodbye>(){}
     ....
```

Encoding RTCP packets:

``` language-nobuild
     let pkt = PictureLossIndication{
         sender_ssrc: sender_ssrc,
         media_ssrc: media_ssrc
     };

     let pli_data = pkt.marshal().unwrap();
     // ...
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="compound_packet/index.html" class="mod"
title="mod rtcp::compound_packet">compound_packet</a>

<a href="extended_report/index.html" class="mod"
title="mod rtcp::extended_report">extended_report</a>

<a href="goodbye/index.html" class="mod"
title="mod rtcp::goodbye">goodbye</a>

<a href="header/index.html" class="mod"
title="mod rtcp::header">header</a>

<a href="packet/index.html" class="mod"
title="mod rtcp::packet">packet</a>

<a href="payload_feedbacks/index.html" class="mod"
title="mod rtcp::payload_feedbacks">payload_feedbacks</a>

<a href="raw_packet/index.html" class="mod"
title="mod rtcp::raw_packet">raw_packet</a>

<a href="receiver_report/index.html" class="mod"
title="mod rtcp::receiver_report">receiver_report</a>

<a href="reception_report/index.html" class="mod"
title="mod rtcp::reception_report">reception_report</a>

<a href="sender_report/index.html" class="mod"
title="mod rtcp::sender_report">sender_report</a>

<a href="source_description/index.html" class="mod"
title="mod rtcp::source_description">source_description</a>

<a href="transport_feedbacks/index.html" class="mod"
title="mod rtcp::transport_feedbacks">transport_feedbacks</a>

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum rtcp::Error">Error</a>
