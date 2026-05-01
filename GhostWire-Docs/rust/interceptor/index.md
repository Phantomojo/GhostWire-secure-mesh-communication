Crate interceptor Copy item path

<span class="sub-heading"><a href="../src/interceptor/lib.rs.html#1-228" class="src">Source</a>
</span>

# Modules<a href="#modules" class="anchor">§</a>

<a href="chain/index.html" class="mod"
title="mod interceptor::chain">chain</a>

<a href="mock/index.html" class="mod"
title="mod interceptor::mock">mock</a>

<a href="nack/index.html" class="mod"
title="mod interceptor::nack">nack</a>

<a href="noop/index.html" class="mod"
title="mod interceptor::noop">noop</a>

<a href="registry/index.html" class="mod"
title="mod interceptor::registry">registry</a>

<a href="report/index.html" class="mod"
title="mod interceptor::report">report</a>

<a href="stats/index.html" class="mod"
title="mod interceptor::stats">stats</a>

<a href="stream_info/index.html" class="mod"
title="mod interceptor::stream_info">stream_info</a>

<a href="stream_reader/index.html" class="mod"
title="mod interceptor::stream_reader">stream_reader</a>

<a href="twcc/index.html" class="mod"
title="mod interceptor::twcc">twcc</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.RTCPReaderFn.html" class="struct"
title="struct interceptor::RTCPReaderFn">RTCPReaderFn</a>

<a href="struct.RTCPWriterFn.html" class="struct"
title="struct interceptor::RTCPWriterFn">RTCPWriterFn</a>

<a href="struct.RTPReaderFn.html" class="struct"
title="struct interceptor::RTPReaderFn">RTPReaderFn</a>

<a href="struct.RTPWriterFn.html" class="struct"
title="struct interceptor::RTPWriterFn">RTPWriterFn</a>

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum interceptor::Error">Error</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Interceptor.html" class="trait"
title="trait interceptor::Interceptor">Interceptor</a>  
Interceptor can be used to add functionality to you PeerConnections by
modifying any incoming/outgoing rtp/rtcp packets, or sending your own
packets as needed.

<a href="trait.InterceptorBuilder.html" class="trait"
title="trait interceptor::InterceptorBuilder">InterceptorBuilder</a>  
InterceptorBuilder provides an interface for constructing interceptors

<a href="trait.RTCPReader.html" class="trait"
title="trait interceptor::RTCPReader">RTCPReader</a>  
RTCPReader is used by Interceptor.bind\_rtcpreader.

<a href="trait.RTCPWriter.html" class="trait"
title="trait interceptor::RTCPWriter">RTCPWriter</a>  
RTCPWriter is used by Interceptor.bind\_rtcpwriter.

<a href="trait.RTPReader.html" class="trait"
title="trait interceptor::RTPReader">RTPReader</a>  
RTPReader is used by Interceptor.bind\_remote\_stream.

<a href="trait.RTPWriter.html" class="trait"
title="trait interceptor::RTPWriter">RTPWriter</a>  
RTPWriter is used by Interceptor.bind\_local\_stream.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Attributes.html" class="type"
title="type interceptor::Attributes">Attributes</a>

Attributes are a generic key/value store used by interceptors

<a href="type.RTCPReaderBoxFn.html" class="type"
title="type interceptor::RTCPReaderBoxFn">RTCPReaderBoxFn</a>

<a href="type.RTCPWriterBoxFn.html" class="type"
title="type interceptor::RTCPWriterBoxFn">RTCPWriterBoxFn</a>

<a href="type.RTPReaderBoxFn.html" class="type"
title="type interceptor::RTPReaderBoxFn">RTPReaderBoxFn</a>

<a href="type.RTPWriterBoxFn.html" class="type"
title="type interceptor::RTPWriterBoxFn">RTPWriterBoxFn</a>
