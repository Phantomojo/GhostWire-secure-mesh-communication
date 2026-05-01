Crate noisy\_float Copy item path

<span class="sub-heading"><a href="../src/noisy_float/lib.rs.html#15-583" class="src">Source</a>
</span>

Expand description

This crate contains floating point types that panic if they are set to
an illegal value, such as NaN.

The name “Noisy Float” comes from the terms “quiet NaN” and “signaling
NaN”; “signaling” was too long to put in a struct/crate name, so “noisy”
is used instead, being the opposite of “quiet.”

The standard types defined in `noisy_float::types` follow the principle
demonstrated by Rust’s handling of integer overflow: a bad arithmetic
operation is considered an error, but it is too costly to check
everywhere in optimized builds. For each floating point number that is
created, a `debug_assert!` invocation is used to check if it is valid or
not. This way, there are guarantees when developing code that floating
point numbers have valid values, but during a release run there is *no
overhead* for using these floating point types compared to using `f32`
or `f64` directly.

This crate makes use of the num, bounded, signed and floating point
traits in the popular `num_traits` crate. This crate can be compiled
with no\_std.

# <a href="#examples" class="doc-anchor">§</a>Examples

An example using the `R64` type, which corresponds to *finite* `f64`
values.

``` {.rust .rust-example-rendered}
use noisy_float::prelude::*;

fn geometric_mean(a: R64, b: R64) -> R64 {
    (a * b).sqrt() //used just like regular floating point numbers
}

fn mean(a: R64, b: R64) -> R64 {
    (a + b) * 0.5 //the RHS of ops can be the underlying float type
}

println!(
    "geometric_mean(10.0, 20.0) = {}",
    geometric_mean(r64(10.0), r64(20.0))
);
//prints 14.142...
assert!(mean(r64(10.0), r64(20.0)) == 15.0);
```

An example using the `N32` type, which corresponds to *non-NaN* `f32`
values. The float types in this crate are able to implement `Eq` and
`Ord` properly, since NaN is not allowed.

``` {.rust .rust-example-rendered}
use noisy_float::prelude::*;

let values = vec![n32(3.0), n32(-1.5), n32(71.3), N32::infinity()];
assert!(values.iter().cloned().min() == Some(n32(-1.5)));
assert!(values.iter().cloned().max() == Some(N32::infinity()));
```

An example converting from R64 to primitive types.

``` {.rust .rust-example-rendered}
use noisy_float::prelude::*;
use num_traits::cast::ToPrimitive;

let value_r64: R64 = r64(1.0);
let value_f64_a: f64 = value_r64.into();
let value_f64_b: f64 = value_r64.raw();
let value_u64: u64 = value_r64.to_u64().unwrap();

assert!(value_f64_a == value_f64_b);
assert!(value_f64_a as u64 == value_u64);
```

# <a href="#features" class="doc-anchor">§</a>Features

This crate has the following cargo features:

-   `serde`: Enable serialization for all `NoisyFloats` using serde 1.0
    and will transparently serialize then as floats
-   `approx`: Adds implementations to use `NoisyFloat` with the `approx`
    crate

# Modules<a href="#modules" class="anchor">§</a>

<a href="checkers/index.html" class="mod"
title="mod noisy_float::checkers">checkers</a>  
Standard implementations of `FloatChecker`.

<a href="prelude/index.html" class="mod"
title="mod noisy_float::prelude">prelude</a>  
Prelude for the `noisy_float` crate.

<a href="types/index.html" class="mod"
title="mod noisy_float::types">types</a>  
Standard definitions of `NoisyFloat`.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.NoisyFloat.html" class="struct"
title="struct noisy_float::NoisyFloat">NoisyFloat</a>  
A floating point number with a restricted set of legal values.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.FloatChecker.html" class="trait"
title="trait noisy_float::FloatChecker">FloatChecker</a>  
Trait for checking whether a floating point number is *valid*.
