Crate futures\_timer Copy item path

<span class="sub-heading"><a href="../src/futures_timer/lib.rs.html#1-27" class="src">Source</a>
</span>

Expand description

A general purpose crate for working with timeouts and delays with
futures.

# <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use std::time::Duration;
use futures_timer::Delay;

let now = Delay::new(Duration::from_secs(3)).await;
println!("waited for 3 secs");
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Delay.html" class="struct"
title="struct futures_timer::Delay">Delay</a>  
A future representing the notification that an elapsed duration has
occurred.
