Crate js\_option Copy item path

<span class="sub-heading"><a href="../src/js_option/lib.rs.html#1-189" class="src">Source</a>
</span>

Expand description

This crate provides a type `JsOption` that is very similar to the
standard library’s `Option` type except that it has three variants:

-   `Some(value)`: Like `Option::Some`
-   `Null`: Explicitly not some value
-   `Undefined`: Implicitly not some value

This type can be useful when you want to deserialize JSON to a Rust
struct and not loose information: A regular `Option` deserializes to
`None` from both an explicit `null` or a missing field (this is due to
special casing of `Option` in the `Deserialize` and `Serialize` derive
macros, for other types a missing field will make deserialization fail
unless there is a `#[serde(skip)]`, `#[serde(skip_deserializing)]` or
`#[serde(default)]` attribute).

# <a href="#example" class="doc-anchor">§</a>Example:

``` {.rust .rust-example-rendered}
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    my_field: JsOption<String>,
}
```

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.JsOption.html" class="enum"
title="enum js_option::JsOption">JsOption</a>  
An `Option`-like type with two data-less variants in addition to `Some`:
`Null` and `Undefined`.
