Crate spinning\_top Copy item path

<span class="sub-heading"><a href="../src/spinning_top/lib.rs.html#1-86" class="src">Source</a>
</span>

Expand description

Provides simple spinlocks based on the abstractions provided by the
[`lock_api`](https://docs.rs/lock_api/) crate.

# <a href="#examples" class="doc-anchor">§</a>Examples

Use [`Spinlock`](type.Spinlock.html "type spinning_top::Spinlock") for
mutual exclusion:

``` {.rust .rust-example-rendered}
use spinning_top::Spinlock;

fn main() {
    let data = String::from("Hello");
    // Wrap some data in a spinlock
    let spinlock = Spinlock::new(data);

    // Lock the spinlock to get a mutex guard for the data
    let mut locked_data = spinlock.lock();
    // The guard implements the `Deref` trait, so we can use it like a `&String`
    assert_eq!(locked_data.as_str(), "Hello");
    // It also implements `DerefMut` so mutation is possible too. This is safe
    // because the spinlock ensures mutual exclusion
    locked_data.make_ascii_uppercase();
    assert_eq!(locked_data.as_str(), "HELLO");

    // the guard automatically frees the lock at the end of the scope
}
```

Use [`RwSpinlock`](type.RwSpinlock.html "type spinning_top::RwSpinlock")
if you need a readers-writer lock:

``` {.rust .rust-example-rendered}
use spinning_top::RwSpinlock;

let lock = RwSpinlock::new(5);

// many reader locks can be held at once
{
    let r1 = lock.read();
    let r2 = lock.read();
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
} // read locks are dropped at this point

// only one write lock may be held, however
{
    let mut w = lock.write();
    *w += 1;
    assert_eq!(*w, 6);
} // write lock is dropped here
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../lock_api/index.html" class="mod"
title="mod lock_api"><code>lock_api</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="guard/index.html" class="mod"
title="mod spinning_top::guard">guard</a>  
Type aliases for guards.

<a href="relax/index.html" class="mod"
title="mod spinning_top::relax">relax</a>  
Relax strategies.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.RawRwSpinlock.html" class="struct"
title="struct spinning_top::RawRwSpinlock">RawRwSpinlock</a>  
A simple, spinning, read-preferring readers-writer lock.

<a href="struct.RawSpinlock.html" class="struct"
title="struct spinning_top::RawSpinlock">RawSpinlock</a>  
Provides mutual exclusion based on spinning on an `AtomicBool`.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.BackoffRwSpinlock.html" class="type"
title="type spinning_top::BackoffRwSpinlock">BackoffRwSpinlock</a>  
A
[`lock_api::RwLock`](../lock_api/rwlock/struct.RwLock.html "struct lock_api::rwlock::RwLock")
based on
[`RawRwSpinlock`](struct.RawRwSpinlock.html "struct spinning_top::RawRwSpinlock")`<`[`Backoff`](relax/struct.Backoff.html "struct spinning_top::relax::Backoff")`>`.

<a href="type.BackoffSpinlock.html" class="type"
title="type spinning_top::BackoffSpinlock">BackoffSpinlock</a>  
A mutual exclusion (Mutex) type based on busy-waiting with exponential
backoff.

<a href="type.RwSpinlock.html" class="type"
title="type spinning_top::RwSpinlock">RwSpinlock</a>  
A
[`lock_api::RwLock`](../lock_api/rwlock/struct.RwLock.html "struct lock_api::rwlock::RwLock")
based on
[`RawRwSpinlock`](struct.RawRwSpinlock.html "struct spinning_top::RawRwSpinlock").

<a href="type.Spinlock.html" class="type"
title="type spinning_top::Spinlock">Spinlock</a>  
A mutual exclusion (Mutex) type based on busy-waiting.
