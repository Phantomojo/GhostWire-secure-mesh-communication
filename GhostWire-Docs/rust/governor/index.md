Crate governor Copy item path

<span class="sub-heading"><a href="../src/governor/lib.rs.html#1-85" class="src">Source</a>
</span>

Expand description

# <a href="#governor---a-rate-limiting-library-for-rust"
class="doc-anchor">§</a>governor - a rate-limiting library for rust.

Governor aims to be a very efficient and ergonomic way to enforce rate
limits in Rust programs. It implements the [Generic Cell Rate
Algorithm](https://en.wikipedia.org/wiki/Generic_cell_rate_algorithm)
and keeps state in a very efficient way.

For detailed information on usage, please see the [user’s
guide](_guide/index.html "mod governor::_guide").

# <a href="#quick-example" class="doc-anchor">§</a>Quick example

In this example, we set up a rate limiter to allow 50 elements per
second, and check that a single element can pass through.

``` {.rust .rust-example-rendered}
use std::num::NonZeroU32;
use nonzero_ext::*;
use governor::{Quota, RateLimiter};

let mut lim = RateLimiter::direct(Quota::per_second(nonzero!(50u32))); // Allow 50 units per second
assert_eq!(Ok(()), lim.check());
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use state::direct::`<a href="state/direct/struct.RatelimitedSink.html" class="struct"
title="struct governor::state::direct::RatelimitedSink"><code>RatelimitedSink</code></a>`;`

`pub use state::direct::`<a href="state/direct/struct.RatelimitedStream.html" class="struct"
title="struct governor::state::direct::RatelimitedStream"><code>RatelimitedStream</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="_guide/index.html" class="mod"
title="mod governor::_guide">_guide</a>  
A more in-depth guide to `governor`

<a href="clock/index.html" class="mod"
title="mod governor::clock">clock</a>  
Time sources for rate limiters.

<a href="middleware/index.html" class="mod"
title="mod governor::middleware">middleware</a>  
Additional, customizable behavior for rate limiters.

<a href="nanos/index.html" class="mod"
title="mod governor::nanos">nanos</a>  
A time-keeping abstraction (nanoseconds) that works for storing in an
atomic integer.

<a href="prelude/index.html" class="mod"
title="mod governor::prelude">prelude</a>  
The collection of asynchronous traits exported from this crate.

<a href="state/index.html" class="mod"
title="mod governor::state">state</a>  
State stores for rate limiters

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.InsufficientCapacity.html" class="struct"
title="struct governor::InsufficientCapacity">InsufficientCapacity</a>  
Error indicating that the number of cells tested (the first argument) is
larger than the bucket’s capacity.

<a href="struct.Jitter.html" class="struct"
title="struct governor::Jitter">Jitter</a>  
An interval specification for deviating from the nominal wait time.

<a href="struct.NotUntil.html" class="struct"
title="struct governor::NotUntil">NotUntil</a>  
A negative rate-limiting outcome.

<a href="struct.Quota.html" class="struct"
title="struct governor::Quota">Quota</a>  
A rate-limiting quota.

<a href="struct.RateLimiter.html" class="struct"
title="struct governor::RateLimiter">RateLimiter</a>  
A rate limiter.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.DefaultDirectRateLimiter.html" class="type"
title="type governor::DefaultDirectRateLimiter">DefaultDirectRateLimiter</a>  
A rate limiter representing a single item of state in memory, running on
the default clock.

<a href="type.DefaultKeyedRateLimiter.html" class="type"
title="type governor::DefaultKeyedRateLimiter">DefaultKeyedRateLimiter</a>  
A rate limiter with one state per key, running on the default clock.
