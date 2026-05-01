Crate web\_time Copy item path

<span class="sub-heading"><a href="../src/web_time/lib.rs.html#1-144" class="src">Source</a>
</span>

Expand description

# <a href="#description" class="doc-anchor">§</a>Description

Complete drop-in replacement for
[`std::time`](https://doc.rust-lang.org/stable/std/time/) that works in
browsers.

Currently
[`Instant::now()`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.now)
and
[`SystemTime::now()`](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now)
will simply panic when using the `wasm32-unknown-unknown` target. This
implementation uses
[`Performance.now()`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now)
for [`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html)
and
[`Date.now()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
for
[`SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
to offer a drop-in replacement that works in browsers.

At the same time the library will simply re-export
[`std::time`](https://doc.rust-lang.org/stable/std/time/) when not using
the `wasm32-unknown-unknown` target and will not pull in any
dependencies.

Additionally, if compiled with `target-feature = "atomics"` it will
synchronize the timestamps to account for different context’s, like web
workers. See
[`Performance.timeOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin)
for more information.

Using `-Ctarget-feature=+nontrapping-fptoint` will improve the
performance of
[`Instant::now()`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.now)
and
[`SystemTime::now()`](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now),
but the vast majority of the time is still spent going through JS.

# <a href="#target" class="doc-anchor">§</a>Target

This library specifically targets browsers, that support
[`Performance.now()`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now),
with the `wasm32-unknown-unknown` target. Emscripten is not supported.
WASI doesn’t require support as it has it’s own native API to deal with
[`std::time`](https://doc.rust-lang.org/stable/std/time/).

Furthermore it depends on
[`wasm-bindgen`](https://crates.io/crates/wasm-bindgen), which is
required. This library will continue to depend on it until a viable
alternative presents itself, in which case multiple ecosystems could be
supported.

# <a href="#note" class="doc-anchor">§</a>Note

## <a href="#ticking-during-sleep" class="doc-anchor">§</a>Ticking during sleep

Currently a known bug is affecting browsers on operating system other
then Windows. This bug prevents
[`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html) from
continuing to tick when the context is asleep. This doesn’t necessarily
conflict with Rusts requirements of
[`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html), but
might still be unexpected.

See [the MDN documentation on
this](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now#ticking_during_sleep)
for more information.

## <a href="#context-support" class="doc-anchor">§</a>Context support

The implementation of
[`Instant::now()`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.now)
relies on the availability of the [`Performance`
object](https://developer.mozilla.org/en-US/docs/Web/API/performance_property),
a lack thereof will cause a panic. This can happen if called from a
[worklet](https://developer.mozilla.org/en-US/docs/Web/API/Worklet).

# <a href="#usage" class="doc-anchor">§</a>Usage

You can simply import the types you need:

``` {.rust .rust-example-rendered}
use web_time::{Instant, SystemTime};

let now = Instant::now();
let time = SystemTime::now();
```

# <a href="#features" class="doc-anchor">§</a>Features

## <a href="#serde" class="doc-anchor">§</a>`serde`

Implements
[`serde::Deserialize`](https://docs.rs/serde/1/serde/trait.Deserialize.html)
and
[`serde::Serialize`](https://docs.rs/serde/1/serde/trait.Serialize.html)
for
[`SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html).

# <a href="#msrv" class="doc-anchor">§</a>MSRV

As this library heavily relies on
[`wasm-bindgen`](https://crates.io/crates/wasm-bindgen) the MSRV depends
on it. At the point of time this was written the MSRV is 1.60.

# <a href="#alternatives" class="doc-anchor">§</a>Alternatives

[instant](https://crates.io/crates/instant)
[![Crates.io](https://img.shields.io/crates/v/instant.svg)](https://crates.io/crates/instant)
is a popular alternative! However the API it implements doesn’t match
[`std::time`](https://doc.rust-lang.org/stable/std/time/) exactly.

# <a href="#contributing" class="doc-anchor">§</a>Contributing

See the
[CONTRIBUTING](https://github.com/daxpedda/web-time/blob/v1.1.0/CONTRIBUTING.md)
file for details.

# <a href="#attribution" class="doc-anchor">§</a>Attribution

Inspiration was taken from the
[instant](https://github.com/sebcrozet/instant/tree/v0.1.12) project.

Additional insight was taken from the
[time](https://github.com/time-rs/time/tree/v0.3.20) project.

# <a href="#changelog" class="doc-anchor">§</a>Changelog

See the
[CHANGELOG](https://github.com/daxpedda/web-time/blob/v1.1.0/CHANGELOG.md)
file for details.

# <a href="#license" class="doc-anchor">§</a>License

Licensed under either of

-   Apache License, Version 2.0
    ([LICENSE-APACHE](https://github.com/daxpedda/web-time/blob/v1.1.0/LICENSE-APACHE)
    or <http://www.apache.org/licenses/LICENSE-2.0>)
-   MIT license
    ([LICENSE-MIT](https://github.com/daxpedda/web-time/blob/v1.1.0/LICENSE-MIT)
    or <http://opensource.org/licenses/MIT>)

at your option.

## <a href="#copyright" class="doc-anchor">§</a>Copyright

A majority of the code and documentation was taken from
[`std::time`](https://doc.rust-lang.org/stable/std/time/). For license
information see
[\#License](https://github.com/rust-lang/rust/tree/1.68.1#license).

## <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Duration.html" class="struct"
title="struct web_time::Duration">Duration</a>  
A `Duration` type to represent a span of time, typically used for system
timeouts.

<a href="struct.Instant.html" class="struct"
title="struct web_time::Instant">Instant</a>  
A measurement of a monotonically nondecreasing clock. Opaque and useful
only with
[`Duration`](struct.Duration.html "struct web_time::Duration").

<a href="struct.SystemTime.html" class="struct"
title="struct web_time::SystemTime">SystemTime</a>  
A measurement of the system clock, useful for talking to external
entities like the file system or other processes.

<a href="struct.SystemTimeError.html" class="struct"
title="struct web_time::SystemTimeError">SystemTimeError</a>  
An error returned from the `duration_since` and `elapsed` methods on
`SystemTime`, used to learn how far in the opposite direction a system
time lies.

<a href="struct.TryFromFloatSecsError.html" class="struct"
title="struct web_time::TryFromFloatSecsError">TryFromFloatSecsError</a>  
An error which can be returned when converting a floating-point value of
seconds into a
[`Duration`](struct.Duration.html "struct web_time::Duration").

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.UNIX_EPOCH.html" class="constant"
title="constant web_time::UNIX_EPOCH">UNIX_EPOCH</a>  
An anchor in time which can be used to create new `SystemTime` instances
or learn about where in time a `SystemTime` lies.
