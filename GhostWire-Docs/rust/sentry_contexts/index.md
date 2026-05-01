Crate sentry\_contexts Copy item path

<span class="sub-heading"><a href="../src/sentry_contexts/lib.rs.html#1-27" class="src">Source</a>
</span>

Expand description

Adds Contexts to Sentry Events.

This integration is enabled by default in `sentry` and adds `device`,
`os` and `rust` contexts to Events, and also sets a `server_name` if it
is not already defined.

See the [Contexts
Interface](https://develop.sentry.dev/sdk/event-payloads/contexts/)
documentation for more info.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
let integration = sentry_contexts::ContextIntegration::new().add_os(false);
let _sentry = sentry::init(sentry::ClientOptions::new().add_integration(integration));
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="utils/index.html" class="mod"
title="mod sentry_contexts::utils">utils</a>  
Contains functions to retrieve various contexts that can be useful to
attach to events

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ContextIntegration.html" class="struct"
title="struct sentry_contexts::ContextIntegration">ContextIntegration</a>  
Adds Contexts to Sentry Events.
