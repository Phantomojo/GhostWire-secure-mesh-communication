Crate ruma\_macros Copy item path

<span class="sub-heading"><a href="../src/ruma_macros/lib.rs.html#1-810" class="src">Source</a>
</span>

Expand description

Procedural macros used by ruma crates.

See the documentation for the individual macros for usage details.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.base64_public_key.html" class="macro"
title="macro ruma_macros::base64_public_key">base64_public_key</a>  
Compile-time checked `Base64PublicKey` construction.

<a href="macro.event_enum.html" class="macro"
title="macro ruma_macros::event_enum">event_enum</a>  
Generates enums to represent the various Matrix event types.

<a href="macro.event_id.html" class="macro"
title="macro ruma_macros::event_id">event_id</a>  
Compile-time checked `EventId` construction.

<a href="macro.mxc_uri.html" class="macro"
title="macro ruma_macros::mxc_uri">mxc_uri</a>  
Compile-time checked `MxcUri` construction.

<a href="macro.room_alias_id.html" class="macro"
title="macro ruma_macros::room_alias_id">room_alias_id</a>  
Compile-time checked `RoomAliasId` construction.

<a href="macro.room_id.html" class="macro"
title="macro ruma_macros::room_id">room_id</a>  
Compile-time checked `RoomId` construction.

<a href="macro.room_version_id.html" class="macro"
title="macro ruma_macros::room_version_id">room_version_id</a>  
Compile-time checked `RoomVersionId` construction.

<a href="macro.server_name.html" class="macro"
title="macro ruma_macros::server_name">server_name</a>  
Compile-time checked `ServerName` construction.

<a href="macro.server_signing_key_version.html" class="macro"
title="macro ruma_macros::server_signing_key_version">server_signing_key_version</a>  
Compile-time checked `ServerSigningKeyVersion` construction.

<a href="macro.user_id.html" class="macro"
title="macro ruma_macros::user_id">user_id</a>  
Compile-time checked `UserId` construction.

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.request.html" class="attr"
title="attr ruma_macros::request">request</a>  
> ⚠ If this is the only documentation you see, please navigate to the
> docs for `ruma_common::api::request`, where actual documentation can
> be found.

<a href="attr.response.html" class="attr"
title="attr ruma_macros::response">response</a>  
> ⚠ If this is the only documentation you see, please navigate to the
> docs for `ruma_common::api::response`, where actual documentation can
> be found.

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.AsRefStr.html" class="derive"
title="derive ruma_macros::AsRefStr">AsRefStr</a>  
Derive the `AsRef<str>` trait for an enum.

<a href="derive.AsStrAsRefStr.html" class="derive"
title="derive ruma_macros::AsStrAsRefStr">AsStrAsRefStr</a>  
Derive the `as_str()` method using the `AsRef<str>` implementation of
the type.

<a href="derive.DebugAsRefStr.html" class="derive"
title="derive ruma_macros::DebugAsRefStr">DebugAsRefStr</a>  
Derive the `fmt::Debug` trait using the `AsRef<str>` implementation of
the type.

<a href="derive.DeserializeFromCowStr.html" class="derive"
title="derive ruma_macros::DeserializeFromCowStr">DeserializeFromCowStr</a>  
Derive the `Deserialize` trait using the `From<Cow<str>>` implementation
of the type.

<a href="derive.DisplayAsRefStr.html" class="derive"
title="derive ruma_macros::DisplayAsRefStr">DisplayAsRefStr</a>  
Derive the `fmt::Display` trait using the `AsRef<str>` implementation of
the type.

<a href="derive.EqAsRefStr.html" class="derive"
title="derive ruma_macros::EqAsRefStr">EqAsRefStr</a>  
Derive the `PartialEq` and `Eq` traits using the `AsRef<str>`
implementation of the type.

<a href="derive.Event.html" class="derive"
title="derive ruma_macros::Event">Event</a>  
Generates trait implementations for Matrix event types.

<a href="derive.EventContent.html" class="derive"
title="derive ruma_macros::EventContent">EventContent</a>  
Generates traits implementations and types for an event content.

<a href="derive.EventEnumFromEvent.html" class="derive"
title="derive ruma_macros::EventEnumFromEvent">EventEnumFromEvent</a>  
Generates `From` implementations for an enum for all its variants.

<a href="derive.FromString.html" class="derive"
title="derive ruma_macros::FromString">FromString</a>  
Derive the `From<T: AsRef<str> + Into<Box<str>>>` trait for an enum.

<a href="derive.IdDst.html" class="derive"
title="derive ruma_macros::IdDst">IdDst</a>  
Generate methods and trait impl’s for DST identifier type.

<a href="derive.OrdAsRefStr.html" class="derive"
title="derive ruma_macros::OrdAsRefStr">OrdAsRefStr</a>  
Derive the `Ord` and `PartialOrd` traits using the `AsRef<str>`
implementation of the type.

<a href="derive.Request.html" class="derive"
title="derive ruma_macros::Request">Request</a>  
Internal helper that the request macro delegates most of its work to.

<a href="derive.Response.html" class="derive"
title="derive ruma_macros::Response">Response</a>  
Internal helper that the response macro delegates most of its work to.

<a href="derive.SerializeAsRefStr.html" class="derive"
title="derive ruma_macros::SerializeAsRefStr">SerializeAsRefStr</a>  
Derive the `Serialize` trait using the `AsRef<str>` implementation of
the type.

<a href="derive.StringEnum.html" class="derive"
title="derive ruma_macros::StringEnum">StringEnum</a>  
Shorthand for the derives
[`AsRefStr`](derive.AsRefStr.html "derive ruma_macros::AsRefStr"),
[`FromString`](derive.FromString.html "derive ruma_macros::FromString"),
[`DisplayAsRefStr`](derive.DisplayAsRefStr.html "derive ruma_macros::DisplayAsRefStr"),
[`DebugAsRefStr`](derive.DebugAsRefStr.html "derive ruma_macros::DebugAsRefStr"),
[`SerializeAsRefStr`](derive.SerializeAsRefStr.html "derive ruma_macros::SerializeAsRefStr"),
[`DeserializeFromCowStr`](derive.DeserializeFromCowStr.html "derive ruma_macros::DeserializeFromCowStr"),
[`EqAsRefStr`](derive.EqAsRefStr.html "derive ruma_macros::EqAsRefStr")
and
[`OrdAsRefStr`](derive.OrdAsRefStr.html "derive ruma_macros::OrdAsRefStr").
