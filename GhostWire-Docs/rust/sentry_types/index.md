Crate sentry\_types Copy item path

<span class="sub-heading"><a href="../src/sentry_types/lib.rs.html#1-60" class="src">Source</a>
</span>

Expand description

This crate provides common types for working with the Sentry protocol or
the Sentry server. It’s used by the Sentry Relay infrastructure as well
as the rust Sentry client.

Most of the types in this crate are serializable in one form or another.
The types in the `protocol` module are generally really only
serializable to JSON as other formats are not supported by Sentry at
this date.

## <a href="#contents" class="doc-anchor">§</a>Contents

The crate provides a bunch of common types for working with Sentry as
such (DSN, ProjectIDs, authentication headers) as well as types for the
Sentry event protocol.

Right now only `v7` of the protocol is implemented but it’s versioned so
later versions might be added later.

## <a href="#api-concepts" class="doc-anchor">§</a>API Concepts

Most types are directly serializable or deserializable and try to
implement the `Default` type. This means that objects can be created
conveniently and missing attributes can be filled in:

``` {.rust .rust-example-rendered}
use sentry_types::protocol::v7;

let event = v7::Event {
    message: Some("Hello World!".to_string()),
    culprit: Some("foo in bar".to_string()),
    level: v7::Level::Info,
    ..Default::default()
};
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="protocol/index.html" class="mod"
title="mod sentry_types::protocol">protocol</a>  
This module exposes the types for the Sentry protocol in different
versions.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Auth.html" class="struct"
title="struct sentry_types::Auth">Auth</a>  
Represents an auth header.

<a href="struct.BreakpadFormat.html" class="struct"
title="struct sentry_types::BreakpadFormat">BreakpadFormat</a>  
Wrapper around [`DebugId`](struct.DebugId.html) for Breakpad formatting.

<a href="struct.CodeId.html" class="struct"
title="struct sentry_types::CodeId">CodeId</a>  
Unique platform-dependent identifier of code files.

<a href="struct.DebugId.html" class="struct"
title="struct sentry_types::DebugId">DebugId</a>  
Unique identifier for debug information files and their debug
information.

<a href="struct.Dsn.html" class="struct"
title="struct sentry_types::Dsn">Dsn</a>  
Represents a Sentry dsn.

<a href="struct.ParseCodeIdError.html" class="struct"
title="struct sentry_types::ParseCodeIdError">ParseCodeIdError</a>  
Indicates an error parsing a [`CodeId`](struct.CodeId.html).

<a href="struct.ParseDebugIdError.html" class="struct"
title="struct sentry_types::ParseDebugIdError">ParseDebugIdError</a>  
Indicates an error parsing a [`DebugId`](struct.DebugId.html).

<a href="struct.ProjectId.html" class="struct"
title="struct sentry_types::ProjectId">ProjectId</a>  
Represents a project ID.

<a href="struct.Uuid.html" class="struct"
title="struct sentry_types::Uuid">Uuid</a>  
A Universally Unique Identifier (UUID).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.ParseAuthError.html" class="enum"
title="enum sentry_types::ParseAuthError">ParseAuthError</a>  
Represents an auth header parsing error.

<a href="enum.ParseDsnError.html" class="enum"
title="enum sentry_types::ParseDsnError">ParseDsnError</a>  
Represents a dsn url parsing error.

<a href="enum.ParseProjectIdError.html" class="enum"
title="enum sentry_types::ParseProjectIdError">ParseProjectIdError</a>  
Raised if a project ID cannot be parsed from a string.

<a href="enum.Scheme.html" class="enum"
title="enum sentry_types::Scheme">Scheme</a>  
Represents the scheme of an url http/https.

<a href="enum.UuidVariant.html" class="enum"
title="enum sentry_types::UuidVariant">UuidVariant</a>  
The reserved variants of UUIDs.

<a href="enum.UuidVersion.html" class="enum"
title="enum sentry_types::UuidVersion">UuidVersion</a>  
The version of the UUID, denoting the generating algorithm.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.random_uuid.html" class="fn"
title="fn sentry_types::random_uuid">random_uuid</a>  
Generates a random [Uuid](struct.Uuid.html "struct sentry_types::Uuid")
using a thread local RNG.
