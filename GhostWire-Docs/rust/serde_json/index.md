Crate serde\_json Copy item path

<span class="sub-heading"><a href="../src/serde_json/lib.rs.html#1-441" class="src">Source</a>
</span>

Expand description

# <a href="#serde-json" class="doc-anchor">§</a>Serde JSON

JSON is a ubiquitous open-standard format that uses human-readable text
to transmit data objects consisting of key-value pairs.

``` language-json
{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    },
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}
```

There are three common ways that you might find yourself needing to work
with JSON data in Rust.

-   **As text data.** An unprocessed string of JSON data that you
    receive on an HTTP endpoint, read from a file, or prepare to send to
    a remote server.
-   **As an untyped or loosely typed representation.** Maybe you want to
    check that some JSON data is valid before passing it on, but without
    knowing the structure of what it contains. Or you want to do very
    basic manipulations like insert a key in a particular spot.
-   **As a strongly typed Rust data structure.** When you expect all or
    most of your data to conform to a particular structure and want to
    get real work done without JSON’s loosey-goosey nature tripping you
    up.

Serde JSON provides efficient, flexible, safe ways of converting data
between each of these representations.

# <a href="#operating-on-untyped-json-values" class="doc-anchor">§</a>Operating on untyped JSON values

Any valid JSON data can be manipulated in the following recursive enum
representation. This data structure is
[`serde_json::Value`](enum.Value.html "enum serde_json::Value").

``` {.rust .rust-example-rendered}
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

A string of JSON data can be parsed into a `serde_json::Value` by the
[`serde_json::from_str`](fn.from_str.html "fn serde_json::from_str")
function. There is also
[`from_slice`](fn.from_slice.html "fn serde_json::from_slice") for
parsing from a byte slice `&[u8]` and
[`from_reader`](fn.from_reader.html "fn serde_json::from_reader") for
parsing from any `io::Read` like a File or a TCP stream.

``` {.rust .rust-example-rendered}
use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
```

The result of square bracket indexing like `v["name"]` is a borrow of
the data at that index, so the type is `&Value`. A JSON map can be
indexed with string keys, while a JSON array can be indexed with integer
keys. If the type of the data is not right for the type with which it is
being indexed, or if a map does not contain the key being indexed, or if
the index into a vector is out of bounds, the returned element is
`Value::Null`.

When a `Value` is printed, it is printed as a JSON string. So in the
code above, the output looks like
`Please call "John Doe" at the number "+44 1234567"`. The quotation
marks appear because `v["name"]` is a `&Value` containing a JSON string
and its JSON representation is `"John Doe"`. Printing as a plain string
without quotation marks involves converting from a JSON string to a Rust
string with
[`as_str()`](enum.Value.html#method.as_str "method serde_json::Value::as_str")
or avoiding the use of `Value` as described in the following section.

The `Value` representation is sufficient for very basic tasks but can be
tedious to work with for anything more significant. Error handling is
verbose to implement correctly, for example imagine trying to detect the
presence of unrecognized fields in the input data. The compiler is
powerless to help you when you make a mistake, for example imagine
typoing `v["name"]` as `v["nmae"]` in one of the dozens of places it is
used in your code.

# <a href="#parsing-json-as-strongly-typed-data-structures"
class="doc-anchor">§</a>Parsing JSON as strongly typed data structures

Serde provides a powerful way of mapping JSON data into Rust data
structures largely automatically.

``` {.rust .rust-example-rendered}
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
```

This is the same `serde_json::from_str` function as before, but this
time we assign the return value to a variable of type `Person` so Serde
will automatically interpret the input data as a `Person` and produce
informative error messages if the layout does not conform to what a
`Person` is expected to look like.

Any type that implements Serde’s `Deserialize` trait can be deserialized
this way. This includes built-in Rust standard library types like
`Vec<T>` and `HashMap<K, V>`, as well as any structs or enums annotated
with `#[derive(Deserialize)]`.

Once we have `p` of type `Person`, our IDE and the Rust compiler can
help us use it correctly like they do for any other Rust code. The IDE
can autocomplete field names to prevent typos, which was impossible in
the `serde_json::Value` representation. And the Rust compiler can check
that when we write `p.phones[0]`, then `p.phones` is guaranteed to be a
`Vec<String>` so indexing into it makes sense and produces a `String`.

# <a href="#constructing-json-values" class="doc-anchor">§</a>Constructing JSON values

Serde JSON provides a [`json!`
macro](macro.json.html "macro serde_json::json") to build
`serde_json::Value` objects with very natural JSON syntax.

``` {.rust .rust-example-rendered}
use serde_json::json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
```

The `Value::to_string()` function converts a `serde_json::Value` into a
`String` of JSON text.

One neat thing about the `json!` macro is that variables and expressions
can be interpolated directly into the JSON value as you are building it.
Serde will check at compile time that the value you are interpolating is
able to be represented as JSON.

