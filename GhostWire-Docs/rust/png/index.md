Crate png Copy item path

<span class="sub-heading"><a href="../src/png/lib.rs.html#1-96" class="src">Source</a>
</span>

Expand description

# <a href="#png-encoder-and-decoder" class="doc-anchor">§</a>PNG encoder and decoder

This crate contains a PNG encoder and decoder. It supports reading of
single lines or whole frames.

## <a href="#the-decoder" class="doc-anchor">§</a>The decoder

The most important types for decoding purposes are
[`Decoder`](struct.Decoder.html "struct png::Decoder") and
[`Reader`](struct.Reader.html "struct png::Reader"). They both wrap a
[`std::io::Read`](https://doc.rust-lang.org/1.93.1/std/io/trait.Read.html "trait std::io::Read").
`Decoder` serves as a builder for `Reader`. Calling
[`Decoder::read_info`](struct.Decoder.html#method.read_info "method png::Decoder::read_info")
reads from the `Read` until the image data is reached.

### <a href="#using-the-decoder" class="doc-anchor">§</a>Using the decoder

``` {.rust .rust-example-rendered}
use std::fs::File;
use std::io::BufReader;
// The decoder is a build for reader and can be used to set various decoding options
// via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
let decoder = png::Decoder::new(BufReader::new(File::open("tests/pngsuite/basi0g01.png").unwrap()));
let mut reader = decoder.read_info().unwrap();
// Allocate the output buffer.
let mut buf = vec![0; reader.output_buffer_size().unwrap()];
// Read the next frame. An APNG might contain multiple frames.
let info = reader.next_frame(&mut buf).unwrap();
// Grab the bytes of the image.
let bytes = &buf[..info.buffer_size()];
// Inspect more details of the last read frame.
let in_animation = reader.info().frame_control.is_some();
```

## <a href="#encoder" class="doc-anchor">§</a>Encoder

### <a href="#using-the-encoder" class="doc-anchor">§</a>Using the encoder

``` {.rust .rust-example-rendered}
// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

let path = Path::new(r"/path/to/image.png");
let file = File::create(path).unwrap();
let ref mut w = BufWriter::new(file);

let mut encoder = png::Encoder::new(w, 2, 1); // Width is 2 pixels and height is 1.
encoder.set_color(png::ColorType::Rgba);
encoder.set_depth(png::BitDepth::Eight);
encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
    (0.31270, 0.32900),
    (0.64000, 0.33000),
    (0.30000, 0.60000),
    (0.15000, 0.06000)
);
encoder.set_source_chromaticities(source_chromaticities);
let mut writer = encoder.write_header().unwrap();

let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
writer.write_image_data(&data).unwrap(); // Save
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="chunk/index.html" class="mod" title="mod png::chunk">chunk</a>  
Chunk types and functions

<a href="text_metadata/index.html" class="mod"
title="mod png::text_metadata">text_metadata</a>  
Text chunks (tEXt/zTXt/iTXt) structs and functions

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Adam7Info.html" class="struct"
title="struct png::Adam7Info">Adam7Info</a>  
Describes which stage of [the Adam7
algorithm](https://en.wikipedia.org/wiki/Adam7_algorithm) applies to a
decoded row.

<a href="struct.AnimationControl.html" class="struct"
title="struct png::AnimationControl">AnimationControl</a>  
Animation control information

<a href="struct.CodingIndependentCodePoints.html" class="struct"
title="struct png::CodingIndependentCodePoints">CodingIndependentCodePoints</a>  
Coding-independent code points (cICP) specify the color space
(primaries), transfer function, matrix coefficients and scaling factor
of the image using the code points specified in
[ITU-T-H.273](https://www.itu.int/rec/T-REC-H.273).

<a href="struct.ContentLightLevelInfo.html" class="struct"
title="struct png::ContentLightLevelInfo">ContentLightLevelInfo</a>  
Content light level information of HDR content.

<a href="struct.DecodeOptions.html" class="struct"
title="struct png::DecodeOptions">DecodeOptions</a>  
Decoder configuration options

<a href="struct.Decoder.html" class="struct"
title="struct png::Decoder">Decoder</a>  
PNG Decoder

<a href="struct.Encoder.html" class="struct"
title="struct png::Encoder">Encoder</a>  
PNG Encoder.

<a href="struct.FrameControl.html" class="struct"
title="struct png::FrameControl">FrameControl</a>  
Frame control information

<a href="struct.Info.html" class="struct"
title="struct png::Info">Info</a>  
PNG info struct

<a href="struct.InterlacedRow.html" class="struct"
title="struct png::InterlacedRow">InterlacedRow</a>  
A row of data with interlace information attached.

<a href="struct.Limits.html" class="struct"
title="struct png::Limits">Limits</a>  
Limits on the resources the `Decoder` is allowed too use

<a href="struct.MasteringDisplayColorVolume.html" class="struct"
title="struct png::MasteringDisplayColorVolume">MasteringDisplayColorVolume</a>  
Mastering Display Color Volume (mDCV) used at the point of content
creation, as specified in
[SMPTE-ST-2086](https://ieeexplore.ieee.org/stamp/stamp.jsp?arnumber=8353899).

<a href="struct.OutputInfo.html" class="struct"
title="struct png::OutputInfo">OutputInfo</a>  
Output info.

<a href="struct.ParameterError.html" class="struct"
title="struct png::ParameterError">ParameterError</a>  
<a href="struct.PixelDimensions.html" class="struct"
title="struct png::PixelDimensions">PixelDimensions</a>  
Pixel dimensions information

<a href="struct.Reader.html" class="struct"
title="struct png::Reader">Reader</a>  
PNG reader (mostly high-level interface)

<a href="struct.ScaledFloat.html" class="struct"
title="struct png::ScaledFloat">ScaledFloat</a>  
An unsigned integer scaled version of a floating point value, equivalent
to an integer quotient with fixed denominator (100\_000)).

<a href="struct.SourceChromaticities.html" class="struct"
title="struct png::SourceChromaticities">SourceChromaticities</a>  
Chromaticities of the color space primaries

<a href="struct.StreamWriter.html" class="struct"
title="struct png::StreamWriter">StreamWriter</a>  
Streaming PNG writer

<a href="struct.StreamingDecoder.html" class="struct"
title="struct png::StreamingDecoder">StreamingDecoder</a>  
PNG StreamingDecoder (low-level interface)

<a href="struct.Transformations.html" class="struct"
title="struct png::Transformations">Transformations</a>  
Output transformations

<a href="struct.UnfilterBuf.html" class="struct"
title="struct png::UnfilterBuf">UnfilterBuf</a>  
An inplace buffer for decompression and filtering of PNG rowlines.

<a href="struct.UnfilterRegion.html" class="struct"
title="struct png::UnfilterRegion">UnfilterRegion</a>  
A region into a buffer utilized as a
[`UnfilterBuf`](struct.UnfilterBuf.html "struct png::UnfilterBuf").

<a href="struct.Writer.html" class="struct"
title="struct png::Writer">Writer</a>  
PNG writer

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Adam7Variant.html" class="enum"
title="enum png::Adam7Variant">Adam7Variant</a>  
The algorithm to use when progressively filling pixel data from Adam7
interlaced passes.

<a href="enum.BitDepth.html" class="enum"
title="enum png::BitDepth">BitDepth</a>  
Bit depth of the PNG file. Specifies the number of bits per sample.

<a href="enum.BlendOp.html" class="enum"
title="enum png::BlendOp">BlendOp</a>  
How pixels are written into the buffer.

<a href="enum.ColorType.html" class="enum"
title="enum png::ColorType">ColorType</a>  
Describes how a pixel is encoded.

<a href="enum.Compression.html" class="enum"
title="enum png::Compression">Compression</a>  
The type and strength of applied compression.

<a href="enum.Decoded.html" class="enum"
title="enum png::Decoded">Decoded</a>  
Result of the decoding process

<a href="enum.DecodingError.html" class="enum"
title="enum png::DecodingError">DecodingError</a>  
Any kind of error during PNG decoding.

<a href="enum.DeflateCompression.html" class="enum"
title="enum png::DeflateCompression">DeflateCompression</a>  
Advanced compression settings with more customization options than
[Compression](enum.Compression.html "enum png::Compression").

<a href="enum.DisposeOp.html" class="enum"
title="enum png::DisposeOp">DisposeOp</a>  
How to reset buffer of an animated png (APNG) at the end of a frame.

<a href="enum.EncodingError.html" class="enum"
title="enum png::EncodingError">EncodingError</a>  
<a href="enum.Filter.html" class="enum"
title="enum png::Filter">Filter</a>  
The byte level filter applied to scanlines to prepare them for
compression.

<a href="enum.InterlaceInfo.html" class="enum"
title="enum png::InterlaceInfo">InterlaceInfo</a>  
Describes which interlacing algorithm applies to a decoded row.

<a href="enum.SrgbRenderingIntent.html" class="enum"
title="enum png::SrgbRenderingIntent">SrgbRenderingIntent</a>  
The rendering intent for an sRGB image.

<a href="enum.Unit.html" class="enum" title="enum png::Unit">Unit</a>  
Physical unit of the pixel dimensions

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.expand_interlaced_row.html" class="fn"
title="fn png::expand_interlaced_row">expand_interlaced_row</a>  
Copies pixels from `interlaced_row` into the right location in `img`.

<a href="fn.splat_interlaced_row.html" class="fn"
title="fn png::splat_interlaced_row">splat_interlaced_row</a>  
Expand pass, but also ensure that after each pass the whole image has
been initialized up to the data available. In constrast to `expand_pass`
there are no holes left in the image.
