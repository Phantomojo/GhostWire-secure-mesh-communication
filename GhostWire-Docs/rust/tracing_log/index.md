Crate tracing\_log Copy item path

<span class="sub-heading"><a href="../src/tracing_log/lib.rs.html#1-600" class="src">Source</a>
</span>

Expand description

Adapters for connecting unstructured log records from the `log` crate
into the `tracing` ecosystem.

# <a href="#overview" class="doc-anchor">§</a>Overview

[`tracing`](https://crates.io/crates/tracing) is a framework for
instrumenting Rust programs with context-aware, structured, event-based
diagnostic information. This crate provides compatibility layers for
using `tracing` alongside the logging facade provided by the
[`log`](../log/index.html "mod log") crate.

This crate provides:

-   [`AsTrace`](trait.AsTrace.html "trait tracing_log::AsTrace") and
    [`AsLog`](trait.AsLog.html "trait tracing_log::AsLog") traits for
    converting between `tracing` and `log` types.
-   [`LogTracer`](struct.LogTracer.html "struct tracing_log::LogTracer"),
    a [`log::Log`](../log/trait.Log.html "trait log::Log")
    implementation that consumes
    [`log::Record`](../log/struct.Record.html "struct log::Record")s and
    outputs them as
    [`tracing::Event`](https://docs.rs/tracing/latest/tracing/struct.Event.html).

*Compiler support: [requires `rustc` 1.56+](#supported-rust-versions)*

# <a href="#usage" class="doc-anchor">§</a>Usage

## <a href="#convert-log-records-to-tracing-events"
class="doc-anchor">§</a>Convert log records to tracing `Event`s

To convert
[`log::Record`](../log/struct.Record.html "struct log::Record")s as
[`tracing::Event`](https://docs.rs/tracing/latest/tracing/struct.Event.html)s,
set `LogTracer` as the default logger by calling its
[`init`](struct.LogTracer.html#method.init "associated function tracing_log::LogTracer::init")
or
[`init_with_filter`](struct.LogTracer.html#method.init_with_filter "associated function tracing_log::LogTracer::init_with_filter")
methods.

``` {.rust .rust-example-rendered}
use tracing_log::LogTracer;
use log;

LogTracer::init()?;

// will be available for Subscribers as a tracing Event
log::trace!("an example trace log");
```

This conversion does not convert unstructured data in log records (such
as values passed as format arguments to the `log!` macro) to structured
`tracing` fields. However, it *does* attach these new events to to the
span that was currently executing when the record was logged. This is
the primary use-case for this library: making it possible to locate the
log records emitted by dependencies which use `log` within the context
of a trace.

## <a href="#convert-tracing-events-to-logs" class="doc-anchor">§</a>Convert tracing `Event`s to logs

Enabling the [“log” and “log-always” feature
flags](https://docs.rs/tracing/latest/tracing/#crate-feature-flags) on
the `tracing` crate will cause all `tracing` spans and events to emit
`log::Record`s as they occur.

## <a href="#caution-mixing-both-conversions" class="doc-anchor">§</a>Caution: Mixing both conversions

Note that `log::Logger` implementations that convert log records to
trace events should not be used with `Subscriber`s that convert trace
events *back* into `log` records, as doing so will result in the event
recursing between the subscriber and the logger forever (or, in real
life, probably overflowing the call stack).

If the logging of trace events generated from log records produced by
the `log` crate is desired, either the `log` crate should not be used to
implement this logging, or an additional layer of filtering will be
required to avoid infinitely converting between `Event` and
`log::Record`.

## <a href="#feature-flags" class="doc-anchor">§</a>Feature Flags

-   `std`: enables features that require the Rust standard library (on
    by default)
-   `log-tracer`: enables the `LogTracer` type (on by default)
-   `interest-cache`: makes it possible to configure an interest cache
    for logs emitted through the `log` crate (see
    [`Builder::with_interest_cache`](log_tracer::Builder::with_interest_cache));
    requires `std`

## <a href="#supported-rust-versions" class="doc-anchor">§</a>Supported Rust Versions

Tracing is built against the latest stable release. The minimum
supported version is 1.56. The current Tracing version is not guaranteed
to build on Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the
Tokio project. The current stable Rust compiler and the three most
recent minor versions before it will always be supported. For example,
if the current stable compiler version is 1.69, the minimum supported
version will not be increased past 1.66, three minor versions prior.
Increasing the minimum supported compiler version is not considered a
semver breaking change as long as doing so complies with this policy.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../log/index.html" class="mod"
title="mod log"><code>log</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="log_tracer/index.html" class="mod"
title="mod tracing_log::log_tracer">log_tracer</a>  
An adapter for converting [`log`](../log/index.html "mod log") records
into `tracing` `Event`s.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.LogTracer.html" class="struct"
title="struct tracing_log::LogTracer">LogTracer</a>  
A simple “logger” that converts all log records into `tracing` `Event`s.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsLog.html" class="trait"
title="trait tracing_log::AsLog">AsLog</a>  
Trait implemented for `tracing` types that can be converted to a `log`
equivalent.

<a href="trait.AsTrace.html" class="trait"
title="trait tracing_log::AsTrace">AsTrace</a>  
Trait implemented for `log` types that can be converted to a `tracing`
equivalent.

<a href="trait.NormalizeEvent.html" class="trait"
title="trait tracing_log::NormalizeEvent">NormalizeEvent</a>  
Extends log `Event`s to provide complete `Metadata`.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.format_trace.html" class="fn"
title="fn tracing_log::format_trace">format_trace</a>  
Format a log record as a trace event in the current span.
