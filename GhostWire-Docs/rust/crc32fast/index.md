Crate crc32fast Copy item path

<span class="sub-heading"><a href="../src/crc32fast/lib.rs.html#1-214" class="src">Source</a>
</span>

Expand description

Fast, SIMD-accelerated CRC32 (IEEE) checksum computation.

## <a href="#usage" class="doc-anchor">§</a>Usage

### <a href="#simple-usage" class="doc-anchor">§</a>Simple usage

For simple use-cases, you can call the
[`hash()`](fn.hash.html "fn crc32fast::hash") convenience function to
directly compute the CRC32 checksum for a given byte slice:

``` {.rust .rust-example-rendered}
let checksum = crc32fast::hash(b"foo bar baz");
```

### <a href="#advanced-usage" class="doc-anchor">§</a>Advanced usage

For use-cases that require more flexibility or performance, for example
when processing large amounts of data, you can create and manipulate a
[`Hasher`](struct.Hasher.html "struct crc32fast::Hasher"):

``` {.rust .rust-example-rendered}
use crc32fast::Hasher;

let mut hasher = Hasher::new();
hasher.update(b"foo bar baz");
let checksum = hasher.finalize();
```

## <a href="#performance" class="doc-anchor">§</a>Performance

This crate contains multiple CRC32 implementations:

-   A fast baseline implementation which processes up to 16 bytes per
    iteration
-   An optimized implementation for modern `x86` using `sse` and
    `pclmulqdq` instructions

Calling the
[`Hasher::new`](struct.Hasher.html#method.new "associated function crc32fast::Hasher::new")
constructor at runtime will perform a feature detection to select the
most optimal implementation for the current CPU feature set.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Hasher.html" class="struct"
title="struct crc32fast::Hasher">Hasher</a>  
Represents an in-progress CRC32 computation.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.hash.html" class="fn" title="fn crc32fast::hash">hash</a>  
Computes the CRC32 hash of a byte slice.
