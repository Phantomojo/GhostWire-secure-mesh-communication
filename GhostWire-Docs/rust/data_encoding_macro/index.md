Crate data\_encoding\_macro Copy item path

<span class="sub-heading"><a href="../src/data_encoding_macro/lib.rs.html#1-265"
class="src">Source</a> </span>

Expand description

Macros for data-encoding

This library provides macros to define compile-time byte arrays from
encoded strings (using common bases like [base64](macro.base64.html),
[base32](macro.base32.html), or
[hexadecimal](macro.hexlower_permissive.html), and also custom bases).
It also provides a macro to define compile-time custom encodings to be
used with the [data-encoding](https://crates.io/crates/data-encoding)
crate at run-time.

Up to Rust 1.50, you may need to add the following to your
`.cargo/config.toml` to use this library in no-std or no-alloc
environments:

``` language-toml
[unstable]
features = ["host_dep"]
```

From Rust 1.51, you may need to add the following to your `Cargo.toml`:

``` language-toml
[package]
resolver = "2"
```

# <a href="#examples" class="doc-anchor">§</a>Examples

You can define a compile-time byte slice from an encoded string literal:

``` {.rust .rust-example-rendered}
const HELLO_SLICE: &'static [u8] = &data_encoding_macro::hexlower!("68656c6c6f");
const FOOBAR_SLICE: &'static [u8] = &data_encoding_macro::base64!("Zm9vYmFy");
```

You can also define a compile-time byte array from an encoded string
literal:

``` {.rust .rust-example-rendered}
data_encoding_macro::hexlower_array!("const HELLO" = "68656c6c6f");
data_encoding_macro::base64_array!("const FOOBAR" = "Zm9vYmFy");
```

You can define a compile-time custom encoding from its specification:

``` {.rust .rust-example-rendered}
const HEX: data_encoding::Encoding = data_encoding_macro::new_encoding! {
    symbols: "0123456789abcdef",
    translate_from: "ABCDEF",
    translate_to: "abcdef",
};
const BASE64: data_encoding::Encoding = data_encoding_macro::new_encoding! {
    symbols: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    padding: '=',
};
```

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.base32.html" class="macro"
title="macro data_encoding_macro::base32">base32</a>  
<a href="macro.base64.html" class="macro"
title="macro data_encoding_macro::base64">base64</a>  
<a href="macro.base32_array.html" class="macro"
title="macro data_encoding_macro::base32_array">base32_array</a>  
<a href="macro.base32_dnscurve.html" class="macro"
title="macro data_encoding_macro::base32_dnscurve">base32_dnscurve</a>  
<a href="macro.base32_dnscurve_array.html" class="macro"
title="macro data_encoding_macro::base32_dnscurve_array">base32_dnscurve_array</a>  
<a href="macro.base32_dnssec.html" class="macro"
title="macro data_encoding_macro::base32_dnssec">base32_dnssec</a>  
<a href="macro.base32_dnssec_array.html" class="macro"
title="macro data_encoding_macro::base32_dnssec_array">base32_dnssec_array</a>  
<a href="macro.base32_nopad.html" class="macro"
title="macro data_encoding_macro::base32_nopad">base32_nopad</a>  
<a href="macro.base32_nopad_array.html" class="macro"
title="macro data_encoding_macro::base32_nopad_array">base32_nopad_array</a>  
<a href="macro.base32hex.html" class="macro"
title="macro data_encoding_macro::base32hex">base32hex</a>  
<a href="macro.base32hex_array.html" class="macro"
title="macro data_encoding_macro::base32hex_array">base32hex_array</a>  
<a href="macro.base32hex_nopad.html" class="macro"
title="macro data_encoding_macro::base32hex_nopad">base32hex_nopad</a>  
<a href="macro.base32hex_nopad_array.html" class="macro"
title="macro data_encoding_macro::base32hex_nopad_array">base32hex_nopad_array</a>  
<a href="macro.base64_array.html" class="macro"
title="macro data_encoding_macro::base64_array">base64_array</a>  
<a href="macro.base64_mime.html" class="macro"
title="macro data_encoding_macro::base64_mime">base64_mime</a>  
<a href="macro.base64_mime_array.html" class="macro"
title="macro data_encoding_macro::base64_mime_array">base64_mime_array</a>  
<a href="macro.base64_mime_permissive.html" class="macro"
title="macro data_encoding_macro::base64_mime_permissive">base64_mime_permissive</a>  
<a href="macro.base64_mime_permissive_array.html" class="macro"
title="macro data_encoding_macro::base64_mime_permissive_array">base64_mime_permissive_array</a>  
<a href="macro.base64_nopad.html" class="macro"
title="macro data_encoding_macro::base64_nopad">base64_nopad</a>  
<a href="macro.base64_nopad_array.html" class="macro"
title="macro data_encoding_macro::base64_nopad_array">base64_nopad_array</a>  
<a href="macro.base64url.html" class="macro"
title="macro data_encoding_macro::base64url">base64url</a>  
<a href="macro.base64url_array.html" class="macro"
title="macro data_encoding_macro::base64url_array">base64url_array</a>  
<a href="macro.base64url_nopad.html" class="macro"
title="macro data_encoding_macro::base64url_nopad">base64url_nopad</a>  
<a href="macro.base64url_nopad_array.html" class="macro"
title="macro data_encoding_macro::base64url_nopad_array">base64url_nopad_array</a>  
<a href="macro.decode_array.html" class="macro"
title="macro data_encoding_macro::decode_array">decode_array</a>  
Defines a compile-time byte array by decoding a string literal

<a href="macro.decode_slice.html" class="macro"
title="macro data_encoding_macro::decode_slice">decode_slice</a>  
Defines a compile-time byte slice by decoding a string literal

<a href="macro.hexlower.html" class="macro"
title="macro data_encoding_macro::hexlower">hexlower</a>  
<a href="macro.hexlower_array.html" class="macro"
title="macro data_encoding_macro::hexlower_array">hexlower_array</a>  
<a href="macro.hexlower_permissive.html" class="macro"
title="macro data_encoding_macro::hexlower_permissive">hexlower_permissive</a>  
<a href="macro.hexlower_permissive_array.html" class="macro"
title="macro data_encoding_macro::hexlower_permissive_array">hexlower_permissive_array</a>  
<a href="macro.hexupper.html" class="macro"
title="macro data_encoding_macro::hexupper">hexupper</a>  
<a href="macro.hexupper_array.html" class="macro"
title="macro data_encoding_macro::hexupper_array">hexupper_array</a>  
<a href="macro.hexupper_permissive.html" class="macro"
title="macro data_encoding_macro::hexupper_permissive">hexupper_permissive</a>  
<a href="macro.hexupper_permissive_array.html" class="macro"
title="macro data_encoding_macro::hexupper_permissive_array">hexupper_permissive_array</a>  
<a href="macro.new_encoding.html" class="macro"
title="macro data_encoding_macro::new_encoding">new_encoding</a>  
Defines a compile-time custom encoding
