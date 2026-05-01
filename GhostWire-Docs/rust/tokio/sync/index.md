[tokio](../index.html)

Module sync Copy item path

<span class="sub-heading"><a href="../../src/tokio/sync/mod.rs.html#1-524" class="src">Source</a>
</span>

Expand description

Synchronization primitives for use in asynchronous contexts.

Tokio programs tend to be organized as a set of
[tasks](../task/index.html "mod tokio::task") where each task operates
independently and may be executed on separate physical threads. The
synchronization primitives provided in this module permit these
independent tasks to communicate together.

# <a href="#message-passing" class="doc-anchor">§</a>Message passing

The most common form of synchronization in a Tokio program is message
passing. Two tasks operate independently and send messages to each other
to synchronize. Doing so has the advantage of avoiding shared state.

Message passing is implemented using channels. A channel supports
sending a message from one producer task to one or more consumer tasks.
There are a few flavors of channels provided by Tokio. Each channel
flavor supports different message passing patterns. When a channel
supports multiple producers, many separate tasks may **send** messages.
When a channel supports multiple consumers, many different separate
tasks may **receive** messages.

Tokio provides many different channel flavors as different message
passing patterns are best handled with different implementations.

## <a href="#oneshot-channel" class="doc-anchor">§</a>`oneshot` channel

The [`oneshot` channel](oneshot/index.html "mod tokio::sync::oneshot")
supports sending a **single** value from a single producer to a single
consumer. This channel is usually used to send the result of a
computation to a waiter.

**Example:** using a [`oneshot`
channel](oneshot/index.html "mod tokio::sync::oneshot") to receive the
result of a computation.

``` {.rust .rust-example-rendered}
use tokio::sync::oneshot;

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}

let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    let res = some_computation().await;
    tx.send(res).unwrap();
});

// Do other work while the computation is happening in the background

// Wait for the computation result
let res = rx.await.unwrap();
```

Note, if the task produces a computation result as its final action
before terminating, the
[`JoinHandle`](../task/struct.JoinHandle.html "struct tokio::task::JoinHandle")
can be used to receive that value instead of allocating resources for
the `oneshot` channel. Awaiting on
[`JoinHandle`](../task/struct.JoinHandle.html "struct tokio::task::JoinHandle")
returns `Result`. If the task panics, the `Joinhandle` yields `Err` with
the panic cause.

**Example:**

``` {.rust .rust-example-rendered}
async fn some_computation() -> String {
    "the result of the computation".to_string()
}

let join_handle = tokio::spawn(async move {
    some_computation().await
});

// Do other work while the computation is happening in the background

// Wait for the computation result
let res = join_handle.await.unwrap();
```

## <a href="#mpsc-channel" class="doc-anchor">§</a>`mpsc` channel

The [`mpsc` channel](mpsc/index.html "mod tokio::sync::mpsc") supports
sending **many** values from **many** producers to a single consumer.
This channel is often used to send work to a task or to receive the
result of many computations.

This is also the channel you should use if you want to send many
messages from a single producer to a single consumer. There is no
dedicated spsc channel.

**Example:** using an mpsc to incrementally stream the results of a
series of computations.

``` {.rust .rust-example-rendered}
use tokio::sync::mpsc;

async fn some_computation(input: u32) -> String {
    format!("the result of computation {}", input)
}

let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    for i in 0..10 {
        let res = some_computation(i).await;
        tx.send(res).await.unwrap();
    }
});

while let Some(res) = rx.recv().await {
    println!("got = {}", res);
}
```

The argument to `mpsc::channel` is the channel capacity. This is the
maximum number of values that can be stored in the channel pending
receipt at any given time. Properly setting this value is key in
implementing robust programs as the channel capacity plays a critical
part in handling back pressure.

A common concurrency pattern for resource management is to spawn a task
dedicated to managing that resource and using message passing between
other tasks to interact with the resource. The resource may be anything
that may not be concurrently used. Some examples include a socket and
program state. For example, if multiple tasks need to send data over a
single socket, spawn a task to manage the socket and use a channel to
synchronize.

**Example:** sending data from many tasks over a single socket using
message passing.

