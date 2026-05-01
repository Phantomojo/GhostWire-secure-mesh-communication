Crate readlock\_tokio Copy item path

<span class="sub-heading"><a href="../src/readlock_tokio/lib.rs.html#1-352" class="src">Source</a>
</span>

Expand description

# <a href="#readlock-tokio" class="doc-anchor">§</a>readlock-tokio

A clone of the [`readlock`](https://crates.io/crates/readlock) crate
that uses tokio’s `RwLock` instead of the one from `std`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.OwnedSharedReadGuard.html" class="struct"
title="struct readlock_tokio::OwnedSharedReadGuard">OwnedSharedReadGuard</a>  
RAII structure used to release the shared read access of a lock when
dropped.

<a href="struct.Shared.html" class="struct"
title="struct readlock_tokio::Shared">Shared</a>  
A wrapper around a resource possibly shared with
[`SharedReadLock`](struct.SharedReadLock.html "struct readlock_tokio::SharedReadLock")s
and
[`WeakReadLock`](struct.WeakReadLock.html "struct readlock_tokio::WeakReadLock")s,
but no other `Shared`s.

<a href="struct.SharedReadGuard.html" class="struct"
title="struct readlock_tokio::SharedReadGuard">SharedReadGuard</a>  
RAII structure used to release the shared read access of a lock when
dropped.

<a href="struct.SharedReadLock.html" class="struct"
title="struct readlock_tokio::SharedReadLock">SharedReadLock</a>  
A read-only reference to a resource possibly shared with up to one
[`Shared`](struct.Shared.html "struct readlock_tokio::Shared") and many
[`WeakReadLock`](struct.WeakReadLock.html "struct readlock_tokio::WeakReadLock")s.

<a href="struct.SharedWriteGuard.html" class="struct"
title="struct readlock_tokio::SharedWriteGuard">SharedWriteGuard</a>  
RAII structure used to release the exclusive write access of a lock when
dropped.

<a href="struct.TryLockError.html" class="struct"
title="struct readlock_tokio::TryLockError">TryLockError</a>  
Error returned from
[`SharedReadLock::try_lock`](struct.SharedReadLock.html#method.try_lock "method readlock_tokio::SharedReadLock::try_lock").

<a href="struct.WeakReadLock.html" class="struct"
title="struct readlock_tokio::WeakReadLock">WeakReadLock</a>  
A weak read-only reference to a resource possibly shared with up to one
[`Shared`](struct.Shared.html "struct readlock_tokio::Shared"), and many
[`SharedReadLock`](struct.SharedReadLock.html "struct readlock_tokio::SharedReadLock")s.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.TryLockResult.html" class="type"
title="type readlock_tokio::TryLockResult">TryLockResult</a>  
A type alias for the result of a nonblocking locking method.
