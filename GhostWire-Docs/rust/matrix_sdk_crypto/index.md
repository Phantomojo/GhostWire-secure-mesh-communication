Crate matrix\_sdk\_crypto Copy item path

<span class="sub-heading"><a href="../src/matrix_sdk_crypto/lib.rs.html#16-1131"
class="src">Source</a> </span>

Expand description

A no-network-IO implementation of a state machine that handles
end-to-end encryption for [Matrix](https://matrix.org/) clients.

If you’re just trying to write a Matrix client or bot in Rust, you’re
probably looking for
[matrix-sdk](https://github.com/matrix-org/matrix-rust-sdk/) instead.

However, if you’re looking to add end-to-end encryption to an existing
Matrix client or library, read on.

The state machine works in a push/pull manner:

-   you push state changes and events retrieved from a Matrix homeserver
    /sync response into the state machine
-   you pull requests that you’ll need to send back to the homeserver
    out of the state machine

``` {.rust .rust-example-rendered}
use std::collections::BTreeMap;

use matrix_sdk_crypto::{
    DecryptionSettings, EncryptionSyncChanges, OlmError, OlmMachine, TrustRequirement,
};
use ruma::{api::client::sync::sync_events::DeviceLists, device_id, user_id};

#[tokio::main]
async fn main() -> Result<(), OlmError> {
    let alice = user_id!("@alice:example.org");
    let machine = OlmMachine::new(&alice, device_id!("DEVICEID")).await;

    let changed_devices = DeviceLists::default();
    let one_time_key_counts = BTreeMap::default();
    let unused_fallback_keys = Some(Vec::new());
    let next_batch_token = "T0K3N".to_owned();

    let decryption_settings =
        DecryptionSettings { sender_device_trust_requirement: TrustRequirement::Untrusted };

    // Push changes that the server sent to us in a sync response.
    let decrypted_to_device = machine
        .receive_sync_changes(
            EncryptionSyncChanges {
                to_device_events: vec![],
                changed_devices: &changed_devices,
                one_time_keys_counts: &one_time_key_counts,
                unused_fallback_keys: unused_fallback_keys.as_deref(),
                next_batch_token: Some(next_batch_token),
            },
            &decryption_settings,
        )
        .await?;

    // Pull requests that we need to send out.
    let outgoing_requests = machine.outgoing_requests().await?;

    // Send the requests out here and call machine.mark_request_as_sent().

    Ok(())
}
```

It is recommended to use the
[tutorial](tutorial/index.html "mod matrix_sdk_crypto::tutorial") to
understand how end-to-end encryption works in Matrix and how to add
end-to-end encryption support in your Matrix client library.

# <a href="#crate-feature-flags" class="doc-anchor">§</a>Crate Feature Flags

The following crate feature flags are available:

| Feature | Default | Description |
|----|:--:|----|
| `qrcode` | No | Enables QR code based interactive verification |
| `js` | No | Enables JavaScript API usage for things like the current system time on WASM (does nothing on other targets) |
| `testing` | No | Provides facilities and functions for tests, in particular for integration testing store implementations. ATTENTION: do not ever use outside of tests, we do not provide any stability warantees on these, these are merely helpers. If you find you *need* any function provided here outside of tests, please open a Github Issue and inform us about your use case for us to consider. |

-   `testing`: Provides facilities and functions for tests, in
    particular for integration testing store implementations. ATTENTION:
    do not ever use outside of tests, we do not provide any stability
    warantees on these, these are merely helpers. If you find you *need*
    any function provided here outside of tests, please open a Github
    Issue and inform us about your use case for us to consider.

-   `_disable-minimum-rotation-period-ms`: Do not use except for
    testing. Disables the floor on the rotation period of room keys.

# <a href="#enabling-logging" class="doc-anchor">§</a>Enabling logging

Users of the `matrix-sdk-crypto` crate can enable log output by
depending on the `tracing-subscriber` crate and including the following
line in their application (e.g. at the start of `main`):

``` language-no_compile
tracing_subscriber::fmt::init();
```

The log output is controlled via the `RUST_LOG` environment variable by
setting it to one of the `error`, `warn`, `info`, `debug` or `trace`
levels. The output is printed to stdout.

The `RUST_LOG` variable also supports a more advanced syntax for
filtering log output more precisely, for instance with crate-level
granularity. For more information on this, check out the
[tracing-subscriber
documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use olm::`<a href="olm/struct.Account.html" class="struct"
title="struct matrix_sdk_crypto::olm::Account"><code>Account</code></a>`;`

`pub use olm::`<a href="olm/struct.CrossSigningStatus.html" class="struct"
title="struct matrix_sdk_crypto::olm::CrossSigningStatus"><code>CrossSigningStatus</code></a>`;`

`pub use olm::`<a href="olm/struct.EncryptionSettings.html" class="struct"
title="struct matrix_sdk_crypto::olm::EncryptionSettings"><code>EncryptionSettings</code></a>`;`

`pub use olm::`<a href="olm/struct.Session.html" class="struct"
title="struct matrix_sdk_crypto::olm::Session"><code>Session</code></a>`;`

`pub use store::types::`<a href="store/types/struct.CrossSigningKeyExport.html" class="struct"
title="struct matrix_sdk_crypto::store::types::CrossSigningKeyExport"><code>CrossSigningKeyExport</code></a>`;`

`pub use store::types::`<a href="store/types/struct.TrackedUser.html" class="struct"
title="struct matrix_sdk_crypto::store::types::TrackedUser"><code>TrackedUser</code></a>`;`

`pub use store::`<a href="store/enum.CryptoStoreError.html" class="enum"
title="enum matrix_sdk_crypto::store::CryptoStoreError"><code>CryptoStoreError</code></a>`;`

`pub use store::`<a href="store/enum.SecretImportError.html" class="enum"
title="enum matrix_sdk_crypto::store::SecretImportError"><code>SecretImportError</code></a>`;`

`pub use store::`<a href="store/enum.SecretInfo.html" class="enum"
title="enum matrix_sdk_crypto::store::SecretInfo"><code>SecretInfo</code></a>`;`

`pub use `<a href="../vodozemac/index.html" class="mod"
title="mod vodozemac"><code>vodozemac</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="backups/index.html" class="mod"
title="mod matrix_sdk_crypto::backups">backups</a>  
Server-side backup support for room keys

<a href="dehydrated_devices/index.html" class="mod"
title="mod matrix_sdk_crypto::dehydrated_devices">dehydrated_devices</a>  
Submodule for device dehydration support.

<a href="olm/index.html" class="mod"
title="mod matrix_sdk_crypto::olm">olm</a>  
The crypto specific Olm objects.

<a href="secret_storage/index.html" class="mod"
title="mod matrix_sdk_crypto::secret_storage">secret_storage</a>  
Helpers for implementing the Secrets Storage mechanism from the Matrix
[spec](https://spec.matrix.org/v1.8/client-server-api/#storage).

<a href="store/index.html" class="mod"
title="mod matrix_sdk_crypto::store">store</a>  
Types and traits to implement the storage layer for the
[`OlmMachine`](/matrix_sdk_crypto/struct.OlmMachine.html)

<a href="tutorial/index.html" class="mod"
title="mod matrix_sdk_crypto::tutorial">tutorial</a>  
A step by step guide that explains how to include
[end-to-end-encryption](https://en.wikipedia.org/wiki/End-to-end_encryption)
support in a [Matrix](https://matrix.org/) client library.

<a href="types/index.html" class="mod"
title="mod matrix_sdk_crypto::types">types</a>  
Module containing customized types modeling Matrix keys and events.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AcceptSettings.html" class="struct"
title="struct matrix_sdk_crypto::AcceptSettings">AcceptSettings</a>  
Customize the accept-reply for a verification process

<a href="struct.AcceptedProtocols.html" class="struct"
title="struct matrix_sdk_crypto::AcceptedProtocols">AcceptedProtocols</a>  
Struct containing the protocols that were agreed to be used for the SAS
flow.

<a href="struct.AttachmentDecryptor.html" class="struct"
title="struct matrix_sdk_crypto::AttachmentDecryptor">AttachmentDecryptor</a>  
A wrapper that transparently encrypts anything that implements `Read` as
an Matrix attachment.

<a href="struct.AttachmentEncryptor.html" class="struct"
title="struct matrix_sdk_crypto::AttachmentEncryptor">AttachmentEncryptor</a>  
A wrapper that transparently encrypts anything that implements `Read`.

<a href="struct.CancelInfo.html" class="struct"
title="struct matrix_sdk_crypto::CancelInfo">CancelInfo</a>  
Information about the cancellation of a verification request or
verification flow.

<a href="struct.CrossSigningBootstrapRequests.html" class="struct"
title="struct matrix_sdk_crypto::CrossSigningBootstrapRequests">CrossSigningBootstrapRequests</a>  
A set of requests to be executed when bootstrapping cross-signing using
[`OlmMachine::bootstrap_cross_signing`](struct.OlmMachine.html#method.bootstrap_cross_signing "method matrix_sdk_crypto::OlmMachine::bootstrap_cross_signing").

<a href="struct.DecryptionSettings.html" class="struct"
title="struct matrix_sdk_crypto::DecryptionSettings">DecryptionSettings</a>  
Settings for decrypting messages

<a href="struct.Device.html" class="struct"
title="struct matrix_sdk_crypto::Device">Device</a>  
A device represents a E2EE capable client of an user.

<a href="struct.DeviceData.html" class="struct"
title="struct matrix_sdk_crypto::DeviceData">DeviceData</a>  
A read-only version of a `Device`.

<a href="struct.Emoji.html" class="struct"
title="struct matrix_sdk_crypto::Emoji">Emoji</a>  
An emoji that is used for interactive verification using a short auth
string.

<a href="struct.EmojiShortAuthString.html" class="struct"
title="struct matrix_sdk_crypto::EmojiShortAuthString">EmojiShortAuthString</a>  
The short auth string for the emoji method of SAS verification.

<a href="struct.EncryptionSyncChanges.html" class="struct"
title="struct matrix_sdk_crypto::EncryptionSyncChanges">EncryptionSyncChanges</a>  
Data contained from a sync response and that needs to be processed by
the OlmMachine.

<a href="struct.GossipRequest.html" class="struct"
title="struct matrix_sdk_crypto::GossipRequest">GossipRequest</a>  
A struct describing an outgoing key request.

<a href="struct.GossippedSecret.html" class="struct"
title="struct matrix_sdk_crypto::GossippedSecret">GossippedSecret</a>  
Struct containing a `m.secret.send` event and its accompanying info.

<a href="struct.IdentityStatusChange.html" class="struct"
title="struct matrix_sdk_crypto::IdentityStatusChange">IdentityStatusChange</a>  
A change in the status of the identity of a member of the room. Returned
by
[`RoomIdentityState::process_change`](struct.RoomIdentityState.html#method.process_change "method matrix_sdk_crypto::RoomIdentityState::process_change")
to indicate that something significant changed in this room and we
should either show or hide a warning.

<a href="struct.MediaEncryptionInfo.html" class="struct"
title="struct matrix_sdk_crypto::MediaEncryptionInfo">MediaEncryptionInfo</a>  
Struct holding all the information that is needed to decrypt an
encrypted file.

<a href="struct.OlmMachine.html" class="struct"
title="struct matrix_sdk_crypto::OlmMachine">OlmMachine</a>  
State machine implementation of the Olm/Megolm encryption protocol used
for Matrix end to end encryption.

<a href="struct.OtherUserIdentity.html" class="struct"
title="struct matrix_sdk_crypto::OtherUserIdentity">OtherUserIdentity</a>  
Struct representing a cross signing identity of a user.

<a href="struct.OtherUserIdentityData.html" class="struct"
title="struct matrix_sdk_crypto::OtherUserIdentityData">OtherUserIdentityData</a>  
Struct representing a cross signing identity of a user.

<a href="struct.OwnUserIdentity.html" class="struct"
title="struct matrix_sdk_crypto::OwnUserIdentity">OwnUserIdentity</a>  
Struct representing a cross signing identity of a user.

<a href="struct.OwnUserIdentityData.html" class="struct"
title="struct matrix_sdk_crypto::OwnUserIdentityData">OwnUserIdentityData</a>  
Struct representing a cross signing identity of our own user.

<a href="struct.RoomIdentityState.html" class="struct"
title="struct matrix_sdk_crypto::RoomIdentityState">RoomIdentityState</a>  
The state of the identities in a given room - whether they are:

<a href="struct.RoomKeyImportResult.html" class="struct"
title="struct matrix_sdk_crypto::RoomKeyImportResult">RoomKeyImportResult</a>  
Return type for the room key importing.

<a href="struct.Sas.html" class="struct"
title="struct matrix_sdk_crypto::Sas">Sas</a>  
Short authentication string object.

<a href="struct.UserDevices.html" class="struct"
title="struct matrix_sdk_crypto::UserDevices">UserDevices</a>  
A read only view over all devices belonging to a user.

<a href="struct.VerificationRequest.html" class="struct"
title="struct matrix_sdk_crypto::VerificationRequest">VerificationRequest</a>  
An object controlling key verification requests.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.CollectStrategy.html" class="enum"
title="enum matrix_sdk_crypto::CollectStrategy">CollectStrategy</a>  
Strategy to collect the devices that should receive room keys for the
current discussion.

<a href="enum.DecryptorError.html" class="enum"
title="enum matrix_sdk_crypto::DecryptorError">DecryptorError</a>  
Error type for attachment decryption.

<a href="enum.EventError.html" class="enum"
title="enum matrix_sdk_crypto::EventError">EventError</a>  
Error that occurs when decrypting an event that is malformed.

<a href="enum.IdentityState.html" class="enum"
title="enum matrix_sdk_crypto::IdentityState">IdentityState</a>  
The state of an identity - verified, pinned etc.

<a href="enum.KeyExportError.html" class="enum"
title="enum matrix_sdk_crypto::KeyExportError">KeyExportError</a>  
Error representing a failure during key export or import.

<a href="enum.LocalTrust.html" class="enum"
title="enum matrix_sdk_crypto::LocalTrust">LocalTrust</a>  
The local trust state of a device.

<a href="enum.MegolmError.html" class="enum"
title="enum matrix_sdk_crypto::MegolmError">MegolmError</a>  
Error representing a failure during a group encryption operation.

<a href="enum.OlmError.html" class="enum"
title="enum matrix_sdk_crypto::OlmError">OlmError</a>  
Error representing a failure during a device to device cryptographic
operation.

<a href="enum.RoomEventDecryptionResult.html" class="enum"
title="enum matrix_sdk_crypto::RoomEventDecryptionResult">RoomEventDecryptionResult</a>  
The result of an attempt to decrypt a room event: either a successful
decryption, or information on a failure.

<a href="enum.RoomIdentityChange.html" class="enum"
title="enum matrix_sdk_crypto::RoomIdentityChange">RoomIdentityChange</a>  
The type of update that can be received by
[`RoomIdentityState::process_change`](struct.RoomIdentityState.html#method.process_change "method matrix_sdk_crypto::RoomIdentityState::process_change") -
either a change of someone’s identity, or a change of room membership.

<a href="enum.SasState.html" class="enum"
title="enum matrix_sdk_crypto::SasState">SasState</a>  
An Enum describing the state the SAS verification is in.

<a href="enum.SessionCreationError.html" class="enum"
title="enum matrix_sdk_crypto::SessionCreationError">SessionCreationError</a>  
Error that occurs when a room key can’t be converted into a valid Megolm
session.

<a href="enum.SessionRecipientCollectionError.html" class="enum"
title="enum matrix_sdk_crypto::SessionRecipientCollectionError">SessionRecipientCollectionError</a>  
Error representing a problem when collecting the recipient devices for
the room key, during an encryption operation.

<a href="enum.SetRoomSettingsError.html" class="enum"
title="enum matrix_sdk_crypto::SetRoomSettingsError">SetRoomSettingsError</a>  
Errors that can be returned by
[`crate::machine::OlmMachine::set_room_settings`](struct.OlmMachine.html#method.set_room_settings "method matrix_sdk_crypto::OlmMachine::set_room_settings").

<a href="enum.SignatureError.html" class="enum"
title="enum matrix_sdk_crypto::SignatureError">SignatureError</a>  
Error type describing different errors that happen when we check or
create signatures for a Matrix JSON object.

<a href="enum.TrustRequirement.html" class="enum"
title="enum matrix_sdk_crypto::TrustRequirement">TrustRequirement</a>  
The trust level in the sender’s device that is required to decrypt an
event.

<a href="enum.UserIdentity.html" class="enum"
title="enum matrix_sdk_crypto::UserIdentity">UserIdentity</a>  
Enum over the different user identity types we can have.

<a href="enum.UserIdentityData.html" class="enum"
title="enum matrix_sdk_crypto::UserIdentityData">UserIdentityData</a>  
Enum over the different user identity types we can have.

<a href="enum.Verification.html" class="enum"
title="enum matrix_sdk_crypto::Verification">Verification</a>  
An enum over the different verification types the SDK supports.

<a href="enum.VerificationRequestState.html" class="enum"
title="enum matrix_sdk_crypto::VerificationRequestState">VerificationRequestState</a>  
An Enum describing the state the verification request is in.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.VERSION.html" class="constant"
title="constant matrix_sdk_crypto::VERSION">VERSION</a>  
The version of the matrix-sdk-cypto crate being used

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.RoomIdentityProvider.html" class="trait"
title="trait matrix_sdk_crypto::RoomIdentityProvider">RoomIdentityProvider</a>  
Something that can answer questions about the membership of a room and
the identities of users.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decrypt_room_key_export.html" class="fn"
title="fn matrix_sdk_crypto::decrypt_room_key_export">decrypt_room_key_export</a>  
Try to decrypt a reader into a list of exported room keys.

<a href="fn.encrypt_room_key_export.html" class="fn"
title="fn matrix_sdk_crypto::encrypt_room_key_export">encrypt_room_key_export</a>  
Encrypt the list of exported room keys using the given passphrase.

<a href="fn.format_emojis.html" class="fn"
title="fn matrix_sdk_crypto::format_emojis">format_emojis</a>  
Format the list of emojis as a two line string.