``` {.rust .rust-example-rendered}
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = TcpStream::connect("www.example.com:1234").await?;
    let (tx, mut rx) = mpsc::channel(100);

    for _ in 0..10 {
        // Each task needs its own `tx` handle. This is done by cloning the
        // original handle.
        let tx = tx.clone();

        tokio::spawn(async move {
            tx.send(&b"data to write"[..]).await.unwrap();
        });
    }

    // The `rx` half of the channel returns `None` once **all** `tx` clones
    // drop. To ensure `None` is returned, drop the handle owned by the
    // current task. If this `tx` handle is not dropped, there will always
    // be a single outstanding `tx` handle.
    drop(tx);

    while let Some(res) = rx.recv().await {
        socket.write_all(res).await?;
    }

    Ok(())
}
```

The [`mpsc`](mpsc/index.html "mod tokio::sync::mpsc") and
[`oneshot`](oneshot/index.html "mod tokio::sync::oneshot") channels can
be combined to provide a request / response type synchronization pattern
with a shared resource. A task is spawned to synchronize a resource and
waits on commands received on a
[`mpsc`](mpsc/index.html "mod tokio::sync::mpsc") channel. Each command
includes a [`oneshot`](oneshot/index.html "mod tokio::sync::oneshot")
`Sender` on which the result of the command is sent.

**Example:** use a task to synchronize a `u64` counter. Each task sends
an “fetch and increment” command. The counter value **before** the
increment is sent over the provided `oneshot` channel.

``` {.rust .rust-example-rendered}
use tokio::sync::{oneshot, mpsc};
use Command::Increment;

enum Command {
    Increment,
    // Other commands can be added here
}

let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<u64>)>(100);

// Spawn a task to manage the counter
tokio::spawn(async move {
    let mut counter: u64 = 0;

    while let Some((cmd, response)) = cmd_rx.recv().await {
        match cmd {
            Increment => {
                let prev = counter;
                counter += 1;
                response.send(prev).unwrap();
            }
        }
    }
});

let mut join_handles = vec![];

// Spawn tasks that will send the increment command.
for _ in 0..10 {
    let cmd_tx = cmd_tx.clone();

    join_handles.push(tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        cmd_tx.send((Increment, resp_tx)).await.ok().unwrap();
        let res = resp_rx.await.unwrap();

        println!("previous value = {}", res);
    }));
}

// Wait for all tasks to complete
for join_handle in join_handles.drain(..) {
    join_handle.await.unwrap();
}
```

## <a href="#broadcast-channel" class="doc-anchor">§</a>`broadcast` channel

The [`broadcast`
channel](broadcast/index.html "mod tokio::sync::broadcast") supports
sending **many** values from **many** producers to **many** consumers.
Each consumer will receive **each** value. This channel can be used to
implement “fan out” style patterns common with pub / sub or “chat”
systems.

This channel tends to be used less often than `oneshot` and `mpsc` but
still has its use cases.

This is also the channel you should use if you want to broadcast values
from a single producer to many consumers. There is no dedicated spmc
broadcast channel.

Basic usage

``` {.rust .rust-example-rendered}
use tokio::sync::broadcast;

let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

tokio::spawn(async move {
    assert_eq!(rx1.recv().await.unwrap(), 10);
    assert_eq!(rx1.recv().await.unwrap(), 20);
});

tokio::spawn(async move {
    assert_eq!(rx2.recv().await.unwrap(), 10);
    assert_eq!(rx2.recv().await.unwrap(), 20);
});

tx.send(10).unwrap();
tx.send(20).unwrap();
```

## <a href="#watch-channel" class="doc-anchor">§</a>`watch` channel

The [`watch` channel](watch/index.html "mod tokio::sync::watch")
supports sending **many** values from **many** producers to **many**
consumers. However, only the **most recent** value is stored in the
channel. Consumers are notified when a new value is sent, but there is
no guarantee that consumers will see **all** values.

The [`watch` channel](watch/index.html "mod tokio::sync::watch") is
similar to a [`broadcast`
channel](broadcast/index.html "mod tokio::sync::broadcast") with
capacity 1.

