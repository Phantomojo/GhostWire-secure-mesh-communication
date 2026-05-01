Crate qrcode\_generator Copy item path

<span class="sub-heading"><a href="../src/qrcode_generator/lib.rs.html#1-712"
class="src">Source</a> </span>

Expand description

# <a href="#qr-code-generator" class="doc-anchor">§</a>QR Code Generator

This crate provides functions to generate QR Code matrices and images in
RAW, PNG and SVG formats.

## <a href="#examples" class="doc-anchor">§</a>Examples

#### <a href="#encode-any-data-to-a-qr-code-matrix-which-is-vecvecbool"
class="doc-anchor">§</a>Encode any data to a QR Code matrix which is `Vec<Vec<bool>>`.

``` {.rust .rust-example-rendered}
use qrcode_generator::QrCodeEcc;

let result: Vec<Vec<bool>> = qrcode_generator::to_matrix("Hello world!", QrCodeEcc::Low).unwrap();

println!("{:?}", result);
```

#### <a href="#encode-any-data-to-a-png-image-stored-in-a-vec-instance"
class="doc-anchor">§</a>Encode any data to a PNG image stored in a Vec instance.

``` {.rust .rust-example-rendered}
use qrcode_generator::QrCodeEcc;

let result: Vec<u8> = qrcode_generator::to_png_to_vec("Hello world!", QrCodeEcc::Low, 1024).unwrap();

println!("{:?}", result);
```

#### <a href="#encode-any-data-to-a-png-image-stored-in-a-file"
class="doc-anchor">§</a>Encode any data to a PNG image stored in a file.

``` {.rust .rust-example-rendered}
use qrcode_generator::QrCodeEcc;

qrcode_generator::to_png_to_file("Hello world!", QrCodeEcc::Low, 1024, "tests/data/file_output.png").unwrap();
```

#### <a href="#encode-any-data-to-a-svg-image-stored-in-a-string-instance"
class="doc-anchor">§</a>Encode any data to a SVG image stored in a String instance.

``` {.rust .rust-example-rendered}
use qrcode_generator::QrCodeEcc;

let result: String = qrcode_generator::to_svg_to_string("Hello world!", QrCodeEcc::Low, 1024, None::<&str>).unwrap();

println!("{:?}", result);
```

#### <a href="#encode-any-data-to-a-svg-image-stored-in-a-file"
class="doc-anchor">§</a>Encode any data to a SVG image stored in a file.

``` {.rust .rust-example-rendered}
use qrcode_generator::QrCodeEcc;

qrcode_generator::to_svg_to_file("Hello world!", QrCodeEcc::Low, 1024, None::<&str>, "tests/data/file_output.png").unwrap();
```

## <a href="#low-level-usage" class="doc-anchor">§</a>Low-level Usage

### <a href="#raw-image-data" class="doc-anchor">§</a>Raw Image Data

The `to_image` and `to_image_buffer` functions can be used, if you want
to modify your image.

### <a href="#segments" class="doc-anchor">§</a>Segments

Every `to_*` function has a corresponding `_from_segments` function. You
can concatenate segments by using different encoding methods, such as
**numeric**, **alphanumeric** or **binary** to reduce the size (level)
of your QR code matrix/image.

``` {.rust .rust-example-rendered}
use qrcode_generator::{QrCodeEcc, QrSegment};

let first = "1234567";

let second = "ABCDEFG";

let segments = [QrSegment::make_numeric(&first), QrSegment::make_alphanumeric(&second)];

let result: Vec<Vec<bool>> = qrcode_generator::to_matrix_from_segments(&segments, QrCodeEcc::Low).unwrap();

println!("{:?}", result);
```

