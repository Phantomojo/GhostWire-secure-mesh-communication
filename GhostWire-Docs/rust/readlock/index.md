Crate readlock Copy item path

<span class="sub-heading"><a href="../src/readlock/lib.rs.html#1-326" class="src">Source</a>
</span>

Expand description

# <a href="#readlock" class="doc-anchor">§</a>readlock

(Shared) Read-Only Lock: A thing that can be useful when you don’t
really want shared mutability, you just want to mutate a value from one
place and read it from many others.

This library provides three types:

-   `Shared<T>`: similar to `Arc<RwLock<T>>`, but you can only create
    `SharedReadLock<T>`s and `WeakReadLock<T>`s from it that share
    access to the same inner value, not further `Shared<T>`s. Also,
    acquiring a write lock requires unique ownership / borrowing
    (`&mut self`). However: Reading requires *no* locking because
    mutably borrowing the `Shared` means that no other thread can be
    mutating the value at the same time (all other reference to the
    value are read-only).
-   `SharedReadLock<T>`: like a `Arc<RwLock<T>>` that is only ever used
    for reading. Can be downgraded to `WeakReadLock`.
-   `WeakReadLock<T>`: like a `Weak<RwLock<T>>`. That is, it references
    the same memory, but if the original `Shared` and any derived
    `SharedReadLock`s to that value are dropped, it will be deallocated
    regardless of any `WeakReadLock`s. Must be upgraded into
    `SharedReadLock` to access the inner value.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Shared.html" class="struct"
title="struct readlock::Shared">Shared</a>  
A wrapper around a resource possibly shared with
[`SharedReadLock`](struct.SharedReadLock.html "struct readlock::SharedReadLock")s
and
[`WeakReadLock`](struct.WeakReadLock.html "struct readlock::WeakReadLock")s,
but no other `Shared`s.

<a href="struct.SharedReadGuard.html" class="struct"
title="struct readlock::SharedReadGuard">SharedReadGuard</a>  
RAII structure used to release the shared read access of a lock when
dropped.

<a href="struct.SharedReadLock.html" class="struct"
title="struct readlock::SharedReadLock">SharedReadLock</a>  
A read-only reference to a resource possibly shared with up to one
[`Shared`](struct.Shared.html "struct readlock::Shared") and many
[`WeakReadLock`](struct.WeakReadLock.html "struct readlock::WeakReadLock")s.

<a href="struct.SharedWriteGuard.html" class="struct"
title="struct readlock::SharedWriteGuard">SharedWriteGuard</a>  
RAII structure used to release the exclusive write access of a lock when
dropped.

<a href="struct.WeakReadLock.html" class="struct"
title="struct readlock::WeakReadLock">WeakReadLock</a>  
A weak read-only reference to a resource possibly shared with up to one
[`Shared`](struct.Shared.html "struct readlock::Shared") and many
[`SharedReadLock`](struct.SharedReadLock.html "struct readlock::SharedReadLock")s.
