Crate signal\_hook\_registry Copy item path

<span class="sub-heading"><a href="../src/signal_hook_registry/lib.rs.html#1-870"
class="src">Source</a> </span>

Expand description

Backend of the [signal-hook](https://docs.rs/signal-hook) crate.

The [signal-hook](https://docs.rs/signal-hook) crate tries to provide an
API to the unix signals, which are a global resource. Therefore, it is
desirable an application contains just one version of the crate which
manages this global resource. But that makes it impossible to make
breaking changes in the API.

Therefore, this crate provides very minimal and low level API to the
signals that is unlikely to have to change, while there may be multiple
versions of the [signal-hook](https://docs.rs/signal-hook) that all use
this low-level API to provide different versions of the high level APIs.

It is also possible some other crates might want to build a completely
different API. This split allows these crates to still reuse the same
low-level routines in this crate instead of going to the (much more
dangerous) unix calls.

# <a href="#what-this-crate-provides" class="doc-anchor">§</a>What this crate provides

The only thing this crate does is multiplexing the signals. An
application or library can add or remove callbacks and have multiple
callbacks for the same signal.

It handles dispatching the callbacks and managing them in a way that
uses only the
[async-signal-safe](http://www.man7.org/linux/man-pages/man7/signal-safety.7.html)
functions inside the signal handler. Note that the callbacks are still
run inside the signal handler, so it is up to the caller to ensure they
are also
[async-signal-safe](http://www.man7.org/linux/man-pages/man7/signal-safety.7.html).

# <a href="#what-this-is-for" class="doc-anchor">§</a>What this is for

This is a building block for other libraries creating reasonable
abstractions on top of signals. The
[signal-hook](https://docs.rs/signal-hook) is the generally preferred
way if you need to handle signals in your application and provides
several safe patterns of doing so.

# <a href="#rust-version-compatibility" class="doc-anchor">§</a>Rust version compatibility

Currently builds on 1.26.0 an newer and this is very unlikely to change.
However, tests require dependencies that don’t build there, so tests
need newer Rust version (they are run on stable).

Note that this ancient version of rustc no longer compiles current
versions of `libc`. If you want to use rustc this old, you need to force
your dependency resolution to pick old enough version of `libc`
(`0.2.156` was found to work, but newer ones may too).

# <a href="#portability" class="doc-anchor">§</a>Portability

This crate includes a limited support for Windows, based on
`signal`/`raise` in the CRT. There are differences in both API and
behavior:

-   Due to lack of `siginfo_t`, we don’t provide `register_sigaction` or
    `register_unchecked`.
-   Due to lack of signal blocking, there’s a race condition. After the
    call to `signal`, there’s a moment where we miss a signal. That
    means when you register a handler, there may be a signal which
    invokes neither the default handler or the handler you register.
-   Handlers registered by `signal` in Windows are cleared on first
    signal. To match behavior in other platforms, we re-register the
    handler each time the handler is called, but there’s a moment where
    we miss a handler. That means when you receive two signals in a row,
    there may be a signal which invokes the default handler,
    nevertheless you certainly have registered the handler.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.SigId.html" class="struct"
title="struct signal_hook_registry::SigId">SigId</a>  
An ID of registered action.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.FORBIDDEN.html" class="constant"
title="constant signal_hook_registry::FORBIDDEN">FORBIDDEN</a>  
List of forbidden signals.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.register.html" class="fn"
title="fn signal_hook_registry::register">register</a><sup>⚠</sup>  
Registers an arbitrary action for the given signal.

<a href="fn.register_sigaction.html" class="fn"
title="fn signal_hook_registry::register_sigaction">register_sigaction</a><sup>⚠</sup>  
Register a signal action.

<a href="fn.register_signal_unchecked.html" class="fn"
title="fn signal_hook_registry::register_signal_unchecked">register_signal_unchecked</a><sup>⚠</sup>  
Register a signal action without checking for forbidden signals.

<a href="fn.register_unchecked.html" class="fn"
title="fn signal_hook_registry::register_unchecked">register_unchecked</a><sup>⚠</sup>  
Register a signal action without checking for forbidden signals.

<a href="fn.unregister.html" class="fn"
title="fn signal_hook_registry::unregister">unregister</a>  
Removes a previously installed action.
