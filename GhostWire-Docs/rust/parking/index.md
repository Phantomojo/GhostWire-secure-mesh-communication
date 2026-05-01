Crate parking Copy item path

<span class="sub-heading"><a href="../src/parking/lib.rs.html#1-449" class="src">Source</a>
</span>

Expand description

Thread parking and unparking.

A [`Parker`](struct.Parker.html "struct parking::Parker") is in either
the notified or unnotified state. The
[`park()`](struct.Parker.html#method.park "method parking::Parker::park")
method blocks the current thread until the
[`Parker`](struct.Parker.html "struct parking::Parker") becomes notified
and then puts it back into the unnotified state. The
[`unpark()`](struct.Unparker.html#method.unpark "method parking::Unparker::unpark")
method puts it into the notified state.

This API is similar to
[`thread::park()`](https://doc.rust-lang.org/std/thread/fn.park.html)
and
[`Thread::unpark()`](https://doc.rust-lang.org/std/thread/struct.Thread.html#method.unpark)
from the standard library. The difference is that the state “token”
managed by those functions is shared across an entire thread, and anyone
can call
[`thread::current()`](https://doc.rust-lang.org/std/thread/fn.current.html)
to access it. If you use `park` and `unpark`, but you also call a
function that uses `park` and `unpark` internally, that function could
cause a deadlock by consuming a wakeup that was intended for you. The
[`Parker`](struct.Parker.html "struct parking::Parker") object in this
crate avoids that problem by managing its own state, which isn’t shared
with unrelated callers.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use std::thread;
use std::time::Duration;
use parking::Parker;

let p = Parker::new();
let u = p.unparker();

// Notify the parker.
u.unpark();

// Wakes up immediately because the parker is notified.
p.park();

thread::spawn(move || {
    thread::sleep(Duration::from_millis(500));
    u.unpark();
});

// Wakes up when `u.unpark()` notifies and then goes back into unnotified state.
p.park();
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Parker.html" class="struct"
title="struct parking::Parker">Parker</a>  
Waits for a notification.

<a href="struct.Unparker.html" class="struct"
title="struct parking::Unparker">Unparker</a>  
Notifies a parker.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.pair.html" class="fn" title="fn parking::pair">pair</a>  
Creates a parker and an associated unparker.
