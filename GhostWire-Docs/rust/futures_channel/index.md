Crate futures\_channel Copy item path

<span class="sub-heading"><a href="../src/futures_channel/lib.rs.html#1-38" class="src">Source</a>
</span>

Expand description

Asynchronous channels.

Like threads, concurrent tasks sometimes need to communicate with each
other. This module contains two basic abstractions for doing so:

-   [oneshot](oneshot/index.html "mod futures_channel::oneshot"), a way
    of sending a single value from one task to another.
-   [mpsc](mpsc/index.html "mod futures_channel::mpsc"), a
    multi-producer, single-consumer channel for sending values between
    tasks, analogous to the similarly-named structure in the standard
    library.

All items are only available when the `std` or `alloc` feature of this
library is activated, and it is activated by default.

# Modules<a href="#modules" class="anchor">§</a>

<a href="mpsc/index.html" class="mod"
title="mod futures_channel::mpsc">mpsc</a>  
A multi-producer, single-consumer queue for sending values across
asynchronous tasks.

<a href="oneshot/index.html" class="mod"
title="mod futures_channel::oneshot">oneshot</a>  
A channel for sending a single message between asynchronous tasks.
