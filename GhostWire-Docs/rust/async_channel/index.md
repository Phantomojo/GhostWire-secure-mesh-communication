Crate async\_channel Copy item path

<span class="sub-heading"><a href="../src/async_channel/lib.rs.html#1-1388" class="src">Source</a>
</span>

Expand description

An async multi-producer multi-consumer channel, where each message can
be received by only one of all existing consumers.

There are two kinds of channels:

1.  [Bounded](fn.bounded.html "fn async_channel::bounded") channel with
    limited capacity.
2.  [Unbounded](fn.unbounded.html "fn async_channel::unbounded") channel
    with unlimited capacity.

A channel has the
[`Sender`](struct.Sender.html "struct async_channel::Sender") and
[`Receiver`](struct.Receiver.html "struct async_channel::Receiver")
side. Both sides are cloneable and can be shared among multiple threads.

When all [`Sender`](struct.Sender.html "struct async_channel::Sender")s
or all
[`Receiver`](struct.Receiver.html "struct async_channel::Receiver")s are
dropped, the channel becomes closed. When a channel is closed, no more
messages can be sent, but remaining messages can still be received.

The channel can also be closed manually by calling
[`Sender::close()`](struct.Sender.html#method.close "method async_channel::Sender::close")
or
[`Receiver::close()`](struct.Receiver.html#method.close "method async_channel::Receiver::close").

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
let (s, r) = async_channel::unbounded();

assert_eq!(s.send("Hello").await, Ok(()));
assert_eq!(r.recv().await, Ok("Hello"));
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Closed.html" class="struct"
title="struct async_channel::Closed">Closed</a>  
A future returned by
[`Sender::closed()`](struct.Sender.html#method.closed "method async_channel::Sender::closed").

<a href="struct.Receiver.html" class="struct"
title="struct async_channel::Receiver">Receiver</a>  
The receiving side of a channel.

<a href="struct.Recv.html" class="struct"
title="struct async_channel::Recv">Recv</a>  
A future returned by
[`Receiver::recv()`](struct.Receiver.html#method.recv "method async_channel::Receiver::recv").

<a href="struct.RecvError.html" class="struct"
title="struct async_channel::RecvError">RecvError</a>  
An error returned from
[`Receiver::recv()`](struct.Receiver.html#method.recv "method async_channel::Receiver::recv").

<a href="struct.Send.html" class="struct"
title="struct async_channel::Send">Send</a>  
A future returned by
[`Sender::send()`](struct.Sender.html#method.send "method async_channel::Sender::send").

<a href="struct.SendError.html" class="struct"
title="struct async_channel::SendError">SendError</a>  
An error returned from
[`Sender::send()`](struct.Sender.html#method.send "method async_channel::Sender::send").

<a href="struct.Sender.html" class="struct"
title="struct async_channel::Sender">Sender</a>  
The sending side of a channel.

<a href="struct.WeakReceiver.html" class="struct"
title="struct async_channel::WeakReceiver">WeakReceiver</a>  
A [`Receiver`](struct.Receiver.html "struct async_channel::Receiver")
that does not prevent the channel from being closed.

<a href="struct.WeakSender.html" class="struct"
title="struct async_channel::WeakSender">WeakSender</a>  
A [`Sender`](struct.Sender.html "struct async_channel::Sender") that
does not prevent the channel from being closed.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.TryRecvError.html" class="enum"
title="enum async_channel::TryRecvError">TryRecvError</a>  
An error returned from
[`Receiver::try_recv()`](struct.Receiver.html#method.try_recv "method async_channel::Receiver::try_recv").

<a href="enum.TrySendError.html" class="enum"
title="enum async_channel::TrySendError">TrySendError</a>  
An error returned from
[`Sender::try_send()`](struct.Sender.html#method.try_send "method async_channel::Sender::try_send").

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.bounded.html" class="fn"
title="fn async_channel::bounded">bounded</a>  
Creates a bounded channel.

<a href="fn.unbounded.html" class="fn"
title="fn async_channel::unbounded">unbounded</a>  
Creates an unbounded channel.
