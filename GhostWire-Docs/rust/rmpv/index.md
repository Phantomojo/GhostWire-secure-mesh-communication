Crate rmpv Copy item path

<span class="sub-heading"><a href="../src/rmpv/lib.rs.html#1-1586" class="src">Source</a>
</span>

Expand description

Contains Value and `ValueRef` structs and its conversion traits.

# Modules<a href="#modules" class="anchor">§</a>

<a href="decode/index.html" class="mod"
title="mod rmpv::decode">decode</a>

<a href="encode/index.html" class="mod"
title="mod rmpv::encode">encode</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Integer.html" class="struct"
title="struct rmpv::Integer">Integer</a>  
Represents a MessagePack integer, whether signed or unsigned.

<a href="struct.Utf8String.html" class="struct"
title="struct rmpv::Utf8String">Utf8String</a>  
Represents an UTF-8 MessagePack string type.

<a href="struct.Utf8StringRef.html" class="struct"
title="struct rmpv::Utf8StringRef">Utf8StringRef</a>  
A non-owning evil twin of `Utf8String`. Does exactly the same thing
except ownership.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Value.html" class="enum"
title="enum rmpv::Value">Value</a>

Represents any valid MessagePack value.

<a href="enum.ValueRef.html" class="enum"
title="enum rmpv::ValueRef">ValueRef</a>

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.MSGPACK_EXT_STRUCT_NAME.html" class="constant"
title="constant rmpv::MSGPACK_EXT_STRUCT_NAME">MSGPACK_EXT_STRUCT_NAME</a>  
Name of Serde newtype struct to Represent Msgpack’s Ext Msgpack Ext:
Ext(tag, binary) Serde data model: \_ExtStruct((tag, binary)) Example
Serde impl for custom type:
