Crate html\_escape Copy item path

<span class="sub-heading"><a href="../src/html_escape/lib.rs.html#1-68" class="src">Source</a>
</span>

Expand description

# <a href="#html-escape" class="doc-anchor">§</a>HTML Escape

This library is for encoding/escaping special characters in HTML and
decoding/unescaping HTML entities as well.

## <a href="#usage" class="doc-anchor">§</a>Usage

### <a href="#encoding" class="doc-anchor">§</a>Encoding

This crate provides some `encode_*` functions to encode HTML text in
different situations.

For example, to put a text between a start tag `<foo>` and an end tag
`</foo>`, use the `encode_text` function to escape every `&`, `<`, and
`>` in the text.

``` {.rust .rust-example-rendered}
assert_eq!("a &gt; b &amp;&amp; a &lt; c", html_escape::encode_text("a > b && a < c"));
```

The functions suffixed with `_to_writer`, `_to_vec` or `_to_string` are
useful to generate HTML.

``` {.rust .rust-example-rendered}
let mut html = String::from("<input value=");
assert_eq!("Hello&#x20;world&#x21;", html_escape::encode_unquoted_attribute_to_string("Hello world!", &mut html));
html.push_str(" placeholder=\"");
assert_eq!("The default value is &quot;Hello world!&quot;.", html_escape::encode_double_quoted_attribute_to_string("The default value is \"Hello world!\".", &mut html));
html.push_str("\"/><script>alert('");
assert_eq!(r"<script>\'s end tag is <\/script>", html_escape::encode_script_single_quoted_text_to_string("<script>'s end tag is </script>", &mut html));
html.push_str("');</script>");

assert_eq!("<input value=Hello&#x20;world&#x21; placeholder=\"The default value is &quot;Hello world!&quot;.\"/><script>alert(\'<script>\\\'s end tag is <\\/script>\');</script>", html);
```

### <a href="#decoding" class="doc-anchor">§</a>Decoding

``` {.rust .rust-example-rendered}
assert_eq!("Hello world!", html_escape::decode_html_entities("Hello&#x20;world&#x21;"));
```

``` {.rust .rust-example-rendered}
assert_eq!("alert('<script></script>');", html_escape::decode_script(r"alert('<script><\/script>');"));
```

## <a href="#no-std" class="doc-anchor">§</a>No Std

Disable the default features to compile this crate without std.

``` language-toml
[dependencies.html-escape]
version = "*"
default-features = false
```

## <a href="#benchmark" class="doc-anchor">§</a>Benchmark

``` language-bash
cargo bench
```

# Statics<a href="#statics" class="anchor">§</a>

<a href="static.NAMED_ENTITIES.html" class="static"
title="static html_escape::NAMED_ENTITIES">NAMED_ENTITIES</a>  
The table of HTML named entities ordered by the names.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decode_html_entities.html" class="fn"
title="fn html_escape::decode_html_entities">decode_html_entities</a>  
Decode html entities in a given string.

<a href="fn.decode_html_entities_to_string.html" class="fn"
title="fn html_escape::decode_html_entities_to_string">decode_html_entities_to_string</a>  
Decode html entities in a given string to a mutable `String` reference
and return the decoded string slice.

<a href="fn.decode_html_entities_to_vec.html" class="fn"
title="fn html_escape::decode_html_entities_to_vec">decode_html_entities_to_vec</a>  
Decode html entities in a given string to a mutable `Vec<u8>` reference
and return the decoded data slice.

<a href="fn.decode_html_entities_to_writer.html" class="fn"
title="fn html_escape::decode_html_entities_to_writer">decode_html_entities_to_writer</a>  
Decode html entities in a given string to a writer.

<a href="fn.decode_script.html" class="fn"
title="fn html_escape::decode_script">decode_script</a>  
Decode text from the `<script>` element.

<a href="fn.decode_script_double_quoted_text.html" class="fn"
title="fn html_escape::decode_script_double_quoted_text">decode_script_double_quoted_text</a>  
Decode text from a double quoted text in the `<script>` element.

<a href="fn.decode_script_double_quoted_text_to_string.html" class="fn"
title="fn html_escape::decode_script_double_quoted_text_to_string">decode_script_double_quoted_text_to_string</a>  
Write text from a double quoted text in the `<script>` element to a
mutable `String` reference and return the encoded string slice.

<a href="fn.decode_script_double_quoted_text_to_vec.html" class="fn"
title="fn html_escape::decode_script_double_quoted_text_to_vec">decode_script_double_quoted_text_to_vec</a>  
Write text from a double quoted text in the `<script>` element to a
mutable `Vec<u8>` reference and return the encoded data slice.

