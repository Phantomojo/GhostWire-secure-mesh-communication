Crate tracing\_serde Copy item path

<span class="sub-heading"><a href="../src/tracing_serde/lib.rs.html#1-607" class="src">Source</a>
</span>

Expand description

# <a href="#tracing-serde" class="doc-anchor">§</a>tracing-serde

An adapter for serializing [`tracing`](https://crates.io/crates/tracing)
types using [`serde`](https://crates.io/crates/serde).

[![Documentation](https://docs.rs/tracing-serde/badge.svg)](https://docs.rs/tracing-serde)
[![Documentation
(master)](https://img.shields.io/badge/docs-master-blue)](https://tracing-rs.netlify.com/tracing_serde)

## <a href="#overview" class="doc-anchor">§</a>Overview

[`tracing`](https://crates.io/crates/tracing) is a framework for
instrumenting Rust programs to collect scoped, structured, and
async-aware diagnostics.`tracing-serde` enables serializing `tracing`
types using [`serde`](https://crates.io/crates/serde).

Traditional logging is based on human-readable text messages. `tracing`
gives us machine-readable structured diagnostic information. This lets
us interact with diagnostic data programmatically. With `tracing-serde`,
you can implement a `Subscriber` to serialize your `tracing` types and
make use of the existing ecosystem of `serde` serializers to talk with
distributed tracing systems.

Serializing diagnostic information allows us to do more with our logged
values. For instance, when working with logging data in JSON gives us
pretty-print when we’re debugging in development and you can emit JSON
and tracing data to monitor your services in production.

The `tracing` crate provides the APIs necessary for instrumenting
libraries and applications to emit trace data.

*Compiler support: [requires `rustc` 1.63+](#supported-rust-versions)*

## <a href="#usage" class="doc-anchor">§</a>Usage

First, add this to your `Cargo.toml`:

``` language-toml
[dependencies]
tracing = "0.1"
tracing-serde = "0.2"
```

Next, add this to your crate:

``` {.rust .rust-example-rendered}
use tracing_serde::AsSerde;
```

Please read the [`tracing`
documentation](https://docs.rs/tracing/latest/tracing/index.html) for
more information on how to create trace data.

This crate provides the `as_serde` function, via the `AsSerde` trait,
which enables serializing the `Attributes`, `Event`, `Id`, `Metadata`,
and `Record` `tracing` values.

For the full example, please see the [examples](../examples) folder.

Implement a `Subscriber` to format the serialization of `tracing` types
how you’d like.

``` {.rust .rust-example-rendered}
use tracing_serde::AsSerde;
use serde_json::json;

pub struct JsonSubscriber {
    next_id: AtomicUsize, // you need to assign span IDs, so you need a counter
}

impl JsonSubscriber {
    fn new() -> Self {
        Self { next_id: 1.into() }
    }
}

impl Subscriber for JsonSubscriber {

    fn new_span(&self, attrs: &Attributes<'_>) -> Id {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let id = Id::from_u64(id as u64);
        let json = json!({
        "new_span": {
            "attributes": attrs.as_serde(),
            "id": id.as_serde(),
        }});
        println!("{}", json);
        id
    }

    fn event(&self, event: &Event<'_>) {
        let json = json!({
           "event": event.as_serde(),
        });
        println!("{}", json);
    }

    // ...
}
```

After you implement your `Subscriber`, you can use your `tracing`
subscriber (`JsonSubscriber` in the above example) to record serialized
trace data.

### <a href="#unstable-features" class="doc-anchor">§</a>Unstable Features

These feature flags enable **unstable** features. The public API may
break in 0.1.x releases. To enable these features, the
`--cfg tracing_unstable` must be passed to `rustc` when compiling.

The following unstable feature flags are currently available:

-   `valuable`: Enables \[`Visit::record_value`\] implementations, for
    serializing values recorded using the
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
supported version is 1.63. The current Tracing version is not guaranteed
to build on Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the
Tokio project. The current stable Rust compiler and the three most
recent minor versions before it will always be supported. For example,
if the current stable compiler version is 1.69, the minimum supported
version will not be increased past 1.66, three minor versions prior.
Increasing the minimum supported compiler version is not considered a
semver breaking change as long as doing so complies with this policy.

# Modules<a href="#modules" class="anchor">§</a>

<a href="fields/index.html" class="mod"
title="mod tracing_serde::fields">fields</a>  
Support for serializing fields as `serde` structs or maps.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.SerdeMapVisitor.html" class="struct"
title="struct tracing_serde::SerdeMapVisitor">SerdeMapVisitor</a>  
Implements `tracing_core::field::Visit` for some
`serde::ser::SerializeMap`.

<a href="struct.SerdeStructVisitor.html" class="struct"
title="struct tracing_serde::SerdeStructVisitor">SerdeStructVisitor</a>  
Implements `tracing_core::field::Visit` for some
`serde::ser::SerializeStruct`.

<a href="struct.SerializeAttributes.html" class="struct"
title="struct tracing_serde::SerializeAttributes">SerializeAttributes</a>  
Implements `serde::Serialize` to write `Attributes` data to a
serializer.

<a href="struct.SerializeEvent.html" class="struct"
title="struct tracing_serde::SerializeEvent">SerializeEvent</a>  
Implements `serde::Serialize` to write `Event` data to a serializer.

<a href="struct.SerializeField.html" class="struct"
title="struct tracing_serde::SerializeField">SerializeField</a>  
<a href="struct.SerializeFieldSet.html" class="struct"
title="struct tracing_serde::SerializeFieldSet">SerializeFieldSet</a>  
<a href="struct.SerializeId.html" class="struct"
title="struct tracing_serde::SerializeId">SerializeId</a>  
<a href="struct.SerializeLevel.html" class="struct"
title="struct tracing_serde::SerializeLevel">SerializeLevel</a>  
<a href="struct.SerializeMetadata.html" class="struct"
title="struct tracing_serde::SerializeMetadata">SerializeMetadata</a>  
<a href="struct.SerializeRecord.html" class="struct"
title="struct tracing_serde::SerializeRecord">SerializeRecord</a>  
Implements `serde::Serialize` to write `Record` data to a serializer.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsSerde.html" class="trait"
title="trait tracing_serde::AsSerde">AsSerde</a>
