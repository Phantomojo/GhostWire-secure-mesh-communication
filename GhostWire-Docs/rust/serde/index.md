Crate serde Copy item path

<span class="sub-heading"><a href="../src/serde/lib.rs.html#1-285" class="src">Source</a>
</span>

Expand description

# <a href="#serde" class="doc-anchor">§</a>Serde

Serde is a framework for ***ser***ializing and ***de***serializing Rust
data structures efficiently and generically.

The Serde ecosystem consists of data structures that know how to
serialize and deserialize themselves along with data formats that know
how to serialize and deserialize other things. Serde provides the layer
by which these two groups interact with each other, allowing any
supported data structure to be serialized and deserialized using any
supported data format.

See the Serde website <https://serde.rs> for additional documentation
and usage examples.

## <a href="#design" class="doc-anchor">§</a>Design

Where many other languages rely on runtime reflection for serializing
data, Serde is instead built on Rust’s powerful trait system. A data
structure that knows how to serialize and deserialize itself is one that
implements Serde’s `Serialize` and `Deserialize` traits (or uses Serde’s
derive attribute to automatically generate implementations at compile
time). This avoids any overhead of reflection or runtime type
information. In fact in many situations the interaction between data
structure and data format can be completely optimized away by the Rust
compiler, leaving Serde serialization to perform the same speed as a
handwritten serializer for the specific selection of data structure and
data format.

## <a href="#data-formats" class="doc-anchor">§</a>Data formats

The following is a partial list of data formats that have been
implemented for Serde by the community.

-   [JSON](https://github.com/serde-rs/json), the ubiquitous JavaScript
    Object Notation used by many HTTP APIs.
-   [Postcard](https://github.com/jamesmunns/postcard), a no\_std and
    embedded-systems friendly compact binary format.
-   [CBOR](https://github.com/enarx/ciborium), a Concise Binary Object
    Representation designed for small message size without the need for
    version negotiation.
-   [YAML](https://github.com/dtolnay/serde-yaml), a self-proclaimed
    human-friendly configuration language that ain’t markup language.
-   [MessagePack](https://github.com/3Hren/msgpack-rust), an efficient
    binary format that resembles a compact JSON.
-   [TOML](https://docs.rs/toml), a minimal configuration format used by
    [Cargo](https://doc.rust-lang.org/cargo/reference/manifest.html).
-   [Pickle](https://github.com/birkenfeld/serde-pickle), a format
    common in the Python world.
-   [RON](https://github.com/ron-rs/ron), a Rusty Object Notation.
-   [BSON](https://github.com/mongodb/bson-rust), the data storage and
    network transfer format used by MongoDB.
-   [Avro](https://docs.rs/apache-avro), a binary format used within
    Apache Hadoop, with support for schema definition.
-   [JSON5](https://github.com/callum-oakley/json5-rs), a superset of
    JSON including some productions from ES5.
-   [URL](https://docs.rs/serde_qs) query strings, in the
    x-www-form-urlencoded format.
-   [Starlark](https://github.com/dtolnay/serde-starlark), the format
    used for describing build targets by the Bazel and Buck build
    systems. *(serialization only)*
-   [Envy](https://github.com/softprops/envy), a way to deserialize
    environment variables into Rust structs. *(deserialization only)*
-   [Envy Store](https://github.com/softprops/envy-store), a way to
    deserialize [AWS Parameter
    Store](https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-parameter-store.html)
    parameters into Rust structs. *(deserialization only)*
-   [S-expressions](https://github.com/rotty/lexpr-rs), the textual
    representation of code and data used by the Lisp language family.
-   [D-Bus](https://docs.rs/zvariant)’s binary wire format.
-   [FlexBuffers](https://github.com/google/flatbuffers/tree/master/rust/flexbuffers),
    the schemaless cousin of Google’s FlatBuffers zero-copy
    serialization format.
-   [Bencode](https://github.com/P3KI/bendy), a simple binary format
    used in the BitTorrent protocol.
-   [Token streams](https://github.com/oxidecomputer/serde_tokenstream),
    for processing Rust procedural macro input. *(deserialization only)*
-   [DynamoDB Items](https://docs.rs/serde_dynamo), the format used by
    [rusoto\_dynamodb](https://docs.rs/rusoto_dynamodb) to transfer data
    to and from DynamoDB.
-   [Hjson](https://github.com/Canop/deser-hjson), a syntax extension to
    JSON designed around human reading and editing. *(deserialization
    only)*
-   [CSV](https://docs.rs/csv), Comma-separated values is a tabular text
    file format.

# Modules<a href="#modules" class="anchor">§</a>

<a href="de/index.html" class="mod" title="mod serde::de">de</a>  
Generic data structure deserialization framework.

<a href="ser/index.html" class="mod" title="mod serde::ser">ser</a>  
Generic data structure serialization framework.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.forward_to_deserialize_any.html" class="macro"
title="macro serde::forward_to_deserialize_any">forward_to_deserialize_any</a>  
Helper macro when implementing the `Deserializer` part of a new data
format for Serde.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Deserialize.html" class="trait"
title="trait serde::Deserialize">Deserialize</a>  
A **data structure** that can be deserialized from any data format
supported by Serde.

<a href="trait.Deserializer.html" class="trait"
title="trait serde::Deserializer">Deserializer</a>  
A **data format** that can deserialize any data structure supported by
Serde.

<a href="trait.Serialize.html" class="trait"
title="trait serde::Serialize">Serialize</a>  
A **data structure** that can be serialized into any data format
supported by Serde.

<a href="trait.Serializer.html" class="trait"
title="trait serde::Serializer">Serializer</a>  
A **data format** that can serialize any data structure supported by
Serde.

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.Deserialize.html" class="derive"
title="derive serde::Deserialize">Deserialize</a>  
Derive macro available if serde is built with `features = ["derive"]`.

<a href="derive.Serialize.html" class="derive"
title="derive serde::Serialize">Serialize</a>  
Derive macro available if serde is built with `features = ["derive"]`.
