Crate futures\_executor Copy item path

<span class="sub-heading"><a href="../src/futures_executor/lib.rs.html#1-73"
class="src">Source</a> </span>

Expand description

Built-in executors and related tools.

All asynchronous computation occurs within an executor, which is capable
of spawning futures as tasks. This module provides several built-in
executors, as well as tools for building your own.

All items are only available when the `std` feature of this library is
activated, and it is activated by default.

# <a href="#using-a-thread-pool-mn-task-scheduling"
class="doc-anchor">§</a>Using a thread pool (M:N task scheduling)

Most of the time tasks should be executed on a [thread
pool](struct.ThreadPool.html "struct futures_executor::ThreadPool"). A
small set of worker threads can handle a very large set of spawned tasks
(which are much lighter weight than threads). Tasks spawned onto the
pool with the
[`spawn_ok`](struct.ThreadPool.html#method.spawn_ok "method futures_executor::ThreadPool::spawn_ok")
function will run ambiently on the created threads.

# <a href="#spawning-additional-tasks" class="doc-anchor">§</a>Spawning additional tasks

Tasks can be spawned onto a spawner by calling its
[`spawn_obj`](https://docs.rs/futures/0.3/futures/task/trait.Spawn.html#tymethod.spawn_obj)
method directly. In the case of `!Send` futures,
[`spawn_local_obj`](https://docs.rs/futures/0.3/futures/task/trait.LocalSpawn.html#tymethod.spawn_local_obj)
can be used instead.

# <a href="#single-threaded-execution" class="doc-anchor">§</a>Single-threaded execution

In addition to thread pools, it’s possible to run a task (and the tasks
it spawns) entirely within a single thread via the
[`LocalPool`](struct.LocalPool.html "struct futures_executor::LocalPool")
executor. Aside from cutting down on synchronization costs, this
executor also makes it possible to spawn non-`Send` tasks, via
[`spawn_local_obj`](https://docs.rs/futures/0.3/futures/task/trait.LocalSpawn.html#tymethod.spawn_local_obj).
The
[`LocalPool`](struct.LocalPool.html "struct futures_executor::LocalPool")
is best suited for running I/O-bound tasks that do relatively little
work between I/O operations.

There is also a convenience function
[`block_on`](fn.block_on.html "fn futures_executor::block_on") for
simply running a future to completion on the current thread.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BlockingStream.html" class="struct"
title="struct futures_executor::BlockingStream">BlockingStream</a>  
An iterator which blocks on values from a stream until they become
available.

<a href="struct.Enter.html" class="struct"
title="struct futures_executor::Enter">Enter</a>  
Represents an executor context.

<a href="struct.EnterError.html" class="struct"
title="struct futures_executor::EnterError">EnterError</a>  
An error returned by `enter` if an execution scope has already been
entered.

<a href="struct.LocalPool.html" class="struct"
title="struct futures_executor::LocalPool">LocalPool</a>  
A single-threaded task pool for polling futures to completion.

<a href="struct.LocalSpawner.html" class="struct"
title="struct futures_executor::LocalSpawner">LocalSpawner</a>  
A handle to a
[`LocalPool`](struct.LocalPool.html "struct futures_executor::LocalPool")
that implements
[`Spawn`](../futures_task/spawn/trait.Spawn.html "trait futures_task::spawn::Spawn").

<a href="struct.ThreadPool.html" class="struct"
title="struct futures_executor::ThreadPool">ThreadPool</a>  
A general-purpose thread pool for scheduling tasks that poll futures to
completion.

<a href="struct.ThreadPoolBuilder.html" class="struct"
title="struct futures_executor::ThreadPoolBuilder">ThreadPoolBuilder</a>  
Thread pool configuration object.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.block_on.html" class="fn"
title="fn futures_executor::block_on">block_on</a>  
Run a future to completion on the current thread.

<a href="fn.block_on_stream.html" class="fn"
title="fn futures_executor::block_on_stream">block_on_stream</a>  
Turn a stream into a blocking iterator.

<a href="fn.enter.html" class="fn"
title="fn futures_executor::enter">enter</a>  
Marks the current thread as being within the dynamic extent of an
executor.
