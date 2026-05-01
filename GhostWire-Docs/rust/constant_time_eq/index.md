Crate constant\_time\_eq Copy item path

<span class="sub-heading"><a href="../src/constant_time_eq/lib.rs.html#1-155"
class="src">Source</a> </span>

Expand description

Compares two equal-sized byte strings in constant time.

The time of the comparison does not depend on:

-   The contents of the inputs;
-   The position of the difference(s) between the inputs.

The time of the comparison can depend on:

-   The memory addresses of the inputs;
-   The length of the inputs.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.constant_time_eq.html" class="fn"
title="fn constant_time_eq::constant_time_eq">constant_time_eq</a>  
Compares two equal-sized byte strings in constant time.

<a href="fn.constant_time_eq_16.html" class="fn"
title="fn constant_time_eq::constant_time_eq_16">constant_time_eq_16</a>  
Compares two 128-bit byte strings in constant time.

<a href="fn.constant_time_eq_32.html" class="fn"
title="fn constant_time_eq::constant_time_eq_32">constant_time_eq_32</a>  
Compares two 256-bit byte strings in constant time.

<a href="fn.constant_time_eq_64.html" class="fn"
title="fn constant_time_eq::constant_time_eq_64">constant_time_eq_64</a>  
Compares two 512-bit byte strings in constant time.

<a href="fn.constant_time_eq_n.html" class="fn"
title="fn constant_time_eq::constant_time_eq_n">constant_time_eq_n</a>  
Compares two fixed-size byte strings in constant time.
