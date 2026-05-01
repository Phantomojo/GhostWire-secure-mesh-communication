Crate rmp\_serde Copy item path

<span class="sub-heading"><a href="../src/rmp_serde/lib.rs.html#1-331" class="src">Source</a>
</span>

Expand description

# <a href="#messagepack--serde" class="doc-anchor">§</a>MessagePack + Serde

This crate connects Rust MessagePack library with
[`serde`](https://serde.rs/) providing an ability to easily serialize
and deserialize both Rust built-in types, the standard library and
custom data structures.

## <a href="#motivating-example" class="doc-anchor">§</a>Motivating example

``` {.rust .rust-example-rendered}
let buf = rmp_serde::to_vec(&(42, "the Answer")).unwrap();

assert_eq!(
    vec![0x92, 0x2a, 0xaa, 0x74, 0x68, 0x65, 0x20, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72],
    buf
);

assert_eq!((42, "the Answer"), rmp_serde::from_slice(&buf).unwrap());
```

## <a href="#type-based-serialization-and-deserialization"
class="doc-anchor">§</a>Type-based Serialization and Deserialization

Serde provides a mechanism for low boilerplate serialization &
deserialization of values to and from MessagePack via the serialization
API.

To be able to serialize a piece of data, it must implement the
`serde::Serialize` trait. To be able to deserialize a piece of data, it
must implement the `serde::Deserialize` trait. Serde provides an
annotation to automatically generate the code for these traits:
`#[derive(Serialize, Deserialize)]`.

## <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Human {
    age: u32,
    name: String,
}

fn main() {
    let mut buf = Vec::new();
    let val = Human {
        age: 42,
        name: "John".into(),
    };

    val.serialize(&mut Serializer::new(&mut buf)).unwrap();
}
```

## <a href="#efficient-storage-of-u8-types" class="doc-anchor">§</a>Efficient storage of `&[u8]` types

MessagePack can efficiently store binary data. However, Serde’s standard
derived implementations *do not* use binary representations by default.
Serde prefers to represent types like `&[u8; N]` or `Vec<u8>` as arrays
of objects of arbitrary/unknown type, and not as slices of bytes. This
creates about a 50% overhead in storage size.

Wrap your data in [`serde_bytes`](https://lib.rs/crates/serde_bytes) to
store blobs quickly and efficiently. Alternatively, [configure an
override in `rmp_serde` to force use of byte
slices](https://docs.rs/rmp-serde/latest/rmp_serde/encode/struct.Serializer.html#method.with_bytes).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::decode::`<a href="decode/fn.from_read.html" class="fn"
title="fn rmp_serde::decode::from_read"><code>from_read</code></a>`;`

`pub use crate::decode::`<a href="decode/struct.Deserializer.html" class="struct"
title="struct rmp_serde::decode::Deserializer"><code>Deserializer</code></a>`;`

`pub use crate::encode::`<a href="encode/fn.to_vec.html" class="fn"
title="fn rmp_serde::encode::to_vec"><code>to_vec</code></a>`;`

`pub use crate::encode::`<a href="encode/fn.to_vec_named.html" class="fn"
title="fn rmp_serde::encode::to_vec_named"><code>to_vec_named</code></a>`;`

`pub use crate::encode::`<a href="encode/struct.Serializer.html" class="struct"
title="struct rmp_serde::encode::Serializer"><code>Serializer</code></a>`;`

`pub use crate::decode::`<a href="decode/fn.from_slice.html" class="fn"
title="fn rmp_serde::decode::from_slice"><code>from_slice</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="config/index.html" class="mod"
title="mod rmp_serde::config">config</a>  
Change MessagePack behavior with configuration wrappers.

<a href="decode/index.html" class="mod"
title="mod rmp_serde::decode">decode</a>  
Generic MessagePack deserialization.

<a href="encode/index.html" class="mod"
title="mod rmp_serde::encode">encode</a>  
Serialize a Rust data structure into MessagePack data.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.MSGPACK_EXT_STRUCT_NAME.html" class="constant"
title="constant rmp_serde::MSGPACK_EXT_STRUCT_NAME">MSGPACK_EXT_STRUCT_NAME</a>  
Hack used to serialize MessagePack Extension types.
