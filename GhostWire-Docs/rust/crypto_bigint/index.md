Crate crypto\_bigint Copy item path

<span class="sub-heading"><a href="../src/crypto_bigint/lib.rs.html#1-217" class="src">Source</a>
</span>

Expand description

# <a href="#rustcrypto-cryptographic-big-integers"
class="doc-anchor">§</a>[RustCrypto](https://github.com/rustcrypto): Cryptographic Big Integers

[![crate](https://buildstats.info/crate/crypto-bigint)](https://crates.io/crates/crypto-bigint)
[![Docs](https://docs.rs/crypto-bigint/badge.svg)](https://docs.rs/crypto-bigint/)
[![Build
Status](https://github.com/RustCrypto/crypto-bigint/actions/workflows/crypto-bigint.yml/badge.svg)](https://github.com/RustCrypto/crypto-bigint/actions/workflows/crypto-bigint.yml)
![Apache2/MIT
licensed](https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rustc-1.65+-blue.svg)
[![Project
Chat](https://img.shields.io/badge/zulip-join_chat-blue.svg)](https://rustcrypto.zulipchat.com/#narrow/stream/300602-crypto-bigint)

Pure Rust implementation of a big integer library which has been
designed from the ground-up for use in cryptographic applications.

Provides constant-time, `no_std`-friendly implementations of modern
formulas using const generics.

[Documentation](https://docs.rs/crypto-bigint/)

## <a href="#goals" class="doc-anchor">§</a>Goals

-   Supports `no_std`-friendly const generic stack-allocated big
    integers.
-   Constant-time by default. Variable-time functions are explicitly
    marked as such.
-   Leverage what is possible today with const generics on `stable`
    rust.
-   Support `const fn` as much as possible, including decoding big
    integers from bytes/hex and performing arithmetic operations on
    them, with the goal of being able to compute values at compile-time.

## <a href="#security-notes" class="doc-anchor">§</a>Security Notes

This crate has been [audited by NCC
Group](https://research.nccgroup.com/2023/08/30/public-report-entropy-rust-cryptography-review/)
with no significant findings. We would like to thank
[Entropy](https://entropy.xyz/) for funding the audit.

All functions contained in the crate are designed to execute in constant
time unless explicitly specified otherwise (via a `*_vartime` name
suffix).

This library is not suitable for use on processors with a variable-time
multiplication operation (e.g. short circuit on multiply-by-zero /
multiply-by-one, such as certain 32-bit PowerPC CPUs and some non-ARM
microcontrollers).

## <a href="#minimum-supported-rust-version" class="doc-anchor">§</a>Minimum Supported Rust Version

This crate requires **Rust 1.65** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a
minor version bump.

## <a href="#license" class="doc-anchor">§</a>License

Licensed under either of:

-   [Apache License, Version
    2.0](http://www.apache.org/licenses/LICENSE-2.0)
-   [MIT license](http://opensource.org/licenses/MIT)

at your option.

### <a href="#contribution" class="doc-anchor">§</a>Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

## <a href="#usage" class="doc-anchor">§</a>Usage

This crate defines a
[`Uint`](struct.Uint.html "struct crypto_bigint::Uint") type which is
const generic around an inner
[`Limb`](struct.Limb.html "struct crypto_bigint::Limb") array, where a
[`Limb`](struct.Limb.html "struct crypto_bigint::Limb") is a newtype for
a word-sized integer. Thus large integers are represented as arrays of
smaller integers which are sized appropriately for the CPU, giving us
some assurances of how arithmetic operations over those smaller integers
will behave.

To obtain appropriately sized integers regardless of what a given CPU’s
word size happens to be, a number of portable type aliases are provided
for integer sizes commonly used in cryptography, for example:
[`U128`](type.U128.html "type crypto_bigint::U128"),
[`U384`](type.U384.html "type crypto_bigint::U384"),
[`U256`](type.U256.html "type crypto_bigint::U256"),
[`U2048`](type.U2048.html "type crypto_bigint::U2048"),
[`U3072`](type.U3072.html "type crypto_bigint::U3072"),
[`U4096`](type.U4096.html "type crypto_bigint::U4096").

### <a href="#const-fn-usage" class="doc-anchor">§</a>`const fn` usage

The [`Uint`](struct.Uint.html "struct crypto_bigint::Uint") type
provides a number of `const fn` inherent methods which can be used for
initializing and performing arithmetic on big integers in const
contexts:

``` {.rust .rust-example-rendered}
use crypto_bigint::U256;

// Parse a constant from a big endian hexadecimal string.
pub const MODULUS: U256 =
    U256::from_be_hex("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");

// Compute `MODULUS` shifted right by 1 at compile time
pub const MODULUS_SHR1: U256 = MODULUS.shr_vartime(1);
```

Note that large constant computations may accidentally trigger a the
`const_eval_limit` of the compiler. The current way to deal with this
problem is to either simplify this computation, or increase the
compiler’s limit (currently a nightly feature). One can completely
remove the compiler’s limit using:

<a href="#" class="tooltip" title="This example is not tested">ⓘ</a>

``` {.rust .rust-example-rendered}
#![feature(const_eval_limit)]
#![const_eval_limit = "0"]
```

### <a href="#trait-based-usage" class="doc-anchor">§</a>Trait-based usage

The [`Uint`](struct.Uint.html "struct crypto_bigint::Uint") type itself
does not implement the standard arithmetic traits such as
[`Add`](https://doc.rust-lang.org/1.93.1/core/ops/arith/trait.Add.html "trait core::ops::arith::Add"),
[`Sub`](https://doc.rust-lang.org/1.93.1/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub"),
[`Mul`](https://doc.rust-lang.org/1.93.1/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul"),
and
[`Div`](https://doc.rust-lang.org/1.93.1/core/ops/arith/trait.Div.html "trait core::ops::arith::Div").

To use these traits you must first pick a wrapper type which determines
overflow behavior:
[`Wrapping`](struct.Wrapping.html "struct crypto_bigint::Wrapping") or
[`Checked`](struct.Checked.html "struct crypto_bigint::Checked").

#### <a href="#wrapping-arithmetic" class="doc-anchor">§</a>Wrapping arithmetic

``` {.rust .rust-example-rendered}
use crypto_bigint::{U256, Wrapping};

let a = Wrapping(U256::MAX);
let b = Wrapping(U256::ONE);
let c = a + b;

// `MAX` + 1 wraps back around to zero
assert_eq!(c.0, U256::ZERO);
```

#### <a href="#checked-arithmetic" class="doc-anchor">§</a>Checked arithmetic

``` {.rust .rust-example-rendered}
use crypto_bigint::{U256, Checked};

let a = Checked::new(U256::ONE);
let b = Checked::new(U256::from(2u8));
let c = a + b;
assert_eq!(c.0.unwrap(), U256::from(3u8))
```

### <a href="#modular-arithmetic" class="doc-anchor">§</a>Modular arithmetic

This library has initial support for modular arithmetic in the form of
the [`AddMod`](trait.AddMod.html "trait crypto_bigint::AddMod"),
[`SubMod`](trait.SubMod.html "trait crypto_bigint::SubMod"),
[`NegMod`](trait.NegMod.html "trait crypto_bigint::NegMod"), and
[`MulMod`](trait.MulMod.html "trait crypto_bigint::MulMod") traits, as
well as the support for the
[`Rem`](https://doc.rust-lang.org/1.93.1/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")
trait when used with a
[`NonZero`](struct.NonZero.html "struct crypto_bigint::NonZero")
operand.

``` {.rust .rust-example-rendered}
use crypto_bigint::{AddMod, U256};

// mod 3
let modulus = U256::from(3u8);

// 1 + 1 mod 3 = 2
let a = U256::ONE.add_mod(&U256::ONE, &modulus);
assert_eq!(a, U256::from(2u8));

// 2 + 1 mod 3 = 0
let b = a.add_mod(&U256::ONE, &modulus);
assert_eq!(b, U256::ZERO);
```

It also supports modular arithmetic over constant moduli using
`Residue`, and over moduli set at runtime using `DynResidue`. That
includes modular exponentiation and multiplicative inverses. These
features are described in the
[`modular`](modular/index.html "mod crypto_bigint::modular") module.

### <a href="#random-number-generation" class="doc-anchor">§</a>Random number generation

When the `rand_core` or `rand` features of this crate are enabled, it’s
possible to generate random numbers using any CSRNG by using the
[`Random`](trait.Random.html "trait crypto_bigint::Random") trait:

``` {.rust .rust-example-rendered}
use crypto_bigint::{Random, U256, rand_core::OsRng};

let n = U256::random(&mut OsRng);
```

#### <a href="#modular-random-number-generation" class="doc-anchor">§</a>Modular random number generation

The [`RandomMod`](trait.RandomMod.html "trait crypto_bigint::RandomMod")
trait supports generating random numbers with a uniform distribution
around a given
[`NonZero`](struct.NonZero.html "struct crypto_bigint::NonZero")
modulus.

``` {.rust .rust-example-rendered}
use crypto_bigint::{NonZero, RandomMod, U256, rand_core::OsRng};

let modulus = NonZero::new(U256::from(3u8)).unwrap();
let n = U256::random_mod(&mut OsRng, &modulus);
```

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../subtle/index.html" class="mod"
title="mod subtle"><code>subtle</code></a>`;`

`pub use `<a href="../generic_array/index.html" class="mod"
title="mod generic_array"><code>generic_array</code></a>`;`

`pub use `<a href="../rand_core/index.html" class="mod"
title="mod rand_core"><code>rand_core</code></a>`;`

`pub use `<a href="../zeroize/index.html" class="mod"
title="mod zeroize"><code>zeroize</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="consts/index.html" class="mod"
title="mod crypto_bigint::consts">consts</a>  
<a href="modular/index.html" class="mod"
title="mod crypto_bigint::modular">modular</a>  
Implements modular arithmetic for constant moduli.

<a href="prelude/index.html" class="mod"
title="mod crypto_bigint::prelude">prelude</a>  
Import prelude for this crate: includes important traits.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.const_assert_eq.html" class="macro"
title="macro crypto_bigint::const_assert_eq">const_assert_eq</a>  
Const-friendly assertion that two values are equal.

<a href="macro.const_assert_ne.html" class="macro"
title="macro crypto_bigint::const_assert_ne">const_assert_ne</a>  
Const-friendly assertion that two values are NOT equal.

<a href="macro.const_residue.html" class="macro"
title="macro crypto_bigint::const_residue">const_residue</a>  
Creates a `Residue` with the given value for a specific modulus. For
example, `residue!(U256::from(105u64), MyModulus);` creates a `Residue`
for 105 mod `MyModulus`. The modulus *must* be odd, or this will panic.

<a href="macro.impl_modulus.html" class="macro"
title="macro crypto_bigint::impl_modulus">impl_modulus</a>  
Implements a modulus with the given name, type, and value, in that
specific order. Please `use crypto_bigint::traits::Encoding` to make
this work. For example,
`impl_modulus!(MyModulus, U256, "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001");`
implements a 256-bit modulus named `MyModulus`. The modulus *must* be
odd, or this will panic.

<a href="macro.nlimbs.html" class="macro"
title="macro crypto_bigint::nlimbs">nlimbs</a>  
Calculate the number of limbs required to represent the given number of
bits.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Checked.html" class="struct"
title="struct crypto_bigint::Checked">Checked</a>  
Provides intentionally-checked arithmetic on `T`.

<a href="struct.CtChoice.html" class="struct"
title="struct crypto_bigint::CtChoice">CtChoice</a>  
A boolean value returned by constant-time `const fn`s.

<a href="struct.Limb.html" class="struct"
title="struct crypto_bigint::Limb">Limb</a>  
Big integers are represented as an array of smaller CPU word-size
integers called “limbs”.

<a href="struct.NonZero.html" class="struct"
title="struct crypto_bigint::NonZero">NonZero</a>  
Wrapper type for non-zero integers.

<a href="struct.Reciprocal.html" class="struct"
title="struct crypto_bigint::Reciprocal">Reciprocal</a>  
A pre-calculated reciprocal for division by a single limb.

<a href="struct.Uint.html" class="struct"
title="struct crypto_bigint::Uint">Uint</a>  
Stack-allocated big unsigned integer.

<a href="struct.Wrapping.html" class="struct"
title="struct crypto_bigint::Wrapping">Wrapping</a>  
Provides intentionally-wrapped arithmetic on `T`.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AddMod.html" class="trait"
title="trait crypto_bigint::AddMod">AddMod</a>  
Compute `self + rhs mod p`.

<a href="trait.ArrayDecoding.html" class="trait"
title="trait crypto_bigint::ArrayDecoding">ArrayDecoding</a>  
Support for decoding a `GenericArray` as a big integer.

<a href="trait.ArrayEncoding.html" class="trait"
title="trait crypto_bigint::ArrayEncoding">ArrayEncoding</a>  
Support for encoding a big integer as a `GenericArray`.

<a href="trait.Bounded.html" class="trait"
title="trait crypto_bigint::Bounded">Bounded</a>  
Integers whose representation takes a bounded amount of space.

<a href="trait.CheckedAdd.html" class="trait"
title="trait crypto_bigint::CheckedAdd">CheckedAdd</a>  
Checked addition.

<a href="trait.CheckedMul.html" class="trait"
title="trait crypto_bigint::CheckedMul">CheckedMul</a>  
Checked multiplication.

<a href="trait.CheckedSub.html" class="trait"
title="trait crypto_bigint::CheckedSub">CheckedSub</a>  
Checked subtraction.

<a href="trait.Concat.html" class="trait"
title="trait crypto_bigint::Concat">Concat</a>  
Concatenate two numbers into a “wide” double-width value, using the `lo`
value as the least significant value.

<a href="trait.ConcatMixed.html" class="trait"
title="trait crypto_bigint::ConcatMixed">ConcatMixed</a>  
Concatenate two numbers into a “wide” combined-width value, using the
`lo` value as the least significant value.

<a href="trait.Encoding.html" class="trait"
title="trait crypto_bigint::Encoding">Encoding</a>  
Encoding support.

<a href="trait.Integer.html" class="trait"
title="trait crypto_bigint::Integer">Integer</a>  
Integer type.

<a href="trait.Invert.html" class="trait"
title="trait crypto_bigint::Invert">Invert</a>  
Constant-time inversion.

<a href="trait.MulMod.html" class="trait"
title="trait crypto_bigint::MulMod">MulMod</a>  
Compute `self * rhs mod p`.

<a href="trait.MultiExponentiate.html" class="trait"
title="trait crypto_bigint::MultiExponentiate">MultiExponentiate</a>  
Performs modular multi-exponentiation using Montgomery’s ladder.

<a href="trait.MultiExponentiateBoundedExp.html" class="trait"
title="trait crypto_bigint::MultiExponentiateBoundedExp">MultiExponentiateBoundedExp</a>  
Performs modular multi-exponentiation using Montgomery’s ladder.
`exponent_bits` represents the number of bits to take into account for
the exponent.

<a href="trait.NegMod.html" class="trait"
title="trait crypto_bigint::NegMod">NegMod</a>  
Compute `-self mod p`.

<a href="trait.Pow.html" class="trait"
title="trait crypto_bigint::Pow">Pow</a>  
Constant-time exponentiation.

<a href="trait.PowBoundedExp.html" class="trait"
title="trait crypto_bigint::PowBoundedExp">PowBoundedExp</a>  
Constant-time exponentiation with exponent of a bounded bit size.

<a href="trait.Random.html" class="trait"
title="trait crypto_bigint::Random">Random</a>  
Random number generation support.

<a href="trait.RandomMod.html" class="trait"
title="trait crypto_bigint::RandomMod">RandomMod</a>  
Modular random number generation support.

<a href="trait.Split.html" class="trait"
title="trait crypto_bigint::Split">Split</a>  
Split a number in half, returning the most significant half followed by
the least significant.

<a href="trait.SplitMixed.html" class="trait"
title="trait crypto_bigint::SplitMixed">SplitMixed</a>  
Split a number into parts, returning the most significant part followed
by the least significant.

<a href="trait.Square.html" class="trait"
title="trait crypto_bigint::Square">Square</a>  
Support for optimized squaring

<a href="trait.SubMod.html" class="trait"
title="trait crypto_bigint::SubMod">SubMod</a>  
Compute `self - rhs mod p`.

<a href="trait.Zero.html" class="trait"
title="trait crypto_bigint::Zero">Zero</a>  
Zero values.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.ByteArray.html" class="type"
title="type crypto_bigint::ByteArray">ByteArray</a>  
Alias for a byte array whose size is defined by
[`ArrayEncoding::ByteSize`](trait.ArrayEncoding.html#associatedtype.ByteSize "associated type crypto_bigint::ArrayEncoding::ByteSize").

<a href="type.U64.html" class="type"
title="type crypto_bigint::U64">U64</a>  
64-bit unsigned big integer.

<a href="type.U128.html" class="type"
title="type crypto_bigint::U128">U128</a>  
128-bit unsigned big integer.

<a href="type.U192.html" class="type"
title="type crypto_bigint::U192">U192</a>  
192-bit unsigned big integer.

<a href="type.U256.html" class="type"
title="type crypto_bigint::U256">U256</a>  
256-bit unsigned big integer.

<a href="type.U320.html" class="type"
title="type crypto_bigint::U320">U320</a>  
320-bit unsigned big integer.

<a href="type.U384.html" class="type"
title="type crypto_bigint::U384">U384</a>  
384-bit unsigned big integer.

<a href="type.U448.html" class="type"
title="type crypto_bigint::U448">U448</a>  
448-bit unsigned big integer.

<a href="type.U512.html" class="type"
title="type crypto_bigint::U512">U512</a>  
512-bit unsigned big integer.

<a href="type.U576.html" class="type"
title="type crypto_bigint::U576">U576</a>  
576-bit unsigned big integer.

<a href="type.U640.html" class="type"
title="type crypto_bigint::U640">U640</a>  
640-bit unsigned big integer.

<a href="type.U704.html" class="type"
title="type crypto_bigint::U704">U704</a>  
704-bit unsigned big integer.

<a href="type.U768.html" class="type"
title="type crypto_bigint::U768">U768</a>  
768-bit unsigned big integer.

<a href="type.U832.html" class="type"
title="type crypto_bigint::U832">U832</a>  
832-bit unsigned big integer.

<a href="type.U896.html" class="type"
title="type crypto_bigint::U896">U896</a>  
896-bit unsigned big integer.

<a href="type.U960.html" class="type"
title="type crypto_bigint::U960">U960</a>  
960-bit unsigned big integer.

<a href="type.U1024.html" class="type"
title="type crypto_bigint::U1024">U1024</a>  
1024-bit unsigned big integer.

<a href="type.U1280.html" class="type"
title="type crypto_bigint::U1280">U1280</a>  
1280-bit unsigned big integer.

<a href="type.U1536.html" class="type"
title="type crypto_bigint::U1536">U1536</a>  
1536-bit unsigned big integer.

<a href="type.U1792.html" class="type"
title="type crypto_bigint::U1792">U1792</a>  
1792-bit unsigned big integer.

<a href="type.U2048.html" class="type"
title="type crypto_bigint::U2048">U2048</a>  
2048-bit unsigned big integer.

<a href="type.U3072.html" class="type"
title="type crypto_bigint::U3072">U3072</a>  
3072-bit unsigned big integer.

<a href="type.U3584.html" class="type"
title="type crypto_bigint::U3584">U3584</a>  
3584-bit unsigned big integer.

<a href="type.U4096.html" class="type"
title="type crypto_bigint::U4096">U4096</a>  
4096-bit unsigned big integer.

<a href="type.U4224.html" class="type"
title="type crypto_bigint::U4224">U4224</a>  
4224-bit unsigned big integer.

<a href="type.U4352.html" class="type"
title="type crypto_bigint::U4352">U4352</a>  
4352-bit unsigned big integer.

<a href="type.U6144.html" class="type"
title="type crypto_bigint::U6144">U6144</a>  
6144-bit unsigned big integer.

<a href="type.U8192.html" class="type"
title="type crypto_bigint::U8192">U8192</a>  
8192-bit unsigned big integer.

<a href="type.U16384.html" class="type"
title="type crypto_bigint::U16384">U16384</a>  
16384-bit unsigned big integer.

<a href="type.U32768.html" class="type"
title="type crypto_bigint::U32768">U32768</a>  
32768-bit unsigned big integer.

<a href="type.WideWord.html" class="type"
title="type crypto_bigint::WideWord">WideWord</a>  
Wide integer type: double the width of
[`Word`](type.Word.html "type crypto_bigint::Word").

<a href="type.Word.html" class="type"
title="type crypto_bigint::Word">Word</a>  
Unsigned integer type that the
[`Limb`](struct.Limb.html "struct crypto_bigint::Limb") newtype wraps.
