Crate num\_conv Copy item path

<span class="sub-heading"><a href="../src/num_conv/lib.rs.html#1-291" class="src">Source</a>
</span>

Expand description

`num_conv` is a crate to convert between integer types without using
`as` casts. This provides better certainty when refactoring, makes the
exact behavior of code more explicit, and allows using turbofish syntax.

# Modules<a href="#modules" class="anchor">§</a>

<a href="prelude/index.html" class="mod"
title="mod num_conv::prelude">prelude</a>  
Anonymously import all extension traits.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Extend.html" class="trait"
title="trait num_conv::Extend">Extend</a>  
Extend to an integer of the same size or larger, preserving its value.

<a href="trait.ExtendTarget.html" class="trait"
title="trait num_conv::ExtendTarget">ExtendTarget</a>  
A type that can be used with turbofish syntax in
[`Extend::extend`](trait.Extend.html#tymethod.extend "method num_conv::Extend::extend").

<a href="trait.Truncate.html" class="trait"
title="trait num_conv::Truncate">Truncate</a>  
Truncate to an integer of the same size or smaller.

<a href="trait.TruncateTarget.html" class="trait"
title="trait num_conv::TruncateTarget">TruncateTarget</a>  
A type that can be used with turbofish syntax in
[`Truncate::truncate`](trait.Truncate.html#tymethod.truncate "method num_conv::Truncate::truncate").
