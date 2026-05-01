Crate event\_listener\_strategy Copy item path

<span class="sub-heading"><a href="../src/event_listener_strategy/lib.rs.html#2-549"
class="src">Source</a> </span>

Expand description

A strategy for using the \[`event-listener`\] crate in both blocking and
non-blocking contexts.

One of the stand-out features of the \[`event-listener`\] crate is the
ability to use it in both asynchronous and synchronous contexts.
However, sometimes using it like this causes a lot of boilerplate to be
duplicated. This crate aims to reduce that boilerplate by providing an
[`EventListenerFuture`](trait.EventListenerFuture.html "trait event_listener_strategy::EventListenerFuture")
trait that implements both blocking and non-blocking functionality.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use event_listener_strategy::{
   event_listener::{Event, EventListener},
   EventListenerFuture, FutureWrapper, Strategy
};

use std::pin::Pin;
use std::task::Poll;
use std::thread;
use std::sync::Arc;

// A future that waits three seconds for an event to be fired.
fn wait_three_seconds() -> WaitThreeSeconds {
    let event = Event::new();
    let listener = event.listen();

    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_secs(3));
        event.notify(1);
    });

    WaitThreeSeconds { listener: Some(listener) }
}

struct WaitThreeSeconds {
    listener: Option<EventListener>,
}

impl EventListenerFuture for WaitThreeSeconds {
    type Output = ();

    fn poll_with_strategy<'a, S: Strategy<'a>>(
        mut self: Pin<&mut Self>,
        strategy: &mut S,
        context: &mut S::Context,
    ) -> Poll<Self::Output> {
        strategy.poll(&mut self.listener, context)
    }
}

// Use the future in a blocking context.
let future = wait_three_seconds();
future.wait();

// Use the future in a non-blocking context.
futures_lite::future::block_on(async {
    let future = FutureWrapper::new(wait_three_seconds());
    future.await;
});
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../event_listener/index.html" class="mod"
title="mod event_listener"><code>event_listener</code></a>`;`

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.easy_wrapper.html" class="macro"
title="macro event_listener_strategy::easy_wrapper">easy_wrapper</a>  
A wrapper around an
[`EventListenerFuture`](trait.EventListenerFuture.html "trait event_listener_strategy::EventListenerFuture")
that can be easily exported for use.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Blocking.html" class="struct"
title="struct event_listener_strategy::Blocking">Blocking</a>  
A strategy that blocks the current thread until the event is signalled.

<a href="struct.FutureWrapper.html" class="struct"
title="struct event_listener_strategy::FutureWrapper">FutureWrapper</a>  
A wrapper around an
[`EventListenerFuture`](trait.EventListenerFuture.html "trait event_listener_strategy::EventListenerFuture")
that implements
[`Future`](https://doc.rust-lang.org/1.93.1/core/future/future/trait.Future.html "trait core::future::future::Future").

<a href="struct.NonBlocking.html" class="struct"
title="struct event_listener_strategy::NonBlocking">NonBlocking</a>  
A strategy that uses polling to efficiently wait for an event.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.EventListenerFuture.html" class="trait"
title="trait event_listener_strategy::EventListenerFuture">EventListenerFuture</a>  
A future that runs using the \[`event-listener`\] crate.

<a href="trait.Strategy.html" class="trait"
title="trait event_listener_strategy::Strategy">Strategy</a>  
A strategy for polling an
[`EventListenerFuture`](trait.EventListenerFuture.html "trait event_listener_strategy::EventListenerFuture")
or an
[`EventListener`](../event_listener/struct.EventListener.html "struct event_listener::EventListener").
