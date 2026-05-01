Crate tracing\_opentelemetry Copy item path

<span class="sub-heading"><a href="../src/tracing_opentelemetry/lib.rs.html#1-212"
class="src">Source</a> </span>

Expand description

# <a href="#tracing-opentelemetry" class="doc-anchor">§</a>Tracing OpenTelemetry

[`tracing`](https://github.com/tokio-rs/tracing) is a framework for
instrumenting Rust programs to collect structured, event-based
diagnostic information. This crate provides a layer that connects spans
from multiple systems into a trace and emits them to
[OpenTelemetry](https://opentelemetry.io)-compatible distributed tracing
systems for processing and visualization.

### <a href="#special-fields" class="doc-anchor">§</a>Special Fields

Fields with an `otel.` prefix are reserved for this crate and have
specific meaning. They are treated as ordinary fields by other layers.
The current special fields are:

-   `otel.name`: Override the span name sent to OpenTelemetry exporters.
    Setting this field is useful if you want to display non-static
    information in your span name.
-   `otel.kind`: Set the span kind to one of the supported OpenTelemetry
    [span
    kinds](../opentelemetry/trace/span/enum.SpanKind.html "enum opentelemetry::trace::span::SpanKind").
    These must be specified as strings such as `"client"` or `"server"`.
    If it is not specified, the span is assumed to be internal.
-   `otel.status_code`: Set the span status code to one of the supported
    OpenTelemetry [span status
    codes](../opentelemetry/trace/span/enum.Status.html "enum opentelemetry::trace::span::Status").
-   `otel.status_description`: Set the span description of the status.
    This should be used only if `otel.status_code` is also set.

### <a href="#semantic-conventions" class="doc-anchor">§</a>Semantic Conventions

OpenTelemetry defines conventional names for attributes of common
operations. These names can be assigned directly as fields, e.g.
`trace_span!("request", "server.port" = 80, "url.full" = ..)`, and they
will be passed through to your configured OpenTelemetry exporter. You
can find the full list of the operations and their expected field names
in the [semantic
conventions](https://github.com/open-telemetry/semantic-conventions)
spec.

### <a href="#stability-status" class="doc-anchor">§</a>Stability Status

The OpenTelemetry tracing specification is stable but the underlying
[opentelemetry
crate](https://github.com/open-telemetry/opentelemetry-rust) is not so
some breaking changes will still occur in this crate as well. Metrics
are not yet fully stable. You can read the specification via the [spec
repository](https://github.com/open-telemetry/opentelemetry-specification).

### <a href="#opentelemetry-logging" class="doc-anchor">§</a>OpenTelemetry Logging

Logging to OpenTelemetry collectors is not supported by this crate, only
traces and metrics are. If you need to export logs through
OpenTelemetry, consider
[`opentelemetry-appender-tracing`](https://crates.io/crates/opentelemetry-appender-tracing).

## <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry::trace::{Tracer, TracerProvider as _};
use tracing::{error, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

// Create a new OpenTelemetry trace pipeline that prints to stdout
let provider = SdkTracerProvider::builder()
    .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
    .build();
let tracer = provider.tracer("readme_example");

// Create a tracing layer with the configured tracer
let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

// Use the tracing subscriber `Registry`, or any other subscriber
// that impls `LookupSpan`
let subscriber = Registry::default().with(telemetry);

// Trace executed code
tracing::subscriber::with_default(subscriber, || {
    // Spans will be sent to the configured OpenTelemetry exporter
    let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    let _enter = root.enter();

    error!("This event will be logged in the root span.");
});
```

## <a href="#feature-flags" class="doc-anchor">§</a>Feature Flags

-   `metrics`: Enables the
    [`MetricsLayer`](struct.MetricsLayer.html "struct tracing_opentelemetry::MetricsLayer")
    type, a
    [layer](../tracing_subscriber/layer/index.html "mod tracing_subscriber::layer")
    that exports OpenTelemetry metrics from specifically-named events.
    This enables the `metrics` feature flag on the `opentelemetry`
    crate. *Enabled by default*.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.FilteredOpenTelemetryLayer.html" class="struct"
title="struct tracing_opentelemetry::FilteredOpenTelemetryLayer">FilteredOpenTelemetryLayer</a>  
A layer wrapping a
[`OpenTelemetryLayer`](struct.OpenTelemetryLayer.html "struct tracing_opentelemetry::OpenTelemetryLayer"),
discarding all events filtered out by a given
[`Filter`](../tracing_subscriber/layer/trait.Filter.html "trait tracing_subscriber::layer::Filter").
This can be built by calling
[`OpenTelemetryLayer::with_counting_event_filter`](struct.OpenTelemetryLayer.html#method.with_counting_event_filter "method tracing_opentelemetry::OpenTelemetryLayer::with_counting_event_filter").

<a href="struct.MetricsLayer.html" class="struct"
title="struct tracing_opentelemetry::MetricsLayer">MetricsLayer</a>  
A layer that publishes metrics via the OpenTelemetry SDK.

<a href="struct.OpenTelemetryLayer.html" class="struct"
title="struct tracing_opentelemetry::OpenTelemetryLayer">OpenTelemetryLayer</a>  
An [OpenTelemetry](https://opentelemetry.io) propagation layer for use
in a project that uses [tracing](https://github.com/tokio-rs/tracing).

<a href="struct.OtelData.html" class="struct"
title="struct tracing_opentelemetry::OtelData">OtelData</a>  
Per-span OpenTelemetry data tracked by this crate.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.SetParentError.html" class="enum"
title="enum tracing_opentelemetry::SetParentError">SetParentError</a>  
An error returned if
[`OpenTelemetrySpanExt::set_parent`](trait.OpenTelemetrySpanExt.html#tymethod.set_parent "method tracing_opentelemetry::OpenTelemetrySpanExt::set_parent")
could not set the parent.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.OpenTelemetrySpanExt.html" class="trait"
title="trait tracing_opentelemetry::OpenTelemetrySpanExt">OpenTelemetrySpanExt</a>  
Utility functions to allow tracing
[`Span`](../tracing/span/struct.Span.html "struct tracing::span::Span")s
to accept and return [OpenTelemetry](https://opentelemetry.io)
[`Context`](../opentelemetry/context/struct.Context.html "struct opentelemetry::context::Context")s.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.get_otel_context.html" class="fn"
title="fn tracing_opentelemetry::get_otel_context">get_otel_context</a>  
Utility functions to allow tracing
[`ExtensionsMut`](../tracing_subscriber/registry/extensions/struct.ExtensionsMut.html "struct tracing_subscriber::registry::extensions::ExtensionsMut")s
to return [OpenTelemetry](https://opentelemetry.io)
[`Context`](../opentelemetry/context/struct.Context.html "struct opentelemetry::context::Context")s.

<a href="fn.layer.html" class="fn"
title="fn tracing_opentelemetry::layer">layer</a>  
Construct a layer to track spans via
[OpenTelemetry](https://opentelemetry.io).
