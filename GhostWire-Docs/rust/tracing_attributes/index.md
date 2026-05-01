Crate tracing\_attributes Copy item path

<span class="sub-heading"><a href="../src/tracing_attributes/lib.rs.html#1-717"
class="src">Source</a> </span>

Expand description

A procedural macro attribute for instrumenting functions with
[`tracing`](https://crates.io/crates/tracing).

[`tracing`](https://crates.io/crates/tracing) is a framework for
instrumenting Rust programs to collect structured, event-based
diagnostic information. This crate provides the
[`#[instrument]`](attr.instrument.html "attr tracing_attributes::instrument")
procedural macro attribute.

Note that this macro is also re-exported by the main `tracing` crate.

*Compiler support: [requires `rustc` 1.65+](#supported-rust-versions)*

## <a href="#usage" class="doc-anchor">§</a>Usage

In the `Cargo.toml`:

``` language-toml
[dependencies]
tracing-attributes = "0.1.24"
```

The
[`#[instrument]`](attr.instrument.html "attr tracing_attributes::instrument")
attribute can now be added to a function to automatically create and
enter `tracing`
[span](https://docs.rs/tracing/latest/tracing/span/index.html) when that
function is called. For example:

``` {.rust .rust-example-rendered}
use tracing::instrument;

#[instrument]
pub fn my_function(my_arg: usize) {
    // ...
}
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

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.instrument.html" class="attr"
title="attr tracing_attributes::instrument">instrument</a>  
Instruments a function to create and enter a `tracing`
[span](https://docs.rs/tracing/latest/tracing/span/index.html) every
time the function is called.
