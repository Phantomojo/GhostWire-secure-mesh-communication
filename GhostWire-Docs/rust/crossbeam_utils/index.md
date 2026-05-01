Crate crossbeam\_utils Copy item path

<span class="sub-heading"><a href="../src/crossbeam_utils/lib.rs.html#1-110"
class="src">Source</a> </span>

Expand description

Miscellaneous tools for concurrent programming.

## <a href="#atomics" class="doc-anchor">§</a>Atomics

-   [`AtomicCell`](atomic/struct.AtomicCell.html "struct crossbeam_utils::atomic::AtomicCell"),
    a thread-safe mutable memory location.
-   [`AtomicConsume`](atomic/trait.AtomicConsume.html "trait crossbeam_utils::atomic::AtomicConsume"),
    for reading from primitive atomic types with “consume” ordering.

## <a href="#thread-synchronization" class="doc-anchor">§</a>Thread synchronization

-   [`Parker`](sync/struct.Parker.html "struct crossbeam_utils::sync::Parker"),
    a thread parking primitive.
-   [`ShardedLock`](sync/struct.ShardedLock.html "struct crossbeam_utils::sync::ShardedLock"),
    a sharded reader-writer lock with fast concurrent reads.
-   [`WaitGroup`](sync/struct.WaitGroup.html "struct crossbeam_utils::sync::WaitGroup"),
    for synchronizing the beginning or end of some computation.

## <a href="#utilities" class="doc-anchor">§</a>Utilities

-   [`Backoff`](struct.Backoff.html "struct crossbeam_utils::Backoff"),
    for exponential backoff in spin loops.
-   [`CachePadded`](struct.CachePadded.html "struct crossbeam_utils::CachePadded"),
    for padding and aligning a value to the length of a cache line.
-   [`scope`](thread/fn.scope.html "fn crossbeam_utils::thread::scope"),
    for spawning threads that borrow local variables from the stack.

# Modules<a href="#modules" class="anchor">§</a>

<a href="atomic/index.html" class="mod"
title="mod crossbeam_utils::atomic">atomic</a>  
Atomic types.

<a href="sync/index.html" class="mod"
title="mod crossbeam_utils::sync">sync</a>  
Thread synchronization primitives.

<a href="thread/index.html" class="mod"
title="mod crossbeam_utils::thread">thread</a>  
Threads that can borrow variables from the stack.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Backoff.html" class="struct"
title="struct crossbeam_utils::Backoff">Backoff</a>  
Performs exponential backoff in spin loops.

<a href="struct.CachePadded.html" class="struct"
title="struct crossbeam_utils::CachePadded">CachePadded</a>  
Pads and aligns a value to the length of a cache line.
