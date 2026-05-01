Crate matrix\_sdk\_base Copy item path

<span class="sub-heading"><a href="../src/matrix_sdk_base/lib.rs.html#16-85"
class="src">Source</a> </span>

Expand description

This crate implements the base to build a [Matrix](https://matrix.org/)
client library.

## <a href="#crate-feature-flags" class="doc-anchor">§</a>Crate Feature Flags

The following crate feature flags are available:

-   `encryption`: Enables end-to-end encryption support in the library.
-   `qrcode`: Enables QRcode generation and reading code.
-   `testing`: Provides facilities and functions for tests, in
    particular for integration testing store implementations. ATTENTION:
    do not ever use outside of tests, we do not provide any stability
    warantees on these, these are merely helpers. If you find you *need*
    any function provided here outside of tests, please open a Github
    Issue and inform us about your use case for us to consider.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use store::`<a href="store/struct.ComposerDraft.html" class="struct"
title="struct matrix_sdk_base::store::ComposerDraft"><code>ComposerDraft</code></a>`;`

`pub use store::`<a href="store/enum.ComposerDraftType.html" class="enum"
title="enum matrix_sdk_base::store::ComposerDraftType"><code>ComposerDraftType</code></a>`;`

`pub use store::`<a href="store/struct.DraftAttachment.html" class="struct"
title="struct matrix_sdk_base::store::DraftAttachment"><code>DraftAttachment</code></a>`;`

`pub use store::`<a href="store/enum.DraftAttachmentContent.html" class="enum"
title="enum matrix_sdk_base::store::DraftAttachmentContent"><code>DraftAttachmentContent</code></a>`;`

`pub use store::`<a href="store/struct.DraftThumbnail.html" class="struct"
title="struct matrix_sdk_base::store::DraftThumbnail"><code>DraftThumbnail</code></a>`;`

`pub use store::`<a href="store/enum.QueueWedgeError.html" class="enum"
title="enum matrix_sdk_base::store::QueueWedgeError"><code>QueueWedgeError</code></a>`;`

`pub use store::`<a href="store/struct.StateChanges.html" class="struct"
title="struct matrix_sdk_base::store::StateChanges"><code>StateChanges</code></a>`;`

`pub use store::`<a href="store/trait.StateStore.html" class="trait"
title="trait matrix_sdk_base::store::StateStore"><code>StateStore</code></a>`;`

`pub use store::`<a href="store/enum.StateStoreDataKey.html" class="enum"
title="enum matrix_sdk_base::store::StateStoreDataKey"><code>StateStoreDataKey</code></a>`;`

`pub use store::`<a href="store/enum.StateStoreDataValue.html" class="enum"
title="enum matrix_sdk_base::store::StateStoreDataValue"><code>StateStoreDataValue</code></a>`;`

`pub use store::`<a href="store/enum.StoreError.html" class="enum"
title="enum matrix_sdk_base::store::StoreError"><code>StoreError</code></a>`;`

`pub use store::`<a href="store/struct.ThreadSubscriptionCatchupToken.html"
class="struct"
title="struct matrix_sdk_base::store::ThreadSubscriptionCatchupToken"><code>ThreadSubscriptionCatchupToken</code></a>`;`

`pub use `<a href="../matrix_sdk_crypto/index.html" class="mod"
title="mod matrix_sdk_crypto"><code>matrix_sdk_crypto</code></a>` as crypto;`

`pub use `<a href="../once_cell/index.html" class="mod"
title="mod once_cell"><code>once_cell</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="cross_process_lock/index.html" class="mod"
title="mod matrix_sdk_base::cross_process_lock">cross_process_lock</a>  
A cross-process lock implementation.

<a href="debug/index.html" class="mod"
title="mod matrix_sdk_base::debug">debug</a>  
Helpers for creating `std::fmt::Debug` implementations.

<a href="deserialized_responses/index.html" class="mod"
title="mod matrix_sdk_base::deserialized_responses">deserialized_responses</a>  
SDK-specific variations of response types from Ruma.

<a href="event_cache/index.html" class="mod"
title="mod matrix_sdk_base::event_cache">event_cache</a>  
Event cache store and common types shared with
`matrix_sdk::event_cache`.

<a href="executor/index.html" class="mod"
title="mod matrix_sdk_base::executor">executor</a>  
Abstraction over an executor so we can spawn tasks under Wasm the same
way we do usually.

<a href="failures_cache/index.html" class="mod"
title="mod matrix_sdk_base::failures_cache">failures_cache</a>  
A TTL cache which can be used to time out repeated operations that might
experience intermittent failures.

<a href="latest_event/index.html" class="mod"
title="mod matrix_sdk_base::latest_event">latest_event</a>  
Utilities for working with events to decide whether they are suitable
for use as a
[crate::Room::latest\_event](struct.Room.html#method.latest_event "method matrix_sdk_base::Room::latest_event").

<a href="linked_chunk/index.html" class="mod"
title="mod matrix_sdk_base::linked_chunk">linked_chunk</a>  
A linked chunk is the underlying data structure that holds all events.

<a href="locks/index.html" class="mod"
title="mod matrix_sdk_base::locks">locks</a>  
Simplified locks hat panic instead of returning a `Result` when the lock
is poisoned.

<a href="media/index.html" class="mod"
title="mod matrix_sdk_base::media">media</a>  
Media store and common types for [media
content](https://matrix.org/docs/spec/client_server/r0.6.1#id66).

<a href="notification_settings/index.html" class="mod"
title="mod matrix_sdk_base::notification_settings">notification_settings</a>  
Some shared types about notification settings.

<a href="read_receipts/index.html" class="mod"
title="mod matrix_sdk_base::read_receipts">read_receipts</a>  
Client-side read receipts computation

<a href="ring_buffer/index.html" class="mod"
title="mod matrix_sdk_base::ring_buffer">ring_buffer</a>  
<a href="ruma/index.html" class="mod"
title="mod matrix_sdk_base::ruma">ruma</a>  
Types and traits for working with the [Matrix](https://matrix.org)
protocol.

<a href="serde_helpers/index.html" class="mod"
title="mod matrix_sdk_base::serde_helpers">serde_helpers</a>  
A collection of serde helpers to avoid having to deserialize an entire
event to access some fields.

<a href="sleep/index.html" class="mod"
title="mod matrix_sdk_base::sleep">sleep</a>  
<a href="sliding_sync/index.html" class="mod"
title="mod matrix_sdk_base::sliding_sync">sliding_sync</a>  
Extend `BaseClient` with capabilities to handle MSC4186.

<a href="store/index.html" class="mod"
title="mod matrix_sdk_base::store">store</a>  
The state store holds the overall state for rooms, users and their
profiles and their timelines. It is an overall cache for faster access
and convenience- accessible through `Store`.

<a href="stream/index.html" class="mod"
title="mod matrix_sdk_base::stream">stream</a>  
Platform-specific stream utilities.

<a href="sync/index.html" class="mod"
title="mod matrix_sdk_base::sync">sync</a>  
The SDK’s representation of the result of a `/sync` request.

<a href="timeout/index.html" class="mod"
title="mod matrix_sdk_base::timeout">timeout</a>  
<a href="tracing_timer/index.html" class="mod"
title="mod matrix_sdk_base::tracing_timer">tracing_timer</a>  
<a href="ttl_cache/index.html" class="mod"
title="mod matrix_sdk_base::ttl_cache">ttl_cache</a>  
A TTL cache which can be used to time out repeated operations that might
experience intermittent failures.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.boxed_into_future.html" class="macro"
title="macro matrix_sdk_base::boxed_into_future">boxed_into_future</a>  
<a href="macro.timer.html" class="macro"
title="macro matrix_sdk_base::timer">timer</a>  
Macro to create a RAII timer that will log on `Drop` how long its
covered section took to execute.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BaseClient.html" class="struct"
title="struct matrix_sdk_base::BaseClient">BaseClient</a>  
A no (network) IO client implementation.

<a href="struct.InviteAcceptanceDetails.html" class="struct"
title="struct matrix_sdk_base::InviteAcceptanceDetails">InviteAcceptanceDetails</a>  
A struct remembering details of an invite and if the invite has been
accepted on this particular client.

<a href="struct.OriginalMinimalStateEvent.html" class="struct"
title="struct matrix_sdk_base::OriginalMinimalStateEvent">OriginalMinimalStateEvent</a>  
An unredacted minimal state event.

<a href="struct.PredecessorRoom.html" class="struct"
title="struct matrix_sdk_base::PredecessorRoom">PredecessorRoom</a>  
When a room A is tombstoned, it is replaced by a room B. The room A is
the predecessor of B, and B is the successor of A. This type holds
information about the predecessor room. See
[`Room::predecessor_room`](struct.Room.html#method.predecessor_room "method matrix_sdk_base::Room::predecessor_room").

<a href="struct.RedactedMinimalStateEvent.html" class="struct"
title="struct matrix_sdk_base::RedactedMinimalStateEvent">RedactedMinimalStateEvent</a>  
A redacted minimal state event.

<a href="struct.RequestedRequiredStates.html" class="struct"
title="struct matrix_sdk_base::RequestedRequiredStates">RequestedRequiredStates</a>  
Represent the `required_state` values sent by a sync request.

<a href="struct.Room.html" class="struct"
title="struct matrix_sdk_base::Room">Room</a>  
The underlying room data structure collecting state for joined, left and
invited rooms.

<a href="struct.RoomCreateWithCreatorEventContent.html" class="struct"
title="struct matrix_sdk_base::RoomCreateWithCreatorEventContent">RoomCreateWithCreatorEventContent</a>  
The content of an `m.room.create` event, with a required `creator`
field.

<a href="struct.RoomHero.html" class="struct"
title="struct matrix_sdk_base::RoomHero">RoomHero</a>  
Information about a member considered to be a room hero.

<a href="struct.RoomInfo.html" class="struct"
title="struct matrix_sdk_base::RoomInfo">RoomInfo</a>  
The underlying pure data structure for joined and left rooms.

<a href="struct.RoomInfoNotableUpdate.html" class="struct"
title="struct matrix_sdk_base::RoomInfoNotableUpdate">RoomInfoNotableUpdate</a>  
Indicates that a notable update of `RoomInfo` has been applied, and why.

<a href="struct.RoomInfoNotableUpdateReasons.html" class="struct"
title="struct matrix_sdk_base::RoomInfoNotableUpdateReasons">RoomInfoNotableUpdateReasons</a>  
The reason why a
[`RoomInfoNotableUpdate`](struct.RoomInfoNotableUpdate.html "struct matrix_sdk_base::RoomInfoNotableUpdate")
is emitted.

<a href="struct.RoomMember.html" class="struct"
title="struct matrix_sdk_base::RoomMember">RoomMember</a>  
A member of a room.

<a href="struct.RoomMemberships.html" class="struct"
title="struct matrix_sdk_base::RoomMemberships">RoomMemberships</a>  
Room membership filter as a bitset.

<a href="struct.RoomRecencyStamp.html" class="struct"
title="struct matrix_sdk_base::RoomRecencyStamp">RoomRecencyStamp</a>  
Type to represent a `RoomInfo::recency_stamp`.

<a href="struct.RoomStateFilter.html" class="struct"
title="struct matrix_sdk_base::RoomStateFilter">RoomStateFilter</a>  
Room state filter as a bitset.

<a href="struct.SessionMeta.html" class="struct"
title="struct matrix_sdk_base::SessionMeta">SessionMeta</a>  
The Matrix user session info.

<a href="struct.SuccessorRoom.html" class="struct"
title="struct matrix_sdk_base::SuccessorRoom">SuccessorRoom</a>  
When a room A is tombstoned, it is replaced by a room B. The room A is
the predecessor of B, and B is the successor of A. This type holds
information about the successor room. See
[`Room::successor_room`](struct.Room.html#method.successor_room "method matrix_sdk_base::Room::successor_room").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.EncryptionState.html" class="enum"
title="enum matrix_sdk_base::EncryptionState">EncryptionState</a>  
Represents the state of a room encryption.

<a href="enum.Error.html" class="enum"
title="enum matrix_sdk_base::Error">Error</a>  
Internal representation of errors.

<a href="enum.MinimalStateEvent.html" class="enum"
title="enum matrix_sdk_base::MinimalStateEvent">MinimalStateEvent</a>  
A minimal state event.

<a href="enum.RoomDisplayName.html" class="enum"
title="enum matrix_sdk_base::RoomDisplayName">RoomDisplayName</a>  
The name of the room, either from the metadata or calculated according
to [matrix
specification](https://matrix.org/docs/spec/client_server/latest#calculating-the-display-name-for-a-room)

<a href="enum.RoomMembersUpdate.html" class="enum"
title="enum matrix_sdk_base::RoomMembersUpdate">RoomMembersUpdate</a>  
The kind of room member updates that just happened.

<a href="enum.RoomState.html" class="enum"
title="enum matrix_sdk_base::RoomState">RoomState</a>  
Enum keeping track in which state the room is, e.g. if our own user is
joined, RoomState::Invited, or has left the room.

<a href="enum.ThreadingSupport.html" class="enum"
title="enum matrix_sdk_base::ThreadingSupport">ThreadingSupport</a>  
Whether this client instance supports threading or not. Currently used
to determine how the client handles read receipts and unread count
computations on the base SDK level.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.LEASE_DURATION_MS.html" class="constant"
title="constant matrix_sdk_base::LEASE_DURATION_MS">LEASE_DURATION_MS</a>  
Amount of time a lease of the lock should last, in milliseconds.

<a href="constant.ROOM_VERSION_FALLBACK.html" class="constant"
title="constant matrix_sdk_base::ROOM_VERSION_FALLBACK">ROOM_VERSION_FALLBACK</a>  
The room version to use as a fallback when the version of a room is
unknown.

<a href="constant.ROOM_VERSION_RULES_FALLBACK.html" class="constant"
title="constant matrix_sdk_base::ROOM_VERSION_RULES_FALLBACK">ROOM_VERSION_RULES_FALLBACK</a>  
The room version rules to use as a fallback when the version of a room
is unknown or unsupported.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsyncTraitDeps.html" class="trait"
title="trait matrix_sdk_base::AsyncTraitDeps">AsyncTraitDeps</a>  
Super trait that is used for our store traits, this trait will differ if
it’s used on WASM. WASM targets will not require `Send` and `Sync` to
have implemented, while other targets will.

<a href="trait.SendOutsideWasm.html" class="trait"
title="trait matrix_sdk_base::SendOutsideWasm">SendOutsideWasm</a>  
Alias for `Send` on non-wasm, empty trait (implemented by everything) on
wasm.

<a href="trait.SyncOutsideWasm.html" class="trait"
title="trait matrix_sdk_base::SyncOutsideWasm">SyncOutsideWasm</a>  
Alias for `Sync` on non-wasm, empty trait (implemented by everything) on
wasm.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.apply_redaction.html" class="fn"
title="fn matrix_sdk_base::apply_redaction">apply_redaction</a>  
Apply a redaction to the given target `event`, given the raw redaction
event and the room version.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BoxFuture.html" class="type"
title="type matrix_sdk_base::BoxFuture">BoxFuture</a>  
<a href="type.MinimalRoomMemberEvent.html" class="type"
title="type matrix_sdk_base::MinimalRoomMemberEvent">MinimalRoomMemberEvent</a>  
A minimal `m.room.member` event.

<a href="type.Result.html" class="type"
title="type matrix_sdk_base::Result">Result</a>  
Result type of the rust-sdk.