<a href="fn.decode_script_double_quoted_text_to_writer.html" class="fn"
title="fn html_escape::decode_script_double_quoted_text_to_writer">decode_script_double_quoted_text_to_writer</a>  
Write text from a double quoted text in the `<script>` element to a
writer.

<a href="fn.decode_script_quoted_text.html" class="fn"
title="fn html_escape::decode_script_quoted_text">decode_script_quoted_text</a>  
Decode text from a quoted text in the `<script>` element.

<a href="fn.decode_script_quoted_text_to_string.html" class="fn"
title="fn html_escape::decode_script_quoted_text_to_string">decode_script_quoted_text_to_string</a>  
Write text from a quoted text in the `<script>` element to a mutable
`String` reference and return the encoded string slice.

<a href="fn.decode_script_quoted_text_to_vec.html" class="fn"
title="fn html_escape::decode_script_quoted_text_to_vec">decode_script_quoted_text_to_vec</a>  
Write text from a quoted text in the `<script>` element to a mutable
`Vec<u8>` reference and return the encoded data slice.

<a href="fn.decode_script_quoted_text_to_writer.html" class="fn"
title="fn html_escape::decode_script_quoted_text_to_writer">decode_script_quoted_text_to_writer</a>  
Write text from a quoted text in the `<script>` element to a writer.

<a href="fn.decode_script_single_quoted_text.html" class="fn"
title="fn html_escape::decode_script_single_quoted_text">decode_script_single_quoted_text</a>  
Decode text from a single quoted text in the `<script>` element.

<a href="fn.decode_script_single_quoted_text_to_string.html" class="fn"
title="fn html_escape::decode_script_single_quoted_text_to_string">decode_script_single_quoted_text_to_string</a>  
Write text from a single quoted text in the `<script>` element to a
mutable `String` reference and return the encoded string slice.

<a href="fn.decode_script_single_quoted_text_to_vec.html" class="fn"
title="fn html_escape::decode_script_single_quoted_text_to_vec">decode_script_single_quoted_text_to_vec</a>  
Write text from a single quoted text in the `<script>` element to a
mutable `Vec<u8>` reference and return the encoded data slice.

<a href="fn.decode_script_single_quoted_text_to_writer.html" class="fn"
title="fn html_escape::decode_script_single_quoted_text_to_writer">decode_script_single_quoted_text_to_writer</a>  
Write text from a single quoted text in the `<script>` element to a
writer.

<a href="fn.decode_script_to_string.html" class="fn"
title="fn html_escape::decode_script_to_string">decode_script_to_string</a>  
Write text from the `<script>` element to a mutable `String` reference
and return the encoded string slice.

<a href="fn.decode_script_to_vec.html" class="fn"
title="fn html_escape::decode_script_to_vec">decode_script_to_vec</a>  
Write text from the `<script>` element to a mutable `Vec<u8>` reference
and return the encoded data slice.

<a href="fn.decode_script_to_writer.html" class="fn"
title="fn html_escape::decode_script_to_writer">decode_script_to_writer</a>  
Write text from the `<script>` element to a writer.

<a href="fn.decode_style.html" class="fn"
title="fn html_escape::decode_style">decode_style</a>  
Decode text from the `<style>` element.

<a href="fn.decode_style_double_quoted_text.html" class="fn"
title="fn html_escape::decode_style_double_quoted_text">decode_style_double_quoted_text</a>  
Decode text from a double quoted text in the `<style>` element.

<a href="fn.decode_style_double_quoted_text_to_string.html" class="fn"
title="fn html_escape::decode_style_double_quoted_text_to_string">decode_style_double_quoted_text_to_string</a>  
Write text from a double quoted text in the `<style>` element to a
mutable `String` reference and return the encoded string slice.

<a href="fn.decode_style_double_quoted_text_to_vec.html" class="fn"
title="fn html_escape::decode_style_double_quoted_text_to_vec">decode_style_double_quoted_text_to_vec</a>  
Write text from a double quoted text in the `<style>` element to a
mutable `Vec<u8>` reference and return the encoded data slice.

<a href="fn.decode_style_double_quoted_text_to_writer.html" class="fn"
title="fn html_escape::decode_style_double_quoted_text_to_writer">decode_style_double_quoted_text_to_writer</a>  
Write text from a double quoted text in the `<style>` element to a
writer.

