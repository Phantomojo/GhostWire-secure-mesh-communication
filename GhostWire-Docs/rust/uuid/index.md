Crate uuid Copy item path

<span class="sub-heading"><a href="../src/uuid/lib.rs.html#12-1740" class="src">Source</a>
</span>

Expand description

Generate and parse universally unique identifiers (UUIDs).

Here’s an example of a UUID:

``` language-text
67e55044-10b1-426f-9247-bb680e5fe0c8
```

A UUID is a unique 128-bit value, stored as 16 octets, and regularly
formatted as a hex string in five groups. UUIDs are used to assign
unique identifiers to entities without requiring a central allocating
authority.

They are particularly useful in distributed systems, though can be used
in disparate areas, such as databases and network protocols. Typically a
UUID is displayed in a readable string form as a sequence of hexadecimal
digits, separated into groups by hyphens.

The uniqueness property is not strictly guaranteed, however for all
practical purposes, it can be assumed that an unintentional collision
would be extremely unlikely.

UUIDs have a number of standardized encodings that are specified in [RFC
9562](https://www.ietf.org/rfc/rfc9562.html).

# <a href="#getting-started" class="doc-anchor">§</a>Getting started

Add the following to your `Cargo.toml`:

``` language-toml
[dependencies.uuid]
version = "1.19.0"
# Lets you generate random UUIDs
features = [
    "v4",
]
```

When you want a UUID, you can generate one:

``` {.rust .rust-example-rendered}
use uuid::Uuid;

let id = Uuid::new_v4();
```

If you have a UUID value, you can use its string literal form inline:

``` {.rust .rust-example-rendered}
use uuid::{uuid, Uuid};

const ID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
```

# <a href="#working-with-different-uuid-versions" class="doc-anchor">§</a>Working with different UUID versions

This library supports all standardized methods for generating UUIDs
through individual Cargo features.

By default, this crate depends on nothing but the Rust standard library
and can parse and format UUIDs, but cannot generate them. Depending on
the kind of UUID you’d like to work with, there are Cargo features that
enable generating them:

-   `v1` - Version 1 UUIDs using a timestamp and monotonic counter.
-   `v3` - Version 3 UUIDs based on the MD5 hash of some data.
-   `v4` - Version 4 UUIDs with random data.
-   `v5` - Version 5 UUIDs based on the SHA1 hash of some data.
-   `v6` - Version 6 UUIDs using a timestamp and monotonic counter.
-   `v7` - Version 7 UUIDs using a Unix timestamp.
-   `v8` - Version 8 UUIDs using user-defined data.

This library also includes a
[`Builder`](struct.Builder.html "struct uuid::Builder") type that can be
used to help construct UUIDs of any version without any additional
dependencies or features. It’s a lower-level API than
[`Uuid`](struct.Uuid.html "struct uuid::Uuid") that can be used when you
need control over implicit requirements on things like a source of
randomness.

## <a href="#which-uuid-version-should-i-use" class="doc-anchor">§</a>Which UUID version should I use?

If you just want to generate unique identifiers then consider version 4
(`v4`) UUIDs. If you want to use UUIDs as database keys or need to sort
them then consider version 7 (`v7`) UUIDs. Other versions should
generally be avoided unless there’s an existing need for them.

Some UUID versions supersede others. Prefer version 6 over version 1 and
version 5 over version 3.

# <a href="#other-features" class="doc-anchor">§</a>Other features

Other crate features can also be useful beyond the version support:

-   `macro-diagnostics` - enhances the diagnostics of `uuid!` macro.
-   `serde` - adds the ability to serialize and deserialize a UUID using
    `serde`.
-   `borsh` - adds the ability to serialize and deserialize a UUID using
    `borsh`.
-   `arbitrary` - adds an `Arbitrary` trait implementation to `Uuid` for
    fuzzing.
-   `fast-rng` - uses a faster algorithm for generating random UUIDs
    when available. This feature requires more dependencies to compile,
    but is just as suitable for UUIDs as the default algorithm.
-   `rng-rand` - forces `rand` as the backend for randomness.
-   `rng-getrandom` - forces `getrandom` as the backend for randomness.
-   `bytemuck` - adds a `Pod` trait implementation to `Uuid` for byte
    manipulation

# <a href="#unstable-features" class="doc-anchor">§</a>Unstable features

Some features are unstable. They may be incomplete or depend on other
unstable libraries. These include:

-   `zerocopy` - adds support for zero-copy deserialization using the
    `zerocopy` library.

Unstable features may break between minor releases.

To allow unstable features, you’ll need to enable the Cargo feature as
normal, but also pass an additional flag through your environment to
opt-in to unstable `uuid` features:

``` language-text
RUSTFLAGS="--cfg uuid_unstable"
```

# <a href="#building-for-other-targets" class="doc-anchor">§</a>Building for other targets

## <a href="#webassembly" class="doc-anchor">§</a>WebAssembly

For WebAssembly, enable the `js` feature:

``` language-toml
[dependencies.uuid]
version = "1.19.0"
features = [
    "v4",
    "v7",
    "js",
]
```

## <a href="#embedded" class="doc-anchor">§</a>Embedded

For embedded targets without the standard library, you’ll need to
disable default features when building `uuid`:

``` language-toml
[dependencies.uuid]
version = "1.19.0"
default-features = false
```

Some additional features are supported in no-std environments:

-   `v1`, `v3`, `v5`, `v6`, and `v8`.
-   `serde`.

If you need to use `v4` or `v7` in a no-std environment, you’ll need to
produce random bytes yourself and then pass them to
[`Builder::from_random_bytes`](struct.Builder.html#method.from_random_bytes "associated function uuid::Builder::from_random_bytes")
without enabling the `v4` or `v7` features.

If you’re using `getrandom`, you can specify the `rng-getrandom` or
`rng-rand` features of `uuid` and configure `getrandom`’s provider per
its docs. `uuid` may upgrade its version of `getrandom` in minor
releases.

# <a href="#examples" class="doc-anchor">§</a>Examples

Parse a UUID given in the simple format and print it as a URN:

``` {.rust .rust-example-rendered}
let my_uuid = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")?;

println!("{}", my_uuid.urn());
```

Generate a random UUID and print it out in hexadecimal form:

``` {.rust .rust-example-rendered}
// Note that this requires the `v4` feature to be enabled.
let my_uuid = Uuid::new_v4();

println!("{}", my_uuid);
```

# <a href="#references" class="doc-anchor">§</a>References

-   [Wikipedia: Universally Unique
    Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
-   [RFC 9562: Universally Unique IDentifiers
    (UUID)](https://www.ietf.org/rfc/rfc9562.html).

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use timestamp::context::`<a href="timestamp/context/struct.NoContext.html" class="struct"
title="struct uuid::timestamp::context::NoContext"><code>NoContext</code></a>`;`

`pub use timestamp::`<a href="timestamp/trait.ClockSequence.html" class="trait"
title="trait uuid::timestamp::ClockSequence"><code>ClockSequence</code></a>`;`

`pub use timestamp::`<a href="timestamp/struct.Timestamp.html" class="struct"
title="struct uuid::timestamp::Timestamp"><code>Timestamp</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="fmt/index.html" class="mod" title="mod uuid::fmt">fmt</a>  
Adapters for alternative string formats.

<a href="serde/index.html" class="mod" title="mod uuid::serde">serde</a>  
Adapters for alternative `serde` formats.

<a href="timestamp/index.html" class="mod"
title="mod uuid::timestamp">timestamp</a>  
Generating UUIDs from timestamps.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.uuid.html" class="macro"
title="macro uuid::uuid">uuid</a>  
Parse [`Uuid`](https://docs.rs/uuid/*/uuid/struct.Uuid.html)s from
string literals at compile time.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Builder.html" class="struct"
title="struct uuid::Builder">Builder</a>  
A builder for creating a UUID.

<a href="struct.Error.html" class="struct"
title="struct uuid::Error">Error</a>  
A general error that can occur when working with UUIDs.

<a href="struct.NonNilUuid.html" class="struct"
title="struct uuid::NonNilUuid">NonNilUuid</a>  
A UUID that is guaranteed not to be the [nil
UUID](https://www.ietf.org/rfc/rfc9562.html#name-nil-uuid).

<a href="struct.Uuid.html" class="struct"
title="struct uuid::Uuid">Uuid</a>  
A Universally Unique Identifier (UUID).

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Variant.html" class="enum"
title="enum uuid::Variant">Variant</a>  
The reserved variants of UUIDs.

<a href="enum.Version.html" class="enum"
title="enum uuid::Version">Version</a>  
The version of the UUID, denoting the generating algorithm.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Bytes.html" class="type"
title="type uuid::Bytes">Bytes</a>  
A 128-bit (16 byte) buffer containing the UUID.
