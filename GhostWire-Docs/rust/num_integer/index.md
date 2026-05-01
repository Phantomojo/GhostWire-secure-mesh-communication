Crate num\_integer Copy item path

<span class="sub-heading"><a href="../src/num_integer/lib.rs.html#11-1386" class="src">Source</a>
</span>

Expand description

Integer trait and functions.

## <a href="#compatibility" class="doc-anchor">§</a>Compatibility

The `num-integer` crate is tested for rustc 1.31 and greater.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ExtendedGcd.html" class="struct"
title="struct num_integer::ExtendedGcd">ExtendedGcd</a>  
Greatest common divisor and Bézout coefficients

<a href="struct.IterBinomial.html" class="struct"
title="struct num_integer::IterBinomial">IterBinomial</a>  
An iterator over binomial coefficients.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Average.html" class="trait"
title="trait num_integer::Average">Average</a>  
Provides methods to compute the average of two integers, without
overflows.

<a href="trait.Integer.html" class="trait"
title="trait num_integer::Integer">Integer</a>  
<a href="trait.Roots.html" class="trait"
title="trait num_integer::Roots">Roots</a>  
Provides methods to compute an integer’s square root, cube root, and
arbitrary `n`th root.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.average_ceil.html" class="fn"
title="fn num_integer::average_ceil">average_ceil</a>  
Returns the ceiling value of the average of `x` and `y` – see
[Average::average\_ceil](trait.Average.html#tymethod.average_ceil).

<a href="fn.average_floor.html" class="fn"
title="fn num_integer::average_floor">average_floor</a>  
Returns the floor value of the average of `x` and `y` – see
[Average::average\_floor](trait.Average.html#tymethod.average_floor).

<a href="fn.binomial.html" class="fn"
title="fn num_integer::binomial">binomial</a>  
Calculate the binomial coefficient.

<a href="fn.cbrt.html" class="fn" title="fn num_integer::cbrt">cbrt</a>  
Returns the truncated principal cube root of an integer – see
[Roots::cbrt](trait.Roots.html#method.cbrt).

<a href="fn.div_ceil.html" class="fn"
title="fn num_integer::div_ceil">div_ceil</a>  
Ceiled integer division

<a href="fn.div_floor.html" class="fn"
title="fn num_integer::div_floor">div_floor</a>  
Floored integer division

<a href="fn.div_mod_floor.html" class="fn"
title="fn num_integer::div_mod_floor">div_mod_floor</a>  
Simultaneous floored integer division and modulus

<a href="fn.div_rem.html" class="fn"
title="fn num_integer::div_rem">div_rem</a>  
Simultaneous integer division and modulus

<a href="fn.gcd.html" class="fn" title="fn num_integer::gcd">gcd</a>  
Calculates the Greatest Common Divisor (GCD) of the number and `other`.
The result is always non-negative.

<a href="fn.gcd_lcm.html" class="fn"
title="fn num_integer::gcd_lcm">gcd_lcm</a>  
Calculates the Greatest Common Divisor (GCD) and Lowest Common Multiple
(LCM) of the number and `other`.

<a href="fn.lcm.html" class="fn" title="fn num_integer::lcm">lcm</a>  
Calculates the Lowest Common Multiple (LCM) of the number and `other`.

<a href="fn.mod_floor.html" class="fn"
title="fn num_integer::mod_floor">mod_floor</a>  
Floored integer modulus

<a href="fn.multinomial.html" class="fn"
title="fn num_integer::multinomial">multinomial</a>  
Calculate the multinomial coefficient.

<a href="fn.nth_root.html" class="fn"
title="fn num_integer::nth_root">nth_root</a>  
Returns the truncated principal `n`th root of an integer – see
[Roots::nth\_root](trait.Roots.html#tymethod.nth_root).

<a href="fn.sqrt.html" class="fn" title="fn num_integer::sqrt">sqrt</a>  
Returns the truncated principal square root of an integer – see
[Roots::sqrt](trait.Roots.html#method.sqrt).