<a href="fn.decode_style_quoted_text.html" class="fn"
title="fn html_escape::decode_style_quoted_text">decode_style_quoted_text</a>  
Decode text from a quoted text in the `<style>` element.

<a href="fn.decode_style_quoted_text_to_string.html" class="fn"
title="fn html_escape::decode_style_quoted_text_to_string">decode_style_quoted_text_to_string</a>  
Write text from a quoted text in the `<style>` element to a mutable
`String` reference and return the encoded string slice.

<a href="fn.decode_style_quoted_text_to_vec.html" class="fn"
title="fn html_escape::decode_style_quoted_text_to_vec">decode_style_quoted_text_to_vec</a>  
Write text from a quoted text in the `<style>` element to a mutable
`Vec<u8>` reference and return the encoded data slice.

<a href="fn.decode_style_quoted_text_to_writer.html" class="fn"
title="fn html_escape::decode_style_quoted_text_to_writer">decode_style_quoted_text_to_writer</a>  
Write text from a quoted text in the `<style>` element to a writer.

<a href="fn.decode_style_single_quoted_text.html" class="fn"
title="fn html_escape::decode_style_single_quoted_text">decode_style_single_quoted_text</a>  
Decode text from a single quoted text in the `<style>` element.

<a href="fn.decode_style_single_quoted_text_to_string.html" class="fn"
title="fn html_escape::decode_style_single_quoted_text_to_string">decode_style_single_quoted_text_to_string</a>  
Write text from a single quoted text in the `<style>` element to a
mutable `String` reference and return the encoded string slice.

<a href="fn.decode_style_single_quoted_text_to_vec.html" class="fn"
title="fn html_escape::decode_style_single_quoted_text_to_vec">decode_style_single_quoted_text_to_vec</a>  
Write text from a single quoted text in the `<style>` element to a
mutable `Vec<u8>` reference and return the encoded data slice.

<a href="fn.decode_style_single_quoted_text_to_writer.html" class="fn"
title="fn html_escape::decode_style_single_quoted_text_to_writer">decode_style_single_quoted_text_to_writer</a>  
Write text from a single quoted text in the `<style>` element to a
writer.

<a href="fn.decode_style_to_string.html" class="fn"
title="fn html_escape::decode_style_to_string">decode_style_to_string</a>  
Write text from the `<style>` element to a mutable `String` reference
and return the encoded string slice.

<a href="fn.decode_style_to_vec.html" class="fn"
title="fn html_escape::decode_style_to_vec">decode_style_to_vec</a>  
Write text from the `<style>` element to a mutable `Vec<u8>` reference
and return the encoded data slice.

<a href="fn.decode_style_to_writer.html" class="fn"
title="fn html_escape::decode_style_to_writer">decode_style_to_writer</a>  
Write text from the `<style>` element to a writer.

<a href="fn.encode_double_quoted_attribute.html" class="fn"
title="fn html_escape::encode_double_quoted_attribute">encode_double_quoted_attribute</a>  
Encode text used in a double-quoted attribute.

<a href="fn.encode_double_quoted_attribute_to_string.html" class="fn"
title="fn html_escape::encode_double_quoted_attribute_to_string">encode_double_quoted_attribute_to_string</a>  
Write text used in a double-quoted attribute to a mutable `String`
reference and return the encoded string slice.

<a href="fn.encode_double_quoted_attribute_to_vec.html" class="fn"
title="fn html_escape::encode_double_quoted_attribute_to_vec">encode_double_quoted_attribute_to_vec</a>  
Write text used in a double-quoted attribute to a mutable `Vec<u8>`
reference and return the encoded data slice.

<a href="fn.encode_double_quoted_attribute_to_writer.html" class="fn"
title="fn html_escape::encode_double_quoted_attribute_to_writer">encode_double_quoted_attribute_to_writer</a>  
Write text used in a double-quoted attribute to a writer.

<a href="fn.encode_quoted_attribute.html" class="fn"
title="fn html_escape::encode_quoted_attribute">encode_quoted_attribute</a>  
Encode text used in a quoted attribute.

<a href="fn.encode_quoted_attribute_to_string.html" class="fn"
title="fn html_escape::encode_quoted_attribute_to_string">encode_quoted_attribute_to_string</a>  
Write text used in a quoted attribute to a mutable `String` reference
and return the encoded string slice.

<a href="fn.encode_quoted_attribute_to_vec.html" class="fn"
title="fn html_escape::encode_quoted_attribute_to_vec">encode_quoted_attribute_to_vec</a>  
Write text used in a quoted attribute to a mutable `Vec<u8>` reference
and return the encoded data slice.

