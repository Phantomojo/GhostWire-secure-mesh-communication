Crate ff Copy item path

<span class="sub-heading"><a href="../src/ff/lib.rs.html#1-498" class="src">Source</a>
</span>

Expand description

This crate provides traits for working with finite fields.

# Modules<a href="#modules" class="anchor">§</a>

<a href="helpers/index.html" class="mod"
title="mod ff::helpers">helpers</a>  
Helper methods for implementing the `ff` traits.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BatchInverter.html" class="struct"
title="struct ff::BatchInverter">BatchInverter</a>  
A non-allocating batch inverter.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.BatchInvert.html" class="trait"
title="trait ff::BatchInvert">BatchInvert</a>  
Extension trait for iterators over mutable field elements which allows
those field elements to be inverted in a batch.

<a href="trait.Field.html" class="trait"
title="trait ff::Field">Field</a>  
This trait represents an element of a field.

<a href="trait.FromUniformBytes.html" class="trait"
title="trait ff::FromUniformBytes">FromUniformBytes</a>  
Trait for constructing a
[`PrimeField`](trait.PrimeField.html "trait ff::PrimeField") element
from a fixed-length uniform byte array.

<a href="trait.PrimeField.html" class="trait"
title="trait ff::PrimeField">PrimeField</a>  
This represents an element of a non-binary prime field.

<a href="trait.WithSmallOrderMulGroup.html" class="trait"
title="trait ff::WithSmallOrderMulGroup">WithSmallOrderMulGroup</a>  
The subset of prime-order fields such that `(modulus - 1)` is divisible
by `N`.
