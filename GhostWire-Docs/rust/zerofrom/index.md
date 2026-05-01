Crate zerofrom Copy item path

<span class="sub-heading"><a href="../src/zerofrom/lib.rs.html#5-37" class="src">Source</a>
</span>

Expand description

This crate provides
[`ZeroFrom`](trait.ZeroFrom.html "trait zerofrom::ZeroFrom"), a trait
for converting types in a zero-copy way.

See the documentation of
[`ZeroFrom`](trait.ZeroFrom.html "trait zerofrom::ZeroFrom") for more
details.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ZeroFrom.html" class="trait"
title="trait zerofrom::ZeroFrom">ZeroFrom</a>  
Trait for types that can be created from a reference to a different type
`C` with no allocations, i.e. a zero-copy (zero-alloc) version of “From”

# Derive Macros<a href="#derives" class="anchor">§</a>

<a href="derive.ZeroFrom.html" class="derive"
title="derive zerofrom::ZeroFrom">ZeroFrom</a>  
Custom derive for `zerofrom::ZeroFrom`,