Use cases for the [`watch`
channel](watch/index.html "mod tokio::sync::watch") include broadcasting
configuration changes or signalling program state changes, such as
transitioning to shutdown.

**Example:** use a [`watch`
channel](watch/index.html "mod tokio::sync::watch") to notify tasks of
configuration changes. In this example, a configuration file is checked
periodically. When the file changes, the configuration changes are
signalled to consumers.

``` {.rust .rust-example-rendered}
use tokio::sync::watch;
use tokio::time::{self, Duration, Instant};

use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Config {
    timeout: Duration,
}

impl Config {
    async fn load_from_file() -> io::Result<Config> {
        // file loading and deserialization logic here
    }
}

async fn my_async_operation() {
    // Do something here
}

// Load initial configuration value
let mut config = Config::load_from_file().await.unwrap();

// Create the watch channel, initialized with the loaded configuration
let (tx, rx) = watch::channel(config.clone());

// Spawn a task to monitor the file.
tokio::spawn(async move {
    loop {
        // Wait 10 seconds between checks
        time::sleep(Duration::from_secs(10)).await;

        // Load the configuration file
        let new_config = Config::load_from_file().await.unwrap();

        // If the configuration changed, send the new config value
        // on the watch channel.
        if new_config != config {
            tx.send(new_config.clone()).unwrap();
            config = new_config;
        }
    }
});

let mut handles = vec![];

// Spawn tasks that runs the async operation for at most `timeout`. If
// the timeout elapses, restart the operation.
//
// The task simultaneously watches the `Config` for changes. When the
// timeout duration changes, the timeout is updated without restarting
// the in-flight operation.
for _ in 0..5 {
    // Clone a config watch handle for use in this task
    let mut rx = rx.clone();

    let handle = tokio::spawn(async move {
        // Start the initial operation and pin the future to the stack.
        // Pinning to the stack is required to resume the operation
        // across multiple calls to `select!`
        let op = my_async_operation();
        tokio::pin!(op);

        // Get the initial config value
        let mut conf = rx.borrow().clone();

        let mut op_start = Instant::now();
        let sleep = time::sleep_until(op_start + conf.timeout);
        tokio::pin!(sleep);

        loop {
            tokio::select! {
                _ = &mut sleep => {
                    // The operation elapsed. Restart it
                    op.set(my_async_operation());

                    // Track the new start time
                    op_start = Instant::now();

                    // Restart the timeout
                    sleep.set(time::sleep_until(op_start + conf.timeout));
                }
                _ = rx.changed() => {
                    conf = rx.borrow_and_update().clone();

                    // The configuration has been updated. Update the
                    // `sleep` using the new `timeout` value.
                    sleep.as_mut().reset(op_start + conf.timeout);
                }
                _ = &mut op => {
                    // The operation completed!
                    return
                }
            }
        }
    });

    handles.push(handle);
}

for handle in handles.drain(..) {
    handle.await.unwrap();
}
```

# <a href="#state-synchronization" class="doc-anchor">§</a>State synchronization

The remaining synchronization primitives focus on synchronizing state.
These are asynchronous equivalents to versions provided by `std`. They
operate in a similar way as their `std` counterparts but will wait
asynchronously instead of blocking the thread.

-   [`Barrier`](struct.Barrier.html "struct tokio::sync::Barrier")
    Ensures multiple tasks will wait for each other to reach a point in
    the program, before continuing execution all together.

-   [`Mutex`](struct.Mutex.html "struct tokio::sync::Mutex") Mutual
    Exclusion mechanism, which ensures that at most one thread at a time
    is able to access some data.

-   [`Notify`](struct.Notify.html "struct tokio::sync::Notify") Basic
    task notification. `Notify` supports notifying a receiving task
    without sending data. In this case, the task wakes up and resumes
    processing.

-   [`RwLock`](struct.RwLock.html "struct tokio::sync::RwLock") Provides
    a mutual exclusion mechanism which allows multiple readers at the
    same time, while allowing only one writer at a time. In some cases,
    this can be more efficient than a mutex.

