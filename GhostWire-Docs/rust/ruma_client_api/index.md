Crate ruma\_client\_api Copy item path

<span class="sub-heading"><a href="../src/ruma_client_api/lib.rs.html#1-78" class="src">Source</a>
</span>

Expand description

(De)serializable types for the [Matrix Client-Server
API](https://spec.matrix.org/latest/client-server-api/). These types can
be shared by client and server code.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use error::`<a href="error/struct.Error.html" class="struct"
title="struct ruma_client_api::error::Error"><code>Error</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="account/index.html" class="mod"
title="mod ruma_client_api::account">account</a>  
Endpoints for account registration and management.

<a href="alias/index.html" class="mod"
title="mod ruma_client_api::alias">alias</a>  
Endpoints for room aliases.

<a href="appservice/index.html" class="mod"
title="mod ruma_client_api::appservice">appservice</a>  
Endpoints part of the application service extension of the client-server
API

<a href="authenticated_media/index.html" class="mod"
title="mod ruma_client_api::authenticated_media">authenticated_media</a>  
Authenticated endpoints for the [content
repository](https://spec.matrix.org/latest/client-server-api/#content-repository).

<a href="backup/index.html" class="mod"
title="mod ruma_client_api::backup">backup</a>  
Endpoints for server-side key backups.

<a href="config/index.html" class="mod"
title="mod ruma_client_api::config">config</a>  
Endpoints for client configuration.

<a href="context/index.html" class="mod"
title="mod ruma_client_api::context">context</a>  
Endpoints for event context.

<a href="dehydrated_device/index.html" class="mod"
title="mod ruma_client_api::dehydrated_device">dehydrated_device</a>  
Endpoints for managing dehydrated devices.

<a href="delayed_events/index.html" class="mod"
title="mod ruma_client_api::delayed_events">delayed_events</a>  
Endpoints for sending and interacting with delayed events.

<a href="device/index.html" class="mod"
title="mod ruma_client_api::device">device</a>  
Endpoints for managing devices.

<a href="directory/index.html" class="mod"
title="mod ruma_client_api::directory">directory</a>  
Endpoints for the public room directory.

<a href="discovery/index.html" class="mod"
title="mod ruma_client_api::discovery">discovery</a>  
Server discovery endpoints.

<a href="error/index.html" class="mod"
title="mod ruma_client_api::error">error</a>  
Errors that can be sent from the homeserver.

<a href="filter/index.html" class="mod"
title="mod ruma_client_api::filter">filter</a>  
Endpoints for event filters.

<a href="http_headers/index.html" class="mod"
title="mod ruma_client_api::http_headers">http_headers</a>  
Helpers for HTTP headers with the `http` crate.

<a href="keys/index.html" class="mod"
title="mod ruma_client_api::keys">keys</a>  
Endpoints for key management

<a href="knock/index.html" class="mod"
title="mod ruma_client_api::knock">knock</a>  
Endpoints to knock on a room.

<a href="media/index.html" class="mod"
title="mod ruma_client_api::media">media</a>  
Endpoints and helpers for the media repository.

<a href="membership/index.html" class="mod"
title="mod ruma_client_api::membership">membership</a>  
Endpoints for room membership.

<a href="message/index.html" class="mod"
title="mod ruma_client_api::message">message</a>  
Endpoints for sending and receiving messages

<a href="peeking/index.html" class="mod"
title="mod ruma_client_api::peeking">peeking</a>  
Endpoints for [previewing a
room](https://spec.matrix.org/latest/client-server-api/#room-previews)
without joining it, aka peeking.

<a href="presence/index.html" class="mod"
title="mod ruma_client_api::presence">presence</a>  
Endpoints for user presence.

<a href="profile/index.html" class="mod"
title="mod ruma_client_api::profile">profile</a>  
Endpoints for user profiles.

<a href="push/index.html" class="mod"
title="mod ruma_client_api::push">push</a>  
Endpoints for push notifications.

<a href="read_marker/index.html" class="mod"
title="mod ruma_client_api::read_marker">read_marker</a>  
Endpoints for “fully read markers”

<a href="receipt/index.html" class="mod"
title="mod ruma_client_api::receipt">receipt</a>  
Endpoints for event receipts.

<a href="redact/index.html" class="mod"
title="mod ruma_client_api::redact">redact</a>  
Endpoints for event redaction.

<a href="relations/index.html" class="mod"
title="mod ruma_client_api::relations">relations</a>  
Endpoints for querying relations

<a href="rendezvous/index.html" class="mod"
title="mod ruma_client_api::rendezvous">rendezvous</a>  
Endpoints for managing rendezvous sessions.

<a href="reporting/index.html" class="mod"
title="mod ruma_client_api::reporting">reporting</a>  
Endpoints for reporting content outside of rooms.

<a href="room/index.html" class="mod"
title="mod ruma_client_api::room">room</a>  
Endpoints for room management.

<a href="rtc/index.html" class="mod"
title="mod ruma_client_api::rtc">rtc</a>  
[MatrixRTC](https://github.com/matrix-org/matrix-spec-proposals/pull/4143)
endpoints.

<a href="search/index.html" class="mod"
title="mod ruma_client_api::search">search</a>  
Endpoints for event searches.

<a href="server/index.html" class="mod"
title="mod ruma_client_api::server">server</a>  
Endpoints for server administration.

<a href="session/index.html" class="mod"
title="mod ruma_client_api::session">session</a>  
Endpoints for user session management.

<a href="space/index.html" class="mod"
title="mod ruma_client_api::space">space</a>  
Endpoints for spaces.

<a href="state/index.html" class="mod"
title="mod ruma_client_api::state">state</a>  
Endpoints for managing room state

<a href="sync/index.html" class="mod"
title="mod ruma_client_api::sync">sync</a>  
Endpoints for getting and synchronizing events.

<a href="tag/index.html" class="mod"
title="mod ruma_client_api::tag">tag</a>  
Endpoints for tagging rooms.

<a href="thirdparty/index.html" class="mod"
title="mod ruma_client_api::thirdparty">thirdparty</a>  
Endpoints for third party lookups

<a href="threads/index.html" class="mod"
title="mod ruma_client_api::threads">threads</a>  
Endpoints for querying threads in a room.

<a href="to_device/index.html" class="mod"
title="mod ruma_client_api::to_device">to_device</a>  
Endpoints for client devices to exchange information not persisted in
room DAG.

<a href="typing/index.html" class="mod"
title="mod ruma_client_api::typing">typing</a>  
Endpoints for typing notifications.

<a href="uiaa/index.html" class="mod"
title="mod ruma_client_api::uiaa">uiaa</a>  
Module for [User-Interactive Authentication
API](https://spec.matrix.org/latest/client-server-api/#user-interactive-authentication-api)
types.

<a href="user_directory/index.html" class="mod"
title="mod ruma_client_api::user_directory">user_directory</a>  
Endpoints for the user directory.

<a href="voip/index.html" class="mod"
title="mod ruma_client_api::voip">voip</a>  
Endpoints for Voice over IP.
