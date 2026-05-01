Crate ruma Copy item path

<span class="sub-heading"><a href="../src/ruma/lib.rs.html#1-149" class="src">Source</a>
</span>

Expand description

Types and traits for working with the [Matrix](https://matrix.org)
protocol.

This crate re-exports things from all of the other ruma crates so you
don’t have to manually keep all the versions in sync.

Which crates are re-exported can be configured through cargo features.

> ⚠ Some details might be missing because rustdoc has trouble with
> re-exports so you may need to refer to other crates’ documentations.

> 🛈 For internal consistency, Ruma uses American spelling for variable
> names. Names may differ in the serialized representation, as the
> Matrix specification has a mix of British and American English.

# <a href="#api-features" class="doc-anchor">§</a>API features

Depending on which parts of Matrix are relevant to you, activate the
following features:

-   `appservice-api` – Application Service API.
-   `client-api` – Client-Server API.
-   `federation-api` – Server-Server (Federation) API.
-   `identity-service-api` – Identity Service API.
-   `push-gateway-api` – Push Gateway API.

These features have `client`- and `server`-optimized variants that are
enabled respectively with the `-c` and `-s` suffixes. For example:

-   `client-api-c` – The Client-Server API optimized for the client
    side.
-   `client-api-s` – The Client-Server API optimized for the server
    side.

# <a href="#compatibility-features" class="doc-anchor">§</a>Compatibility features

By default, the ruma crates are only able to handle strictly
spec-compliant data and behaviour. However, due to the fact that Matrix
is federated, that it is used by various implementations that might have
different bugs, and that much of its data is immutable, they need to be
able to interoperate with data that might differ slightly from the
specification.

This is the role of the `compat-*` cargo features. They allow the crates
be more tolerant of external data and incoming requests for known and
reasonable deviations from the spec, usually for historical reasons.
They however do not permit the ruma crates to generate data that is not
spec-compliant.

Each cargo feature is documented briefly in the cargo manifest of the
crate, and more thoroughly where the feature applies.

# <a href="#convenience-features" class="doc-anchor">§</a>Convenience features

These features are only useful if you want to use a method that requires
it:

-   `rand` – Generate random identifiers.
-   `markdown` – Parse markdown to construct messages.
-   `html` – Parse HTML to sanitize it or navigate its tree.
    -   `html-matrix` – Enables the `matrix` feature of `ruma-html` to
        parse HTML elements data to typed data as suggested by the
        Matrix Specification.

# <a href="#unstable-features" class="doc-anchor">§</a>Unstable features

By using these features, you opt out of all semver guarantees Ruma
otherwise provides:

-   `unstable-mscXXXX`, where `XXXX` is the MSC number – Upcoming Matrix
    features that may be subject to change or removal.

# <a href="#common-features" class="doc-anchor">§</a>Common features

These submodules are usually activated by the API features when needed:

-   `api`
-   `events`
-   `signatures`

# <a href="#compile-time-cfg-settings" class="doc-anchor">§</a>Compile-time `cfg` settings

These settings are accepted at compile time to configure the generated
code. They can be set as `--cfg={key}={value}` using `RUSTFLAGS` or
`.cargo/config.toml` (under `[build]` -&gt; `rustflags = ["..."]`). They
can also be configured using an environment variable at compile time,
which has the benefit of not requiring to re-compile the whole
dependency chain when their value is changed.

-   `ruma_identifiers_storage` – Choose the inner representation of
    `Owned*` wrapper types for identifiers. By default they use
    [`Box`](https://doc.rust-lang.org/1.93.1/alloc/boxed/struct.Box.html "struct alloc::boxed::Box"),
    setting the value to `Arc` makes them use
    [`Arc`](https://doc.rust-lang.org/1.93.1/alloc/sync/struct.Arc.html "struct alloc::sync::Arc").
    This can also be configured by setting the
    `RUMA_IDENTIFIERS_STORAGE` environment variable.
-   `ruma_unstable_exhaustive_types` – Most types in Ruma are marked as
    non-exhaustive to avoid breaking changes when new fields are added
    in the specification. This setting compiles all types as exhaustive.
    By enabling this feature you opt out of all semver guarantees Ruma
    otherwise provides. This can also be configured by setting the
    `RUMA_UNSTABLE_EXHAUSTIVE_TYPES` environment variable.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use assign::`<a href="../assign/macro.assign.html" class="macro"
title="macro assign::assign"><code>assign</code></a>`;`

`pub use js_int::`<a href="../js_int/int/struct.Int.html" class="struct"
title="struct js_int::int::Int"><code>Int</code></a>`;`

`pub use js_int::`<a href="../js_int/uint/struct.UInt.html" class="struct"
title="struct js_int::uint::UInt"><code>UInt</code></a>`;`

`pub use js_int::`<a href="../js_int/macro.int.html" class="macro"
title="macro js_int::int"><code>int</code></a>`;`

`pub use js_int::`<a href="../js_int/macro.uint.html" class="macro"
title="macro js_int::uint"><code>uint</code></a>`;`

`pub use js_option::`<a href="../js_option/enum.JsOption.html" class="enum"
title="enum js_option::JsOption"><code>JsOption</code></a>`;`

`pub use `<a href="../web_time/index.html" class="mod"
title="mod web_time"><code>web_time</code></a>` as time;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="api/index.html" class="mod" title="mod ruma::api">api</a>  
(De)serializable types for various [Matrix
APIs](https://spec.matrix.org/latest/#matrix-apis) requests and
responses and abstractions for them.

<a href="authentication/index.html" class="mod"
title="mod ruma::authentication">authentication</a>  
Common types for authentication.

<a href="canonical_json/index.html" class="mod"
title="mod ruma::canonical_json">canonical_json</a>  
Canonical JSON types and related functions.

<a href="directory/index.html" class="mod"
title="mod ruma::directory">directory</a>  
Common types for room directory endpoints.

<a href="encryption/index.html" class="mod"
title="mod ruma::encryption">encryption</a>  
Common types for
[encryption](https://spec.matrix.org/latest/client-server-api/#end-to-end-encryption)
related tasks.

<a href="events/index.html" class="mod"
title="mod ruma::events">events</a>  
(De)serializable types for the events in the
[Matrix](https://matrix.org) specification. These types are used by
other Ruma crates.

<a href="http_headers/index.html" class="mod"
title="mod ruma::http_headers">http_headers</a>  
Helpers for HTTP headers.

<a href="matrix_uri/index.html" class="mod"
title="mod ruma::matrix_uri">matrix_uri</a>  
Matrix URIs.

<a href="media/index.html" class="mod" title="mod ruma::media">media</a>  
Common types and functions for the [content
repository](https://spec.matrix.org/latest/client-server-api/#content-repository).

<a href="power_levels/index.html" class="mod"
title="mod ruma::power_levels">power_levels</a>  
Common types for the [`m.room.power_levels`
event](https://spec.matrix.org/latest/client-server-api/#mroompower_levels).

<a href="presence/index.html" class="mod"
title="mod ruma::presence">presence</a>  
Common types for the [presence
module](https://spec.matrix.org/latest/client-server-api/#presence).

<a href="push/index.html" class="mod" title="mod ruma::push">push</a>  
Common types for the [push notifications
module](https://spec.matrix.org/latest/client-server-api/#push-notifications).

<a href="room/index.html" class="mod" title="mod ruma::room">room</a>  
Common types for rooms.

<a href="room_version_rules/index.html" class="mod"
title="mod ruma::room_version_rules">room_version_rules</a>  
Types for the rules applied to the different [room
versions](https://spec.matrix.org/latest/rooms/).

<a href="serde/index.html" class="mod" title="mod ruma::serde">serde</a>  
(De)serialization helpers for other Ruma crates.

<a href="third_party_invite/index.html" class="mod"
title="mod ruma::third_party_invite">third_party_invite</a>  
Common types for [third-party
invites](https://spec.matrix.org/latest/client-server-api/#third-party-invites).

<a href="thirdparty/index.html" class="mod"
title="mod ruma::thirdparty">thirdparty</a>  
Common types for the [third party networks
module](https://spec.matrix.org/latest/client-server-api/#third-party-networks).

<a href="to_device/index.html" class="mod"
title="mod ruma::to_device">to_device</a>  
Common types for the Send-To-Device Messaging

<a href="user_id/index.html" class="mod"
title="mod ruma::user_id">user_id</a>  
Matrix user identifiers.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.base64_public_key.html" class="macro"
title="macro ruma::base64_public_key">base64_public_key</a>  
Compile-time checked
[`Base64PublicKey`](struct.Base64PublicKey.html "struct ruma::Base64PublicKey")
construction.

<a href="macro.device_id.html" class="macro"
title="macro ruma::device_id">device_id</a>  
Shorthand for `<&DeviceId>::from`.

<a href="macro.event_id.html" class="macro"
title="macro ruma::event_id">event_id</a>  
Compile-time checked
[`EventId`](struct.EventId.html "struct ruma::EventId") construction.

<a href="macro.mxc_uri.html" class="macro"
title="macro ruma::mxc_uri">mxc_uri</a>  
Compile-time checked
[`MxcUri`](struct.MxcUri.html "struct ruma::MxcUri") construction.

<a href="macro.owned_base64_public_key.html" class="macro"
title="macro ruma::owned_base64_public_key">owned_base64_public_key</a>  
Compile-time checked
[`OwnedBase64PublicKey`](struct.OwnedBase64PublicKey.html "struct ruma::OwnedBase64PublicKey")
construction.

<a href="macro.owned_device_id.html" class="macro"
title="macro ruma::owned_device_id">owned_device_id</a>  
Shorthand for `OwnedDeviceId::from`.

<a href="macro.owned_event_id.html" class="macro"
title="macro ruma::owned_event_id">owned_event_id</a>  
Compile-time checked
[`OwnedEventId`](struct.OwnedEventId.html "struct ruma::OwnedEventId")
construction.

<a href="macro.owned_mxc_uri.html" class="macro"
title="macro ruma::owned_mxc_uri">owned_mxc_uri</a>  
Compile-time checked
[`OwnedMxcUri`](struct.OwnedMxcUri.html "struct ruma::OwnedMxcUri")
construction.

<a href="macro.owned_room_alias_id.html" class="macro"
title="macro ruma::owned_room_alias_id">owned_room_alias_id</a>  
Compile-time checked
[`OwnedRoomAliasId`](struct.OwnedRoomAliasId.html "struct ruma::OwnedRoomAliasId")
construction.

<a href="macro.owned_room_id.html" class="macro"
title="macro ruma::owned_room_id">owned_room_id</a>  
Compile-time checked
[`OwnedRoomId`](struct.OwnedRoomId.html "struct ruma::OwnedRoomId")
construction.

<a href="macro.owned_server_name.html" class="macro"
title="macro ruma::owned_server_name">owned_server_name</a>  
Compile-time checked
[`OwnedServerName`](struct.OwnedServerName.html "struct ruma::OwnedServerName")
construction.

<a href="macro.owned_server_signing_key_version.html" class="macro"
title="macro ruma::owned_server_signing_key_version">owned_server_signing_key_version</a>  
Compile-time checked
[`OwnedServerSigningKeyVersion`](struct.OwnedServerSigningKeyVersion.html "struct ruma::OwnedServerSigningKeyVersion")
construction.

<a href="macro.owned_session_id.html" class="macro"
title="macro ruma::owned_session_id">owned_session_id</a>  
Compile-time checked
[`OwnedSessionId`](struct.OwnedSessionId.html "struct ruma::OwnedSessionId")
construction.

<a href="macro.owned_user_id.html" class="macro"
title="macro ruma::owned_user_id">owned_user_id</a>  
Compile-time checked
[`OwnedUserId`](struct.OwnedUserId.html "struct ruma::OwnedUserId")
construction.

<a href="macro.room_alias_id.html" class="macro"
title="macro ruma::room_alias_id">room_alias_id</a>  
Compile-time checked
[`RoomAliasId`](struct.RoomAliasId.html "struct ruma::RoomAliasId")
construction.

<a href="macro.room_id.html" class="macro"
title="macro ruma::room_id">room_id</a>  
Compile-time checked
[`RoomId`](struct.RoomId.html "struct ruma::RoomId") construction.

<a href="macro.room_version_id.html" class="macro"
title="macro ruma::room_version_id">room_version_id</a>  
Compile-time checked
[`RoomVersionId`](enum.RoomVersionId.html "enum ruma::RoomVersionId")
construction.

<a href="macro.server_name.html" class="macro"
title="macro ruma::server_name">server_name</a>  
Compile-time checked
[`ServerName`](struct.ServerName.html "struct ruma::ServerName")
construction.

<a href="macro.server_signing_key_version.html" class="macro"
title="macro ruma::server_signing_key_version">server_signing_key_version</a>  
Compile-time checked
[`ServerSigningKeyVersion`](struct.ServerSigningKeyVersion.html "struct ruma::ServerSigningKeyVersion")
construction.

<a href="macro.session_id.html" class="macro"
title="macro ruma::session_id">session_id</a>  
Compile-time checked
[`SessionId`](struct.SessionId.html "struct ruma::SessionId")
construction.

<a href="macro.user_id.html" class="macro"
title="macro ruma::user_id">user_id</a>  
Compile-time checked
[`UserId`](struct.UserId.html "struct ruma::UserId") construction.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AnyKeyName.html" class="struct"
title="struct ruma::AnyKeyName">AnyKeyName</a>  
An opaque identifier type to use with
[`KeyId`](struct.KeyId.html "struct ruma::KeyId").

<a href="struct.Base64PublicKey.html" class="struct"
title="struct ruma::Base64PublicKey">Base64PublicKey</a>  
A public key encoded using unpadded base64, used as an identifier for
[cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
keys.

<a href="struct.Base64PublicKeyOrDeviceId.html" class="struct"
title="struct ruma::Base64PublicKeyOrDeviceId">Base64PublicKeyOrDeviceId</a>  
A Matrix ID that can be either a
[`DeviceId`](struct.DeviceId.html "struct ruma::DeviceId") or a
[`Base64PublicKey`](struct.Base64PublicKey.html "struct ruma::Base64PublicKey").

<a href="struct.ClientSecret.html" class="struct"
title="struct ruma::ClientSecret">ClientSecret</a>  
A client secret.

<a href="struct.DeviceId.html" class="struct"
title="struct ruma::DeviceId">DeviceId</a>  
A Matrix device ID.

<a href="struct.EventId.html" class="struct"
title="struct ruma::EventId">EventId</a>  
A Matrix [event
ID](https://spec.matrix.org/latest/appendices/#event-ids).

<a href="struct.KeyId.html" class="struct"
title="struct ruma::KeyId">KeyId</a>  
A key algorithm and key name delimited by a colon.

<a href="struct.MatrixToUri.html" class="struct"
title="struct ruma::MatrixToUri">MatrixToUri</a>  
The [`matrix.to`
URI](https://spec.matrix.org/latest/appendices/#matrixto-navigation)
representation of a user, room or event.

<a href="struct.MatrixUri.html" class="struct"
title="struct ruma::MatrixUri">MatrixUri</a>  
The [`matrix:`
URI](https://spec.matrix.org/latest/appendices/#matrix-uri-scheme)
representation of a user, room or event.

<a href="struct.MilliSecondsSinceUnixEpoch.html" class="struct"
title="struct ruma::MilliSecondsSinceUnixEpoch">MilliSecondsSinceUnixEpoch</a>  
A timestamp represented as the number of milliseconds since the unix
epoch.

<a href="struct.MxcUri.html" class="struct"
title="struct ruma::MxcUri">MxcUri</a>  
A URI that should be a Matrix-spec compliant [MXC
URI](https://spec.matrix.org/latest/client-server-api/#matrix-content-mxc-uris).

<a href="struct.OneTimeKeyName.html" class="struct"
title="struct ruma::OneTimeKeyName">OneTimeKeyName</a>  
The name of a [one-time or fallback
key](https://spec.matrix.org/latest/client-server-api/#one-time-and-fallback-keys).

<a href="struct.OwnedBase64PublicKey.html" class="struct"
title="struct ruma::OwnedBase64PublicKey">OwnedBase64PublicKey</a>  
Owned variant of
[`Base64PublicKey`](struct.Base64PublicKey.html "struct ruma::Base64PublicKey")

<a href="struct.OwnedBase64PublicKeyOrDeviceId.html" class="struct"
title="struct ruma::OwnedBase64PublicKeyOrDeviceId">OwnedBase64PublicKeyOrDeviceId</a>  
Owned variant of
[`Base64PublicKeyOrDeviceId`](struct.Base64PublicKeyOrDeviceId.html "struct ruma::Base64PublicKeyOrDeviceId")

<a href="struct.OwnedClientSecret.html" class="struct"
title="struct ruma::OwnedClientSecret">OwnedClientSecret</a>  
Owned variant of
[`ClientSecret`](struct.ClientSecret.html "struct ruma::ClientSecret")

<a href="struct.OwnedDeviceId.html" class="struct"
title="struct ruma::OwnedDeviceId">OwnedDeviceId</a>  
Owned variant of
[`DeviceId`](struct.DeviceId.html "struct ruma::DeviceId")

<a href="struct.OwnedEventId.html" class="struct"
title="struct ruma::OwnedEventId">OwnedEventId</a>  
Owned variant of [`EventId`](struct.EventId.html "struct ruma::EventId")

<a href="struct.OwnedKeyId.html" class="struct"
title="struct ruma::OwnedKeyId">OwnedKeyId</a>  
Owned variant of [`KeyId`](struct.KeyId.html "struct ruma::KeyId")

<a href="struct.OwnedMxcUri.html" class="struct"
title="struct ruma::OwnedMxcUri">OwnedMxcUri</a>  
Owned variant of [`MxcUri`](struct.MxcUri.html "struct ruma::MxcUri")

<a href="struct.OwnedOneTimeKeyName.html" class="struct"
title="struct ruma::OwnedOneTimeKeyName">OwnedOneTimeKeyName</a>  
Owned variant of
[`OneTimeKeyName`](struct.OneTimeKeyName.html "struct ruma::OneTimeKeyName")

<a href="struct.OwnedRoomAliasId.html" class="struct"
title="struct ruma::OwnedRoomAliasId">OwnedRoomAliasId</a>  
Owned variant of
[`RoomAliasId`](struct.RoomAliasId.html "struct ruma::RoomAliasId")

<a href="struct.OwnedRoomId.html" class="struct"
title="struct ruma::OwnedRoomId">OwnedRoomId</a>  
Owned variant of [`RoomId`](struct.RoomId.html "struct ruma::RoomId")

<a href="struct.OwnedRoomOrAliasId.html" class="struct"
title="struct ruma::OwnedRoomOrAliasId">OwnedRoomOrAliasId</a>  
Owned variant of
[`RoomOrAliasId`](struct.RoomOrAliasId.html "struct ruma::RoomOrAliasId")

<a href="struct.OwnedServerName.html" class="struct"
title="struct ruma::OwnedServerName">OwnedServerName</a>  
Owned variant of
[`ServerName`](struct.ServerName.html "struct ruma::ServerName")

<a href="struct.OwnedServerSigningKeyVersion.html" class="struct"
title="struct ruma::OwnedServerSigningKeyVersion">OwnedServerSigningKeyVersion</a>  
Owned variant of
[`ServerSigningKeyVersion`](struct.ServerSigningKeyVersion.html "struct ruma::ServerSigningKeyVersion")

<a href="struct.OwnedSessionId.html" class="struct"
title="struct ruma::OwnedSessionId">OwnedSessionId</a>  
Owned variant of
[`SessionId`](struct.SessionId.html "struct ruma::SessionId")

<a href="struct.OwnedSpaceChildOrder.html" class="struct"
title="struct ruma::OwnedSpaceChildOrder">OwnedSpaceChildOrder</a>  
Owned variant of
[`SpaceChildOrder`](struct.SpaceChildOrder.html "struct ruma::SpaceChildOrder")

<a href="struct.OwnedTransactionId.html" class="struct"
title="struct ruma::OwnedTransactionId">OwnedTransactionId</a>  
Owned variant of
[`TransactionId`](struct.TransactionId.html "struct ruma::TransactionId")

<a href="struct.OwnedUserId.html" class="struct"
title="struct ruma::OwnedUserId">OwnedUserId</a>  
Owned variant of [`UserId`](struct.UserId.html "struct ruma::UserId")

<a href="struct.OwnedVoipId.html" class="struct"
title="struct ruma::OwnedVoipId">OwnedVoipId</a>  
Owned variant of [`VoipId`](struct.VoipId.html "struct ruma::VoipId")

<a href="struct.RoomAliasId.html" class="struct"
title="struct ruma::RoomAliasId">RoomAliasId</a>  
A Matrix [room alias
ID](https://spec.matrix.org/latest/appendices/#room-aliases).

<a href="struct.RoomId.html" class="struct"
title="struct ruma::RoomId">RoomId</a>  
A Matrix [room ID](https://spec.matrix.org/latest/appendices/#room-ids).

<a href="struct.RoomOrAliasId.html" class="struct"
title="struct ruma::RoomOrAliasId">RoomOrAliasId</a>  
A Matrix [room ID](https://spec.matrix.org/latest/appendices/#room-ids)
or a Matrix [room alias
ID](https://spec.matrix.org/latest/appendices/#room-aliases).

<a href="struct.SecondsSinceUnixEpoch.html" class="struct"
title="struct ruma::SecondsSinceUnixEpoch">SecondsSinceUnixEpoch</a>  
A timestamp represented as the number of seconds since the unix epoch.

<a href="struct.ServerName.html" class="struct"
title="struct ruma::ServerName">ServerName</a>  
A Matrix-spec compliant [server
name](https://spec.matrix.org/latest/appendices/#server-name).

<a href="struct.ServerSigningKeyVersion.html" class="struct"
title="struct ruma::ServerSigningKeyVersion">ServerSigningKeyVersion</a>  
The version of a [homeserver signing
key](https://spec.matrix.org/latest/server-server-api/#retrieving-server-keys).

<a href="struct.SessionId.html" class="struct"
title="struct ruma::SessionId">SessionId</a>  
A session ID.

<a href="struct.Signatures.html" class="struct"
title="struct ruma::Signatures">Signatures</a>  
Map of all signatures, grouped by entity.

<a href="struct.SpaceChildOrder.html" class="struct"
title="struct ruma::SpaceChildOrder">SpaceChildOrder</a>  
The order of an
[`m.space.child`](https://spec.matrix.org/latest/client-server-api/#mspacechild)
event.

<a href="struct.TransactionId.html" class="struct"
title="struct ruma::TransactionId">TransactionId</a>  
A Matrix transaction ID.

<a href="struct.UserId.html" class="struct"
title="struct ruma::UserId">UserId</a>  
A Matrix [user
ID](https://spec.matrix.org/latest/appendices/#user-identifiers).

<a href="struct.VoipId.html" class="struct"
title="struct ruma::VoipId">VoipId</a>  
A VoIP identifier.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.CanonicalJsonError.html" class="enum"
title="enum ruma::CanonicalJsonError">CanonicalJsonError</a>  
The set of possible errors when serializing to canonical JSON.

<a href="enum.CanonicalJsonValue.html" class="enum"
title="enum ruma::CanonicalJsonValue">CanonicalJsonValue</a>  
Represents a canonical JSON value as per the Matrix specification.

<a href="enum.DeviceKeyAlgorithm.html" class="enum"
title="enum ruma::DeviceKeyAlgorithm">DeviceKeyAlgorithm</a>  
The algorithms for the [device
keys](https://spec.matrix.org/latest/client-server-api/#device-keys)
defined in the Matrix spec.

<a href="enum.EventEncryptionAlgorithm.html" class="enum"
title="enum ruma::EventEncryptionAlgorithm">EventEncryptionAlgorithm</a>  
An encryption algorithm to be used to encrypt messages sent to a room.

<a href="enum.IdParseError.html" class="enum"
title="enum ruma::IdParseError">IdParseError</a>  
An error encountered when trying to parse an invalid ID string.

<a href="enum.KeyDerivationAlgorithm.html" class="enum"
title="enum ruma::KeyDerivationAlgorithm">KeyDerivationAlgorithm</a>  
A key algorithm to be used to generate a key from a passphrase.

<a href="enum.MatrixIdError.html" class="enum"
title="enum ruma::MatrixIdError">MatrixIdError</a>  
An error occurred while validating a `MatrixId`.

<a href="enum.MatrixToError.html" class="enum"
title="enum ruma::MatrixToError">MatrixToError</a>  
An error occurred while validating a `matrix.to` URI.

<a href="enum.MatrixUriError.html" class="enum"
title="enum ruma::MatrixUriError">MatrixUriError</a>  
An error occurred while validating a `MatrixURI`.

<a href="enum.MxcUriError.html" class="enum"
title="enum ruma::MxcUriError">MxcUriError</a>  
An error occurred while validating an MXC URI.

<a href="enum.OneTimeKeyAlgorithm.html" class="enum"
title="enum ruma::OneTimeKeyAlgorithm">OneTimeKeyAlgorithm</a>  
The algorithms for [one-time and fallback
keys](https://spec.matrix.org/latest/client-server-api/#one-time-and-fallback-keys)
defined in the Matrix spec.

<a href="enum.RoomVersionId.html" class="enum"
title="enum ruma::RoomVersionId">RoomVersionId</a>  
A Matrix [room version](https://spec.matrix.org/latest/rooms/) ID.

<a href="enum.SigningKeyAlgorithm.html" class="enum"
title="enum ruma::SigningKeyAlgorithm">SigningKeyAlgorithm</a>  
The signing key algorithms defined in the Matrix spec.

<a href="enum.VoipVersionId.html" class="enum"
title="enum ruma::VoipVersionId">VoipVersionId</a>  
A Matrix VoIP version ID.

<a href="enum.VoipVersionIdError.html" class="enum"
title="enum ruma::VoipVersionIdError">VoipVersionIdError</a>  
An error occurred while validating a `VoipVersionId`.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ID_MAX_BYTES.html" class="constant"
title="constant ruma::ID_MAX_BYTES">ID_MAX_BYTES</a>  
The maximum allowed length of Matrix identifiers, in bytes.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.KeyAlgorithm.html" class="trait"
title="trait ruma::KeyAlgorithm">KeyAlgorithm</a>  
The algorithm of a key.

<a href="trait.KeyName.html" class="trait"
title="trait ruma::KeyName">KeyName</a>  
Helper trait to validate the name of a key.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.CanonicalJsonObject.html" class="type"
title="type ruma::CanonicalJsonObject">CanonicalJsonObject</a>  
The inner type of `CanonicalJsonValue::Object`.

<a href="type.CrossSigningKeyId.html" class="type"
title="type ruma::CrossSigningKeyId">CrossSigningKeyId</a>  
Algorithm + key name for
[cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
keys.

<a href="type.CrossSigningOrDeviceSignatures.html" class="type"
title="type ruma::CrossSigningOrDeviceSignatures">CrossSigningOrDeviceSignatures</a>  
Map of cross-signing or device signatures, grouped by user.

<a href="type.CrossSigningOrDeviceSigningKeyId.html" class="type"
title="type ruma::CrossSigningOrDeviceSigningKeyId">CrossSigningOrDeviceSigningKeyId</a>  
Algorithm + key name for
[cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
or [device
signing](https://spec.matrix.org/latest/client-server-api/#device-keys)
keys.

<a href="type.DeviceKeyId.html" class="type"
title="type ruma::DeviceKeyId">DeviceKeyId</a>  
Algorithm + key name for [device
keys](https://spec.matrix.org/latest/client-server-api/#device-keys).

<a href="type.DeviceSignatures.html" class="type"
title="type ruma::DeviceSignatures">DeviceSignatures</a>  
Map of device signatures, grouped by user.

<a href="type.DeviceSigningKeyId.html" class="type"
title="type ruma::DeviceSigningKeyId">DeviceSigningKeyId</a>  
Algorithm + key name for [device signing
keys](https://spec.matrix.org/latest/client-server-api/#device-keys).

<a href="type.EntitySignatures.html" class="type"
title="type ruma::EntitySignatures">EntitySignatures</a>  
Map of key identifier to signature values.

<a href="type.OneTimeKeyId.html" class="type"
title="type ruma::OneTimeKeyId">OneTimeKeyId</a>  
Algorithm + key name for [one-time and fallback
keys](https://spec.matrix.org/latest/client-server-api/#one-time-and-fallback-keys).

<a href="type.OwnedCrossSigningKeyId.html" class="type"
title="type ruma::OwnedCrossSigningKeyId">OwnedCrossSigningKeyId</a>  
Algorithm + key name for
[cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
keys.

<a href="type.OwnedCrossSigningOrDeviceSigningKeyId.html" class="type"
title="type ruma::OwnedCrossSigningOrDeviceSigningKeyId">OwnedCrossSigningOrDeviceSigningKeyId</a>  
Algorithm + key name for
[cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
or [device
signing](https://spec.matrix.org/latest/client-server-api/#device-keys)
keys.

<a href="type.OwnedDeviceKeyId.html" class="type"
title="type ruma::OwnedDeviceKeyId">OwnedDeviceKeyId</a>  
Algorithm + key name for [device
keys](https://spec.matrix.org/latest/client-server-api/#device-keys).

<a href="type.OwnedDeviceSigningKeyId.html" class="type"
title="type ruma::OwnedDeviceSigningKeyId">OwnedDeviceSigningKeyId</a>  
Algorithm + key name for \[device signing\] keys.

<a href="type.OwnedOneTimeKeyId.html" class="type"
title="type ruma::OwnedOneTimeKeyId">OwnedOneTimeKeyId</a>  
Algorithm + key name for [one-time and fallback
keys](https://spec.matrix.org/latest/client-server-api/#one-time-and-fallback-keys).

<a href="type.OwnedServerSigningKeyId.html" class="type"
title="type ruma::OwnedServerSigningKeyId">OwnedServerSigningKeyId</a>  
Algorithm + key name for homeserver signing keys.

<a href="type.OwnedSigningKeyId.html" class="type"
title="type ruma::OwnedSigningKeyId">OwnedSigningKeyId</a>  
Algorithm + key name for signing keys.

<a href="type.ServerSignatures.html" class="type"
title="type ruma::ServerSignatures">ServerSignatures</a>  
Map of server signatures, grouped by server.

<a href="type.ServerSigningKeyId.html" class="type"
title="type ruma::ServerSigningKeyId">ServerSigningKeyId</a>  
Algorithm + key name for homeserver signing keys.

<a href="type.SigningKeyId.html" class="type"
title="type ruma::SigningKeyId">SigningKeyId</a>  
Algorithm + key name for signing keys.
