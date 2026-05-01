Crate mime Copy item path

<span class="sub-heading"><a href="../src/mime/lib.rs.html#1-944" class="src">Source</a>
</span>

Expand description

# <a href="#mime" class="doc-anchor">§</a>Mime

Mime is now Media Type, technically, but `Mime` is more immediately
understandable, so the main type here is `Mime`.

## <a href="#what-is-mime" class="doc-anchor">§</a>What is Mime?

Example mime string: `text/plain`

``` {.rust .rust-example-rendered}
let plain_text: mime::Mime = "text/plain".parse().unwrap();
assert_eq!(plain_text, mime::TEXT_PLAIN);
```

## <a href="#inspecting-mimes" class="doc-anchor">§</a>Inspecting Mimes

``` {.rust .rust-example-rendered}
let mime = mime::TEXT_PLAIN;
match (mime.type_(), mime.subtype()) {
    (mime::TEXT, mime::PLAIN) => println!("plain text!"),
    (mime::TEXT, _) => println!("structured text"),
    _ => println!("not text"),
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.FromStrError.html" class="struct"
title="struct mime::FromStrError">FromStrError</a>  
An error when parsing a `Mime` from a string.

<a href="struct.Mime.html" class="struct"
title="struct mime::Mime">Mime</a>  
A parsed mime or media type.

<a href="struct.MimeIter.html" class="struct"
title="struct mime::MimeIter">MimeIter</a>  
An iterator of parsed mime

<a href="struct.Name.html" class="struct"
title="struct mime::Name">Name</a>  
A section of a `Mime`.

<a href="struct.Params.html" class="struct"
title="struct mime::Params">Params</a>  
An iterator over the parameters of a MIME.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.APPLICATION.html" class="constant"
title="constant mime::APPLICATION">APPLICATION</a>  
application

<a href="constant.APPLICATION_JAVASCRIPT.html" class="constant"
title="constant mime::APPLICATION_JAVASCRIPT">APPLICATION_JAVASCRIPT</a>  
`application/javascript`

<a href="constant.APPLICATION_JAVASCRIPT_UTF_8.html" class="constant"
title="constant mime::APPLICATION_JAVASCRIPT_UTF_8">APPLICATION_JAVASCRIPT_UTF_8</a>  
`application/javascript; charset=utf-8`

<a href="constant.APPLICATION_JSON.html" class="constant"
title="constant mime::APPLICATION_JSON">APPLICATION_JSON</a>  
`application/json`

<a href="constant.APPLICATION_MSGPACK.html" class="constant"
title="constant mime::APPLICATION_MSGPACK">APPLICATION_MSGPACK</a>  
`application/msgpack`

<a href="constant.APPLICATION_OCTET_STREAM.html" class="constant"
title="constant mime::APPLICATION_OCTET_STREAM">APPLICATION_OCTET_STREAM</a>  
`application/octet-stream`

<a href="constant.APPLICATION_PDF.html" class="constant"
title="constant mime::APPLICATION_PDF">APPLICATION_PDF</a>  
`application/pdf`

<a href="constant.APPLICATION_WWW_FORM_URLENCODED.html" class="constant"
title="constant mime::APPLICATION_WWW_FORM_URLENCODED">APPLICATION_WWW_FORM_URLENCODED</a>  
`application/x-www-form-urlencoded`

<a href="constant.AUDIO.html" class="constant"
title="constant mime::AUDIO">AUDIO</a>  
audio

<a href="constant.BASIC.html" class="constant"
title="constant mime::BASIC">BASIC</a>  
basic

<a href="constant.BMP.html" class="constant"
title="constant mime::BMP">BMP</a>  
bmp

<a href="constant.BOUNDARY.html" class="constant"
title="constant mime::BOUNDARY">BOUNDARY</a>  
boundary

<a href="constant.CHARSET.html" class="constant"
title="constant mime::CHARSET">CHARSET</a>  
charset

<a href="constant.CSS.html" class="constant"
title="constant mime::CSS">CSS</a>  
css

<a href="constant.CSV.html" class="constant"
title="constant mime::CSV">CSV</a>  
csv

<a href="constant.EVENT_STREAM.html" class="constant"
title="constant mime::EVENT_STREAM">EVENT_STREAM</a>  
event-stream

<a href="constant.FONT.html" class="constant"
title="constant mime::FONT">FONT</a>  
font

<a href="constant.FONT_WOFF.html" class="constant"
title="constant mime::FONT_WOFF">FONT_WOFF</a>  
`font/woff`

<a href="constant.FONT_WOFF2.html" class="constant"
title="constant mime::FONT_WOFF2">FONT_WOFF2</a>  
`font/woff2`

<a href="constant.FORM_DATA.html" class="constant"
title="constant mime::FORM_DATA">FORM_DATA</a>  
form-data

<a href="constant.GIF.html" class="constant"
title="constant mime::GIF">GIF</a>  
gif

<a href="constant.HTML.html" class="constant"
title="constant mime::HTML">HTML</a>  
html

<a href="constant.IMAGE.html" class="constant"
title="constant mime::IMAGE">IMAGE</a>  
image

<a href="constant.IMAGE_BMP.html" class="constant"
title="constant mime::IMAGE_BMP">IMAGE_BMP</a>  
`image/bmp`

<a href="constant.IMAGE_GIF.html" class="constant"
title="constant mime::IMAGE_GIF">IMAGE_GIF</a>  
`image/gif`

<a href="constant.IMAGE_JPEG.html" class="constant"
title="constant mime::IMAGE_JPEG">IMAGE_JPEG</a>  
`image/jpeg`

<a href="constant.IMAGE_PNG.html" class="constant"
title="constant mime::IMAGE_PNG">IMAGE_PNG</a>  
`image/png`

<a href="constant.IMAGE_STAR.html" class="constant"
title="constant mime::IMAGE_STAR">IMAGE_STAR</a>  
`image/*`

<a href="constant.IMAGE_SVG.html" class="constant"
title="constant mime::IMAGE_SVG">IMAGE_SVG</a>  
`image/svg+xml`

<a href="constant.JAVASCRIPT.html" class="constant"
title="constant mime::JAVASCRIPT">JAVASCRIPT</a>  
javascript

<a href="constant.JPEG.html" class="constant"
title="constant mime::JPEG">JPEG</a>  
jpeg

<a href="constant.JSON.html" class="constant"
title="constant mime::JSON">JSON</a>  
json

<a href="constant.MESSAGE.html" class="constant"
title="constant mime::MESSAGE">MESSAGE</a>  
message

<a href="constant.MODEL.html" class="constant"
title="constant mime::MODEL">MODEL</a>  
model

<a href="constant.MP4.html" class="constant"
title="constant mime::MP4">MP4</a>  
mp4

<a href="constant.MPEG.html" class="constant"
title="constant mime::MPEG">MPEG</a>  
mpeg

<a href="constant.MSGPACK.html" class="constant"
title="constant mime::MSGPACK">MSGPACK</a>  
msgpack

<a href="constant.MULTIPART.html" class="constant"
title="constant mime::MULTIPART">MULTIPART</a>  
multipart

<a href="constant.MULTIPART_FORM_DATA.html" class="constant"
title="constant mime::MULTIPART_FORM_DATA">MULTIPART_FORM_DATA</a>  
`multipart/form-data`

<a href="constant.OCTET_STREAM.html" class="constant"
title="constant mime::OCTET_STREAM">OCTET_STREAM</a>  
octet-stream

<a href="constant.OGG.html" class="constant"
title="constant mime::OGG">OGG</a>  
ogg

<a href="constant.PDF.html" class="constant"
title="constant mime::PDF">PDF</a>  
pdf

<a href="constant.PLAIN.html" class="constant"
title="constant mime::PLAIN">PLAIN</a>  
plain

<a href="constant.PNG.html" class="constant"
title="constant mime::PNG">PNG</a>  
png

<a href="constant.STAR.html" class="constant"
title="constant mime::STAR">STAR</a>  
<a href="constant.STAR_STAR.html" class="constant"
title="constant mime::STAR_STAR">STAR_STAR</a>  
`*/*`

<a href="constant.SVG.html" class="constant"
title="constant mime::SVG">SVG</a>  
svg

<a href="constant.TEXT.html" class="constant"
title="constant mime::TEXT">TEXT</a>  
text

<a href="constant.TEXT_CSS.html" class="constant"
title="constant mime::TEXT_CSS">TEXT_CSS</a>  
`text/css`

<a href="constant.TEXT_CSS_UTF_8.html" class="constant"
title="constant mime::TEXT_CSS_UTF_8">TEXT_CSS_UTF_8</a>  
`text/css; charset=utf-8`

<a href="constant.TEXT_CSV.html" class="constant"
title="constant mime::TEXT_CSV">TEXT_CSV</a>  
`text/csv`

<a href="constant.TEXT_CSV_UTF_8.html" class="constant"
title="constant mime::TEXT_CSV_UTF_8">TEXT_CSV_UTF_8</a>  
`text/csv; charset=utf-8`

<a href="constant.TEXT_EVENT_STREAM.html" class="constant"
title="constant mime::TEXT_EVENT_STREAM">TEXT_EVENT_STREAM</a>  
`text/event-stream`

<a href="constant.TEXT_HTML.html" class="constant"
title="constant mime::TEXT_HTML">TEXT_HTML</a>  
`text/html`

<a href="constant.TEXT_HTML_UTF_8.html" class="constant"
title="constant mime::TEXT_HTML_UTF_8">TEXT_HTML_UTF_8</a>  
`text/html; charset=utf-8`

<a href="constant.TEXT_JAVASCRIPT.html" class="constant"
title="constant mime::TEXT_JAVASCRIPT">TEXT_JAVASCRIPT</a>  
`text/javascript`

<a href="constant.TEXT_PLAIN.html" class="constant"
title="constant mime::TEXT_PLAIN">TEXT_PLAIN</a>  
`text/plain`

<a href="constant.TEXT_PLAIN_UTF_8.html" class="constant"
title="constant mime::TEXT_PLAIN_UTF_8">TEXT_PLAIN_UTF_8</a>  
`text/plain; charset=utf-8`

<a href="constant.TEXT_STAR.html" class="constant"
title="constant mime::TEXT_STAR">TEXT_STAR</a>  
`text/*`

<a href="constant.TEXT_TAB_SEPARATED_VALUES.html" class="constant"
title="constant mime::TEXT_TAB_SEPARATED_VALUES">TEXT_TAB_SEPARATED_VALUES</a>  
`text/tab-separated-values`

<a href="constant.TEXT_TAB_SEPARATED_VALUES_UTF_8.html" class="constant"
title="constant mime::TEXT_TAB_SEPARATED_VALUES_UTF_8">TEXT_TAB_SEPARATED_VALUES_UTF_8</a>  
`text/tab-separated-values; charset=utf-8`

<a href="constant.TEXT_VCARD.html" class="constant"
title="constant mime::TEXT_VCARD">TEXT_VCARD</a>  
`text/vcard`

<a href="constant.TEXT_XML.html" class="constant"
title="constant mime::TEXT_XML">TEXT_XML</a>  
`text/xml`

<a href="constant.UTF_8.html" class="constant"
title="constant mime::UTF_8">UTF_8</a>  
utf-8

<a href="constant.VCARD.html" class="constant"
title="constant mime::VCARD">VCARD</a>  
vcard

<a href="constant.VIDEO.html" class="constant"
title="constant mime::VIDEO">VIDEO</a>  
video

<a href="constant.WOFF.html" class="constant"
title="constant mime::WOFF">WOFF</a>  
woff

<a href="constant.WOFF2.html" class="constant"
title="constant mime::WOFF2">WOFF2</a>  
woff2

<a href="constant.WWW_FORM_URLENCODED.html" class="constant"
title="constant mime::WWW_FORM_URLENCODED">WWW_FORM_URLENCODED</a>  
x-www-form-urlencoded

<a href="constant.XML.html" class="constant"
title="constant mime::XML">XML</a>  
xml
