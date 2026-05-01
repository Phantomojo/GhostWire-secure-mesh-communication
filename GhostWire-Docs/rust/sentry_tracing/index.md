Crate sentry\_tracing Copy item path

<span class="sub-heading"><a href="../src/sentry_tracing/lib.rs.html#1-251" class="src">Source</a>
</span>

Expand description

Support for automatic breadcrumb, event, and trace capturing from
`tracing` events and spans.

The `tracing` crate is supported in four ways:

-   `tracing` events can be captured as Sentry events. These are grouped
    and show up in the Sentry
    [issues](https://docs.sentry.io/product/issues/) page, representing
    high severity issues to be acted upon.
-   `tracing` events can be captured as
    [breadcrumbs](https://docs.sentry.io/product/issues/issue-details/breadcrumbs/).
    Breadcrumbs create a trail of what happened prior to an event, and
    are therefore sent only when an event is captured, either manually
    through e.g. `sentry::capture_message` or through integrations (e.g.
    the panic integration is enabled (default) and a panic happens).
-   `tracing` events can be captured as traditional [structured
    logs](https://docs.sentry.io/product/explore/logs/). The `tracing`
    fields are captured as attributes on the logs, which can be queried
    in the Logs explorer. (Available on crate feature `logs`)
-   `tracing` spans can be captured as Sentry spans. These can be used
    to provide more contextual information for errors, diagnose
    [performance
    issues](https://docs.sentry.io/product/insights/overview/), and
    capture additional attributes to aggregate and compute
    [metrics](https://docs.sentry.io/product/explore/trace-explorer/).

By default, events above `Info` are recorded as breadcrumbs, events
above `Error` are captured as error events, and spans above `Info` are
recorded as spans.

# <a href="#configuration" class="doc-anchor">§</a>Configuration

To fully enable the tracing integration, set the traces sample rate and
add a layer to the tracing subscriber:

``` {.rust .rust-example-rendered}
use tracing_subscriber::prelude::*;

let _guard = sentry::init(sentry::ClientOptions {
    // Enable capturing of traces; set this a to lower value in production:
    traces_sample_rate: 1.0,
    ..sentry::ClientOptions::default()
});

// Register the Sentry tracing layer to capture breadcrumbs, events, and spans:
tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(sentry::integrations::tracing::layer())
    .init();
```

You can customize the behavior of the layer by providing an explicit
event filter, to customize which events are captured by Sentry and the
data type they are mapped to. Similarly, you can provide a span filter
to customize which spans are captured by Sentry.

``` {.rust .rust-example-rendered}
use sentry::integrations::tracing::EventFilter;
use tracing_subscriber::prelude::*;

let sentry_layer = sentry::integrations::tracing::layer()
    .event_filter(|md| match *md.level() {
        tracing::Level::ERROR => EventFilter::Event,
        _ => EventFilter::Ignore,
    })
    .span_filter(|md| matches!(*md.level(), tracing::Level::ERROR | tracing::Level::WARN));

tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(sentry_layer)
    .init();
```

In addition, a custom event mapper can be provided, to fully customize
if and how `tracing` events are converted to Sentry data.

Note that if both an event mapper and event filter are set, the mapper
takes precedence, thus the filter has no effect.

# <a href="#capturing-breadcrumbs" class="doc-anchor">§</a>Capturing breadcrumbs

Tracing events automatically create breadcrumbs that are attached to the
current scope in Sentry. They show up on errors and transactions
captured within this scope as shown in the examples below.

Fields passed to the event macro are automatically tracked as structured
data in Sentry. For breadcrumbs, they are shown directly with the
breadcrumb message. For other types of data, read below.

``` {.rust .rust-example-rendered}
for i in 0..10 {
    tracing::debug!(number = i, "Generates a breadcrumb");
}
```

# <a href="#capturing-logs" class="doc-anchor">§</a>Capturing logs

Tracing events can be captured as traditional structured logs in Sentry.
This is gated by the `logs` feature flag and requires setting up a
custom Event filter/mapper to capture logs. You also need to pass
`enable_logs: true` in your `sentry::init` call.

``` {.rust .rust-example-rendered}
// assuming `tracing::Level::INFO => EventFilter::Log` in your `event_filter`
for i in 0..10 {
    tracing::info!(number = i, my.key = "val", my.num = 42, "This is a log");
}
```

The fields of a `tracing` event are captured as attributes of the log.
Logs can be viewed and queried in the Logs explorer based on message and
attributes. Fields containing dots will be displayed as nested under
their common prefix in the UI.

# <a href="#tracking-errors" class="doc-anchor">§</a>Tracking Errors

The easiest way to emit errors is by logging an event with `ERROR`
level. This will create a grouped issue in Sentry. To add custom
information, prepend the message with fields. It is also possible to add
Sentry tags if a field is prefixed with `"tags."`

``` {.rust .rust-example-rendered}
tracing::error!(
    field = "value",                  // will become a context field
    tags.custom = "value",            // will become a tag in Sentry
    "this is an error with a custom tag",
);
```

To track [error
structs](https://doc.rust-lang.org/1.93.1/core/error/trait.Error.html "trait core::error::Error"),
assign a reference to error trait object as field in one of the logging
macros. By convention, it is recommended to use the `ERROR` level and
assign it to a field called `error`, although the integration will also
work with all other levels and field names.

All other fields passed to the macro are captured in a custom “Tracing
Fields” context in Sentry.

``` {.rust .rust-example-rendered}
use std::error::Error;
use std::io;

let custom_error = io::Error::new(io::ErrorKind::Other, "oh no");
tracing::error!(error = &custom_error as &dyn Error);
```

It is also possible to combine error messages with error structs. In
Sentry, this creates issues grouped by the message and location of the
error log, and adds the passed error as nested source.

``` {.rust .rust-example-rendered}
use std::error::Error;
use std::io;

let custom_error = io::Error::new(io::ErrorKind::Other, "oh no");
tracing::error!(error = &custom_error as &dyn Error, "my operation failed");
```

# <a href="#sending-multiple-items-to-sentry" class="doc-anchor">§</a>Sending multiple items to Sentry

To map a `tracing` event to multiple items in Sentry, you can combine
multiple event filters using the bitwise or operator:

``` {.rust .rust-example-rendered}
use sentry::integrations::tracing::EventFilter;
use tracing_subscriber::prelude::*;

let sentry_layer = sentry::integrations::tracing::layer()
    .event_filter(|md| match *md.level() {
        tracing::Level::ERROR => EventFilter::Event | EventFilter::Log,
        tracing::Level::TRACE => EventFilter::Ignore,
        _ => EventFilter::Log,
    })
    .span_filter(|md| matches!(*md.level(), tracing::Level::ERROR | tracing::Level::WARN));

tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(sentry_layer)
    .init();
```

If you’re using a custom event mapper instead of an event filter, use
`EventMapping::Combined`.

# <a href="#tracing-spans" class="doc-anchor">§</a>Tracing Spans

The integration automatically tracks `tracing` spans as spans in Sentry.
A convenient way to do this is with the `#[instrument]` attribute macro,
which creates a span/transaction for the function in Sentry.

Function arguments are added as context fields automatically, which can
be configured through attribute arguments. Refer to documentation of the
macro for more information.

``` {.rust .rust-example-rendered}
use std::time::Duration;

use tracing_subscriber::prelude::*;

// Functions instrumented by tracing automatically
// create spans/transactions around their execution.
#[tracing::instrument]
async fn outer() {
    for i in 0..10 {
        inner(i).await;
    }
}

// This creates spans inside the outer transaction, unless called directly.
#[tracing::instrument]
async fn inner(i: u32) {
    // Also works, since log events are ingested by the tracing system
    tracing::debug!(number = i, "Generates a breadcrumb");

    tokio::time::sleep(Duration::from_millis(100)).await;
}
```

By default, the name of the span sent to Sentry matches the name of the
`tracing` span, which is the name of the function when using
`tracing::instrument`, or the name passed to the `tracing::<level>_span`
macros.

By default, the `op` of the span sent to Sentry is `default`.

## <a href="#special-span-fields" class="doc-anchor">§</a>Special Span Fields

Some fields on spans are treated specially by the Sentry tracing
integration:

-   `sentry.name`: overrides the span name sent to Sentry. This is
    useful to customize the span name when using
    `#[tracing::instrument]`, or to update it retroactively (using
    `span.record`) after the span has been created.
-   `sentry.op`: overrides the span `op` sent to Sentry.
-   `sentry.trace`: in Sentry, the `sentry-trace` header is sent with
    HTTP requests to achieve distributed tracing. If the value of this
    field is set to the value of a valid `sentry-trace` header, which
    other Sentry SDKs send automatically with outgoing requests, then
    the SDK will continue the trace using the given distributed tracing
    information. This is useful to achieve distributed tracing at
    service boundaries by using only the `tracing` API. Note that
    `sentry.trace` will only be effective on span creation (it cannot be
    applied retroactively) and requires the span it’s applied to to be a
    root span, i.e. no span should active upon its creation.

Example:

``` {.rust .rust-example-rendered}
#[tracing::instrument(skip_all, fields(
    sentry.name = "GET /payments",
    sentry.op = "http.server",
    sentry.trace = headers.get("sentry-trace").unwrap_or(&"".to_owned()),
))]
async fn handle_request(headers: std::collections::HashMap<String, String>) {
    // ...
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.CombinedEventMapping.html" class="struct"
title="struct sentry_tracing::CombinedEventMapping">CombinedEventMapping</a>  
A list of event mappings.

<a href="struct.EventFilter.html" class="struct"
title="struct sentry_tracing::EventFilter">EventFilter</a>  
The action that Sentry should perform for a given
[`Event`](../tracing_core/event/struct.Event.html "struct tracing_core::event::Event")

<a href="struct.SentryLayer.html" class="struct"
title="struct sentry_tracing::SentryLayer">SentryLayer</a>  
Provides a tracing layer that dispatches events to sentry

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.EventMapping.html" class="enum"
title="enum sentry_tracing::EventMapping">EventMapping</a>  
The type of data Sentry should ingest for an
[`Event`](../tracing_core/event/struct.Event.html "struct tracing_core::event::Event").

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.breadcrumb_from_event.html" class="fn"
title="fn sentry_tracing::breadcrumb_from_event">breadcrumb_from_event</a>  
Creates a
[`Breadcrumb`](../sentry_types/protocol/v7/struct.Breadcrumb.html "struct sentry_types::protocol::v7::Breadcrumb")
from a given
[`tracing_core::Event`](../tracing_core/event/struct.Event.html "struct tracing_core::event::Event").

<a href="fn.default_event_filter.html" class="fn"
title="fn sentry_tracing::default_event_filter">default_event_filter</a>  
The default event filter.

<a href="fn.default_span_filter.html" class="fn"
title="fn sentry_tracing::default_span_filter">default_span_filter</a>  
The default span filter.

<a href="fn.event_from_event.html" class="fn"
title="fn sentry_tracing::event_from_event">event_from_event</a>  
Creates an
[`Event`](../sentry_types/protocol/v7/struct.Event.html "struct sentry_types::protocol::v7::Event")
(possibly carrying exceptions) from a given
[`tracing_core::Event`](../tracing_core/event/struct.Event.html "struct tracing_core::event::Event").

<a href="fn.layer.html" class="fn"
title="fn sentry_tracing::layer">layer</a>  
Creates a default Sentry layer
