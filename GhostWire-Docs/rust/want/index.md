Crate want Copy item path

<span class="sub-heading"><a href="../src/want/lib.rs.html#1-579" class="src">Source</a>
</span>

Expand description

A Futures channel-like utility to signal when a value is wanted.

Futures are supposed to be lazy, and only starting work if
`Future::poll` is called. The same is true of `Stream`s, but when using
a channel as a `Stream`, it can be hard to know if the receiver is ready
for the next value.

Put another way, given a `(tx, rx)` from
`futures::sync::mpsc::channel()`, how can the sender (`tx`) know when
the receiver (`rx`) actually wants more work to be produced? Just
because there is room in the channel buffer doesn’t mean the work would
be used by the receiver.

This is where something like `want` comes in. Added to a channel, you
can make sure that the `tx` only creates the message and sends it when
the `rx` has `poll()` for it, and the buffer was empty.

# <a href="#example" class="doc-anchor">§</a>Example

``` language-nightly
# //#![feature(async_await)]
extern crate want;

# fn spawn<T>(_t: T) {}
# fn we_still_want_message() -> bool { true }
# fn mpsc_channel() -> (Tx, Rx) { (Tx, Rx) }
# struct Tx;
# impl Tx { fn send<T>(&mut self, _: T) {} }
# struct Rx;
# impl Rx { async fn recv(&mut self) -> Option<Expensive> { Some(Expensive) } }

// Some message that is expensive to produce.
struct Expensive;

// Some futures-aware MPSC channel...
let (mut tx, mut rx) = mpsc_channel();

// And our `want` channel!
let (mut gv, mut tk) = want::new();


// Our receiving task...
spawn(async move {
    // Maybe something comes up that prevents us from ever
    // using the expensive message.
    //
    // Without `want`, the "send" task may have started to
    // produce the expensive message even though we wouldn't
    // be able to use it.
    if !we_still_want_message() {
        return;
    }

    // But we can use it! So tell the `want` channel.
    tk.want();

    match rx.recv().await {
        Some(_msg) => println!("got a message"),
        None => println!("DONE"),
    }
});

// Our sending task
spawn(async move {
    // It's expensive to create a new message, so we wait until the
    // receiving end truly *wants* the message.
    if let Err(_closed) = gv.want().await {
        // Looks like they will never want it...
        return;
    }

    // They want it, let's go!
    tx.send(Expensive);
});

# fn main() {}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Closed.html" class="struct"
title="struct want::Closed">Closed</a>  
The `Taker` has canceled its interest in a value.

<a href="struct.Giver.html" class="struct"
title="struct want::Giver">Giver</a>  
An entity that gives a value when wanted.

<a href="struct.SharedGiver.html" class="struct"
title="struct want::SharedGiver">SharedGiver</a>  
A cloneable `Giver`.

<a href="struct.Taker.html" class="struct"
title="struct want::Taker">Taker</a>  
An entity that wants a value.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.new.html" class="fn" title="fn want::new">new</a>  
Create a new `want` channel.
