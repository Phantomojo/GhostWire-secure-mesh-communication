Crate toml Copy item path

<span class="sub-heading"><a href="../src/toml/lib.rs.html#1-204" class="src">Source</a>
</span>

Expand description

A [serde](https://serde.rs/)-compatible
[TOML](https://github.com/toml-lang/toml)-parsing library

TOML itself is a simple, ergonomic, and readable configuration format:

``` language-toml
[package]
name = "toml"

[dependencies]
serde = "1.0"
```

The TOML format tends to be relatively common throughout the Rust
community for configuration, notably being used by
[Cargo](https://crates.io/), Rust’s package manager.

## <a href="#toml-values" class="doc-anchor">§</a>TOML values

A TOML document is represented with the
[`Table`](type.Table.html "type toml::Table") type which maps `String`
to the [`Value`](enum.Value.html "enum toml::Value") enum:

``` {.rust .rust-example-rendered}
pub enum Value {
   String(String),
   Integer(i64),
   Float(f64),
   Boolean(bool),
   Datetime(Datetime),
   Array(Array),
   Table(Table),
}
```

## <a href="#parsing-toml" class="doc-anchor">§</a>Parsing TOML

The easiest way to parse a TOML document is via the
[`Table`](type.Table.html "type toml::Table") type:

``` {.rust .rust-example-rendered}
use toml::Table;

let value = "foo = 'bar'".parse::<Table>().unwrap();

assert_eq!(value["foo"].as_str(), Some("bar"));
```

The [`Table`](type.Table.html "type toml::Table") type implements a
number of convenience methods and traits; the example above uses
[`FromStr`](https://doc.rust-lang.org/1.93.1/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr")
to parse a
[`str`](https://doc.rust-lang.org/1.93.1/std/primitive.str.html "primitive str")
into a [`Table`](type.Table.html "type toml::Table").

## <a href="#deserialization-and-serialization" class="doc-anchor">§</a>Deserialization and Serialization

This crate supports [`serde`](https://serde.rs/) 1.0 with a number of
implementations of the `Deserialize`, `Serialize`, `Deserializer`, and
`Serializer` traits. Namely, you’ll find:

-   `Deserialize for Table`
-   `Serialize for Table`
-   `Deserialize for Value`
-   `Serialize for Value`
-   `Deserialize for Datetime`
-   `Serialize for Datetime`
-   `Deserializer for de::Deserializer`
-   `Serializer for ser::Serializer`
-   `Deserializer for Table`
-   `Deserializer for Value`

This means that you can use Serde to deserialize/serialize the
[`Table`](type.Table.html "type toml::Table") type as well as
[`Value`](enum.Value.html "enum toml::Value") and
[`Datetime`](value/struct.Datetime.html "struct toml::value::Datetime")
type in this crate. You can also use the
[`Deserializer`](struct.Deserializer.html "struct toml::Deserializer"),
[`Serializer`](struct.Serializer.html "struct toml::Serializer"), or
[`Table`](type.Table.html "type toml::Table") type itself to act as a
deserializer/serializer for arbitrary types.

An example of deserializing with TOML is:

``` {.rust .rust-example-rendered}
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
   ip: String,
   port: Option<u16>,
   keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
   github: String,
   travis: Option<String>,
}

let config: Config = toml::from_str(r#"
   ip = '127.0.0.1'

   [keys]
   github = 'xxxxxxxxxxxxxxxxx'
   travis = 'yyyyyyyyyyyyyyyyy'
"#).unwrap();

assert_eq!(config.ip, "127.0.0.1");
assert_eq!(config.port, None);
assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
```

You can serialize types in a similar fashion:

``` {.rust .rust-example-rendered}
use serde::Serialize;

#[derive(Serialize)]
struct Config {
   ip: String,
   port: Option<u16>,
   keys: Keys,
}

#[derive(Serialize)]
struct Keys {
   github: String,
   travis: Option<String>,
}

let config = Config {
   ip: "127.0.0.1".to_string(),
   port: None,
   keys: Keys {
       github: "xxxxxxxxxxxxxxxxx".to_string(),
       travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
   },
};

let toml = toml::to_string(&config).unwrap();
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="de/index.html" class="mod" title="mod toml::de">de</a>  
Deserializing TOML into Rust structures.

<a href="map/index.html" class="mod" title="mod toml::map">map</a>  
A map of `String` to [Value](enum.Value.html "enum toml::Value").

<a href="ser/index.html" class="mod" title="mod toml::ser">ser</a>  
Serializing Rust structures into TOML.

<a href="value/index.html" class="mod" title="mod toml::value">value</a>  
Definition of a TOML [value](enum.Value.html "enum toml::Value")

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.toml.html" class="macro"
title="macro toml::toml">toml</a>  
Construct a [`Table`](type.Table.html "type toml::Table") from TOML
syntax.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Deserializer.html" class="struct"
title="struct toml::Deserializer">Deserializer</a>  
Deserialization for TOML
[documents](type.Table.html "type toml::Table").

<a href="struct.Serializer.html" class="struct"
title="struct toml::Serializer">Serializer</a>  
Serialization for TOML documents.

<a href="struct.Spanned.html" class="struct"
title="struct toml::Spanned">Spanned</a>  
A spanned value, indicating the range at which it is defined in the
source.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Value.html" class="enum"
title="enum toml::Value">Value</a>  
Representation of a TOML value.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.from_slice.html" class="fn"
title="fn toml::from_slice">from_slice</a>  
Deserializes bytes into a type.

<a href="fn.from_str.html" class="fn"
title="fn toml::from_str">from_str</a>  
Deserializes a string into a type.

<a href="fn.to_string.html" class="fn"
title="fn toml::to_string">to_string</a>  
Serialize the given data structure as a String of TOML.

<a href="fn.to_string_pretty.html" class="fn"
title="fn toml::to_string_pretty">to_string_pretty</a>  
Serialize the given data structure as a “pretty” String of TOML.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Table.html" class="type"
title="type toml::Table">Table</a>  
Type representing a TOML table, payload of the `Value::Table` variant.
