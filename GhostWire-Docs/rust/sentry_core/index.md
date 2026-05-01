Crate sentry\_core Copy item path

<span class="sub-heading"><a href="../src/sentry_core/lib.rs.html#1-165" class="src">Source</a>
</span>

Expand description

This crate provides the core of the [Sentry](https://sentry.io/) SDK,
which can be used to log events and errors.

`sentry-core` is meant for integration authors and third-party library
authors that want to instrument their code for sentry.

Regular users who wish to integrate sentry into their applications
should instead use the [`sentry`](https://crates.io/crates/sentry)
crate, which comes with a default transport and a large set of
integrations for various third-party libraries.

# <a href="#core-concepts" class="doc-anchor">§</a>Core Concepts

This crate follows the [Unified
API](https://develop.sentry.dev/sdk/unified-api/) guidelines and is
centered around the concepts of
[`Client`](struct.Client.html "struct sentry_core::Client"),
[`Hub`](struct.Hub.html "struct sentry_core::Hub") and
[`Scope`](struct.Scope.html "struct sentry_core::Scope"), as well as the
extension points via the
[`Integration`](trait.Integration.html "trait sentry_core::Integration"),
[`Transport`](trait.Transport.html "trait sentry_core::Transport") and
[`TransportFactory`](trait.TransportFactory.html "trait sentry_core::TransportFactory")
traits.

# <a href="#parallelism-concurrency-and-async" class="doc-anchor">§</a>Parallelism, Concurrency and Async

The main concurrency primitive is the
[`Hub`](struct.Hub.html "struct sentry_core::Hub"). In general, all
concurrent code, no matter if multithreaded parallelism or futures
concurrency, needs to run with its own copy of a
[`Hub`](struct.Hub.html "struct sentry_core::Hub"). Even though the
[`Hub`](struct.Hub.html "struct sentry_core::Hub") is internally
synchronized, using it concurrently may lead to unexpected results up to
panics.

For threads or tasks that are running concurrently or outlive the
current execution context, a new
[`Hub`](struct.Hub.html "struct sentry_core::Hub") needs to be created
and bound for the computation.

``` {.rust .rust-example-rendered}
use rayon::prelude::*;
use sentry::{Hub, SentryFutureExt};
use std::sync::Arc;

// Parallel multithreaded code:
let outer_hub = Hub::current();
let results: Vec<_> = [1_u32, 2, 3]
    .into_par_iter()
    .map(|num| {
        let thread_hub = Arc::new(Hub::new_from_top(&outer_hub));
        Hub::run(thread_hub, || num * num)
    })
    .collect();

assert_eq!(&results, &[1, 4, 9]);

// Concurrent futures code:
let futures = [1_u32, 2, 3]
    .into_iter()
    .map(|num| async move { num * num }.bind_hub(Hub::new_from_top(Hub::current())));
let results = futures::future::join_all(futures).await;

assert_eq!(&results, &[1, 4, 9]);
```

For tasks that are not concurrent and do not outlive the current
execution context, no *new*
[`Hub`](struct.Hub.html "struct sentry_core::Hub") needs to be created,
but the current [`Hub`](struct.Hub.html "struct sentry_core::Hub") has
to be bound.

``` {.rust .rust-example-rendered}
use sentry::{Hub, SentryFutureExt};

// Spawned thread that is being joined:
let hub = Hub::current();
let result = std::thread::spawn(|| Hub::run(hub, || 1_u32)).join();

assert_eq!(result.unwrap(), 1);

// Spawned future that is being awaited:
let result = tokio::spawn(async { 1_u32 }.bind_hub(Hub::current())).await;

assert_eq!(result.unwrap(), 1);
```

# <a href="#minimal-api" class="doc-anchor">§</a>Minimal API

By default, this crate comes with a so-called “minimal” mode. This mode
will provide all the APIs needed to instrument code with sentry, and to
write sentry integrations, but it will blackhole a lot of operations.

In minimal mode some types are restricted in functionality. For instance
the [`Client`](struct.Client.html "struct sentry_core::Client") is not
available and the [`Hub`](struct.Hub.html "struct sentry_core::Hub")
does not retain all API functionality.

# <a href="#features" class="doc-anchor">§</a>Features

-   `feature = "client"`: Activates the
    [`Client`](struct.Client.html "struct sentry_core::Client") type and
    certain [`Hub`](struct.Hub.html "struct sentry_core::Hub")
    functionality.
-   `feature = "test"`: Activates the [`test`](test/index.html) module,
    which can be used to write integration tests. It comes with a test
    transport which can capture all sent events for inspection.

# Modules<a href="#modules" class="anchor">§</a>

<a href="protocol/index.html" class="mod"
title="mod sentry_core::protocol">protocol</a>  
The current latest sentry protocol version.

<a href="types/index.html" class="mod"
title="mod sentry_core::types">types</a>  
This crate provides common types for working with the Sentry protocol or
the Sentry server. It’s used by the Sentry Relay infrastructure as well
as the rust Sentry client.

<a href="utils/index.html" class="mod"
title="mod sentry_core::utils">utils</a>  
Utilities reused across dependant crates and integrations.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.release_name.html" class="macro"
title="macro sentry_core::release_name">release_name</a>  
Returns the intended release for Sentry as an
`Option<Cow<'static, str>>`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Breadcrumb.html" class="struct"
title="struct sentry_core::Breadcrumb">Breadcrumb</a>  
Represents a single breadcrumb.

<a href="struct.Client.html" class="struct"
title="struct sentry_core::Client">Client</a>  
The Sentry Client.

<a href="struct.ClientOptions.html" class="struct"
title="struct sentry_core::ClientOptions">ClientOptions</a>  
Configuration settings for the client.

<a href="struct.Data.html" class="struct"
title="struct sentry_core::Data">Data</a>  
A smart pointer to a span’s [`data`
field](protocol/struct.Span.html#structfield.data "field sentry_core::protocol::Span::data").

<a href="struct.Envelope.html" class="struct"
title="struct sentry_core::Envelope">Envelope</a>  
A Sentry Envelope.

<a href="struct.Hub.html" class="struct"
title="struct sentry_core::Hub">Hub</a>  
The central object that can manage scopes and clients.

<a href="struct.HubSwitchGuard.html" class="struct"
title="struct sentry_core::HubSwitchGuard">HubSwitchGuard</a>  
A guard that temporarily swaps the active hub in thread-local storage.

<a href="struct.Scope.html" class="struct"
title="struct sentry_core::Scope">Scope</a>  
Holds contextual data for the current scope.

<a href="struct.ScopeGuard.html" class="struct"
title="struct sentry_core::ScopeGuard">ScopeGuard</a>  
A scope guard.

<a href="struct.SentryFuture.html" class="struct"
title="struct sentry_core::SentryFuture">SentryFuture</a>  
A future that binds a `Hub` to its execution.

<a href="struct.SentryTrace.html" class="struct"
title="struct sentry_core::SentryTrace">SentryTrace</a>  
A container for distributed tracing metadata that can be extracted from
e.g. the `sentry-trace` HTTP header.

<a href="struct.Span.html" class="struct"
title="struct sentry_core::Span">Span</a>  
A running Performance Monitoring Span.

<a href="struct.TraceHeadersIter.html" class="struct"
title="struct sentry_core::TraceHeadersIter">TraceHeadersIter</a>  
An Iterator over HTTP header names and values needed for distributed
tracing.

<a href="struct.Transaction.html" class="struct"
title="struct sentry_core::Transaction">Transaction</a>  
A running Performance Monitoring Transaction.

<a href="struct.TransactionContext.html" class="struct"
title="struct sentry_core::TransactionContext">TransactionContext</a>  
The Transaction Context used to start a new Performance Monitoring
Transaction.

<a href="struct.TransactionContextBuilder.html" class="struct"
title="struct sentry_core::TransactionContextBuilder">TransactionContextBuilder</a>  
A transaction context builder created by
[`TransactionContext::builder`](struct.TransactionContext.html#method.builder "associated function sentry_core::TransactionContext::builder").

<a href="struct.TransactionData.html" class="struct"
title="struct sentry_core::TransactionData">TransactionData</a>  
Iterable for a transaction’s [data
attributes](protocol/struct.TraceContext.html#structfield.data "field sentry_core::protocol::TraceContext::data").

<a href="struct.User.html" class="struct"
title="struct sentry_core::User">User</a>  
Represents user info.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Level.html" class="enum"
title="enum sentry_core::Level">Level</a>  
Represents the level of severity of an event or breadcrumb.

<a href="enum.MaxRequestBodySize.html" class="enum"
title="enum sentry_core::MaxRequestBodySize">MaxRequestBodySize</a>  
The maximum size of an HTTP request body that the SDK captures.

<a href="enum.SessionMode.html" class="enum"
title="enum sentry_core::SessionMode">SessionMode</a>  
The Session Mode of the SDK.

<a href="enum.TransactionOrSpan.html" class="enum"
title="enum sentry_core::TransactionOrSpan">TransactionOrSpan</a>  
A wrapper that groups a
[`Transaction`](struct.Transaction.html "struct sentry_core::Transaction")
and a [`Span`](struct.Span.html "struct sentry_core::Span") together.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Integration.html" class="trait"
title="trait sentry_core::Integration">Integration</a>  
Integration abstraction.

<a href="trait.IntoBreadcrumbs.html" class="trait"
title="trait sentry_core::IntoBreadcrumbs">IntoBreadcrumbs</a>  
A helper trait that converts self into an Iterator of Breadcrumbs.

<a href="trait.IntoDsn.html" class="trait"
title="trait sentry_core::IntoDsn">IntoDsn</a>  
Helper trait to convert a string into an `Option<Dsn>`.

<a href="trait.SentryFutureExt.html" class="trait"
title="trait sentry_core::SentryFutureExt">SentryFutureExt</a>  
Future extensions for Sentry.

<a href="trait.Transport.html" class="trait"
title="trait sentry_core::Transport">Transport</a>  
The trait for transports.

<a href="trait.TransportFactory.html" class="trait"
title="trait sentry_core::TransportFactory">TransportFactory</a>  
A factory creating transport instances.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.add_breadcrumb.html" class="fn"
title="fn sentry_core::add_breadcrumb">add_breadcrumb</a>  
Records a breadcrumb by calling a function.

<a href="fn.capture_error.html" class="fn"
title="fn sentry_core::capture_error">capture_error</a>  
Captures a `std::error::Error`.

<a href="fn.capture_event.html" class="fn"
title="fn sentry_core::capture_event">capture_event</a>  
Captures an event on the currently active client if any.

<a href="fn.capture_message.html" class="fn"
title="fn sentry_core::capture_message">capture_message</a>  
Captures an arbitrary message.

<a href="fn.configure_scope.html" class="fn"
title="fn sentry_core::configure_scope">configure_scope</a>  
Invokes a function that can modify the current scope.

<a href="fn.end_session.html" class="fn"
title="fn sentry_core::end_session">end_session</a>  
End the current Release Health Session.

<a href="fn.end_session_with_status.html" class="fn"
title="fn sentry_core::end_session_with_status">end_session_with_status</a>  
End the current Release Health Session with the given
[`SessionStatus`](protocol/enum.SessionStatus.html "enum sentry_core::protocol::SessionStatus").

<a href="fn.event_from_error.html" class="fn"
title="fn sentry_core::event_from_error">event_from_error</a>  
Create a sentry `Event` from a `std::error::Error`.

<a href="fn.last_event_id.html" class="fn"
title="fn sentry_core::last_event_id">last_event_id</a>  
Returns the last event ID captured.

<a href="fn.parse_headers.html" class="fn"
title="fn sentry_core::parse_headers">parse_headers</a>  
Extracts distributed tracing metadata from headers (or, generally,
key-value pairs), considering the values for `sentry-trace`.

<a href="fn.parse_type_from_debug.html" class="fn"
title="fn sentry_core::parse_type_from_debug">parse_type_from_debug</a>  
Parse the types name from `Debug` output.

<a href="fn.start_session.html" class="fn"
title="fn sentry_core::start_session">start_session</a>  
Start a new session for Release Health.

<a href="fn.start_transaction.html" class="fn"
title="fn sentry_core::start_transaction">start_transaction</a>  
Start a new Performance Monitoring Transaction.

<a href="fn.start_transaction_with_timestamp.html" class="fn"
title="fn sentry_core::start_transaction_with_timestamp">start_transaction_with_timestamp</a>  
Start a new Performance Monitoring Transaction with the provided start
timestamp.

<a href="fn.with_integration.html" class="fn"
title="fn sentry_core::with_integration">with_integration</a>  
Looks up an integration on the current Hub.

<a href="fn.with_scope.html" class="fn"
title="fn sentry_core::with_scope">with_scope</a>  
Temporarily pushes a scope for a single call optionally reconfiguring
it.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BeforeCallback.html" class="type"
title="type sentry_core::BeforeCallback">BeforeCallback</a>  
Type alias for before event/breadcrumb handlers.

<a href="type.CustomTransactionContext.html" class="type"
title="type sentry_core::CustomTransactionContext">CustomTransactionContext</a>  
Arbitrary data passed by the caller, when starting a transaction.

<a href="type.TraceHeader.html" class="type"
title="type sentry_core::TraceHeader">TraceHeader</a>  
Represents a key-value pair such as an HTTP header.

<a href="type.TracesSampler.html" class="type"
title="type sentry_core::TracesSampler">TracesSampler</a>  
A function to be run for each new transaction, to determine the rate at
which it should be sampled.
