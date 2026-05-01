Crate sentry\_debug\_images Copy item path

<span class="sub-heading"><a href="../src/sentry_debug_images/lib.rs.html#1-28"
class="src">Source</a> </span>

Expand description

The Sentry Debug Images integration.

The
[`DebugImagesIntegration`](struct.DebugImagesIntegration.html "struct sentry_debug_images::DebugImagesIntegration")
adds metadata about the loaded shared libraries to Sentry
[`Event`](../sentry_types/protocol/v7/struct.Event.html "struct sentry_types::protocol::v7::Event")s.

# <a href="#configuration" class="doc-anchor">§</a>Configuration

The integration by default attaches this information to all
[`Event`](../sentry_types/protocol/v7/struct.Event.html "struct sentry_types::protocol::v7::Event")s,
but a custom filter can be defined as well.

``` {.rust .rust-example-rendered}
use sentry_core::Level;
let integration = sentry_debug_images::DebugImagesIntegration::new()
    .filter(|event| event.level >= Level::Warning);
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DebugImagesIntegration.html" class="struct"
title="struct sentry_debug_images::DebugImagesIntegration">DebugImagesIntegration</a>  
The Sentry Debug Images Integration.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.debug_images.html" class="fn"
title="fn sentry_debug_images::debug_images">debug_images</a>  
Returns the list of loaded libraries/images.
