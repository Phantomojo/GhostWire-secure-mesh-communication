Crate serde\_html\_form Copy item path

<span class="sub-heading"><a href="../src/serde_html_form/lib.rs.html#1-24" class="src">Source</a>
</span>

Expand description

# <a href="#serde_html_form" class="doc-anchor">§</a>`serde_html_form`

(De-)serialization support for the `application/x-www-form-urlencoded`
format.

This crate is a Rust library for serialising to and deserialising from
the
[`application/x-www-form-urlencoded`](https://url.spec.whatwg.org/#application/x-www-form-urlencoded)
format. It is built upon [Serde](https://github.com/serde-rs/serde), a
high performance generic serialization framework and
[rust-url](https://github.com/servo/rust-url), a URL parser for Rust.

It is a fork of
[`serde_urlencoded`](https://github.com/nox/serde_urlencoded), with
additional support for maps or structs with fields of sequence type
(e.g. `Vec<String>`). It also supports `Option` in values, treating
`foo=` as `foo: None`.

## <a href="#examples" class="doc-anchor">§</a>Examples

Sequences like `value=x&value=y`:

``` {.rust .rust-example-rendered}
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
struct Form {
    // By default, at least one occurrence of this field must be present (this
    // is mandated by how serde works).
    //
    // Since this is usually not desired, use `serde(default)` to instantiate
    // this struct's field with a `Default` value if input doesn't contain that
    // field.
    #[serde(default)]
    value: Vec<String>,
}

assert_eq!(
    serde_html_form::from_str("value=&value=abc"),
    Ok(Form { value: vec!["".to_owned(), "abc".to_owned()] })
);
assert_eq!(
    serde_html_form::from_str(""),
    Ok(Form { value: vec![] })
);
```

Sequences like `value[]=x&value[]=y`:

``` {.rust .rust-example-rendered}
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
struct Form {
    // If you want to support `value[]=x&value[]=y`, you can use
    // `serde(rename)`. You could even use `serde(alias)` instead to allow both,
    // but note that mixing both in one input string would also be allowed then.
    #[serde(default, rename = "value[]")]
    value: Vec<String>,
}

assert_eq!(
    serde_html_form::from_str("value[]=x&value[]=y"),
    Ok(Form { value: vec!["x".to_owned(), "y".to_owned()] })
);
assert_eq!(
    serde_html_form::from_str("value[]=hello"),
    Ok(Form { value: vec!["hello".to_owned()] })
);
```

Optional values:

``` {.rust .rust-example-rendered}
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
struct Form {
    // Finally, this crate also supports deserializing empty values as `None`
    // if your values are `Option`s.
    // Note that serde's `Deserialize` derive implicitly allows omission of
    // `Option`-typed fields (except when combined with some other attributes).
    single: Option<u32>,
    // Not using `serde(default)` here to require at least one occurrence.
    at_least_one: Vec<Option<u32>>,
}

assert_eq!(
    serde_html_form::from_str("at_least_one=5"),
    Ok(Form {
        // Implicit `serde(default)` in action.
        single: None,
        // `serde_html_form`'s support for optional values being used.
        at_least_one: vec![Some(5)],
    })
);
assert_eq!(
    serde_html_form::from_str("at_least_one=&single=1&at_least_one=5"),
    Ok(Form {
        single: Some(1),
        at_least_one: vec![
            // Empty strings get deserialized as `None`.
            None,
            // It's no problem that the `at_least_one` field repetitions are
            // not consecutive (single comes in between).
            Some(5),
        ]
    })
);
assert!(
    serde_html_form::from_str::<Form>("").is_err(),
    "at_least_one is not part of the input"
);
```

## <a href="#license" class="doc-anchor">§</a>License

This crate is licensed under the MIT license ([LICENSE](LICENSE) or
<https://opensource.org/license/mit/>).

# Modules<a href="#modules" class="anchor">§</a>

<a href="de/index.html" class="mod"
title="mod serde_html_form::de">de</a>  
Deserialization support for the `application/x-www-form-urlencoded`
format.

<a href="ser/index.html" class="mod"
title="mod serde_html_form::ser">ser</a>  
Serialization support for the `application/x-www-form-urlencoded`
format.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Deserializer.html" class="struct"
title="struct serde_html_form::Deserializer">Deserializer</a>  
A deserializer for the `application/x-www-form-urlencoded` format.

<a href="struct.Serializer.html" class="struct"
title="struct serde_html_form::Serializer">Serializer</a>  
A serializer for the `application/x-www-form-urlencoded` format.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.from_bytes.html" class="fn"
title="fn serde_html_form::from_bytes">from_bytes</a>  
Deserializes a `application/x-www-form-urlencoded` value from a `&[u8]`.

<a href="fn.from_reader.html" class="fn"
title="fn serde_html_form::from_reader">from_reader</a>  
Convenience function that reads all bytes from `reader` and deserializes
them with `from_bytes`.

<a href="fn.from_str.html" class="fn"
title="fn serde_html_form::from_str">from_str</a>  
Deserializes a `application/x-www-form-urlencoded` value from a `&str`.

<a href="fn.push_to_string.html" class="fn"
title="fn serde_html_form::push_to_string">push_to_string</a>  
Serializes a value into the provided `application/x-www-form-urlencoded`
`String` buffer.

<a href="fn.to_string.html" class="fn"
title="fn serde_html_form::to_string">to_string</a>  
Serializes a value into a `application/x-www-form-urlencoded` `String`
buffer.
