Crate sentry\_backtrace Copy item path

<span class="sub-heading"><a href="../src/sentry_backtrace/lib.rs.html#1-26"
class="src">Source</a> </span>

Expand description

Backtrace Integration and utilities for sentry.

Exposes functions to capture, process and convert/parse stacktraces, as
well as integrations to process event stacktraces.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AttachStacktraceIntegration.html" class="struct"
title="struct sentry_backtrace::AttachStacktraceIntegration">AttachStacktraceIntegration</a>  
Integration to attach stacktraces to Events.

<a href="struct.Frame.html" class="struct"
title="struct sentry_backtrace::Frame">Frame</a>  
Represents a frame.

<a href="struct.ProcessStacktraceIntegration.html" class="struct"
title="struct sentry_backtrace::ProcessStacktraceIntegration">ProcessStacktraceIntegration</a>  
Integration to process Event stacktraces.

<a href="struct.Stacktrace.html" class="struct"
title="struct sentry_backtrace::Stacktrace">Stacktrace</a>  
Represents a stacktrace.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.backtrace_to_stacktrace.html" class="fn"
title="fn sentry_backtrace::backtrace_to_stacktrace">backtrace_to_stacktrace</a>  
Convert a `backtrace::Backtrace` into a Rust `Stacktrace`

<a href="fn.current_stacktrace.html" class="fn"
title="fn sentry_backtrace::current_stacktrace">current_stacktrace</a>  
Returns the current backtrace as sentry stacktrace.

<a href="fn.current_thread.html" class="fn"
title="fn sentry_backtrace::current_thread">current_thread</a>  
Captures information about the current thread.

<a href="fn.parse_stacktrace.html" class="fn"
title="fn sentry_backtrace::parse_stacktrace">parse_stacktrace</a>  
Parses a backtrace string into a Sentry `Stacktrace`.

<a href="fn.process_event_stacktrace.html" class="fn"
title="fn sentry_backtrace::process_event_stacktrace">process_event_stacktrace</a>  
Processes a `Stacktrace`.
