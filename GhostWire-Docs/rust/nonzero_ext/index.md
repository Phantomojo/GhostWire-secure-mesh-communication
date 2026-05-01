Crate nonzero\_ext Copy item path

<span class="sub-heading"><a href="../src/nonzero_ext/lib.rs.html#1-329" class="src">Source</a>
</span>

Expand description

# <a href="#traits-to-represent-generic-nonzero-integer-types"
class="doc-anchor">§</a>Traits to represent generic nonzero integer types

[![Build
Status](https://travis-ci.com/antifuchs/nonzero_ext.svg?branch=master)](https://travis-ci.com/antifuchs/nonzero_ext)
[![Docs](https://docs.rs/nonzero_ext/badge.svg)](https://docs.rs/nonzero_ext)

Rust ships with non-zero integer types now, which let programmers
promise (memory-savingly!) that a number can never be zero. That’s
great, but sadly the standard library has not got a whole lot of tools
to help you use them ergonomically.

## <a href="#a-macro-for-non-zero-constant-literals"
class="doc-anchor">§</a>A macro for non-zero constant literals

Creating and handling constant literals is neat, but the standard
library (and the rust parser at the moment) have no affordances to
easily create values of `num::NonZeroU*` types from constant literals.
This crate ships a `nonzero!` macro that lets you write
`nonzero!(20u32)`, which checks at compile time that the constant being
converted is non-zero, instead of the cumbersome (and runtime-checked!)
`NonZeroU32::new(20).unwrap()`.

## <a href="#traits-for-generic-non-zeroness" class="doc-anchor">§</a>Traits for generic non-zeroness

The stdlib `num::NonZeroU*` types do not implement any common traits
(and neither do their zeroable equivalents). Where this lack of traits
in the standard library becomes problematic is if you want to write a
function that takes a vector of integers, and that returns a vector of
the corresponding non-zero integer types, minus any elements that were
zero in the original. You can write that with the standard library quite
easily for concrete types:

``` {.rust .rust-example-rendered}
fn only_nonzeros(v: Vec<u8>) -> Vec<NonZeroU8>
{
    v.into_iter()
        .filter_map(|n| NonZeroU8::new(n))
        .collect::<Vec<NonZeroU8>>()
}
let expected: Vec<NonZeroU8> = vec![nonzero!(20u8), nonzero!(5u8)];
assert_eq!(expected, only_nonzeros(vec![0, 20, 5]));
```

But what if you want to allow this function to work with any integer
type that has a corresponding non-zero type? This crate can help:

``` {.rust .rust-example-rendered}
fn only_nonzeros<I>(v: Vec<I>) -> Vec<I::NonZero>
where
    I: Sized + NonZeroAble,
{
    v.into_iter()
        .filter_map(|n| n.as_nonzero())
        .collect::<Vec<I::NonZero>>()
}

// It works for `u8`:
let input_u8: Vec<u8> = vec![0, 20, 5];
let expected_u8: Vec<NonZeroU8> = vec![nonzero!(20u8), nonzero!(5u8)];
assert_eq!(expected_u8, only_nonzeros(input_u8));

// And it works for `u32`:
let input_u32: Vec<u32> = vec![0, 20, 5];
let expected_u32: Vec<NonZeroU32> = vec![nonzero!(20u32), nonzero!(5u32)];
assert_eq!(expected_u32, only_nonzeros(input_u32));
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="literals/index.html" class="mod"
title="mod nonzero_ext::literals">literals</a>  
Handling non-zero literal values.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.nonzero.html" class="macro"
title="macro nonzero_ext::nonzero">nonzero</a>  
Create non-zero values from constant literals easily.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.NonZero.html" class="trait"
title="trait nonzero_ext::NonZero">NonZero</a>  
A trait identifying a non-zero integral type. It is useful mostly in
order to give to genericized helper functions as `impl NonZero`
arguments.

<a href="trait.NonZeroAble.html" class="trait"
title="trait nonzero_ext::NonZeroAble">NonZeroAble</a>  
A trait identifying integral types that have a non-zeroable equivalent.
