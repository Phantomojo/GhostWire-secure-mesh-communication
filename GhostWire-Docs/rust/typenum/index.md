Crate typenum Copy item path

<span class="sub-heading"><a href="../src/typenum/lib.rs.html#1-172" class="src">Source</a>
</span>

Expand description

This crate provides type-level numbers evaluated at compile time. It
depends only on libcore.

The traits defined or used in this crate are used in a typical manner.
They can be divided into two categories: **marker traits** and **type
operators**.

Many of the marker traits have functions defined, but they all do
essentially the same thing: convert a type into its runtime counterpart,
and are really just there for debugging. For example,

``` {.rust .rust-example-rendered}
use typenum::{Integer, N4};

assert_eq!(N4::to_i32(), -4);
```

**Type operators** are traits that behave as functions at the type
level. These are the meat of this library. Where possible, traits
defined in libcore have been used.

For example, the `Add` trait is implemented for both unsigned and signed
integers, where its associated type `Output` represents the sum of the
two integers.

``` {.rust .rust-example-rendered}
use std::ops::Add;
use typenum::{Integer, P3, P4};

type X = <P3 as Add<P4>>::Output;
assert_eq!(<X as Integer>::to_i32(), 7);
```

In addition, helper aliases are defined for type operators. For example,
the above snippet could be replaced with

``` {.rust .rust-example-rendered}
use typenum::{Integer, Sum, P3, P4};

type X = Sum<P3, P4>;
assert_eq!(<X as Integer>::to_i32(), 7);
```

Documented in each module is the full list of type operators
implemented.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::array::`<a href="array/struct.ATerm.html" class="struct"
title="struct typenum::array::ATerm"><code>ATerm</code></a>`;`

`pub use crate::array::`<a href="array/struct.TArr.html" class="struct"
title="struct typenum::array::TArr"><code>TArr</code></a>`;`

`pub use crate::int::`<a href="int/struct.NInt.html" class="struct"
title="struct typenum::int::NInt"><code>NInt</code></a>`;`

`pub use crate::int::`<a href="int/struct.PInt.html" class="struct"
title="struct typenum::int::PInt"><code>PInt</code></a>`;`

`pub use crate::uint::`<a href="uint/struct.UInt.html" class="struct"
title="struct typenum::uint::UInt"><code>UInt</code></a>`;`

`pub use crate::uint::`<a href="uint/struct.UTerm.html" class="struct"
title="struct typenum::uint::UTerm"><code>UTerm</code></a>`;`

`pub use generic_const_mappings::`<a href="generic_const_mappings/struct.Const.html" class="struct"
title="struct typenum::generic_const_mappings::Const"><code>Const</code></a>`;`

`pub use generic_const_mappings::`<a href="generic_const_mappings/trait.ToUInt.html" class="trait"
title="trait typenum::generic_const_mappings::ToUInt"><code>ToUInt</code></a>`;`

`pub use generic_const_mappings::`<a href="generic_const_mappings/type.U.html" class="type"
title="type typenum::generic_const_mappings::U"><code>U</code></a>`;`

`pub use crate::`<a href="marker_traits/index.html" class="mod"
title="mod typenum::marker_traits"><code>marker_traits</code></a>`::*;`

`pub use crate::`<a href="operator_aliases/index.html" class="mod"
title="mod typenum::operator_aliases"><code>operator_aliases</code></a>`::*;`

`pub use crate::`<a href="type_operators/index.html" class="mod"
title="mod typenum::type_operators"><code>type_operators</code></a>`::*;`

`pub use `<a href="consts/index.html" class="mod"
title="mod typenum::consts"><code>consts</code></a>`::*;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="array/index.html" class="mod"
title="mod typenum::array">array</a>  
A type-level array of type-level numbers.

<a href="bit/index.html" class="mod" title="mod typenum::bit">bit</a>  
Type-level bits.

<a href="consts/index.html" class="mod"
title="mod typenum::consts">consts</a>  
<a href="generic_const_mappings/index.html" class="mod"
title="mod typenum::generic_const_mappings">generic_const_mappings</a>  
Module with some `const`-generics-friendly definitions, to help bridge
the gap between those and `typenum` types.

<a href="int/index.html" class="mod" title="mod typenum::int">int</a>  
Type-level signed integers.

<a href="marker_traits/index.html" class="mod"
title="mod typenum::marker_traits">marker_traits</a>  
All of the **marker traits** used in typenum.

<a href="operator_aliases/index.html" class="mod"
title="mod typenum::operator_aliases">operator_aliases</a>  
Aliases for the type operators used in this crate. Their purpose is to
increase the ergonomics of performing operations on the types defined
here. For even more ergonomics, consider using the `op!` macro instead.

<a href="type_operators/index.html" class="mod"
title="mod typenum::type_operators">type_operators</a>  
Useful **type operators** that are not defined in `core::ops`.

<a href="uint/index.html" class="mod" title="mod typenum::uint">uint</a>  
Type-level unsigned integers.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.assert_type.html" class="macro"
title="macro typenum::assert_type">assert_type</a>  
Asserts that a type is `True`, aka `B1`.

<a href="macro.assert_type_eq.html" class="macro"
title="macro typenum::assert_type_eq">assert_type_eq</a>  
Asserts that two types are the same.

<a href="macro.cmp.html" class="macro"
title="macro typenum::cmp">cmp</a><span class="stab deprecated" title="">Deprecated</span>  
A convenience macro for comparing type numbers. Use `op!` instead.

<a href="macro.op.html" class="macro" title="macro typenum::op">op</a>  
Convenient type operations.

<a href="macro.tarr.html" class="macro"
title="macro typenum::tarr">tarr</a>  
Create a new type-level array. Only usable on Rust 1.13.0 or newer.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Equal.html" class="struct"
title="struct typenum::Equal">Equal</a>  
A potential output from `Cmp`, this is the type equivalent to the enum
variant `core::cmp::Ordering::Equal`.

<a href="struct.Greater.html" class="struct"
title="struct typenum::Greater">Greater</a>  
A potential output from `Cmp`, this is the type equivalent to the enum
variant `core::cmp::Ordering::Greater`.

<a href="struct.Less.html" class="struct"
title="struct typenum::Less">Less</a>  
A potential output from `Cmp`, this is the type equivalent to the enum
variant `core::cmp::Ordering::Less`.
