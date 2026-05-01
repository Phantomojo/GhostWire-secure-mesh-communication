[tokio](../index.html)

Module time Copy item path

<span class="sub-heading"><a href="../../src/tokio/time/mod.rs.html#1-110" class="src">Source</a>
</span>

Expand description

Utilities for tracking time.

This module provides a number of types for executing code after a set
period of time.

-   [`Sleep`](fn.sleep.html "fn tokio::time::sleep") is a future that
    does no work and completes at a specific
    [`Instant`](struct.Instant.html "struct tokio::time::Instant") in
    time.

-   [`Interval`](fn.interval.html "fn tokio::time::interval") is a
    stream yielding a value at a fixed period. It is initialized with a
    [`Duration`](https://doc.rust-lang.org/1.93.1/core/time/struct.Duration.html "struct core::time::Duration")
    and repeatedly yields each time the duration elapses.

-   [`Timeout`](struct.Timeout.html "struct tokio::time::Timeout"):
    Wraps a future or stream, setting an upper bound to the amount of
    time it is allowed to execute. If the future or stream does not
    complete in time, then it is canceled and an error is returned.

These types are sufficient for handling a large number of scenarios
involving time.

These types must be used from within the context of the
[`Runtime`](../runtime/struct.Runtime.html "struct tokio::runtime::Runtime").

# <a href="#examples" class="doc-anchor">§</a>Examples

Wait 100ms and print “100 ms have elapsed”

``` {.rust .rust-example-rendered}
use std::time::Duration;
use tokio::time::sleep;

sleep(Duration::from_millis(100)).await;
println!("100 ms have elapsed");
```

Require that an operation takes no more than 1s.

``` {.rust .rust-example-rendered}
use tokio::time::{timeout, Duration};

async fn long_future() {
    // do work here
}

let res = timeout(Duration::from_secs(1), long_future()).await;

if res.is_err() {
    println!("operation timed out");
}
```

A simple example using
[`interval`](fn.interval.html "fn tokio::time::interval") to execute a
task every two seconds.

The difference between
[`interval`](fn.interval.html "fn tokio::time::interval") and
[`sleep`](fn.sleep.html "fn tokio::time::sleep") is that an
[`interval`](fn.interval.html "fn tokio::time::interval") measures the
time since the last tick, which means that `.tick().await` may wait for
a shorter time than the duration specified for the interval if some time
has passed between calls to `.tick().await`.

If the tick in the example below was replaced with
[`sleep`](fn.sleep.html "fn tokio::time::sleep"), the task would only be
executed once every three seconds, and not every two seconds.

``` {.rust .rust-example-rendered}
use tokio::time;

async fn task_that_takes_a_second() {
    println!("hello");
    time::sleep(time::Duration::from_secs(1)).await
}

let mut interval = time::interval(time::Duration::from_secs(2));
for _i in 0..5 {
    interval.tick().await;
    task_that_takes_a_second().await;
}
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use std::time::`<a
href="https://doc.rust-lang.org/1.93.1/core/time/struct.Duration.html"
class="struct"
title="struct core::time::Duration"><code>Duration</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="error/index.html" class="mod"
title="mod tokio::time::error">error</a>  
Time error types.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Instant.html" class="struct"
title="struct tokio::time::Instant">Instant</a>  
A measurement of a monotonically nondecreasing clock. Opaque and useful
only with `Duration`.

<a href="struct.Interval.html" class="struct"
title="struct tokio::time::Interval">Interval</a>  
Interval returned by
[`interval`](fn.interval.html "fn tokio::time::interval") and
[`interval_at`](fn.interval_at.html "fn tokio::time::interval_at").

<a href="struct.Sleep.html" class="struct"
title="struct tokio::time::Sleep">Sleep</a>  
Future returned by [`sleep`](fn.sleep.html "fn tokio::time::sleep") and
[`sleep_until`](fn.sleep_until.html "fn tokio::time::sleep_until").

<a href="struct.Timeout.html" class="struct"
title="struct tokio::time::Timeout">Timeout</a>  
Future returned by
[`timeout`](fn.timeout.html "fn tokio::time::timeout") and
[`timeout_at`](fn.timeout_at.html "fn tokio::time::timeout_at").

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.MissedTickBehavior.html" class="enum"
title="enum tokio::time::MissedTickBehavior">MissedTickBehavior</a>  
Defines the behavior of an
[`Interval`](struct.Interval.html "struct tokio::time::Interval") when
it misses a tick.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.advance.html" class="fn"
title="fn tokio::time::advance">advance</a>  
Advances time.

<a href="fn.interval.html" class="fn"
title="fn tokio::time::interval">interval</a>  
Creates new
[`Interval`](struct.Interval.html "struct tokio::time::Interval") that
yields with interval of `period`. The first tick completes immediately.
The default
[`MissedTickBehavior`](enum.MissedTickBehavior.html "enum tokio::time::MissedTickBehavior")
is
[`Burst`](enum.MissedTickBehavior.html#variant.Burst "variant tokio::time::MissedTickBehavior::Burst"),
but this can be configured by calling
[`set_missed_tick_behavior`](struct.Interval.html#method.set_missed_tick_behavior "method tokio::time::Interval::set_missed_tick_behavior").

<a href="fn.interval_at.html" class="fn"
title="fn tokio::time::interval_at">interval_at</a>  
Creates new
[`Interval`](struct.Interval.html "struct tokio::time::Interval") that
yields with interval of `period` with the first tick completing at
`start`. The default
[`MissedTickBehavior`](enum.MissedTickBehavior.html "enum tokio::time::MissedTickBehavior")
is
[`Burst`](enum.MissedTickBehavior.html#variant.Burst "variant tokio::time::MissedTickBehavior::Burst"),
but this can be configured by calling
[`set_missed_tick_behavior`](struct.Interval.html#method.set_missed_tick_behavior "method tokio::time::Interval::set_missed_tick_behavior").

<a href="fn.pause.html" class="fn"
title="fn tokio::time::pause">pause</a>  
Pauses time.

<a href="fn.resume.html" class="fn"
title="fn tokio::time::resume">resume</a>  
Resumes time.

<a href="fn.sleep.html" class="fn"
title="fn tokio::time::sleep">sleep</a>  
Waits until `duration` has elapsed.

<a href="fn.sleep_until.html" class="fn"
title="fn tokio::time::sleep_until">sleep_until</a>  
Waits until `deadline` is reached.

<a href="fn.timeout.html" class="fn"
title="fn tokio::time::timeout">timeout</a>  
Requires a `Future` to complete before the specified duration has
elapsed.

<a href="fn.timeout_at.html" class="fn"
title="fn tokio::time::timeout_at">timeout_at</a>  
Requires a `Future` to complete before the specified instant in time.