<a href="fn.encode_quoted_attribute_to_writer.html" class="fn"
title="fn html_escape::encode_quoted_attribute_to_writer">encode_quoted_attribute_to_writer</a>  
Write text used in a quoted attribute to a writer.

<a href="fn.encode_safe.html" class="fn"
title="fn html_escape::encode_safe">encode_safe</a>  
Encode text to prevent special characters functioning.

<a href="fn.encode_safe_to_string.html" class="fn"
title="fn html_escape::encode_safe_to_string">encode_safe_to_string</a>  
Encode text to prevent special characters functioning and write it to a
mutable `String` reference and return the encoded string slice.

<a href="fn.encode_safe_to_vec.html" class="fn"
title="fn html_escape::encode_safe_to_vec">encode_safe_to_vec</a>  
Encode text to prevent special characters functioning and write it to a
mutable `Vec<u8>` reference and return the encoded data slice.

<a href="fn.encode_safe_to_writer.html" class="fn"
title="fn html_escape::encode_safe_to_writer">encode_safe_to_writer</a>  
Encode text to prevent special characters functioning and write it to a
writer.

<a href="fn.encode_script.html" class="fn"
title="fn html_escape::encode_script">encode_script</a>  
Encode text used in the `<script>` element.

<a href="fn.encode_script_double_quoted_text.html" class="fn"
title="fn html_escape::encode_script_double_quoted_text">encode_script_double_quoted_text</a>  
Encode text used in a double quoted text in the `<script>` element.

<a href="fn.encode_script_double_quoted_text_to_string.html" class="fn"
title="fn html_escape::encode_script_double_quoted_text_to_string">encode_script_double_quoted_text_to_string</a>  
Write text used in a double quoted text in the `<script>` element to a
mutable `String` reference and return the encoded string slice.

<a href="fn.encode_script_double_quoted_text_to_vec.html" class="fn"
title="fn html_escape::encode_script_double_quoted_text_to_vec">encode_script_double_quoted_text_to_vec</a>  
Write text used in a double quoted text in the `<script>` element to a
mutable `Vec<u8>` reference and return the encoded data slice.

<a href="fn.encode_script_double_quoted_text_to_writer.html" class="fn"
title="fn html_escape::encode_script_double_quoted_text_to_writer">encode_script_double_quoted_text_to_writer</a>  
Write text used in a double quoted text in the `<script>` element to a
writer.

<a href="fn.encode_script_quoted_text.html" class="fn"
title="fn html_escape::encode_script_quoted_text">encode_script_quoted_text</a>  
Encode text used in a quoted text in the `<script>` element.

<a href="fn.encode_script_quoted_text_to_string.html" class="fn"
title="fn html_escape::encode_script_quoted_text_to_string">encode_script_quoted_text_to_string</a>  
Write text used in a quoted text in the `<script>` element to a mutable
`String` reference and return the encoded string slice.

<a href="fn.encode_script_quoted_text_to_vec.html" class="fn"
title="fn html_escape::encode_script_quoted_text_to_vec">encode_script_quoted_text_to_vec</a>  
Write text used in a quoted text in the `<script>` element to a mutable
`Vec<u8>` reference and return the encoded data slice.

<a href="fn.encode_script_quoted_text_to_writer.html" class="fn"
title="fn html_escape::encode_script_quoted_text_to_writer">encode_script_quoted_text_to_writer</a>  
Write text used in a quoted text in the `<script>` element to a writer.

<a href="fn.encode_script_single_quoted_text.html" class="fn"
title="fn html_escape::encode_script_single_quoted_text">encode_script_single_quoted_text</a>  
Encode text used in a single quoted text in the `<script>` element.

<a href="fn.encode_script_single_quoted_text_to_string.html" class="fn"
title="fn html_escape::encode_script_single_quoted_text_to_string">encode_script_single_quoted_text_to_string</a>  
Write text used in a single quoted text in the `<script>` element to a
mutable `String` reference and return the encoded string slice.

<a href="fn.encode_script_single_quoted_text_to_vec.html" class="fn"
title="fn html_escape::encode_script_single_quoted_text_to_vec">encode_script_single_quoted_text_to_vec</a>  
Write text used in a single quoted text in the `<script>` element to a
mutable `Vec<u8>` reference and return the encoded data slice.

