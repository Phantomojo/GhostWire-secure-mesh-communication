Crate js\_int Copy item path

<span class="sub-heading"><a href="../src/js_int/lib.rs.html#1-68" class="src">Source</a>
</span>

Expand description

Crate `js_int` provides JavaScript-interoperable integer types.

JavaScript does not have native integers. Instead it represents all
numeric values with a single `Number` type which is represented as an
[IEEE 754 floating-point](https://en.wikipedia.org/wiki/IEEE_754)
value.\* Rust’s `i64` and `u64` types can contain values outside the
range of what can be represented in a JavaScript `Number`.

This crate provides the types `Int` and `UInt` which wrap `i64` and
`u64`, respectively. These types add bounds checking to ensure the
contained value is within the range representable by a JavaScript
`Number`. They provide useful trait implementations to easily convert
from Rust’s primitive integer types.

<span class="small">\* In the upcoming ECMAScript 2020, JavaScript will
probably gain support for integers. There is a proposal for a
[`BigInt`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt)
type type that is not far from becoming part of the JavaScript
specification. It won’t make this crate obsolete in any way though,
since there will still be lots of JS code using `Number`, and code in
other languages that assumes its use.</span>

# <a href="#no_std" class="doc-anchor">§</a>`#![no_std]`

The `js_int` crate does not use Rust’s standard library, and is
compatible with `#![no_std]` programs.

# <a href="#features" class="doc-anchor">§</a>Features

-   `serde`: Serialization and deserialization support via
    [serde](https://serde.rs). Disabled by default. You can use
    `js_int` + `serde` in `#![no_std]` contexts if you use
    `default-features = false` for both.
-   `float_deserialize`: Deserialize via `f64`, not via `u64`. If the
    input has a fraction, deserialization will fail.
-   `lax_deserialize`: Like `float_deserialize`, but if the input has a
    fraction, it is deserialized with the fractional part discarded.
    Please be aware that `serde_json` doesn’t losslessly parse large
    floats with a fractional part by default (even if the fractional
    part is `.0`). To fix that, enable its `float_roundtrip` feature.
-   `std`: Enable `std::error::Error` implementations for
    `ParseIntError`, `TryFromIntError`. Enabled by default.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.int.html" class="macro" title="macro js_int::int">int</a>  
Creates an `Int` from a numeric literal.

<a href="macro.uint.html" class="macro"
title="macro js_int::uint">uint</a>  
Creates a `UInt` from a numeric literal.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Int.html" class="struct"
title="struct js_int::Int">Int</a>  
An integer limited to the range of integers that can be represented
exactly by an f64.

<a href="struct.ParseIntError.html" class="struct"
title="struct js_int::ParseIntError">ParseIntError</a>  
The error type returned when when parsing an integer fails.

<a href="struct.TryFromIntError.html" class="struct"
title="struct js_int::TryFromIntError">TryFromIntError</a>  
The error type returned when a checked integral type conversion fails.

<a href="struct.UInt.html" class="struct"
title="struct js_int::UInt">UInt</a>  
An integer limited to the range of non-negative integers that can be
represented exactly by an f64.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.MAX_SAFE_INT.html" class="constant"
title="constant js_int::MAX_SAFE_INT">MAX_SAFE_INT</a>  
The largest integer value that can be represented exactly by an f64.

<a href="constant.MAX_SAFE_UINT.html" class="constant"
title="constant js_int::MAX_SAFE_UINT">MAX_SAFE_UINT</a>  
The same as `MAX_SAFE_INT`, but with `u64` as the type.

<a href="constant.MIN_SAFE_INT.html" class="constant"
title="constant js_int::MIN_SAFE_INT">MIN_SAFE_INT</a>  
The smallest integer value that can be represented exactly by an f64.
