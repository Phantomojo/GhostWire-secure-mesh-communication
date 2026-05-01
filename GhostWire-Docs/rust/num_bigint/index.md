Crate num\_bigint Copy item path

<span class="sub-heading"><a href="../src/num_bigint/lib.rs.html#11-268" class="src">Source</a>
</span>

Expand description

Big Integer Types for Rust

-   A [`BigUint`](struct.BigUint.html "struct num_bigint::BigUint") is
    unsigned and represented as a vector of digits.
-   A [`BigInt`](struct.BigInt.html "struct num_bigint::BigInt") is
    signed and is a combination of
    [`BigUint`](struct.BigUint.html "struct num_bigint::BigUint") and
    [`Sign`](enum.Sign.html "enum num_bigint::Sign").

Common numerical operations are overloaded, so we can treat them the
same way we treat other numbers.

## <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use num_bigint::BigUint;
use num_traits::One;

// Calculate large fibonacci numbers.
fn fib(n: usize) -> BigUint {
    let mut f0 = BigUint::ZERO;
    let mut f1 = BigUint::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}

// This is a very large number.
println!("fib(1000) = {}", fib(1000));
```

It’s easy to generate large random numbers:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
use num_bigint::{ToBigInt, RandBigInt};

let mut rng = rand::thread_rng();
let a = rng.gen_bigint(1000);

let low = -10000.to_bigint().unwrap();
let high = 10000.to_bigint().unwrap();
let b = rng.gen_bigint_range(&low, &high);

// Probably an even larger number.
println!("{}", a * b);
```

See the “Features” section for instructions for enabling random number
generation.

## <a href="#features" class="doc-anchor">§</a>Features

The `std` crate feature is enabled by default, which enables
[`std::error::Error`](https://doc.rust-lang.org/1.93.1/core/error/trait.Error.html "trait core::error::Error")
implementations and some internal use of floating point approximations.
This can be disabled by depending on `num-bigint` with
`default-features = false`. Either way, the `alloc` crate is always
required for heap allocation of the `BigInt`/`BigUint` digits.

### <a href="#random-generation" class="doc-anchor">§</a>Random Generation

`num-bigint` supports the generation of random big integers when the
`rand` feature is enabled. To enable it include rand as

``` language-toml
rand = "0.8"
num-bigint = { version = "0.4", features = ["rand"] }
```

Note that you must use the version of `rand` that `num-bigint` is
compatible with: `0.8`.

### <a href="#arbitrary-big-integers" class="doc-anchor">§</a>Arbitrary Big Integers

`num-bigint` supports `arbitrary` and `quickcheck` features to implement
\[`arbitrary::Arbitrary`\] and \[`quickcheck::Arbitrary`\],
respectively, for both `BigInt` and `BigUint`. These are useful for
fuzzing and other forms of randomized testing.

### <a href="#serialization" class="doc-anchor">§</a>Serialization

The `serde` feature adds implementations of
\[`Serialize`\]\[serde::Serialize\] and
\[`Deserialize`\]\[serde::Deserialize\] for both `BigInt` and `BigUint`.
Their serialized data is generated portably, regardless of platform
differences like the internal digit size.

## <a href="#compatibility" class="doc-anchor">§</a>Compatibility

The `num-bigint` crate is tested for rustc 1.60 and greater.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BigInt.html" class="struct"
title="struct num_bigint::BigInt">BigInt</a>  
A big signed integer type.

<a href="struct.BigUint.html" class="struct"
title="struct num_bigint::BigUint">BigUint</a>  
A big unsigned integer type.

<a href="struct.ParseBigIntError.html" class="struct"
title="struct num_bigint::ParseBigIntError">ParseBigIntError</a>  
<a href="struct.TryFromBigIntError.html" class="struct"
title="struct num_bigint::TryFromBigIntError">TryFromBigIntError</a>  
The error type returned when a checked conversion regarding big integer
fails.

<a href="struct.U32Digits.html" class="struct"
title="struct num_bigint::U32Digits">U32Digits</a>  
An iterator of `u32` digits representation of a `BigUint` or `BigInt`,
ordered least significant digit first.

<a href="struct.U64Digits.html" class="struct"
title="struct num_bigint::U64Digits">U64Digits</a>  
An iterator of `u64` digits representation of a `BigUint` or `BigInt`,
ordered least significant digit first.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Sign.html" class="enum"
title="enum num_bigint::Sign">Sign</a>  
A `Sign` is a
[`BigInt`](struct.BigInt.html "struct num_bigint::BigInt")’s composing
element.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ToBigInt.html" class="trait"
title="trait num_bigint::ToBigInt">ToBigInt</a>  
A generic trait for converting a value to a
[`BigInt`](struct.BigInt.html "struct num_bigint::BigInt"). This may
return `None` when converting from `f32` or `f64`, and will always
succeed when converting from any integer or unsigned primitive, or
[`BigUint`](struct.BigUint.html "struct num_bigint::BigUint").

<a href="trait.ToBigUint.html" class="trait"
title="trait num_bigint::ToBigUint">ToBigUint</a>  
A generic trait for converting a value to a
[`BigUint`](struct.BigUint.html "struct num_bigint::BigUint").
