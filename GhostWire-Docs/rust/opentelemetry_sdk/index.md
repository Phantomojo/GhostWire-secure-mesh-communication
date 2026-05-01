Crate opentelemetry\_sdk Copy item path

<span class="sub-heading"><a href="../src/opentelemetry_sdk/lib.rs.html#1-169"
class="src">Source</a> </span>

Expand description

Implements the
[`SDK`](https://opentelemetry.io/docs/specs/otel/overview/#sdk)
component of
[OpenTelemetry](https://opentelemetry.io/docs/what-is-opentelemetry/).

*[Supported Rust Versions](#supported-rust-versions)*

# <a href="#getting-started" class="doc-anchor">§</a>Getting Started

``` {.rust .rust-example-rendered}
use opentelemetry::{global, trace::{Tracer, TracerProvider}};
use opentelemetry_sdk::trace::SdkTracerProvider;

fn main() {
    // Choose an exporter like `opentelemetry_stdout::SpanExporter`
    let exporter = new_exporter();

    // Create a new trace pipeline that prints to stdout
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(exporter)
        .build();
    let tracer = provider.tracer("readme_example");

    tracer.in_span("doing_work", |cx| {
        // Traced app logic here...
    });

    // Shutdown trace pipeline
    provider.shutdown().expect("TracerProvider should shutdown successfully")
}
```

See the
[examples](https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples)
directory for different integration patterns.

See the API
[`trace`](https://docs.rs/opentelemetry/latest/opentelemetry/trace/index.html)
module docs for more information on creating and managing spans.

# <a href="#metrics" class="doc-anchor">§</a>Metrics

### <a href="#creating-instruments-and-recording-measurements"
class="doc-anchor">§</a>Creating instruments and recording measurements

``` {.rust .rust-example-rendered}
use opentelemetry::{global, KeyValue};

// get a meter from a provider
let meter = global::meter("my_service");

// create an instrument
let counter = meter.u64_counter("my_counter").build();

// record a measurement
counter.add(1, &[KeyValue::new("http.client_ip", "83.164.160.102")]);
```

See the
[examples](https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples)
directory for different integration patterns.

See the API
[`metrics`](https://docs.rs/opentelemetry/latest/opentelemetry/metrics/index.html)
module docs for more information on creating and managing instruments.

## <a href="#crate-feature-flags" class="doc-anchor">§</a>Crate Feature Flags

The following feature flags can used to control the telemetry signals to
use:

-   `trace`: Includes the trace SDK (enabled by default).
-   `metrics`: Includes the metrics SDK.
-   `logs`: Includes the logs SDK.

For `trace` the following feature flags are available:

-   `jaeger_remote_sampler`: Enables the [Jaeger remote
    sampler](https://www.jaegertracing.io/docs/1.53/sampling/).

For `logs` the following feature flags are available:

-   `spec_unstable_logs_enabled`: control the log level

Support for recording and exporting telemetry asynchronously and perform
metrics aggregation can be added via the following flags:

-   `experimental_async_runtime`: Enables the experimental `Runtime`
    trait and related functionality.
-   `rt-tokio`: Spawn telemetry tasks using
    [tokio](https://crates.io/crates/tokio)’s multi-thread runtime.
-   `rt-tokio-current-thread`: Spawn telemetry tasks on a separate
    runtime so that the main runtime won’t be blocked.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use error::`<a href="error/trait.ExportError.html" class="trait"
title="trait opentelemetry_sdk::error::ExportError"><code>ExportError</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="error/index.html" class="mod"
title="mod opentelemetry_sdk::error">error</a>  
Wrapper for error from trace, logs and metrics part of open telemetry.

<a href="logs/index.html" class="mod"
title="mod opentelemetry_sdk::logs">logs</a>  
OpenTelemetry Log SDK

<a href="metrics/index.html" class="mod"
title="mod opentelemetry_sdk::metrics">metrics</a>  
The crust of the OpenTelemetry metrics SDK.

<a href="propagation/index.html" class="mod"
title="mod opentelemetry_sdk::propagation">propagation</a>  
OpenTelemetry Propagators

<a href="resource/index.html" class="mod"
title="mod opentelemetry_sdk::resource">resource</a>  
Representations of entities producing telemetry.

<a href="trace/index.html" class="mod"
title="mod opentelemetry_sdk::trace">trace</a>  
OpenTelemetry Trace SDK

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Resource.html" class="struct"
title="struct opentelemetry_sdk::Resource">Resource</a>  
An immutable representation of the entity producing telemetry as
attributes. Utilizes `Arc` for efficient sharing and cloning.