More segments optimization apporaches:
[magiclen/qrcode-segments-optimizer](https://github.com/magiclen/qrcode-segments-optimizer)

# Re-exports<a href="#reexports" class="anchor">§</a>

pub extern crate <a href="../qrcodegen/index.html" class="mod"
title="mod qrcodegen">qrcodegen</a>;

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.QrSegment.html" class="struct"
title="struct qrcode_generator::QrSegment">QrSegment</a>  
A segment of character/binary/control data in a QR Code symbol.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.QRCodeError.html" class="enum"
title="enum qrcode_generator::QRCodeError">QRCodeError</a>  
Errors when encoding QR code.

<a href="enum.QrCodeEcc.html" class="enum"
title="enum qrcode_generator::QrCodeEcc">QrCodeEcc</a>  
The error correction level in a QR Code symbol.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.to_image.html" class="fn"
title="fn qrcode_generator::to_image">to_image</a>  
Encode data to raw image in memory.

<a href="fn.to_image_buffer.html" class="fn"
title="fn qrcode_generator::to_image_buffer">to_image_buffer</a>  
Encode data to a image buffer.

<a href="fn.to_image_buffer_from_segments.html" class="fn"
title="fn qrcode_generator::to_image_buffer_from_segments">to_image_buffer_from_segments</a>  
Encode segments to a image buffer.

<a href="fn.to_image_buffer_from_str.html" class="fn"
title="fn qrcode_generator::to_image_buffer_from_str">to_image_buffer_from_str</a>  
Encode text to a image buffer.

<a href="fn.to_image_from_segments.html" class="fn"
title="fn qrcode_generator::to_image_from_segments">to_image_from_segments</a>  
Encode segments to raw image in memory.

<a href="fn.to_image_from_str.html" class="fn"
title="fn qrcode_generator::to_image_from_str">to_image_from_str</a>  
Encode text to raw image in memory.

<a href="fn.to_matrix.html" class="fn"
title="fn qrcode_generator::to_matrix">to_matrix</a>  
Encode data to a QR code matrix.

<a href="fn.to_matrix_from_segments.html" class="fn"
title="fn qrcode_generator::to_matrix_from_segments">to_matrix_from_segments</a>  
Encode segments to a QR code matrix.

<a href="fn.to_matrix_from_str.html" class="fn"
title="fn qrcode_generator::to_matrix_from_str">to_matrix_from_str</a>  
Encode text to a QR code matrix.

<a href="fn.to_png_to_file.html" class="fn"
title="fn qrcode_generator::to_png_to_file">to_png_to_file</a>  
Encode data to a PNG image via a file path.

<a href="fn.to_png_to_file_from_segments.html" class="fn"
title="fn qrcode_generator::to_png_to_file_from_segments">to_png_to_file_from_segments</a>  
Encode text to a PNG image via a file path.

<a href="fn.to_png_to_file_from_str.html" class="fn"
title="fn qrcode_generator::to_png_to_file_from_str">to_png_to_file_from_str</a>  
Encode text to a PNG image via a file path.

<a href="fn.to_png_to_vec.html" class="fn"
title="fn qrcode_generator::to_png_to_vec">to_png_to_vec</a>  
Encode data to a PNG image in memory.

<a href="fn.to_png_to_vec_from_segments.html" class="fn"
title="fn qrcode_generator::to_png_to_vec_from_segments">to_png_to_vec_from_segments</a>  
Encode segments to a PNG image in memory.

<a href="fn.to_png_to_vec_from_str.html" class="fn"
title="fn qrcode_generator::to_png_to_vec_from_str">to_png_to_vec_from_str</a>  
Encode text to a PNG image in memory.

<a href="fn.to_png_to_writer.html" class="fn"
title="fn qrcode_generator::to_png_to_writer">to_png_to_writer</a>  
Encode data to a PNG image via a writer.

<a href="fn.to_png_to_writer_from_segments.html" class="fn"
title="fn qrcode_generator::to_png_to_writer_from_segments">to_png_to_writer_from_segments</a>  
Encode segments to a PNG image via a writer.

<a href="fn.to_png_to_writer_from_str.html" class="fn"
title="fn qrcode_generator::to_png_to_writer_from_str">to_png_to_writer_from_str</a>  
Encode text to a PNG image via a writer.

<a href="fn.to_svg_to_file.html" class="fn"
title="fn qrcode_generator::to_svg_to_file">to_svg_to_file</a>  
Encode data to a SVG image via a file path.

<a href="fn.to_svg_to_file_from_segments.html" class="fn"
title="fn qrcode_generator::to_svg_to_file_from_segments">to_svg_to_file_from_segments</a>  
Encode segments to a SVG image via a file path.

<a href="fn.to_svg_to_file_from_str.html" class="fn"
title="fn qrcode_generator::to_svg_to_file_from_str">to_svg_to_file_from_str</a>  
Encode text to a SVG image via a file path.

<a href="fn.to_svg_to_string.html" class="fn"
title="fn qrcode_generator::to_svg_to_string">to_svg_to_string</a>  
Encode data to a SVG image in memory.

<a href="fn.to_svg_to_string_from_segments.html" class="fn"
title="fn qrcode_generator::to_svg_to_string_from_segments">to_svg_to_string_from_segments</a>  
Encode segments to a SVG image in memory.

<a href="fn.to_svg_to_string_from_str.html" class="fn"
title="fn qrcode_generator::to_svg_to_string_from_str">to_svg_to_string_from_str</a>  
Encode text to a SVG image in memory.

<a href="fn.to_svg_to_writer.html" class="fn"
title="fn qrcode_generator::to_svg_to_writer">to_svg_to_writer</a>  
Encode data to a SVG image via a writer.

<a href="fn.to_svg_to_writer_from_segments.html" class="fn"
title="fn qrcode_generator::to_svg_to_writer_from_segments">to_svg_to_writer_from_segments</a>  
Encode segments to a SVG image via a writer.

<a href="fn.to_svg_to_writer_from_str.html" class="fn"
title="fn qrcode_generator::to_svg_to_writer_from_str">to_svg_to_writer_from_str</a>  
Encode text to a SVG image via a writer.
