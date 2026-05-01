Crate concurrent\_queue Copy item path

<span class="sub-heading"><a href="../src/concurrent_queue/lib.rs.html#1-660"
class="src">Source</a> </span>

Expand description

A concurrent multi-producer multi-consumer queue.

There are two kinds of queues:

1.  [Bounded](struct.ConcurrentQueue.html#method.bounded "associated function concurrent_queue::ConcurrentQueue::bounded")
    queue with limited capacity.
2.  [Unbounded](struct.ConcurrentQueue.html#method.unbounded "associated function concurrent_queue::ConcurrentQueue::unbounded")
    queue with unlimited capacity.

Queues also have the capability to get
[closed](struct.ConcurrentQueue.html#method.close "method concurrent_queue::ConcurrentQueue::close")
at any point. When closed, no more items can be pushed into the queue,
although the remaining items can still be popped.

These features make it easy to build channels similar to
[`std::sync::mpsc`](https://doc.rust-lang.org/1.93.1/std/sync/mpsc/index.html "mod std::sync::mpsc")
on top of this crate.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use concurrent_queue::ConcurrentQueue;

let q = ConcurrentQueue::unbounded();
q.push(1).unwrap();
q.push(2).unwrap();

assert_eq!(q.pop(), Ok(1));
assert_eq!(q.pop(), Ok(2));
```

# <a href="#features" class="doc-anchor">§</a>Features

`concurrent-queue` uses an `std` default feature. With this feature
enabled, this crate will use
[`std::thread::yield_now`](https://doc.rust-lang.org/1.93.1/std/thread/functions/fn.yield_now.html "fn std::thread::functions::yield_now")
to avoid busy waiting in tight loops. However, with this feature
disabled,
[`core::hint::spin_loop`](https://doc.rust-lang.org/1.93.1/core/hint/fn.spin_loop.html "fn core::hint::spin_loop")
will be used instead. Disabling `std` will allow this crate to be used
on `no_std` platforms at the potential expense of more busy waiting.

There is also a `portable-atomic` feature, which uses a polyfill from
the [`portable-atomic`](https://crates.io/crates/portable-atomic) crate
to provide atomic operations on platforms that do not support them. See
the
[`README`](https://github.com/taiki-e/portable-atomic/blob/main/README.md#optional-cfg)
for the [`portable-atomic`](https://crates.io/crates/portable-atomic)
crate for more information on how to use it. Note that even with this
feature enabled, `concurrent-queue` still requires a global allocator to
be available. See the documentation for the
[`std::alloc::GlobalAlloc`](https://doc.rust-lang.org/1.93.1/core/alloc/global/trait.GlobalAlloc.html "trait core::alloc::global::GlobalAlloc")
trait for more information.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ConcurrentQueue.html" class="struct"
title="struct concurrent_queue::ConcurrentQueue">ConcurrentQueue</a>  
A concurrent queue.

<a href="struct.ForcePushError.html" class="struct"
title="struct concurrent_queue::ForcePushError">ForcePushError</a>  
Error that occurs when force-pushing into a full queue.

<a href="struct.TryIter.html" class="struct"
title="struct concurrent_queue::TryIter">TryIter</a>  
An iterator that pops items from a
[`ConcurrentQueue`](struct.ConcurrentQueue.html "struct concurrent_queue::ConcurrentQueue").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.PopError.html" class="enum"
title="enum concurrent_queue::PopError">PopError</a>  
Error which occurs when popping from an empty queue.

<a href="enum.PushError.html" class="enum"
title="enum concurrent_queue::PushError">PushError</a>  
Error which occurs when pushing into a full or closed queue.
