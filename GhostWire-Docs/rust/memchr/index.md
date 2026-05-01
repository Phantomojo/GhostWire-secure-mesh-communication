Crate memchr Copy item path

<span class="sub-heading"><a href="../src/memchr/lib.rs.html#1-221" class="src">Source</a>
</span>

Expand description

This library provides heavily optimized routines for string search
primitives.

# <a href="#overview" class="doc-anchor">§</a>Overview

This section gives a brief high level overview of what this crate
offers.

-   The top-level module provides routines for searching for 1, 2 or 3
    bytes in the forward or reverse direction. When searching for more
    than one byte, positions are considered a match if the byte at that
    position matches any of the bytes.
-   The [`memmem`](memmem/index.html "mod memchr::memmem") sub-module
    provides forward and reverse substring search routines.

In all such cases, routines operate on `&[u8]` without regard to
encoding. This is exactly what you want when searching either UTF-8 or
arbitrary bytes.

# <a href="#example-using-memchr" class="doc-anchor">§</a>Example: using `memchr`

This example shows how to use `memchr` to find the first occurrence of
`z` in a haystack:

``` {.rust .rust-example-rendered}
use memchr::memchr;

let haystack = b"foo bar baz quuz";
assert_eq!(Some(10), memchr(b'z', haystack));
```

# <a href="#example-matching-one-of-three-possible-bytes"
class="doc-anchor">§</a>Example: matching one of three possible bytes

This examples shows how to use `memrchr3` to find occurrences of `a`,
`b` or `c`, starting at the end of the haystack.

``` {.rust .rust-example-rendered}
use memchr::memchr3_iter;

let haystack = b"xyzaxyzbxyzc";

let mut it = memchr3_iter(b'a', b'b', b'c', haystack).rev();
assert_eq!(Some(11), it.next());
assert_eq!(Some(7), it.next());
assert_eq!(Some(3), it.next());
assert_eq!(None, it.next());
```

# <a href="#example-iterating-over-substring-matches"
class="doc-anchor">§</a>Example: iterating over substring matches

This example shows how to use the
[`memmem`](memmem/index.html "mod memchr::memmem") sub-module to find
occurrences of a substring in a haystack.

``` {.rust .rust-example-rendered}
use memchr::memmem;

let haystack = b"foo bar foo baz foo";

let mut it = memmem::find_iter(haystack, "foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

# <a href="#example-repeating-a-search-for-the-same-needle"
class="doc-anchor">§</a>Example: repeating a search for the same needle

It may be possible for the overhead of constructing a substring searcher
to be measurable in some workloads. In cases where the same needle is
used to search many haystacks, it is possible to do construction once
and thus to avoid it for subsequent searches. This can be done with a
[`memmem::Finder`](memmem/struct.Finder.html "struct memchr::memmem::Finder"):

``` {.rust .rust-example-rendered}
use memchr::memmem;

let finder = memmem::Finder::new("foo");

