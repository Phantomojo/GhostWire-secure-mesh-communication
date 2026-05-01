Crate crossbeam\_epoch Copy item path

<span class="sub-heading"><a href="../src/crossbeam_epoch/lib.rs.html#1-166"
class="src">Source</a> </span>

Expand description

Epoch-based memory reclamation.

An interesting problem concurrent collections deal with comes from the
remove operation. Suppose that a thread removes an element from a
lock-free map, while another thread is reading that same element at the
same time. The first thread must wait until the second thread stops
reading the element. Only then it is safe to destruct it.

Programming languages that come with garbage collectors solve this
problem trivially. The garbage collector will destruct the removed
element when no thread can hold a reference to it anymore.

This crate implements a basic memory reclamation mechanism, which is
based on epochs. When an element gets removed from a concurrent
collection, it is inserted into a pile of garbage and marked with the
current epoch. Every time a thread accesses a collection, it checks the
current epoch, attempts to increment it, and destructs some garbage that
became so old that no thread can be referencing it anymore.

That is the general mechanism behind epoch-based memory reclamation, but
the details are a bit more complicated. Anyhow, memory reclamation is
designed to be fully automatic and something users of concurrent
collections don’t have to worry much about.

# <a href="#pointers" class="doc-anchor">§</a>Pointers

Concurrent collections are built using atomic pointers. This module
provides
[`Atomic`](struct.Atomic.html "struct crossbeam_epoch::Atomic"), which
is just a shared atomic pointer to a heap-allocated object. Loading an
[`Atomic`](struct.Atomic.html "struct crossbeam_epoch::Atomic") yields a
[`Shared`](struct.Shared.html "struct crossbeam_epoch::Shared"), which
is an epoch-protected pointer through which the loaded object can be
safely read.

# <a href="#pinning" class="doc-anchor">§</a>Pinning

Before an
[`Atomic`](struct.Atomic.html "struct crossbeam_epoch::Atomic") can be
loaded, a participant must be
[`pin`](fn.pin.html "fn crossbeam_epoch::pin")ned. By pinning a
participant we declare that any object that gets removed from now on
must not be destructed just yet. Garbage collection of newly removed
objects is suspended until the participant gets unpinned.

# <a href="#garbage" class="doc-anchor">§</a>Garbage

Objects that get removed from concurrent collections must be stashed
away until all currently pinned participants get unpinned. Such objects
can be stored into a thread-local or global storage, where they are kept
until the right time for their destruction comes.

There is a global shared instance of garbage queue. You can
[`defer`](struct.Guard.html#method.defer "method crossbeam_epoch::Guard::defer")
the execution of an arbitrary function until the global epoch is
advanced enough. Most notably, concurrent data structures may defer the
deallocation of an object.

# <a href="#apis" class="doc-anchor">§</a>APIs

For majority of use cases, just use the default garbage collector by
invoking [`pin`](fn.pin.html "fn crossbeam_epoch::pin"). If you want to
create your own garbage collector, use the
[`Collector`](struct.Collector.html "struct crossbeam_epoch::Collector")
API.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Atomic.html" class="struct"
title="struct crossbeam_epoch::Atomic">Atomic</a>  
An atomic pointer that can be safely shared between threads.

<a href="struct.Collector.html" class="struct"
title="struct crossbeam_epoch::Collector">Collector</a>  
An epoch-based garbage collector.

<a href="struct.CompareExchangeError.html" class="struct"
title="struct crossbeam_epoch::CompareExchangeError">CompareExchangeError</a>  
The error returned on failed compare-and-swap operation.

<a href="struct.Guard.html" class="struct"
title="struct crossbeam_epoch::Guard">Guard</a>  
A guard that keeps the current thread pinned.

<a href="struct.LocalHandle.html" class="struct"
title="struct crossbeam_epoch::LocalHandle">LocalHandle</a>  
A handle to a garbage collector.

<a href="struct.Owned.html" class="struct"
title="struct crossbeam_epoch::Owned">Owned</a>  
An owned heap-allocated object.

<a href="struct.Shared.html" class="struct"
title="struct crossbeam_epoch::Shared">Shared</a>  
A pointer to an object protected by the epoch GC.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.CompareAndSetOrdering.html" class="trait"
title="trait crossbeam_epoch::CompareAndSetOrdering">CompareAndSetOrdering</a><span class="stab deprecated" title="">Deprecated</span>  
Memory orderings for compare-and-set operations.

<a href="trait.Pointable.html" class="trait"
title="trait crossbeam_epoch::Pointable">Pointable</a>  
Types that are pointed to by a single word.

<a href="trait.Pointer.html" class="trait"
title="trait crossbeam_epoch::Pointer">Pointer</a>  
A trait for either `Owned` or `Shared` pointers.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.default_collector.html" class="fn"
title="fn crossbeam_epoch::default_collector">default_collector</a>  
Returns the default global collector.

<a href="fn.is_pinned.html" class="fn"
title="fn crossbeam_epoch::is_pinned">is_pinned</a>  
Returns `true` if the current thread is pinned.

<a href="fn.pin.html" class="fn" title="fn crossbeam_epoch::pin">pin</a>  
Pins the current thread.

<a href="fn.unprotected.html" class="fn"
title="fn crossbeam_epoch::unprotected">unprotected</a><sup>⚠</sup>  
Returns a reference to a dummy guard that allows unprotected access to
[`Atomic`](struct.Atomic.html "struct crossbeam_epoch::Atomic")s.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.CompareAndSetError.html" class="type"
title="type crossbeam_epoch::CompareAndSetError">CompareAndSetError</a><span class="stab deprecated" title="">Deprecated</span>  
The error returned on failed compare-and-set operation.
