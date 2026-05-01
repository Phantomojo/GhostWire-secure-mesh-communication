Crate quanta Copy item path

<span class="sub-heading"><a href="../src/quanta/lib.rs.html#1-727" class="src">Source</a>
</span>

Expand description

Performant cross-platform timing with goodies.

`quanta` provides a simple and fast API for measuring the current time
and the duration between events. It does this by providing a thin layer
on top of native OS timing functions, or, if available, using the Time
Stamp Counter feature found on modern CPUs.

# <a href="#design" class="doc-anchor">§</a>Design

Internally, `quanta` maintains the concept of two potential clock
sources: a reference clock and a source clock.

The reference clock is provided by the OS, and always available. It is
equivalent to what is provided by the standard library in terms of the
underlying system calls being made. As it uses the native timing
facilities provided by the operating system, we ultimately depend on the
OS itself to give us a stable and correct value.

The source clock is a potential clock source based on the [Time Stamp
Counter](https://en.wikipedia.org/wiki/Time_Stamp_Counter) feature found
on modern CPUs. If the TSC feature is not present or is not reliable
enough, `quanta` will transparently utilize the reference clock instead.

Depending on the underlying processor(s) in the system, `quanta` will
figure out the most accurate/efficient way to calibrate the source clock
to the reference clock in order to provide measurements scaled to wall
clock time.

Details on TSC support, and calibration, are detailed below.

# <a href="#features" class="doc-anchor">§</a>Features

Beyond simply taking measurements of the current time, `quanta` provides
features for more easily working with clocks, as well as being able to
enhance performance further:

-   `Clock` can be mocked for testing
-   globally accessible “recent” time with amortized overhead

## <a href="#mocked-time" class="doc-anchor">§</a>Mocked time

For any code that uses a `Clock`, a mocked version can be substituted.
This allows for application authors to control the time in tests, which
allows simulating not only the normal passage of time but provides the
ability to warp time forwards and backwards in order to test corner
cases in logic, etc. Creating a mocked clock can be acheived with
[`Clock::mock`](struct.Clock.html#method.mock "associated function quanta::Clock::mock"),
and [`Mock`](struct.Mock.html "struct quanta::Mock") contains more
details on mock usage.

## <a href="#coarsely-updated-or-recent-time" class="doc-anchor">§</a>Coarsely-updated, or recent, time

`quanta` also provides a “recent” time feature, which allows a
slightly-delayed version of time to be provided to callers, trading
accuracy for speed of access. An upkeep thread is spawned, which is
responsible for taking measurements and updating the global recent time.
Callers then can access the cached value by calling `Clock::recent`.
This interface can be 4-10x faster than directly calling `Clock::now`,
even when TSC support is available. As the upkeep thread is the only
code updating the recent time, the accuracy of the value given to
callers is limited by how often the upkeep thread updates the time, thus
the trade off between accuracy and speed of access.

# <a href="#feature-flags" class="doc-anchor">§</a>Feature Flags

`quanta` comes with feature flags that enable convenient conversions to
time types in other popular crates, such as:

-   `prost` - provides an implementation into
    [`Timestamp`](https://docs.rs/prost-types/0.7.0/prost_types/struct.Timestamp.html)
    from `prost_types`

# <a href="#platform-support" class="doc-anchor">§</a>Platform Support

At a high level, `quanta` carries support for most major operating
systems out of the box:

-   Windows
    ([`QueryPerformanceCounter`](https://msdn.microsoft.com/en-us/library/ms644904(v=VS.85).aspx))
-   macOS/OS X/iOS
    ([`mach_absolute_time`](https://developer.apple.com/documentation/kernel/1462446-mach_absolute_time))
-   Linux/\*BSD/Solaris
    ([`clock_gettime`](https://linux.die.net/man/3/clock_gettime))

These platforms are supported in the “reference” clock sense, and
support for using the Time Stamp Counter as a clocksource is more
subtle, and explained below.

## <a href="#wasm-support" class="doc-anchor">§</a>WASM support

This library can be built for WASM targets, but in this case the
resolution and accuracy of measurements can be limited by the WASM
environment. In particular, when running on the `wasm32-unknown-unknown`
target in browsers, `quanta` will use
[window.performance.now](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now)
as a clock. This mean the accuracy is limited to milliseconds instead of
the usual nanoseconds on other targets. When running within a WASI
environment (target `wasm32-wasi`), the accuracy of the clock depends on
the VM implementation.

# <a href="#tsc-support" class="doc-anchor">§</a>TSC Support

Accessing the TSC requires being on the `x86_64` architecture, with
access to SSE2. Additionally, the processor must support either constant
or nonstop/invariant TSC. This ensures that the TSC ticks at a constant
rate which can be easily scaled.

A caveat is that “constant” TSC doesn’t account for all possible power
states (levels of power down or sleep that a CPU can enter to save power
under light load, etc) and so a constant TSC can lead to drift in
measurements over time, after they’ve been scaled to reference time.

This is a limitation of the TSC mode, as well as the nature of `quanta`
not being able to know, as the OS would, when a power state transition
has happened, and thus compensate with a recalibration.
Nonstop/invariant TSC does not have this limitation and is stable over
long periods of time.

Roughly speaking, the following list contains the beginning
model/generation of processors where you should be able to expect having
invariant TSC support:

-   Intel Nehalem and newer for server-grade
-   Intel Skylake and newer for desktop-grade
-   VIA Centaur Nano and newer (circumstantial evidence here)
-   AMD Phenom and newer

Ultimately, `quanta` will query CPUID information to determine if the
processor has the required features to use the TSC.

# <a href="#calibration" class="doc-anchor">§</a>Calibration

As the TSC doesn’t necessarily tick at reference scale – i.e. one tick
isn’t always one nanosecond – we have to apply a scaling factor when
converting from source to reference time scale to provide this. We
acquire this scaling factor by repeatedly taking measurements from both
the reference and source clocks, until we have a statistically-relevant
measure of the average scaling factor. We do some additional work to
convert this scaling factor into a power-of-two number that allows us to
optimize the code, and thus reduce the generated instructions required
to scale a TSC value.

This calibration is stored globally and reused. However, the first
`Clock` that is created in an application will block for a small period
of time as it runs this calibration loop. The time spent in the
calibration loop is limited to 200ms overall. In practice, `quanta` will
reach a stable calibration quickly (usually 10-20ms, if not less) and so
this deadline is unlikely to be reached.

# <a href="#caveats" class="doc-anchor">§</a>Caveats

Utilizing the TSC can be a tricky affair, and so here is a list of
caveats that may or may not apply, and is in no way exhaustive:

-   CPU hotplug behavior is undefined
-   raw values may time warp
-   measurements from the TSC may drift past or behind the comparable
    reference clock

Another important caveat is that `quanta` does not track time across
system suspends. Simply put, if a time measurement (such as using
[`Instant::now`](struct.Instant.html#method.now "associated function quanta::Instant::now"))
is taken, and then the system is suspended, and then another measurement
is taken, the difference between those the two would not include the
time the system was in suspend.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Clock.html" class="struct"
title="struct quanta::Clock">Clock</a>  
Unified clock for taking measurements.

<a href="struct.Handle.html" class="struct"
title="struct quanta::Handle">Handle</a>  
Handle to a running upkeep thread.

<a href="struct.Instant.html" class="struct"
title="struct quanta::Instant">Instant</a>  
A point-in-time wall-clock measurement.

<a href="struct.Mock.html" class="struct"
title="struct quanta::Mock">Mock</a>  
Controllable time source for use in tests.

<a href="struct.Upkeep.html" class="struct"
title="struct quanta::Upkeep">Upkeep</a>  
Ultra-low-overhead access to slightly-delayed time.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum quanta::Error">Error</a>  
Errors thrown during the creation/spawning of the upkeep thread.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.IntoNanoseconds.html" class="trait"
title="trait quanta::IntoNanoseconds">IntoNanoseconds</a>  
Type which can be converted into a nanosecond representation.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.set_recent.html" class="fn"
title="fn quanta::set_recent">set_recent</a>  
Sets the global recent time.

<a href="fn.with_clock.html" class="fn"
title="fn quanta::with_clock">with_clock</a>  
Sets this clock as the default for the duration of a closure.
