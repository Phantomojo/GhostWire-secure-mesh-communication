[tokio](../index.html)

Module task Copy item path

<span class="sub-heading"><a href="../../src/tokio/task/mod.rs.html#1-335" class="src">Source</a>
</span>

Expand description

Asynchronous green-threads.

## <a href="#what-are-tasks" class="doc-anchor">§</a>What are Tasks?

A *task* is a light weight, non-blocking unit of execution. A task is
similar to an OS thread, but rather than being managed by the OS
scheduler, they are managed by the [Tokio
runtime](../runtime/index.html "mod tokio::runtime"). Another name for
this general pattern is [green
threads](https://en.wikipedia.org/wiki/Green_threads). If you are
familiar with [Go’s goroutines](https://tour.golang.org/concurrency/1),
[Kotlin’s
coroutines](https://kotlinlang.org/docs/reference/coroutines-overview.html),
or [Erlang’s
processes](http://erlang.org/doc/getting_started/conc_prog.html#processes),
you can think of Tokio’s tasks as something similar.

Key points about tasks include:

-   Tasks are **light weight**. Because tasks are scheduled by the Tokio
    runtime rather than the operating system, creating new tasks or
    switching between tasks does not require a context switch and has
    fairly low overhead. Creating, running, and destroying large numbers
    of tasks is quite cheap, especially compared to OS threads.

-   Tasks are scheduled **cooperatively**. Most operating systems
    implement *preemptive multitasking*. This is a scheduling technique
    where the operating system allows each thread to run for a period of
    time, and then *preempts* it, temporarily pausing that thread and
    switching to another. Tasks, on the other hand, implement
    *cooperative multitasking*. In cooperative multitasking, a task is
    allowed to run until it *yields*, indicating to the Tokio runtime’s
    scheduler that it cannot currently continue executing. When a task
    yields, the Tokio runtime switches to executing the next task.

-   Tasks are **non-blocking**. Typically, when an OS thread performs
    I/O or must synchronize with another thread, it *blocks*, allowing
    the OS to schedule another thread. When a task cannot continue
    executing, it must yield instead, allowing the Tokio runtime to
    schedule another task. Tasks should generally not perform system
    calls or other operations that could block a thread, as this would
    prevent other tasks running on the same thread from executing as
    well. Instead, this module provides APIs for running blocking
    operations in an asynchronous context.

## <a href="#working-with-tasks" class="doc-anchor">§</a>Working with Tasks

This module provides the following APIs for working with tasks:

### <a href="#spawning" class="doc-anchor">§</a>Spawning

Perhaps the most important function in this module is
[`task::spawn`](fn.spawn.html "fn tokio::task::spawn"). This function
can be thought of as an async equivalent to the standard library’s
[`thread::spawn`](https://doc.rust-lang.org/1.93.1/std/thread/functions/fn.spawn.html "fn std::thread::functions::spawn").
It takes an `async` block or other
[future](https://doc.rust-lang.org/1.93.1/core/future/future/trait.Future.html "trait core::future::future::Future"),
and creates a new task to run that work concurrently:

``` {.rust .rust-example-rendered}
use tokio::task;

task::spawn(async {
    // perform some work here...
});
```

Like
[`std::thread::spawn`](https://doc.rust-lang.org/1.93.1/std/thread/functions/fn.spawn.html "fn std::thread::functions::spawn"),
`task::spawn` returns a
[`JoinHandle`](struct.JoinHandle.html "struct tokio::task::JoinHandle")
struct. A `JoinHandle` is itself a future which may be used to await the
output of the spawned task. For example:

``` {.rust .rust-example-rendered}
use tokio::task;

let join = task::spawn(async {
    // ...
    "hello world!"
});

// ...

// Await the result of the spawned task.
let result = join.await?;
assert_eq!(result, "hello world!");
```

Again, like `std::thread`’s [`JoinHandle`
type](https://doc.rust-lang.org/1.93.1/std/thread/join_handle/struct.JoinHandle.html "struct std::thread::join_handle::JoinHandle"),
if the spawned task panics, awaiting its `JoinHandle` will return a
[`JoinError`](struct.JoinError.html "struct tokio::task::JoinError").
For example:

``` {.rust .rust-example-rendered}
use tokio::task;

let join = task::spawn(async {
    panic!("something bad happened!")
});

// The returned result indicates that the task failed.
assert!(join.await.is_err());
```

`spawn`, `JoinHandle`, and `JoinError` are present when the “rt” feature
flag is enabled.

#### <a href="#cancellation" class="doc-anchor">§</a>Cancellation

Spawned tasks may be cancelled using the
[`JoinHandle::abort`](struct.JoinHandle.html#method.abort "method tokio::task::JoinHandle::abort")
or
[`AbortHandle::abort`](struct.AbortHandle.html#method.abort "method tokio::task::AbortHandle::abort")
methods. When one of these methods are called, the task is signalled to
shut down next time it yields at an `.await` point. If the task is
already idle, then it will be shut down as soon as possible without
running again before being shut down. Additionally, shutting down a
Tokio runtime (e.g. by returning from `#[tokio::main]`) immediately
cancels all tasks on it.

When tasks are shut down, it will stop running at whichever `.await` it
has yielded at. All local variables are destroyed by running their
destructor. Once shutdown has completed, awaiting the
[`JoinHandle`](struct.JoinHandle.html "struct tokio::task::JoinHandle")
will fail with a [cancelled
error](struct.JoinError.html#method.is_cancelled "method tokio::task::JoinError::is_cancelled").

Note that aborting a task does not guarantee that it fails with a
cancelled error, since it may complete normally first. For example, if
the task does not yield to the runtime at any point between the call to
`abort` and the end of the task, then the
[`JoinHandle`](struct.JoinHandle.html "struct tokio::task::JoinHandle")
will instead report that the task exited normally.

Be aware that tasks spawned using
[`spawn_blocking`](fn.spawn_blocking.html "fn tokio::task::spawn_blocking")
cannot be aborted because they are not async. If you call `abort` on a
`spawn_blocking` task, then this *will not have any effect*, and the
task will continue running normally. The exception is if the task has
not started running yet; in that case, calling `abort` may prevent the
task from starting.

Be aware that calls to
[`JoinHandle::abort`](struct.JoinHandle.html#method.abort "method tokio::task::JoinHandle::abort")
just schedule the task for cancellation, and will return before the
cancellation has completed. To wait for cancellation to complete, wait
for the task to finish by awaiting the
[`JoinHandle`](struct.JoinHandle.html "struct tokio::task::JoinHandle").
Similarly, the
[`JoinHandle::is_finished`](struct.JoinHandle.html#method.is_finished "method tokio::task::JoinHandle::is_finished")
method does not return `true` until the cancellation has finished.

Calling
[`JoinHandle::abort`](struct.JoinHandle.html#method.abort "method tokio::task::JoinHandle::abort")
multiple times has the same effect as calling it once.

Tokio also provides an
[`AbortHandle`](struct.AbortHandle.html "struct tokio::task::AbortHandle"),
which is like the
[`JoinHandle`](struct.JoinHandle.html "struct tokio::task::JoinHandle"),
except that it does not provide a mechanism to wait for the task to
finish. Each task can only have one
[`JoinHandle`](struct.JoinHandle.html "struct tokio::task::JoinHandle"),
but it can have more than one
[`AbortHandle`](struct.AbortHandle.html "struct tokio::task::AbortHandle").

### <a href="#blocking-and-yielding" class="doc-anchor">§</a>Blocking and Yielding

As we discussed above, code running in asynchronous tasks should not
perform operations that can block. A blocking operation performed in a
task running on a thread that is also running other tasks would block
the entire thread, preventing other tasks from running.

Instead, Tokio provides two APIs for running blocking operations in an
asynchronous context:
[`task::spawn_blocking`](fn.spawn_blocking.html "fn tokio::task::spawn_blocking")
and
[`task::block_in_place`](fn.block_in_place.html "fn tokio::task::block_in_place").

Be aware that if you call a non-async method from async code, that
non-async method is still inside the asynchronous context, so you should
also avoid blocking operations there. This includes destructors of
objects destroyed in async code.

#### <a href="#spawn_blocking" class="doc-anchor">§</a>`spawn_blocking`

The `task::spawn_blocking` function is similar to the `task::spawn`
function discussed in the previous section, but rather than spawning a
*non-blocking* future on the Tokio runtime, it instead spawns a
*blocking* function on a dedicated thread pool for blocking tasks. For
example:

``` {.rust .rust-example-rendered}
use tokio::task;

task::spawn_blocking(|| {
    // do some compute-heavy work or call synchronous code
});
```

Just like `task::spawn`, `task::spawn_blocking` returns a `JoinHandle`
which we can use to await the result of the blocking operation:

``` {.rust .rust-example-rendered}
let join = task::spawn_blocking(|| {
    // do some compute-heavy work or call synchronous code
    "blocking completed"
});

let result = join.await?;
assert_eq!(result, "blocking completed");
```

#### <a href="#block_in_place" class="doc-anchor">§</a>`block_in_place`

When using the [multi-threaded
runtime](../runtime/index.html#threaded-scheduler), the
[`task::block_in_place`](fn.block_in_place.html "fn tokio::task::block_in_place")
function is also available. Like `task::spawn_blocking`, this function
allows running a blocking operation from an asynchronous context. Unlike
`spawn_blocking`, however, `block_in_place` works by transitioning the
*current* worker thread to a blocking thread, moving other tasks running
on that thread to another worker thread. This can improve performance by
avoiding context switches.

For example:

``` {.rust .rust-example-rendered}
use tokio::task;

let result = task::block_in_place(|| {
    // do some compute-heavy work or call synchronous code
    "blocking completed"
});

assert_eq!(result, "blocking completed");
```

#### <a href="#yield_now" class="doc-anchor">§</a>`yield_now`

In addition, this module provides a
[`task::yield_now`](fn.yield_now.html "fn tokio::task::yield_now") async
function that is analogous to the standard library’s
[`thread::yield_now`](https://doc.rust-lang.org/1.93.1/std/thread/functions/fn.yield_now.html "fn std::thread::functions::yield_now").
Calling and `await`ing this function will cause the current task to
yield to the Tokio runtime’s scheduler, allowing other tasks to be
scheduled. Eventually, the yielding task will be polled again, allowing
it to execute. For example:

``` {.rust .rust-example-rendered}
use tokio::task;

async {
    task::spawn(async {
        // ...
        println!("spawned task done!")
    });

    // Yield, allowing the newly-spawned task to execute first.
    task::yield_now().await;
    println!("main task done!");
}
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="coop/index.html" class="mod"
title="mod tokio::task::coop">coop</a>  
Utilities for improved cooperative scheduling.

<a href="futures/index.html" class="mod"
title="mod tokio::task::futures">futures</a>  
Task-related futures.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AbortHandle.html" class="struct"
title="struct tokio::task::AbortHandle">AbortHandle</a>  
An owned permission to abort a spawned task, without awaiting its
completion.

<a href="struct.Id.html" class="struct"
title="struct tokio::task::Id">Id</a>  
An opaque ID that uniquely identifies a task relative to all other
currently running tasks.

<a href="struct.JoinError.html" class="struct"
title="struct tokio::task::JoinError">JoinError</a>  
Task failed to execute to completion.

<a href="struct.JoinHandle.html" class="struct"
title="struct tokio::task::JoinHandle">JoinHandle</a>  
An owned permission to join on a task (await its termination).

<a href="struct.JoinSet.html" class="struct"
title="struct tokio::task::JoinSet">JoinSet</a>  
A collection of tasks spawned on a Tokio runtime.

<a href="struct.LocalEnterGuard.html" class="struct"
title="struct tokio::task::LocalEnterGuard">LocalEnterGuard</a>  
Context guard for `LocalSet`

<a href="struct.LocalKey.html" class="struct"
title="struct tokio::task::LocalKey">LocalKey</a>  
A key for task-local data.

<a href="struct.LocalSet.html" class="struct"
title="struct tokio::task::LocalSet">LocalSet</a>  
A set of tasks which are executed on the same thread.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.block_in_place.html" class="fn"
title="fn tokio::task::block_in_place">block_in_place</a>  
Runs the provided blocking function on the current thread without
blocking the executor.

<a href="fn.id.html" class="fn" title="fn tokio::task::id">id</a>  
Returns the [`Id`](struct.Id.html "struct tokio::task::Id") of the
currently running task.

<a href="fn.spawn.html" class="fn"
title="fn tokio::task::spawn">spawn</a>  
Spawns a new asynchronous task, returning a
[`JoinHandle`](struct.JoinHandle.html "struct tokio::task::JoinHandle")
for it.

<a href="fn.spawn_blocking.html" class="fn"
title="fn tokio::task::spawn_blocking">spawn_blocking</a>  
Runs the provided closure on a thread where blocking is acceptable.

<a href="fn.spawn_local.html" class="fn"
title="fn tokio::task::spawn_local">spawn_local</a>  
Spawns a `!Send` future on the current
[`LocalSet`](struct.LocalSet.html "struct tokio::task::LocalSet") or
[`LocalRuntime`](../runtime/struct.LocalRuntime.html "struct tokio::runtime::LocalRuntime").

<a href="fn.try_id.html" class="fn"
title="fn tokio::task::try_id">try_id</a>  
Returns the [`Id`](struct.Id.html "struct tokio::task::Id") of the
currently running task, or `None` if called outside of a task.

<a href="fn.yield_now.html" class="fn"
title="fn tokio::task::yield_now">yield_now</a>  
Yields execution back to the Tokio runtime.
