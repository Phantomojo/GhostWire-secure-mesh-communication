Crate matrix\_sdk Copy item path

<span class="sub-heading"><a href="../src/matrix_sdk/lib.rs.html#16-103" class="src">Source</a>
</span>

Expand description

A high-level, batteries-included [Matrix](https://matrix.org/) client
library written in Rust.

This crate seeks to be a general-purpose library for writing software
using the Matrix [Client-Server
API](https://matrix.org/docs/spec/client_server/latest) to communicate
with a Matrix homeserver. If you’re writing a typical Matrix client or
bot, this is likely the crate you need.

However, the crate is designed in a modular way and depends on several
other lower-level crates. If you’re attempting something more custom,
you might be interested in these:

-   [`matrix_sdk_base`](https://docs.rs/matrix-sdk-base/): A
    no-network-IO client state machine which can be used to embed a
    Matrix client into an existing network stack or to build a new
    Matrix client library on top.
-   [`matrix_sdk_crypto`](https://docs.rs/matrix-sdk-crypto/): A
    no-network-IO encryption state machine which can be used to add
    Matrix E2EE support into an existing client or library.

# <a href="#getting-started" class="doc-anchor">§</a>Getting started

The central component you’ll be interacting with is the
[`Client`](struct.Client.html "struct matrix_sdk::Client"). A basic use
case will include instantiating the client, logging in as a user,
registering some event handlers and then syncing.

This is demonstrated in the example below.

``` {.rust .rust-example-rendered}
use matrix_sdk::{
    Client, config::SyncSettings,
    ruma::{user_id, events::room::message::SyncRoomMessageEvent},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let alice = user_id!("@alice:example.org");
    let client = Client::builder().server_name(alice.server_name()).build().await?;

    // First we need to log in.
    client.matrix_auth().login_username(alice, "password").send().await?;

    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        println!("Received a message {:?}", ev);
    });

    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    client.sync(SyncSettings::default()).await?;

    Ok(())
}
```

More examples can be found in the
[examples](https://github.com/matrix-org/matrix-rust-sdk/tree/main/examples/)
directory.

# <a href="#crate-feature-flags" class="doc-anchor">§</a>Crate Feature Flags

The following crate feature flags are available:

| Feature | Default | Description |
|----|:--:|----|
| `anyhow` | No | Better logging for event handlers that return `anyhow::Result` |
| `e2e-encryption` | Yes | End-to-end encryption (E2EE) support |
| `eyre` | No | Better logging for event handlers that return `eyre::Result` |
| `js` | No | Enables JavaScript API usage on WASM (does nothing on other targets) |
| `markdown` | No | Support for sending Markdown-formatted messages |
| `qrcode` | Yes | QR code verification support |
| `sqlite` | Yes | Persistent storage of state and E2EE data (optionally, if feature `e2e-encryption` is enabled), via SQLite available on system |
| `bundled-sqlite` | No | Persistent storage of state and E2EE data (optionally, if feature `e2e-encryption` is enabled), via SQLite compiled and bundled with the binary |
| `indexeddb` | No | Persistent storage of state and E2EE data (optionally, if feature `e2e-encryption` is enabled) for browsers, via IndexedDB |
| `socks` | No | SOCKS support in the default HTTP client, [`reqwest`](https://docs.rs/reqwest/0.11.5/reqwest/index.html) |
| `sso-login` | No | Support for SSO login with a local HTTP server |

# <a href="#enabling-logging" class="doc-anchor">§</a>Enabling logging

Users of the matrix-sdk crate can enable log output by depending on the
`tracing-subscriber` crate and including the following line in their
application (e.g. at the start of `main`):

``` {.rust .rust-example-rendered}
tracing_subscriber::fmt::init();
```

The log output is controlled via the `RUST_LOG` environment variable by
setting it to one of the `error`, `warn`, `info`, `debug` or `trace`
levels. The output is printed to stdout.

The `RUST_LOG` variable also supports a more advanced syntax for
filtering log output more precisely, for instance with crate-level
granularity. For more information on this, check out the
[tracing\_subscriber
documentation](https://tracing.rs/tracing_subscriber/filter/struct.envfilter).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use authentication::`<a href="authentication/enum.AuthApi.html" class="enum"
title="enum matrix_sdk::authentication::AuthApi"><code>AuthApi</code></a>`;`

`pub use authentication::`<a href="authentication/enum.AuthSession.html" class="enum"
title="enum matrix_sdk::authentication::AuthSession"><code>AuthSession</code></a>`;`

`pub use authentication::`<a href="authentication/struct.SessionTokens.html" class="struct"
title="struct matrix_sdk::authentication::SessionTokens"><code>SessionTokens</code></a>`;`

`pub use media::`<a href="media/struct.Media.html" class="struct"
title="struct matrix_sdk::media::Media"><code>Media</code></a>`;`

`pub use pusher::`<a href="pusher/struct.Pusher.html" class="struct"
title="struct matrix_sdk::pusher::Pusher"><code>Pusher</code></a>`;`

`pub use room::`<a href="room/struct.Room.html" class="struct"
title="struct matrix_sdk::room::Room"><code>Room</code></a>`;`

`pub use sliding_sync::`<a href="sliding_sync/struct.SlidingSync.html" class="struct"
title="struct matrix_sdk::sliding_sync::SlidingSync"><code>SlidingSync</code></a>`;`

`pub use sliding_sync::`<a href="sliding_sync/struct.SlidingSyncBuilder.html" class="struct"
title="struct matrix_sdk::sliding_sync::SlidingSyncBuilder"><code>SlidingSyncBuilder</code></a>`;`

`pub use sliding_sync::`<a href="sliding_sync/struct.SlidingSyncList.html" class="struct"
title="struct matrix_sdk::sliding_sync::SlidingSyncList"><code>SlidingSyncList</code></a>`;`

`pub use sliding_sync::`<a href="sliding_sync/struct.SlidingSyncListBuilder.html" class="struct"
title="struct matrix_sdk::sliding_sync::SlidingSyncListBuilder"><code>SlidingSyncListBuilder</code></a>`;`

`pub use sliding_sync::`<a href="sliding_sync/enum.SlidingSyncListLoadingState.html"
class="enum"
title="enum matrix_sdk::sliding_sync::SlidingSyncListLoadingState"><code>SlidingSyncListLoadingState</code></a>`;`

`pub use sliding_sync::`<a href="sliding_sync/enum.SlidingSyncMode.html" class="enum"
title="enum matrix_sdk::sliding_sync::SlidingSyncMode"><code>SlidingSyncMode</code></a>`;`

`pub use sliding_sync::`<a href="sliding_sync/struct.UpdateSummary.html" class="struct"
title="struct matrix_sdk::sliding_sync::UpdateSummary"><code>UpdateSummary</code></a>`;`

`pub use `<a href="../bytes/index.html" class="mod"
title="mod bytes"><code>bytes</code></a>`;`

`pub use `<a href="../reqwest/index.html" class="mod"
title="mod reqwest"><code>reqwest</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="attachment/index.html" class="mod"
title="mod matrix_sdk::attachment">attachment</a>  
Types and traits for attachments.

<a href="authentication/index.html" class="mod"
title="mod matrix_sdk::authentication">authentication</a>  
Types and functions related to authentication in Matrix.

<a href="config/index.html" class="mod"
title="mod matrix_sdk::config">config</a>  
Configuration to change the behaviour of the
[`Client`](struct.Client.html "struct matrix_sdk::Client").

<a href="cross_process_lock/index.html" class="mod"
title="mod matrix_sdk::cross_process_lock">cross_process_lock</a>  
A cross-process lock implementation.

<a href="debug/index.html" class="mod"
title="mod matrix_sdk::debug">debug</a>  
Helpers for creating `std::fmt::Debug` implementations.

<a href="deserialized_responses/index.html" class="mod"
title="mod matrix_sdk::deserialized_responses">deserialized_responses</a>  
SDK-specific variations of response types from Ruma.

<a href="encryption/index.html" class="mod"
title="mod matrix_sdk::encryption">encryption</a>  
End-to-end encryption related types

<a href="event_cache/index.html" class="mod"
title="mod matrix_sdk::event_cache">event_cache</a>  
The event cache is an abstraction layer, sitting between the Rust SDK
and a final client, that acts as a global observer of all the rooms,
gathering and inferring some extra useful information about each room.
In particular, this doesn’t require subscribing to a specific room to
get access to this information.

<a href="event_handler/index.html" class="mod"
title="mod matrix_sdk::event_handler">event_handler</a>  
Types and traits related for event handlers. For usage, see
[`Client::add_event_handler`](struct.Client.html#method.add_event_handler "method matrix_sdk::Client::add_event_handler").

<a href="executor/index.html" class="mod"
title="mod matrix_sdk::executor">executor</a>  
Abstraction over an executor so we can spawn tasks under Wasm the same
way we do usually.

<a href="failures_cache/index.html" class="mod"
title="mod matrix_sdk::failures_cache">failures_cache</a>  
A TTL cache which can be used to time out repeated operations that might
experience intermittent failures.

<a href="futures/index.html" class="mod"
title="mod matrix_sdk::futures">futures</a>  
Named futures returned from methods on types in [the crate
root](index.html "mod matrix_sdk").

<a href="latest_events/index.html" class="mod"
title="mod matrix_sdk::latest_events">latest_events</a>  
The Latest Events API provides a lazy, reactive and efficient way to
compute the latest event for a room or a thread.

<a href="linked_chunk/index.html" class="mod"
title="mod matrix_sdk::linked_chunk">linked_chunk</a>  
A linked chunk is the underlying data structure that holds all events.

<a href="live_location_share/index.html" class="mod"
title="mod matrix_sdk::live_location_share">live_location_share</a>  
Types for live location sharing.

<a href="locks/index.html" class="mod"
title="mod matrix_sdk::locks">locks</a>  
Simplified locks hat panic instead of returning a `Result` when the lock
is poisoned.

<a href="media/index.html" class="mod"
title="mod matrix_sdk::media">media</a>  
High-level media API.

<a href="notification_settings/index.html" class="mod"
title="mod matrix_sdk::notification_settings">notification_settings</a>  
High-level push notification settings API

<a href="paginators/index.html" class="mod"
title="mod matrix_sdk::paginators">paginators</a>  
Stateful paginators to help with paginated APIs.

<a href="pusher/index.html" class="mod"
title="mod matrix_sdk::pusher">pusher</a>  
High-level pusher API.

<a href="ring_buffer/index.html" class="mod"
title="mod matrix_sdk::ring_buffer">ring_buffer</a>  
<a href="room/index.html" class="mod"
title="mod matrix_sdk::room">room</a>  
High-level room API

<a href="room_directory_search/index.html" class="mod"
title="mod matrix_sdk::room_directory_search">room_directory_search</a>  
Types for searching the public room directory.

<a href="room_preview/index.html" class="mod"
title="mod matrix_sdk::room_preview">room_preview</a>  
Preview of a room, whether we’ve joined it/left it/been invited to it,
or not.

<a href="ruma/index.html" class="mod"
title="mod matrix_sdk::ruma">ruma</a>  
Types and traits for working with the [Matrix](https://matrix.org)
protocol.

<a href="send_queue/index.html" class="mod"
title="mod matrix_sdk::send_queue">send_queue</a>  
A send queue facility to serializing queuing and sending of messages.

<a href="serde_helpers/index.html" class="mod"
title="mod matrix_sdk::serde_helpers">serde_helpers</a>  
A collection of serde helpers to avoid having to deserialize an entire
event to access some fields.

<a href="sleep/index.html" class="mod"
title="mod matrix_sdk::sleep">sleep</a>  
<a href="sliding_sync/index.html" class="mod"
title="mod matrix_sdk::sliding_sync">sliding_sync</a>  
Sliding Sync Client implementation of
[MSC3575](https://github.com/matrix-org/matrix-spec-proposals/pull/3575)
& extensions

<a href="store/index.html" class="mod"
title="mod matrix_sdk::store">store</a>  
The state store holds the overall state for rooms, users and their
profiles and their timelines. It is an overall cache for faster access
and convenience- accessible through `Store`.

<a href="stream/index.html" class="mod"
title="mod matrix_sdk::stream">stream</a>  
Platform-specific stream utilities.

<a href="sync/index.html" class="mod"
title="mod matrix_sdk::sync">sync</a>  
The SDK’s representation of the result of a `/sync` request.

<a href="timeout/index.html" class="mod"
title="mod matrix_sdk::timeout">timeout</a>  
<a href="tracing_timer/index.html" class="mod"
title="mod matrix_sdk::tracing_timer">tracing_timer</a>  
<a href="ttl_cache/index.html" class="mod"
title="mod matrix_sdk::ttl_cache">ttl_cache</a>  
A TTL cache which can be used to time out repeated operations that might
experience intermittent failures.

<a href="utils/index.html" class="mod"
title="mod matrix_sdk::utils">utils</a>  
Utility types and traits.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.boxed_into_future.html" class="macro"
title="macro matrix_sdk::boxed_into_future">boxed_into_future</a>  
<a href="macro.timer.html" class="macro"
title="macro matrix_sdk::timer">timer</a>  
Macro to create a RAII timer that will log on `Drop` how long its
covered section took to execute.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Account.html" class="struct"
title="struct matrix_sdk::Account">Account</a>  
A high-level API to manage the client owner’s account.

<a href="struct.BaseRoom.html" class="struct"
title="struct matrix_sdk::BaseRoom">BaseRoom</a>  
The underlying room data structure collecting state for joined, left and
invited rooms.

<a href="struct.BaseRoomMember.html" class="struct"
title="struct matrix_sdk::BaseRoomMember">BaseRoomMember</a>  
A member of a room.

<a href="struct.Client.html" class="struct"
title="struct matrix_sdk::Client">Client</a>  
An async/await enabled Matrix client.

<a href="struct.ClientBuilder.html" class="struct"
title="struct matrix_sdk::ClientBuilder">ClientBuilder</a>  
Builder that allows creating and configuring various parts of a
[`Client`](struct.Client.html "struct matrix_sdk::Client").

<a href="struct.ComposerDraft.html" class="struct"
title="struct matrix_sdk::ComposerDraft">ComposerDraft</a>  
Current draft of the composer for the room.

<a href="struct.DraftAttachment.html" class="struct"
title="struct matrix_sdk::DraftAttachment">DraftAttachment</a>  
An attachment stored with a composer draft.

<a href="struct.DraftThumbnail.html" class="struct"
title="struct matrix_sdk::DraftThumbnail">DraftThumbnail</a>  
Thumbnail data for a draft attachment.

<a href="struct.MemoryStore.html" class="struct"
title="struct matrix_sdk::MemoryStore">MemoryStore</a>  
In-memory, non-persistent implementation of the `StateStore`.

<a href="struct.OwnedServerName.html" class="struct"
title="struct matrix_sdk::OwnedServerName">OwnedServerName</a>  
Owned variant of
[`ServerName`](struct.ServerName.html "struct matrix_sdk::ServerName")

<a href="struct.PredecessorRoom.html" class="struct"
title="struct matrix_sdk::PredecessorRoom">PredecessorRoom</a>  
When a room A is tombstoned, it is replaced by a room B. The room A is
the predecessor of B, and B is the successor of A. This type holds
information about the predecessor room. See
[`Room::predecessor_room`](struct.BaseRoom.html#method.predecessor_room "method matrix_sdk::BaseRoom::predecessor_room").

<a href="struct.RoomCreateWithCreatorEventContent.html" class="struct"
title="struct matrix_sdk::RoomCreateWithCreatorEventContent">RoomCreateWithCreatorEventContent</a>  
The content of an `m.room.create` event, with a required `creator`
field.

<a href="struct.RoomHero.html" class="struct"
title="struct matrix_sdk::RoomHero">RoomHero</a>  
Information about a member considered to be a room hero.

<a href="struct.RoomInfo.html" class="struct"
title="struct matrix_sdk::RoomInfo">RoomInfo</a>  
The underlying pure data structure for joined and left rooms.

<a href="struct.RoomMemberships.html" class="struct"
title="struct matrix_sdk::RoomMemberships">RoomMemberships</a>  
Room membership filter as a bitset.

<a href="struct.RoomRecencyStamp.html" class="struct"
title="struct matrix_sdk::RoomRecencyStamp">RoomRecencyStamp</a>  
Type to represent a `RoomInfo::recency_stamp`.

<a href="struct.ServerName.html" class="struct"
title="struct matrix_sdk::ServerName">ServerName</a>  
A Matrix-spec compliant [server
name](https://spec.matrix.org/latest/appendices/#server-name).

<a href="struct.ServerVendorInfo.html" class="struct"
title="struct matrix_sdk::ServerVendorInfo">ServerVendorInfo</a>  
Information about the server vendor obtained from the federation API.

<a href="struct.SessionMeta.html" class="struct"
title="struct matrix_sdk::SessionMeta">SessionMeta</a>  
The Matrix user session info.

<a href="struct.SqliteCryptoStore.html" class="struct"
title="struct matrix_sdk::SqliteCryptoStore">SqliteCryptoStore</a>  
An SQLite-based crypto store.

<a href="struct.SqliteEventCacheStore.html" class="struct"
title="struct matrix_sdk::SqliteEventCacheStore">SqliteEventCacheStore</a>  
An SQLite-based event cache store.

<a href="struct.SqliteMediaStore.html" class="struct"
title="struct matrix_sdk::SqliteMediaStore">SqliteMediaStore</a>  
An SQLite-based media store.

<a href="struct.SqliteStateStore.html" class="struct"
title="struct matrix_sdk::SqliteStateStore">SqliteStateStore</a>  
An SQLite-based state store.

<a href="struct.SqliteStoreConfig.html" class="struct"
title="struct matrix_sdk::SqliteStoreConfig">SqliteStoreConfig</a>  
A configuration structure used for opening a store.

<a href="struct.StateChanges.html" class="struct"
title="struct matrix_sdk::StateChanges">StateChanges</a>  
Store state changes and pass them to the StateStore.

<a href="struct.StoreSizes.html" class="struct"
title="struct matrix_sdk::StoreSizes">StoreSizes</a>  
Contains the disk size of the different stores, if known. It won’t be
available for in-memory stores.

<a href="struct.SuccessorRoom.html" class="struct"
title="struct matrix_sdk::SuccessorRoom">SuccessorRoom</a>  
When a room A is tombstoned, it is replaced by a room B. The room A is
the predecessor of B, and B is the successor of A. This type holds
information about the successor room. See
[`Room::successor_room`](struct.BaseRoom.html#method.successor_room "method matrix_sdk::BaseRoom::successor_room").

<a href="struct.TransmissionProgress.html" class="struct"
title="struct matrix_sdk::TransmissionProgress">TransmissionProgress</a>  
Progress of sending or receiving a payload.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ClientBuildError.html" class="enum"
title="enum matrix_sdk::ClientBuildError">ClientBuildError</a>  
Errors that can happen in
[`ClientBuilder::build`](struct.ClientBuilder.html#method.build "method matrix_sdk::ClientBuilder::build").

<a href="enum.ComposerDraftType.html" class="enum"
title="enum matrix_sdk::ComposerDraftType">ComposerDraftType</a>  
The type of draft of the composer.

<a href="enum.DraftAttachmentContent.html" class="enum"
title="enum matrix_sdk::DraftAttachmentContent">DraftAttachmentContent</a>  
The content of a draft attachment with type-specific data.

<a href="enum.EncryptionState.html" class="enum"
title="enum matrix_sdk::EncryptionState">EncryptionState</a>  
Represents the state of a room encryption.

<a href="enum.Error.html" class="enum"
title="enum matrix_sdk::Error">Error</a>  
Internal representation of errors.

<a href="enum.HttpError.html" class="enum"
title="enum matrix_sdk::HttpError">HttpError</a>  
An HTTP error, representing either a connection error or an error while
converting the raw HTTP response into a Matrix response.

<a href="enum.IdParseError.html" class="enum"
title="enum matrix_sdk::IdParseError">IdParseError</a>  
An error encountered when trying to parse an invalid ID string.

<a href="enum.LoopCtrl.html" class="enum"
title="enum matrix_sdk::LoopCtrl">LoopCtrl</a>  
Enum controlling if a loop running callbacks should continue or abort.

<a href="enum.NotificationSettingsError.html" class="enum"
title="enum matrix_sdk::NotificationSettingsError">NotificationSettingsError</a>  
Errors that can occur when manipulating push notification settings.

<a href="enum.QueueWedgeError.html" class="enum"
title="enum matrix_sdk::QueueWedgeError">QueueWedgeError</a>  
Represents a failed to send unrecoverable error of an event sent via the
send queue.

<a href="enum.RefreshTokenError.html" class="enum"
title="enum matrix_sdk::RefreshTokenError">RefreshTokenError</a>  
Errors that can happen when refreshing an access token.

<a href="enum.RoomDisplayName.html" class="enum"
title="enum matrix_sdk::RoomDisplayName">RoomDisplayName</a>  
The name of the room, either from the metadata or calculated according
to [matrix
specification](https://matrix.org/docs/spec/client_server/latest#calculating-the-display-name-for-a-room)

<a href="enum.RoomState.html" class="enum"
title="enum matrix_sdk::RoomState">RoomState</a>  
Enum keeping track in which state the room is, e.g. if our own user is
joined, RoomState::Invited, or has left the room.

<a href="enum.RumaApiError.html" class="enum"
title="enum matrix_sdk::RumaApiError">RumaApiError</a>  
An error response from a Matrix API call, using a client API specific
representation if the endpoint is from that.

<a href="enum.SessionChange.html" class="enum"
title="enum matrix_sdk::SessionChange">SessionChange</a>  
Represents changes that can occur to a `Client`s `Session`.

<a href="enum.StoreError.html" class="enum"
title="enum matrix_sdk::StoreError">StoreError</a>  
State store specific error type.

<a href="enum.ThreadingSupport.html" class="enum"
title="enum matrix_sdk::ThreadingSupport">ThreadingSupport</a>  
Whether this client instance supports threading or not. Currently used
to determine how the client handles read receipts and unread count
computations on the base SDK level.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.LEASE_DURATION_MS.html" class="constant"
title="constant matrix_sdk::LEASE_DURATION_MS">LEASE_DURATION_MS</a>  
Amount of time a lease of the lock should last, in milliseconds.

<a href="constant.ROOM_VERSION_FALLBACK.html" class="constant"
title="constant matrix_sdk::ROOM_VERSION_FALLBACK">ROOM_VERSION_FALLBACK</a>  
The room version to use as a fallback when the version of a room is
unknown.

<a href="constant.ROOM_VERSION_RULES_FALLBACK.html" class="constant"
title="constant matrix_sdk::ROOM_VERSION_RULES_FALLBACK">ROOM_VERSION_RULES_FALLBACK</a>  
The room version rules to use as a fallback when the version of a room
is unknown or unsupported.

<a href="constant.STATE_STORE_DATABASE_NAME.html" class="constant"
title="constant matrix_sdk::STATE_STORE_DATABASE_NAME">STATE_STORE_DATABASE_NAME</a>  
The filename used for the SQLITE database file used by the state store.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsyncTraitDeps.html" class="trait"
title="trait matrix_sdk::AsyncTraitDeps">AsyncTraitDeps</a>  
Super trait that is used for our store traits, this trait will differ if
it’s used on WASM. WASM targets will not require `Send` and `Sync` to
have implemented, while other targets will.

<a href="trait.SendOutsideWasm.html" class="trait"
title="trait matrix_sdk::SendOutsideWasm">SendOutsideWasm</a>  
Alias for `Send` on non-wasm, empty trait (implemented by everything) on
wasm.

<a href="trait.StateStore.html" class="trait"
title="trait matrix_sdk::StateStore">StateStore</a>  
An abstract state store trait that can be used to implement different
stores for the SDK.

<a href="trait.StateStoreExt.html" class="trait"
title="trait matrix_sdk::StateStoreExt">StateStoreExt</a>  
Convenience functionality for state stores.

<a href="trait.SyncOutsideWasm.html" class="trait"
title="trait matrix_sdk::SyncOutsideWasm">SyncOutsideWasm</a>  
Alias for `Sync` on non-wasm, empty trait (implemented by everything) on
wasm.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.sanitize_server_name.html" class="fn"
title="fn matrix_sdk::sanitize_server_name">sanitize_server_name</a>  
Creates a server name from a user supplied string. The string is first
sanitized by removing whitespace, the http(s) scheme and any trailing
slashes before being parsed.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BoxFuture.html" class="type"
title="type matrix_sdk::BoxFuture">BoxFuture</a>  
<a href="type.DynStateStore.html" class="type"
title="type matrix_sdk::DynStateStore">DynStateStore</a>  
A type-erased
[`StateStore`](trait.StateStore.html "trait matrix_sdk::StateStore").

<a href="type.HttpResult.html" class="type"
title="type matrix_sdk::HttpResult">HttpResult</a>  
Result type of a pure HTTP request.

<a href="type.Result.html" class="type"
title="type matrix_sdk::Result">Result</a>  
Result type of the matrix-sdk.

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.async_trait.html" class="attr"
title="attr matrix_sdk::async_trait">async_trait</a>