assert_eq!(Some(4), finder.find(b"baz foo quux"));
assert_eq!(None, finder.find(b"quux baz bar"));
```

# <a href="#why-use-this-crate" class="doc-anchor">§</a>Why use this crate?

At first glance, the APIs provided by this crate might seem weird. Why
provide a dedicated routine like `memchr` for something that could be
implemented clearly and trivially in one line:

``` {.rust .rust-example-rendered}
fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == needle)
}
```

Or similarly, why does this crate provide substring search routines when
Rust’s core library already provides them?

``` {.rust .rust-example-rendered}
fn search(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
}
```

The primary reason for both of them to exist is performance. When it
comes to performance, at a high level at least, there are two primary
ways to look at it:

-   **Throughput**: For this, think about it as, “given some very large
    haystack and a byte that never occurs in that haystack, how long
    does it take to search through it and determine that it, in fact,
    does not occur?”
-   **Latency**: For this, think about it as, “given a tiny
    haystack—just a few bytes—how long does it take to determine if a
    byte is in it?”

The `memchr` routine in this crate has *slightly* worse latency than the
solution presented above, however, its throughput can easily be over an
order of magnitude faster. This is a good general purpose trade off to
make. You rarely lose, but often gain big.

**NOTE:** The name `memchr` comes from the corresponding routine in
`libc`. A key advantage of using this library is that its performance is
not tied to its quality of implementation in the `libc` you happen to be
using, which can vary greatly from platform to platform.

But what about substring search? This one is a bit more complicated. The
primary reason for its existence is still indeed performance, but it’s
also useful because Rust’s core library doesn’t actually expose any
substring search routine on arbitrary bytes. The only substring search
routine that exists works exclusively on valid UTF-8.

So if you have valid UTF-8, is there a reason to use this over the
standard library substring search routine? Yes. This routine is faster
on almost every metric, including latency. The natural question then, is
why isn’t this implementation in the standard library, even if only for
searching on UTF-8? The reason is that the implementation details for
using SIMD in the standard library haven’t quite been worked out yet.

**NOTE:** Currently, only `x86_64`, `wasm32` and `aarch64` targets have
vector accelerated implementations of `memchr` (and friends) and
`memmem`.

# <a href="#crate-features" class="doc-anchor">§</a>Crate features

-   **std** - When enabled (the default), this will permit features
    specific to the standard library. Currently, the only thing used
    from the standard library is runtime SIMD CPU feature detection.
    This means that this feature must be enabled to get AVX2 accelerated
    routines on `x86_64` targets without enabling the `avx2` feature at
    compile time, for example. When `std` is not enabled, this crate
    will still attempt to use SSE2 accelerated routines on `x86_64`. It
    will also use AVX2 accelerated routines when the `avx2` feature is
    enabled at compile time. In general, enable this feature if you can.
-   **alloc** - When enabled (the default), APIs in this crate requiring
    some kind of allocation will become available. For example, the
    [`memmem::Finder::into_owned`](memmem/struct.Finder.html#method.into_owned "method memchr::memmem::Finder::into_owned")
    API and the
    [`arch::all::shiftor`](arch/all/shiftor/index.html "mod memchr::arch::all::shiftor")
    substring search implementation. Otherwise, this crate is designed
    from the ground up to be usable in core-only contexts, so the
    `alloc` feature doesn’t add much currently. Notably, disabling `std`
    but enabling `alloc` will **not** result in the use of AVX2 on
    `x86_64` targets unless the `avx2` feature is enabled at compile
    time. (With `std` enabled, AVX2 can be used even without the `avx2`
    feature enabled at compile time by way of runtime CPU feature
    detection.)
-   **logging** - When enabled (disabled by default), the `log` crate is
    used to emit log messages about what kinds of `memchr` and `memmem`
    algorithms are used. Namely, both `memchr` and `memmem` have a
    number of different implementation choices depending on the target
    and CPU, and the log messages can help show what specific
    implementations are being used. Generally, this is useful for
    debugging performance issues.
-   **libc** - **DEPRECATED**. Previously, this enabled the use of the
    target’s `memchr` function from whatever `libc` was linked into the
    program. This feature is now a no-op because this crate’s
    implementation of `memchr` should now be sufficiently fast on a
    number of platforms that `libc` should no longer be needed. (This
    feature is somewhat of a holdover from this crate’s origins.
    Originally, this crate was literally just a safe wrapper function
    around the `memchr` function from `libc`.)

# Modules<a href="#modules" class="anchor">§</a>

<a href="arch/index.html" class="mod" title="mod memchr::arch">arch</a>  
A module with low-level architecture dependent routines.

<a href="memmem/index.html" class="mod"
title="mod memchr::memmem">memmem</a>  
This module provides forward and reverse substring search routines.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Memchr.html" class="struct"
title="struct memchr::Memchr">Memchr</a>  
An iterator over all occurrences of a single byte in a haystack.

<a href="struct.Memchr2.html" class="struct"
title="struct memchr::Memchr2">Memchr2</a>  
An iterator over all occurrences of two possible bytes in a haystack.

<a href="struct.Memchr3.html" class="struct"
title="struct memchr::Memchr3">Memchr3</a>  
An iterator over all occurrences of three possible bytes in a haystack.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.memchr.html" class="fn" title="fn memchr::memchr">memchr</a>  
Search for the first occurrence of a byte in a slice.

<a href="fn.memchr2.html" class="fn"
title="fn memchr::memchr2">memchr2</a>  
Search for the first occurrence of two possible bytes in a haystack.

<a href="fn.memchr3.html" class="fn"
title="fn memchr::memchr3">memchr3</a>  
Search for the first occurrence of three possible bytes in a haystack.

<a href="fn.memchr2_iter.html" class="fn"
title="fn memchr::memchr2_iter">memchr2_iter</a>  
Returns an iterator over all occurrences of the needles in a haystack.

<a href="fn.memchr3_iter.html" class="fn"
title="fn memchr::memchr3_iter">memchr3_iter</a>  
Returns an iterator over all occurrences of the needles in a haystack.

<a href="fn.memchr_iter.html" class="fn"
title="fn memchr::memchr_iter">memchr_iter</a>  
Returns an iterator over all occurrences of the needle in a haystack.

<a href="fn.memrchr.html" class="fn"
title="fn memchr::memrchr">memrchr</a>  
Search for the last occurrence of a byte in a slice.

<a href="fn.memrchr2.html" class="fn"
title="fn memchr::memrchr2">memrchr2</a>  
Search for the last occurrence of two possible bytes in a haystack.

<a href="fn.memrchr3.html" class="fn"
title="fn memchr::memrchr3">memrchr3</a>  
Search for the last occurrence of three possible bytes in a haystack.

<a href="fn.memrchr2_iter.html" class="fn"
title="fn memchr::memrchr2_iter">memrchr2_iter</a>  
Returns an iterator over all occurrences of the needles in a haystack,
in reverse.

<a href="fn.memrchr3_iter.html" class="fn"
title="fn memchr::memrchr3_iter">memrchr3_iter</a>  
Returns an iterator over all occurrences of the needles in a haystack,
in reverse.

<a href="fn.memrchr_iter.html" class="fn"
title="fn memchr::memrchr_iter">memrchr_iter</a>  
Returns an iterator over all occurrences of the needle in a haystack, in
reverse.
