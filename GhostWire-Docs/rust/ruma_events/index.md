Crate ruma\_events Copy item path

<span class="sub-heading"><a href="../src/ruma_events/lib.rs.html#1-313" class="src">Source</a>
</span>

Expand description

(De)serializable types for the events in the
[Matrix](https://matrix.org) specification. These types are used by
other Ruma crates.

All data exchanged over Matrix is expressed as an event. Different event
types represent different actions, such as joining a room or sending a
message. Events are stored and transmitted as simple JSON structures.
While anyone can create a new event type for their own purposes, the
Matrix specification defines a number of event types which are
considered core to the protocol. This module contains Rust types for all
of the event types defined by the specification and facilities for
extending the event system for custom event types.

# <a href="#core-event-types" class="doc-anchor">§</a>Core event types

This module includes Rust types for all event types in the Matrix
specification. To better organize the crate, these types live in
separate modules with a hierarchy that matches the reverse domain name
notation of the event type. For example, the `m.room.message` event
lives at `ruma::events::room::message::RoomMessageEvent`. Each type’s
module also contains a Rust type for that event type’s `content` field,
and any other supporting types required by the event’s other fields.

# <a href="#extending-ruma-with-custom-events" class="doc-anchor">§</a>Extending Ruma with custom events

For our examples we will start with a simple custom state event.
`ruma_event` specifies the state event’s `type` and its `kind`.

``` {.rust .rust-example-rendered}
use ruma_events::macros::EventContent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "org.example.event", kind = State, state_key_type = String)]
pub struct ExampleContent {
    field: String,
}
```

This can be used with events structs, such as passing it into
`ruma::api::client::state::send_state_event`’s `Request`.

As a more advanced example we create a reaction message event. For this
event we will use a
[`OriginalSyncMessageLikeEvent`](struct.OriginalSyncMessageLikeEvent.html "struct ruma_events::OriginalSyncMessageLikeEvent")
struct but any
[`OriginalMessageLikeEvent`](struct.OriginalMessageLikeEvent.html "struct ruma_events::OriginalMessageLikeEvent")
struct would work.

``` {.rust .rust-example-rendered}
use ruma_common::OwnedEventId;
use ruma_events::{macros::EventContent, OriginalSyncMessageLikeEvent};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "rel_type")]
pub enum RelatesTo {
    #[serde(rename = "m.annotation")]
    Annotation {
        /// The event this reaction relates to.
        event_id: OwnedEventId,
        /// The displayable content of the reaction.
        key: String,
    },

    /// Since this event is not fully specified in the Matrix spec
    /// it may change or types may be added, we are ready!
    #[serde(rename = "m.whatever")]
    Whatever,
}

/// The payload for our reaction event.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "m.reaction", kind = MessageLike)]
pub struct ReactionEventContent {
    #[serde(rename = "m.relates_to")]
    pub relates_to: RelatesTo,
}

let json = serde_json::json!({
    "content": {
        "m.relates_to": {
            "event_id": "$xxxx-xxxx",
            "key": "👍",
            "rel_type": "m.annotation"
        }
    },
    "event_id": "$xxxx-xxxx",
    "origin_server_ts": 1,
    "sender": "@someone:example.org",
    "type": "m.reaction",
    "unsigned": {
        "age": 85
    }
});

// The downside of this event is we cannot use it with event enums,
// but could be deserialized from a `Raw<_>` that has failed to deserialize.
assert!(matches!(
    serde_json::from_value::<OriginalSyncMessageLikeEvent<ReactionEventContent>>(json),
    Ok(OriginalSyncMessageLikeEvent {
        content: ReactionEventContent {
            relates_to: RelatesTo::Annotation { key, .. },
        },
        ..
    }) if key == "👍"
));
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::relation::`<a href="relation/struct.BundledMessageLikeRelations.html"
class="struct"
title="struct ruma_events::relation::BundledMessageLikeRelations"><code>BundledMessageLikeRelations</code></a>`;`

`pub use self::relation::`<a href="relation/struct.BundledStateRelations.html" class="struct"
title="struct ruma_events::relation::BundledStateRelations"><code>BundledStateRelations</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="beacon/index.html" class="mod"
title="mod ruma_events::beacon">beacon</a>  
Types for the `org.matrix.msc3489.beacon` event, the unstable version of
`m.beacon`
([MSC3489](https://github.com/matrix-org/matrix-spec-proposals/pull/3489)).

<a href="beacon_info/index.html" class="mod"
title="mod ruma_events::beacon_info">beacon_info</a>  
Types for the `org.matrix.msc3489.beacon_info` state event, the unstable
version of `m.beacon_info`
([MSC3489](https://github.com/matrix-org/matrix-spec-proposals/pull/3489)).

<a href="call/index.html" class="mod"
title="mod ruma_events::call">call</a>  
Modules for events in the `m.call` namespace.

<a href="direct/index.html" class="mod"
title="mod ruma_events::direct">direct</a>  
Types for the
[`m.direct`](https://spec.matrix.org/latest/client-server-api/#mdirect)
event.

<a href="dummy/index.html" class="mod"
title="mod ruma_events::dummy">dummy</a>  
Types for the
[`m.dummy`](https://spec.matrix.org/latest/client-server-api/#mdummy)
event.

<a href="forwarded_room_key/index.html" class="mod"
title="mod ruma_events::forwarded_room_key">forwarded_room_key</a>  
Types for the
[`m.forwarded_room_key`](https://spec.matrix.org/latest/client-server-api/#mforwarded_room_key)
event.

<a href="fully_read/index.html" class="mod"
title="mod ruma_events::fully_read">fully_read</a>  
Types for the
[`m.fully_read`](https://spec.matrix.org/latest/client-server-api/#mfully_read)
event.

<a href="identity_server/index.html" class="mod"
title="mod ruma_events::identity_server">identity_server</a>  
Types for the
[`m.identity_server`](https://spec.matrix.org/latest/client-server-api/#midentity_server)
event.

<a href="ignored_user_list/index.html" class="mod"
title="mod ruma_events::ignored_user_list">ignored_user_list</a>  
Types for the
[`m.ignored_user_list`](https://spec.matrix.org/latest/client-server-api/#mignored_user_list)
event.

<a href="key/index.html" class="mod"
title="mod ruma_events::key">key</a>  
Modules for events in the `m.key` namespace.

<a href="location/index.html" class="mod"
title="mod ruma_events::location">location</a>  
Types for extensible location message events
([MSC3488](https://github.com/matrix-org/matrix-spec-proposals/pull/3488)).

<a href="macros/index.html" class="mod"
title="mod ruma_events::macros">macros</a>  
Re-export of all the derives needed to create your own event types.

<a href="marked_unread/index.html" class="mod"
title="mod ruma_events::marked_unread">marked_unread</a>  
Types for the
[`m.marked_unread`](https://spec.matrix.org/latest/client-server-api/#unread-markers)
event.

<a href="media_preview_config/index.html" class="mod"
title="mod ruma_events::media_preview_config">media_preview_config</a>  
Types for the
[`m.media_preview_config`](https://github.com/matrix-org/matrix-spec-proposals/pull/4278)
event.

<a href="member_hints/index.html" class="mod"
title="mod ruma_events::member_hints">member_hints</a>  
Types for Matrix member hint state events
([MSC4171](https://github.com/matrix-org/matrix-spec-proposals/pull/4171)).

<a href="message/index.html" class="mod"
title="mod ruma_events::message">message</a>  
Types for extensible text message events
([MSC1767](https://github.com/matrix-org/matrix-spec-proposals/pull/1767)).

<a href="policy/index.html" class="mod"
title="mod ruma_events::policy">policy</a>  
Modules for events in the `m.policy` namespace.

<a href="poll/index.html" class="mod"
title="mod ruma_events::poll">poll</a>  
Modules for events in the `m.poll` namespace
([MSC3381](https://github.com/matrix-org/matrix-spec-proposals/pull/3381)).

<a href="presence/index.html" class="mod"
title="mod ruma_events::presence">presence</a>  
A presence event is represented by a struct with a set content field.

<a href="push_rules/index.html" class="mod"
title="mod ruma_events::push_rules">push_rules</a>  
Types for the
[`m.push_rules`](https://spec.matrix.org/latest/client-server-api/#mpush_rules)
event.

<a href="reaction/index.html" class="mod"
title="mod ruma_events::reaction">reaction</a>  
Types for the
[`m.reaction`](https://spec.matrix.org/latest/client-server-api/#mreaction)
event.

<a href="receipt/index.html" class="mod"
title="mod ruma_events::receipt">receipt</a>  
Types for the
[`m.receipt`](https://spec.matrix.org/latest/client-server-api/#mreceipt)
event.

<a href="relation/index.html" class="mod"
title="mod ruma_events::relation">relation</a>  
Types describing [relationships between
events](https://spec.matrix.org/latest/client-server-api/#forming-relationships-between-events).

<a href="room/index.html" class="mod"
title="mod ruma_events::room">room</a>  
Modules for events in the `m.room` namespace.

<a href="room_key/index.html" class="mod"
title="mod ruma_events::room_key">room_key</a>  
Types for the
[`m.room_key`](https://spec.matrix.org/latest/client-server-api/#mroom_key)
event.

<a href="room_key_request/index.html" class="mod"
title="mod ruma_events::room_key_request">room_key_request</a>  
Types for the
[`m.room_key_request`](https://spec.matrix.org/latest/client-server-api/#mroom_key_request)
event.

<a href="rtc/index.html" class="mod"
title="mod ruma_events::rtc">rtc</a>  
Modules for events in the `m.rtc` namespace.

<a href="secret/index.html" class="mod"
title="mod ruma_events::secret">secret</a>  
Module for events in the `m.secret` namespace.

<a href="secret_storage/index.html" class="mod"
title="mod ruma_events::secret_storage">secret_storage</a>  
Module for events in the `m.secret_storage` namespace.

<a href="space/index.html" class="mod"
title="mod ruma_events::space">space</a>  
Types for the `m.space` events.

<a href="space_order/index.html" class="mod"
title="mod ruma_events::space_order">space_order</a>  
Types for the
[`m.space_order`](https://github.com/matrix-org/matrix-spec-proposals/pull/3230)
event.

<a href="sticker/index.html" class="mod"
title="mod ruma_events::sticker">sticker</a>  
Types for the
[`m.sticker`](https://spec.matrix.org/latest/client-server-api/#msticker)
event.

<a href="tag/index.html" class="mod"
title="mod ruma_events::tag">tag</a>  
Types for the
[`m.tag`](https://spec.matrix.org/latest/client-server-api/#mtag) event.

<a href="typing/index.html" class="mod"
title="mod ruma_events::typing">typing</a>  
Types for the
[`m.typing`](https://spec.matrix.org/latest/client-server-api/#mtyping)
event.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DecryptedMegolmV1Event.html" class="struct"
title="struct ruma_events::DecryptedMegolmV1Event">DecryptedMegolmV1Event</a>  
The decrypted payload of an `m.megolm.v1.aes-sha2` event.

<a href="struct.DecryptedOlmV1Event.html" class="struct"
title="struct ruma_events::DecryptedOlmV1Event">DecryptedOlmV1Event</a>  
The decrypted payload of an `m.olm.v1.curve25519-aes-sha2` event.

<a href="struct.EmptyStateKey.html" class="struct"
title="struct ruma_events::EmptyStateKey">EmptyStateKey</a>  
A type that can be used as the `state_key` for event types where that
field is always empty.

<a href="struct.EphemeralRoomEvent.html" class="struct"
title="struct ruma_events::EphemeralRoomEvent">EphemeralRoomEvent</a>  
An ephemeral room event.

<a href="struct.False.html" class="struct"
title="struct ruma_events::False">False</a>  
The equivalent of the `false` boolean.

<a href="struct.GlobalAccountDataEvent.html" class="struct"
title="struct ruma_events::GlobalAccountDataEvent">GlobalAccountDataEvent</a>  
A global account data event.

<a href="struct.InitialStateEvent.html" class="struct"
title="struct ruma_events::InitialStateEvent">InitialStateEvent</a>  
A minimal state event, used for creating a new room.

<a href="struct.Mentions.html" class="struct"
title="struct ruma_events::Mentions">Mentions</a>  
Describes whether the event mentions other users or the room.

<a href="struct.MessageLikeUnsigned.html" class="struct"
title="struct ruma_events::MessageLikeUnsigned">MessageLikeUnsigned</a>  
Extra information about a message event that is not incorporated into
the event’s hash.

<a href="struct.OlmV1Keys.html" class="struct"
title="struct ruma_events::OlmV1Keys">OlmV1Keys</a>  
Public keys used for an `m.olm.v1.curve25519-aes-sha2` event.

<a href="struct.OriginalMessageLikeEvent.html" class="struct"
title="struct ruma_events::OriginalMessageLikeEvent">OriginalMessageLikeEvent</a>  
An unredacted message-like event.

<a href="struct.OriginalStateEvent.html" class="struct"
title="struct ruma_events::OriginalStateEvent">OriginalStateEvent</a>  
An unredacted state event.

<a href="struct.OriginalSyncMessageLikeEvent.html" class="struct"
title="struct ruma_events::OriginalSyncMessageLikeEvent">OriginalSyncMessageLikeEvent</a>  
An unredacted message-like event without a `room_id`.

<a href="struct.OriginalSyncStateEvent.html" class="struct"
title="struct ruma_events::OriginalSyncStateEvent">OriginalSyncStateEvent</a>  
An unredacted state event without a `room_id`.

<a href="struct.RedactedMessageLikeEvent.html" class="struct"
title="struct ruma_events::RedactedMessageLikeEvent">RedactedMessageLikeEvent</a>  
A redacted message-like event.

<a href="struct.RedactedStateEvent.html" class="struct"
title="struct ruma_events::RedactedStateEvent">RedactedStateEvent</a>  
A redacted state event.

<a href="struct.RedactedSyncMessageLikeEvent.html" class="struct"
title="struct ruma_events::RedactedSyncMessageLikeEvent">RedactedSyncMessageLikeEvent</a>  
A redacted message-like event without a `room_id`.

<a href="struct.RedactedSyncStateEvent.html" class="struct"
title="struct ruma_events::RedactedSyncStateEvent">RedactedSyncStateEvent</a>  
A redacted state event without a `room_id`.

<a href="struct.RedactedUnsigned.html" class="struct"
title="struct ruma_events::RedactedUnsigned">RedactedUnsigned</a>  
Extra information about a redacted event that is not incorporated into
the event’s hash.

<a href="struct.RoomAccountDataEvent.html" class="struct"
title="struct ruma_events::RoomAccountDataEvent">RoomAccountDataEvent</a>  
A room account data event.

<a href="struct.StateUnsigned.html" class="struct"
title="struct ruma_events::StateUnsigned">StateUnsigned</a>  
Extra information about a state event that is not incorporated into the
event’s hash.

<a href="struct.StrippedStateEvent.html" class="struct"
title="struct ruma_events::StrippedStateEvent">StrippedStateEvent</a>  
A stripped-down state event, used for previews of rooms the user has
been invited to.

<a href="struct.SyncEphemeralRoomEvent.html" class="struct"
title="struct ruma_events::SyncEphemeralRoomEvent">SyncEphemeralRoomEvent</a>  
An ephemeral room event without a `room_id`.

<a href="struct.ToDeviceEvent.html" class="struct"
title="struct ruma_events::ToDeviceEvent">ToDeviceEvent</a>  
An event sent using send-to-device messaging.

<a href="struct.True.html" class="struct"
title="struct ruma_events::True">True</a>  
The equivalent of the `true` boolean.

<a href="struct.UnsignedRoomRedactionEvent.html" class="struct"
title="struct ruma_events::UnsignedRoomRedactionEvent">UnsignedRoomRedactionEvent</a>  
A redaction event as found in `unsigned.redacted_because`.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AnyEphemeralRoomEventContent.html" class="enum"
title="enum ruma_events::AnyEphemeralRoomEventContent">AnyEphemeralRoomEventContent</a>  
Any ephemeral room event.

<a href="enum.AnyFullStateEventContent.html" class="enum"
title="enum ruma_events::AnyFullStateEventContent">AnyFullStateEventContent</a>  
Any state event.

<a href="enum.AnyGlobalAccountDataEvent.html" class="enum"
title="enum ruma_events::AnyGlobalAccountDataEvent">AnyGlobalAccountDataEvent</a>  
Any global account data event.

<a href="enum.AnyGlobalAccountDataEventContent.html" class="enum"
title="enum ruma_events::AnyGlobalAccountDataEventContent">AnyGlobalAccountDataEventContent</a>  
Any global account data event.

<a href="enum.AnyInitialStateEvent.html" class="enum"
title="enum ruma_events::AnyInitialStateEvent">AnyInitialStateEvent</a>  
Any state event.

<a href="enum.AnyMessageLikeEvent.html" class="enum"
title="enum ruma_events::AnyMessageLikeEvent">AnyMessageLikeEvent</a>  
Any message-like event.

<a href="enum.AnyMessageLikeEventContent.html" class="enum"
title="enum ruma_events::AnyMessageLikeEventContent">AnyMessageLikeEventContent</a>  
Any message-like event.

<a href="enum.AnyRoomAccountDataEvent.html" class="enum"
title="enum ruma_events::AnyRoomAccountDataEvent">AnyRoomAccountDataEvent</a>  
Any room account data event.

<a href="enum.AnyRoomAccountDataEventContent.html" class="enum"
title="enum ruma_events::AnyRoomAccountDataEventContent">AnyRoomAccountDataEventContent</a>  
Any room account data event.

<a href="enum.AnyStateEvent.html" class="enum"
title="enum ruma_events::AnyStateEvent">AnyStateEvent</a>  
Any state event.

<a href="enum.AnyStateEventContent.html" class="enum"
title="enum ruma_events::AnyStateEventContent">AnyStateEventContent</a>  
Any state event.

<a href="enum.AnyStrippedStateEvent.html" class="enum"
title="enum ruma_events::AnyStrippedStateEvent">AnyStrippedStateEvent</a>  
Any state event.

<a href="enum.AnySyncEphemeralRoomEvent.html" class="enum"
title="enum ruma_events::AnySyncEphemeralRoomEvent">AnySyncEphemeralRoomEvent</a>  
Any ephemeral room event.

<a href="enum.AnySyncMessageLikeEvent.html" class="enum"
title="enum ruma_events::AnySyncMessageLikeEvent">AnySyncMessageLikeEvent</a>  
Any message-like event.

<a href="enum.AnySyncStateEvent.html" class="enum"
title="enum ruma_events::AnySyncStateEvent">AnySyncStateEvent</a>  
Any state event.

<a href="enum.AnySyncTimelineEvent.html" class="enum"
title="enum ruma_events::AnySyncTimelineEvent">AnySyncTimelineEvent</a>  
Any sync room event.

<a href="enum.AnyTimelineEvent.html" class="enum"
title="enum ruma_events::AnyTimelineEvent">AnyTimelineEvent</a>  
Any room event.

<a href="enum.AnyToDeviceEvent.html" class="enum"
title="enum ruma_events::AnyToDeviceEvent">AnyToDeviceEvent</a>  
Any to-device event.

<a href="enum.AnyToDeviceEventContent.html" class="enum"
title="enum ruma_events::AnyToDeviceEventContent">AnyToDeviceEventContent</a>  
Any to-device event.

<a href="enum.EphemeralRoomEventType.html" class="enum"
title="enum ruma_events::EphemeralRoomEventType">EphemeralRoomEventType</a>  
The type of `EphemeralRoomEvent` this is.

<a href="enum.FullStateEventContent.html" class="enum"
title="enum ruma_events::FullStateEventContent">FullStateEventContent</a>  
A possibly-redacted state event content.

<a href="enum.GlobalAccountDataEventType.html" class="enum"
title="enum ruma_events::GlobalAccountDataEventType">GlobalAccountDataEventType</a>  
The type of `GlobalAccountDataEvent` this is.

<a href="enum.MessageLikeEvent.html" class="enum"
title="enum ruma_events::MessageLikeEvent">MessageLikeEvent</a>  
A possibly-redacted message-like event.

<a href="enum.MessageLikeEventType.html" class="enum"
title="enum ruma_events::MessageLikeEventType">MessageLikeEventType</a>  
The type of `MessageLikeEvent` this is.

<a href="enum.RoomAccountDataEventType.html" class="enum"
title="enum ruma_events::RoomAccountDataEventType">RoomAccountDataEventType</a>  
The type of `RoomAccountDataEvent` this is.

<a href="enum.StateEvent.html" class="enum"
title="enum ruma_events::StateEvent">StateEvent</a>  
A possibly-redacted state event.

<a href="enum.StateEventType.html" class="enum"
title="enum ruma_events::StateEventType">StateEventType</a>  
The type of `StateEvent` this is.

<a href="enum.SyncMessageLikeEvent.html" class="enum"
title="enum ruma_events::SyncMessageLikeEvent">SyncMessageLikeEvent</a>  
A possibly-redacted message-like event without a `room_id`.

<a href="enum.SyncStateEvent.html" class="enum"
title="enum ruma_events::SyncStateEvent">SyncStateEvent</a>  
A possibly-redacted state event without a `room_id`.

<a href="enum.TimelineEventType.html" class="enum"
title="enum ruma_events::TimelineEventType">TimelineEventType</a>  
The type of `TimelineEvent` this is.

<a href="enum.ToDeviceEventType.html" class="enum"
title="enum ruma_events::ToDeviceEventType">ToDeviceEventType</a>  
The type of `ToDeviceEvent` this is.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.RECOMMENDED_STRIPPED_STATE_EVENT_TYPES.html"
class="constant"
title="constant ruma_events::RECOMMENDED_STRIPPED_STATE_EVENT_TYPES">RECOMMENDED_STRIPPED_STATE_EVENT_TYPES</a>  
Event types that servers should send as [stripped
state](https://spec.matrix.org/latest/client-server-api/#stripped-state)
to help clients identify a room when they can’t access the full room
state.

<a href="constant.RECOMMENDED_TRANSFERABLE_STATE_EVENT_TYPES.html"
class="constant"
title="constant ruma_events::RECOMMENDED_TRANSFERABLE_STATE_EVENT_TYPES">RECOMMENDED_TRANSFERABLE_STATE_EVENT_TYPES</a>  
Event types that servers should transfer upon [room
upgrade](https://spec.matrix.org/v1.17/client-server-api/#server-behaviour-19).
The exact details for what is transferred is left as an implementation
detail.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.BooleanType.html" class="trait"
title="trait ruma_events::BooleanType">BooleanType</a>  
A trait for types representing a boolean value.

<a href="trait.EphemeralRoomEventContent.html" class="trait"
title="trait ruma_events::EphemeralRoomEventContent">EphemeralRoomEventContent</a>  
Content of an ephemeral room event.

<a href="trait.EventContentFromType.html" class="trait"
title="trait ruma_events::EventContentFromType">EventContentFromType</a>  
Event content that can be deserialized with its event type.

<a href="trait.GlobalAccountDataEventContent.html" class="trait"
title="trait ruma_events::GlobalAccountDataEventContent">GlobalAccountDataEventContent</a>  
Content of a global account-data event.

<a href="trait.MessageLikeEventContent.html" class="trait"
title="trait ruma_events::MessageLikeEventContent">MessageLikeEventContent</a>  
Content of a non-redacted message-like event.

<a href="trait.PossiblyRedactedStateEventContent.html" class="trait"
title="trait ruma_events::PossiblyRedactedStateEventContent">PossiblyRedactedStateEventContent</a>  
Content of a state event.

<a href="trait.RawExt.html" class="trait"
title="trait ruma_events::RawExt">RawExt</a>  
Extension trait for
[`Raw<T>`](../ruma_common/serde/raw/struct.Raw.html "struct ruma_common::serde::raw::Raw").

<a href="trait.RedactContent.html" class="trait"
title="trait ruma_events::RedactContent">RedactContent</a>  
Trait to define the behavior of redact an event’s content object.

<a href="trait.RedactedMessageLikeEventContent.html" class="trait"
title="trait ruma_events::RedactedMessageLikeEventContent">RedactedMessageLikeEventContent</a>  
Content of a redacted message-like event.

<a href="trait.RedactedStateEventContent.html" class="trait"
title="trait ruma_events::RedactedStateEventContent">RedactedStateEventContent</a>  
Content of a redacted state event.

<a href="trait.RoomAccountDataEventContent.html" class="trait"
title="trait ruma_events::RoomAccountDataEventContent">RoomAccountDataEventContent</a>  
Content of a room-specific account-data event.

<a href="trait.StateEventContent.html" class="trait"
title="trait ruma_events::StateEventContent">StateEventContent</a>  
Content of a non-redacted state event.

<a href="trait.StaticEventContent.html" class="trait"
title="trait ruma_events::StaticEventContent">StaticEventContent</a>  
An event content type with a statically-known event `type` value.

<a href="trait.StaticStateEventContent.html" class="trait"
title="trait ruma_events::StaticStateEventContent">StaticStateEventContent</a>  
Content of a non-redacted state event with a corresponding
possibly-redacted type.

<a href="trait.ToDeviceEventContent.html" class="trait"
title="trait ruma_events::ToDeviceEventContent">ToDeviceEventContent</a>  
Content of a to-device event.
