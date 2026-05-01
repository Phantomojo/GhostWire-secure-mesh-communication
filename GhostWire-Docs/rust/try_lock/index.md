Crate try\_lock Copy item path

<span class="sub-heading"><a href="../src/try_lock/lib.rs.html#1-266" class="src">Source</a>
</span>

Expand description

A light-weight lock guarded by an atomic boolean.

Most efficient when contention is low, acquiring the lock is a single
atomic swap, and releasing it just 1 more atomic swap.

# <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use std::sync::Arc;
use try_lock::TryLock;

// a thing we want to share
struct Widget {
    name: String,
}

// lock it up!
let widget1 = Arc::new(TryLock::new(Widget {
    name: "Spanner".into(),
}));

let widget2 = widget1.clone();


// mutate the widget
let mut locked = widget1.try_lock().expect("example isn't locked yet");
locked.name.push_str(" Bundle");

// hands off, buddy
let not_locked = widget2.try_lock();
assert!(not_locked.is_none(), "widget1 has the lock");

// ok, you can have it
drop(locked);

let locked2 = widget2.try_lock().expect("widget1 lock is released");

assert_eq!(locked2.name, "Spanner Bundle");
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Locked.html" class="struct"
title="struct try_lock::Locked">Locked</a>  
A locked value acquired from a `TryLock`.

<a href="struct.TryLock.html" class="struct"
title="struct try_lock::TryLock">TryLock</a>  
A light-weight lock guarded by an atomic boolean.
