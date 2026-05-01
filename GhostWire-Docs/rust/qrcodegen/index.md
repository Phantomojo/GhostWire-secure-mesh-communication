Crate qrcodegen Copy item path

<span class="sub-heading"><a href="../src/qrcodegen/lib.rs.html#25-1302" class="src">Source</a>
</span>

Expand description

Generates QR Codes from text strings and byte arrays.

This project aims to be the best, clearest QR Code generator library.
The primary goals are flexible options and absolute correctness.
Secondary goals are compact implementation size and good documentation
comments.

Home page with live JavaScript demo, extensive descriptions, and
competitor comparisons:
<https://www.nayuki.io/page/qr-code-generator-library>

# <a href="#features" class="doc-anchor">§</a>Features

Core features:

-   Significantly shorter code but more documentation comments compared
    to competing libraries
-   Supports encoding all 40 versions (sizes) and all 4 error correction
    levels, as per the QR Code Model 2 standard
-   Output format: Raw modules/pixels of the QR symbol
-   Detects finder-like penalty patterns more accurately than other
    implementations
-   Encodes numeric and special-alphanumeric text in less space than
    general text
-   Open-source code under the permissive MIT License

Manual parameters:

-   User can specify minimum and maximum version numbers allowed, then
    library will automatically choose smallest version in the range that
    fits the data
-   User can specify mask pattern manually, otherwise library will
    automatically evaluate all 8 masks and select the optimal one
-   User can specify absolute error correction level, or allow the
    library to boost it if it doesn’t increase the version number
-   User can create a list of data segments manually and add ECI
    segments

More information about QR Code technology and this library’s design can
be found on the project home page.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
extern crate qrcodegen;
use qrcodegen::Mask;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::QrSegment;
use qrcodegen::Version;
```

Simple operation:

``` {.rust .rust-example-rendered}
let qr = QrCode::encode_text("Hello, world!",
    QrCodeEcc::Medium).unwrap();
let svg = to_svg_string(&qr, 4);  // See qrcodegen-demo
```

Manual operation:

``` {.rust .rust-example-rendered}
let text: &str = "3141592653589793238462643383";
let segs = QrSegment::make_segments(text);
let qr = QrCode::encode_segments_advanced(&segs, QrCodeEcc::High,
    Version::new(5), Version::new(5), Some(Mask::new(2)), false).unwrap();
for y in 0 .. qr.size() {
    for x in 0 .. qr.size() {
        (... paint qr.get_module(x, y) ...)
    }
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BitBuffer.html" class="struct"
title="struct qrcodegen::BitBuffer">BitBuffer</a>  
An appendable sequence of bits (0s and 1s).

<a href="struct.Mask.html" class="struct"
title="struct qrcodegen::Mask">Mask</a>  
A number between 0 and 7 (inclusive).

<a href="struct.QrCode.html" class="struct"
title="struct qrcodegen::QrCode">QrCode</a>  
A QR Code symbol, which is a type of two-dimension barcode.

<a href="struct.QrSegment.html" class="struct"
title="struct qrcodegen::QrSegment">QrSegment</a>  
A segment of character/binary/control data in a QR Code symbol.

<a href="struct.Version.html" class="struct"
title="struct qrcodegen::Version">Version</a>  
A number between 1 and 40 (inclusive).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DataTooLong.html" class="enum"
title="enum qrcodegen::DataTooLong">DataTooLong</a>  
The error type when the supplied data does not fit any QR Code version.

<a href="enum.QrCodeEcc.html" class="enum"
title="enum qrcodegen::QrCodeEcc">QrCodeEcc</a>  
The error correction level in a QR Code symbol.

<a href="enum.QrSegmentMode.html" class="enum"
title="enum qrcodegen::QrSegmentMode">QrSegmentMode</a>  
Describes how a segment’s data bits are interpreted.
