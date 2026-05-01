Crate serde\_core Copy item path

<span class="sub-heading"><a href="../src/serde_core/lib.rs.html#1-121" class="src">Source</a>
</span>

Expand description

Serde is a framework for ***ser***ializing and ***de***serializing Rust
data structures efficiently and generically.

The `serde_core` crate contains Serde’s trait definitions with **no
support for \#\[derive()\]**.

In crates that derive an implementation of `Serialize` or `Deserialize`,
you must depend on the [`serde`](https://crates.io/crates/serde) crate,
not `serde_core`.

In crates that handwrite implementations of Serde traits, or only use
them as trait bounds, depending on `serde_core` is permitted. But
`serde` re-exports all of these traits and can be used for this use case
too. If in doubt, disregard `serde_core` and always use `serde`.

Crates that depend on `serde_core` instead of `serde` are able to
compile in parallel with `serde_derive` even when `serde`’s “derive”
feature is turned on, as shown in the following build timings.

  

|  |
|:--:|
| When `serde_json` depends on `serde` |
| ![](https://github.com/user-attachments/assets/78dc179c-6ab1-4059-928c-1474b0d9d0bb) |

  

|  |
|:--:|
| When `serde_json` depends on `serde_core` |
| ![](https://github.com/user-attachments/assets/6b6cff5e-3e45-4ac7-9db1-d99ee8b9f5f7) |

# Modules<a href="#modules" class="anchor">§</a>

<a href="de/index.html" class="mod" title="mod serde_core::de">de</a>  
Generic data structure deserialization framework.

<a href="ser/index.html" class="mod" title="mod serde_core::ser">ser</a>  
Generic data structure serialization framework.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.forward_to_deserialize_any.html" class="macro"
title="macro serde_core::forward_to_deserialize_any">forward_to_deserialize_any</a>  
Helper macro when implementing the `Deserializer` part of a new data
format for Serde.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Deserialize.html" class="trait"
title="trait serde_core::Deserialize">Deserialize</a>  
A **data structure** that can be deserialized from any data format
supported by Serde.

<a href="trait.Deserializer.html" class="trait"
title="trait serde_core::Deserializer">Deserializer</a>  
A **data format** that can deserialize any data structure supported by
Serde.

<a href="trait.Serialize.html" class="trait"
title="trait serde_core::Serialize">Serialize</a>  
A **data structure** that can be serialized into any data format
supported by Serde.

<a href="trait.Serializer.html" class="trait"
title="trait serde_core::Serializer">Serializer</a>  
A **data format** that can serialize any data structure supported by
Serde.