-   [`Semaphore`](struct.Semaphore.html "struct tokio::sync::Semaphore")
    Limits the amount of concurrency. A semaphore holds a number of
    permits, which tasks may request in order to enter a critical
    section. Semaphores are useful for implementing limiting or bounding
    of any kind.

# <a href="#runtime-compatibility" class="doc-anchor">§</a>Runtime compatibility

All synchronization primitives provided in this module are runtime
agnostic. You can freely move them between different instances of the
Tokio runtime or even use them from non-Tokio runtimes.

When used in a Tokio runtime, the synchronization primitives participate
in [cooperative
scheduling](../task/coop/index.html#cooperative-scheduling "mod tokio::task::coop")
to avoid starvation. This feature does not apply when used from
non-Tokio runtimes.

As an exception, methods ending in `_timeout` are not runtime agnostic
because they require access to the Tokio timer. See the documentation of
each `*_timeout` method for more information on its use.

# Modules<a href="#modules" class="anchor">§</a>

<a href="broadcast/index.html" class="mod"
title="mod tokio::sync::broadcast">broadcast</a>  
A multi-producer, multi-consumer broadcast queue. Each sent value is
seen by all consumers.

<a href="futures/index.html" class="mod"
title="mod tokio::sync::futures">futures</a>  
Named future types.

<a href="mpsc/index.html" class="mod"
title="mod tokio::sync::mpsc">mpsc</a>  
A multi-producer, single-consumer queue for sending values between
asynchronous tasks.

<a href="oneshot/index.html" class="mod"
title="mod tokio::sync::oneshot">oneshot</a>  
A one-shot channel is used for sending a single message between
asynchronous tasks. The
[`channel`](oneshot/fn.channel.html "fn tokio::sync::oneshot::channel")
function is used to create a
[`Sender`](oneshot/struct.Sender.html "struct tokio::sync::oneshot::Sender")
and
[`Receiver`](oneshot/struct.Receiver.html "struct tokio::sync::oneshot::Receiver")
handle pair that form the channel.

<a href="watch/index.html" class="mod"
title="mod tokio::sync::watch">watch</a>  
A multi-producer, multi-consumer channel that only retains the *last*
sent value.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AcquireError.html" class="struct"
title="struct tokio::sync::AcquireError">AcquireError</a>  
Error returned from the
[`Semaphore::acquire`](struct.Semaphore.html#method.acquire "method tokio::sync::Semaphore::acquire")
function.

<a href="struct.Barrier.html" class="struct"
title="struct tokio::sync::Barrier">Barrier</a>  
A barrier enables multiple tasks to synchronize the beginning of some
computation.

<a href="struct.BarrierWaitResult.html" class="struct"
title="struct tokio::sync::BarrierWaitResult">BarrierWaitResult</a>  
A `BarrierWaitResult` is returned by `wait` when all tasks in the
`Barrier` have rendezvoused.

<a href="struct.MappedMutexGuard.html" class="struct"
title="struct tokio::sync::MappedMutexGuard">MappedMutexGuard</a>  
A handle to a held `Mutex` that has had a function applied to it via
[`MutexGuard::map`](struct.MutexGuard.html#method.map "associated function tokio::sync::MutexGuard::map").

<a href="struct.Mutex.html" class="struct"
title="struct tokio::sync::Mutex">Mutex</a>  
An asynchronous `Mutex`-like type.

<a href="struct.MutexGuard.html" class="struct"
title="struct tokio::sync::MutexGuard">MutexGuard</a>  
A handle to a held `Mutex`. The guard can be held across any `.await`
point as it is
[`Send`](https://doc.rust-lang.org/1.93.1/core/marker/trait.Send.html "trait core::marker::Send").

<a href="struct.Notify.html" class="struct"
title="struct tokio::sync::Notify">Notify</a>  
Notifies a single task to wake up.

<a href="struct.OnceCell.html" class="struct"
title="struct tokio::sync::OnceCell">OnceCell</a>  
A thread-safe cell that can be written to only once.

<a href="struct.OwnedMappedMutexGuard.html" class="struct"
title="struct tokio::sync::OwnedMappedMutexGuard">OwnedMappedMutexGuard</a>  
A owned handle to a held `Mutex` that has had a function applied to it
via
[`OwnedMutexGuard::map`](struct.OwnedMutexGuard.html#method.map "associated function tokio::sync::OwnedMutexGuard::map").

<a href="struct.OwnedMutexGuard.html" class="struct"
title="struct tokio::sync::OwnedMutexGuard">OwnedMutexGuard</a>  
An owned handle to a held `Mutex`.

<a href="struct.OwnedRwLockMappedWriteGuard.html" class="struct"
title="struct tokio::sync::OwnedRwLockMappedWriteGuard">OwnedRwLockMappedWriteGuard</a>  
Owned RAII structure used to release the exclusive write access of a
lock when dropped.

<a href="struct.OwnedRwLockReadGuard.html" class="struct"
title="struct tokio::sync::OwnedRwLockReadGuard">OwnedRwLockReadGuard</a>  
Owned RAII structure used to release the shared read access of a lock
when dropped.

<a href="struct.OwnedRwLockWriteGuard.html" class="struct"
title="struct tokio::sync::OwnedRwLockWriteGuard">OwnedRwLockWriteGuard</a>  
Owned RAII structure used to release the exclusive write access of a
lock when dropped.

<a href="struct.OwnedSemaphorePermit.html" class="struct"
title="struct tokio::sync::OwnedSemaphorePermit">OwnedSemaphorePermit</a>  
An owned permit from the semaphore.

<a href="struct.RwLock.html" class="struct"
title="struct tokio::sync::RwLock">RwLock</a>  
An asynchronous reader-writer lock.

<a href="struct.RwLockMappedWriteGuard.html" class="struct"
title="struct tokio::sync::RwLockMappedWriteGuard">RwLockMappedWriteGuard</a>  
RAII structure used to release the exclusive write access of a lock when
dropped.

<a href="struct.RwLockReadGuard.html" class="struct"
title="struct tokio::sync::RwLockReadGuard">RwLockReadGuard</a>  
RAII structure used to release the shared read access of a lock when
dropped.

<a href="struct.RwLockWriteGuard.html" class="struct"
title="struct tokio::sync::RwLockWriteGuard">RwLockWriteGuard</a>  
RAII structure used to release the exclusive write access of a lock when
dropped.

<a href="struct.Semaphore.html" class="struct"
title="struct tokio::sync::Semaphore">Semaphore</a>  
Counting semaphore performing asynchronous permit acquisition.

<a href="struct.SemaphorePermit.html" class="struct"
title="struct tokio::sync::SemaphorePermit">SemaphorePermit</a>  
A permit from the semaphore.

<a href="struct.SetOnce.html" class="struct"
title="struct tokio::sync::SetOnce">SetOnce</a>  
A thread-safe cell that can be written to only once.

<a href="struct.SetOnceError.html" class="struct"
title="struct tokio::sync::SetOnceError">SetOnceError</a>  
Error that can be returned from
[`SetOnce::set`](struct.SetOnce.html#method.set "method tokio::sync::SetOnce::set").

<a href="struct.TryLockError.html" class="struct"
title="struct tokio::sync::TryLockError">TryLockError</a>  
Error returned from the
[`Mutex::try_lock`](struct.Mutex.html#method.try_lock "method tokio::sync::Mutex::try_lock"),
[`RwLock::try_read`](struct.RwLock.html#method.try_read "method tokio::sync::RwLock::try_read")
and
[`RwLock::try_write`](struct.RwLock.html#method.try_write "method tokio::sync::RwLock::try_write")
functions.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.SetError.html" class="enum"
title="enum tokio::sync::SetError">SetError</a>  
Errors that can be returned from
[`OnceCell::set`](struct.OnceCell.html#method.set "method tokio::sync::OnceCell::set").

<a href="enum.TryAcquireError.html" class="enum"
title="enum tokio::sync::TryAcquireError">TryAcquireError</a>  
Error returned from the
[`Semaphore::try_acquire`](struct.Semaphore.html#method.try_acquire "method tokio::sync::Semaphore::try_acquire")
function.