``` {.rust .rust-example-rendered}
let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `serde_json::Value`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

This is amazingly convenient, but we have the problem we had before with
`Value`: the IDE and Rust compiler cannot help us if we get it wrong.
Serde JSON provides a better way of serializing strongly-typed data
structures into JSON text.

# <a href="#creating-json-by-serializing-data-structures"
class="doc-anchor">§</a>Creating JSON by serializing data structures

A data structure can be converted to a JSON string by
[`serde_json::to_string`](fn.to_string.html "fn serde_json::to_string").
There is also
[`serde_json::to_vec`](fn.to_vec.html "fn serde_json::to_vec") which
serializes to a `Vec<u8>` and
[`serde_json::to_writer`](fn.to_writer.html "fn serde_json::to_writer")
which serializes to any `io::Write` such as a File or a TCP stream.

``` {.rust .rust-example-rendered}
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}
```

Any type that implements Serde’s `Serialize` trait can be serialized
this way. This includes built-in Rust standard library types like
`Vec<T>` and `HashMap<K, V>`, as well as any structs or enums annotated
with `#[derive(Serialize)]`.

# <a href="#no-std-support" class="doc-anchor">§</a>No-std support

As long as there is a memory allocator, it is possible to use
serde\_json without the rest of the Rust standard library. Disable the
default “std” feature and enable the “alloc” feature:

``` language-toml
[dependencies]
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
```

For JSON support in Serde without a memory allocator, please see the
[`serde-json-core`](https://github.com/rust-embedded-community/serde-json-core)
crate.

# Modules<a href="#modules" class="anchor">§</a>

<a href="de/index.html" class="mod" title="mod serde_json::de">de</a>  
Deserialize JSON data to a Rust data structure.

<a href="error/index.html" class="mod"
title="mod serde_json::error">error</a>  
When serializing or deserializing JSON goes wrong.

<a href="map/index.html" class="mod" title="mod serde_json::map">map</a>  
A map of String to serde\_json::Value.

<a href="ser/index.html" class="mod" title="mod serde_json::ser">ser</a>  
Serialize a Rust data structure into JSON data.

<a href="value/index.html" class="mod"
title="mod serde_json::value">value</a>  
The Value enum, a loosely typed way of representing any valid JSON
value.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.json.html" class="macro"
title="macro serde_json::json">json</a>  
Construct a `serde_json::Value` from a JSON literal.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Deserializer.html" class="struct"
title="struct serde_json::Deserializer">Deserializer</a>  
A structure that deserializes JSON into Rust values.

<a href="struct.Error.html" class="struct"
title="struct serde_json::Error">Error</a>  
This type represents all possible errors that can occur when serializing
or deserializing JSON data.

<a href="struct.Map.html" class="struct"
title="struct serde_json::Map">Map</a>  
Represents a JSON key/value type.

<a href="struct.Number.html" class="struct"
title="struct serde_json::Number">Number</a>  
Represents a JSON number, whether integer or floating point.

<a href="struct.Serializer.html" class="struct"
title="struct serde_json::Serializer">Serializer</a>  
A structure for serializing Rust values into JSON.

<a href="struct.StreamDeserializer.html" class="struct"
title="struct serde_json::StreamDeserializer">StreamDeserializer</a>  
Iterator that deserializes a stream into multiple JSON values.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Value.html" class="enum"
title="enum serde_json::Value">Value</a>  
Represents any valid JSON value.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.from_reader.html" class="fn"
title="fn serde_json::from_reader">from_reader</a>  
Deserialize an instance of type `T` from an I/O stream of JSON.

<a href="fn.from_slice.html" class="fn"
title="fn serde_json::from_slice">from_slice</a>  
Deserialize an instance of type `T` from bytes of JSON text.

<a href="fn.from_str.html" class="fn"
title="fn serde_json::from_str">from_str</a>  
Deserialize an instance of type `T` from a string of JSON text.

<a href="fn.from_value.html" class="fn"
title="fn serde_json::from_value">from_value</a>  
Interpret a `serde_json::Value` as an instance of type `T`.

<a href="fn.to_string.html" class="fn"
title="fn serde_json::to_string">to_string</a>  
Serialize the given data structure as a String of JSON.

<a href="fn.to_string_pretty.html" class="fn"
title="fn serde_json::to_string_pretty">to_string_pretty</a>  
Serialize the given data structure as a pretty-printed String of JSON.

<a href="fn.to_value.html" class="fn"
title="fn serde_json::to_value">to_value</a>  
Convert a `T` into `serde_json::Value` which is an enum that can
represent any valid JSON data.

<a href="fn.to_vec.html" class="fn"
title="fn serde_json::to_vec">to_vec</a>  
Serialize the given data structure as a JSON byte vector.

<a href="fn.to_vec_pretty.html" class="fn"
title="fn serde_json::to_vec_pretty">to_vec_pretty</a>  
Serialize the given data structure as a pretty-printed JSON byte vector.

<a href="fn.to_writer.html" class="fn"
title="fn serde_json::to_writer">to_writer</a>  
Serialize the given data structure as JSON into the I/O stream.

<a href="fn.to_writer_pretty.html" class="fn"
title="fn serde_json::to_writer_pretty">to_writer_pretty</a>  
Serialize the given data structure as pretty-printed JSON into the I/O
stream.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type serde_json::Result">Result</a>  
Alias for a `Result` with the error type `serde_json::Error`.
