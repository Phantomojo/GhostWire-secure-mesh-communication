Crate tracing\_core Copy item path

<span class="sub-heading"><a href="../src/tracing_core/lib.rs.html#1-311" class="src">Source</a>
</span>

Expand description

Core primitives for `tracing`.

[`tracing`](https://crates.io/crates/tracing) is a framework for
instrumenting Rust programs to collect structured, event-based
diagnostic information. This crate defines the core primitives of
`tracing`.

This crate provides:

-   [`span::Id`](span/struct.Id.html "struct tracing_core::span::Id")
    identifies a span within the execution of a program.

-   [`Event`](struct.Event.html "struct tracing_core::Event") represents
    a single event within a trace.

-   [`Subscriber`](trait.Subscriber.html "trait tracing_core::Subscriber"),
    the trait implemented to collect trace data.

-   [`Metadata`](struct.Metadata.html "struct tracing_core::Metadata")
    and [`Callsite`](trait.Callsite.html "trait tracing_core::Callsite")
    provide information describing spans and `Event`s.

-   [`Field`](struct.Field.html "struct tracing_core::Field"),
    [`FieldSet`](field/struct.FieldSet.html "struct tracing_core::field::FieldSet"),
    [`Value`](field/trait.Value.html "trait tracing_core::field::Value"),
    and
    [`ValueSet`](field/struct.ValueSet.html "struct tracing_core::field::ValueSet")
    represent the structured data attached to a span.

-   [`Dispatch`](struct.Dispatch.html "struct tracing_core::Dispatch")
    allows spans and events to be dispatched to `Subscriber`s.

In addition, it defines the global callsite registry and per-thread
current dispatcher which other components of the tracing system rely on.

*Compiler support: [requires `rustc` 1.65+](#supported-rust-versions)*

## <a href="#usage" class="doc-anchor">§</a>Usage

Application authors will typically not use this crate directly. Instead,
they will use the [`tracing`](https://crates.io/crates/tracing) crate,
which provides a much more fully-featured API. However, this crate’s API
will change very infrequently, so it may be used when dependencies must
be very stable.

`Subscriber` implementations may depend on `tracing-core` rather than
`tracing`, as the additional APIs provided by `tracing` are primarily
useful for instrumenting libraries and applications, and are generally
not necessary for `Subscriber` implementations.

The [`tokio-rs/tracing`](https://github.com/tokio-rs/tracing) repository
contains less stable crates designed to be used with the `tracing`
ecosystem. It includes a collection of `Subscriber` implementations, as
well as utility and adapter crates.

## <a href="#crate-feature-flags" class="doc-anchor">§</a>Crate Feature Flags

The following crate [feature
flags](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section)
are available:

-   `std`: Depend on the Rust standard library (enabled by default).

    `no_std` users may disable this feature with
    `default-features = false`:

    ``` language-toml
    [dependencies]
    tracing-core = { version = "0.1.22", default-features = false }
    ```

    **Note**:`tracing-core`’s `no_std` support requires `liballoc`.

### <a href="#unstable-features" class="doc-anchor">§</a>Unstable Features

These feature flags enable **unstable** features. The public API may
break in 0.1.x releases. To enable these features, the
`--cfg tracing_unstable` must be passed to `rustc` when compiling.

The following unstable feature flags are currently available:

-   `valuable`: Enables support for recording [field
    values](field/index.html "mod tracing_core::field") using the
    [`valuable`](https://crates.io/crates/valuable) crate.

#### <a href="#enabling-unstable-features" class="doc-anchor">§</a>Enabling Unstable Features

The easiest way to set the `tracing_unstable` cfg is to use the
`RUSTFLAGS` env variable when running `cargo` commands:

``` language-shell
RUSTFLAGS="--cfg tracing_unstable" cargo build
```

Alternatively, the following can be added to the `.cargo/config` file in
a project to automatically enable the cfg flag for that project:

``` language-toml
[build]
rustflags = ["--cfg", "tracing_unstable"]
```

## <a href="#supported-rust-versions" class="doc-anchor">§</a>Supported Rust Versions

Tracing is built against the latest stable release. The minimum
supported version is 1.65. The current Tracing version is not guaranteed
to build on Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the
Tokio project. The current stable Rust compiler and the three most
recent minor versions before it will always be supported. For example,
if the current stable compiler version is 1.69, the minimum supported
version will not be increased past 1.66, three minor versions prior.
Increasing the minimum supported compiler version is not considered a
semver breaking change as long as doing so complies with this policy.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::metadata::`<a href="metadata/struct.Kind.html" class="struct"
title="struct tracing_core::metadata::Kind"><code>Kind</code></a>`;`

`pub use self::subscriber::`<a href="subscriber/struct.Interest.html" class="struct"
title="struct tracing_core::subscriber::Interest"><code>Interest</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="callsite/index.html" class="mod"
title="mod tracing_core::callsite">callsite</a>  
Callsites represent the source locations from which spans or events
originate.

<a href="dispatcher/index.html" class="mod"
title="mod tracing_core::dispatcher">dispatcher</a>  
Dispatches trace events to
[`Subscriber`](trait.Subscriber.html "trait tracing_core::Subscriber")s.

<a href="event/index.html" class="mod"
title="mod tracing_core::event">event</a>  
Events represent single points in time during the execution of a
program.

<a href="field/index.html" class="mod"
title="mod tracing_core::field">field</a>  
`Span` and `Event` key-value data.

<a href="metadata/index.html" class="mod"
title="mod tracing_core::metadata">metadata</a>  
Metadata describing trace data.

<a href="span/index.html" class="mod"
title="mod tracing_core::span">span</a>  
Spans represent periods of time in the execution of a program.

<a href="subscriber/index.html" class="mod"
title="mod tracing_core::subscriber">subscriber</a>  
Collectors collect and record trace data.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.identify_callsite.html" class="macro"
title="macro tracing_core::identify_callsite">identify_callsite</a>  
Statically constructs an
[`Identifier`](callsite/struct.Identifier.html "struct tracing_core::callsite::Identifier")
for the provided
[`Callsite`](trait.Callsite.html "trait tracing_core::Callsite").

<a href="macro.metadata.html" class="macro"
title="macro tracing_core::metadata">metadata</a>  
Statically constructs new span
[metadata](struct.Metadata.html "struct tracing_core::Metadata").

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Dispatch.html" class="struct"
title="struct tracing_core::Dispatch">Dispatch</a>  
`Dispatch` trace data to a
[`Subscriber`](trait.Subscriber.html "trait tracing_core::Subscriber").

<a href="struct.Event.html" class="struct"
title="struct tracing_core::Event">Event</a>  
`Event`s represent single points in time where something occurred during
the execution of a program.

<a href="struct.Field.html" class="struct"
title="struct tracing_core::Field">Field</a>  
An opaque key allowing *O*(1) access to a field in a `Span`’s key-value
data.

<a href="struct.Level.html" class="struct"
title="struct tracing_core::Level">Level</a>  
Describes the level of verbosity of a span or event.

<a href="struct.LevelFilter.html" class="struct"
title="struct tracing_core::LevelFilter">LevelFilter</a>  
A filter comparable to a verbosity
[`Level`](struct.Level.html "struct tracing_core::Level").

<a href="struct.Metadata.html" class="struct"
title="struct tracing_core::Metadata">Metadata</a>  
Metadata describing a [span](span/index.html "mod tracing_core::span")
or [event](event/index.html "mod tracing_core::event").

<a href="struct.Once.html" class="struct"
title="struct tracing_core::Once">Once</a>  
A low-level synchronization primitive for one-time global execution.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Callsite.html" class="trait"
title="trait tracing_core::Callsite">Callsite</a>  
Trait implemented by callsites.

<a href="trait.Subscriber.html" class="trait"
title="trait tracing_core::Subscriber">Subscriber</a>  
Trait representing the functions required to collect trace data.