<a href="fn.encode_script_single_quoted_text_to_writer.html" class="fn"
title="fn html_escape::encode_script_single_quoted_text_to_writer">encode_script_single_quoted_text_to_writer</a>  
Write text used in a single quoted text in the `<script>` element to a
writer.

<a href="fn.encode_script_to_string.html" class="fn"
title="fn html_escape::encode_script_to_string">encode_script_to_string</a>  
Write text used in the `<script>` element to a mutable `String`
reference and return the encoded string slice.

<a href="fn.encode_script_to_vec.html" class="fn"
title="fn html_escape::encode_script_to_vec">encode_script_to_vec</a>  
Write text used in the `<script>` element to a mutable `Vec<u8>`
reference and return the encoded data slice.

<a href="fn.encode_script_to_writer.html" class="fn"
title="fn html_escape::encode_script_to_writer">encode_script_to_writer</a>  
Write text used in the `<script>` element to a writer.

<a href="fn.encode_single_quoted_attribute.html" class="fn"
title="fn html_escape::encode_single_quoted_attribute">encode_single_quoted_attribute</a>  
Encode text used in a single-quoted attribute.

<a href="fn.encode_single_quoted_attribute_to_string.html" class="fn"
title="fn html_escape::encode_single_quoted_attribute_to_string">encode_single_quoted_attribute_to_string</a>  
Write text used in a single-quoted attribute to a mutable `String`
reference and return the encoded string slice.

<a href="fn.encode_single_quoted_attribute_to_vec.html" class="fn"
title="fn html_escape::encode_single_quoted_attribute_to_vec">encode_single_quoted_attribute_to_vec</a>  
Write text used in a single-quoted attribute to a mutable `Vec<u8>`
reference and return the encoded data slice.

<a href="fn.encode_single_quoted_attribute_to_writer.html" class="fn"
title="fn html_escape::encode_single_quoted_attribute_to_writer">encode_single_quoted_attribute_to_writer</a>  
Write text used in a single-quoted attribute to a writer.

<a href="fn.encode_style.html" class="fn"
title="fn html_escape::encode_style">encode_style</a>  
Encode text used in the `<style>` element.

<a href="fn.encode_style_double_quoted_text.html" class="fn"
title="fn html_escape::encode_style_double_quoted_text">encode_style_double_quoted_text</a>  
Encode text used in a double quoted text in the `<style>` element.

<a href="fn.encode_style_double_quoted_text_to_string.html" class="fn"
title="fn html_escape::encode_style_double_quoted_text_to_string">encode_style_double_quoted_text_to_string</a>  
Write text used in a double quoted text in the `<style>` element to a
mutable `String` reference and return the encoded string slice.

<a href="fn.encode_style_double_quoted_text_to_vec.html" class="fn"
title="fn html_escape::encode_style_double_quoted_text_to_vec">encode_style_double_quoted_text_to_vec</a>  
Write text used in a double quoted text in the `<style>` element to a
mutable `Vec<u8>` reference and return the encoded data slice.

<a href="fn.encode_style_double_quoted_text_to_writer.html" class="fn"
title="fn html_escape::encode_style_double_quoted_text_to_writer">encode_style_double_quoted_text_to_writer</a>  
Write text used in a double quoted text in the `<style>` element to a
writer.

<a href="fn.encode_style_quoted_text.html" class="fn"
title="fn html_escape::encode_style_quoted_text">encode_style_quoted_text</a>  
Encode text used in a quoted text in the `<style>` element.

<a href="fn.encode_style_quoted_text_to_string.html" class="fn"
title="fn html_escape::encode_style_quoted_text_to_string">encode_style_quoted_text_to_string</a>  
Write text used in a quoted text in the `<style>` element to a mutable
`String` reference and return the encoded string slice.

<a href="fn.encode_style_quoted_text_to_vec.html" class="fn"
title="fn html_escape::encode_style_quoted_text_to_vec">encode_style_quoted_text_to_vec</a>  
Write text used in a quoted text in the `<style>` element to a mutable
`Vec<u8>` reference and return the encoded data slice.

<a href="fn.encode_style_quoted_text_to_writer.html" class="fn"
title="fn html_escape::encode_style_quoted_text_to_writer">encode_style_quoted_text_to_writer</a>  
Write text used in a quoted text in the `<style>` element to a writer.

<a href="fn.encode_style_single_quoted_text.html" class="fn"
title="fn html_escape::encode_style_single_quoted_text">encode_style_single_quoted_text</a>  
Encode text used in a single quoted text in the `<style>` element.

