Crate serde\_path\_to\_error Copy item path

<span class="sub-heading"><a href="../src/serde_path_to_error/lib.rs.html#1-202"
class="src">Source</a> </span>

Expand description

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/path-to-error) [![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/serde_path_to_error) [![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/serde_path_to_error)

  

Find out the path at which a deserialization error occurred. This crate
provides a wrapper that works with any existing Serde `Deserializer` and
exposes the chain of field names leading to the error.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use serde::Deserialize;
use std::collections::BTreeMap as Map;

#[derive(Deserialize)]
struct Package {
    name: String,
    dependencies: Map<String, Dependency>,
}

#[derive(Deserialize)]
struct Dependency {
    version: String,
}

fn main() {
    let j = r#"{
        "name": "demo",
        "dependencies": {
            "serde": {
                "version": 1
            }
        }
    }"#;

    // Some Deserializer.
    let jd = &mut serde_json::Deserializer::from_str(j);

    let result: Result<Package, _> = serde_path_to_error::deserialize(jd);
    match result {
        Ok(_) => panic!("expected a type error"),
        Err(err) => {
            let path = err.path().to_string();
            assert_eq!(path, "dependencies.serde.version");
        }
    }
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Deserializer.html" class="struct"
title="struct serde_path_to_error::Deserializer">Deserializer</a>  
Deserializer adapter that records path to deserialization errors.

<a href="struct.Error.html" class="struct"
title="struct serde_path_to_error::Error">Error</a>  
Original deserializer error together with the path at which it occurred.

<a href="struct.Path.html" class="struct"
title="struct serde_path_to_error::Path">Path</a>  
Path to the error value in the input, like `dependencies.serde.typo1`.

<a href="struct.Segments.html" class="struct"
title="struct serde_path_to_error::Segments">Segments</a>  
Iterator over segments of a path.

<a href="struct.Serializer.html" class="struct"
title="struct serde_path_to_error::Serializer">Serializer</a>  
Serializer adapter that records path to serialization errors.

<a href="struct.Track.html" class="struct"
title="struct serde_path_to_error::Track">Track</a>  
State for bookkeeping across nested deserializer calls.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Segment.html" class="enum"
title="enum serde_path_to_error::Segment">Segment</a>  
Single segment of a path.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.deserialize.html" class="fn"
title="fn serde_path_to_error::deserialize">deserialize</a>  
Entry point. See [crate
documentation](index.html "mod serde_path_to_error") for an example.

<a href="fn.serialize.html" class="fn"
title="fn serde_path_to_error::serialize">serialize</a>  
Entry point for tracking path to Serialize error.
