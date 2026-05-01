[axum](../index.html)

Module routing Copy item path

<span class="sub-heading"><a href="../../src/axum/routing/mod.rs.html#1-761"
class="src">Source</a> </span>

Expand description

Routing between
[`Service`](../../tower_service/trait.Service.html "trait tower_service::Service")s
and handlers.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use self::method_routing::`<a href="method_routing/fn.any.html" class="fn"
title="fn axum::routing::method_routing::any"><code>any</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.any_service.html" class="fn"
title="fn axum::routing::method_routing::any_service"><code>any_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.connect.html" class="fn"
title="fn axum::routing::method_routing::connect"><code>connect</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.connect_service.html" class="fn"
title="fn axum::routing::method_routing::connect_service"><code>connect_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.delete.html" class="fn"
title="fn axum::routing::method_routing::delete"><code>delete</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.delete_service.html" class="fn"
title="fn axum::routing::method_routing::delete_service"><code>delete_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.get.html" class="fn"
title="fn axum::routing::method_routing::get"><code>get</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.get_service.html" class="fn"
title="fn axum::routing::method_routing::get_service"><code>get_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.head.html" class="fn"
title="fn axum::routing::method_routing::head"><code>head</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.head_service.html" class="fn"
title="fn axum::routing::method_routing::head_service"><code>head_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.on.html" class="fn"
title="fn axum::routing::method_routing::on"><code>on</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.on_service.html" class="fn"
title="fn axum::routing::method_routing::on_service"><code>on_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.options.html" class="fn"
title="fn axum::routing::method_routing::options"><code>options</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.options_service.html" class="fn"
title="fn axum::routing::method_routing::options_service"><code>options_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.patch.html" class="fn"
title="fn axum::routing::method_routing::patch"><code>patch</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.patch_service.html" class="fn"
title="fn axum::routing::method_routing::patch_service"><code>patch_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.post.html" class="fn"
title="fn axum::routing::method_routing::post"><code>post</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.post_service.html" class="fn"
title="fn axum::routing::method_routing::post_service"><code>post_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.put.html" class="fn"
title="fn axum::routing::method_routing::put"><code>put</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.put_service.html" class="fn"
title="fn axum::routing::method_routing::put_service"><code>put_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.trace.html" class="fn"
title="fn axum::routing::method_routing::trace"><code>trace</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/fn.trace_service.html" class="fn"
title="fn axum::routing::method_routing::trace_service"><code>trace_service</code></a>`;`

`pub use self::method_routing::`<a href="method_routing/struct.MethodRouter.html" class="struct"
title="struct axum::routing::method_routing::MethodRouter"><code>MethodRouter</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="future/index.html" class="mod"
title="mod axum::routing::future">future</a>  
Future types.

<a href="method_routing/index.html" class="mod"
title="mod axum::routing::method_routing">method_routing</a>  
Route to services and handlers based on HTTP methods.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.IntoMakeService.html" class="struct"
title="struct axum::routing::IntoMakeService">IntoMakeService</a>  
A
[`MakeService`](../../tower/make/make_service/trait.MakeService.html "trait tower::make::make_service::MakeService")
that produces axum router services.

<a href="struct.MethodFilter.html" class="struct"
title="struct axum::routing::MethodFilter">MethodFilter</a>  
A filter that matches one or more HTTP methods.

<a href="struct.Route.html" class="struct"
title="struct axum::routing::Route">Route</a>  
How routes are stored inside a
[`Router`](../struct.Router.html "struct axum::Router").

<a href="struct.Router.html" class="struct"
title="struct axum::routing::Router">Router</a>  
The router type for composing handlers and services.

<a href="struct.RouterAsService.html" class="struct"
title="struct axum::routing::RouterAsService">RouterAsService</a>  
A [`Router`](../struct.Router.html "struct axum::Router") converted into
a borrowed
[`Service`](../../tower_service/trait.Service.html "trait tower_service::Service")
with a fixed body type.

<a href="struct.RouterIntoService.html" class="struct"
title="struct axum::routing::RouterIntoService">RouterIntoService</a>  
A [`Router`](../struct.Router.html "struct axum::Router") converted into
an owned
[`Service`](../../tower_service/trait.Service.html "trait tower_service::Service")
with a fixed body type.
