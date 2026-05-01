[tokio](../index.html)

Module runtime Copy item path

<span class="sub-heading"><a href="../../src/tokio/runtime/mod.rs.html#1-668"
class="src">Source</a> </span>

Expand description

The Tokio runtime.

Unlike other Rust programs, asynchronous applications require runtime
support. In particular, the following runtime services are necessary:

-   An **I/O event loop**, called the driver, which drives I/O resources
    and dispatches I/O events to tasks that depend on them.
-   A **scheduler** to execute
    [tasks](../task/index.html "mod tokio::task") that use these I/O
    resources.
-   A **timer** for scheduling work to run after a set period of time.

Tokio’s
[`Runtime`](struct.Runtime.html "struct tokio::runtime::Runtime")
bundles all of these services as a single type, allowing them to be
started, shut down, and configured together. However, often it is not
required to configure a
[`Runtime`](struct.Runtime.html "struct tokio::runtime::Runtime")
manually, and a user may just use the [`tokio::main`](../attr.main.html)
attribute macro, which creates a
[`Runtime`](struct.Runtime.html "struct tokio::runtime::Runtime") under
the hood.

# <a href="#choose-your-runtime" class="doc-anchor">§</a>Choose your runtime

Here is the rules of thumb to choose the right runtime for your
application.

``` language-plaintext
   +------------------------------------------------------+
   | Do you want work-stealing or multi-thread scheduler? |
   +------------------------------------------------------+
                   | Yes              | No
                   |                  |
                   |                  |
                   v                  |
     +------------------------+       |
     | Multi-threaded Runtime |       |
     +------------------------+       |
                                      |
                                      V
                     +--------------------------------+
                     | Do you execute `!Send` Future? |
                     +--------------------------------+
                           | Yes                 | No
                           |                     |
                           V                     |
                   +---------------+             |
                   | Local Runtime |             |
                   +---------------+             |
                                                 |
                                                 v
                                     +------------------------+
                                     | Current-thread Runtime |
                                     +------------------------+
```

The above decision tree is not exhaustive. there are other factors that
may influence your decision.

## <a href="#bridging-with-sync-code" class="doc-anchor">§</a>Bridging with sync code

See <https://tokio.rs/tokio/topics/bridging> for details.

## <a href="#numa-awareness" class="doc-anchor">§</a>NUMA awareness

The tokio runtime is not NUMA (Non-Uniform Memory Access) aware. You may
want to start multiple runtimes instead of a single runtime for better
performance on NUMA systems.

# <a href="#usage" class="doc-anchor">§</a>Usage

When no fine tuning is required, the [`tokio::main`](../attr.main.html)
attribute macro can be used.

``` {.rust .rust-example-rendered}
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        println!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    println!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```

From within the context of the runtime, additional tasks are spawned
using the
[`tokio::spawn`](../task/fn.spawn.html "fn tokio::task::spawn")
function. Futures spawned using this function will be executed on the
same thread pool used by the
[`Runtime`](struct.Runtime.html "struct tokio::runtime::Runtime").

A [`Runtime`](struct.Runtime.html "struct tokio::runtime::Runtime")
instance can also be used directly.

``` {.rust .rust-example-rendered}
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    let rt  = Runtime::new()?;

    // Spawn the root task
    rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;

        loop {
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(0) => return,
                        Ok(n) => n,
                        Err(e) => {
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        println!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    })
}
```

## <a href="#runtime-configurations" class="doc-anchor">§</a>Runtime Configurations

Tokio provides multiple task scheduling strategies, suitable for
different applications. The [runtime
builder](struct.Builder.html "struct tokio::runtime::Builder") or
`#[tokio::main]` attribute may be used to select which scheduler to use.

#### <a href="#multi-thread-scheduler" class="doc-anchor">§</a>Multi-Thread Scheduler

The multi-thread scheduler executes futures on a *thread pool*, using a
work-stealing strategy. By default, it will start a worker thread for
each CPU core available on the system. This tends to be the ideal
configuration for most applications. The multi-thread scheduler requires
the `rt-multi-thread` feature flag, and is selected by default:

``` {.rust .rust-example-rendered}
use tokio::runtime;

let threaded_rt = runtime::Runtime::new()?;
```

Most applications should use the multi-thread scheduler, except in some
niche use-cases, such as when running only a single thread is required.

