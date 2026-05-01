Crate subtle Copy item path

<span class="sub-heading"><a href="../src/subtle/lib.rs.html#11-1008" class="src">Source</a>
</span>

Expand description

# <a href="#subtle---" class="doc-anchor">§</a>subtle [![](https://img.shields.io/crates/v/subtle.svg)](https://crates.io/crates/subtle) [![](https://img.shields.io/badge/dynamic/json.svg?label=docs&uri=https%3A%2F%2Fcrates.io%2Fapi%2Fv1%2Fcrates%2Fsubtle%2Fversions&query=%24.versions%5B0%5D.num&colorB=4F74A6)](https://doc.dalek.rs/subtle) [![](https://travis-ci.org/dalek-cryptography/subtle.svg?branch=master)](https://travis-ci.org/dalek-cryptography/subtle)

**Pure-Rust traits and utilities for constant-time cryptographic
implementations.**

It consists of a `Choice` type, and a collection of traits using
`Choice` instead of `bool` which are intended to execute in
constant-time. The `Choice` type is a wrapper around a `u8` that holds a
`0` or `1`.

``` language-toml
subtle = "2.6"
```

This crate represents a “best-effort” attempt, since side-channels are
ultimately a property of a deployed cryptographic system including the
hardware it runs on, not just of software.

The traits are implemented using bitwise operations, and should execute
in constant time provided that a) the bitwise operations are
constant-time and b) the bitwise operations are not recognized as a
conditional assignment and optimized back into a branch.

For a compiler to recognize that bitwise operations represent a
conditional assignment, it needs to know that the value used to generate
the bitmasks is really a boolean `i1` rather than an `i8` byte value. In
an attempt to prevent this refinement, the crate tries to hide the value
of a `Choice`’s inner `u8` by passing it through a volatile read. For
more information, see the *About* section below.

Rust versions from 1.51 or higher have const generics support. You may
enable `const-generics` feautre to have `subtle` traits implemented for
arrays `[T; N]`.

Versions prior to `2.2` recommended use of the `nightly` feature to
enable an optimization barrier; this is not required in versions `2.2`
and above.

Note: the `subtle` crate contains `debug_assert`s to check invariants
during debug builds. These invariant checks involve secret-dependent
branches, and are not present when compiled in release mode. This crate
is intended to be used in release mode.

## <a href="#documentation" class="doc-anchor">§</a>Documentation

Documentation is available [here](https://docs.rs/subtle).

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

Rust **1.41** or higher.

Minimum supported Rust version can be changed in the future, but it will
be done with a minor version bump.

## <a href="#about" class="doc-anchor">§</a>About

This library aims to be the Rust equivalent of Go’s `crypto/subtle`
module.

Old versions of the optimization barrier in `impl From<u8> for Choice`
were based on Tim Maclean’s [work on
`rust-timing-shield`](https://www.chosenplaintext.ca/open-source/rust-timing-shield/security),
which attempts to provide a more comprehensive approach for preventing
software side-channels in Rust code. From version `2.2`, it was based on
Diane Hosfelt and Amber Sprenkels’ work on “Secret Types in Rust”.

`subtle` is authored by isis agora lovecruft and Henry de Valence.

## <a href="#warning" class="doc-anchor">§</a>Warning

This code is a low-level library, intended for specific use-cases
implementing cryptographic protocols. It represents a best-effort
attempt to protect against some software side-channels. Because
side-channel resistance is not a property of software alone, but of
software together with hardware, any such effort is fundamentally
limited.

**USE AT YOUR OWN RISK**

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.BlackBox.html" class="struct"
title="struct subtle::BlackBox">BlackBox</a>  
Wrapper type which implements an optimization barrier for all accesses.

<a href="struct.Choice.html" class="struct"
title="struct subtle::Choice">Choice</a>  
The `Choice` struct represents a choice for use in conditional
assignment.

<a href="struct.CtOption.html" class="struct"
title="struct subtle::CtOption">CtOption</a>  
The `CtOption<T>` type represents an optional value similar to the
[`Option<T>`](https://doc.rust-lang.org/1.93.1/core/option/enum.Option.html "enum core::option::Option")
type but is intended for use in constant time APIs.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.ConditionallyNegatable.html" class="trait"
title="trait subtle::ConditionallyNegatable">ConditionallyNegatable</a>  
A type which can be conditionally negated in constant time.

<a href="trait.ConditionallySelectable.html" class="trait"
title="trait subtle::ConditionallySelectable">ConditionallySelectable</a>  
A type which can be conditionally selected in constant time.

<a href="trait.ConstantTimeEq.html" class="trait"
title="trait subtle::ConstantTimeEq">ConstantTimeEq</a>  
An `Eq`-like trait that produces a `Choice` instead of a `bool`.

<a href="trait.ConstantTimeGreater.html" class="trait"
title="trait subtle::ConstantTimeGreater">ConstantTimeGreater</a>  
A type which can be compared in some manner and be determined to be
greater than another of the same type.

<a href="trait.ConstantTimeLess.html" class="trait"
title="trait subtle::ConstantTimeLess">ConstantTimeLess</a>  
A type which can be compared in some manner and be determined to be less
than another of the same type.
