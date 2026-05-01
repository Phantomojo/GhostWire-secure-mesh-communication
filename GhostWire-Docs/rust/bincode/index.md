Crate bincode Copy item path

<span class="sub-heading"><a href="../src/bincode/lib.rs.html#1-201" class="src">Source</a>
</span>

Expand description

Bincode is a crate for encoding and decoding using a tiny binary
serialization strategy. Using it, you can easily go from having an
object in memory, quickly serialize it to bytes, and then deserialize it
back just as fast!

### <a href="#using-basic-functions" class="doc-anchor">§</a>Using Basic Functions

<a href="#" class="tooltip"
title="This example runs with edition 2018">ⓘ</a>

``` {.rust .rust-example-rendered}
fn main() {
    // The object that we will serialize.
    let target: Option<String>  = Some("hello world".to_string());

    let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
    let decoded: Option<String> = bincode::deserialize(&encoded[..]).unwrap();
    assert_eq!(target, decoded);
}
```

### <a href="#128bit-numbers" class="doc-anchor">§</a>128bit numbers

Support for `i128` and `u128` is automatically enabled on Rust
toolchains greater than or equal to `1.26.0` and disabled for targets
which do not support it

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use config::`<a href="config/struct.Config.html" class="struct"
title="struct bincode::config::Config"><code>Config</code></a>`;`<span class="stab deprecated"
title="">Deprecated</span>

`pub use config::`<a href="config/struct.DefaultOptions.html" class="struct"
title="struct bincode::config::DefaultOptions"><code>DefaultOptions</code></a>`;`

`pub use config::`<a href="config/trait.Options.html" class="trait"
title="trait bincode::config::Options"><code>Options</code></a>`;`

`pub use de::read::`<a href="de/read/trait.BincodeRead.html" class="trait"
title="trait bincode::de::read::BincodeRead"><code>BincodeRead</code></a>`;`

`pub use de::`<a href="de/struct.Deserializer.html" class="struct"
title="struct bincode::de::Deserializer"><code>Deserializer</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="config/index.html" class="mod"
title="mod bincode::config">config</a>  
`bincode` uses a Builder-pattern to configure the Serializers and
Deserializers in this crate. This means that if you need to customize
the behavior of `bincode`, you should create an instance of the
`DefaultOptions` struct:

<a href="de/index.html" class="mod" title="mod bincode::de">de</a>  
Deserialize bincode data to a Rust data structure.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Serializer.html" class="struct"
title="struct bincode::Serializer">Serializer</a>  
An Serializer that encodes values directly into a Writer.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ErrorKind.html" class="enum"
title="enum bincode::ErrorKind">ErrorKind</a>  
The kind of error that can be produced during a serialization or
deserialization.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.config.html" class="fn"
title="fn bincode::config">config</a><span class="stab deprecated" title="">Deprecated</span>  
Get a default configuration object.

<a href="fn.deserialize.html" class="fn"
title="fn bincode::deserialize">deserialize</a>  
Deserializes a slice of bytes into an instance of `T` using the default
configuration.

<a href="fn.deserialize_from.html" class="fn"
title="fn bincode::deserialize_from">deserialize_from</a>  
Deserializes an object directly from a `Read`er using the default
configuration.

<a href="fn.deserialize_from_custom.html" class="fn"
title="fn bincode::deserialize_from_custom">deserialize_from_custom</a>  
Deserializes an object from a custom `BincodeRead`er using the default
configuration. It is highly recommended to use `deserialize_from` unless
you need to implement `BincodeRead` for performance reasons.

<a href="fn.options.html" class="fn"
title="fn bincode::options">options</a>  
Get a default configuration object.

<a href="fn.serialize.html" class="fn"
title="fn bincode::serialize">serialize</a>  
Serializes a serializable object into a `Vec` of bytes using the default
configuration.

<a href="fn.serialize_into.html" class="fn"
title="fn bincode::serialize_into">serialize_into</a>  
Serializes an object directly into a `Writer` using the default
configuration.

<a href="fn.serialized_size.html" class="fn"
title="fn bincode::serialized_size">serialized_size</a>  
Returns the size that an object would be if serialized using Bincode
with the default configuration.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Error.html" class="type"
title="type bincode::Error">Error</a>  
An error that can be produced during (de)serializing.

<a href="type.Result.html" class="type"
title="type bincode::Result">Result</a>  
The result of a serialization or deserialization operation.
