Crate sentry Copy item path

<span class="sub-heading"><a href="../src/sentry/lib.rs.html#1-242" class="src">Source</a>
</span>

Expand description

This crate provides support for logging events and errors / panics to
the [Sentry](https://sentry.io/) error logging service. It integrates
with the standard panic system in Rust as well as a few popular error
handling setups.

# <a href="#quickstart" class="doc-anchor">§</a>Quickstart

The most convenient way to use this library is via the
[`sentry::init`](fn.init.html) function, which starts a sentry client
with a default set of integrations, and binds it to the current
[`Hub`](struct.Hub.html). More Information on how to use Sentry in
parallel, concurrent and async scenarios can be found on the
[`Hub`](struct.Hub.html) docs as well.

The [`sentry::init`](fn.init.html) function returns a guard that when
dropped will flush Events that were not yet sent to the sentry service.
It has a two second deadline for this so shutdown of applications might
slightly delay as a result of this. Keep the guard around or sending
events will not work.

``` {.rust .rust-example-rendered}
let _guard = sentry::init("https://key@sentry.io/42");
sentry::capture_message("Hello World!", sentry::Level::Info);
// when the guard goes out of scope here, the client will wait up to two
// seconds to send remaining events to the service.
```

More complex examples on how to use sentry can also be found in
[examples](https://github.com/getsentry/sentry-rust/tree/master/sentry/examples).
Extended instructions may also be found on [Sentry
itself](https://docs.sentry.io/platforms/rust).

# <a href="#integrations" class="doc-anchor">§</a>Integrations

What makes this crate useful are its various integrations. Some of them
are enabled by default; See [Features](#features). Uncommon integrations
or integrations for deprecated parts of the ecosystem require a feature
flag. For available integrations and how to use them, see
[integrations](integrations/index.html) and
[apply\_defaults](fn.apply_defaults.html).

# <a href="#minimal-api" class="doc-anchor">§</a>Minimal API

This crate comes fully-featured. If the goal is to instrument libraries
for usage with sentry, or to extend sentry with a custom
[`Integration`](trait.Integration.html) or a
[`Transport`](trait.Transport.html), one should use the
[`sentry-core`](https://crates.io/crates/sentry-core) crate instead.

# <a href="#features" class="doc-anchor">§</a>Features

Additional functionality and integrations are enabled via feature flags.
Some features require extra setup to function properly.

| Feature | Default | Is Integration | Deprecated | Additional notes |
|----|----|----|----|----|
| `backtrace` | ✅ | 🔌 |  |  |
| `contexts` | ✅ | 🔌 |  |  |
| `panic` | ✅ | 🔌 |  |  |
| `transport` | ✅ |  |  |  |
| `anyhow` |  | 🔌 |  |  |
| `test` |  |  |  |  |
| `debug-images` | ✅ | 🔌 |  |  |
| `log` |  | 🔌 |  | Requires extra setup; See [`sentry-log`](https://crates.io/crates/sentry-log)’s documentation. |
| `slog` |  | 🔌 |  | Requires extra setup; See [`sentry-slog`](https://crates.io/crates/sentry-slog)’s documentation. |
| `reqwest` | ✅ |  |  |  |
| `native-tls` | ✅ |  |  | `reqwest` must be enabled. |
| `rustls` |  |  |  | `reqwest` must be enabled. `native-tls` must be disabled via `default-features = false`. |
| `ureq` |  |  |  | `ureq` transport support using `rustls` by default |
| `ureq-native-tls` |  |  |  |  |
| `curl` |  |  |  |  |
| `actix` |  | 🔌 |  | Requires extra setup; See [`sentry-actix`](https://crates.io/crates/sentry-actix)’s documentation. |
| `tower` |  | 🔌 |  | Requires extra setup; See [`sentry-tower`](https://crates.io/crates/sentry-tower)’s documentation. |
| `tracing` |  | 🔌 |  | Requires extra setup; See [`sentry-tracing`](https://crates.io/crates/sentry-tracing)’s documentation. |
| `opentelemetry` |  | 🔌 |  | Requires extra setup; See [`sentry-opentelemetry`](https://crates.io/crates/sentry-opentelemetry)’s documentation. |

## <a href="#default-features" class="doc-anchor">§</a>Default features

-   `backtrace`: Enables backtrace support.
-   `contexts`: Enables capturing device, OS, and Rust contexts.
-   `panic`: Enables support for capturing panics.
-   `transport`: Enables the default transport, which is currently
    `reqwest` with `native-tls`.
-   `debug-images`: Enables capturing metadata about the loaded shared
    libraries.

## <a href="#debuggingtesting" class="doc-anchor">§</a>Debugging/Testing

-   `anyhow`: Enables support for the `anyhow` crate.
-   `test`: Enables testing support.

## <a href="#logging" class="doc-anchor">§</a>Logging

-   `log`: Enables support for the `log` crate.
-   `slog`: Enables support for the `slog` crate.

## <a href="#transports" class="doc-anchor">§</a>Transports

-   `reqwest`: **Default**. Enables the `reqwest` transport.
-   `native-tls`: **Default**. Uses the `native-tls` crate. This only
    affects the `reqwest` transport.
-   `rustls`: Enables `rustls` support for `reqwest`. Please note that
    `native-tls` is a default feature, and `default-features = false`
    must be set to completely disable building `native-tls`
    dependencies.
-   `curl`: Enables the `curl` transport.
-   `ureq`: Enables the `ureq` transport using `rustls`.
-   `ureq-native-tls`: Enables the `ureq` transport using `native-tls`.

## <a href="#integrations-1" class="doc-anchor">§</a>Integrations

-   `actix`: Enables support for the `actix-web` crate.
-   `tower`: Enables support for the `tower` crate and those using it.
-   `tracing`: Enables support for the `tracing` crate and those using
    it.
-   `opentelemetry`: Enables support for the `opentelemetry` and
    `opentelemetry-sdk` crates.

# Modules<a href="#modules" class="anchor">§</a>

<a href="integrations/index.html" class="mod"
title="mod sentry::integrations">integrations</a>  
Available Sentry Integrations.

<a href="protocol/index.html" class="mod"
title="mod sentry::protocol">protocol</a>  
The current latest sentry protocol version.

<a href="transports/index.html" class="mod"
title="mod sentry::transports">transports</a>  
The provided transports.

<a href="types/index.html" class="mod"
title="mod sentry::types">types</a>  
This crate provides common types for working with the Sentry protocol or
the Sentry server. It’s used by the Sentry Relay infrastructure as well
as the rust Sentry client.

<a href="utils/index.html" class="mod"
title="mod sentry::utils">utils</a>  
Utilities reused across dependant crates and integrations.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.release_name.html" class="macro"
title="macro sentry::release_name">release_name</a>  
Returns the intended release for Sentry as an
`Option<Cow<'static, str>>`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Breadcrumb.html" class="struct"
title="struct sentry::Breadcrumb">Breadcrumb</a>  
Represents a single breadcrumb.

<a href="struct.Client.html" class="struct"
title="struct sentry::Client">Client</a>  
The Sentry Client.

<a href="struct.ClientInitGuard.html" class="struct"
title="struct sentry::ClientInitGuard">ClientInitGuard</a>  
Helper struct that is returned from `init`.

<a href="struct.ClientOptions.html" class="struct"
title="struct sentry::ClientOptions">ClientOptions</a>  
Configuration settings for the client.

<a href="struct.Data.html" class="struct"
title="struct sentry::Data">Data</a>  
A smart pointer to a span’s [`data`
field](protocol/struct.Span.html#structfield.data "field sentry::protocol::Span::data").

<a href="struct.Envelope.html" class="struct"
title="struct sentry::Envelope">Envelope</a>  
A Sentry Envelope.

<a href="struct.Hub.html" class="struct"
title="struct sentry::Hub">Hub</a>  
The central object that can manage scopes and clients.

<a href="struct.HubSwitchGuard.html" class="struct"
title="struct sentry::HubSwitchGuard">HubSwitchGuard</a>  
A guard that temporarily swaps the active hub in thread-local storage.

<a href="struct.Scope.html" class="struct"
title="struct sentry::Scope">Scope</a>  
Holds contextual data for the current scope.

<a href="struct.ScopeGuard.html" class="struct"
title="struct sentry::ScopeGuard">ScopeGuard</a>  
A scope guard.

<a href="struct.SentryFuture.html" class="struct"
title="struct sentry::SentryFuture">SentryFuture</a>  
A future that binds a `Hub` to its execution.

<a href="struct.SentryTrace.html" class="struct"
title="struct sentry::SentryTrace">SentryTrace</a>  
A container for distributed tracing metadata that can be extracted from
e.g. the `sentry-trace` HTTP header.

<a href="struct.Span.html" class="struct"
title="struct sentry::Span">Span</a>  
A running Performance Monitoring Span.

<a href="struct.TraceHeadersIter.html" class="struct"
title="struct sentry::TraceHeadersIter">TraceHeadersIter</a>  
An Iterator over HTTP header names and values needed for distributed
tracing.

<a href="struct.Transaction.html" class="struct"
title="struct sentry::Transaction">Transaction</a>  
A running Performance Monitoring Transaction.

<a href="struct.TransactionContext.html" class="struct"
title="struct sentry::TransactionContext">TransactionContext</a>  
The Transaction Context used to start a new Performance Monitoring
Transaction.

<a href="struct.TransactionContextBuilder.html" class="struct"
title="struct sentry::TransactionContextBuilder">TransactionContextBuilder</a>  
A transaction context builder created by
[`TransactionContext::builder`](struct.TransactionContext.html#method.builder "associated function sentry::TransactionContext::builder").

<a href="struct.TransactionData.html" class="struct"
title="struct sentry::TransactionData">TransactionData</a>  
Iterable for a transaction’s [data
attributes](protocol/struct.TraceContext.html#structfield.data "field sentry::protocol::TraceContext::data").

<a href="struct.User.html" class="struct"
title="struct sentry::User">User</a>  
Represents user info.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Level.html" class="enum"
title="enum sentry::Level">Level</a>  
Represents the level of severity of an event or breadcrumb.

<a href="enum.MaxRequestBodySize.html" class="enum"
title="enum sentry::MaxRequestBodySize">MaxRequestBodySize</a>  
The maximum size of an HTTP request body that the SDK captures.

<a href="enum.SessionMode.html" class="enum"
title="enum sentry::SessionMode">SessionMode</a>  
The Session Mode of the SDK.

<a href="enum.TransactionOrSpan.html" class="enum"
title="enum sentry::TransactionOrSpan">TransactionOrSpan</a>  
A wrapper that groups a
[`Transaction`](struct.Transaction.html "struct sentry::Transaction")
and a [`Span`](struct.Span.html "struct sentry::Span") together.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Integration.html" class="trait"
title="trait sentry::Integration">Integration</a>  
Integration abstraction.

<a href="trait.IntoBreadcrumbs.html" class="trait"
title="trait sentry::IntoBreadcrumbs">IntoBreadcrumbs</a>  
A helper trait that converts self into an Iterator of Breadcrumbs.

<a href="trait.IntoDsn.html" class="trait"
title="trait sentry::IntoDsn">IntoDsn</a>  
Helper trait to convert a string into an `Option<Dsn>`.

<a href="trait.SentryFutureExt.html" class="trait"
title="trait sentry::SentryFutureExt">SentryFutureExt</a>  
Future extensions for Sentry.

<a href="trait.Transport.html" class="trait"
title="trait sentry::Transport">Transport</a>  
The trait for transports.

<a href="trait.TransportFactory.html" class="trait"
title="trait sentry::TransportFactory">TransportFactory</a>  
A factory creating transport instances.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.add_breadcrumb.html" class="fn"
title="fn sentry::add_breadcrumb">add_breadcrumb</a>  
Records a breadcrumb by calling a function.

<a href="fn.apply_defaults.html" class="fn"
title="fn sentry::apply_defaults">apply_defaults</a>  
Apply default client options.

<a href="fn.capture_error.html" class="fn"
title="fn sentry::capture_error">capture_error</a>  
Captures a `std::error::Error`.

<a href="fn.capture_event.html" class="fn"
title="fn sentry::capture_event">capture_event</a>  
Captures an event on the currently active client if any.

<a href="fn.capture_message.html" class="fn"
title="fn sentry::capture_message">capture_message</a>  
Captures an arbitrary message.

<a href="fn.configure_scope.html" class="fn"
title="fn sentry::configure_scope">configure_scope</a>  
Invokes a function that can modify the current scope.

<a href="fn.end_session.html" class="fn"
title="fn sentry::end_session">end_session</a>  
End the current Release Health Session.

<a href="fn.end_session_with_status.html" class="fn"
title="fn sentry::end_session_with_status">end_session_with_status</a>  
End the current Release Health Session with the given
[`SessionStatus`](protocol/enum.SessionStatus.html "enum sentry::protocol::SessionStatus").

<a href="fn.event_from_error.html" class="fn"
title="fn sentry::event_from_error">event_from_error</a>  
Create a sentry `Event` from a `std::error::Error`.

<a href="fn.init.html" class="fn" title="fn sentry::init">init</a>  
Creates the Sentry client for a given client config and binds it.

<a href="fn.last_event_id.html" class="fn"
title="fn sentry::last_event_id">last_event_id</a>  
Returns the last event ID captured.

<a href="fn.parse_headers.html" class="fn"
title="fn sentry::parse_headers">parse_headers</a>  
Extracts distributed tracing metadata from headers (or, generally,
key-value pairs), considering the values for `sentry-trace`.

<a href="fn.parse_type_from_debug.html" class="fn"
title="fn sentry::parse_type_from_debug">parse_type_from_debug</a>  
Parse the types name from `Debug` output.

<a href="fn.start_session.html" class="fn"
title="fn sentry::start_session">start_session</a>  
Start a new session for Release Health.

<a href="fn.start_transaction.html" class="fn"
title="fn sentry::start_transaction">start_transaction</a>  
Start a new Performance Monitoring Transaction.

<a href="fn.start_transaction_with_timestamp.html" class="fn"
title="fn sentry::start_transaction_with_timestamp">start_transaction_with_timestamp</a>  
Start a new Performance Monitoring Transaction with the provided start
timestamp.

<a href="fn.with_integration.html" class="fn"
title="fn sentry::with_integration">with_integration</a>  
Looks up an integration on the current Hub.

<a href="fn.with_scope.html" class="fn"
title="fn sentry::with_scope">with_scope</a>  
Temporarily pushes a scope for a single call optionally reconfiguring
it.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BeforeCallback.html" class="type"
title="type sentry::BeforeCallback">BeforeCallback</a>  
Type alias for before event/breadcrumb handlers.

<a href="type.CustomTransactionContext.html" class="type"
title="type sentry::CustomTransactionContext">CustomTransactionContext</a>  
Arbitrary data passed by the caller, when starting a transaction.

<a href="type.TraceHeader.html" class="type"
title="type sentry::TraceHeader">TraceHeader</a>  
Represents a key-value pair such as an HTTP header.

<a href="type.TracesSampler.html" class="type"
title="type sentry::TracesSampler">TracesSampler</a>  
A function to be run for each new transaction, to determine the rate at
which it should be sampled.
