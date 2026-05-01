Crate event\_listener Copy item path

<span class="sub-heading"><a href="../src/event_listener/lib.rs.html#1-1559"
class="src">Source</a> </span>

Expand description

Notify async tasks or threads.

This is a synchronization primitive similar to
[eventcounts](https://www.1024cores.net/home/lock-free-algorithms/eventcounts)
invented by Dmitry Vyukov.

You can use this crate to turn non-blocking data structures into async
or blocking data structures. See a [simple
mutex](https://github.com/smol-rs/event-listener/blob/master/examples/mutex.rs)
implementation that exposes an async and a blocking interface for
acquiring locks.

# <a href="#examples" class="doc-anchor">§</a>Examples

Wait until another thread sets a boolean flag:

``` {.rust .rust-example-rendered}
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::usize;
use event_listener::{Event, Listener};

let flag = Arc::new(AtomicBool::new(false));
let event = Arc::new(Event::new());

// Spawn a thread that will set the flag after 1 second.
thread::spawn({
    let flag = flag.clone();
    let event = event.clone();
    move || {
        // Wait for a second.
        thread::sleep(Duration::from_secs(1));

        // Set the flag.
        flag.store(true, Ordering::SeqCst);

        // Notify all listeners that the flag has been set.
        event.notify(usize::MAX);
    }
});

// Wait until the flag is set.
loop {
    // Check the flag.
    if flag.load(Ordering::SeqCst) {
        break;
    }

    // Start listening for events.
    let mut listener = event.listen();

    // Check the flag again after creating the listener.
    if flag.load(Ordering::SeqCst) {
        break;
    }

    // Wait for a notification and continue the loop.
    listener.wait();
}
```

# <a href="#features" class="doc-anchor">§</a>Features

-   The `std` feature (enabled by default) enables the use of the Rust
    standard library. Disable it for `no_std` support.

-   The `critical-section` feature enables usage of the
    [`critical-section`](https://crates.io/crates/critical-section)
    crate to enable a more efficient implementation of `event-listener`
    for `no_std` platforms.

-   The `portable-atomic` feature enables the use of the
    [`portable-atomic`](https://crates.io/crates/portable-atomic) crate
    to provide atomic operations on platforms that don’t support them.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.listener.html" class="macro"
title="macro event_listener::listener">listener</a>  
Create a stack-based event listener for an
[`Event`](struct.Event.html "struct event_listener::Event").

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Event.html" class="struct"
title="struct event_listener::Event">Event</a>  
A synchronization primitive for notifying async tasks and threads.

<a href="struct.EventListener.html" class="struct"
title="struct event_listener::EventListener">EventListener</a>  
A guard waiting for a notification from an
[`Event`](struct.Event.html "struct event_listener::Event").

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.IntoNotification.html" class="trait"
title="trait event_listener::IntoNotification">IntoNotification</a>  
A value that can be converted into a
[`Notification`](trait.Notification.html "trait event_listener::Notification").

<a href="trait.Listener.html" class="trait"
title="trait event_listener::Listener">Listener</a>  
A handle that is listening to an
[`Event`](struct.Event.html "struct event_listener::Event").

<a href="trait.Notification.html" class="trait"
title="trait event_listener::Notification">Notification</a>  
A notification that can be used to notify an
[`Event`](struct.Event.html "struct event_listener::Event").
