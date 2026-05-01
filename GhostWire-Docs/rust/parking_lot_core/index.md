Crate parking\_lot\_core Copy item path

<span class="sub-heading"><a href="../src/parking_lot_core/lib.rs.html#8-67"
class="src">Source</a> </span>

Expand description

This library exposes a low-level API for creating your own efficient
synchronization primitives.

# <a href="#the-parking-lot" class="doc-anchor">§</a>The parking lot

To keep synchronization primitives small, all thread queuing and
suspending functionality is offloaded to the *parking lot*. The idea
behind this is based on the Webkit
[`WTF::ParkingLot`](https://webkit.org/blog/6161/locking-in-webkit/)
class, which essentially consists of a hash table mapping of lock
addresses to queues of parked (sleeping) threads. The Webkit parking lot
was itself inspired by Linux
[futexes](http://man7.org/linux/man-pages/man2/futex.2.html), but it is
more powerful since it allows invoking callbacks while holding a queue
lock.

There are two main operations that can be performed on the parking lot:

-   *Parking* refers to suspending the thread while simultaneously
    enqueuing it on a queue keyed by some address.
-   *Unparking* refers to dequeuing a thread from a queue keyed by some
    address and resuming it.

See the documentation of the individual functions for more details.

# <a href="#building-custom-synchronization-primitives"
class="doc-anchor">§</a>Building custom synchronization primitives

Building custom synchronization primitives is very simple since the
parking lot takes care of all the hard parts for you. A simple example
for a custom primitive would be to integrate a `Mutex` inside another
data type. Since a mutex only requires 2 bits, it can share space with
other data. For example, one could create an `ArcMutex` type that
combines the atomic reference count and the two mutex bits in the same
atomic word.

# Modules<a href="#modules" class="anchor">§</a>

<a href="deadlock/index.html" class="mod"
title="mod parking_lot_core::deadlock">deadlock</a>  
\[Experimental\] Deadlock detection

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ParkToken.html" class="struct"
title="struct parking_lot_core::ParkToken">ParkToken</a>  
A value associated with a parked thread which can be used by
`unpark_filter`.

<a href="struct.SpinWait.html" class="struct"
title="struct parking_lot_core::SpinWait">SpinWait</a>  
A counter used to perform exponential backoff in spin loops.

<a href="struct.UnparkResult.html" class="struct"
title="struct parking_lot_core::UnparkResult">UnparkResult</a>  
Result of an unpark operation.

<a href="struct.UnparkToken.html" class="struct"
title="struct parking_lot_core::UnparkToken">UnparkToken</a>  
A value which is passed from an unparker to a parked thread.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.FilterOp.html" class="enum"
title="enum parking_lot_core::FilterOp">FilterOp</a>  
Operation that `unpark_filter` should perform for each thread.

<a href="enum.ParkResult.html" class="enum"
title="enum parking_lot_core::ParkResult">ParkResult</a>  
Result of a park operation.

<a href="enum.RequeueOp.html" class="enum"
title="enum parking_lot_core::RequeueOp">RequeueOp</a>  
Operation that `unpark_requeue` should perform.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.DEFAULT_PARK_TOKEN.html" class="constant"
title="constant parking_lot_core::DEFAULT_PARK_TOKEN">DEFAULT_PARK_TOKEN</a>  
A default park token to use.

<a href="constant.DEFAULT_UNPARK_TOKEN.html" class="constant"
title="constant parking_lot_core::DEFAULT_UNPARK_TOKEN">DEFAULT_UNPARK_TOKEN</a>  
A default unpark token to use.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.park.html" class="fn"
title="fn parking_lot_core::park">park</a><sup>⚠</sup>  
Parks the current thread in the queue associated with the given key.

<a href="fn.unpark_all.html" class="fn"
title="fn parking_lot_core::unpark_all">unpark_all</a><sup>⚠</sup>  
Unparks all threads in the queue associated with the given key.

<a href="fn.unpark_filter.html" class="fn"
title="fn parking_lot_core::unpark_filter">unpark_filter</a><sup>⚠</sup>  
Unparks a number of threads from the front of the queue associated with
`key` depending on the results of a filter function which inspects the
`ParkToken` associated with each thread.

<a href="fn.unpark_one.html" class="fn"
title="fn parking_lot_core::unpark_one">unpark_one</a><sup>⚠</sup>  
Unparks one thread from the queue associated with the given key.

<a href="fn.unpark_requeue.html" class="fn"
title="fn parking_lot_core::unpark_requeue">unpark_requeue</a><sup>⚠</sup>  
Removes all threads from the queue associated with `key_from`,
optionally unparks the first one and requeues the rest onto the queue
associated with `key_to`.