<a href="fn.encode_style_single_quoted_text_to_string.html" class="fn"
title="fn html_escape::encode_style_single_quoted_text_to_string">encode_style_single_quoted_text_to_string</a>  
Write text used in a single quoted text in the `<style>` element to a
mutable `String` reference and return the encoded string slice.

<a href="fn.encode_style_single_quoted_text_to_vec.html" class="fn"
title="fn html_escape::encode_style_single_quoted_text_to_vec">encode_style_single_quoted_text_to_vec</a>  
Write text used in a single quoted text in the `<style>` element to a
mutable `Vec<u8>` reference and return the encoded data slice.

<a href="fn.encode_style_single_quoted_text_to_writer.html" class="fn"
title="fn html_escape::encode_style_single_quoted_text_to_writer">encode_style_single_quoted_text_to_writer</a>  
Write text used in a single quoted text in the `<style>` element to a
writer.

<a href="fn.encode_style_to_string.html" class="fn"
title="fn html_escape::encode_style_to_string">encode_style_to_string</a>  
Write text used in the `<style>` element to a mutable `String` reference
and return the encoded string slice.

<a href="fn.encode_style_to_vec.html" class="fn"
title="fn html_escape::encode_style_to_vec">encode_style_to_vec</a>  
Write text used in the `<style>` element to a mutable `Vec<u8>`
reference and return the encoded data slice.

<a href="fn.encode_style_to_writer.html" class="fn"
title="fn html_escape::encode_style_to_writer">encode_style_to_writer</a>  
Write text used in the `<style>` element to a writer.

<a href="fn.encode_text.html" class="fn"
title="fn html_escape::encode_text">encode_text</a>  
Encode text used as regular HTML text.

<a href="fn.encode_text_minimal.html" class="fn"
title="fn html_escape::encode_text_minimal">encode_text_minimal</a>  
Encode text used as regular HTML text.

<a href="fn.encode_text_minimal_to_string.html" class="fn"
title="fn html_escape::encode_text_minimal_to_string">encode_text_minimal_to_string</a>  
Write text used as regular HTML text to a mutable `String` reference and
return the encoded string slice.

<a href="fn.encode_text_minimal_to_vec.html" class="fn"
title="fn html_escape::encode_text_minimal_to_vec">encode_text_minimal_to_vec</a>  
Write text used as regular HTML text to a mutable `Vec<u8>` reference
and return the encoded data slice.

<a href="fn.encode_text_minimal_to_writer.html" class="fn"
title="fn html_escape::encode_text_minimal_to_writer">encode_text_minimal_to_writer</a>  
Write text used as regular HTML text to a writer.

<a href="fn.encode_text_to_string.html" class="fn"
title="fn html_escape::encode_text_to_string">encode_text_to_string</a>  
Write text used as regular HTML text to a mutable `String` reference and
return the encoded string slice.

<a href="fn.encode_text_to_vec.html" class="fn"
title="fn html_escape::encode_text_to_vec">encode_text_to_vec</a>  
Write text used as regular HTML text to a mutable `Vec<u8>` reference
and return the encoded data slice.

<a href="fn.encode_text_to_writer.html" class="fn"
title="fn html_escape::encode_text_to_writer">encode_text_to_writer</a>  
Write text used as regular HTML text to a writer.

<a href="fn.encode_unquoted_attribute.html" class="fn"
title="fn html_escape::encode_unquoted_attribute">encode_unquoted_attribute</a>  
Encode text used in an unquoted attribute. Except for alphanumeric
characters, escape all characters which are less than 128.

<a href="fn.encode_unquoted_attribute_to_string.html" class="fn"
title="fn html_escape::encode_unquoted_attribute_to_string">encode_unquoted_attribute_to_string</a>  
Write text used in an unquoted attribute to a mutable `String` reference
and return the encoded string slice. Except for alphanumeric characters,
escape all characters which are less than 128.

<a href="fn.encode_unquoted_attribute_to_vec.html" class="fn"
title="fn html_escape::encode_unquoted_attribute_to_vec">encode_unquoted_attribute_to_vec</a>  
Write text used in an unquoted attribute to a mutable `Vec<u8>`
reference and return the encoded data slice. Except for alphanumeric
characters, escape all characters which are less than 128.

<a href="fn.encode_unquoted_attribute_to_writer.html" class="fn"
title="fn html_escape::encode_unquoted_attribute_to_writer">encode_unquoted_attribute_to_writer</a>  
Write text used in an unquoted attribute to a writer. Except for
alphanumeric characters, escape all characters which are less than 128.
