Crate tracing Copy item path

<span class="sub-heading"><a href="../src/tracing/lib.rs.html#1-1205" class="src">Source</a>
</span>

Expand description

A scoped, structured logging and diagnostics system.

# <a href="#overview" class="doc-anchor">§</a>Overview

`tracing` is a framework for instrumenting Rust programs to collect
structured, event-based diagnostic information.

In asynchronous systems like Tokio, interpreting traditional log
messages can often be quite challenging. Since individual tasks are
multiplexed on the same thread, associated events and log lines are
intermixed making it difficult to trace the logic flow. `tracing`
expands upon logging-style diagnostics by allowing libraries and
applications to record structured events with additional information
about *temporality* and *causality* — unlike a log message, a span in
`tracing` has a beginning and end time, may be entered and exited by the
flow of execution, and may exist within a nested tree of similar spans.
In addition, `tracing` spans are *structured*, with the ability to
record typed data as well as textual messages.

The `tracing` crate provides the APIs necessary for instrumenting
libraries and applications to emit trace data.

*Compiler support: [requires `rustc` 1.65+](#supported-rust-versions)*

# <a href="#core-concepts" class="doc-anchor">§</a>Core Concepts

The core of `tracing`’s API is composed of *spans*, *events* and
*subscribers*. We’ll cover these in turn.

## <a href="#spans" class="doc-anchor">§</a>Spans

To record the flow of execution through a program, `tracing` introduces
the concept of [spans](span/index.html "mod tracing::span"). Unlike a
log line that represents a *moment in time*, a span represents a *period
of time* with a beginning and an end. When a program begins executing in
a context or performing a unit of work, it *enters* that context’s span,
and when it stops executing in that context, it *exits* the span. The
span in which a thread is currently executing is referred to as that
thread’s *current* span.

For example:

``` {.rust .rust-example-rendered}
use tracing::{span, Level};
let span = span!(Level::TRACE, "my_span");
// `enter` returns a RAII guard which, when dropped, exits the span. this
// indicates that we are in the span for the current lexical scope.
let _enter = span.enter();
// perform some work in the context of `my_span`...
```

The [`span` module](span/index.html "mod tracing::span")’s documentation
provides further details on how to use spans.

``` {.compile_fail style="white-space:normal;font:inherit;"}
Warning: In asynchronous code that uses async/await syntax,
Span::enter may produce incorrect traces if the returned drop
guard is held across an await point. See
the method documentation for details.
```

## <a href="#events" class="doc-anchor">§</a>Events

An [`Event`](struct.Event.html "struct tracing::Event") represents a
*moment* in time. It signifies something that happened while a trace was
being recorded. `Event`s are comparable to the log records emitted by
unstructured logging code, but unlike a typical log line, an `Event` may
occur within the context of a span.

For example:

``` {.rust .rust-example-rendered}
use tracing::{event, span, Level};

// records an event outside of any span context:
event!(Level::INFO, "something happened");

let span = span!(Level::INFO, "my_span");
let _guard = span.enter();

// records an event within "my_span".
event!(Level::DEBUG, "something happened inside my_span");
```

In general, events should be used to represent points in time *within* a
span — a request returned with a given status code, *n* new items were
taken from a queue, and so on.

The [`Event` struct](struct.Event.html "struct tracing::Event")
documentation provides further details on using events.

## <a href="#subscribers" class="doc-anchor">§</a>Subscribers

As `Span`s and `Event`s occur, they are recorded or aggregated by
implementations of the
[`Subscriber`](trait.Subscriber.html "trait tracing::Subscriber") trait.
`Subscriber`s are notified when an `Event` takes place and when a `Span`
is entered or exited. These notifications are represented by the
following `Subscriber` trait methods:

-   [`event`](trait.Subscriber.html#tymethod.event "method tracing::Subscriber::event"),
    called when an `Event` takes place,
-   [`enter`](trait.Subscriber.html#tymethod.enter "method tracing::Subscriber::enter"),
    called when execution enters a `Span`,
-   [`exit`](trait.Subscriber.html#tymethod.exit "method tracing::Subscriber::exit"),
    called when execution exits a `Span`

In addition, subscribers may implement the
[`enabled`](trait.Subscriber.html#tymethod.enabled "method tracing::Subscriber::enabled")
function to *filter* the notifications they receive based on
[metadata](struct.Metadata.html "struct tracing::Metadata") describing
each `Span` or `Event`. If a call to `Subscriber::enabled` returns
`false` for a given set of metadata, that `Subscriber` will *not* be
notified about the corresponding `Span` or `Event`. For performance
reasons, if no currently active subscribers express interest in a given
set of metadata by returning `true`, then the corresponding `Span` or
`Event` will never be constructed.

# <a href="#usage" class="doc-anchor">§</a>Usage

First, add this to your `Cargo.toml`:

``` language-toml
[dependencies]
tracing = "0.1"
```

## <a href="#recording-spans-and-events" class="doc-anchor">§</a>Recording Spans and Events

Spans and events are recorded using macros.

### <a href="#spans-1" class="doc-anchor">§</a>Spans

The [`span!`](macro.span.html "macro tracing::span") macro expands to a
[`Span` struct](struct.Span.html "struct tracing::Span") which is used
to record a span. The
[`Span::enter`](struct.Span.html#method.enter "method tracing::Span::enter")
method on that struct records that the span has been entered, and
returns a
[RAII](https://github.com/rust-unofficial/patterns/blob/main/src/patterns/behavioural/RAII.md)
guard object, which will exit the span when dropped.

For example:

``` {.rust .rust-example-rendered}
use tracing::{span, Level};
// Construct a new span named "my span" with trace log level.
let span = span!(Level::TRACE, "my span");

// Enter the span, returning a guard object.
let _enter = span.enter();

// Any trace events that occur before the guard is dropped will occur
// within the span.

// Dropping the guard will exit the span.
```

The
[`#[instrument]`](https://docs.rs/tracing-attributes/latest/tracing_attributes/attr.instrument.html)
attribute provides an easy way to add `tracing` spans to functions. A
function annotated with `#[instrument]` will create and enter a span
with that function’s name every time the function is called, with
arguments to that function will be recorded as fields using
`fmt::Debug`.

For example:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use tracing::{Level, event, instrument};

#[instrument]
pub fn my_function(my_arg: usize) {
    // This event will be recorded inside a span named `my_function` with the
    // field `my_arg`.
    event!(Level::INFO, "inside my_function!");
    // ...
}
```

For functions which don’t have built-in tracing support and can’t have
the `#[instrument]` attribute applied (such as from an external crate),
the [`Span` struct](struct.Span.html "struct tracing::Span") has a
[`in_scope()`
method](struct.Span.html#method.in_scope "method tracing::Span::in_scope")
which can be used to easily wrap synchronous code in a span.

For example:

``` {.rust .rust-example-rendered}
use tracing::info_span;

let json = info_span!("json.parse").in_scope(|| serde_json::from_slice(&buf))?;
```

You can find more examples showing how to use this crate
[here](https://github.com/tokio-rs/tracing/tree/main/examples).

### <a href="#events-1" class="doc-anchor">§</a>Events

[`Event`](struct.Event.html "struct tracing::Event")s are recorded using
the [`event!`](macro.event.html "macro tracing::event") macro:

``` {.rust .rust-example-rendered}
use tracing::{event, Level};
event!(Level::INFO, "something has happened!");
```

## <a href="#using-the-macros" class="doc-anchor">§</a>Using the Macros

The [`span!`](macro.span.html "macro tracing::span") and
[`event!`](macro.event.html "macro tracing::event") macros as well as
the `#[instrument]` attribute use fairly similar syntax, with some
exceptions.

### <a href="#configuring-attributes" class="doc-anchor">§</a>Configuring Attributes

Both macros require a
[`Level`](struct.Level.html "struct tracing::Level") specifying the
verbosity of the span or event. Optionally, the,
[target](struct.Metadata.html#method.target "method tracing::Metadata::target")
and [parent
span](span/struct.Attributes.html#method.parent "method tracing::span::Attributes::parent")
may be overridden. If the target and parent span are not overridden,
they will default to the module path where the macro was invoked and the
current span (as determined by the subscriber), respectively.

For example:

``` {.rust .rust-example-rendered}
span!(target: "app_spans", Level::TRACE, "my span");
event!(target: "app_events", Level::INFO, "something has happened!");
```

``` {.rust .rust-example-rendered}
let span = span!(Level::TRACE, "my span");
event!(parent: &span, Level::INFO, "something has happened!");
```

The span macros also take a string literal after the level, to set the
name of the span (as above). In the case of the event macros, the name
of the event can be overridden (the default is `event file:line`) using
the `name:` specifier.

``` {.rust .rust-example-rendered}
span!(Level::TRACE, "my span");
event!(name: "some_info", Level::INFO, "something has happened!");
```

### <a href="#recording-fields" class="doc-anchor">§</a>Recording Fields

Structured fields on spans and events are specified using the syntax
`field_name = field_value`. Fields are separated by commas.

``` {.rust .rust-example-rendered}
// records an event with two fields:
//  - "answer", with the value 42
//  - "question", with the value "life, the universe and everything"
event!(Level::INFO, answer = 42, question = "life, the universe, and everything");
```

As shorthand, local variables may be used as field values without an
assignment, similar to [struct
initializers](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name).
For example:

``` {.rust .rust-example-rendered}
let user = "ferris";

span!(Level::TRACE, "login", user);
// is equivalent to:
span!(Level::TRACE, "login", user = user);
```

Field names can include dots, but should not be terminated by them:

``` {.rust .rust-example-rendered}
let user = "ferris";
let email = "ferris@rust-lang.org";
span!(Level::TRACE, "login", user, user.email = email);
```

Since field names can include dots, fields on local structs can be used
using the local variable shorthand:

``` {.rust .rust-example-rendered}
let user = User {
    name: "ferris",
    email: "ferris@rust-lang.org",
};
// the span will have the fields `user.name = "ferris"` and
// `user.email = "ferris@rust-lang.org"`.
span!(Level::TRACE, "login", user.name, user.email);
```

Fields with names that are not Rust identifiers, or with names that are
Rust reserved words, may be created using quoted string literals.
However, this may not be used with the local variable shorthand.

``` {.rust .rust-example-rendered}
// records an event with fields whose names are not Rust identifiers
//  - "guid:x-request-id", containing a `:`, with the value "abcdef"
//  - "type", which is a reserved word, with the value "request"
span!(Level::TRACE, "api", "guid:x-request-id" = "abcdef", "type" = "request");
```

Constant expressions can also be used as field names. Constants must be
enclosed in curly braces (`{}`) to indicate that the *value* of the
constant is to be used as the field name, rather than the constant’s
name. For example:

``` {.rust .rust-example-rendered}
const RESOURCE_NAME: &str = "foo";
// this span will have the field `foo = "some_id"`
span!(Level::TRACE, "get", { RESOURCE_NAME } = "some_id");
```

The `?` sigil is shorthand that specifies a field should be recorded
using its
[`fmt::Debug`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Debug.html "trait core::fmt::Debug")
implementation:

``` {.rust .rust-example-rendered}
#[derive(Debug)]
struct MyStruct {
    field: &'static str,
}

let my_struct = MyStruct {
    field: "Hello world!"
};

// `my_struct` will be recorded using its `fmt::Debug` implementation.
event!(Level::TRACE, greeting = ?my_struct);
// is equivalent to:
event!(Level::TRACE, greeting = tracing::field::debug(&my_struct));
```

The `%` sigil operates similarly, but indicates that the value should be
recorded using its
[`fmt::Display`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Display.html "trait core::fmt::Display")
implementation:

``` {.rust .rust-example-rendered}
// `my_struct.field` will be recorded using its `fmt::Display` implementation.
event!(Level::TRACE, greeting = %my_struct.field);
// is equivalent to:
event!(Level::TRACE, greeting = tracing::field::display(&my_struct.field));
```

The `%` and `?` sigils may also be used with local variable shorthand:

``` {.rust .rust-example-rendered}
// `my_struct.field` will be recorded using its `fmt::Display` implementation.
event!(Level::TRACE, %my_struct.field);
```

Additionally, a span may declare fields with the special value
[`Empty`](field/struct.Empty.html "struct tracing::field::Empty"), which
indicates that that the value for that field does not currently exist
but may be recorded later. For example:

``` {.rust .rust-example-rendered}
use tracing::{trace_span, field};

// Create a span with two fields: `greeting`, with the value "hello world", and
// `parting`, without a value.
let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);

// ...

// Now, record a value for parting as well.
span.record("parting", &"goodbye world!");
```

Finally, events may also include human-readable messages, in the form of
a [format
string](https://doc.rust-lang.org/1.93.1/alloc/fmt/index.html#usage "mod alloc::fmt")
and (optional) arguments, **after** the event’s key-value fields. If a
format string and arguments are provided, they will implicitly create a
new field named `message` whose value is the provided set of format
arguments.

For example:

``` {.rust .rust-example-rendered}
let question = "the ultimate question of life, the universe, and everything";
let answer = 42;
// records an event with the following fields:
// - `question.answer` with the value 42,
// - `question.tricky` with the value `true`,
// - "message", with the value "the answer to the ultimate question of life, the
//    universe, and everything is 42."
event!(
    Level::DEBUG,
    question.answer = answer,
    question.tricky = true,
    "the answer to {} is {}.", question, answer
);
```

Specifying a formatted message in this manner does not allocate by
default.

### <a href="#shorthand-macros" class="doc-anchor">§</a>Shorthand Macros

`tracing` also offers a number of macros with preset verbosity levels.
The [`trace!`](macro.trace.html "macro tracing::trace"),
[`debug!`](macro.debug.html "macro tracing::debug"),
[`info!`](macro.info.html "macro tracing::info"),
[`warn!`](macro.warn.html "macro tracing::warn"), and
[`error!`](macro.error.html "macro tracing::error") behave similarly to
the [`event!`](macro.event.html "macro tracing::event") macro, but with
the [`Level`](struct.Level.html "struct tracing::Level") argument
already specified, while the corresponding
[`trace_span!`](macro.trace_span.html "macro tracing::trace_span"),
[`debug_span!`](macro.debug_span.html "macro tracing::debug_span"),
[`info_span!`](macro.info_span.html "macro tracing::info_span"),
[`warn_span!`](macro.warn_span.html "macro tracing::warn_span"), and
[`error_span!`](macro.error_span.html "macro tracing::error_span")
macros are the same, but for the
[`span!`](macro.span.html "macro tracing::span") macro.

These are intended both as a shorthand, and for compatibility with the
[`log`](https://docs.rs/log/0.4.6/log/) crate (see the next section).

### <a href="#for-log-users" class="doc-anchor">§</a>For `log` Users

Users of the [`log`](https://docs.rs/log/0.4.6/log/) crate should note
that `tracing` exposes a set of macros for creating `Event`s (`trace!`,
`debug!`, `info!`, `warn!`, and `error!`) which may be invoked with the
same syntax as the similarly-named macros from the `log` crate. Often,
the process of converting a project to use `tracing` can begin with a
simple drop-in replacement.

Let’s consider the `log` crate’s yak-shaving example:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use std::{error::Error, io};
use tracing::{debug, error, info, span, warn, Level};

// the `#[tracing::instrument]` attribute creates and enters a span
// every time the instrumented function is called. The span is named after the
// the function or method. Parameters passed to the function are recorded as fields.
#[tracing::instrument]
pub fn shave(yak: usize) -> Result<(), Box<dyn Error + 'static>> {
    // this creates an event at the DEBUG level with two fields:
    // - `excitement`, with the key "excitement" and the value "yay!"
    // - `message`, with the key "message" and the value "hello! I'm gonna shave a yak."
    //
    // unlike other fields, `message`'s shorthand initialization is just the string itself.
    debug!(excitement = "yay!", "hello! I'm gonna shave a yak.");
    if yak == 3 {
        warn!("could not locate yak!");
        // note that this is intended to demonstrate `tracing`'s features, not idiomatic
        // error handling! in a library or application, you should consider returning
        // a dedicated `YakError`. libraries like snafu or thiserror make this easy.
        return Err(io::Error::new(io::ErrorKind::Other, "shaving yak failed!").into());
    } else {
        debug!("yak shaved successfully");
    }
    Ok(())
}

pub fn shave_all(yaks: usize) -> usize {
    // Constructs a new span named "shaving_yaks" at the TRACE level,
    // and a field whose key is "yaks". This is equivalent to writing:
    //
    // let span = span!(Level::TRACE, "shaving_yaks", yaks = yaks);
    //
    // local variables (`yaks`) can be used as field values
    // without an assignment, similar to struct initializers.
    let _span = span!(Level::TRACE, "shaving_yaks", yaks).entered();

    info!("shaving yaks");

    let mut yaks_shaved = 0;
    for yak in 1..=yaks {
        let res = shave(yak);
        debug!(yak, shaved = res.is_ok());

        if let Err(ref error) = res {
            // Like spans, events can also use the field initialization shorthand.
            // In this instance, `yak` is the field being initalized.
            error!(yak, error = error.as_ref(), "failed to shave yak!");
        } else {
            yaks_shaved += 1;
        }
        debug!(yaks_shaved);
    }

    yaks_shaved
}
```

## <a href="#in-libraries" class="doc-anchor">§</a>In libraries

Libraries should link only to the `tracing` crate, and use the provided
macros to record whatever information will be useful to downstream
consumers.

## <a href="#in-executables" class="doc-anchor">§</a>In executables

In order to record trace events, executables have to use a `Subscriber`
implementation compatible with `tracing`. A `Subscriber` implements a
way of collecting trace data, such as by logging it to standard output.

This library does not contain any `Subscriber` implementations; these
are provided by [other crates](#related-crates).

The simplest way to use a subscriber is to call the
[`set_global_default`](subscriber/fn.set_global_default.html "fn tracing::subscriber::set_global_default")
function:

``` {.rust .rust-example-rendered}
extern crate tracing;

let my_subscriber = FooSubscriber::new();
tracing::subscriber::set_global_default(my_subscriber)
    .expect("setting tracing default failed");
```

``` {.compile_fail style="white-space:normal;font:inherit;"}
     Warning: In general, libraries should not call
     set_global_default()! Doing so will cause conflicts when
     executables that depend on the library try to set the default later.
 
```

This subscriber will be used as the default in all threads for the
remainder of the duration of the program, similar to setting the logger
in the `log` crate.

In addition, the default subscriber can be set through using the
[`with_default`](subscriber/fn.with_default.html "fn tracing::subscriber::with_default")
function. This follows the `tokio` pattern of using closures to
represent executing code in a context that is exited at the end of the
closure. For example:

``` {.rust .rust-example-rendered}
let my_subscriber = FooSubscriber::new();
tracing::subscriber::with_default(my_subscriber, || {
    // Any trace events generated in this closure or by functions it calls
    // will be collected by `my_subscriber`.
})
```

This approach allows trace data to be collected by multiple subscribers
within different contexts in the program. Note that the override only
applies to the currently executing thread; other threads will not see
the change from with\_default.

Any trace events generated outside the context of a subscriber will not
be collected.

Once a subscriber has been set, instrumentation points may be added to
the executable using the `tracing` crate’s macros.

## <a href="#log-compatibility" class="doc-anchor">§</a>`log` Compatibility

The [`log`](https://docs.rs/log/0.4.6/log/) crate provides a simple,
lightweight logging facade for Rust. While `tracing` builds upon `log`’s
foundation with richer structured diagnostic data, `log`’s simplicity
and ubiquity make it the “lowest common denominator” for text-based
logging in Rust — a vast majority of Rust libraries and applications
either emit or consume `log` records. Therefore, `tracing` provides
multiple forms of interoperability with `log`: `tracing` instrumentation
can emit `log` records, and a compatibility layer enables `tracing`
[`Subscriber`](trait.Subscriber.html "trait tracing::Subscriber")s to
consume `log` records as `tracing`
[`Event`](struct.Event.html "struct tracing::Event")s.

### <a href="#emitting-log-records" class="doc-anchor">§</a>Emitting `log` Records

This crate provides two feature flags, “log” and “log-always”, which
will cause [spans](span/index.html "mod tracing::span") and
[events](struct.Event.html "struct tracing::Event") to emit `log`
records. When the “log” feature is enabled, if no `tracing` `Subscriber`
is active, invoking an event macro or creating a span with fields will
emit a `log` record. This is intended primarily for use in libraries
which wish to emit diagnostics that can be consumed by applications
using `tracing` *or* `log`, without paying the additional overhead of
emitting both forms of diagnostics when `tracing` is in use.

Enabling the “log-always” feature will cause `log` records to be emitted
even if a `tracing` `Subscriber` *is* set. This is intended to be used
in applications where a `log` `Logger` is being used to record a textual
log, and `tracing` is used only to record other forms of diagnostics
(such as metrics, profiling, or distributed tracing data). Unlike the
“log” feature, libraries generally should **not** enable the
“log-always” feature, as doing so will prevent applications from being
able to opt out of the `log` records.

See [here](#crate-feature-flags) for more details on this crate’s
feature flags.

The generated `log` records’ messages will be a string representation of
the span or event’s fields, and all additional information recorded by
`log` (target, verbosity level, module path, file, and line number) will
also be populated. Additionally, `log` records are also generated when
spans are entered, exited, and closed. Since these additional span
lifecycle logs have the potential to be very verbose, and don’t include
additional fields, they will always be emitted at the `Trace` level,
rather than inheriting the level of the span that generated them.
Furthermore, they are categorized under a separate `log` target,
“tracing::span” (and its sub-target, “tracing::span::active”, for the
logs on entering and exiting a span), which may be enabled or disabled
separately from other `log` records emitted by `tracing`.

### <a href="#consuming-log-records" class="doc-anchor">§</a>Consuming `log` Records

The [`tracing-log`](https://crates.io/crates/tracing-log) crate provides
a compatibility layer which allows a `tracing`
[`Subscriber`](trait.Subscriber.html "trait tracing::Subscriber") to
consume `log` records as though they were `tracing`
[events](struct.Event.html "struct tracing::Event"). This allows
applications using `tracing` to record the logs emitted by dependencies
using `log` as events within the context of the application’s trace
tree. See [that crate’s
documentation](https://docs.rs/tracing-log/latest/tracing_log/#convert-log-records-to-tracing-events)
for details.

## <a href="#related-crates" class="doc-anchor">§</a>Related Crates

In addition to `tracing` and `tracing-core`, the
[`tokio-rs/tracing`](https://github.com/tokio-rs/tracing) repository
contains several additional crates designed to be used with the
`tracing` ecosystem. This includes a collection of `Subscriber`
implementations, as well as utility and adapter crates to assist in
writing `Subscriber`s and instrumenting applications.

In particular, the following crates are likely to be of interest:

-   [`tracing-futures`](https://crates.io/crates/tracing-futures)
    provides a compatibility layer with the `futures` crate, allowing
    spans to be attached to `Future`s, `Stream`s, and `Executor`s.
-   [`tracing-subscriber`](https://crates.io/crates/tracing-subscriber)
    provides `Subscriber` implementations and utilities for working with
    `Subscriber`s. This includes a
    [`FmtSubscriber`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/struct.Subscriber.html)
    `FmtSubscriber` for logging formatted trace data to stdout, with
    similar filtering and formatting to the
    [`env_logger`](https://crates.io/crates/env_logger) crate.
-   [`tracing-log`](https://crates.io/crates/tracing-log) provides a
    compatibility layer with the [`log`](https://docs.rs/log/0.4.6/log/)
    crate, allowing log messages to be recorded as `tracing` `Event`s
    within the trace tree. This is useful when a project using `tracing`
    have dependencies which use `log`. Note that if you’re using
    `tracing-subscriber`’s `FmtSubscriber`, you don’t need to depend on
    `tracing-log` directly.
-   [`tracing-appender`](https://crates.io/crates/tracing-appender)
    provides utilities for outputting tracing data, including a file
    appender and non blocking writer.

Additionally, there are also several third-party crates which are not
maintained by the `tokio` project. These include:

-   [`tracing-timing`](https://crates.io/crates/tracing-timing)
    implements inter-event timing metrics on top of `tracing`. It
    provides a subscriber that records the time elapsed between pairs of
    `tracing` events and generates histograms.
-   [`tracing-opentelemetry`](https://crates.io/crates/tracing-opentelemetry)
    provides a subscriber for emitting traces to
    [OpenTelemetry](https://opentelemetry.io/)-compatible distributed
    tracing systems.
-   [`tracing-honeycomb`](https://crates.io/crates/tracing-honeycomb)
    Provides a layer that reports traces spanning multiple machines to
    [honeycomb.io](https://www.honeycomb.io/). Backed by
    [`tracing-distributed`](https://crates.io/crates/tracing-distributed).
-   [`tracing-distributed`](https://crates.io/crates/tracing-distributed)
    Provides a generic implementation of a layer that reports traces
    spanning multiple machines to some backend.
-   [`tracing-actix-web`](https://crates.io/crates/tracing-actix-web)
    provides `tracing` integration for the `actix-web` web framework.
-   [`tracing-actix`](https://crates.io/crates/tracing-actix) provides
    `tracing` integration for the `actix` actor framework.
-   [`axum-insights`](https://crates.io/crates/axum-insights) provides
    `tracing` integration and Application insights export for the `axum`
    web framework.
-   [`tracing-gelf`](https://crates.io/crates/tracing-gelf) implements a
    subscriber for exporting traces in Greylog GELF format.
-   [`tracing-coz`](https://crates.io/crates/tracing-coz) provides
    integration with the [coz](https://github.com/plasma-umass/coz)
    causal profiler (Linux-only).
-   [`tracing-bunyan-formatter`](https://crates.io/crates/tracing-bunyan-formatter)
    provides a layer implementation that reports events and spans in
    [bunyan](https://github.com/trentm/node-bunyan) format, enriched
    with timing information.
-   [`tracing-wasm`](https://docs.rs/tracing-wasm) provides a
    `Subscriber`/`Layer` implementation that reports events and spans
    via browser `console.log` and [User Timing API
    (`window.performance`)](https://developer.mozilla.org/en-US/docs/Web/API/User_Timing_API).
-   [`tracing-web`](https://docs.rs/tracing-web) provides a layer
    implementation of level-aware logging of events to web browsers’
    `console.*` and span events to the [User Timing API
    (`window.performance`)](https://developer.mozilla.org/en-US/docs/Web/API/User_Timing_API).
-   [`tide-tracing`](https://crates.io/crates/tide-tracing) provides a
    [tide](https://crates.io/crates/tide) middleware to trace all
    incoming requests and responses.
-   [`test-log`](https://crates.io/crates/test-log) takes care of
    initializing `tracing` for tests, based on environment variables
    with an `env_logger` compatible syntax.
-   [`tracing-unwrap`](https://docs.rs/tracing-unwrap) provides
    convenience methods to report failed unwraps on `Result` or `Option`
    types to a `Subscriber`.
-   [`diesel-tracing`](https://crates.io/crates/diesel-tracing) provides
    integration with [`diesel`](https://crates.io/crates/diesel)
    database connections.
-   [`tracing-tracy`](https://crates.io/crates/tracing-tracy) provides a
    way to collect [Tracy](https://github.com/wolfpld/tracy) profiles in
    instrumented applications.
-   [`tracing-elastic-apm`](https://crates.io/crates/tracing-elastic-apm)
    provides a layer for reporting traces to [Elastic
    APM](https://www.elastic.co/apm).
-   [`tracing-etw`](https://github.com/microsoft/rust_win_etw/tree/main/win_etw_tracing)
    provides a layer for emitting Windows
    [ETW](https://docs.microsoft.com/en-us/windows/win32/etw/about-event-tracing)
    events.
-   [`tracing-fluent-assertions`](https://crates.io/crates/tracing-fluent-assertions)
    provides a fluent assertions-style testing framework for validating
    the behavior of `tracing` spans.
-   [`sentry-tracing`](https://crates.io/crates/sentry-tracing) provides
    a layer for reporting events and traces to
    [Sentry](https://sentry.io/welcome/).
-   [`tracing-forest`](https://crates.io/crates/tracing-forest) provides
    a subscriber that preserves contextual coherence by grouping
    together logs from the same spans during writing.
-   [`tracing-loki`](https://crates.io/crates/tracing-loki) provides a
    layer for shipping logs to [Grafana
    Loki](https://grafana.com/oss/loki/).
-   [`tracing-logfmt`](https://crates.io/crates/tracing-logfmt) provides
    a layer that formats events and spans into the logfmt format.
-   [`reqwest-tracing`](https://crates.io/crates/reqwest-tracing)
    provides a middleware to trace
    [`reqwest`](https://crates.io/crates/reqwest) HTTP requests.
-   [`tracing-cloudwatch`](https://crates.io/crates/tracing-cloudwatch)
    provides a layer that sends events to AWS CloudWatch Logs.
-   [`clippy-tracing`](https://crates.io/crates/clippy-tracing) provides
    a tool to add, remove and check for `tracing::instrument`.
-   [`json-subscriber`](https://crates.io/crates/json-subscriber)
    provides a subscriber for emitting JSON logs. The output can be
    customized much more than with
    [`tracing-subscriber`](https://crates.io/crates/tracing-subscriber)’s
    JSON output.

If you’re the maintainer of a `tracing` ecosystem crate not listed
above, please let us know! We’d love to add your project to the list!

``` {.ignore style="white-space:normal;font:inherit;"}
     Note: Some of these ecosystem crates are currently
     unreleased and/or in earlier stages of development. They may be less stable
     than tracing and tracing-core.
 
```

## <a href="#crate-feature-flags" class="doc-anchor">§</a>Crate Feature Flags

The following crate [feature
flags](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section)
are available:

-   A set of features controlling the [static verbosity
    level](level_filters/index.html#compile-time-filters "mod tracing::level_filters").

-   `log`: causes trace instrumentation points to emit
    [`log`](https://docs.rs/log/0.4.6/log/) records as well as trace
    events, if a default `tracing` subscriber has not been set. This is
    intended for use in libraries whose users may be using either
    `tracing` or `log`.

-   `log-always`: Emit `log` records from all `tracing` spans and
    events, even if a `tracing` subscriber has been set. This should be
    set only by applications which intend to collect traces and logs
    separately; if an adapter is used to convert `log` records into
    `tracing` events, this will cause duplicate events to occur.

-   `attributes`: Includes support for the `#[instrument]` attribute.
    This is on by default, but does bring in the `syn` crate as a
    dependency, which may add to the compile time of crates that do not
    already use it.

-   `std`: Depend on the Rust standard library (enabled by default).

    `no_std` users may disable this feature with
    `default-features = false`:

    ``` language-toml
    [dependencies]
    tracing = { version = "0.1.38", default-features = false }
    ```

``` {.ignore style="white-space:normal;font:inherit;"}
     Note: tracing's no_std support
     requires liballoc.
 
```

### <a href="#unstable-features" class="doc-anchor">§</a>Unstable Features

These feature flags enable **unstable** features. The public API may
break in 0.1.x releases. To enable these features, the
`--cfg tracing_unstable` must be passed to `rustc` when compiling.

The following unstable feature flags are currently available:

-   `valuable`: Enables support for recording [field
    values](field/index.html "mod tracing::field") using the
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

# Modules<a href="#modules" class="anchor">§</a>

<a href="dispatcher/index.html" class="mod"
title="mod tracing::dispatcher">dispatcher</a>  
Dispatches trace events to
[`Subscriber`](trait.Subscriber.html "trait tracing::Subscriber")s.

<a href="event/index.html" class="mod"
title="mod tracing::event">event</a>  
Events represent single points in time during the execution of a
program.

<a href="field/index.html" class="mod"
title="mod tracing::field">field</a>  
`Span` and `Event` key-value data.

<a href="instrument/index.html" class="mod"
title="mod tracing::instrument">instrument</a>  
Attach a span to a `std::future::Future`.

<a href="level_filters/index.html" class="mod"
title="mod tracing::level_filters">level_filters</a>  
Trace verbosity level filtering.

<a href="span/index.html" class="mod" title="mod tracing::span">span</a>  
Spans represent periods of time in which a program was executing in a
particular context.

<a href="subscriber/index.html" class="mod"
title="mod tracing::subscriber">subscriber</a>  
Collects and records trace data.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.debug.html" class="macro"
title="macro tracing::debug">debug</a>  
Constructs an event at the debug level.

<a href="macro.debug_span.html" class="macro"
title="macro tracing::debug_span">debug_span</a>  
Constructs a span at the debug level.

<a href="macro.enabled.html" class="macro"
title="macro tracing::enabled">enabled</a>  
Checks whether a span or event is
[enabled](trait.Subscriber.html#tymethod.enabled "method tracing::Subscriber::enabled")
based on the provided
[metadata](struct.Metadata.html "struct tracing::Metadata").

<a href="macro.error.html" class="macro"
title="macro tracing::error">error</a>  
Constructs an event at the error level.

<a href="macro.error_span.html" class="macro"
title="macro tracing::error_span">error_span</a>  
Constructs a span at the error level.

<a href="macro.event.html" class="macro"
title="macro tracing::event">event</a>  
Constructs a new `Event`.

<a href="macro.event_enabled.html" class="macro"
title="macro tracing::event_enabled">event_enabled</a>  
Tests whether an event with the specified level and target would be
enabled.

<a href="macro.info.html" class="macro"
title="macro tracing::info">info</a>  
Constructs an event at the info level.

<a href="macro.info_span.html" class="macro"
title="macro tracing::info_span">info_span</a>  
Constructs a span at the info level.

<a href="macro.record_all.html" class="macro"
title="macro tracing::record_all">record_all</a>  
Records multiple values on a span in a single call. As with recording
individual values, all fields must be declared when the span is created.

<a href="macro.span.html" class="macro"
title="macro tracing::span">span</a>  
Constructs a new span.

<a href="macro.span_enabled.html" class="macro"
title="macro tracing::span_enabled">span_enabled</a>  
Tests whether a span with the specified level and target would be
enabled.

<a href="macro.trace.html" class="macro"
title="macro tracing::trace">trace</a>  
Constructs an event at the trace level.

<a href="macro.trace_span.html" class="macro"
title="macro tracing::trace_span">trace_span</a>  
Constructs a span at the trace level.

<a href="macro.warn.html" class="macro"
title="macro tracing::warn">warn</a>  
Constructs an event at the warn level.

<a href="macro.warn_span.html" class="macro"
title="macro tracing::warn_span">warn_span</a>  
Constructs a span at the warn level.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Dispatch.html" class="struct"
title="struct tracing::Dispatch">Dispatch</a>  
`Dispatch` trace data to a
[`Subscriber`](trait.Subscriber.html "trait tracing::Subscriber").

<a href="struct.Event.html" class="struct"
title="struct tracing::Event">Event</a>  
`Event`s represent single points in time where something occurred during
the execution of a program.

<a href="struct.Level.html" class="struct"
title="struct tracing::Level">Level</a>  
Describes the level of verbosity of a span or event.

<a href="struct.Metadata.html" class="struct"
title="struct tracing::Metadata">Metadata</a>  
Metadata describing a
[span](../tracing_core/span/index.html "mod tracing_core::span") or
[event](event/index.html "mod tracing::event").

<a href="struct.Span.html" class="struct"
title="struct tracing::Span">Span</a>  
A handle representing a span, with the capability to enter the span if
it exists.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Instrument.html" class="trait"
title="trait tracing::Instrument">Instrument</a>  
Attaches spans to a
[`std::future::Future`](https://doc.rust-lang.org/1.93.1/core/future/future/trait.Future.html "trait core::future::future::Future").

<a href="trait.Subscriber.html" class="trait"
title="trait tracing::Subscriber">Subscriber</a>  
Trait representing the functions required to collect trace data.

<a href="trait.Value.html" class="trait"
title="trait tracing::Value">Value</a>  
A field value of an erased type.

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.instrument.html" class="attr"
title="attr tracing::instrument">instrument</a>  
Instruments a function to create and enter a `tracing`
[span](https://docs.rs/tracing/latest/tracing/span/index.html) every
time the function is called.
