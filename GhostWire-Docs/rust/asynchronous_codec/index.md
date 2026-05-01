Crate asynchronous\_codec Copy item path

<span class="sub-heading"><a href="../src/asynchronous_codec/lib.rs.html#1-48"
class="src">Source</a> </span>

Expand description

Utilities for encoding and decoding frames using `async/await`.

Contains adapters to go from streams of bytes,
[`AsyncRead`](futures::io::AsyncRead) and
[`AsyncWrite`](futures::io::AsyncWrite), to framed streams implementing
[`Sink`](futures::Sink) and [`Stream`](futures::Stream). Framed streams
are also known as `transports`.

``` {.rust .rust-example-rendered}
use futures::TryStreamExt;
use futures::io::Cursor;
use asynchronous_codec::{LinesCodec, Framed};

let io = Cursor::new(Vec::new());
let mut framed = Framed::new(io, LinesCodec);

while let Some(line) = framed.try_next().await? {
    dbg!(line);
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Bytes.html" class="struct"
title="struct asynchronous_codec::Bytes">Bytes</a>  
A cheaply cloneable and sliceable chunk of contiguous memory.

<a href="struct.BytesCodec.html" class="struct"
title="struct asynchronous_codec::BytesCodec">BytesCodec</a>  
A simple codec that ships bytes around

<a href="struct.BytesMut.html" class="struct"
title="struct asynchronous_codec::BytesMut">BytesMut</a>  
A unique reference to a contiguous slice of memory.

<a href="struct.Framed.html" class="struct"
title="struct asynchronous_codec::Framed">Framed</a>  
A unified `Stream` and `Sink` interface to an underlying I/O object,
using the `Encoder` and `Decoder` traits to encode and decode frames.

<a href="struct.FramedParts.html" class="struct"
title="struct asynchronous_codec::FramedParts">FramedParts</a>  
The parts obtained from
[`Framed::into_parts`](struct.Framed.html#method.into_parts "method asynchronous_codec::Framed::into_parts").

<a href="struct.FramedRead.html" class="struct"
title="struct asynchronous_codec::FramedRead">FramedRead</a>  
A `Stream` of messages decoded from an `AsyncRead`.

<a href="struct.FramedReadParts.html" class="struct"
title="struct asynchronous_codec::FramedReadParts">FramedReadParts</a>  
The parts obtained from (FramedRead::into\_parts).

<a href="struct.FramedWrite.html" class="struct"
title="struct asynchronous_codec::FramedWrite">FramedWrite</a>  
A `Sink` of frames encoded to an `AsyncWrite`.

<a href="struct.FramedWriteParts.html" class="struct"
title="struct asynchronous_codec::FramedWriteParts">FramedWriteParts</a>  
The parts obtained from
[`FramedWrite::into_parts`](struct.FramedWrite.html#method.into_parts "method asynchronous_codec::FramedWrite::into_parts").

<a href="struct.LengthCodec.html" class="struct"
title="struct asynchronous_codec::LengthCodec">LengthCodec</a>  
A simple `Codec` implementation sending your data by prefixing it by its
length.

<a href="struct.LinesCodec.html" class="struct"
title="struct asynchronous_codec::LinesCodec">LinesCodec</a>  
A simple `Codec` implementation that splits up data into lines.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Decoder.html" class="trait"
title="trait asynchronous_codec::Decoder">Decoder</a>  
Decoding of frames via buffers, for use with `FramedRead`.

<a href="trait.Encoder.html" class="trait"
title="trait asynchronous_codec::Encoder">Encoder</a>  
Encoding of messages as bytes, for use with `FramedWrite`.
