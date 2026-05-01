Crate parking\_lot Copy item path

<span class="sub-heading"><a href="../src/parking_lot/lib.rs.html#8-62" class="src">Source</a>
</span>

Expand description

This library provides implementations of `Mutex`, `RwLock`, `Condvar`
and `Once` that are smaller, faster and more flexible than those in the
Rust standard library. It also provides a `ReentrantMutex` type.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use ::`<a href="../lock_api/index.html" class="mod"
title="mod lock_api"><code>lock_api</code></a>`;`

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Condvar.html" class="struct"
title="struct parking_lot::Condvar">Condvar</a>  
A Condition Variable

<a href="struct.Once.html" class="struct"
title="struct parking_lot::Once">Once</a>  
A synchronization primitive which can be used to run a one-time
initialization. Useful for one-time initialization for globals, FFI or
related functionality.

<a href="struct.RawFairMutex.html" class="struct"
title="struct parking_lot::RawFairMutex">RawFairMutex</a>  
Raw fair mutex type backed by the parking lot.

<a href="struct.RawMutex.html" class="struct"
title="struct parking_lot::RawMutex">RawMutex</a>  
Raw mutex type backed by the parking lot.

<a href="struct.RawRwLock.html" class="struct"
title="struct parking_lot::RawRwLock">RawRwLock</a>  
Raw reader-writer lock type backed by the parking lot.

<a href="struct.RawThreadId.html" class="struct"
title="struct parking_lot::RawThreadId">RawThreadId</a>  
Implementation of the `GetThreadId` trait for
`lock_api::ReentrantMutex`.

<a href="struct.WaitTimeoutResult.html" class="struct"
title="struct parking_lot::WaitTimeoutResult">WaitTimeoutResult</a>  
A type indicating whether a timed wait on a condition variable returned
due to a time out or not.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.OnceState.html" class="enum"
title="enum parking_lot::OnceState">OnceState</a>  
Current state of a `Once`.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.const_fair_mutex.html" class="fn"
title="fn parking_lot::const_fair_mutex">const_fair_mutex</a>  
Creates a new fair mutex in an unlocked state ready for use.

<a href="fn.const_mutex.html" class="fn"
title="fn parking_lot::const_mutex">const_mutex</a>  
Creates a new mutex in an unlocked state ready for use.

<a href="fn.const_reentrant_mutex.html" class="fn"
title="fn parking_lot::const_reentrant_mutex">const_reentrant_mutex</a>  
Creates a new reentrant mutex in an unlocked state ready for use.

<a href="fn.const_rwlock.html" class="fn"
title="fn parking_lot::const_rwlock">const_rwlock</a>  
Creates a new instance of an `RwLock<T>` which is unlocked.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.FairMutex.html" class="type"
title="type parking_lot::FairMutex">FairMutex</a>  
A mutual exclusive primitive that is always fair, useful for protecting
shared data

<a href="type.FairMutexGuard.html" class="type"
title="type parking_lot::FairMutexGuard">FairMutexGuard</a>  
An RAII implementation of a “scoped lock” of a mutex. When this
structure is dropped (falls out of scope), the lock will be unlocked.

<a href="type.MappedFairMutexGuard.html" class="type"
title="type parking_lot::MappedFairMutexGuard">MappedFairMutexGuard</a>  
An RAII mutex guard returned by `FairMutexGuard::map`, which can point
to a subfield of the protected data.

<a href="type.MappedMutexGuard.html" class="type"
title="type parking_lot::MappedMutexGuard">MappedMutexGuard</a>  
An RAII mutex guard returned by `MutexGuard::map`, which can point to a
subfield of the protected data.

<a href="type.MappedReentrantMutexGuard.html" class="type"
title="type parking_lot::MappedReentrantMutexGuard">MappedReentrantMutexGuard</a>  
An RAII mutex guard returned by `ReentrantMutexGuard::map`, which can
point to a subfield of the protected data.

<a href="type.MappedRwLockReadGuard.html" class="type"
title="type parking_lot::MappedRwLockReadGuard">MappedRwLockReadGuard</a>  
An RAII read lock guard returned by `RwLockReadGuard::map`, which can
point to a subfield of the protected data.

<a href="type.MappedRwLockWriteGuard.html" class="type"
title="type parking_lot::MappedRwLockWriteGuard">MappedRwLockWriteGuard</a>  
An RAII write lock guard returned by `RwLockWriteGuard::map`, which can
point to a subfield of the protected data.

<a href="type.Mutex.html" class="type"
title="type parking_lot::Mutex">Mutex</a>  
A mutual exclusion primitive useful for protecting shared data

<a href="type.MutexGuard.html" class="type"
title="type parking_lot::MutexGuard">MutexGuard</a>  
An RAII implementation of a “scoped lock” of a mutex. When this
structure is dropped (falls out of scope), the lock will be unlocked.

<a href="type.ReentrantMutex.html" class="type"
title="type parking_lot::ReentrantMutex">ReentrantMutex</a>  
A mutex which can be recursively locked by a single thread.

<a href="type.ReentrantMutexGuard.html" class="type"
title="type parking_lot::ReentrantMutexGuard">ReentrantMutexGuard</a>  
An RAII implementation of a “scoped lock” of a reentrant mutex. When
this structure is dropped (falls out of scope), the lock will be
unlocked.

<a href="type.RwLock.html" class="type"
title="type parking_lot::RwLock">RwLock</a>  
A reader-writer lock

<a href="type.RwLockReadGuard.html" class="type"
title="type parking_lot::RwLockReadGuard">RwLockReadGuard</a>  
RAII structure used to release the shared read access of a lock when
dropped.

<a href="type.RwLockUpgradableReadGuard.html" class="type"
title="type parking_lot::RwLockUpgradableReadGuard">RwLockUpgradableReadGuard</a>  
RAII structure used to release the upgradable read access of a lock when
dropped.

<a href="type.RwLockWriteGuard.html" class="type"
title="type parking_lot::RwLockWriteGuard">RwLockWriteGuard</a>  
RAII structure used to release the exclusive write access of a lock when
dropped.
