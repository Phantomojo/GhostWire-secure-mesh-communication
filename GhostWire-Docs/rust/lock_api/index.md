Crate lock\_api Copy item path

<span class="sub-heading"><a href="../src/lock_api/lib.rs.html#8-117" class="src">Source</a>
</span>

Expand description

This library provides type-safe and fully-featured
[`Mutex`](struct.Mutex.html "struct lock_api::Mutex") and
[`RwLock`](struct.RwLock.html "struct lock_api::RwLock") types which
wrap a simple raw mutex or rwlock type. This has several benefits: not
only does it eliminate a large portion of the work in implementing
custom lock types, it also allows users to write code which is generic
with regards to different lock implementations.

Basic usage of this crate is very straightforward:

1.  Create a raw lock type. This should only contain the lock state, not
    any data protected by the lock.
2.  Implement the `RawMutex` trait for your custom lock type.
3.  Export your mutex as a type alias for `lock_api::Mutex`, and your
    mutex guard as a type alias for `lock_api::MutexGuard`. See the
    [example](#example) below for details.

This process is similar for
[`RwLock`](struct.RwLock.html "struct lock_api::RwLock")s, except that
two guards need to be exported instead of one. (Or 3 guards if your type
supports upgradable read locks, see [extension
traits](#extension-traits) below for details)

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use lock_api::{RawMutex, Mutex, GuardSend};
use std::sync::atomic::{AtomicBool, Ordering};

// 1. Define our raw lock type
pub struct RawSpinlock(AtomicBool);

// 2. Implement RawMutex for this type
unsafe impl RawMutex for RawSpinlock {
    const INIT: RawSpinlock = RawSpinlock(AtomicBool::new(false));

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = GuardSend;

    fn lock(&self) {
        // Note: This isn't the best way of implementing a spinlock, but it
        // suffices for the sake of this example.
        while !self.try_lock() {}
    }

    fn try_lock(&self) -> bool {
        self.0
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    unsafe fn unlock(&self) {
        self.0.store(false, Ordering::Release);
    }
}

// 3. Export the wrappers. This are the types that your users will actually use.
pub type Spinlock<T> = lock_api::Mutex<RawSpinlock, T>;
pub type SpinlockGuard<'a, T> = lock_api::MutexGuard<'a, RawSpinlock, T>;
```

# <a href="#extension-traits" class="doc-anchor">§</a>Extension traits

In addition to basic locking & unlocking functionality, you have the
option of exposing additional functionality in your lock types by
implementing additional traits for it. Examples of extension features
include:

-   Fair unlocking (`RawMutexFair`, `RawRwLockFair`)
-   Lock timeouts (`RawMutexTimed`, `RawRwLockTimed`)
-   Downgradable write locks (`RawRwLockDowngradable`)
-   Recursive read locks (`RawRwLockRecursive`)
-   Upgradable read locks (`RawRwLockUpgrade`)

The `Mutex` and `RwLock` wrappers will automatically expose this
additional functionality if the raw lock type implements these extension
traits.

# <a href="#cargo-features" class="doc-anchor">§</a>Cargo features

This crate supports three cargo features:

-   `owning_ref`: Allows your lock types to be used with the
    `owning_ref` crate.
-   `arc_lock`: Enables locking from an `Arc`. This enables types such
    as `ArcMutexGuard`. Note that this requires the `alloc` crate to be
    present.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.GuardNoSend.html" class="struct"
title="struct lock_api::GuardNoSend">GuardNoSend</a>  
Marker type which indicates that the Guard type for a lock is not
`Send`.

<a href="struct.GuardSend.html" class="struct"
title="struct lock_api::GuardSend">GuardSend</a>  
Marker type which indicates that the Guard type for a lock is `Send`.

<a href="struct.MappedMutexGuard.html" class="struct"
title="struct lock_api::MappedMutexGuard">MappedMutexGuard</a>  
An RAII mutex guard returned by `MutexGuard::map`, which can point to a
subfield of the protected data.

<a href="struct.MappedReentrantMutexGuard.html" class="struct"
title="struct lock_api::MappedReentrantMutexGuard">MappedReentrantMutexGuard</a>  
An RAII mutex guard returned by `ReentrantMutexGuard::map`, which can
point to a subfield of the protected data.

<a href="struct.MappedRwLockReadGuard.html" class="struct"
title="struct lock_api::MappedRwLockReadGuard">MappedRwLockReadGuard</a>  
An RAII read lock guard returned by `RwLockReadGuard::map`, which can
point to a subfield of the protected data.

<a href="struct.MappedRwLockWriteGuard.html" class="struct"
title="struct lock_api::MappedRwLockWriteGuard">MappedRwLockWriteGuard</a>  
An RAII write lock guard returned by `RwLockWriteGuard::map`, which can
point to a subfield of the protected data.

<a href="struct.Mutex.html" class="struct"
title="struct lock_api::Mutex">Mutex</a>  
A mutual exclusion primitive useful for protecting shared data

<a href="struct.MutexGuard.html" class="struct"
title="struct lock_api::MutexGuard">MutexGuard</a>  
An RAII implementation of a “scoped lock” of a mutex. When this
structure is dropped (falls out of scope), the lock will be unlocked.

<a href="struct.RawReentrantMutex.html" class="struct"
title="struct lock_api::RawReentrantMutex">RawReentrantMutex</a>  
A raw mutex type that wraps another raw mutex to provide reentrancy.

<a href="struct.ReentrantMutex.html" class="struct"
title="struct lock_api::ReentrantMutex">ReentrantMutex</a>  
A mutex which can be recursively locked by a single thread.

<a href="struct.ReentrantMutexGuard.html" class="struct"
title="struct lock_api::ReentrantMutexGuard">ReentrantMutexGuard</a>  
An RAII implementation of a “scoped lock” of a reentrant mutex. When
this structure is dropped (falls out of scope), the lock will be
unlocked.

<a href="struct.RwLock.html" class="struct"
title="struct lock_api::RwLock">RwLock</a>  
A reader-writer lock

<a href="struct.RwLockReadGuard.html" class="struct"
title="struct lock_api::RwLockReadGuard">RwLockReadGuard</a>  
RAII structure used to release the shared read access of a lock when
dropped.

<a href="struct.RwLockUpgradableReadGuard.html" class="struct"
title="struct lock_api::RwLockUpgradableReadGuard">RwLockUpgradableReadGuard</a>  
RAII structure used to release the upgradable read access of a lock when
dropped.

<a href="struct.RwLockWriteGuard.html" class="struct"
title="struct lock_api::RwLockWriteGuard">RwLockWriteGuard</a>  
RAII structure used to release the exclusive write access of a lock when
dropped.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.GetThreadId.html" class="trait"
title="trait lock_api::GetThreadId">GetThreadId</a>  
Helper trait which returns a non-zero thread ID.

<a href="trait.RawMutex.html" class="trait"
title="trait lock_api::RawMutex">RawMutex</a>  
Basic operations for a mutex.

<a href="trait.RawMutexFair.html" class="trait"
title="trait lock_api::RawMutexFair">RawMutexFair</a>  
Additional methods for mutexes which support fair unlocking.

<a href="trait.RawMutexTimed.html" class="trait"
title="trait lock_api::RawMutexTimed">RawMutexTimed</a>  
Additional methods for mutexes which support locking with timeouts.

<a href="trait.RawRwLock.html" class="trait"
title="trait lock_api::RawRwLock">RawRwLock</a>  
Basic operations for a reader-writer lock.

<a href="trait.RawRwLockDowngrade.html" class="trait"
title="trait lock_api::RawRwLockDowngrade">RawRwLockDowngrade</a>  
Additional methods for `RwLock`s which support atomically downgrading an
exclusive lock to a shared lock.

<a href="trait.RawRwLockFair.html" class="trait"
title="trait lock_api::RawRwLockFair">RawRwLockFair</a>  
Additional methods for `RwLock`s which support fair unlocking.

<a href="trait.RawRwLockRecursive.html" class="trait"
title="trait lock_api::RawRwLockRecursive">RawRwLockRecursive</a>  
Additional methods for `RwLock`s which support recursive read locks.

<a href="trait.RawRwLockRecursiveTimed.html" class="trait"
title="trait lock_api::RawRwLockRecursiveTimed">RawRwLockRecursiveTimed</a>  
Additional methods for `RwLock`s which support recursive read locks and
timeouts.

<a href="trait.RawRwLockTimed.html" class="trait"
title="trait lock_api::RawRwLockTimed">RawRwLockTimed</a>  
Additional methods for `RwLock`s which support locking with timeouts.

<a href="trait.RawRwLockUpgrade.html" class="trait"
title="trait lock_api::RawRwLockUpgrade">RawRwLockUpgrade</a>  
Additional methods for `RwLock`s which support atomically upgrading a
shared lock to an exclusive lock.

<a href="trait.RawRwLockUpgradeDowngrade.html" class="trait"
title="trait lock_api::RawRwLockUpgradeDowngrade">RawRwLockUpgradeDowngrade</a>  
Additional methods for `RwLock`s which support upgradable locks and lock
downgrading.

<a href="trait.RawRwLockUpgradeFair.html" class="trait"
title="trait lock_api::RawRwLockUpgradeFair">RawRwLockUpgradeFair</a>  
Additional methods for `RwLock`s which support upgradable locks and fair
unlocking.

<a href="trait.RawRwLockUpgradeTimed.html" class="trait"
title="trait lock_api::RawRwLockUpgradeTimed">RawRwLockUpgradeTimed</a>  
Additional methods for `RwLock`s which support upgradable locks and
locking with timeouts.
