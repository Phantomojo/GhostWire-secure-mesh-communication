Crate image Copy item path

<span class="sub-heading"><a href="../src/image/lib.rs.html#1-307" class="src">Source</a>
</span>

Expand description

# <a href="#overview" class="doc-anchor">§</a>Overview

This crate provides native rust implementations of image encoding and
decoding as well as some basic image manipulation functions. Additional
documentation can currently also be found in the [README.md file which
is most easily viewed on
github](https://github.com/image-rs/image/blob/main/README.md).

There are two core problems for which this library provides solutions: a
unified interface for image encodings and simple generic buffers for
their content. It’s possible to use either feature without the other.
The focus is on a small and stable set of common operations that can be
supplemented by other specialized crates. The library also prefers safe
solutions with few dependencies.

# <a href="#high-level-api" class="doc-anchor">§</a>High level API

Load images using
[`ImageReader`](struct.ImageReader.html "struct image::ImageReader"):

``` {.rust .rust-example-rendered}
use std::io::Cursor;
use image::ImageReader;

let img = ImageReader::open("myimage.png")?.decode()?;
let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
```

And save them using [`save`](enum.DynamicImage.html#method.save) or
[`write_to`](enum.DynamicImage.html#method.write_to) methods:

``` {.rust .rust-example-rendered}
img.save("empty.jpg")?;

let mut bytes: Vec<u8> = Vec::new();
img2.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)?;
```

With default features, the crate includes support for [many common image
formats](codecs/index.html#supported-formats).

# <a href="#image-buffers" class="doc-anchor">§</a>Image buffers

The two main types for storing images:

-   [`ImageBuffer`](struct.ImageBuffer.html) which holds statically
    typed image contents.
-   [`DynamicImage`](enum.DynamicImage.html) which is an enum over the
    supported `ImageBuffer` formats and supports conversions between
    them.

As well as a few more specialized options:

-   [`GenericImage`](trait.GenericImage.html) trait for a mutable image
    buffer.
-   [`GenericImageView`](trait.GenericImageView.html) trait for read
    only references to a `GenericImage`.
-   [`flat`](flat/index.html) module containing types for
    interoperability with generic channel matrices and foreign
    interfaces.

# <a href="#low-level-encodingdecoding-api" class="doc-anchor">§</a>Low level encoding/decoding API

Implementations of [`ImageEncoder`](trait.ImageEncoder.html) provides
low level control over encoding:

``` {.rust .rust-example-rendered}
let encoder = JpegEncoder::new_with_quality(&mut writer, 95);
img.write_with_encoder(encoder)?;
```

While [`ImageDecoder`](trait.ImageDecoder.html) and
[`ImageDecoderRect`](trait.ImageDecoderRect.html) give access to more
advanced decoding options:

``` {.rust .rust-example-rendered}
let decoder = PngDecoder::new(&mut reader)?;
let icc = decoder.icc_profile();
let img = DynamicImage::from_decoder(decoder)?;
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::error::`<a href="error/enum.ImageError.html" class="enum"
title="enum image::error::ImageError"><code>ImageError</code></a>`;`

`pub use crate::error::`<a href="error/type.ImageResult.html" class="type"
title="type image::error::ImageResult"><code>ImageResult</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="buffer/index.html" class="mod"
title="mod image::buffer">buffer</a>  
Iterators and other auxiliary structure for the `ImageBuffer` type.

<a href="codecs/index.html" class="mod"
title="mod image::codecs">codecs</a>  
Encoding and decoding for various image file formats.

<a href="error/index.html" class="mod"
title="mod image::error">error</a>  
Contains detailed error representation.

<a href="flat/index.html" class="mod" title="mod image::flat">flat</a>  
Image representations for ffi.

<a href="hooks/index.html" class="mod"
title="mod image::hooks">hooks</a>  
This module provides a way to register decoding hooks for image formats
not directly supported by this crate.

<a href="imageops/index.html" class="mod"
title="mod image::imageops">imageops</a>  
Image Processing Functions

<a href="io/index.html" class="mod" title="mod image::io">io</a>  
Deprecated io module the original io module has been renamed to
`image_reader`. This is going to be internal. Input and output of
images.

<a href="math/index.html" class="mod" title="mod image::math">math</a>  
Mathematical helper functions and types.

<a href="metadata/index.html" class="mod"
title="mod image::metadata">metadata</a>  
Types describing image metadata

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ConvertColorOptions.html" class="struct"
title="struct image::ConvertColorOptions">ConvertColorOptions</a>  
Inputs to
[`ImageBuffer::copy_from_color_space`](struct.ImageBuffer.html#method.copy_from_color_space "method image::ImageBuffer::copy_from_color_space").

<a href="struct.Delay.html" class="struct"
title="struct image::Delay">Delay</a>  
The delay of a frame relative to the previous one.

<a href="struct.FlatSamples.html" class="struct"
title="struct image::FlatSamples">FlatSamples</a>  
A flat buffer over a (multi channel) image.

<a href="struct.Frame.html" class="struct"
title="struct image::Frame">Frame</a>  
A single animation frame

<a href="struct.Frames.html" class="struct"
title="struct image::Frames">Frames</a>  
An implementation dependent iterator, reading the frames as requested

<a href="struct.ImageBuffer.html" class="struct"
title="struct image::ImageBuffer">ImageBuffer</a>  
Generic image buffer

<a href="struct.ImageReader.html" class="struct"
title="struct image::ImageReader">ImageReader</a>  
A multi-format image reader.

<a href="struct.LimitSupport.html" class="struct"
title="struct image::LimitSupport">LimitSupport</a>  
Set of supported strict limits for a decoder.

<a href="struct.Limits.html" class="struct"
title="struct image::Limits">Limits</a>  
Resource limits for decoding.

<a href="struct.Luma.html" class="struct"
title="struct image::Luma">Luma</a>  
Grayscale colors.

<a href="struct.LumaA.html" class="struct"
title="struct image::LumaA">LumaA</a>  
Grayscale colors + alpha channel

<a href="struct.Pixels.html" class="struct"
title="struct image::Pixels">Pixels</a>  
Immutable pixel iterator

<a href="struct.Rgb.html" class="struct"
title="struct image::Rgb">Rgb</a>  
RGB colors.

<a href="struct.Rgba.html" class="struct"
title="struct image::Rgba">Rgba</a>  
RGB colors + alpha channel

<a href="struct.SubImage.html" class="struct"
title="struct image::SubImage">SubImage</a>  
A View into another image

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ColorType.html" class="enum"
title="enum image::ColorType">ColorType</a>  
An enumeration over supported color types and bit depths

<a href="enum.DynamicImage.html" class="enum"
title="enum image::DynamicImage">DynamicImage</a>  
A Dynamic Image

<a href="enum.ExtendedColorType.html" class="enum"
title="enum image::ExtendedColorType">ExtendedColorType</a>  
An enumeration of color types encountered in image formats.

<a href="enum.ImageFormat.html" class="enum"
title="enum image::ImageFormat">ImageFormat</a>  
An enumeration of supported image formats. Not all formats support both
encoding and decoding.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AnimationDecoder.html" class="trait"
title="trait image::AnimationDecoder">AnimationDecoder</a>  
`AnimationDecoder` trait

<a href="trait.EncodableLayout.html" class="trait"
title="trait image::EncodableLayout">EncodableLayout</a>  
Types which are safe to treat as an immutable byte slice in a pixel
layout for image encoding.

<a href="trait.GenericImage.html" class="trait"
title="trait image::GenericImage">GenericImage</a>  
A trait for manipulating images.

<a href="trait.GenericImageView.html" class="trait"
title="trait image::GenericImageView">GenericImageView</a>  
Trait to inspect an image.

<a href="trait.ImageDecoder.html" class="trait"
title="trait image::ImageDecoder">ImageDecoder</a>  
The trait that all decoders implement

<a href="trait.ImageDecoderRect.html" class="trait"
title="trait image::ImageDecoderRect">ImageDecoderRect</a>  
Specialized image decoding not be supported by all formats

<a href="trait.ImageEncoder.html" class="trait"
title="trait image::ImageEncoder">ImageEncoder</a>  
The trait all encoders implement

<a href="trait.Pixel.html" class="trait"
title="trait image::Pixel">Pixel</a>  
A generalized pixel.

<a href="trait.PixelWithColorType.html" class="trait"
title="trait image::PixelWithColorType">PixelWithColorType</a>  
The pixel with an associated `ColorType`. Not all possible pixels
represent one of the predefined `ColorType`s.

<a href="trait.Primitive.html" class="trait"
title="trait image::Primitive">Primitive</a>  
The type of each channel in a pixel. For example, this can be `u8`,
`u16`, `f32`.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.guess_format.html" class="fn"
title="fn image::guess_format">guess_format</a>  
Guess image format from memory block

<a href="fn.image_dimensions.html" class="fn"
title="fn image::image_dimensions">image_dimensions</a>  
Read a tuple containing the (width, height) of the image located at the
specified path. This is faster than fully loading the image and then
getting its dimensions.

<a href="fn.load.html" class="fn" title="fn image::load">load</a>  
Create a new image from a Reader.

<a href="fn.load_from_memory.html" class="fn"
title="fn image::load_from_memory">load_from_memory</a>  
Create a new image from a byte slice

<a href="fn.load_from_memory_with_format.html" class="fn"
title="fn image::load_from_memory_with_format">load_from_memory_with_format</a>  
Create a new image from a byte slice

<a href="fn.open.html" class="fn" title="fn image::open">open</a>  
Open the image located at the path specified. The image’s format is
determined from the path’s file extension.

<a href="fn.save_buffer.html" class="fn"
title="fn image::save_buffer">save_buffer</a>  
Saves the supplied buffer to a file at the path specified.

<a href="fn.save_buffer_with_format.html" class="fn"
title="fn image::save_buffer_with_format">save_buffer_with_format</a>  
Saves the supplied buffer to a file given the path and desired format.

<a href="fn.write_buffer_with_format.html" class="fn"
title="fn image::write_buffer_with_format">write_buffer_with_format</a>  
Writes the supplied buffer to a writer in the specified format.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.GrayAlphaImage.html" class="type"
title="type image::GrayAlphaImage">GrayAlphaImage</a>  
Sendable grayscale + alpha channel image buffer

<a href="type.GrayImage.html" class="type"
title="type image::GrayImage">GrayImage</a>  
Sendable grayscale image buffer

<a href="type.Rgb32FImage.html" class="type"
title="type image::Rgb32FImage">Rgb32FImage</a>  
An image buffer for 32-bit float RGB pixels, where the backing container
is a flattened vector of floats.

<a href="type.RgbImage.html" class="type"
title="type image::RgbImage">RgbImage</a>  
Sendable Rgb image buffer

<a href="type.Rgba32FImage.html" class="type"
title="type image::Rgba32FImage">Rgba32FImage</a>  
An image buffer for 32-bit float RGBA pixels, where the backing
container is a flattened vector of floats.

<a href="type.RgbaImage.html" class="type"
title="type image::RgbaImage">RgbaImage</a>  
Sendable Rgb + alpha channel image buffer
