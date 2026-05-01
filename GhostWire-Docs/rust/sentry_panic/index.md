Crate sentry\_panic Copy item path

<span class="sub-heading"><a href="../src/sentry_panic/lib.rs.html#1-142" class="src">Source</a>
</span>

Expand description

The Sentry Panic handler integration.

The `PanicIntegration`, which is enabled by default in `sentry`,
installs a panic handler that will automatically dispatch all errors to
Sentry that are caused by a panic. Additionally, panics are forwarded to
the previously registered panic hook.

# <a href="#configuration" class="doc-anchor">§</a>Configuration

The panic integration can be configured with an additional extractor,
which might optionally create a sentry `Event` out of a `PanicInfo`.

``` {.rust .rust-example-rendered}
let integration = sentry_panic::PanicIntegration::default().add_extractor(|info| None);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.PanicIntegration.html" class="struct"
title="struct sentry_panic::PanicIntegration">PanicIntegration</a>  
The Sentry Panic handler Integration.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.message_from_panic_info.html" class="fn"
title="fn sentry_panic::message_from_panic_info">message_from_panic_info</a>  
Extract the message of a panic.

<a href="fn.panic_handler.html" class="fn"
title="fn sentry_panic::panic_handler">panic_handler</a>  
A panic handler that sends to Sentry.
