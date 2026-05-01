Crate adler2 Copy item path

<span class="sub-heading"><a href="../src/adler2/lib.rs.html#1-287" class="src">Source</a>
</span>

Expand description

Adler-32 checksum implementation.

This implementation features:

-   Permissively licensed (0BSD) clean-room implementation.
-   Zero dependencies.
-   Zero `unsafe`.
-   Decent performance (3-4 GB/s).
-   `#![no_std]` support (with `default-features = false`).

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Adler32.html" class="struct"
title="struct adler2::Adler32">Adler32</a>  
Adler-32 checksum calculator.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.adler32_slice.html" class="fn"
title="fn adler2::adler32_slice">adler32_slice</a>  
Calculates the Adler-32 checksum of a byte slice.
