[ort](../index.html)

Module environment Copy item path

<span class="sub-heading"><a href="../../src/ort/environment.rs.html#1-700" class="src">Source</a>
</span>

Expand description

The
[`Environment`](struct.Environment.html "struct ort::environment::Environment")
is a process-global configuration under which
[`Session`](../session/struct.Session.html "struct ort::session::Session")s
are created.

With it, you can configure [default execution
providers](struct.EnvironmentBuilder.html#method.with_execution_providers "method ort::environment::EnvironmentBuilder::with_execution_providers"),
enable/disable
[telemetry](struct.EnvironmentBuilder.html#method.with_telemetry "method ort::environment::EnvironmentBuilder::with_telemetry"),
share a [global thread
pool](struct.EnvironmentBuilder.html#method.with_global_thread_pool "method ort::environment::EnvironmentBuilder::with_global_thread_pool")
across all sessions, or add a [custom
logger](struct.EnvironmentBuilder.html#method.with_logger "method ort::environment::EnvironmentBuilder::with_logger").

Environments can be set up via
[`ort::init`](fn.init.html "fn ort::environment::init"):

``` {.rust .rust-example-rendered}
ort::init().with_execution_providers([ep::CUDA::default().build()]).commit();

// ... do other ort things now that our environment is set up...
```

With the `load-dynamic` feature, you can also load the runtime from a
direct path to a DLL with [`ort::init_from`](init_from):

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
let lib_path = std::env::current_exe().unwrap().parent().unwrap().join("lib");
ort::init_from(lib_path.join("onnxruntime.dll"))?
    .with_execution_providers([ep::CUDA::default().build()])
    .commit();
```

If you don’t configure an environment, one will be created with default
settings at the first creation of a session. The environment can’t be
re-configured after one is committed, so it’s important `ort::init` come
before any other `ort` API for the config to take effect. Authors of
libraries using `ort` should **never** have the library configure the
environment itself; allow the application developer to do that
themselves if they wish.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Environment.html" class="struct"
title="struct ort::environment::Environment">Environment</a>

Holds shared global configuration for all
[`Session`](../session/struct.Session.html "struct ort::session::Session")s
in the process.

<a href="struct.EnvironmentBuilder.html" class="struct"
title="struct ort::environment::EnvironmentBuilder">EnvironmentBuilder</a>

Struct used to build an
[`Environment`](struct.Environment.html "struct ort::environment::Environment");
see [`crate::init`](fn.init.html "fn ort::environment::init").

<a href="struct.GlobalThreadPoolOptions.html" class="struct"
title="struct ort::environment::GlobalThreadPoolOptions">GlobalThreadPoolOptions</a>

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ThreadManager.html" class="trait"
title="trait ort::environment::ThreadManager">ThreadManager</a>  
Used for customizing the thread spawning process of a [global thread
pool](struct.GlobalThreadPoolOptions.html "struct ort::environment::GlobalThreadPoolOptions")
or [session thread
pool](../session/builder/struct.SessionBuilder.html#method.with_thread_manager "method ort::session::builder::SessionBuilder::with_thread_manager").
Could be used to add additional initialization/cleanup code to inference
threads for better debugging/error handling.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.current.html" class="fn"
title="fn ort::environment::current">current</a>  
Returns a handle to the currently active `Environment`. If one has not
yet been committed (or an old environment has fallen out of usage), a
new environment will be created & committed.

<a href="fn.init.html" class="fn"
title="fn ort::environment::init">init</a>  
Creates an ONNX Runtime environment.
