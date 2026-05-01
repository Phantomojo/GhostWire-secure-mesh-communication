Crate tower\_layer Copy item path

<span class="sub-heading"><a href="../src/tower_layer/lib.rs.html#1-112" class="src">Source</a>
</span>

Expand description

Layer traits and extensions.

A layer decorates an service and provides additional functionality. It
allows other services to be composed with the service that implements
layer.

A middleware implements the
[`Layer`](trait.Layer.html "trait tower_layer::Layer") and
[`Service`](https://docs.rs/tower/*/tower/trait.Service.html) trait.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Identity.html" class="struct"
title="struct tower_layer::Identity">Identity</a>  
A no-op middleware.

<a href="struct.LayerFn.html" class="struct"
title="struct tower_layer::LayerFn">LayerFn</a>  
A `Layer` implemented by a closure. See the docs for
[`layer_fn`](fn.layer_fn.html "fn tower_layer::layer_fn") for more
details.

<a href="struct.Stack.html" class="struct"
title="struct tower_layer::Stack">Stack</a>  
Two middlewares chained together.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Layer.html" class="trait"
title="trait tower_layer::Layer">Layer</a>  
Decorates a
[`Service`](https://docs.rs/tower/*/tower/trait.Service.html),
transforming either the request or the response.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.layer_fn.html" class="fn"
title="fn tower_layer::layer_fn">layer_fn</a>  
Returns a new
[`LayerFn`](struct.LayerFn.html "struct tower_layer::LayerFn") that
implements [`Layer`](trait.Layer.html "trait tower_layer::Layer") by
calling the given function.
