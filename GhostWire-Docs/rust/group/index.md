Crate group Copy item path

<span class="sub-heading"><a href="../src/group/lib.rs.html#1-174" class="src">Source</a>
</span>

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../ff/index.html" class="mod"
title="mod ff"><code>ff</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="cofactor/index.html" class="mod"
title="mod group::cofactor">cofactor</a>

<a href="prime/index.html" class="mod"
title="mod group::prime">prime</a>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Wnaf.html" class="struct"
title="struct group::Wnaf">Wnaf</a>  
A “w-ary non-adjacent form” scalar multiplication (also known as
exponentiation) context.

<a href="struct.WnafBase.html" class="struct"
title="struct group::WnafBase">WnafBase</a>  
A fixed window table for a group element, precomputed to improve the
speed of scalar multiplication.

<a href="struct.WnafScalar.html" class="struct"
title="struct group::WnafScalar">WnafScalar</a>  
A “w-ary non-adjacent form” scalar, that uses precomputation to improve
the speed of scalar multiplication.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Curve.html" class="trait"
title="trait group::Curve">Curve</a>  
Efficient representation of an elliptic curve point guaranteed.

<a href="trait.Group.html" class="trait"
title="trait group::Group">Group</a>  
This trait represents an element of a cryptographic group.

<a href="trait.GroupEncoding.html" class="trait"
title="trait group::GroupEncoding">GroupEncoding</a>  
<a href="trait.GroupOps.html" class="trait"
title="trait group::GroupOps">GroupOps</a>  
A helper trait for types with a group operation.

<a href="trait.GroupOpsOwned.html" class="trait"
title="trait group::GroupOpsOwned">GroupOpsOwned</a>  
A helper trait for references with a group operation.

<a href="trait.ScalarMul.html" class="trait"
title="trait group::ScalarMul">ScalarMul</a>  
A helper trait for types implementing group scalar multiplication.

<a href="trait.ScalarMulOwned.html" class="trait"
title="trait group::ScalarMulOwned">ScalarMulOwned</a>  
A helper trait for references implementing group scalar multiplication.

<a href="trait.UncompressedEncoding.html" class="trait"
title="trait group::UncompressedEncoding">UncompressedEncoding</a>  
Affine representation of a point on an elliptic curve that has a defined
uncompressed encoding.

<a href="trait.WnafGroup.html" class="trait"
title="trait group::WnafGroup">WnafGroup</a>  
Extension trait on a [`Group`](trait.Group.html "trait group::Group")
that provides helpers used by
[`Wnaf`](struct.Wnaf.html "struct group::Wnaf").
