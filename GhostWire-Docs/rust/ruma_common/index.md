Crate ruma\_common Copy item path

<span class="sub-heading"><a href="../src/ruma_common/lib.rs.html#1-78" class="src">Source</a>
</span>

Expand description

Common types for the Ruma crates.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::canonical_json::`<a href="canonical_json/enum.CanonicalJsonError.html" class="enum"
title="enum ruma_common::canonical_json::CanonicalJsonError"><code>CanonicalJsonError</code></a>`;`

`pub use self::canonical_json::`<a href="canonical_json/type.CanonicalJsonObject.html" class="type"
title="type ruma_common::canonical_json::CanonicalJsonObject"><code>CanonicalJsonObject</code></a>`;`

`pub use self::canonical_json::`<a href="canonical_json/enum.CanonicalJsonValue.html" class="enum"
title="enum ruma_common::canonical_json::CanonicalJsonValue"><code>CanonicalJsonValue</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="api/index.html" class="mod"
title="mod ruma_common::api">api</a>  
Core types used to define the requests and responses for each endpoint
in the various [Matrix API
specifications](https://spec.matrix.org/latest/#matrix-apis).

<a href="authentication/index.html" class="mod"
title="mod ruma_common::authentication">authentication</a>  
Common types for authentication.

<a href="canonical_json/index.html" class="mod"
title="mod ruma_common::canonical_json">canonical_json</a>  
Canonical JSON types and related functions.

<a href="directory/index.html" class="mod"
title="mod ruma_common::directory">directory</a>  
Common types for room directory endpoints.

<a href="encryption/index.html" class="mod"
title="mod ruma_common::encryption">encryption</a>  
Common types for
[encryption](https://spec.matrix.org/latest/client-server-api/#end-to-end-encryption)
related tasks.

<a href="http_headers/index.html" class="mod"
title="mod ruma_common::http_headers">http_headers</a>  
Helpers for HTTP headers.

<a href="matrix_uri/index.html" class="mod"
title="mod ruma_common::matrix_uri">matrix_uri</a>  
Matrix URIs.

<a href="media/index.html" class="mod"
title="mod ruma_common::media">media</a>  
Common types and functions for the [content
repository](https://spec.matrix.org/latest/client-server-api/#content-repository).

<a href="power_levels/index.html" class="mod"
title="mod ruma_common::power_levels">power_levels</a>  
Common types for the [`m.room.power_levels`
event](https://spec.matrix.org/latest/client-server-api/#mroompower_levels).

<a href="presence/index.html" class="mod"
title="mod ruma_common::presence">presence</a>  
Common types for the [presence
module](https://spec.matrix.org/latest/client-server-api/#presence).

<a href="push/index.html" class="mod"
title="mod ruma_common::push">push</a>  
Common types for the [push notifications
module](https://spec.matrix.org/latest/client-server-api/#push-notifications).

<a href="room/index.html" class="mod"
title="mod ruma_common::room">room</a>  
Common types for rooms.

<a href="room_version_rules/index.html" class="mod"
title="mod ruma_common::room_version_rules">room_version_rules</a>  
Types for the rules applied to the different [room
versions](https://spec.matrix.org/latest/rooms/).

<a href="serde/index.html" class="mod"
title="mod ruma_common::serde">serde</a>  
(De)serialization helpers for other Ruma crates.

<a href="third_party_invite/index.html" class="mod"
title="mod ruma_common::third_party_invite">third_party_invite</a>  
Common types for [third-party
invites](https://spec.matrix.org/latest/client-server-api/#third-party-invites).

<a href="thirdparty/index.html" class="mod"
title="mod ruma_common::thirdparty">thirdparty</a>  
Common types for the [third party networks
module](https://spec.matrix.org/latest/client-server-api/#third-party-networks).

<a href="to_device/index.html" class="mod"
title="mod ruma_common::to_device">to_device</a>  
Common types for the Send-To-Device Messaging

<a href="user_id/index.html" class="mod"
title="mod ruma_common::user_id">user_id</a>  
Matrix user identifiers.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.base64_public_key.html" class="macro"
title="macro ruma_common::base64_public_key">base64_public_key</a>  
Compile-time checked
[`Base64PublicKey`](struct.Base64PublicKey.html "struct ruma_common::Base64PublicKey")
construction.

<a href="macro.device_id.html" class="macro"
title="macro ruma_common::device_id">device_id</a>  
Shorthand for `<&DeviceId>::from`.

<a href="macro.event_id.html" class="macro"
title="macro ruma_common::event_id">event_id</a>  
Compile-time checked
[`EventId`](struct.EventId.html "struct ruma_common::EventId")
construction.

<a href="macro.mxc_uri.html" class="macro"
title="macro ruma_common::mxc_uri">mxc_uri</a>  
Compile-time checked
[`MxcUri`](struct.MxcUri.html "struct ruma_common::MxcUri")
construction.

<a href="macro.owned_base64_public_key.html" class="macro"
title="macro ruma_common::owned_base64_public_key">owned_base64_public_key</a>  
Compile-time checked
[`OwnedBase64PublicKey`](struct.OwnedBase64PublicKey.html "struct ruma_common::OwnedBase64PublicKey")
construction.

<a href="macro.owned_device_id.html" class="macro"
title="macro ruma_common::owned_device_id">owned_device_id</a>  
Shorthand for `OwnedDeviceId::from`.

<a href="macro.owned_event_id.html" class="macro"
title="macro ruma_common::owned_event_id">owned_event_id</a>  
Compile-time checked
[`OwnedEventId`](struct.OwnedEventId.html "struct ruma_common::OwnedEventId")
construction.

<a href="macro.owned_mxc_uri.html" class="macro"
title="macro ruma_common::owned_mxc_uri">owned_mxc_uri</a>  
Compile-time checked
[`OwnedMxcUri`](struct.OwnedMxcUri.html "struct ruma_common::OwnedMxcUri")
construction.

<a href="macro.owned_room_alias_id.html" class="macro"
title="macro ruma_common::owned_room_alias_id">owned_room_alias_id</a>  
Compile-time checked
[`OwnedRoomAliasId`](struct.OwnedRoomAliasId.html "struct ruma_common::OwnedRoomAliasId")
construction.

<a href="macro.owned_room_id.html" class="macro"
title="macro ruma_common::owned_room_id">owned_room_id</a>  
Compile-time checked
[`OwnedRoomId`](struct.OwnedRoomId.html "struct ruma_common::OwnedRoomId")
construction.

<a href="macro.owned_server_name.html" class="macro"
title="macro ruma_common::owned_server_name">owned_server_name</a>  
Compile-time checked
[`OwnedServerName`](struct.OwnedServerName.html "struct ruma_common::OwnedServerName")
construction.

<a href="macro.owned_server_signing_key_version.html" class="macro"
title="macro ruma_common::owned_server_signing_key_version">owned_server_signing_key_version</a>  
Compile-time checked
[`OwnedServerSigningKeyVersion`](struct.OwnedServerSigningKeyVersion.html "struct ruma_common::OwnedServerSigningKeyVersion")
construction.

<a href="macro.owned_session_id.html" class="macro"
title="macro ruma_common::owned_session_id">owned_session_id</a>  
Compile-time checked
[`OwnedSessionId`](struct.OwnedSessionId.html "struct ruma_common::OwnedSessionId")
construction.

<a href="macro.owned_user_id.html" class="macro"
title="macro ruma_common::owned_user_id">owned_user_id</a>  
Compile-time checked
[`OwnedUserId`](struct.OwnedUserId.html "struct ruma_common::OwnedUserId")
construction.

<a href="macro.room_alias_id.html" class="macro"
title="macro ruma_common::room_alias_id">room_alias_id</a>  
Compile-time checked
[`RoomAliasId`](struct.RoomAliasId.html "struct ruma_common::RoomAliasId")
construction.

<a href="macro.room_id.html" class="macro"
title="macro ruma_common::room_id">room_id</a>  
Compile-time checked
[`RoomId`](struct.RoomId.html "struct ruma_common::RoomId")
construction.

<a href="macro.room_version_id.html" class="macro"
title="macro ruma_common::room_version_id">room_version_id</a>  
Compile-time checked
[`RoomVersionId`](enum.RoomVersionId.html "enum ruma_common::RoomVersionId")
construction.

<a href="macro.server_name.html" class="macro"
title="macro ruma_common::server_name">server_name</a>  
Compile-time checked
[`ServerName`](struct.ServerName.html "struct ruma_common::ServerName")
construction.

<a href="macro.server_signing_key_version.html" class="macro"
title="macro ruma_common::server_signing_key_version">server_signing_key_version</a>  
Compile-time checked
[`ServerSigningKeyVersion`](struct.ServerSigningKeyVersion.html "struct ruma_common::ServerSigningKeyVersion")
construction.

<a href="macro.session_id.html" class="macro"
title="macro ruma_common::session_id">session_id</a>  
Compile-time checked
[`SessionId`](struct.SessionId.html "struct ruma_common::SessionId")
construction.

<a href="macro.user_id.html" class="macro"
title="macro ruma_common::user_id">user_id</a>  
Compile-time checked
[`UserId`](struct.UserId.html "struct ruma_common::UserId")
construction.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AnyKeyName.html" class="struct"
title="struct ruma_common::AnyKeyName">AnyKeyName</a>  
An opaque identifier type to use with
[`KeyId`](struct.KeyId.html "struct ruma_common::KeyId").

<a href="struct.Base64PublicKey.html" class="struct"
title="struct ruma_common::Base64PublicKey">Base64PublicKey</a>  
A public key encoded using unpadded base64, used as an identifier for
[cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
keys.

<a href="struct.Base64PublicKeyOrDeviceId.html" class="struct"
title="struct ruma_common::Base64PublicKeyOrDeviceId">Base64PublicKeyOrDeviceId</a>  
A Matrix ID that can be either a
[`DeviceId`](struct.DeviceId.html "struct ruma_common::DeviceId") or a
[`Base64PublicKey`](struct.Base64PublicKey.html "struct ruma_common::Base64PublicKey").

<a href="struct.ClientSecret.html" class="struct"
title="struct ruma_common::ClientSecret">ClientSecret</a>  
A client secret.

<a href="struct.DeviceId.html" class="struct"
title="struct ruma_common::DeviceId">DeviceId</a>  
A Matrix device ID.

<a href="struct.EventId.html" class="struct"
title="struct ruma_common::EventId">EventId</a>  
A Matrix [event
ID](https://spec.matrix.org/latest/appendices/#event-ids).

<a href="struct.KeyId.html" class="struct"
title="struct ruma_common::KeyId">KeyId</a>  
A key algorithm and key name delimited by a colon.

<a href="struct.MatrixToUri.html" class="struct"
title="struct ruma_common::MatrixToUri">MatrixToUri</a>  
The [`matrix.to`
URI](https://spec.matrix.org/latest/appendices/#matrixto-navigation)
representation of a user, room or event.

<a href="struct.MatrixUri.html" class="struct"
title="struct ruma_common::MatrixUri">MatrixUri</a>  
The [`matrix:`
URI](https://spec.matrix.org/latest/appendices/#matrix-uri-scheme)
representation of a user, room or event.

<a href="struct.MilliSecondsSinceUnixEpoch.html" class="struct"
title="struct ruma_common::MilliSecondsSinceUnixEpoch">MilliSecondsSinceUnixEpoch</a>  
A timestamp represented as the number of milliseconds since the unix
epoch.

<a href="struct.MxcUri.html" class="struct"
title="struct ruma_common::MxcUri">MxcUri</a>  
A URI that should be a Matrix-spec compliant [MXC
URI](https://spec.matrix.org/latest/client-server-api/#matrix-content-mxc-uris).

<a href="struct.OneTimeKeyName.html" class="struct"
title="struct ruma_common::OneTimeKeyName">OneTimeKeyName</a>  
The name of a [one-time or fallback
key](https://spec.matrix.org/latest/client-server-api/#one-time-and-fallback-keys).

<a href="struct.OwnedBase64PublicKey.html" class="struct"
title="struct ruma_common::OwnedBase64PublicKey">OwnedBase64PublicKey</a>  
Owned variant of
[`Base64PublicKey`](struct.Base64PublicKey.html "struct ruma_common::Base64PublicKey")

<a href="struct.OwnedBase64PublicKeyOrDeviceId.html" class="struct"
title="struct ruma_common::OwnedBase64PublicKeyOrDeviceId">OwnedBase64PublicKeyOrDeviceId</a>  
Owned variant of
[`Base64PublicKeyOrDeviceId`](struct.Base64PublicKeyOrDeviceId.html "struct ruma_common::Base64PublicKeyOrDeviceId")

<a href="struct.OwnedClientSecret.html" class="struct"
title="struct ruma_common::OwnedClientSecret">OwnedClientSecret</a>  
Owned variant of
[`ClientSecret`](struct.ClientSecret.html "struct ruma_common::ClientSecret")

<a href="struct.OwnedDeviceId.html" class="struct"
title="struct ruma_common::OwnedDeviceId">OwnedDeviceId</a>  
Owned variant of
[`DeviceId`](struct.DeviceId.html "struct ruma_common::DeviceId")

<a href="struct.OwnedEventId.html" class="struct"
title="struct ruma_common::OwnedEventId">OwnedEventId</a>  
Owned variant of
[`EventId`](struct.EventId.html "struct ruma_common::EventId")

<a href="struct.OwnedKeyId.html" class="struct"
title="struct ruma_common::OwnedKeyId">OwnedKeyId</a>  
Owned variant of
[`KeyId`](struct.KeyId.html "struct ruma_common::KeyId")

<a href="struct.OwnedMxcUri.html" class="struct"
title="struct ruma_common::OwnedMxcUri">OwnedMxcUri</a>  
Owned variant of
[`MxcUri`](struct.MxcUri.html "struct ruma_common::MxcUri")

<a href="struct.OwnedOneTimeKeyName.html" class="struct"
title="struct ruma_common::OwnedOneTimeKeyName">OwnedOneTimeKeyName</a>  
Owned variant of
[`OneTimeKeyName`](struct.OneTimeKeyName.html "struct ruma_common::OneTimeKeyName")

<a href="struct.OwnedRoomAliasId.html" class="struct"
title="struct ruma_common::OwnedRoomAliasId">OwnedRoomAliasId</a>  
Owned variant of
[`RoomAliasId`](struct.RoomAliasId.html "struct ruma_common::RoomAliasId")

<a href="struct.OwnedRoomId.html" class="struct"
title="struct ruma_common::OwnedRoomId">OwnedRoomId</a>  
Owned variant of
[`RoomId`](struct.RoomId.html "struct ruma_common::RoomId")

<a href="struct.OwnedRoomOrAliasId.html" class="struct"
title="struct ruma_common::OwnedRoomOrAliasId">OwnedRoomOrAliasId</a>  
Owned variant of
[`RoomOrAliasId`](struct.RoomOrAliasId.html "struct ruma_common::RoomOrAliasId")

<a href="struct.OwnedServerName.html" class="struct"
title="struct ruma_common::OwnedServerName">OwnedServerName</a>  
Owned variant of
[`ServerName`](struct.ServerName.html "struct ruma_common::ServerName")

<a href="struct.OwnedServerSigningKeyVersion.html" class="struct"
title="struct ruma_common::OwnedServerSigningKeyVersion">OwnedServerSigningKeyVersion</a>  
Owned variant of
[`ServerSigningKeyVersion`](struct.ServerSigningKeyVersion.html "struct ruma_common::ServerSigningKeyVersion")

<a href="struct.OwnedSessionId.html" class="struct"
title="struct ruma_common::OwnedSessionId">OwnedSessionId</a>  
Owned variant of
[`SessionId`](struct.SessionId.html "struct ruma_common::SessionId")

<a href="struct.OwnedSpaceChildOrder.html" class="struct"
title="struct ruma_common::OwnedSpaceChildOrder">OwnedSpaceChildOrder</a>  
Owned variant of
[`SpaceChildOrder`](struct.SpaceChildOrder.html "struct ruma_common::SpaceChildOrder")

<a href="struct.OwnedTransactionId.html" class="struct"
title="struct ruma_common::OwnedTransactionId">OwnedTransactionId</a>  
Owned variant of
[`TransactionId`](struct.TransactionId.html "struct ruma_common::TransactionId")

<a href="struct.OwnedUserId.html" class="struct"
title="struct ruma_common::OwnedUserId">OwnedUserId</a>  
Owned variant of
[`UserId`](struct.UserId.html "struct ruma_common::UserId")

<a href="struct.OwnedVoipId.html" class="struct"
title="struct ruma_common::OwnedVoipId">OwnedVoipId</a>  
Owned variant of
[`VoipId`](struct.VoipId.html "struct ruma_common::VoipId")

<a href="struct.RoomAliasId.html" class="struct"
title="struct ruma_common::RoomAliasId">RoomAliasId</a>  
A Matrix [room alias
ID](https://spec.matrix.org/latest/appendices/#room-aliases).

<a href="struct.RoomId.html" class="struct"
title="struct ruma_common::RoomId">RoomId</a>  
A Matrix [room ID](https://spec.matrix.org/latest/appendices/#room-ids).

<a href="struct.RoomOrAliasId.html" class="struct"
title="struct ruma_common::RoomOrAliasId">RoomOrAliasId</a>  
A Matrix [room ID](https://spec.matrix.org/latest/appendices/#room-ids)
or a Matrix [room alias
ID](https://spec.matrix.org/latest/appendices/#room-aliases).

<a href="struct.SecondsSinceUnixEpoch.html" class="struct"
title="struct ruma_common::SecondsSinceUnixEpoch">SecondsSinceUnixEpoch</a>  
A timestamp represented as the number of seconds since the unix epoch.

<a href="struct.ServerName.html" class="struct"
title="struct ruma_common::ServerName">ServerName</a>  
A Matrix-spec compliant [server
name](https://spec.matrix.org/latest/appendices/#server-name).

<a href="struct.ServerSigningKeyVersion.html" class="struct"
title="struct ruma_common::ServerSigningKeyVersion">ServerSigningKeyVersion</a>  
The version of a [homeserver signing
key](https://spec.matrix.org/latest/server-server-api/#retrieving-server-keys).

<a href="struct.SessionId.html" class="struct"
title="struct ruma_common::SessionId">SessionId</a>  
A session ID.

<a href="struct.Signatures.html" class="struct"
title="struct ruma_common::Signatures">Signatures</a>  
Map of all signatures, grouped by entity.

<a href="struct.SpaceChildOrder.html" class="struct"
title="struct ruma_common::SpaceChildOrder">SpaceChildOrder</a>  
The order of an
[`m.space.child`](https://spec.matrix.org/latest/client-server-api/#mspacechild)
event.

<a href="struct.TransactionId.html" class="struct"
title="struct ruma_common::TransactionId">TransactionId</a>  
A Matrix transaction ID.

<a href="struct.UserId.html" class="struct"
title="struct ruma_common::UserId">UserId</a>  
A Matrix [user
ID](https://spec.matrix.org/latest/appendices/#user-identifiers).

<a href="struct.VoipId.html" class="struct"
title="struct ruma_common::VoipId">VoipId</a>  
A VoIP identifier.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.DeviceKeyAlgorithm.html" class="enum"
title="enum ruma_common::DeviceKeyAlgorithm">DeviceKeyAlgorithm</a>  
The algorithms for the [device
keys](https://spec.matrix.org/latest/client-server-api/#device-keys)
defined in the Matrix spec.

<a href="enum.EventEncryptionAlgorithm.html" class="enum"
title="enum ruma_common::EventEncryptionAlgorithm">EventEncryptionAlgorithm</a>  
An encryption algorithm to be used to encrypt messages sent to a room.

<a href="enum.IdParseError.html" class="enum"
title="enum ruma_common::IdParseError">IdParseError</a>  
An error encountered when trying to parse an invalid ID string.

<a href="enum.KeyDerivationAlgorithm.html" class="enum"
title="enum ruma_common::KeyDerivationAlgorithm">KeyDerivationAlgorithm</a>  
A key algorithm to be used to generate a key from a passphrase.

<a href="enum.MatrixIdError.html" class="enum"
title="enum ruma_common::MatrixIdError">MatrixIdError</a>  
An error occurred while validating a `MatrixId`.

<a href="enum.MatrixToError.html" class="enum"
title="enum ruma_common::MatrixToError">MatrixToError</a>  
An error occurred while validating a `matrix.to` URI.

<a href="enum.MatrixUriError.html" class="enum"
title="enum ruma_common::MatrixUriError">MatrixUriError</a>  
An error occurred while validating a `MatrixURI`.

<a href="enum.MxcUriError.html" class="enum"
title="enum ruma_common::MxcUriError">MxcUriError</a>  
An error occurred while validating an MXC URI.

<a href="enum.OneTimeKeyAlgorithm.html" class="enum"
title="enum ruma_common::OneTimeKeyAlgorithm">OneTimeKeyAlgorithm</a>  
The algorithms for [one-time and fallback
keys](https://spec.matrix.org/latest/client-server-api/#one-time-and-fallback-keys)
defined in the Matrix spec.

<a href="enum.RoomVersionId.html" class="enum"
title="enum ruma_common::RoomVersionId">RoomVersionId</a>  
A Matrix [room version](https://spec.matrix.org/latest/rooms/) ID.

<a href="enum.SigningKeyAlgorithm.html" class="enum"
title="enum ruma_common::SigningKeyAlgorithm">SigningKeyAlgorithm</a>  
The signing key algorithms defined in the Matrix spec.

<a href="enum.VoipVersionId.html" class="enum"
title="enum ruma_common::VoipVersionId">VoipVersionId</a>  
A Matrix VoIP version ID.

<a href="enum.VoipVersionIdError.html" class="enum"
title="enum ruma_common::VoipVersionIdError">VoipVersionIdError</a>  
An error occurred while validating a `VoipVersionId`.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ID_MAX_BYTES.html" class="constant"
title="constant ruma_common::ID_MAX_BYTES">ID_MAX_BYTES</a>  
The maximum allowed length of Matrix identifiers, in bytes.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.KeyAlgorithm.html" class="trait"
title="trait ruma_common::KeyAlgorithm">KeyAlgorithm</a>  
The algorithm of a key.

<a href="trait.KeyName.html" class="trait"
title="trait ruma_common::KeyName">KeyName</a>  
Helper trait to validate the name of a key.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.CrossSigningKeyId.html" class="type"
title="type ruma_common::CrossSigningKeyId">CrossSigningKeyId</a>  
Algorithm + key name for
[cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
keys.

<a href="type.CrossSigningOrDeviceSignatures.html" class="type"
title="type ruma_common::CrossSigningOrDeviceSignatures">CrossSigningOrDeviceSignatures</a>  
Map of cross-signing or device signatures, grouped by user.

<a href="type.CrossSigningOrDeviceSigningKeyId.html" class="type"
title="type ruma_common::CrossSigningOrDeviceSigningKeyId">CrossSigningOrDeviceSigningKeyId</a>  
Algorithm + key name for
[cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
or [device
signing](https://spec.matrix.org/latest/client-server-api/#device-keys)
keys.

<a href="type.DeviceKeyId.html" class="type"
title="type ruma_common::DeviceKeyId">DeviceKeyId</a>  
Algorithm + key name for [device
keys](https://spec.matrix.org/latest/client-server-api/#device-keys).

<a href="type.DeviceSignatures.html" class="type"
title="type ruma_common::DeviceSignatures">DeviceSignatures</a>  
Map of device signatures, grouped by user.

<a href="type.DeviceSigningKeyId.html" class="type"
title="type ruma_common::DeviceSigningKeyId">DeviceSigningKeyId</a>  
Algorithm + key name for [device signing
keys](https://spec.matrix.org/latest/client-server-api/#device-keys).

<a href="type.EntitySignatures.html" class="type"
title="type ruma_common::EntitySignatures">EntitySignatures</a>  
Map of key identifier to signature values.

<a href="type.OneTimeKeyId.html" class="type"
title="type ruma_common::OneTimeKeyId">OneTimeKeyId</a>  
Algorithm + key name for [one-time and fallback
keys](https://spec.matrix.org/latest/client-server-api/#one-time-and-fallback-keys).

<a href="type.OwnedCrossSigningKeyId.html" class="type"
title="type ruma_common::OwnedCrossSigningKeyId">OwnedCrossSigningKeyId</a>  
Algorithm + key name for
[cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
keys.

<a href="type.OwnedCrossSigningOrDeviceSigningKeyId.html" class="type"
title="type ruma_common::OwnedCrossSigningOrDeviceSigningKeyId">OwnedCrossSigningOrDeviceSigningKeyId</a>  
Algorithm + key name for
[cross-signing](https://spec.matrix.org/latest/client-server-api/#cross-signing)
or [device
signing](https://spec.matrix.org/latest/client-server-api/#device-keys)
keys.

<a href="type.OwnedDeviceKeyId.html" class="type"
title="type ruma_common::OwnedDeviceKeyId">OwnedDeviceKeyId</a>  
Algorithm + key name for [device
keys](https://spec.matrix.org/latest/client-server-api/#device-keys).

<a href="type.OwnedDeviceSigningKeyId.html" class="type"
title="type ruma_common::OwnedDeviceSigningKeyId">OwnedDeviceSigningKeyId</a>  
Algorithm + key name for \[device signing\] keys.

<a href="type.OwnedOneTimeKeyId.html" class="type"
title="type ruma_common::OwnedOneTimeKeyId">OwnedOneTimeKeyId</a>  
Algorithm + key name for [one-time and fallback
keys](https://spec.matrix.org/latest/client-server-api/#one-time-and-fallback-keys).

<a href="type.OwnedServerSigningKeyId.html" class="type"
title="type ruma_common::OwnedServerSigningKeyId">OwnedServerSigningKeyId</a>  
Algorithm + key name for homeserver signing keys.

<a href="type.OwnedSigningKeyId.html" class="type"
title="type ruma_common::OwnedSigningKeyId">OwnedSigningKeyId</a>  
Algorithm + key name for signing keys.

<a href="type.ServerSignatures.html" class="type"
title="type ruma_common::ServerSignatures">ServerSignatures</a>  
Map of server signatures, grouped by server.

<a href="type.ServerSigningKeyId.html" class="type"
title="type ruma_common::ServerSigningKeyId">ServerSigningKeyId</a>  
Algorithm + key name for homeserver signing keys.

<a href="type.SigningKeyId.html" class="type"
title="type ruma_common::SigningKeyId">SigningKeyId</a>  
Algorithm + key name for signing keys.
