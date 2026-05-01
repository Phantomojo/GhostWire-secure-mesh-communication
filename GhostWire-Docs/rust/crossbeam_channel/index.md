Crate crossbeam\_channel Copy item path

<span class="sub-heading"><a href="../src/crossbeam_channel/lib.rs.html#1-378"
class="src">Source</a> </span>

Expand description

Multi-producer multi-consumer channels for message passing.

This crate is an alternative to
[`std::sync::mpsc`](https://doc.rust-lang.org/1.93.1/std/sync/mpsc/index.html "mod std::sync::mpsc")
with more features and better performance.

# <a href="#hello-world" class="doc-anchor">§</a>Hello, world!

``` {.rust .rust-example-rendered}
use crossbeam_channel::unbounded;

// Create a channel of unbounded capacity.
let (s, r) = unbounded();

// Send a message into the channel.
s.send("Hello, world!").unwrap();

// Receive the message from the channel.
assert_eq!(r.recv(), Ok("Hello, world!"));
```

# <a href="#channel-types" class="doc-anchor">§</a>Channel types

Channels can be created using two functions:

-   [`bounded`](fn.bounded.html "fn crossbeam_channel::bounded") creates
    a channel of bounded capacity, i.e. there is a limit to how many
    messages it can hold at a time.

-   [`unbounded`](fn.unbounded.html "fn crossbeam_channel::unbounded")
    creates a channel of unbounded capacity, i.e. it can hold any number
    of messages at a time.

Both functions return a
[`Sender`](struct.Sender.html "struct crossbeam_channel::Sender") and a
[`Receiver`](struct.Receiver.html "struct crossbeam_channel::Receiver"),
which represent the two opposite sides of a channel.

Creating a bounded channel:

``` {.rust .rust-example-rendered}
use crossbeam_channel::bounded;

// Create a channel that can hold at most 5 messages at a time.
let (s, r) = bounded(5);

// Can send only 5 messages without blocking.
for i in 0..5 {
    s.send(i).unwrap();
}

// Another call to `send` would block because the channel is full.
// s.send(5).unwrap();
```

Creating an unbounded channel:

``` {.rust .rust-example-rendered}
use crossbeam_channel::unbounded;

// Create an unbounded channel.
let (s, r) = unbounded();

// Can send any number of messages into the channel without blocking.
for i in 0..1000 {
    s.send(i).unwrap();
}
```

A special case is zero-capacity channel, which cannot hold any messages.
Instead, send and receive operations must appear at the same time in
order to pair up and pass the message over:

``` {.rust .rust-example-rendered}
use std::thread;
use crossbeam_channel::bounded;

// Create a zero-capacity channel.
let (s, r) = bounded(0);

// Sending blocks until a receive operation appears on the other side.
thread::spawn(move || s.send("Hi!").unwrap());

// Receiving blocks until a send operation appears on the other side.
assert_eq!(r.recv(), Ok("Hi!"));
```

# <a href="#sharing-channels" class="doc-anchor">§</a>Sharing channels

Senders and receivers can be cloned and sent to other threads:

``` {.rust .rust-example-rendered}
use std::thread;
use crossbeam_channel::bounded;

let (s1, r1) = bounded(0);
let (s2, r2) = (s1.clone(), r1.clone());

// Spawn a thread that receives a message and then sends one.
thread::spawn(move || {
    r2.recv().unwrap();
    s2.send(2).unwrap();
});

// Send a message and then receive one.
s1.send(1).unwrap();
r1.recv().unwrap();
```

Note that cloning only creates a new handle to the same sending or
receiving side. It does not create a separate stream of messages in any
way:

``` {.rust .rust-example-rendered}
use crossbeam_channel::unbounded;

let (s1, r1) = unbounded();
let (s2, r2) = (s1.clone(), r1.clone());
let (s3, r3) = (s2.clone(), r2.clone());

s1.send(10).unwrap();
s2.send(20).unwrap();
s3.send(30).unwrap();

assert_eq!(r3.recv(), Ok(10));
assert_eq!(r1.recv(), Ok(20));
assert_eq!(r2.recv(), Ok(30));
```

It’s also possible to share senders and receivers by reference:

``` {.rust .rust-example-rendered}
use crossbeam_channel::bounded;
use crossbeam_utils::thread::scope;

let (s, r) = bounded(0);

scope(|scope| {
    // Spawn a thread that receives a message and then sends one.
    scope.spawn(|_| {
        r.recv().unwrap();
        s.send(2).unwrap();
    });

    // Send a message and then receive one.
    s.send(1).unwrap();
    r.recv().unwrap();
}).unwrap();
```

# <a href="#disconnection" class="doc-anchor">§</a>Disconnection

When all senders or all receivers associated with a channel get dropped,
the channel becomes disconnected. No more messages can be sent, but any
remaining messages can still be received. Send and receive operations on
a disconnected channel never block.

``` {.rust .rust-example-rendered}
use crossbeam_channel::{unbounded, RecvError};

let (s, r) = unbounded();
s.send(1).unwrap();
s.send(2).unwrap();
s.send(3).unwrap();

// The only sender is dropped, disconnecting the channel.
drop(s);

// The remaining messages can be received.
assert_eq!(r.recv(), Ok(1));
assert_eq!(r.recv(), Ok(2));
assert_eq!(r.recv(), Ok(3));

// There are no more messages in the channel.
assert!(r.is_empty());

// Note that calling `r.recv()` does not block.
// Instead, `Err(RecvError)` is returned immediately.
assert_eq!(r.recv(), Err(RecvError));
```

# <a href="#blocking-operations" class="doc-anchor">§</a>Blocking operations

Send and receive operations come in three flavors:

-   Non-blocking (returns immediately with success or failure).
-   Blocking (waits until the operation succeeds or the channel becomes
    disconnected).
-   Blocking with a timeout (blocks only for a certain duration of
    time).

A simple example showing the difference between non-blocking and
blocking operations:

``` {.rust .rust-example-rendered}
use crossbeam_channel::{bounded, RecvError, TryRecvError};

let (s, r) = bounded(1);

// Send a message into the channel.
s.send("foo").unwrap();

// This call would block because the channel is full.
// s.send("bar").unwrap();

// Receive the message.
assert_eq!(r.recv(), Ok("foo"));

// This call would block because the channel is empty.
// r.recv();

// Try receiving a message without blocking.
assert_eq!(r.try_recv(), Err(TryRecvError::Empty));

// Disconnect the channel.
drop(s);

// This call doesn't block because the channel is now disconnected.
assert_eq!(r.recv(), Err(RecvError));
```

# <a href="#iteration" class="doc-anchor">§</a>Iteration

Receivers can be used as iterators. For example, method
[`iter`](struct.Receiver.html#method.iter "method crossbeam_channel::Receiver::iter")
creates an iterator that receives messages until the channel becomes
empty and disconnected. Note that iteration may block waiting for next
message to arrive.

``` {.rust .rust-example-rendered}
use std::thread;
use crossbeam_channel::unbounded;

let (s, r) = unbounded();

thread::spawn(move || {
    s.send(1).unwrap();
    s.send(2).unwrap();
    s.send(3).unwrap();
    drop(s); // Disconnect the channel.
});

// Collect all messages from the channel.
// Note that the call to `collect` blocks until the sender is dropped.
let v: Vec<_> = r.iter().collect();

assert_eq!(v, [1, 2, 3]);
```

A non-blocking iterator can be created using
[`try_iter`](struct.Receiver.html#method.try_iter "method crossbeam_channel::Receiver::try_iter"),
which receives all available messages without blocking:

``` {.rust .rust-example-rendered}
use crossbeam_channel::unbounded;

let (s, r) = unbounded();
s.send(1).unwrap();
s.send(2).unwrap();
s.send(3).unwrap();
// No need to drop the sender.

// Receive all messages currently in the channel.
let v: Vec<_> = r.try_iter().collect();

assert_eq!(v, [1, 2, 3]);
```

# <a href="#selection" class="doc-anchor">§</a>Selection

The [`select!`](macro.select.html "macro crossbeam_channel::select")
macro allows you to define a set of channel operations, wait until any
one of them becomes ready, and finally execute it. If multiple
operations are ready at the same time, a random one among them is
selected.

It is also possible to define a `default` case that gets executed if
none of the operations are ready, either right away or for a certain
duration of time.

An operation is considered to be ready if it doesn’t have to block. Note
that it is ready even when it will simply return an error because the
channel is disconnected.

An example of receiving a message from two channels:

``` {.rust .rust-example-rendered}
use std::thread;
use std::time::Duration;
use crossbeam_channel::{select, unbounded};

let (s1, r1) = unbounded();
let (s2, r2) = unbounded();

thread::spawn(move || s1.send(10).unwrap());
thread::spawn(move || s2.send(20).unwrap());

// At most one of these two receive operations will be executed.
select! {
    recv(r1) -> msg => assert_eq!(msg, Ok(10)),
    recv(r2) -> msg => assert_eq!(msg, Ok(20)),
    default(Duration::from_secs(1)) => println!("timed out"),
}
```

If you need to select over a dynamically created list of channel
operations, use
[`Select`](struct.Select.html "struct crossbeam_channel::Select")
instead. The
[`select!`](macro.select.html "macro crossbeam_channel::select") macro
is just a convenience wrapper around
[`Select`](struct.Select.html "struct crossbeam_channel::Select").

# <a href="#extra-channels" class="doc-anchor">§</a>Extra channels

Three functions can create special kinds of channels, all of which
return just a
[`Receiver`](struct.Receiver.html "struct crossbeam_channel::Receiver")
handle:

-   [`after`](fn.after.html "fn crossbeam_channel::after") creates a
    channel that delivers a single message after a certain duration of
    time.
-   [`tick`](fn.tick.html "fn crossbeam_channel::tick") creates a
    channel that delivers messages periodically.
-   [`never`](fn.never.html "fn crossbeam_channel::never") creates a
    channel that never delivers messages.

These channels are very efficient because messages get lazily generated
on receive operations.

An example that prints elapsed time every 50 milliseconds for the
duration of 1 second:

``` {.rust .rust-example-rendered}
use std::time::{Duration, Instant};
use crossbeam_channel::{after, select, tick};

let start = Instant::now();
let ticker = tick(Duration::from_millis(50));
let timeout = after(Duration::from_secs(1));

loop {
    select! {
        recv(ticker) -> _ => println!("elapsed: {:?}", start.elapsed()),
        recv(timeout) -> _ => break,
    }
}
```

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.select.html" class="macro"
title="macro crossbeam_channel::select">select</a>  
Selects from a set of channel operations.

<a href="macro.select_biased.html" class="macro"
title="macro crossbeam_channel::select_biased">select_biased</a>  
Selects from a set of channel operations.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.IntoIter.html" class="struct"
title="struct crossbeam_channel::IntoIter">IntoIter</a>  
A blocking iterator over messages in a channel.

<a href="struct.Iter.html" class="struct"
title="struct crossbeam_channel::Iter">Iter</a>  
A blocking iterator over messages in a channel.

<a href="struct.ReadyTimeoutError.html" class="struct"
title="struct crossbeam_channel::ReadyTimeoutError">ReadyTimeoutError</a>  
An error returned from the
[`ready_timeout`](struct.Select.html#method.ready_timeout "method crossbeam_channel::Select::ready_timeout")
method.

<a href="struct.Receiver.html" class="struct"
title="struct crossbeam_channel::Receiver">Receiver</a>  
The receiving side of a channel.

<a href="struct.RecvError.html" class="struct"
title="struct crossbeam_channel::RecvError">RecvError</a>  
An error returned from the
[`recv`](struct.Receiver.html#method.recv "method crossbeam_channel::Receiver::recv")
method.

<a href="struct.Select.html" class="struct"
title="struct crossbeam_channel::Select">Select</a>  
Selects from a set of channel operations.

<a href="struct.SelectTimeoutError.html" class="struct"
title="struct crossbeam_channel::SelectTimeoutError">SelectTimeoutError</a>  
An error returned from the
[`select_timeout`](struct.Select.html#method.select_timeout "method crossbeam_channel::Select::select_timeout")
method.

<a href="struct.SelectedOperation.html" class="struct"
title="struct crossbeam_channel::SelectedOperation">SelectedOperation</a>  
A selected operation that needs to be completed.

<a href="struct.SendError.html" class="struct"
title="struct crossbeam_channel::SendError">SendError</a>  
An error returned from the
[`send`](struct.Sender.html#method.send "method crossbeam_channel::Sender::send")
method.

<a href="struct.Sender.html" class="struct"
title="struct crossbeam_channel::Sender">Sender</a>  
The sending side of a channel.

<a href="struct.TryIter.html" class="struct"
title="struct crossbeam_channel::TryIter">TryIter</a>  
A non-blocking iterator over messages in a channel.

<a href="struct.TryReadyError.html" class="struct"
title="struct crossbeam_channel::TryReadyError">TryReadyError</a>  
An error returned from the
[`try_ready`](struct.Select.html#method.try_ready "method crossbeam_channel::Select::try_ready")
method.

<a href="struct.TrySelectError.html" class="struct"
title="struct crossbeam_channel::TrySelectError">TrySelectError</a>  
An error returned from the
[`try_select`](struct.Select.html#method.try_select "method crossbeam_channel::Select::try_select")
method.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.RecvTimeoutError.html" class="enum"
title="enum crossbeam_channel::RecvTimeoutError">RecvTimeoutError</a>  
An error returned from the
[`recv_timeout`](struct.Receiver.html#method.recv_timeout "method crossbeam_channel::Receiver::recv_timeout")
method.

<a href="enum.SendTimeoutError.html" class="enum"
title="enum crossbeam_channel::SendTimeoutError">SendTimeoutError</a>  
An error returned from the
[`send_timeout`](struct.Sender.html#method.send_timeout "method crossbeam_channel::Sender::send_timeout")
method.

<a href="enum.TryRecvError.html" class="enum"
title="enum crossbeam_channel::TryRecvError">TryRecvError</a>  
An error returned from the
[`try_recv`](struct.Receiver.html#method.try_recv "method crossbeam_channel::Receiver::try_recv")
method.

<a href="enum.TrySendError.html" class="enum"
title="enum crossbeam_channel::TrySendError">TrySendError</a>  
An error returned from the
[`try_send`](struct.Sender.html#method.try_send "method crossbeam_channel::Sender::try_send")
method.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.after.html" class="fn"
title="fn crossbeam_channel::after">after</a>  
Creates a receiver that delivers a message after a certain duration of
time.

<a href="fn.at.html" class="fn" title="fn crossbeam_channel::at">at</a>  
Creates a receiver that delivers a message at a certain instant in time.

<a href="fn.bounded.html" class="fn"
title="fn crossbeam_channel::bounded">bounded</a>  
Creates a channel of bounded capacity.

<a href="fn.never.html" class="fn"
title="fn crossbeam_channel::never">never</a>  
Creates a receiver that never delivers messages.

<a href="fn.tick.html" class="fn"
title="fn crossbeam_channel::tick">tick</a>  
Creates a receiver that delivers messages periodically.

<a href="fn.unbounded.html" class="fn"
title="fn crossbeam_channel::unbounded">unbounded</a>  
Creates a channel of unbounded capacity.
