Crate opentelemetry Copy item path

<span class="sub-heading"><a href="../src/opentelemetry/lib.rs.html#1-307" class="src">Source</a>
</span>

Expand description

Implements the
[`API`](https://opentelemetry.io/docs/specs/otel/overview/#api)
component of
[OpenTelemetry](https://opentelemetry.io/docs/what-is-opentelemetry/).

*[Supported Rust Versions](#supported-rust-versions)*

# <a href="#getting-started-with-traces" class="doc-anchor">§</a>Getting Started with Traces

The [`trace`](trace/index.html "mod opentelemetry::trace") module
includes types for tracking the progression of a single request while it
is handled by services that make up an application. A trace is a tree of
[`Span`](trace/trait.Span.html "trait opentelemetry::trace::Span")s
which are objects that represent the work being done by individual
services or components involved in a request as it flows through a
system.

``` {.rust .rust-example-rendered}
use opentelemetry::{global, trace::{Span, Tracer}, KeyValue};

// get a tracer from a provider
let tracer = global::tracer("my_service");

// start a new span
let mut span = tracer.start("my_span");

// set some attributes
span.set_attribute(KeyValue::new("http.client_ip", "83.164.160.102"));

// perform some more work...

// end or drop the span to export
span.end();
```

See the
[examples](https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples)
directory for different integration patterns.

See the [`trace`](trace/index.html "mod opentelemetry::trace") module
docs for more information on creating and managing spans.

# <a href="#getting-started-with-metrics" class="doc-anchor">§</a>Getting Started with Metrics

The [`metrics`](metrics/index.html "mod opentelemetry::metrics") module
provides types for recording measurements about a service at runtime.
Below are the key steps to report measurements using OpenTelemetry
Metrics:

1.  **Obtain a Meter:** Get a `Meter` from a `MeterProvider`.
2.  **Create Instruments:** Use the `Meter` to create one or more
    instruments (e.g., counters, histograms).
3.  **Record Measurements:** Use the instruments to record measurement
    values along with optional attributes.

## <a href="#how-metrics-work-in-opentelemetry" class="doc-anchor">§</a>How Metrics work in OpenTelemetry

In OpenTelemetry, raw measurements recorded using instruments are
**aggregated in memory** to form metrics. These aggregated metrics are
periodically exported by the
[`opentelemetry_sdk`](https://crates.io/crates/opentelemetry_sdk) at
fixed intervals (e.g., every 60 seconds) via exporters such as
[`opentelemetry-stdout`](https://crates.io/crates/opentelemetry_stdout)
or [`opentelemetry-otlp`](https://crates.io/crates/opentelemetry-otlp).
This reduces reporting overhead while ensuring up-to-date data. The
aggregation strategy and export interval can be customized in the
[`opentelemetry_sdk`](https://crates.io/crates/opentelemetry_sdk) based
on your use case.

## <a href="#choosing-the-right-instrument" class="doc-anchor">§</a>Choosing the Right Instrument

Selecting the correct instrument is critical for accurately representing
your metrics data:

-   Use **Counters** for values that only increase, such as the number
    of requests served or errors encountered.
-   Use **UpDownCounters** for values that can increase or decrease,
    such as the number of active connections, number of items in a queue
    etc.
-   **Gauges:** Use for values that can go up or down and represent the
    current state, such as CPU usage, temperature etc.
-   Use **Histograms** for measuring the distribution of a value, such
    as response times or payload sizes.

### <a href="#observable-instruments" class="doc-anchor">§</a>Observable Instruments

Counters, UpDownCounters, and Gauges have Observable variants that allow
values to be reported through a callback function. Observable
instruments are ideal when the metric value is managed elsewhere and
needs to be observed by OpenTelemetry instrumentation. The callbacks are
automatically invoked by the OpenTelemetry SDK before every export
(e.g., every 60 seconds).

For example:

-   An **ObservableCounter** can monitor the number of page faults in a
    process as reported by the operating system.
-   An **ObservableUpDownCounter** can monitor the size of an in-memory
    queue by reporting the size using queue’s len() method within the
    callback function.
-   An **ObservableGauge** can monitor the CPU temperature by using
    temperature sensor APIs within the callback function.

For detailed guidance, refer to [OpenTelemetry Metrics API -
Instrumentation
Guidance](https://opentelemetry.io/docs/specs/otel/metrics/supplementary-guidelines/#instrument-selection).

## <a href="#best-practices" class="doc-anchor">§</a>Best Practices

-   **Re-use Instruments:** Instruments are designed for reuse. Avoid
    creating new instruments repeatedly.
-   **Clone for Sharing:** If the same instrument needs to be used
    across multiple parts of your code, you can safely clone it to
    share.

## <a href="#example-usage" class="doc-anchor">§</a>Example Usage

``` {.rust .rust-example-rendered}
use opentelemetry::{global, KeyValue};

// Get a meter from a provider.
let meter = global::meter("my_service");

// Create an instrument (in this case, a Counter).
let counter = meter.u64_counter("request.count").build();

// Record a measurement by passing the value and a set of attributes.
counter.add(1, &[KeyValue::new("http.client_ip", "83.164.160.102")]);

// Create an ObservableCounter and register a callback that reports the measurement.
let _observable_counter = meter
.u64_observable_counter("bytes_received")
.with_callback(|observer| {
    observer.observe(
        100,
        &[
            KeyValue::new("protocol", "udp"),
        ],
    )
})
.build();
```

See the
[examples](https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples/metrics-basic)
directory that show a runnable example with all type of instruments.

See the [`metrics`](metrics/index.html "mod opentelemetry::metrics")
module docs for more information on creating and managing instruments.

# <a href="#getting-started-with-logs" class="doc-anchor">§</a>Getting Started with Logs

The [`logs`](logs/index.html "mod opentelemetry::logs") module contains
the Logs Bridge API. It is not intended to be called by application
developers directly. It is provided for logging library authors to build
log appenders, that bridges existing logging systems with OpenTelemetry.
Bridges for [`log`](https://crates.io/crates/log) and
[`tracing`](https://crates.io/crates/tracing) libraries are provided via
the
[`opentelemetry-appender-log`](https://crates.io/crates/opentelemetry-appender-log)
and
[`opentelemetry-appender-tracing`](https://crates.io/crates/opentelemetry-appender-tracing)
crates.

# <a href="#crate-feature-flags" class="doc-anchor">§</a>Crate Feature Flags

The following core crate feature flags are available:

-   `trace`: Includes the trace API.
-   `metrics`: Includes the metrics API.
-   `logs`: Includes the logs bridge API.
-   `internal-logs`: Includes internal logging for the OpenTelemetry
    library via `tracing`.

The default feature flags are \[“trace”, “metrics”, “logs”,
“internal-logs”\].

The following feature flags provides additional configuration for
`logs`:

-   `spec_unstable_logs_enabled`: Allow users to control the log level

The following feature flags enable APIs defined in OpenTelemetry
specification that is in experimental phase:

-   `otel_unstable`: Includes unstable APIs. There are no features
    behind this flag at the moment.

# <a href="#related-crates" class="doc-anchor">§</a>Related Crates

In addition to `opentelemetry`, the
[`open-telemetry/opentelemetry-rust`](https://github.com/open-telemetry/opentelemetry-rust)
repository contains several additional crates designed to be used with
the `opentelemetry` ecosystem. This includes exporters, samplers, as
well as utility and adapter crates to assist in propagating context and
instrumenting applications.

In particular, the following crates are likely to be of interest:

-   [`opentelemetry_sdk`](https://crates.io/crates/opentelemetry_sdk)
    provides the OpenTelemetry SDK used to configure providers.
-   [`opentelemetry-http`](https://crates.io/crates/opentelemetry-http)
    provides an interface for injecting and extracting trace information
    from [`http`](https://crates.io/crates/http) headers.
-   [`opentelemetry-otlp`](https://crates.io/crates/opentelemetry-otlp)
    exporter for sending telemetry in the OTLP format.
-   [`opentelemetry-stdout`](https://crates.io/crates/opentelemetry_stdout)
    provides ability to output telemetry to stdout, primarily used for
    learning/debugging purposes.
-   [`opentelemetry-prometheus`](https://crates.io/crates/opentelemetry-prometheus)
    provides a pipeline and exporter for sending metrics information to
    [`Prometheus`](https://prometheus.io).
-   [`opentelemetry-zipkin`](https://crates.io/crates/opentelemetry-zipkin)
    provides a pipeline and exporter for sending trace information to
    [`Zipkin`](https://zipkin.io).

In addition, there are several other useful crates in the [OTel Rust
Contrib
repo](https://github.com/open-telemetry/opentelemetry-rust-contrib). A
lot of crates maintained outside OpenTelemetry owned repos can be found
in the [OpenTelemetry
Registry](https://opentelemetry.io/ecosystem/registry/?language=rust).

# <a href="#supported-rust-versions" class="doc-anchor">§</a>Supported Rust Versions

OpenTelemetry is built against the latest stable release. The minimum
supported version is 1.70. The current OpenTelemetry version is not
guaranteed to build on Rust versions earlier than the minimum supported
version.

The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current
stable compiler version is 1.49, the minimum supported version will not
be increased past 1.46, three minor versions prior. Increasing the
minimum supported compiler version is not considered a semver breaking
change as long as doing so complies with this policy.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use context::`<a href="context/struct.Context.html" class="struct"
title="struct opentelemetry::context::Context"><code>Context</code></a>`;`

`pub use context::`<a href="context/struct.ContextGuard.html" class="struct"
title="struct opentelemetry::context::ContextGuard"><code>ContextGuard</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="baggage/index.html" class="mod"
title="mod opentelemetry::baggage">baggage</a>  
Primitives for sending name/value data across system boundaries.

<a href="context/index.html" class="mod"
title="mod opentelemetry::context">context</a>  
Execution-scoped context propagation.

<a href="global/index.html" class="mod"
title="mod opentelemetry::global">global</a>  
Utilities for working with global telemetry primitives

<a href="logs/index.html" class="mod"
title="mod opentelemetry::logs">logs</a>  
OpenTelemetry Logs Bridge API

<a href="metrics/index.html" class="mod"
title="mod opentelemetry::metrics">metrics</a>  
OpenTelemetry Metrics API

<a href="propagation/index.html" class="mod"
title="mod opentelemetry::propagation">propagation</a>  
OpenTelemetry Propagator interface

<a href="trace/index.html" class="mod"
title="mod opentelemetry::trace">trace</a>  
API for tracing applications and libraries.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.otel_debug.html" class="macro"
title="macro opentelemetry::otel_debug">otel_debug</a>  
Macro for logging debug messages in OpenTelemetry.

<a href="macro.otel_error.html" class="macro"
title="macro opentelemetry::otel_error">otel_error</a>  
Macro for logging error messages in OpenTelemetry.

<a href="macro.otel_info.html" class="macro"
title="macro opentelemetry::otel_info">otel_info</a>  
**Note**: These macros (`otel_info!`, `otel_warn!`, `otel_debug!`, and
`otel_error!`) are intended to be used **internally within OpenTelemetry
code** or for **custom exporters, processors and other plugins**. They
are not designed for general application logging and should not be used
for that purpose.

<a href="macro.otel_warn.html" class="macro"
title="macro opentelemetry::otel_warn">otel_warn</a>  
Macro for logging warning messages in OpenTelemetry.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.InstrumentationScope.html" class="struct"
title="struct opentelemetry::InstrumentationScope">InstrumentationScope</a>  
Information about a library or crate providing instrumentation.

<a href="struct.InstrumentationScopeBuilder.html" class="struct"
title="struct opentelemetry::InstrumentationScopeBuilder">InstrumentationScopeBuilder</a>  
Configuration options for
[InstrumentationScope](struct.InstrumentationScope.html "struct opentelemetry::InstrumentationScope").

<a href="struct.Key.html" class="struct"
title="struct opentelemetry::Key">Key</a>  
The key part of attribute
[KeyValue](struct.KeyValue.html "struct opentelemetry::KeyValue") pairs.

<a href="struct.KeyValue.html" class="struct"
title="struct opentelemetry::KeyValue">KeyValue</a>  
A key-value pair describing an attribute.

<a href="struct.SpanId.html" class="struct"
title="struct opentelemetry::SpanId">SpanId</a>  
An 8-byte value which identifies a given span.

<a href="struct.StringValue.html" class="struct"
title="struct opentelemetry::StringValue">StringValue</a>  
Wrapper for string-like values

<a href="struct.TraceFlags.html" class="struct"
title="struct opentelemetry::TraceFlags">TraceFlags</a>  
Flags that can be set on a `SpanContext`.

<a href="struct.TraceId.html" class="struct"
title="struct opentelemetry::TraceId">TraceId</a>  
A 16-byte value which identifies a given trace.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Array.html" class="enum"
title="enum opentelemetry::Array">Array</a>  
A
[Value::Array](enum.Value.html#variant.Array "variant opentelemetry::Value::Array")
containing homogeneous values.

<a href="enum.Value.html" class="enum"
title="enum opentelemetry::Value">Value</a>  
The value part of attribute
[KeyValue](struct.KeyValue.html "struct opentelemetry::KeyValue") pairs.