#### <a href="#current-thread-scheduler" class="doc-anchor">§</a>Current-Thread Scheduler

The current-thread scheduler provides a *single-threaded* future
executor. All tasks will be created and executed on the current thread.
This requires the `rt` feature flag.

``` {.rust .rust-example-rendered}
use tokio::runtime;

let rt = runtime::Builder::new_current_thread()
    .build()?;
```

#### <a href="#resource-drivers" class="doc-anchor">§</a>Resource drivers

When configuring a runtime by hand, no resource drivers are enabled by
default. In this case, attempting to use networking types or time types
will fail. In order to enable these types, the resource drivers must be
enabled. This is done with
[`Builder::enable_io`](struct.Builder.html#method.enable_io "method tokio::runtime::Builder::enable_io")
and
[`Builder::enable_time`](struct.Builder.html#method.enable_time "method tokio::runtime::Builder::enable_time").
As a shorthand,
[`Builder::enable_all`](struct.Builder.html#method.enable_all "method tokio::runtime::Builder::enable_all")
enables both resource drivers.

## <a href="#driving-the-runtime" class="doc-anchor">§</a>Driving the runtime

A Tokio runtime can only execute tasks if the runtime is running.
Normally this is not an issue as the default configuration of a runtime
is always running, but alternate configurations such as the
current-thread runtime require that
[`Runtime::block_on`](struct.Runtime.html#method.block_on "method tokio::runtime::Runtime::block_on")
is called.

-   A multi-threaded runtime is always running because it spawns its own
    worker threads.
-   A current-thread runtime does not spawn any worker threads, so it
    can only execute tasks when you provide a thread by calling
    [`Runtime::block_on`](struct.Runtime.html#method.block_on "method tokio::runtime::Runtime::block_on").
-   A
    [`LocalSet`](../task/struct.LocalSet.html "struct tokio::task::LocalSet")
    only executes local tasks spawned on it when the `LocalSet` is
    `.awaited` or otherwise driven using one of its methods for this
    purpose.

Please be aware that
[`Handle::block_on`](struct.Handle.html#method.block_on "method tokio::runtime::Handle::block_on")
does not drive the runtime. There must be at least one call to
[`Runtime::block_on`](struct.Runtime.html#method.block_on "method tokio::runtime::Runtime::block_on")
when using the current thread runtime.
[`Handle::block_on`](struct.Handle.html#method.block_on "method tokio::runtime::Handle::block_on")
is not enough.

## <a href="#lifetime-of-spawned-threads" class="doc-anchor">§</a>Lifetime of spawned threads

The runtime may spawn threads depending on its configuration and usage.
The multi-thread scheduler spawns threads to schedule tasks and for
`spawn_blocking` calls.

While the `Runtime` is active, threads may shut down after periods of
being idle. Once `Runtime` is dropped, all runtime threads have usually
been terminated, but in the presence of unstoppable spawned work are not
guaranteed to have been terminated. See the [struct level
documentation](struct.Runtime.html#shutdown "struct tokio::runtime::Runtime")
for more details.

# <a href="#detailed-runtime-behavior" class="doc-anchor">§</a>Detailed runtime behavior

This section gives more details into how the Tokio runtime will schedule
tasks for execution.

At its most basic level, a runtime has a collection of tasks that need
to be scheduled. It will repeatedly remove a task from that collection
and schedule it (by calling
[`poll`](https://doc.rust-lang.org/1.93.1/core/future/future/trait.Future.html#tymethod.poll "method core::future::future::Future::poll")).
When the collection is empty, the thread will go to sleep until a task
is added to the collection.

However, the above is not sufficient to guarantee a well-behaved
runtime. For example, the runtime might have a single task that is
always ready to be scheduled, and schedule that task every time. This is
a problem because it starves other tasks by not scheduling them. To
solve this, Tokio provides the following fairness guarantee:

> If the total number of tasks does not grow without bound, and no task
> is [blocking the
> thread](https://ryhl.io/blog/async-what-is-blocking/), then it is
> guaranteed that tasks are scheduled fairly.

Or, more formally:

> Under the following two assumptions:
>
> -   There is some number `MAX_TASKS` such that the total number of
>     tasks on the runtime at any specific point in time never exceeds
>     `MAX_TASKS`.
> -   There is some number `MAX_SCHEDULE` such that calling
>     [`poll`](https://doc.rust-lang.org/1.93.1/core/future/future/trait.Future.html#tymethod.poll "method core::future::future::Future::poll")
>     on any task spawned on the runtime returns within `MAX_SCHEDULE`
>     time units.
>
> Then, there is some number `MAX_DELAY` such that when a task is woken,
> it will be scheduled by the runtime within `MAX_DELAY` time units.

(Here, `MAX_TASKS` and `MAX_SCHEDULE` can be any number and the user of
the runtime may choose them. The `MAX_DELAY` number is controlled by the
runtime, and depends on the value of `MAX_TASKS` and `MAX_SCHEDULE`.)

Other than the above fairness guarantee, there is no guarantee about the
order in which tasks are scheduled. There is also no guarantee that the
runtime is equally fair to all tasks. For example, if the runtime has
two tasks A and B that are both ready, then the runtime may schedule A
five times before it schedules B. This is the case even if A yields
using
[`yield_now`](../task/fn.yield_now.html "fn tokio::task::yield_now").
All that is guaranteed is that it will schedule B eventually.

Normally, tasks are scheduled only if they have been woken by calling
[`wake`](https://doc.rust-lang.org/1.93.1/core/task/wake/struct.Waker.html#method.wake "method core::task::wake::Waker::wake")
on their waker. However, this is not guaranteed, and Tokio may schedule
tasks that have not been woken under some circumstances. This is called
a spurious wakeup.

## <a href="#io-and-timers" class="doc-anchor">§</a>IO and timers

Beyond just scheduling tasks, the runtime must also manage IO resources
and timers. It does this by periodically checking whether there are any
IO resources or timers that are ready, and waking the relevant task so
that it will be scheduled.

These checks are performed periodically between scheduling tasks. Under
the same assumptions as the previous fairness guarantee, Tokio
guarantees that it will wake tasks with an IO or timer event within some
maximum number of time units.

## <a href="#current-thread-runtime-behavior-at-the-time-of-writing"
class="doc-anchor">§</a>Current thread runtime (behavior at the time of writing)

This section describes how the [current thread
runtime](struct.Builder.html#method.new_current_thread "associated function tokio::runtime::Builder::new_current_thread")
behaves today. This behavior may change in future versions of Tokio.

The current thread runtime maintains two FIFO queues of tasks that are
ready to be scheduled: the global queue and the local queue. The runtime
will prefer to choose the next task to schedule from the local queue,
and will only pick a task from the global queue if the local queue is
empty, or if it has picked a task from the local queue 31 times in a
row. The number 31 can be changed using the
[`global_queue_interval`](struct.Builder.html#method.global_queue_interval "method tokio::runtime::Builder::global_queue_interval")
setting.

The runtime will check for new IO or timer events whenever there are no
tasks ready to be scheduled, or when it has scheduled 61 tasks in a row.
The number 61 may be changed using the
[`event_interval`](struct.Builder.html#method.event_interval "method tokio::runtime::Builder::event_interval")
setting.

When a task is woken from within a task running on the runtime, then the
woken task is added directly to the local queue. Otherwise, the task is
added to the global queue. The current thread runtime does not use [the
lifo slot
optimization](struct.Builder.html#structfield.disable_lifo_slot "field tokio::runtime::Builder::disable_lifo_slot").

## <a href="#multi-threaded-runtime-behavior-at-the-time-of-writing"
class="doc-anchor">§</a>Multi threaded runtime (behavior at the time of writing)

This section describes how the [multi thread
runtime](struct.Builder.html#method.new_multi_thread "associated function tokio::runtime::Builder::new_multi_thread")
behaves today. This behavior may change in future versions of Tokio.

A multi thread runtime has a fixed number of worker threads, which are
all created on startup. The multi thread runtime maintains one global
queue, and a local queue for each worker thread. The local queue of a
worker thread can fit at most 256 tasks. If more than 256 tasks are
added to the local queue, then half of them are moved to the global
queue to make space.

The runtime will prefer to choose the next task to schedule from the
local queue, and will only pick a task from the global queue if the
local queue is empty, or if it has picked a task from the local queue
[`global_queue_interval`](struct.Builder.html#method.global_queue_interval "method tokio::runtime::Builder::global_queue_interval")
times in a row. If the value of
[`global_queue_interval`](struct.Builder.html#method.global_queue_interval "method tokio::runtime::Builder::global_queue_interval")
is not explicitly set using the runtime builder, then the runtime will
dynamically compute it using a heuristic that targets 10ms intervals
between each check of the global queue (based on the
[`worker_mean_poll_time`](crate::runtime::RuntimeMetrics::worker_mean_poll_time)
metric).

If both the local queue and global queue is empty, then the worker
thread will attempt to steal tasks from the local queue of another
worker thread. Stealing is done by moving half of the tasks in one local
queue to another local queue.

The runtime will check for new IO or timer events whenever there are no
tasks ready to be scheduled, or when it has scheduled 61 tasks in a row.
The number 61 may be changed using the
[`event_interval`](struct.Builder.html#method.event_interval "method tokio::runtime::Builder::event_interval")
setting.

The multi thread runtime uses [the lifo slot
optimization](struct.Builder.html#structfield.disable_lifo_slot "field tokio::runtime::Builder::disable_lifo_slot"):
Whenever a task wakes up another task, the other task is added to the
worker thread’s lifo slot instead of being added to a queue. If there
was already a task in the lifo slot when this happened, then the lifo
slot is replaced, and the task that used to be in the lifo slot is
placed in the thread’s local queue. When the runtime finishes scheduling
a task, it will schedule the task in the lifo slot immediately, if any.
When the lifo slot is used, the [coop
budget](../task/coop/index.html#cooperative-scheduling "mod tokio::task::coop")
is not reset. Furthermore, if a worker thread uses the lifo slot three
times in a row, it is temporarily disabled until the worker thread has
scheduled a task that didn’t come from the lifo slot. The lifo slot can
be disabled using the
[`disable_lifo_slot`](struct.Builder.html#structfield.disable_lifo_slot "field tokio::runtime::Builder::disable_lifo_slot")
setting. The lifo slot is separate from the local queue, and is stolen
from by other worker threads only when a worker’s local queue has been
drained.

When a task is woken from a thread that is not a worker thread, then the
task is placed in the global queue.

# <a href="#performance-tuning" class="doc-anchor">§</a>Performance tuning

## <a href="#file-descriptor-table-pre-warming" class="doc-anchor">§</a>File descriptor table pre-warming

On Linux, file descriptor table growth can stall worker threads. See the
[`prewarm-fd-table`](https://github.com/tokio-rs/tokio/blob/master/examples/prewarm-fd-table.rs)
example.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Builder.html" class="struct"
title="struct tokio::runtime::Builder">Builder</a>  
Builds Tokio Runtime with custom configuration values.

<a href="struct.EnterGuard.html" class="struct"
title="struct tokio::runtime::EnterGuard">EnterGuard</a>  
Runtime context guard.

<a href="struct.Handle.html" class="struct"
title="struct tokio::runtime::Handle">Handle</a>  
Handle to the runtime.

<a href="struct.Id.html" class="struct"
title="struct tokio::runtime::Id">Id</a>  
An opaque ID that uniquely identifies a runtime relative to all other
currently running runtimes.

<a href="struct.LocalOptions.html" class="struct"
title="struct tokio::runtime::LocalOptions">LocalOptions</a>  
[`LocalRuntime`](struct.LocalRuntime.html "struct tokio::runtime::LocalRuntime")-only
config options

<a href="struct.LocalRuntime.html" class="struct"
title="struct tokio::runtime::LocalRuntime">LocalRuntime</a>  
A local Tokio runtime.

<a href="struct.Runtime.html" class="struct"
title="struct tokio::runtime::Runtime">Runtime</a>  
The Tokio runtime.

<a href="struct.RuntimeMetrics.html" class="struct"
title="struct tokio::runtime::RuntimeMetrics">RuntimeMetrics</a>  
Handle to the runtime’s metrics.

<a href="struct.TryCurrentError.html" class="struct"
title="struct tokio::runtime::TryCurrentError">TryCurrentError</a>  
Error returned by `try_current` when no Runtime has been started

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.RuntimeFlavor.html" class="enum"
title="enum tokio::runtime::RuntimeFlavor">RuntimeFlavor</a>  
The flavor of a `Runtime`.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.is_rt_shutdown_err.html" class="fn"
title="fn tokio::runtime::is_rt_shutdown_err">is_rt_shutdown_err</a>  
Checks whether the given error was emitted by Tokio when shutting down
its runtime.
