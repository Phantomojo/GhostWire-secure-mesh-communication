Crate tracing\_subscriber Copy item path

<span class="sub-heading"><a href="../src/tracing_subscriber/lib.rs.html#1-256"
class="src">Source</a> </span>

Expand description

Utilities for implementing and composing
[`tracing`](https://docs.rs/tracing/latest/tracing) subscribers.

[`tracing`](https://docs.rs/tracing/latest/tracing) is a framework for
instrumenting Rust programs to collect scoped, structured, and
async-aware diagnostics. The
[`Subscriber`](../tracing_core/subscriber/trait.Subscriber.html "trait tracing_core::subscriber::Subscriber")
trait represents the functionality necessary to collect this trace data.
This crate contains tools for composing subscribers out of smaller units
of behaviour, and batteries-included implementations of common
subscriber functionality.

`tracing-subscriber` is intended for use by both `Subscriber` authors
and application authors using `tracing` to instrument their
applications.

*Compiler support: [requires `rustc` 1.65+](#supported-rust-versions)*

## <a href="#layers-and-filters" class="doc-anchor">§</a>`Layer`s and `Filter`s

The most important component of the `tracing-subscriber` API is the
[`Layer`](layer/trait.Layer.html "trait tracing_subscriber::layer::Layer")
trait, which provides a composable abstraction for building
[`Subscriber`](../tracing_core/subscriber/trait.Subscriber.html "trait tracing_core::subscriber::Subscriber")s.
Like the
[`Subscriber`](../tracing_core/subscriber/trait.Subscriber.html "trait tracing_core::subscriber::Subscriber")
trait, a
[`Layer`](layer/trait.Layer.html "trait tracing_subscriber::layer::Layer")
defines a particular behavior for collecting trace data. Unlike
[`Subscriber`](../tracing_core/subscriber/trait.Subscriber.html "trait tracing_core::subscriber::Subscriber")s,
which implement a *complete* strategy for how trace data is collected,
[`Layer`](layer/trait.Layer.html "trait tracing_subscriber::layer::Layer")s
provide *modular* implementations of specific behaviors. Therefore, they
can be [composed
together](layer/index.html#composing-layers "mod tracing_subscriber::layer")
to form a
[`Subscriber`](../tracing_core/subscriber/trait.Subscriber.html "trait tracing_core::subscriber::Subscriber")
which is capable of recording traces in a variety of ways. See the
[`layer` module’s
documentation](layer/index.html "mod tracing_subscriber::layer") for
details on using
[`Layer`](layer/trait.Layer.html "trait tracing_subscriber::layer::Layer")s.

In addition, the
[`Filter`](layer/trait.Filter.html "trait tracing_subscriber::layer::Filter")
trait defines an interface for filtering what spans and events are
recorded by a particular layer. This allows different
[`Layer`](layer/trait.Layer.html "trait tracing_subscriber::layer::Layer")s
to handle separate subsets of the trace data emitted by a program. See
the [documentation on per-layer
filtering](layer/index.html#per-layer-filtering "mod tracing_subscriber::layer")
for more information on using
[`Filter`](layer/trait.Filter.html "trait tracing_subscriber::layer::Filter")s.

## <a href="#included-subscribers" class="doc-anchor">§</a>Included Subscribers

The following `Subscriber`s are provided for application authors:

-   [`fmt`](fmt/index.html "mod tracing_subscriber::fmt") - Formats and
    logs tracing data (requires the `fmt` feature flag)

## <a href="#feature-flags" class="doc-anchor">§</a>Feature Flags

-   `std`: Enables APIs that depend on the Rust standard library
    (enabled by default).
-   `alloc`: Depend on
    [`liballoc`](https://doc.rust-lang.org/alloc/index.html) (enabled by
    “std”).
-   `env-filter`: Enables the
    [`EnvFilter`](filter/struct.EnvFilter.html "struct tracing_subscriber::filter::EnvFilter")
    type, which implements filtering similar to the [`env_logger`
    crate](https://crates.io/crates/env_logger). **Requires “std”**.
-   `fmt`: Enables the
    [`fmt`](fmt/index.html "mod tracing_subscriber::fmt") module, which
    provides a subscriber implementation for printing formatted
    representations of trace events. Enabled by default. **Requires
    “registry” and “std”**.
-   `ansi`: Enables `fmt` support for ANSI terminal colors. Enabled by
    default.
-   `registry`: enables the
    [`registry`](registry/index.html "mod tracing_subscriber::registry")
    module. Enabled by default. **Requires “std”**.
-   `json`: Enables `fmt` support for JSON output. In JSON output, the
    ANSI feature does nothing. **Requires “fmt” and “std”**.
-   `local-time`: Enables local time formatting when using the [`time`
    crate](https://crates.io/crates/time)’s timestamp formatters with
    the `fmt` subscriber.

### <a href="#optional-dependencies" class="doc-anchor">§</a>Optional Dependencies

-   [`tracing-log`](https://crates.io/crates/tracing-log): Enables
    better formatting for events emitted by `log` macros in the `fmt`
    subscriber. Enabled by default.
-   [`time`](https://crates.io/crates/time): Enables support for using
    the [`time` crate](https://crates.io/crates/time) for timestamp
    formatting in the `fmt` subscriber.
-   [`smallvec`](https://crates.io/crates/smallvec): Causes the
    `EnvFilter` type to use the `smallvec` crate (rather than `Vec`) as
    a performance optimization. Enabled by default.
-   [`parking_lot`](https://crates.io/crates/parking_lot): Use the
    `parking_lot` crate’s `RwLock` implementation rather than the Rust
    standard library’s implementation.

### <a href="#no_std-support" class="doc-anchor">§</a>`no_std` Support

In embedded systems and other bare-metal applications, `tracing` can be
used without requiring the Rust standard library, although some features
are disabled. Although most of the APIs provided by
`tracing-subscriber`, such as
[`fmt`](fmt/index.html "mod tracing_subscriber::fmt") and
[`EnvFilter`](filter/struct.EnvFilter.html "struct tracing_subscriber::filter::EnvFilter"),
require the standard library, some functionality, such as the
[`Layer`](layer/trait.Layer.html "trait tracing_subscriber::layer::Layer")
trait, can still be used in `no_std` environments.

The dependency on the standard library is controlled by two crate
feature flags, “std”, which enables the dependency on
[`libstd`](https://doc.rust-lang.org/std/index.html), and “alloc”, which
enables the dependency on
[`liballoc`](https://doc.rust-lang.org/alloc/index.html) (and is enabled
by the “std” feature). These features are enabled by default, but
`no_std` users can disable them using:

``` language-toml
# Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false }
```

Additional APIs are available when
[`liballoc`](https://doc.rust-lang.org/alloc/index.html) is available.
To enable `liballoc` but not `std`, use:

``` language-toml
# Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false, features = ["alloc"] }
```

### <a href="#unstable-features" class="doc-anchor">§</a>Unstable Features

These feature flags enable **unstable** features. The public API may
break in 0.1.x releases. To enable these features, the
`--cfg tracing_unstable` must be passed to `rustc` when compiling.

The following unstable feature flags are currently available:

-   `valuable`: Enables support for serializing values recorded using
    the [`valuable`](https://crates.io/crates/valuable) crate as
    structured JSON in the
    [`format::Json`](fmt/format/struct.Json.html "struct tracing_subscriber::fmt::format::Json")
    formatter.

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

`pub use fmt::`<a href="fmt/fn.fmt.html" class="fn"
title="fn tracing_subscriber::fmt::fmt"><code>fmt</code></a>`;`

`pub use fmt::`<a href="fmt/struct.Subscriber.html" class="struct"
title="struct tracing_subscriber::fmt::Subscriber"><code>Subscriber</code></a>` as FmtSubscriber;`

`pub use filter::`<a href="filter/struct.EnvFilter.html" class="struct"
title="struct tracing_subscriber::filter::EnvFilter"><code>EnvFilter</code></a>`;`

`pub use layer::`<a href="layer/trait.Layer.html" class="trait"
title="trait tracing_subscriber::layer::Layer"><code>Layer</code></a>`;`

`pub use registry::`<a href="registry/struct.Registry.html" class="struct"
title="struct tracing_subscriber::registry::Registry"><code>Registry</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="field/index.html" class="mod"
title="mod tracing_subscriber::field">field</a>  
Utilities for working with
[fields](../tracing_core/field/index.html "mod tracing_core::field") and
[field
visitors](field/trait.Visit.html "trait tracing_subscriber::field::Visit").

<a href="filter/index.html" class="mod"
title="mod tracing_subscriber::filter">filter</a>  
[`Layer`](layer/index.html "mod tracing_subscriber::layer")s that
control which spans and events are enabled by the wrapped subscriber.

<a href="fmt/index.html" class="mod"
title="mod tracing_subscriber::fmt">fmt</a>  
A `Subscriber` for formatting and logging `tracing` data.

<a href="layer/index.html" class="mod"
title="mod tracing_subscriber::layer">layer</a>  
The
[`Layer`](layer/trait.Layer.html "trait tracing_subscriber::layer::Layer")
trait, a composable abstraction for building
[`Subscriber`](../tracing_core/subscriber/trait.Subscriber.html "trait tracing_core::subscriber::Subscriber")s.

<a href="prelude/index.html" class="mod"
title="mod tracing_subscriber::prelude">prelude</a>  
The `tracing-subscriber` prelude.

<a href="registry/index.html" class="mod"
title="mod tracing_subscriber::registry">registry</a>  
Storage for span data shared by multiple
[`Layer`](layer/trait.Layer.html "trait tracing_subscriber::layer::Layer")s.

<a href="reload/index.html" class="mod"
title="mod tracing_subscriber::reload">reload</a>  
Wrapper for a `Layer` to allow it to be dynamically reloaded.

<a href="util/index.html" class="mod"
title="mod tracing_subscriber::util">util</a>  
Extension traits and other utilities to make working with subscribers
more ergonomic.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.registry.html" class="fn"
title="fn tracing_subscriber::registry">registry</a>  
Returns a default
[`Registry`](registry/struct.Registry.html "struct tracing_subscriber::registry::Registry").
